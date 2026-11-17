# Instance gate and formalization scope

The full elliptic divisor/jet/native equivalence is not formally established
here. Two finite matrix-certificate endpoints and the initial assembly probe
use existing Mathlib operations and a pinned checking context.

1. Structure above: Matrix(Fin6)(Fin6)(ZMod101), existing mulVec,determinant
and ring/vector operations. The geometric theorem also needs divisors,
sections,jets and Pic^0(E); those contracts are not replaced by assumptions
that encode the desired conclusion.
2. Theorem used as stated: Matrix.eq_zero_of_mulVec_eq_zero from
Mathlib.LinearAlgebra.Matrix.Nondegenerate. A nonzero kernel vector
contradicts a nonzero determinant. Matrix/ring laws are not re-proved.
3. Instance: toyMatrix and toySection are literal values copied from the
qualified native case Fp101-multiplicity-6-0-0-target-6 in LEAN_INSTANCE.json.
The source and full native JSON hash are pinned separately. The numeral101
primality instance is checked by decide; no actual25519 primality proof is
borrowed from it.
4. Assembly probe: the original abstract6x6 implication type-checks once
the named dependency module is built. Actual instance: toy_section_kernel
checks M*c=0 and c!=0 by kernel reduction; toy_section_det_zero applies the
existing theorem to that concrete certificate. No native_decide or increased
limits. The final focused gate/axiom records, when complete, are in lean-gate.
5. Missing: divisor/Riemann--Roch and Pic^0 specialization; kernel=L(D-Z);
local section/jet semantics; Rust field/series/matrix conversion; original
raw membership/index/sign/cofactor binding; unknown-column discovery and
complete work/yield/family bounds. Native agreement and the two concrete
matrix endpoints do not discharge them.
6. Inheritance: all field,matrix,vector and determinant operations are
Mathlib's. The finite numeral and matrix literals are instance obligations.
The determinant endpoint is one existing-law application after restructuring
by contradiction. No generic law is copied into the study.

| Declaration | Kind | Scope |
|---|---|---|
| SectionCertificate.toy_section_kernel | Finite instance obligation | One literal Fp101 native matrix/section, no geometric binding |
| SectionCertificate.toy_section_det_zero | Endpoint | Existing matrix law at that literal instance |
| Full geometric/native theorem | Missing | Not represented by either endpoint |

Pinned Lean4.35.0,CryptoLean5f2ca251cb34806f69fd5455175434cac3567dfa,
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Bounded library search
did not assemble the required geometric bridge; this is not proof of global
library absence. Source .lean and project configuration participate in the
ordinary focused proof gate, not a renamed or exempted source.

First failures are retained: missing cached module; a packaging command with
a wrong working directory stopped before source writes; incorrect notation
module path; missing ZMod field import. The failed elaboration log mentions
sorryAx from an unsolved theorem; it is explicitly failed evidence, not a
passing proof. Final source has no hole,axiom,exception or raised limit.
Native source9,qualifier,PRECALL and results stayed unchanged through this
separate Lean project addition.
