# CT66 — complementary elliptic quotients, explicit dual, full F1024 Cantor inverse

**Scope and standing (2026-11-13): [pcc:unverified].** This is a complete *same-characteristic*, bounded order-31 dual/inverse for the CT62–65 genus-two example, plus an exact census of the 100 ordered cross-factor maps on 31-torsion. It is **not** a prime-field-to-binary point lift, an end-to-end cross-characteristic ECDLP attack, or a cold-cost speedup. All computations are public synthetic / known-answer.

## Object, complementary involution and explicit quotients

Let F1024=F2[z]/(z^10+z^3+1), with the root omega=236 of omega^2+omega+1=0, and let

    C/F4: y^2+y=x^5+omega*x^3.

From CT64, the genuine involution sigma_+ is

    sigma_+(x,y)=(x+20, y+6*x^2+120*x+432),

where field integers encode polynomials in z (not ordinary decimal coefficients). The hyperelliptic involution is iota(x,y)=(x,y+1) and acts as [-1] on J(C). The **complement** is the other Artin–Schreier root

    sigma_- = iota*sigma_+:
    sigma_-(x,y)=(x+20, y+6*x^2+120*x+433).

Both are order two; their product is iota. The cubic quotient formula of CT64, with

    U=x^2+r*x,
    V=y+A*x^3+D*x, A=a/r, D=(c+A*r^3)/r,

therefore yields two distinct elliptic curves

    E_+: V^2+V=941 U^3 +20 U^2 +336 U
    E_-: V^2+V=941 U^3 +20 U^2 +714 U.

The difference in the U coefficient is r^(-2)=922, since 336 XOR 922=714. Both elliptic curves have 961 F1024-rational points; each is (Z/31)^2. Their quotient maps pi_+, pi_- have degree two (x^2+r*x-U is separable, derivative r!=0). The two quotient maps are the **complementary pair for the same translation parameter**: CT64's maps using r=20 and r=534 are instead a distinct, noncomplementary pair, so their naive sum-of-pullbacks is not the scalar-[2] dual.

## Exact isogeny and its dual, no lookup-table assumption

Let phi=((pi_+)_*,(pi_-)_*): J -> E_+ × E_-, and
psi=(pi_+)^*+(pi_-)^*: E_+ × E_- -> J.

For a separable double cover pi with deck involution sigma, the standard divisor norm identity is

    pi^* pi_* = 1 + sigma_*       on J
    pi_* pi^* = [2]             on E.

Because sigma_- = iota*sigma_+ and iota_*=[-1], the two norms sum to

    psi phi = (1+sigma_+)+(1-sigma_+) = [2] on J.

The cross maps (pi_-)_*(pi_+)^* and (pi_+)_*(pi_-)^* vanish: the image of one pullback has sigma_+ eigenvalue +1 and the other -1, so the composition is annihilated by [2]. Hom(E_+,E_-) has no torsion (alternatively, a morphism of connected abelian varieties with finite image must be zero). Hence also

    phi psi = diag([2],[2]) on E_+ × E_-.

The pullback map is dual to pushforward for the canonical principal polarizations, so deg(phi)=deg(psi), while deg([2] on J)=2^4=16. Therefore **deg(phi)=4**, a (2,2)-isogeny, even in characteristic two (its kernel scheme need not be étale).

On **all full 31-torsion** over F1024, 2*16=32=1 modulo 31, so we have the explicit, logarithm-free inverse

    phi^(-1)|E_+[31]×E_-[31] = [16] * ((pi_+)^* + (pi_-)^*).

The actual pullback of a nonidentity encoded elliptic point (U,V) is a Mumford divisor (u(x),v(x)) with

    u(x)=x^2+r*x+U,
    v(x)=(V+A*x^3+D*x) mod u(x).

No branch-selection oracle or scalar label is passed to the pullback. The implementation handles split and irreducible quadratic fibres through F1024 and its quadratic algebra and uses genuine full F1024 Cantor composition/reduction to combine and invert them. This is a *same-characteristic* map and does not supply any compatible global lift from an unrelated prime field.

## Exhaustive rank pattern for all ten quotients

Repeat the two Artin–Schreier choices c and c+1 for each of five r:
20, 534, 619, 809, 832. Let pi_i denote the resulting ten quotient maps, ordered with each complementary pair adjacent. For all 100 ordered compositions (pi_j)_*(pi_i)^*:E_i[31]->E_j[31], two independently coded checks give:

* ten diagonal maps have rank **2** and are exactly [2];
* ten off-diagonal **complementary** maps have rank **0** (the zero morphism);
* the other **80** noncomplementary maps have rank **2** modulo 31, hence are nonzero and 31-prime isogenies onto their image.

