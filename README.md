# InvertedIndex 🗂️

Link to try it out: [inverted-index](https://inverted-index.fly.dev)


A rust implementation of an inverted index.

- Ranked retrieval through okapi bm25.

- An http API is served by Actix.

-  Mutexes and threads to achieve higher throughput to the index and concurrent access.

- From the front-end you can also index webpages by passing a url.

- Stop words are removed at query time, and the querying is a naive df count.

