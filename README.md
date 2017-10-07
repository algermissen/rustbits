# rustbits
Rust examples from my journey to clarity.

## A Tokio futures stream that produces a sequence
The original inspiration of this was to zip a stream with an index sequence
to have a counter of stream items within the stream, as opposed to
maintaining a counter outside.

[Code](sequence-stream/src/main.rs)