In a 10x10 matrix the pattern has five 2x2 diagonal blocks diag(2,2) (the entries mean mod-31 rank), with every off-block entry equal to 2. For example, in independently enumerated elliptic bases, the map (pi_534)_*(pi_20)^* is

    [[2,2],[25,20]]  (mod 31), determinant 21 (nonzero).

These ranks concern the **specified compositions**, not the entire Hom groups: a complementary pair has zero cross-composition while E_+,E_- may still admit other nonzero isogenies. The checks evaluate the split fibres of all 896 affine C(F1024) points, verify their projections generate E_i[31], and validate the induced linear map on each of those 896 inputs. Python independently checks the rank via 31-point cyclic-subgroup containment rather than JS discrete-log basis tables.

The Python F1024 Cantor program also checks all 31 encoded F4-rational source classes, with support inventory

    {zero:1, linear:6, split:12, repeated:6, nonsplit:6}.

It tests psi phi(D)=[2]D and [16]psi phi(D)=D for every class. It then constructs 6 independently selected target pairs (e_+,e_-) outside the single source rational 31-line, pulls them back, and checks

    phi psi(e_+,e_-) = ([2]e_+,[2]e_-),
    phi([16]psi(e_+,e_-))=(e_+,e_-).

This includes extension-field nonsplit fibre reductions, unlike an F4-only Cantor check. CT62's earlier independent ramified case is a separate curve; the present smooth Artin–Schreier C has no finite ramification for its hyperelliptic double map. We do **not** claim an exhaustive census of all 31^4 source classes or all 31^4 elliptic-product target pairs.

## Reproduction and independent validation

Run from repository root:

    D=research/cross_characteristic_global_torsion_20261112/ct66
    python3 -B "$D/ct66_full_dual.py"
    python3 -O -B "$D/ct66_full_dual.py"
    python3 -B "$D/ct66_dual_crossfactor.py"
    python3 -O -B "$D/ct66_dual_crossfactor.py"
    node "$D/ct66_dual_crossfactor_independent.js"
    node --no-opt "$D/ct66_dual_crossfactor_independent.js"

Local normal/optimized Python and Node independently agreed: 10 quotient models, all 100 ordered rank results, zero 10 complementary maps, and rank two for 80 noncomplementary cross maps. The full-dual Python source recovers all 31 F4 classes and 6 independent target samples; normal and optimized outputs match. GitHub Actions integrated exact-head CI and independent scientific PCC certification must be checked separately. Bounded local validation is not a project-wide cargo or cold ECDLP benchmark.

Local source SHA256 checks before publication:
* ct66_full_dual.py: 8dc9a851c8c885833ac422012b574400a2a39c27ffaa0f4b36f337fac5242b80
* ct66_dual_crossfactor.py: 711d3678a67157fd9b4e787ea69409a9c6c0b8e35c75ea7540c21bf9097e0556
* ct66_dual_crossfactor_independent.js: 55de687f2b6504a62c7182a74e504670c0d143c1064656b5ac2134a568f41be6

## Literature, KB gate and next scientific dependency

This is a worked computational realization of classical genus-two complementary elliptic subcovers and (2,2)-splitting, not a novel general isogeny theorem. Sources reviewed: CT62–65 and root crypto #3877; T. Shaska, *Genus two curves covering elliptic curves: a computational approach*, https://arxiv.org/abs/1209.3187; Howe–Zhu, *Absolutely simple abelian varieties*, https://doi.org/10.1006/jnth.2001.2697; and Kani's work on elliptic subcovers as cited in CT64. Related distinct-root chained-transfer work in cryptanalysis #1520/#1521 was not modified. Shared research-KB MCP and checkout wrapper were absent in this runtime; fallback cross-repository GitHub searches included “complementary elliptic quotient genus two involution”, “Richelot dual pushforward trace characteristic 2”, and “chained genus two quotient field 1024”. Search results are not evidence of novelty or absence of prior work.

**Remaining blocked requirement:** construct a uniformly efficient algebraic number-field lift of arbitrary *encoded* prime-field source P,Q with compatible specializations in both characteristics **without determining their logarithm**, including all field, height, arithmetic, transport, inverse, target solving, and end-to-end cold-cost charges. The present explicit dual solves the separate finite-field component. Full-cost optimized negation-map rho and admitted index-calculus comparison remain unperformed for a genuine cross-characteristic chain. No EKS/RunPod/payed compute used. No protected merge. [pcc:unverified]
