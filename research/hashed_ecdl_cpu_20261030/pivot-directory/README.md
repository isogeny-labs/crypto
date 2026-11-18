# Contiguous pivot directory

The dependency reducer now keeps its ordered (column, provenance-index)
directory in a sorted vector. Every incoming row traverses this directory;
the vector removes tree-node traversal from that loop. Pivot tails, elimination
arithmetic, normalization, first-dependent-row selection and reverse provenance
reconstruction are unchanged. Provenance indices remain in source insertion
order, independent of sorted column order.

This is a candidate locality improvement, not a measured speedup. Inserting an
out-of-order pivot shifts later directory entries, so worst-case directory
insertion work changes from logarithmic to linear per pivot. The tradeoff must
be measured on the full pipeline and relevant matrix sizes; no latency or
complexity improvement is claimed. The vector grows on demand rather than
allocating the maximum matrix width before an early dependency.

References reviewed on 2026-10-30: the retained tree-based eager elimination
test, the streamed-matrix source/notes, and Rust's documented
[Vec insertion](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.insert)
and [partition_point](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point)
contracts. No new mathematical algorithm is introduced.

## Validation

All 46 standalone release library tests pass. The independent eager reference
still uses BTreeMap and full provenance vectors. Its exact-coefficient comparison
now includes 234 additional matrices with reversed/rotated pivot arrival across
three primes and widths 1 through 12. These force insertion before and between
existing pivots while retaining source-order provenance. Existing random-density
comparisons, streamed incidence tests, and archived full certificate equality
also pass. Strict all-target release Clippy, formatting and complete-example
build pass; raw logs and source hash are retained here.

Commands use the offline locked crates/hashed-ecdl-cpu/Cargo.toml manifest and
CARGO_TARGET_DIR=/tmp/cpu-projective-relations-target: release library tests,
all-target release Clippy with -D warnings, release complete-example build and
cargo fmt --check. The raw test log retains cargo's trailing blank line.

## Session handoff, 2026-10-30

### What was done

T-1041 implemented and checked the compact directory on parent
2f08282ec3f5ee9b16f61795421e4871a1282800. The previous turn delivered projective
replay equality; current-head hosted CI was still running when this work began.

### Decisions made

Preserve round20's frozen bundle and original baseline. Do not include this
candidate in its previously frozen component contrasts. No benchmark, priced
performance row or new cloud capacity was created by this software work.

### Open threads

Round20 awaits eligible existing capacity for its already approved single
attempt. The occupying semaev-n47-rand02-d5-blocked-20261030-2120 job was checked
live and remained Running. Measure this candidate separately before claiming
an end-to-end gain; the 10x objective remains active.

### Errors

No software validation failed. Clippy waited for the test build's Cargo lock.
The shared dated session log remains reserved by T-828; this scoped record
preserves the handoff without editing that owner's file.
