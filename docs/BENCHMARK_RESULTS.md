# Local benchmark results

Run on 2026-10-10, Linux x86_64, Rust 1.99.0, optimized Cargo bench profile.
Command: cargo bench -p app-context --bench context --offline -- --sample-size 20
--warm-up-time 1 --measurement-time 1.

| Operation | Criterion estimated time interval |
| --- | --- |
| get_arc | 46.897–71.312 ns |
| get_ref | 46.421–62.735 ns |
| clone_context | 22.251–32.402 ns |
| initialize_one_type | 209.07–305.65 ns |
| scope_one_type | 100.77–132.39 ns |
| initialize_four_types | 505.61–599.70 ns |

Other compilation and checks ran on the same machine, so these are a smoke
measurement, not a performance guarantee or a comparison with other libraries.
Several cases contain outliers. Repeat on an otherwise idle target machine with
longer samples before making application-specific decisions. Raw Criterion data
is local under target/criterion and is excluded from crate packages.
