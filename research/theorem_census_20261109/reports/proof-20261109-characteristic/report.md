---
geometry: margin=17mm
fontsize: 10pt
mainfont: DejaVu Serif
monofont: DejaVu Sans Mono
---

# Characteristic torsion changes the field and pairing conditions

On the actual nonsingular F2curve E:y²+xy=x³+1, Lean proves that every
geometric2-torsion point is rational, yet2does not divide2^1−1. This refutes
the unrestricted set-inclusion version of the torsion-field criterion.
It also proves that every μ2-valued map on this nontrivial geometric
torsion group fails pointwise left nondegeneracy. [pcc:unverified]

![Actual Mathlib groups and algebraic closure. Counts are kernel-proved cardinalities, not measurements. The r²point-count condition is stronger than set inclusion at characteristic torsion. [pcc:unverified]](proof.pdf){width=90%}

| Object | Checked cardinality | Actual meaning |
|---|---:|---|
| $E(\mathbb F_2)$ | 4 | Nonsingular rational points, including infinity |
| $E[2](\overline{\mathbb F}_2)$ | 2 | Only infinity and (0,1), both rational |
| $\mu_2(\overline{\mathbb F}_2)$ | 1 | Every square root of 1 equals 1 |

BK1998 Theorem1 uses the existence of r²torsion points, not merely
containment of the geometric torsion set. Its proof first obtains
r not dividing q from divisibility. Atq=r=2the geometric torsion has2points,
not4, so the primary source is not refuted. The compressed set-inclusion
version needs a prime-to-characteristic/rank-two qualification. The actual
corpus node already includes gcd(r,q)=1; this is its excluded-case negative
control, not a disproof of that node. The earlier
F7/r3counterexample separately preserves the r not dividing q−1 exclusion.
[pcc:unverified]

For the pairing obstruction, the proof identifies a nonzero actual point
killed by2and a singleton root-of-unity value type. Every map has value1
on its entire row. This is stronger than checking one arbitrarily chosen
bilinear map. It is explicitly a statement about geometric points; the
finite-flat group-scheme Weil duality statement is not refuted, and actual
Weil/Tate constructions on their qualified domains remain open.
[pcc:unverified]

Source: BK1998, pp142–143, Conditions1–2/Theorem1/proof,
DOI10.1007/s001459900040; PDF SHA256prefix ac837b8edc47.
Full hashes/read scope: ../../CHARACTERISTIC_TORSION.md. Lean source:
../../lean/CharacteristicTorsionBoundary.lean, combined theorem
characteristic_boundary_certificate. The full project gate passes 134
declarations, 0 failing and 0 trusted-native exceptions. Packet
PCC-20261109-92d64d remains unsigned. Full transitive gate and failed
diagnostics are retained under ../../evidence/characteristic-torsion-*.
No new native population or frozen-data change; zero paid spend.
[pcc:unverified]
