# Two actual models for unbalanced factor transfers

The complete changed suite passes34 tests,zero failed/ignored/filtered;
oldguards,formatting,strictClippy and all622 source/fixture/engine hashes
remain unchanged. Whole82,084-byte output→standardXZ9,716,source/status/
hashes/exact roundtrip are PCC-20261112-925b02. Selected1/33filteredfeff57 is
diagnostic only. These are model/cover/graph checks,not executed whole
Jacobian maps or independent scientific signatures. [pcc:unverified]

## Exact objects and sources

Over F101,put

    C10: y²=61x6+8x4+93x²+89,
    C20: y²=18x6+75x4+45x²+40.

They are smooth degree-six genus-two models with no normalization or retwist.
Both leading and constant coefficients are nonsquares over F101. Each has
120 finite F101 points,zero rational infinities and zero rational x0 points;
over the declared Fp12 field,both x0 points and both infinities are retained.
Complete two-field counts are120 and10436,#Jac12317. [pcc:unverified]

Primary references:Bruin–Doerksen0902.3480 Lemma2.4/2.6,Definition2.7 and
Theorem3.2,Galbraith25/26,Mille's degree/polarization results as cited in
ISOGENY_CYCLE_PROOF_V2.md (f48205/b7c7e7). Use the accepted actual C0
[56,0,13,0,6,0,1],E0=(1,46),F0=(44,21),and the oriented E seven-isogenies
alpha:E0→E1=(26,58),alpha2:E1→E2=(59,27),eigenvalue5. The unchanged
constructor's coefficientwise identities and normalized duals are essential.
KB MCP/wrapper and encrypted atlas are unavailable;these exact sources are
the fallback. CT44's one-factor7 four-pairing obstruction is credited,
not a new abstract finding.

## 1. Actual field-defined elliptic covers

For f(x)=c6*x6+c4*x4+c2*x²+c0,the two degree-two quotient cubics are
[c0,c2,c4,c6] at u=x²,v=y, and[c6,c4,c2,c0] at u=1/x²,v=y/x³.
For a cubic v²=a*u³+b*u²+c*u+d,the checked short-coordinate change is
X=a*u+b/3,Y=a*v. A checked further F101 isomorphism multiplies X,Y by
s²,s³. The actual parameters are:

| Model | Direct cover destination;a,b/3,s | Reciprocal destination;a,b/3,s |
| --- | --- | --- |
| C10 | F0;61,70,43 | E1;89,31,43 |
| C20 | F0;18,25,33 | E2;40,15,33 |

All factor roles,including the swap relative to C0,are checked rather than
assumed from a polynomial's order. Native verifies the complete cleared
curve identity coefficientwise for both covers,then every actual rational
point image. At x0,the reciprocal image is O;at infinity the direct image
is O and the reciprocal image is (s²*b/3,±s³*a*sqrt(c6)). All these extension
points and six ramified Weierstrass roots are checked. [pcc:unverified]

Since characteristic is101,2 and3 are invertible. Each rational map extends
uniquely to a morphism between the smooth projective curves. Its degree is
exactly2:the first fixes the nontrivial involution(x,y)→(-x,y),the second
fixes(x,y)→(-x,-y),and each recovers x² from its target X. No point-count
coincidence substitutes for these equations and maps. [pcc:unverified]

The two universal cleared identities are

    27a²(a*x6+b*x4+c*x²+d)
      =(3a*x²+b)^3+(9ac-3b²)(3a*x²+b)+2b³-9abc+27a²d,
    27a²(a+b*x²+c*x4+d*x6)
      =(3a+b*x²)^3+(9ac-3b²)(3a+b*x²)*x4
        +(2b³-9abc+27a²d)*x6.

UnbalancedCovers.direct_short_quotient/1bde23 and
reciprocal_short_quotient/d03d51 are exactly bound local-pass core Lean
proofs over every commutative ring;gate01 passes2/zero trusted native.
Actual invertibility,scales and geometry are separate inputs. The unchanged
native identity checker emits IDC1hdb5d8c25ad6d1262 and
IDC1h15a63844aad1bcd0,32issuer+64extra points each over2^61-1,with exact
final Lean-source digest. All23 algebra tests pass,all11 input hashes remain
unchanged,and the old five certificates replay byte-identically,f404e3.
The project has171 written declarations across34 modules;bulk compiled
count for this new source remains a separate CI observation. [pcc:unverified]

## 2. The actual gluing is the transported source graph

Let P0:E0×F0→Jac(C0) be the old product of degree-two pullbacks. Its kernel
Gamma0 has order4. Native independently extracts the actual Q0=Ker(K0)
on T0 from all elliptic4-product Cantor pulls,checks [2]Q0=Gamma0,and
compares Gamma0 with the three old C0 root pairs
(u+2,56/u+38),where u³+6u²+13u+56=0. [pcc:unverified]

