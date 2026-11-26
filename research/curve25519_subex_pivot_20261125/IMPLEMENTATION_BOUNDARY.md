# Symbolic versus live implementation boundary

The growing-arity result is an asymptotic producer audit. The current Rust
graph representation cannot execute its positive recursive branch on a
host-representable graph. This note records that boundary explicitly; it is
not an attack or runtime claim. `[pcc:unverified]`

## Native audit rows

| row | `lambda=ceil(log2(s))` | root `u` | result |
| --- | ---: | ---: | --- |
| native `usize` graph | at most `usize::BITS` | 0 | recursive branch unreachable |
| first symbolic active size | 131072 | 1 | numeric source guards pass; graph is not host-representable |
| uniform-history threshold | `2^26` | 512 | all source guards pass at five active depths; `u>=lambda^(4/5)` fails |
| first passing power-of-two scan | `2^480` | `2^463` | all registered source/proof invariants pass at 80 active depths |

The last row is the first passing `lambda=2^x` in the bounded scan
`0<=x<=520`, not the exact global threshold. `[pcc:unverified]`

The rows are emitted by native Rust:

```text
cargo run --locked --release \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin tensor-admission-audit

cargo run --locked --release \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin tensor-guards -- point 67108864

cargo run --locked --release \
  --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --bin tensor-guards -- scan 0 520
```

## Interpretation

`lambda` is the logarithm of the tensor graph's edge count, not the bit length
of the elliptic-curve field. Even the first nonzero source scale requires an
edge count with 131072 bits. The finite `u=1`, `u=2`, and `u=3` controls in the
suite deliberately force diagnostic parameters; they validate identities and
look for counterexamples but are not calls admitted by the live `Setup` path.
`[pcc:unverified]`

At `lambda=2^26`, the all-scale precision argument is already valid and every
numeric source guard passes. The stronger theorem invariant
`u>=lambda^(4/5)` is still false. Its eventual satisfaction is why the total
work statement is asymptotic rather than a finite security estimate.
`[pcc:unverified]`

The Curve25519 ledger remains a finite extrapolation of the conditional cost
formulas. It reports enormous costs and does not imply a practical discrete-log
computation. No executed test allocates the corresponding graph or enters a
positive production recursion at that scale. `[pcc:unverified]`

## Engineering consequence

A runnable positive instance would require a different representation: graph
sizes and parameter schedules must be host-independent, while graph objects
must remain implicit enough that `L`, packet arrays, and child descriptions do
not materialize at their symbolic cardinalities. Such an implementation would
need a fresh refinement and cost audit; merely replacing `usize` counters with
large integers would not make the construction executable.
`[pcc:unverified]`
