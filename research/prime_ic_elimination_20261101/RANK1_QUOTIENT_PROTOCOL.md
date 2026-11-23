# Frozen rank-1 equality-quotient held-out screen

Frozen: 2026-11-01, after the F5 development row and before any listed
held-out cell

Status: exact natural-network structural screen; no rewired-null or speed claim

## Question

At the semantic mid-slot cut of the actual hashed decomposition encoder, can
the many raw fanout edges be quotient by their typed source signals while
preserving the exact raw balanced-flattening ranks?  If so, does the resulting
natural rank remain materially below the maximum balanced quotient dimension
as the prime-field bit width grows?

The F5, `m=2,3,4` output is development evidence and is excluded from this
held-out decision.  This screen decides only whether implementing an exact
matched-null path is justified.  It cannot decide E1 without that null.

## Frozen held-out fixtures

Each cell has two slots, so the primary cut is after slot one.  The factor-base
threshold is `floor(s/2)`, both slots use seeds `(alpha,beta)=(2,1),(3,4)`, and
the target is the listed generator.

| field bits | `p` | `a` | `b` | prime order `r` | generator | hash prime `s` | threshold |
| ---: | ---: | ---: | ---: | ---: | --- | ---: | ---: |
| 4 | 11 | 1 | 5 | 11 | `(0,4)` | 23 | 11 |
| 5 | 17 | 1 | 3 | 17 | `(2,8)` | 37 | 18 |
| 6 | 37 | 0 | 5 | 37 | `(6,6)` | 79 | 39 |
| 7 | 67 | 0 | 2 | 73 | `(2,12)` | 137 | 68 |

`Curve::new` must independently certify every field, curve, full prime order,
and generator. `FactorBase::new` must independently certify every hash prime.

## Exact method and controls

For each cell:

1. Build the typed semantic-region network and canonical IR twice.  Every IR
   hash, cut, class, tensor value, support and rank field must match exactly.
2. Enumerate only the left slot's free source bits with the live parsimonious
   program, and replay every selected graph predicate, including equality
   trees, on every accepted assignment.
3. Group raw cut edges only when the encoder metadata maps them to the same
   exact signal.  Arbitrary or rewired graphs without trusted metadata fail.
4. Compute the exact rank of every balanced raw split up to equality-class
   symmetry: each class is column-only, split, or row-only.  A class pattern is
   admitted exactly when its possible row-edge interval contains half the raw
   width.
5. Independently enumerate the first factor base.  Quotient support must equal
   its point count, and every nonzero completion count must be one.

Before held-out rows, the symmetry rank path must match exhaustive raw-split
rank on all 48 registered synthetic tensors, and the functional quotient must
match direct raw-edge contraction on the fanout control. Removing functional
metadata must be rejected.

## Frozen reading

For quotient width `q`, define the maximum balanced matrix dimension as
`2^floor(q/2)` and the rank fraction as `best_raw_rank / maximum_dimension`.

- **advance to matched-null implementation** if all controls pass, all four
  cells have quotient width at most 15, and every rank fraction is at most
  `0.5`;
- **reject this quotient as a practical exact diagnostic** if any cell exceeds
  quotient width 20 or at least three cells have rank fraction above `0.9`;
- otherwise report **inconclusive**.

Also report the ordinary least-squares slope of `log2(best rank)` against field
bit width, but it does not change this gate.  Advancing says only that the
natural side is exactly compressible enough to justify an independently exact
rewired-null backend.  It is not a natural/null effect, asymptotic result,
contraction speedup, relation-oracle speedup, or ECDLP claim.
