# Performance methodology

Criterion benchmarks cover get_arc, get_ref, clone_context, initialize_one_type,
initialize_four_types and scope_one_type. Run `cargo bench -p app-context --bench
context`. Results depend on allocator, machine load, compiler and graph size.
No performance superiority, constant-time or zero-cost claim is made.

Context clone shares the Arc map. Borrowing hashes TypeId and safely downcasts;
shared retrieval also updates an Arc reference count. Scope creation copies the
map and clones each stored Arc (O(n) entries); scope depth does not affect lookup.
Building consumes the map without copying it into the Context. There are no locks
in the map, but values may require application-defined locks. Debug allocates a
sorted list of type names. Values and their maps allocate through std.

Local benchmark results and toolchain/platform evidence are recorded in
BENCHMARK_RESULTS.md after the benchmark run, without extrapolating to other hosts.
