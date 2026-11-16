# Conditional proof map

`Centering.lean` was checked with Lean 4.19 and mathlib revision `c44e0c8ee63ca166450922a373c7409c5d26b00b`. Exit status is zero; `LEAN.txt` reports only `propext` for each theorem.

- `mixed_center`: in an additive commutative group, separately supplied halves of the odd and 2-primary components combine to a half of their sum.
- `four_clears_translation`: a supplied translation of order dividing two disappears after fourfold addition.

The hypotheses are explicit. These proofs do not establish the geometric translation premise, the completeness of the rational 2-primary computation, the quotient correspondence, global map identity, or implementation correctness. Those remain separate claims and computational checks. No admitted proof is used.
