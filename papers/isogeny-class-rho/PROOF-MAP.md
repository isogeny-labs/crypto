# Proof map, 2026-10-19

All project sources are in [lean/](lean/). The toolchain is
`leanprover/lean4:v4.34.1`; the Lake manifest pins Mathlib to
`d13f23b723b8a846827a245b89c10fc7d3f11612` (official v4.34.1) and pins all
transitive packages. No project theorem uses `sorry`, `admit` or a custom
axiom. Dependency pins and sources, rather than a moving branch, define
the checked statement. Public Mathlib cache artifacts were fetched only
for the matching pinned sources; `lake build` elaborated all new proofs.

| Paper statement | Checked theorem | Exact scope and open bridge |
| --- | --- | --- |
| Scalar transport (§2) | `ScalarTransport.mapNatMul` | Recursive natural multiplication is preserved by an explicitly zero/addition-preserving map. Does not construct a concrete elliptic-curve group or isogeny. |
| Answer reflection (§2) | `ScalarTransport.equalityReflected`, `answersReflected` | Additionally assumes injectivity. The actual isogeny restriction/kernel-degree argument and native arithmetic refinement remain open. |
| Square-factor cancellation (§4 dependency) | `ConductorSquares.cancelSquare`, `oddPowerSquare` | Generic field statement, with nonzero denominator/base and an explicit square/odd-power hypothesis. No concrete conductor certification. |
| Mod-8 proposition (§4) | `ConductorModEight.primeDivisorModEight` | For prime natural p with p≠2, odd natural m, integer t,f,D, p dividing f, and t²−4·2^m=f²D, proves p%8=1 or 7. Uses `ZMod.exists_sq_eq_two_iff`. The supplied discriminant and divisor hypotheses match the number-theoretic proposition; extracting certified t,f,D from the concrete curve/native records remains open. |
| Generic algorithm distribution (§2) | Not yet formalized | Requires transported oracle encodings and coupled randomness; the checked scalar lemma alone does not prove cost equality for arbitrary injective encodings, concrete rho hashes, or machine execution. |
| Characteristic-two Vélu map (Appendix A) | Not yet formalized/certified | Concrete map/domain/pole handling, subgroup homomorphism/injectivity, an identity-certificate record and native refinement remain required. Point sampling is not a substitute. |
| Koblitz off-crater degrees (§4) | Not yet formally connected | Requires certified conductor 703889=409·1721, ordinary/class-number and volcano hypotheses, and linkage to the exact recorded curve. Mod-8 alone allows smaller primes. |
| Statistical null / performance (§6) | Not a formal-proof consequence | Correct native statistics/covariance, exact producer/raw lineage and sealed experiment admission remain required. |

## Reproduction and axiom audit

```sh
cd papers/isogeny-class-rho/lean
lake build
lake env leanchecker --verbose ScalarTransport ConductorSquares ConductorModEight
```

Use the pinned Lean version. The build completed successfully (2900 jobs,
including cached dependencies and all new theorem elaborations).
`#print axioms` is in every source: `equalityReflected` has no axioms,
`mapNatMul` and `answersReflected` use `propext`, and the other three
theorems use only `propext`, `Classical.choice`, `Quot.sound`.
Those are Lean's standard logical axioms, explicitly reported here.
The versioned review receipts bind the exact source and manifest hashes.
Builtin `leanchecker` replays the new declarations against their imported
kernel environment; it is not the separately distributed external
`lean4checker --fresh`, nor a proof of the native executable.

No theorem in this map certifies a measured speedup or replaces the pending
research gates in [the review](REVIEW-20261019.md) and
[the problem record](../../problems/isogeny-class-rho-statistical-admission.md).
