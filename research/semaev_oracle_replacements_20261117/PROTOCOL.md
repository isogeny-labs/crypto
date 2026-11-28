# Protocol: exact projection joins and compiled target functions for Semaev oracles

Status: frozen before any measured row. Study id
`EXP-SEMAEV-ORACLE-REPLACEMENTS-001`. This is a toy-scale oracle-stage
experiment. It does not assert an ECDLP speedup, a cryptographic-size result,
or a comparison with Pollard rho `[pcc:unverified]`.

## Question

Can either of these replace the Gröbner-basis decomposition call in a
prime-field index-calculus relation trial?

1. **Projection join.** Project every factor-base pair through
   `S_3(x_1,x_2,U)`. Join base-field roots by value and irreducible quadratic
   roots by their monic minimal polynomial. For four summands, use the balanced
   tree
   `S_3(x_1,x_2,U)=S_3(x_3,x_4,V)=S_3(U,V,x_R)=0`.
2. **Compiled target function.** For the four-summand oracle, compile the
   complete map from `x_R` to (a) existence and (b) the number of finite
   root-key matches modulo `p`. Test whether its unique degree-`<p`
   interpolant or a prefix-trained linear recurrence is substantially smaller
   than a table.

The first arm is exact but stores `O(B^2)` pair information. The second is a
falsifier for algebraic compression of that target-dependent online map; its
offline compilation cost is fully charged and it is not an online algorithm
unless the representation is small.

## Fixed implementation and controls

- Native Rust only:
  `src/cryptanalysis/semaev_projection.rs` and
  `examples/semaev_oracle_replacements.rs`.
- Curve: `y^2 = x^3 + 2x + 3` over `F_251`.
- Interval bounds: `8,16,24,32`; the actual factor base keeps only interval
  x-coordinates whose curve right-hand side is a square.
- Every target `0 <= x_R < p` is queried. Invalid curve x-coordinates remain
  in the compiler domain and must agree with the independent control (no
  relation).
- Independent exact control: construct both signs of every factor-base point,
  use only elliptic-curve group additions, and record the reachable target
  x-coordinates after three and four summands. It does not call a summation
  polynomial or inspect the projection table.
- Pair cancellation through the point at infinity is a required component,
  not silently lost by affine auxiliary variables.
- The interpolant is replayed at all `p` points. Berlekamp--Massey sees the
  first `floor(2p/3)` values; every remaining value is an untouched holdout.
- Headline units are exact counts: pair keys, distinct projected roots,
  quadratic classifications, hash lookups, interpolant degree/nonzero
  coefficients, recurrence order and holdout correctness. Wall time is not a
  result.

Command after the source/protocol commit is pinned:

```sh
cargo run --release --example semaev_oracle_replacements -- \
  --p 251 --a 2 --b 3 --bounds 8,16,24,32
```

Stdout is written once to `runs/frozen.jsonl`; stderr, commit id, binary hash,
host and toolchain are retained beside it. A failed correctness control fails
the study; its row is retained.

## Pre-registered predictions and falsifiers

- **P1 — exactness.** Both projection oracles have zero disagreements with the
  signed group-law control at every bound, and every returned tuple itself
  reaches the target under an independently enumerated sign choice. Any set or
  witness disagreement falsifies the implementation and no cost or compression
  result is accepted.
- **P2 — the projection cost remains a table cost.** The number of distinct
  finite pair roots grows with the factor-base size and the four-summand query
  performs one quadratic classification per distinct finite pair root. This
  arm is classified `SO-TABLE`, even if it is much cheaper than fresh F4.
  A sublinear exact representation plus sublinear query would falsify this
  disposition; a smaller constant does not.
- **P3 — no useful dense-polynomial compiler.** On every nonconstant
  compatibility-count row with factor-base size at least 8, the interpolant
  has degree at least `0.75p` and at least `0.50p` nonzero coefficients.
  A row below either threshold keeps the parametric-compiler lane open and
  triggers a disjoint-prime confirmation rather than a speedup claim.
- **P4 — no short target recurrence.** On the same rows, the recurrence order
  is at least `0.20p`, or it predicts fewer than 90% of the untouched suffix.
  Order below `0.20p` with at least 90% holdout correctness keeps the lane open
  and triggers exact rational reconstruction on a disjoint prime.
- **P5 — saturation is not compression.** A constant four-sum existence map
  may have degree/order zero once relations cover every target. It is reported
  but cannot satisfy P3/P4; the compatibility-count map is the decisive one.

## Decision rule

- P1 fails: fix the implementation under a dated amendment and rerun every
  row; do not interpret other fields.
- P1 holds and P3/P4 hold: retain the projection join as a correct engineering
  baseline, close dense interpolation/short recurrence for this ladder, and
  record that its `O(B^2)` table does not supply the required non-enumerative
  `S_5` oracle.
- P1 holds and either P3 or P4 fails: do not claim a win. Freeze a second
  protocol with disjoint primes/curves and compare the compressed evaluator,
  including compilation and verification, against the strongest current
  native decomposition route.

## Inadmissible changes

- Dropping invalid-target rows, cancellation cases, or a factor-base bound.
- Learning the recurrence on any holdout value.
- Replacing the compatibility-count map with the saturated Boolean map for a
  favorable compression verdict.
- Excluding the pair-table build or verification from any later end-to-end
  cost.

## Amendments

1. **2026-11-17, before any measured run.** P1 now checks each returned tuple,
   not only the set of targets for which a witness exists. The signed group-law
   control enumerates the signs of that exact tuple and requires its sum to have
   the requested x-coordinate. This closes the case where an incorrect witness
   could be hidden by a different valid tuple for the same target. No input,
   ladder, cost metric or compression threshold changed.
