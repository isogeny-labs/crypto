# Proof map

All mathematical deductions in REPORT.md are unsigned [pcc:unverified].

Round03 adds PCC-be4337 (all-degree place inequalities) and PCC-e8b7ce
(genus131 natural-A degree34453/norm263 necessary condition), both unverified.
The first uses finite exact degree1–19 data plus a written uniform tail bound.
The second additionally needs polarized decomposition, coefficient-order
Artin invariance, Torelli and the CM polarization/ideal correspondence.
Native GMP/PARI replay does not formalize these bridges. The generator,
glued Jacobian, curve equations and subgroup maps remain absent.

Round02 adds a written Artin-obstruction argument restricting isogeny
degrees to principally polarized representatives. Its PCC record is
PCC-20261123-b7901e, standing unverified. The native 260-embedding residue
census is only an arithmetic check. Full Lean formalization of coefficient
orders, polarization-class invariance, Artin reciprocity and integral Tate
modules is still absent; the degree-divisibility result is not Lean checked.

| Report obligation | Evidence | Formal status |
|---|---|---|
| U131 recurrence and factorization | check.cpp/check.txt; Lucas integer recurrence | factorization only in Arithmetic.lean, uncompiled |
| ell primality | full n-1 Lucas certificate, base13, trial-proved factors | native producer replay; no Lean declaration |
| residue fields, primitive z and pi=barpi | exact quadratic-field arithmetic in check.cpp | native producer replay; no Lean bridge |
| A simple with CM field K | written field-generation argument | not formalized |
| principal polarization exists in A's isogeny class | written residue-prime argument + Howe1996 Thm1.1 | no Lean CM/isogeny/Howe bridge |
| integral zeta excludes genus130 Jacobian | written Rosati/Torelli/tame Riemann–Hurwitz proof | no Lean Jacobian/Torelli/ramification bridge |
| zeta not in universal Frobenius order | written residue-field-image argument | not formalized |
| curve genus130–300 and explicit subgroup transfer | none constructed | original task unresolved |

`lake --version` and `lean --version` are unavailable (exit127). Current
crypto-lean toolchain was read from upstream: leanprover/lean4:v4.35.0.
No crypto-lean dependency revision is presented as built or checked.
Arithmetic.lean imports only core Init and is an uncompiled candidate;
its four finite equalities do not establish the number-field or geometric theorems.

The finite modular witness is an exact custom native certificate replay,
not an identity.certificate/v1 or signed PCC theorem proof. No IDC1 claim
is made. The geometric deductions remain written mathematics pending the
bridges above and independent review; this limitation is explicit.
