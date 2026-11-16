# Exact Weil polynomials, modulo-3 kernel screen, and projectors

All20archived genus2/3/4curves have uniquely reconstructed middle coefficients
and hence exact Weil polynomials from low-extension point counts and their
verified prime-order publicD1. Modulo3,11curves have a proved kernel obstruction,
6remain unknown in this screen, and3have exactly two abstract stable kernels.
No new selector, target, timing or favorable replacement. This is established
module theory plus a constructive prerequisite, not a10xspeed result.

| Genus | Curves | Obstructed | Unknown repeated/self-dual | Positive regular |
|---|---:|---:|---:|---:|
|2|8|2|5|1|
|3|6|5|0|1|
|4|6|4|1|1|

Every case, raw point count and coefficient is in CHECKS.txt; SUMMARY.json
retains unknown counts as null. All20middle coefficients have exactly one
candidate within the registered coefficient bound, using ell|#J. All input
D1classes are valid/nonzero and annihilated by a separately primality-checked
ell; thus their orders are ell, proving the divisibility used in reconstruction.

## Positive regular cases and exact point fields

| Genus / prime / family | Irreducible dual factor degrees mod3 | Abstract stable kernels | Period of every nonzero point in either kernel |
|---|---|---:|---:|
|2 / 16381 / 1|2 + 2|2|8|
|3 / 127 / 0|3 + 3|2|13|
|4 / 127 / 1|4 + 4|2|80|

The two primary subspaces are the only stable Lagrangians: each factor is
irreducible and occurs once, and the factors are distinct twisted duals. Each
primary space is isotropic and has half the total dimension. Frobenius acts
as multiplication by X in its finite-field simple module. Exact order of
Xmodfactor gives every nonzero point's period; all divisors1..3^degree−1
were tested, not just a plausible upper bound. Six factor-order checks pass.

The degree is the field needed to define an individual nonzero torsion class,
not a lower bound on every possible isogeny algorithm. Compressed Galois-orbit
or etale-algebra methods may avoid explicit materialization. No coordinates
or runtime follow just from the field degree. A genus4polarized quotient
still need not be a Jacobian. Genus2destination geometry is unchanged.

For the genus3positive curve, the exact Lpolynomial is
[1,1,-43,-1224,-5461,16129,2048383]in ascending order. Its characteristic
polynomial modulo3factors as
f=X³+2X+2 and g=X³+X²+2. Both roots have order13. Complementary projectors
on J[3]are E0=1+2pi+2pi²+pi⁴and E1=pi+pi²+2pi⁴. All polynomial identities
E_i²=E_i, E0E1=0, E0+E1=1, fE0=gE1=0mod(fg)pass.
Analogous projectors for both genus2and4kernels are in PROJECTORS.txt.
They apply ONLYto3torsion; applying their integer lifts to arbitrary target
points is not a3torsion projection and supplies no kernel point.

## Reconstruction proof and controls

Write L(T)=1+a1T+...+agT^g+...+p^gT^(2g), with reciprocal coefficient
p^(g-j)*a_j. Newton identities recover a1fromC1,a2fromC2,a3fromC3.
Count through degreeg−1only: the unknown middle coefficient satisfies
|ag|<=binomial(2g,g)*p^(g/2). Registered integer upper bounds are6p,
20p*ceil(sqrt(p)),70p². Since L(1)=S+ag and ell divides it, enumerate all
agcongruent to−Smodell within the bound. A unique candidate is the actual
coefficient; no numerical root approximation or probabilistic order inference.
Order and reciprocal-polynomial checks all pass. Real extension samples total
7226370elements, below10million; no cap/error or point-count retry.

Extension characters use chi(z)=chi_p(Normz), with exact quadratic/cubic
norms. Independent Euler tests pass on all1994field elements across p5/7/11
and degrees2/3. Nine smooth genus2curves compare the resulting zeta order
with exhaustive reduced Mumford enumeration. Forty-three congruence controls
and two modulo3factor/module controls pass. No correctness mismatch.

The obstruction uses twisted reciprocal Pdual=X^d P(p/X)/P(0). A self-dual
irreducible even-degree primary component is nondegenerate. Any stable
Lagrangian must intersect it in half its dimension. Odd multiplicity makes
that dimension not divisible by the irreducible factor degree, whereas all
invariant subspaces have composition factors of that degree. Thus no such
Lagrangian exists. For squarefree polynomials consisting only of distinct
dual pairs, choosing one primary component per pair constructs all2^pairs
abstract kernels. Other cases are deliberately unresolved by this screen.

## Scientific custody and limits

First source da1f554b0cf6/checkpoint20261115T131141Z-da1f554b0cf6preceded
point counts. Order source56ffb219aa1a/checkpoint20261115T131546Z-56ffb219aa1a
preceded its algebra. Projector sourcebd0b987de8fb/checkpoint
20261115T131938Z-bd0b987de8fbpreceded projector algebra. Protocols, raw
outputs, fixture/binary hashes and inspected PDF/SVGretained. No scientific
Python/cloud/performance benchmark/default/old closed-record mutation.

The first raw schema printed numeric0as a placeholder with unknown status
on six cases. It is NOTzero kernels. Native reduction maps those entries to
null; future source prints null directly. The original raw/source remains
unchanged, no point counts rerun merely to change formatting.

Read ecdbguide first; read lookup failed for missing credentials (ECDB.stderr).
Pinned fixtures used instead. KB/source receipts in PROTOCOL.md; Milio and
Tian both require the isotropic kernel as input. Their algorithms do not make
kernel construction, same-field destination/divisor transport or complete
solve cost free. Current standalone science gates are local evidence, outside
automatic native CI. Full mathematical10xgenus2/3/4goal remains active.

## Next concrete dependency

For genus3p127family0, construct and validate actual3torsion overFp^13,
then apply the certified projectors to obtain explicit kernel bases. Charge
that construction before an isogeny/plane-quartic cost comparison. The point
source, finite-field/Cantor extension arithmetic, group-order cofactor,
isotropy and target transport need separate frozen correctness/resource gates.
Do not infer a10xcold gain from abstract kernels, known asymptotic exponents,
or the reduction in point-count extension degree used for this prerequisite.
