- unused_mut warning after refactoring
- mut != mut
- comparison of binding via self, function arguments, and let
- self == self: Self

NOTE: There are four versions. Use the `--bin` option for `cargo run` to specify which to run.
```
cargo run --bin self        # uses methods (some steps have no changes)
cargo run --bin self-long   # uses methods without abbreviating self pattern
cargo run --bin function    # uses functions
cargo run --bin block       # uses blocks
```
