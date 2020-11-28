# delog-examples

While it's possible to have multi-file examples, the usual examples mechanism
in `cargo` does not support defining features (we need to set a subset of the `log-*`
features to see output from the "local" logging macros), not requiring features
(for convenience, we use the `std-example-flushers` feature).

So this is a "library" that sets things up, apart from that the examples here
have the same role as normal examples for `delog` would have.
