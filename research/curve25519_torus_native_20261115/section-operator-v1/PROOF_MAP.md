# Instance gate and formalization boundary

Read lean-instance-gate before this source. This round does NOT claim a
formal proof of the elliptic-section equivalence or native jet compiler.
The central divisor argument is readable in DERIVATION.md and classical in
the cited elliptic-code literature. A matrix assembly probe checks only the
existing linear-algebra implication; it is not promoted to a new theorem.

1. Structure above: Mathlib Matrix(Fin6)(Fin6)K and FieldK, existing mulVec,
   determinant and zero vector. Geometric objects would additionally require
   rational divisors, line-bundle sections, jets and Pic^0(E) equivalence.
2. Existing law used as stated: Matrix.eq_zero_of_mulVec_eq_zero in
   Mathlib.LinearAlgebra.Matrix.Nondegenerate. A nonzero vector in the kernel
   contradicts a nonzero determinant. No matrix inverse or determinant law is
   re-proved. The full geometric bridge has not been assembled from a pinned
   library in this round; do not replace it by an assumed conclusion.
3. Object: abstract6x6 evaluation matrix M over a field; c is a nonzero
   candidate coefficient vector. The Rust BigUint matrix is not automatically
   a value of this formal object. Conversion/field/jet soundness is missing.
4. Probe: AssemblyProbe.lean applies the library implication by contradiction
   with M,c explicit. It also prints the existing theorem's axioms. A successful
   probe is only type checking of this applicability, not a new proof-gate
   receipt or independently signed theorem.
5. Missing generic components: divisor/Riemann--Roch and Pic^0 specialization
   to this curve, evaluation-kernel=L(D-Z), local section/jet semantics,
   Rust field/series/matrix correspondence, raw member/index/group provenance,
   and root-discovery/cost/yield/family bounds. They belong in the shared
   library before a study endpoint could invoke them. Native comparison with
   group addition does not fill them.
6. Inheritance: determinant, mulVec and vector operations are Mathlib's;
   six only instantiates the matrix index type. FieldK is an explicit contract,
   not an unproved actual-primality assertion. No composite is unfolded, limits
   raised, native_decide exception, custom axiom or proof hole added.

| Component | Status | Owner |
|---|---|---|
| Nonzero null vector implies zero determinant | Existing Mathlib law; assembly probe only | Mathlib |
| Classical degree-zero divisor and group-sum equivalence | Readable derivation, not formalized here | Shared algebraic geometry library |
| Native local jets and section evaluation | Fixed control comparison, subject to qualification | Rust component |
| Find unknown singular six-column configurations subcubically | Open | Structured relation search |

Pinned checking context: CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2,
leanprover/lean4:v4.35.0. Bounded source search found no assembled
Riemann--Roch/Pic^0 bridge in CryptoLean or the inspected Mathlib algebraic
geometry files. This is a search gap, not proof of global library absence.
No new named theorem or formal geometric endpoint is claimed, so the ordinary
nonempty-theorem proof gate is not reported as passing for this probe.
