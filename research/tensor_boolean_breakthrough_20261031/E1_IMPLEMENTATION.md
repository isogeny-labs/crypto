# E1 exact boundary-census implementation receipt

Date: 2026-11-01
Status: exact census core implemented and locally verified; protocol remains draft
Affected E1 data: none

> Follow-up: the initial receipt below is historical. Later on 2026-11-01 the
> semantic cut, equality quotient and typed rewiring diagnostics were
> implemented and run under separately frozen screens. See
> [`../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md`](../prime_ic_elimination_20261101/RANK1_QUOTIENT_RESULT.md).
> The full E1 protocol remains draft and no full-E1 row was promoted.

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

## 2026-11-01 follow-up boundary

The production encoder now binds semantic slot regions into canonical
predicate parameters and assigns equality/fanout wiring to the producer
region. A shape preflight showed why the original contractor cannot run the
declared object: F5 has raw cut width 154 and free enumeration exponents
15,959--31,918.

An exact functional quotient maps the raw cut copies back to seven typed
signals on F5 and 9--15 signals on four held-out fields. Exact raw balanced
ranks are computed from the quotient without materializing zero copy rows or
columns; exhaustive tiny controls validate the symmetry reduction. A
support-matched sparse null advanced, but a frozen one-pair typed topology null
returned the same conditioned rank in all four fields and rejected that
minimal global-wiring mechanism.

This follow-up does not satisfy the draft protocol's arbitrary all-interior
rewired null. Such a null destroys the parsimonious program and needs an exact
projected model counter. Binary/image/power/final-ladder fields therefore
remain open and the protocol remains draft.
