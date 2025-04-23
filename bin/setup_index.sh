#!/bin/bash

curl -XPUT -H "Content-Type: application/json" http://localhost:9200/inventory -d '{
  "mappings": {
    "dynamic": false,
    "properties": {
      "category": {
        "type": "keyword"
      },
      "sub_category": {
        "type": "keyword"
      },
      "active": {
        "type": "boolean"
      },
      "cost": {
        "type": "long"
      },
      "vendors": {
        "type": "nested",
        "properties": {
          "slug": {
            "type": "keyword"
          },
          "country": {
            "type": "keyword"
          }
        }
      }
    }
  }
}'

curl -XPOST -H "Content-Type: application/x-ndjson" http://localhost:9200/_bulk -d '
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "1" } }
{ "category": "clothing", "sub_category": "t-shirt", "active": true, "cost": 1300, "vendors": [{"slug":"kmart", "country":"usa"}, {"slug":"walmart", "country":"usa"}]}
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "2" } }
{ "category": "clothing", "sub_category": "pants", "active": true, "cost": 2500, "vendors": [{"slug":"kmart", "country":"usa"}, {"slug":"walmart", "country":"usa"}] }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "3" } }
{ "category": "clothing", "sub_category": "belt", "active": true, "cost": 1800, "vendors": [{"slug":"kmart", "country":"usa"}, {"slug":"walmart", "country":"usa"}] }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "4" } }
{ "category": "clothing", "sub_category": "beanie", "active": false, "cost": 1100, "vendors": [{"slug":"kmart", "country":"ca"}, {"slug":"walmart", "country":"ca"}] }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "5" } }
{ "category": "office", "sub_category": "pen", "active": true, "cost": 120, "vendors": [{"slug":"kmart", "country":"usa"}, {"slug":"walmart", "country":"usa"}] }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "6" } }
{ "category": "office", "sub_category": "paper", "active": true, "cost": 500, "vendors": [{"slug":"korte", "country":"de"}, {"slug":"walle", "country":"fr"}] }
'
