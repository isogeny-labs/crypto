# Proof map

All mathematical deductions in REPORT.md are unsigned [pcc:unverified].

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
