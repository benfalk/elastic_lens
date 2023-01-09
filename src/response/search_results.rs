use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt;
use std::marker::PhantomData;
use std::time::Duration;

mod agg_result;

pub use agg_result::*;

/// The data that comes back from an Elasticsearch search
#[derive(Debug, Clone)]
pub struct SearchResults<T> {
    search_time: Duration,
    count: ResultCount,
    timed_out: bool,
    shard_stats: ShardStats,
    max_score: Option<f64>,
    hits: Vec<DocumentHit<T>>,
    aggs: AggResultCollection,
}

impl<T> SearchResults<T> {
    /// how long did the search take for the cluster
    pub fn search_time(&self) -> Duration {
        self.search_time
    }

    /// exactly or about at least how many matches total around found
    pub fn count(&self) -> ResultCount {
        self.count
    }

    /// did the search time out?
    pub fn timed_out(&self) -> bool {
        self.timed_out
    }

    /// stats on the shards from your search
    pub fn shard_stats(&self) -> ShardStats {
        self.shard_stats
    }

    /// max score from the search ( if any )
    pub fn max_score(&self) -> Option<f64> {
        self.max_score
    }

    /// iterator over the hits in your current search
    pub fn hits(&self) -> impl Iterator<Item = &DocumentHit<T>> {
        self.hits.iter()
    }

    /// convenience iterator over the documents from your current search
    pub fn docs(&self) -> impl Iterator<Item = &T> {
        self.hits.iter().map(|d| &d.doc)
    }

    /// Read-Only Access to the aggregations in the response
    pub fn aggs(&self) -> &AggResultCollection {
        &self.aggs
    }

    /// Mutable reference to the aggregations, useful if
    /// you want to take individual aggregations from the
    /// results because you don't need the rest of the results.
    pub fn aggs_mut(&mut self) -> &mut AggResultCollection {
        &mut self.aggs
    }
}

/// Idea of how many results matched a search
#[derive(Debug, Clone, Copy)]
pub enum ResultCount {
    /// at least this many documents matched
    AtLeast(usize),
    /// exactly this many documents matched
    Exactly(usize),
}

/// counts on shards which participated in the search
#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ShardStats {
    /// how many total shards are there
    pub total: u8,

    /// shard coutn that worked
    pub successful: u8,

    /// shard count that was skipped during this search
    pub skipped: u8,

    /// how many shards failed during this search
    pub failed: u8,
}

/// wrapper for a document with details around it
#[derive(Debug, Clone, Deserialize)]
pub struct DocumentHit<T> {
    /// Elasticsearch document ID
    #[serde(rename = "_id")]
    pub id: String,

    /// The "actual" index this document came from
    /// this is common to not match your index when
    /// using aliases
    #[serde(rename = "_index")]
    pub index: String,

    /// doc type ( if any )
    #[serde(rename = "_type")]
    pub doc_type: Option<String>,

    /// relievence search score if one was applied
    #[serde(rename = "_score")]
    pub score: Option<f64>,

    /// the document data
    #[serde(rename = "_source")]
    pub doc: T,
}

impl<'de, T> Deserialize<'de> for SearchResults<T>
where
    T: Deserialize<'de> + fmt::Debug + Clone,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //
        // Deserialize ResultCount
        //

        impl<'de> Deserialize<'de> for ResultCount {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct ResultCountVistor;

                enum Relation {
                    AtLeast,
                    Exactly,
                }

                const KEYS: &[&str; 2] = &["value", "relation"];
                const RELATIONS: &[&str; 2] = &["gte", "eq"];

                impl<'de> Visitor<'de> for ResultCountVistor {
                    type Value = ResultCount;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("ResultCount data")
                    }

                    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
                    where
                        M: MapAccess<'de>,
                    {
                        let mut amount: Option<usize> = None;
                        let mut relation: Option<Relation> = None;

                        while let Some(key) = map.next_key()? {
                            match key {
                                "value" => {
                                    amount = Some(map.next_value()?);
                                }
                                "relation" => match map.next_value()? {
                                    "gte" => {
                                        relation = Some(Relation::AtLeast);
                                    }
                                    "eq" => {
                                        relation = Some(Relation::Exactly);
                                    }
                                    unknown => {
                                        return Err(de::Error::unknown_variant(unknown, RELATIONS));
                                    }
                                },
                                unknown => {
                                    return Err(de::Error::unknown_variant(unknown, KEYS));
                                }
                            }
                        }

                        let amount = amount.ok_or_else(|| de::Error::custom("missing `value`"))?;
                        match relation.ok_or_else(|| de::Error::custom("missing `relation`"))? {
                            Relation::AtLeast => Ok(ResultCount::AtLeast(amount)),
                            Relation::Exactly => Ok(ResultCount::Exactly(amount)),
                        }
                    }
                }

                deserializer.deserialize_map(ResultCountVistor)
            }
        }

        //
        // Deserialize Results
        //

        #[derive(Deserialize)]
        struct Hits<T: Clone + fmt::Debug> {
            total: ResultCount,
            max_score: Option<f64>,
            hits: Vec<DocumentHit<T>>,
        }

        struct ResultsVistor<T>(PhantomData<T>);

        const FIELDS: &[&str; 5] = &["hits", "aggregations", "took", "timed_out", "_shards"];

        impl<'de, T: Clone + fmt::Debug + Deserialize<'de>> Visitor<'de> for ResultsVistor<T> {
            type Value = SearchResults<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Map with results from ElasticSearch")
            }

            fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut hits: Option<Hits<T>> = None;
                let mut shard_stats: Option<ShardStats> = None;
                let mut timed_out: Option<bool> = None;
                let mut took: Option<Duration> = None;
                let mut aggs: Option<AggResultCollection> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        "hits" => {
                            hits = Some(map.next_value()?);
                        }
                        "took" => {
                            took = Some(Duration::from_millis(map.next_value()?));
                        }
                        "timed_out" => {
                            timed_out = Some(map.next_value()?);
                        }
                        "_shards" => {
                            shard_stats = Some(map.next_value()?);
                        }
                        "aggregations" => {
                            aggs = Some(map.next_value()?);
                        }
                        unknown => Err(de::Error::unknown_field(unknown, FIELDS))?,
                    }
                }

                let hits = hits.ok_or_else(|| de::Error::missing_field("hits"))?;
                let took = took.ok_or_else(|| de::Error::missing_field("took"))?;
                let timed_out = timed_out.ok_or_else(|| de::Error::missing_field("timed_out"))?;
                let shard_stats = shard_stats.ok_or_else(|| de::Error::missing_field("_shards"))?;
                let aggs = aggs.unwrap_or_else(AggResultCollection::default);

                Ok(SearchResults {
                    search_time: took,
                    count: hits.total,
                    max_score: hits.max_score,
                    timed_out,
                    shard_stats,
                    hits: hits.hits,
                    aggs,
                })
            }
        }

        deserializer.deserialize_map(ResultsVistor(PhantomData))
    }
}
