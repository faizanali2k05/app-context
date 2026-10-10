# Contributing

Read docs/design-decisions.md before API changes. Keep the default core dependency-free
and all unsafe code forbidden. Add regression tests for behavior changes, preserve
structured errors, and update documentation and CHANGELOG.md.

Run cargo fmt --all --check; cargo clippy --workspace --all-targets --all-features -- -D warnings;
cargo test --workspace --all-features; cargo test --workspace --doc --all-features;
cargo doc --workspace --all-features --no-deps; cargo build --workspace --examples --all-features.
CI also checks no-default-features, individual features, MSRV and three operating systems.
Compiler diagnostic snapshots are checked on stable; review changes before accepting
them with TRYBUILD=overwrite cargo test -p app-context --test ui --all-features.

Benchmark changes with cargo bench -p app-context --bench context. Release packages
must contain the full license texts and a verified repository URL. Never publish
from an unchecked or dirty release checkout. Follow the release procedure in
[`docs/releasing.md`](docs/releasing.md).

Contributions are accepted under MIT OR Apache-2.0, without an additional CLA.
