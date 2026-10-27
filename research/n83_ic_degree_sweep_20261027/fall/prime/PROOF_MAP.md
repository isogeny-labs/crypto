# Formal proof status

The full first-fall formula in ARGUMENT.md is a written producer derivation with exact small-matrix controls and explicit target certificates. It is **not fully formalized**. The overall Lean theorem remains open.

| Obligation | Lean declaration / evidence | Status and missing bridge |
|---|---|---|
| A square-zero remainder transfers Lg=gr to L²g=0 | PrimeFirstFall.square_zero_remainder_annihilates in lean/Annihilator.lean | Checked, no axioms. Assumes associative multiplication, right zero, Lg=gr and r²=0 explicitly. |
| Pair-product construction gives Lg=gr in the actual selector quotient | Native pair identities and written upper-bound argument | Not formalized; quotient-ring product induction and nonzero square-free coefficient needed. |
| First nontrivial syzygy at h+4 | Native factored witness and specialization; ARGUMENT.md | Not formalized; actual Koszul complex, its boundary ideal and specialization map needed. |
| No nontrivial syzygy below h+4 | Phuong–Tran Theorem2.7, finite filtered Koszul argument, small explicit matrices | Not formalized; graded selector algebras, Lefschetz string decomposition, tensor-product first homology and filtered lifting are missing. |
| Earliest new affine degree fall equals this value | No such claim | Unresolved; a nonzero lower-degree product can already lie in a lower V(F,d). |

Checked command: `cd lean && lean Annihilator.lean`, Lean4.34.1, dependency only bundled Std. Full output and `#print axioms` result are in validation/lean-build.log. `lean4checker` is not installed. Searches found no reusable installed mathlib checkout at the checked existing-project dependency paths; the main formalization gap is the missing structural development, not an unavailable theorem asserted as an axiom. No sorry, admit, custom research axiom or native_decide is used. This one dependency is not a Lean proof of the complete first-fall formula.
