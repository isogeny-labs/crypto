# Qualified BK linear step and its Jordan exception

For every 2×2 matrix A over a field, a nonzero fixed vector and
`det A=q≠1` imply `A^k=I ↔ q^k=1` for every k. Lean derives
`trace A=1+q` and the characteristic polynomial `(X−1)(X−q)` from
those hypotheses. Coprime factors, actual polynomial evaluation and
Cayley–Hamilton supply the power criterion. No trace or diagonalization
premise is supplied. Whole gate passed 22 declarations with zero failures and zero trusted native
exceptions. Exact local **PCC-20261113-c139f1** replay passed at 2026-11-13T21:20:25Z.
Signed standing remains unverified; see [receipt](verification.json). [pcc:unverified]

The excluded case is checked directly. For `J=[[1,1],[0,1]]`,
Lean proves `J^k=[[1,k],[0,1]]` over every commutative ring. Over
Z/n this equals I exactly when n divides k. For every prime r,
J has determinant1 and a nonzero fixed vector, but `1^k=1` always
while `J^k=I` only at multiples of r. The closed certificate directly
negates the universal matrix claim obtained by deleting q≠1 while
retaining the determinant and nonzero fixed-vector hypotheses.
These are actual matrix identities. [pcc:unverified]

The source’s preceding rational-point input is also formalized for
actual Mathlib curve groups. The point type injects into `Option(K×K)`,
so it is finite over every finite field. If prime r divides its actual
point count, additive Cauchy supplies a nonzero rational point of
exact order r, killed by r. No elliptic torsion basis, Hasse bound
or asserted group cardinality replaces that construction. [pcc:unverified]

The generic elliptic bridges remain open: geometric E[r] rank2 in the
prime-to-characteristic case, Frobenius determinant q, transporting
the rational point into that representation, and torsion-field descent.
The characteristic-order rank case is also separate. This packet
proves the source’s linear step and rational input, not the general
elliptic BK theorem. Existing actual F7 all-extension counterexamples
are linked in the requirement map; J is not asserted to be their
computed Frobenius matrix. The qualified original source is not refuted.
[pcc:unverified]

Main: `BKLinearAudit.qualified_BK_linear_certificate` in
[LinearCertificate.lean](lean/LinearCertificate.lean). Lean4.35.0 and
Mathlib56a005855ea17e0b0958f73ec801ae140b0a1679 are pinned. Only standard
foundations; no holes, custom axioms, native evaluator, raised limits,
global elaboration flags or waivers. Failed/successful sources, logs
and exits are retained. No native experiment, paid compute or agent
upload. [pcc:unverified]

Sources checked: original Balasubramanian–Koblitz1998 Theorem1/proof
and Remarks, pp142–143, DOI10.1007/s001459900040. The original explicitly
excludes r dividing q−1 and describes the Jordan boundary. Existing
actual torsion/Frobenius projects distinguish their concrete proofs
from the generic geometric bridges. Pinned Mathlib matrix kernel,
characteristic polynomial, coprime factors, Cayley–Hamilton, actual
curve-point encoding and additive Cauchy APIs were reviewed.
KB unavailable; bounded primary/repository/library fallback, no
novelty claim. [pcc:unverified]

Reproduce from the repository root:
[pcc:unverified]

```sh
tools/lean/proof-gate.sh research/bk_linear_audit_20261113/lean
```

See the [fourteen-item map](../theorem_census_20261109/README.md#original-fourteen-requirements)
for all original requirements and remaining obligations. [pcc:unverified]