Construct the new HLP models from the actual transported graphs
Gamma10=(alpha×id)Gamma0 and Gamma20=(alpha2*alpha×id)Gamma0. For every
new Weierstrass root x,both covers send x to paired nonzero elliptic2-points.
The six roots give exactly the three transported pairs. Pullbacks of such
a pair are the same class W_x+W_-x-infinity_+-infinity_-;the reciprocal
origin fibre at x0 is equivalent to the infinity sum by div(x). On2-torsion
negative equals positive,so each pair is killed by the product pullback.
Optimality of each prime-degree2 cover gives an elliptic embedding,and
cross Hom=0 gives orthogonal factors. Bruin–Doerksen Lemma2.6 then gives
product-pull degree4 and norm[2]. The displayed order4 graph is therefore
the entire kernel,not merely a candidate subgroup. [pcc:unverified]

All coefficients,scales and graphs are F101-defined. Thus P10/P20 identify
the two new Jacobians with the corresponding F101 stable quotients. Native
compares the graphs with the complete paired walk:Gamma10=Gamma21,
Gamma20=Gamma6;the primitive four-graph Q20 is classA6 up to relative sign.
The kernel identifications,not just spectral equality,select those classes.
[pcc:unverified]

## 3. Global unbalanced maps and nonuniform norms

The universal property now supplies F10/F20 with

    F10*P0=P10*(alpha×id),
    F20*P0=P20*(alpha2*alpha×id).

Degree cancellation gives degF10=7,degF20=49. Both are separable and
preserve the109/113 rational DLP subgroups because their degrees are coprime
to those orders. Their kernels are the P0 images of the first-factor
elliptic kernels:degreeP0=4 is coprime to7. The nondual second kernel is
cyclic49 by the selected ordinary7-eigenline;it is not the old balanced
rank-two kernel. Actual whole Mumford-map and kernel-scheme execution remain
phaseB obligations;this paragraph is the written quotient proof.
[pcc:unverified]

Write H_i=P_i^dagger,so H_iP_i and P_iH_i are[2]. Taking adjoints and
cancelling the finite-kernel isogeny H0 in Hom gives
F10^dagger P10=P0*(alphaHat×id). Therefore

    F10^dagger F10 P0=P0*diag(7,1),
    F20^dagger F20 P0=P0*diag(49,1).

The diagonal maps preserve Gamma0 modulo2 and descend. Since P0 is
surjective,the displayed equations determine the actual global norms.
They are not uniform[7]/[49] on the full Jacobian. No cancellation in a
finite point group occurs in this argument. On rational points only,the
cleared relation2F=P_target*factor*H0 uses public inverse2 modulo109/113.
The whole-domain algorithm must compute those norms from the input divisor,
not use the enumeration's logarithm labels. [pcc:unverified]

The exact finite field/infinity and nonuniform-norm cases are preserved in
UNBALANCED_PHASE_B_NOTES.md. The original source group's F113 component
explains the future uniform-norm mutant:it should differ on12208 classes,
with109 equalities. This is a prediction,not an executed result.
[pcc:unverified]

## 4. Four-glue acceptance and remaining implementation

Transporting the actual Q0 by alpha×id gives a Galois-stable16-point graph
with96 nontrivial product-pair witnesses among256,all equal-1. This is the
native actual-coordinate realization of the credited CT44 mismatch.
The catalog's forced principal-pullback argument rules out a principal
polarization on that quotient;mod2 alone misses it. Transport by
alpha2*alpha×id has degree49≡1mod4 and retains anti-isometry;its graph is
A6. Constructing T20 through the C20 Richelot kernel and checking the
whole primitive-four-glue map remains phaseC. [pcc:unverified]

C10/C20 have intrinsic factors(E1,F0)/(E2,F0) and canonical sum degree4.
They are geometrically distinct from each other and all six earlier C_i/T_i:
factor j values distinguish old C_i,while sum degree4 versus16 distinguishes
all T_i. This uses the accepted intrinsic-factor theorem,not a native
genus-two isomorphism solver. They are two additional explicit model
presentations within the existing fixed72-Jacobian catalog,not an increase
of that catalog or a novelty claim. [pcc:unverified]

Full34 and algebra23 acceptance,universal identities and global written
proofs complete the model phase. Whole12317 rational maps,actual kernel/
dual/recovery/exception execution and T20 construction remain open;the
source-controlled next-action notes make those requirements concrete.
No timing protocol,paid experiment,solver cost or scientific signature
is supplied by this component. [pcc:unverified]
