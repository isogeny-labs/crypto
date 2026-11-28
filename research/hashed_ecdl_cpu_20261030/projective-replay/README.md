# Direct projective equality during certificate replay

For curves without a bounded inverse cache, certificate relation replay now
compares the Jacobian tuple sum directly with the independently reconstructed
Jacobian target. It previously normalized the target to affine coordinates
before comparing it with the projective tuple sum. This removes one field
inversion for each finite target in each replay. Cross multiplication adds
field products; measured latency is still pending. Cached-field replay retains
its affine lookup path.

For finite Jacobian points, equality requires both X1*Z2^2=X2*Z1^2 and
Y1*Z2^3=Y2*Z1^3. Identity is handled separately. The
[EFD Jacobian definitions](https://www.hyperelliptic.org/EFD/g1p/auto-shortw-jacobian.html)
were reviewed on 2026-10-30. This is an application of standard coordinate
equality, not a new group algorithm. The existing projective-compare candidate
and projective-relations implementation informed the change.

All relation point, scalar and base-membership checks remain before comparison.
Each complete replay still rebuilds its own factor bases and fixed-base tables,
checks every relation and dependency, and checks the recovered scalar. Collection
still requests affine targets and retains its existing operation order. No
state is shared between the internal and independent replays, and the certificate
format and first-witness selection are unchanged.

## Software validation

- 46 standalone release library tests pass, including archived complete
  certificate byte equality and the original independent group arithmetic tests.
- New tests compare fixed-pair projective targets with independent affine scalar
  multiplication on the uncached small group and the certified 40-bit fixture.
  They exercise zero and boundary scalars, identity, non-unit scales, negative-y
  points, and rejection after corrupting a relation's u coefficient while
  retaining valid base members.
- Strict all-target release Clippy, formatting and the complete-example build pass.

Commands used the offline locked standalone manifest
`crates/hashed-ecdl-cpu/Cargo.toml`, with
`CARGO_TARGET_DIR=/tmp/cpu-projective-relations-target`: `cargo test --release
--lib`, `cargo clippy --release --all-targets -- -D warnings`, `cargo build
--release --example complete`, and `cargo fmt --check`. Logs are retained here.
No root crypto or CUDA build was performed.

## 2026-10-30 session handoff

### What was done

Implemented projective replay equality under T-1039, based on parent
5b37f957e00d2347210f9bcadaeeac09a895ce8f. Rechecked the occupied EKS worker:
job semaev-n47-rand02-d5-blocked-20261030-2120 remained Running. Round20's
authorized single attempt remains unsubmitted; its frozen bundle is unchanged.

### Decisions made

Continue local CPU software work while eligible existing capacity is unavailable.
Keep this candidate outside round20 to preserve its frozen comparisons. No new
benchmark rows or dashboard performance claim arise from these unit tests.

### Open threads

Run the already authorized round20 attempt when eligible capacity is available.
This additional candidate needs a separate frozen comparison before attributing
latency improvement to it. The full 10x goal remains active.

### Errors

The shared dated session log is reserved by T-828. Preserve this scoped handoff
here rather than editing another owner's file. No software validation failed.
