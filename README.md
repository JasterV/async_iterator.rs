# async_iterator.rs

This is a pet project that aims to provide  a set of Iterator implementations to iterate over a
sequence using async closures.
                                                                                                                                       
It is built on top of Tokio and it only uses tokio primitives for the implementation.
                                                                                                                                       
This is not meant to be a replace for libraries like [Rayon](https://github.com/rayon-rs/rayon), my knowledge on Rust can't compare to
the great minds behind libraries like that. This aims to be a very simple and small codebase to
suit my own needs.
                                                                                                                                       
Feel free to contribute to improve it if you want to also satisfy your own use cases. I will gladly review
and discuss any issues/PRs to make this a proper crate :)
