# Fixed-input differential compatibility development contract

2026-11-16. Scope: actual P-192 d7 kernel1, its pinned quotient theta vector and descended candidate C. This is a proposed implementation of a mathematical compatibility test, not a pre-existing twist verdict, full isogeny map, scalar solve or performance experiment. Do not replace the source, kernel, theta vector, target equation or acceptance identities after seeing data.

## Sources and argument to check

Primary sources inspected: Tian arXiv:2007.03172 sections2.1--2.3,4.2.2--4.2.3 (determinant Weil functions, normal sets differing by characters, map construction); Kirschmer--Narbonne--Ritzenthaler--Robert, Spanning the isogeny class of a power of an elliptic curve, sections4.3 and5.2 (differential-dependent affine lifts and Serre obstruction). The latter article's E^3 algorithm is not asserted to apply directly to our source. KB executable/tools remain unavailable; primary PDFs and pinned native sources supply the fallback.

The marked geometric ppav identification induces a differential matrix M from the quotient's Fp basis (pulled back to x^j dx/(2y) on H) to the plane quartic's Fp basis. If the marking is correct and rigid up to sign, its Frobenius is M or -M. Corresponding normalized odd Weil functions have quadratic leading forms Q_A=M^t Q_C M. A projective matrix M0 defined over Fp and a common scalar s with Q_A=s M0^t Q_C M0 would thus determine the quadratic extension of the differential identification: s square gives a rational differential lift, s nonsquare the opposite sign. This implication is conditional on the checked Weil-function/marking normalizations; projective bitangent data alone do not supply it. No signed or formal proof is claimed.

## Target divisor and function layer

- Work first on the pinned Riemann quartic over Fp24, keeping its labelled28 bitangents and the descent matrix B. Use label52 (Z=0) as Lb and its two contact points infinity1,infinity2; reject nonsplit/repeated contact roots in the declared field.
- A finite bitangent La supplies its degree2 contact divisor. Translation by La-Lb uses two conics: conic through E+La has residual R, conic through R+Lb has residual Enew. Check both exact intersection factorizations and Enew-E~La-Lb. Reject nonunique conics or unsupported charts; do not silently substitute points.
- Construct E0 representing infinity1+delta_C from the fixed two bitangent differences18 and38 (their xor52); construct the evaluation class Bi61 and other heldout even2-torsion classes by these translations. Retain every effective cubic and conic witness.
- For S in the fixed odd set63,38,41,13, La has label S xor52. Choose a basis of conics Q_i through La and infinity2. Tian's raw Weil function in the finite cubic chart is det(Q_i(R_j))^2 divided by det(1,x,y)^2 times the norm of La. Evaluate using polynomial remainders/resultants, without choosing roots of the cubic support. Require the denominator units and expected theta zero at E0.
- Normalize at Bi61 using the actual quotient theta-square values and d2 cocycle. Check the diagonal group law and a separate parity-admissible even class for each S. The heldout class is selected from labels in ascending order before any function value, by parity only. A failed check is a normalization failure, not a twist verdict.

## Differential layer

- Use exact first-order dual numbers. The raw odd functions are squares with zero constant term, so their quadratic leading forms are computed from first derivatives of the determinants/sections and their nonzero constant denominators; no finite-difference approximation.
- Source: translate a formal generic divisor by the constant source2-torsion class and differentiate the existing theta section. Try actual kernel representatives1..26 in order, rejecting unsupported polynomial charts; preserve exclusions. Normalize the source2 Weil set by the same deterministic method as the pinned moduli producer and check heldout equations.
- Three source flows use h=1,x,x^2 with dx=2y h,dy=f' h. Three target flows use h=1,x,y with dx=F_y h,dy=-F_x h. Convert their velocities to canonical Abel coordinates by exact trace matrices; require nonsingular matrices and curve equations over the dual numbers.
- Transform target differentials through B, including det(B)/lambda where F_old(BX)=lambda F_base(X). Recover M0 from the four labelled odd gradient lines, normalize its first nonzero entry to1, require nonsingularity and every entry in Fp.
- Require all four full quadratic forms to have one identical nonzero Fp ratio s. Retain matrices/forms/scalar, not just a Boolean. A mismatch or non-base-field scalar rejects the proposed test. Only after all these checks compute the Legendre symbol of s.

## Controls and standing

Exact curve/conic identities, changed contact divisor rejection, changed function value rejection, constant/first-order checks, heldout normalization and common-form checks are mandatory. The scalar comparison must distinguish an identical synthetic differential model (square) from multiplication of its quadratic forms by a fixed Fp nonsquare (nonsquare); this tests only the field discriminator, not the geometric construction independently. All assertions remain producer-unverified. No source DLP transfer is accepted without an executable divisor-class map and its own kernel/Frobenius/dual checks.
