This is the fuzzing harness that I've used to discover a [memory disclosure vulnerability](https://github.com/ruuda/claxon/issues/10) in [Claxon](https://github.com/ruuda/claxon), when combined with [libdiffuzz](https://github.com/Shnatsel/libdiffuzz). All it does is validate that decoding the same file twice produces identical results.

It uses [AFL.rs](https://github.com/rust-fuzz/afl.rs), see [Rust Fuzz Book](https://fuzz.rs/book/afl.html) for a quick start.
