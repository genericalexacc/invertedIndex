# InvertedIndex

Link to try it out: [inverted-index](https://inverted-index.fly.dev)


A rust implementation of an inverted index.
An http API is served by Actix.
It uses mutexes and threads to achieve higher throughput to the index and concurrent access.
From the front-end you can also index webpages by passing a url.

Stop words are removed at query time, and the querying is a naive df count.

I'm planning on adding an Okapi BM25 weighting scheme for ranked retrieval querying, and maybe some sentiment analysis or k-clustering.

