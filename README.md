# rustbits
Rust examples from my journey to clarity.

## A Tokio futures stream that produces a sequence
The original inspiration of this was to zip a stream with an index sequence
to have a counter of stream items within the stream, as opposed to
maintaining a counter outside.


    let seq1 = SeqStream::new(0);
    let seq2 = SeqStream::new(0);
    
    let s = seq1.zip(seq2).for_each(|(a, b)| {
        println!("Seq: {},{}", a, b);
        Ok(())
    });
    
    let mut core = Core::new().unwrap();
    core.run(s).unwrap();


[Code](sequence-stream/src/main.rs)
