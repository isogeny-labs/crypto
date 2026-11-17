# Formal status and exact open proof obligations

There is no new general theorem or polynomial identity in this round. The
central statements are finite computational claims. Native checks are not
Lean kernel proofs and are not promoted to degree-of-regularity certificates.

| Report statement | Current evidence | Lean declaration/status |
| --- | --- | --- |
| Degree13 B027+B033 has 14 inverse-pair columns and 242 distinct nonidentity r-torsion sums | Full point/witness census, direct scalar checks, pinned source/input hashes | None; not formally proved |
| Each reported pair's direct r-torsion flag agrees with its cofactor-image flag | All 62,986 unique pairs checked, including rejections and identity | None; finite checks only |
| Useful same-target-difference ranks in355 cases | Native modular elimination, projected-zero columns removed, unit dependency tests, full retained witnesses | None; not a kernel-verified rank certificate |
| Lower successful solving degree, regularity or full cost | No result claimed | Open, not encoded |

Actual tool checks `lean --version` and `lake --version` both returned command
not found (127); output is in the validation records. No Lean source is
presented as checked. The repository has existing Lean4.34.1 core projects,
but no installed compiler was found in this workspace. A faithful central
formalization also requires polynomial-basis GF(2^n) arithmetic, the exact
binary Weierstrass group law and exceptional cases, encoded finite inventory
sets, r-torsion tests, inverse-pair quotient and cardinality/rank proofs.
A lemma assuming the desired support or arithmetic oracle would not establish
the recorded statement and is deliberately not substituted for it.

No IDC1 polynomial certificate is attached: no new polynomial identity is
claimed or used as a substitute for direct group arithmetic. A v1 certificate
over its fixed prime field would not certify these binary-field group counts.
The finite results remain native checked and reproducible; the full formal
proof obligation remains open, as noted in the named obstruction record.
