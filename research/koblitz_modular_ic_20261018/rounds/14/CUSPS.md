# Normalized cusp descent and closed divisors

O28 refines the frozen O27 stage. It enumerates all480 geometric labels,
finds44 transitive Frobenius orbits, and assembles all eight F2 closed
signed divisors. The modular interpretation below is a written bridge;
the actual finite permutations, orbit coverage and degrees have Lean
certificates. O27's earlier open-cusp flag remains unchanged as history.

Use a local Tate parameter and write an ordered11-torsion basis as
P=zeta^a q^(b/11), Q=zeta^c q^(d/11), with ad-bc=1 modulo11. The
unipotent change of q^(1/11) changes the upper row (a,c). The lower row
(b,d), up to simultaneous sign, labels the cusp in the retained M13
convention. It ranges over the60 nonzero vectors modulo sign.

Arithmetic coefficient Frobenius sends zeta to zeta^2. The normalized
level action used throughout the signed modular maps also replaces Q by
Q/2, so its exponent matrix is (2a,b;c,d/2), with the positions understood
as the two columns (a,b) and (c,d). Its determinant is again1. Thus its
action on the cusp label is (b,d) -> (b,6d) modulo11 and sign. This is a
derivation from the Tate moduli description, rather than a rule inferred
from coincident target coordinates.

The level49 cusp undergoes ordinary coefficient Frobenius, namely
squaring in the actual F8 model E49. A choice of local root of the Tate
parameter can add upper-row terms, which disappear in the unipotent
quotient. A nonsplit multiplicative twist contributes simultaneous sign,
which also disappears. Since49 and11 are coprime, the independent level
data are combined by this product action. This explains the action at
all eight base cusps, including those with nontrivial residue fields.

The supporting general Tate-curve construction, torsion identification
and uniqueness/descent statements are in Brian Conrad,
[Arithmetic moduli of generalized elliptic curves, section2.5,
equation(2.5.2), Theorem2.5.2 and section4.3](https://math.stanford.edu/~conrad/papers/kmpaper.pdf).
The normalized basis convention and the displayed action are our
specialization and derivation; they are not quoted as a theorem of that
reference.

O28 applies this rule to the actual retained labels. All480 target pairs,
all eight valuation lists and both ramification indices are covariant
(5760 finite checks). The canonical line is multiplied by2, with infinity
fixed. Every orbit is transitive, all labels occur exactly once, and the
degree histogram is1:10,3:10,5:2,10:10,15:2,30:10. The weighted sum is480.
Restricting to x0*xinf reproduces the earlier22 closed entries exactly as
sets of exponent labels, closed degrees and signed multiplicities.

Constant valuations on a transitive orbit give its closed multiplicity.
Combining these44 closed cusps with the retained closed SS points and
all63 ordinary points completes each of the eight signed divisors. Each
has degree zero with the degree partitions in REPORT.md. This finite
enumeration and the written global norm/divisor bridges are both required;
balanced degrees alone would not prove completeness.

KoblitzRound14Cusps proves the universal determinant residual and its
normalized consequence, the actual F8 point/Frobenius checks, the actual
480-label permutation, exact orbit coverage and cyclic action, the degree
histogram, all eight signed degree identities, and the old product degree
control. It does not formalize generalized elliptic moduli, valuation
formulas, or the equivalence between polynomial masks and formal fields.
