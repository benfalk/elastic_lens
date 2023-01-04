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
      }
    }
  }
}'

curl -XPOST -H "Content-Type: application/x-ndjson" http://localhost:9200/_bulk -d '
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "1" } }
{ "category": "clothing", "sub_category": "t-shirt", "active": true, "cost": 1300 }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "2" } }
{ "category": "clothing", "sub_category": "pants", "active": true, "cost": 2500 }
{ "index": { "_index": "inventory", "_type": "_doc", "_id": "3" } }
{ "category": "office", "sub_category": "paper", "active": true, "cost": 100 }
'
