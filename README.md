# Elastic Lens

> An opinionated framework to work with Elasticsearch.

## About

Simple DSL framework to build Elasticsearch requests as well
as parse the responses back.  There is a **STRONG** chance not
every piece of functionality is available so read the docs and
make sure it's the right fit for you.

This project is in it's infancy and is currently supporting a
real work project.  This is what is driving it's development
for now; however, if you have suggestions or edits please feel
free to open an issue :+1:.

## Playing with the Examples

There is a bin scripts under `bin/` to get started.  It does
expect a wide-open Elasticsearch running locally on the default
port of 9200.  If you don't have that either modify the scripts
or setup an instance.

I personally use docker:

```bash
docker run -it -p 9200:9200 -e "discovery.type=single-node" elasticsearch:7.11.2
```

The examples are found in the `examples/` directory and can be
run with cargo: `cargo run --example <name> (..args..)`  Here is
current list of examples:

- `fetch_a_document`
