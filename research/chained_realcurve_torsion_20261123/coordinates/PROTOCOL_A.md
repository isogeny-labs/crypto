# Genuine Jacobian eight-torsion coordinate profile A

This is a distinct actual-divisor qualification after census B, not a rerun
of the abstract CT332 lifts or the genus-three product example. No coordinate
outcome has yet been observed. All statements remain unsigned. [pcc:unverified]

## Fixed object, inheritance and acceptance

Use exactly the predetermined split census id0 over F3:
C:y^2=x^6+x^5+x^2+1. The retained census gives
#J(F3^12)=282375475200=4096 times an odd integer. All J[8] is rational there.
This is a smooth genus-two Jacobian with its own canonical polarization;
it is not a product declared to be a Jacobian. Its explicit elliptic-factor
maps and their costs are still open. [pcc:unverified]

Reuse selected exact Fld, polynomial, Div and Hyp Cantor bodies from pinned
crypto816f9c3a src/cryptanalysis/jv_cover.rs, blob990760b1. The closure in
inherited.rs omits unrelated solvers, concrete Fp2/Fp6 implementations and
unused BigUint scalar multiplication. INHERITANCE.json pins the complete
original source and exact selected fragments. No selected body is changed;
enable debug assertions in the release build so exact divisions are checked.
The new F3^12 instance, chart and Frobenius code carry the new risk.
Existing genus2_fp.rs c4fc53ce and balanced.rs16decfa3 independently agree
on the composition/reduction formulas and were read before adaptation.
[pcc:unverified]

Construct F3^12 with the first irreducible monic degree12 polynomial in
increasing ternary coefficient-code order (constant coefficient first;
skip zero constants). Decide irreducibility by Rabin: X^(3^12)=X and gcd
against X^(3^4)-X and X^(3^6)-X. Choose the first primitive element in numeric
field-code order by testing divisors2,5,7,13,73 of531440. Build a complete
log/antilog bijection, requiring every nonzero element exactly once and
return to1. Check every nonzero inverse and a^531441=a. Compare the first27
by27 products with polynomial arithmetic. Derive F27 from an element of
order26, enumerate all27 elements, and check Frobenius/distributivity and
multiplication associativity on all27^3 triples. [pcc:unverified]

Find all6 roots of the selected sextic in that F27, choose the least encoded
root r, and use t=1/(x-r), Z=y*t^3/s with s^2 equal to the leading coefficient
of t^6*f(r+1/t). The resulting h(t) is monic degree5 and must pass squarefree
and point/Mumford validity checks. Derive both chart-pole2-torsion classes.
Check the full15 Weierstrass-pair classes, uniqueness, order2 and actual
Frobenius action against cubing the original roots. [pcc:unverified]

Use fixed xorshift64 seed0x4a61636f6269616e, at most256 original-curve x draws,
canonical minimum encoded square root, and stop at the first4 classes whose
quadruples are independent in J[2]. Multiply each valid sampled point class
by the fixed odd cofactor #J(F3^12)/4096. Require [8]R=0, build all16 top-layer
classes, and match them exactly with identity plus the15 branch pairs.
For every valid sampled point independently compare chart Frobenius with
the cubed original point and the basepoint correction. Print all draw outcomes;
no seed/budget/curve replacement after failure. [pcc:unverified]

Enumerate every one of the4096 combinations of the4 order8 basis classes.
Require distinct canonical reduced Mumford pairs, validity, [8]D=0 and inverse
cancellation for every class. Check commutation and all64 associativity
triples on the basis. Extract the4 by4 Frobenius matrix modulo8 from the
actual divisor images. For all4096 classes compare their genuine Frobenius
images with that matrix; require4096 unique images. Check all16384 generator
edges for addition and Frobenius equivariance. Check P(F)=F^4+2F^3+3F^2+6F+9
annihilates every basis class, F^12=1, and exact fixed counts1/16/256/4096
at powers1/3/6/12, as independently predicted by the census. [pcc:unverified]

Determine the genuine mod2 Weil pairing on4 times the basis via branch-pair
intersection parity. Require nondegeneracy and Frobenius invariance modulo2.
Do not invent a mod8 lift and call it the actual Weil pairing. The mod8
pairing, compatible isotropic kernels, chained Richelot/Jacobian maps,
odd-prime DLP transfer and comparative solver costs remain successor work.
[pcc:unverified]

## Negative control and mathematical chart contract

For delta=r^3-r and original d-degree Mumford support, cubing and changing
charts sends t to t^3/(1+delta*t^3). With monic scaling the ordinate multiplier
is s^3/s=s^2=leading. Thus u'(z) is proportional to
(1-delta*z)^d*u^sigma(z/(1-delta*z)), and v'(z) is leading times
(1-delta*z)^3*v^sigma(z/(1-delta*z)) modulo u'. The divisor class is
D' - d*T, with T=[W(r^3)-W(r)], rather than D' alone. [pcc:unverified]

The pole preimage is W(r^9), since r^27=r. Remove its2-torsion class before
the regular substitution when it occurs in support and add its actual image
T afterwards. A double pole support is incompatible with squarefree h and
the Mumford relation. Check degree drops on pole removal. [pcc:unverified]

The mandatory copied-rule mutant omits -d*T on the degree1 class T itself;
require it to disagree with the direct root-cubing image, and restore the
correct rule. Validity alone cannot detect this error because the incorrect
image is another valid divisor. [pcc:unverified]

Independence of the4 quadruples proves the4 order8 classes are a Z/8 basis:
a nonzero relation, multiplied by the appropriate power of2 determined by
its least coefficient valuation, would give a nontrivial relation between
those quadruples. Enumeration checks canonical representatives independently
of that short argument. No universal Lean/Jacobian theorem is compiled here.
[pcc:unverified]

## Execution, evidence and scope

Source-first full publication/readback, pinned Rust1.99.1 standard library,
strict rustfmt/Clippy/release compilation, debug and overflow checks enabled.
The inherited byte-exact source is excluded only from formatting edits; every
new Rust file is checked explicitly with skip_children=true. No warnings or
lints are suppressed. All A sources remain immutable after first call; a
failure requires a separately frozen successor. Each phase retains2GiB,
120 seconds and one worker, with copied-source/tool/build/pre-call/post-call
guards and an immutable separately copied ELF. [pcc:unverified]

The native program prints complete field/chart/basis/Frobenius/mod2-pairing,
all draw and control outcomes plus coverage counts. It also writes a separate
whole coordinate TSV containing all4096 original Mumford pairs and their
actual Frobenius indices. That scientific data file and whole ELF remain
private custody-pending if approved remote storage is unavailable; retain
their complete bytes/hashes without compressing/splitting or claiming a
remote mirror. All small stdout/stderr/argv/manifests are committed whole.
No hermetic execution proof, external validator or signed standing is implied.
[pcc:unverified]

No old scientific population is rerun, and no timing or ECDLP computational
advantage is claimed. Prime-field transfer eligibility and extension-field
torsion qualification are distinct; this fixture alone does not descend a
cryptographic DLP or give its destination solver. Paid STOP, locked vault,
formal/CI billing gaps and normal merge requirements remain unchanged.
[pcc:unverified]
