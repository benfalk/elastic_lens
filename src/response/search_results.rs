use serde::de::{self, Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::fmt::{self, Debug};
use std::marker::PhantomData;
use std::time::Duration;

mod agg_result;

pub use agg_result::*;

/// The data that comes back from an Elasticsearch search
pub struct SearchResults<T> {
    search_time: Duration,
    count: ResultCount,
    timed_out: bool,
    shard_stats: ShardStats,
    max_score: Option<f64>,
    hits: Vec<DocumentHit<T>>,
    aggs: AggResultCollection,
}

impl<T: Debug> Debug for SearchResults<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SearchResults")
            .field("search_time", &self.search_time)
            .field("count", &self.count)
            .field("timed_out", &self.timed_out)
            .field("shard_stats", &self.shard_stats)
            .field("max_score", &self.max_score)
            .field("hits", &self.hits)
            .field("aggs", &self.aggs)
            .finish()
    }
}

impl<T: Clone> Clone for SearchResults<T> {
    fn clone(&self) -> Self {
        Self {
            search_time: self.search_time,
            count: self.count,
            timed_out: self.timed_out,
            shard_stats: self.shard_stats,
            max_score: self.max_score,
            hits: self.hits.clone(),
            aggs: self.aggs.clone(),
        }
    }
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

    /// mutable iterator over the entire hit from your search
    pub fn hits_mut(&mut self) -> impl Iterator<Item = &mut DocumentHit<T>> {
        self.hits.iter_mut()
    }

    /// takes the entire document hit collection from the results
    pub fn hits_take(&mut self) -> Vec<DocumentHit<T>> {
        let mut hits = vec![];
        std::mem::swap(&mut hits, &mut self.hits);
        hits
    }

    /// extracts the documents out of the results and returns them
    /// as a vector.  This completely drains all of the document data
    /// from the results leaving it empty.
    pub fn docs_take(&mut self) -> Vec<T> {
        let capacity = self.hits.len();

        self.hits
            .drain(..)
            .map(|h| h.doc)
            .rfold(Vec::with_capacity(capacity), |mut docs, doc| {
                docs.push(doc);
                docs
            })
    }

    /// convenience iterator over the documents from your current search
    pub fn docs(&self) -> impl Iterator<Item = &T> {
        self.hits.iter().map(|d| &d.doc)
    }

    /// convenience mutable document iterator from your search
    pub fn docs_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.hits.iter_mut().map(|d| &mut d.doc)
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
#[derive(Deserialize)]
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

impl<T: Debug> Debug for DocumentHit<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DocumentHit")
            .field("id", &self.id)
            .field("index", &self.index)
            .field("doc_type", &self.doc_type)
            .field("score", &self.score)
            .field("doc", &self.doc)
            .finish()
    }
}

impl<T: Clone> Clone for DocumentHit<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            index: self.index.clone(),
            doc_type: self.doc_type.clone(),
            score: self.score,
            doc: self.doc.clone(),
        }
    }
}

impl<'de, T> Deserialize<'de> for SearchResults<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        //
        // Deserialize ResultCount
        //

        #[allow(non_local_definitions)]
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
        struct Hits<T> {
            total: ResultCount,
            max_score: Option<f64>,
            hits: Vec<DocumentHit<T>>,
        }

        struct ResultsVistor<T>(PhantomData<T>);

        const FIELDS: &[&str; 6] = &[
            "hits",
            "aggregations",
            "took",
            "timed_out",
            "_shards",
            "status",
        ];

        impl<'de, T: Deserialize<'de>> Visitor<'de> for ResultsVistor<T> {
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
                        // TODO: This wires up msearch to get it working as each
                        //       result has a status in this case... but in the
                        //       case of failure we'll need to figure out what
                        //       happens.  Probably in the case of a multi-search
                        //       each response could be a separate result.
                        "status" => {
                            let status: u16 = map.next_value()?;

                            if status != 200 {
                                return Err(de::Error::custom(format!(
                                    "A response had a bad status: {status}"
                                )));
                            }
                        }
                        unknown => Err(de::Error::unknown_field(unknown, FIELDS))?,
                    }
                }

                let hits = hits.ok_or_else(|| de::Error::missing_field("hits"))?;
                let took = took.ok_or_else(|| de::Error::missing_field("took"))?;
                let timed_out = timed_out.ok_or_else(|| de::Error::missing_field("timed_out"))?;
                let shard_stats = shard_stats.ok_or_else(|| de::Error::missing_field("_shards"))?;
                let aggs = aggs.unwrap_or_default();

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
