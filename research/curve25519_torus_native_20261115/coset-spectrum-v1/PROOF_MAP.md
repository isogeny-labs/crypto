# Instance gate and fidelity map

This is an assessment, not a new solver. All producer claims remain unsigned.
The actual compiler's frozen54 inputs and its evidence are unchanged.

## Before the first Lean edit

1. **Structure above:** Mathlib `Matrix (Fin 2) (Fin 2) (ZMod 101)`, its
   multiplication, trace, determinant and characteristic polynomial. This is
   a concrete finite instance, not a new matrix or curve structure.
2. **Theorem used as stated:** `Matrix.charpoly_fin_two`,
   `Matrix.trace_fin_two`, `Matrix.det_fin_two`; entrywise decidable equality
   closes the finite matrix obligations. Existing eigenvalue laws are inherited.
3. **Object:** `!![0,1;0,0]` and the zero matrix. These are existing Mathlib
   matrices. The nonzero nilpotent is the multiplication-by-X matrix of
   F101[X]/X^2 in the reversed monomial basis.
4. **Assembly probe:** check the concrete characteristic polynomial is X^2,
   the matrix square is zero, and its (0,1) entry is nonzero. Compile the
   probe before the final audit. Field arithmetic uses ZMod's existing ring;
   no primality assumption is needed for these three finite identities.
5. **Missing components:** the full degree6 quotient, localized lambda,
   Chebyshev operator, characteristic-polynomial equations, bit ranges,
   original native compiler and group replay are NOT instantiated by this
   two-dimensional example. The full equivalence in REVIEW.md stays a
   written argument. No native/full Curve25519 theorem is inferred.
6. **Inheritance:** see the table below. No missing generic law is needed
   for this finite counterexample; no crypto-lean source edit is planned.

| Operation | Inherited object/law |
| --- | --- |
| Coefficient arithmetic | `ZMod 101` commutative-ring instance |
| Nontrivial coefficients | existing ZMod instance, `Fact (1 < 101)` by finite `decide` |
| Matrix arithmetic | existing `Matrix` instances on `Fin 2` |
| Trace | `Matrix.trace_fin_two` |
| Determinant | `Matrix.det_fin_two` |
| Characteristic polynomial | `Matrix.charpoly_fin_two` |
| Zero/nonzero matrix entries | kernel-checked finite `decide` |

## Dependencies and proof chain

First instantiate trace/determinant and use the existing characteristic-
polynomial formula. Separately discharge square-zero and nonzero by finite
matrix arithmetic. Their conjunction is the counterexample to the proposed
general inference from matching spectra to annihilation; it does not assert
that this matrix is a complete original section fixture.

Written semantic chain: original membership implies S squarefree; after
localizing 2X+A, form g=T_q(lambda)^2. Cayley--Hamilton and a polynomial
divisibility chi_g | H^6 imply H(g)^6=0. Reducedness then implies H(g)=0.
Conversely H(g)=0 and reducedness yield a six-value eigenvalue multiset
selected from H's roots, by the classical eigenvalue theorem. Original
rational norm, torsion and indexed-row arguments are retained separately.

## Statement and fidelity table

| Claim | Kind and owner | Formal strength / missing bridge |
| --- | --- | --- |
| Two-dimensional matching-spectrum counterexample over F101 | finite instance obligation, `SpectrumAssembly` | Exactly the finite matrix identities; not an actual section/compiler instance |
| Guarded six-dimensional spectral membership equivalence | written application of classical algebra in REVIEW.md | No Lean quotient/Chebychev/localization or original native binding yet |
| O(log q + log k) arithmetic-circuit construction at fixed arity | written proposed construction | No compiled gate count, solver bound or Lean cost theorem |
| Solver quotient dimension is not fixed to six | distinction between two explicitly defined algebras | No numerical D, zero-dimensionality or degree estimate asserted |
| Conditional relation-phase budget | model calculation in REVIEW.md | Assumptions and other attack phases remain obligations |

Actual pinned dependency: Mathlib cfc55fc58714a59dc06e6e79edc6d1d95140d3c2,
Lean4 v4.35.0. The cache host is the unchanged crypto-lean checkout5f2ca251,
not a logical dependency of this Mathlib-only module. The exact Lake manifest
and source are retained in lean/. No generic law or crypto-lean edit was needed.

Maintained focused file gate: PASS,3 theorems,0 trusted-native exceptions.
The final printed axioms for all three endpoints are propext,
Classical.choice and Quot.sound. Receipts are in lean-gate/. The containing
Lake target also builds; the focused file mode checks this one source and
avoids an unrelated full-Mathlib cache fetch. No gate source or policy was
changed. lean4checker is unavailable. This is local kernel/gate evidence;
signed PCC standing remains unverified. [pcc:unverified]

The exact declaration bindings are `SpectrumAssembly.toy_charpoly`,
`SpectrumAssembly.toy_square_zero` and `SpectrumAssembly.toy_nonzero`, in
PCC-20261123-8d8628. The statement is exactly the concrete ZMod101 matrix
identities, with no extra conclusion about a full native section.

Initial failures are retained under development/: a workdir setup error
caused an interrupted fresh-cache fetch; a guessed Mathlib.Tactic.Decide
import did not exist; the first actual matrix proof needed Fact(1<101).
That failed elaboration printed sorryAx and was NOT accepted. The corrected
source supplies the finite fact, has no proof holes and passes the gate.
A later letI style suggestion was resolved using let before final checking.
No limits, custom axioms, native_decide or kernel bypass were added.

## KB and retrieval receipt

No research-kb MCP tools are loaded. The exact maintained wrapper reports
exit127, no executable. Intended queries: `Curve25519 finite-free coset
spectrum`; `characteristic polynomial nilpotent membership false positive`;
`symmetric summation polynomial coefficient solver`; `FGLM quotient dimension
multiplication matrices`. Repository derivations and primary papers replace
the unavailable retrieval. Lead-atlas local checkout is absent and its
connected README request returned no readable content; no novelty clearance
or exhaustive prior-art claim follows.
