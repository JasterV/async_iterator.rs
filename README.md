# async_iterator.rs

This is a pet project that aims to provide  a set of Iterator implementations to iterate over a
sequence using async closures.
                                                                                                                                       
It is built on top of Tokio and it only uses tokio primitives for the implementation.
                                                                                                                                       
This is not meant to be a replace for libraries like [Rayon](https://github.com/rayon-rs/rayon), my knowledge on Rust can't compare to
the great minds behind libraries like that. This aims to be a very simple and small codebase to
suit my own needs.
                                                                                                                                       
Feel free to contribute to improve it if you want to also satisfy your own use cases. I will gladly review
and discuss any issues/PRs to make this a proper crate :)

Well I just realized that an async iterator at the end of the day is just a Stream, and the work I was trying to implement in here like running concurrent tasks with a limit on an iterable type is already implemented in [here](https://docs.rs/futures-util/0.3.28/futures_util/stream/trait.StreamExt.html#method.for_each_concurrent)

So this crate will be discontinued thank you all :D
