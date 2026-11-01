# E1 exact boundary-census implementation receipt

Date: 2026-11-01
Status: exact census core implemented and locally verified; protocol remains draft
Affected E1 data: none

## Outcome

The missing `tensor-census` core now contracts one explicitly selected side of
a canonical `BTNIR001` network, retains the cut edges as an exact integer
boundary tensor, and computes exact ranks of its balanced flattenings. The
implementation fails closed on malformed cuts, pinned cut edges, cap
exhaustion, malformed canonical predicates and inconsistent tensor shapes.

This closes the arithmetic instrument named as the immediate E1 implementation
step. It does not close the E1 experiment: the production semantic cut
generator, matched interior-rewired nulls, final ladder, fresh-process A/A
runner, preflight and frozen protocol are still open.

## Implemented boundary object

[`census.rs`](../../crates/hashed-ecdl-tensor/src/census.rs) takes a validated
canonical network and a sorted set of left-side vertices.

1. Edge identifiers remain in canonical construction order.
2. Edges crossing the selected vertex set are open tensor indices. A pin on
   one of those edges is rejected.
3. Pinned internal edges are fixed and free internal edges are enumerated.
4. Every admitted free assignment is checked against every selected canonical
   predicate, using the exact truth table or typed program bound into that
   predicate's semantic digest.
5. Completion counts are accumulated as arbitrary-precision nonnegative
   integers. The record carries support, assignments examined and a stable
   SHA-256 of the cut and count vector.

The caller supplies `max_free_edges`. Exceeding it is `Error::Limit`; it never
becomes a zero count or a censored success. The CLI also has a development
safety ceiling of 30 free edges.

## Exact flattening ranks

The canonical split alternates cut positions. At boundary width at most the
declared exhaustive cap, every distinct balanced split is also examined and
the minimum rank is reported. Each matrix is filled directly from exact tensor
counts.

Rank uses integer row elimination only. A pivot eliminates a lower row with
`pivot * row - factor * pivot_row`; dividing the resulting row by the gcd of
its entries preserves its rational row space while controlling coefficient
growth. The implementation records big-integer additions/subtractions,
multiplications, comparisons and row normalizations. No modular rank is
promoted to exact rank.

## CLI

[`tensor-census.rs`](../../crates/hashed-ecdl-tensor/src/bin/tensor-census.rs)
reads canonical IR bytes and a prospective cut:

```text
tensor-census BTNIR LEFT_VERTICES [MAX_FREE_EDGES] [EXHAUSTIVE_WIDTH]
```

`LEFT_VERTICES` is a strictly increasing comma-separated list. Output is one
`tensor-boundary-census/v1` JSON object carrying the canonical-IR and tensor
digests, exact ranks and counts, cut identities, operation counters and the
bounded claim scope. A caller can rerun the same bytes in a fresh process and
compare every exact field; the experiment runner that performs and records
that A/A comparison is not yet implemented.

## Verification

With Rust 1.78.0:

```text
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml -- --check
cargo test --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release census --lib
cargo check --manifest-path crates/hashed-ecdl-tensor/Cargo.toml \
  --locked --release --bin tensor-census
```

The four focused release tests pass:

- exact rank-one, rank-two and rank-four controls;
- exact contraction of a canonical equality network to the 2-by-2 identity;
- free-edge-cap and pinned-cut rejection;
- 1,152 deterministic integer matrices of sizes 1-by-1 through 6-by-6,
  cross-checked against independent rational Gaussian elimination.

Current stable Rust 1.99.1 all-target Clippy passes with warnings denied. Rust
1.78 all-target Clippy reaches an unrelated pre-existing `format_collect`
diagnostic in `panel_driver.rs`; no suppression or unrelated edit was made.
The complete crate and root-library gates are recorded separately in
[`VALIDATION.md`](VALIDATION.md).

## Evidence boundary and remaining gates

No production network, natural/null pair, rank ladder or wall time was
measured. In particular, this receipt does not establish low boundary rank,
full boundary rank, a solver improvement, a relation-oracle improvement or an
asymptotic result.

Before E1 freezes, the workstream still needs:

- stable semantic-region/cut metadata for every final production fixture;
- a degree- and predicate-preserving interior rewiring constructor;
- exact conditioned and unconditioned null selection;
- fresh-process A/A and known-rank control records from the same runner;
- disjoint preflight, final cap and ladder, power justification and hashes;
- a committed source revision and verified Iso Lab execution image.
