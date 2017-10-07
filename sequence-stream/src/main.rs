extern crate futures;
extern crate tokio_core;

use futures::{Stream, Async, Poll};
use futures::future::Ok;
use tokio_core::reactor::Core;

/// Helps us to make the sequence generic over the incrementable
/// type.
///
/// Incrementable is copied from the excellent Stackoverflow answer
/// https://stackoverflow.com/a/41671697/267196
///
trait Incrementable: Copy + std::ops::AddAssign<Self> {
    /// Generic incrementaton step, eg 1 for i32
    fn one() -> Self;

    /// Increment by one after use of value
    fn post_inc(&mut self) -> Self {
        self.post_inc_by(Self::one())
    }

    /// Increment by n after use of value
    fn post_inc_by(&mut self, n: Self) -> Self {
        let tmp = *self;
        *self += n;
        tmp
    }
}

/// A macro helps to implement it for numeric types
macro_rules! impl_Incrementable{
    ($($m:ty),*) => {$( impl Incrementable for $m  { fn one() -> Self { 1 as $m } })*}
}
impl_Incrementable!{u8, u16, u32, u64, i8, i16, i32, i64, f32, f64}



/// A stream that produces a sequence of numbers
pub struct SeqStream<T> {
    next: T,
}

impl<T: Incrementable> SeqStream<T> {
    fn new(start: T) -> SeqStream<T> {
        SeqStream { next: start }
    }
}

/// The implementation of the Stream trait uses the generic
/// post_inc() to produce the next value.
impl<T: Incrementable> Stream for SeqStream<T> {
    type Item = T;

    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let v = self.next;
        self.next.post_inc();

        Ok(Async::Ready(Some(v)))

    }
}


fn main() {
    println!("Streaming sequence numbers");
    let seq1 = SeqStream::new(0);
    let seq2 = SeqStream::new(0);

    let s = seq1.zip(seq2).for_each(|(a, b)| {
        println!("Seq: {},{}", a, b);
        Ok(())
    });

    let mut core = Core::new().unwrap();
    core.run(s).unwrap();
}
