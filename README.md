This repo demonstrates an issue with rust proc macros failing
to round-trip spans.

If you run `cargo check` in the crate root, you should see two
debug-printed token trees that are exactly the same as one
another. But if you run `rust-analyzer diagnostics .`, you
should see two debug-printed token trees that differ, in that
the second one has Groups with nonsense spans.