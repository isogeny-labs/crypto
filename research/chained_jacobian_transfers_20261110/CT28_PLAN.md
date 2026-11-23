# CT28 prospective — direct prime-field elliptic covers of T0/T1/T2

Proposal after CT27 arbitrary-class replay. No cover, coefficient, runtime
acceptance or computational gain is claimed here. First consult available
KB/atlas/local implementations; if access remains unavailable, record that
limit and make no novelty or absence claim. [pcc:unverified]

## Question and fixed baseline

Can the actual degree4 elliptic quotient homomorphisms H_i=N_i R_iHat on
the accepted nonsplit sextics T_i be evaluated directly over F101, avoiding
the existing F101³/F101⁶ Richelot evaluation? Baseline is CT26 consumer
25fd0be7cea37cefd298d3ac38b6f863bb9105d5. First construct just T0→E109,
then the other component/model only after this first exact cover passes.
This is a new evaluator for an existing homomorphism, not a new transfer
or an ECDLP algorithm advantage. [pcc:unverified]

## Discriminating construction attempt

Use existing arithmetic only. From actual T0 rational points P, evaluate
H_0([P-infinity-]) by retained RiHat plus C0 norms. Also evaluate
D=H_0([infinity+-infinity-]) and q=D/2 using the actual odd elliptic order,
not a discrete-log label. Predict that shifting the point map by -q gives
an odd elliptic cover pi with pi(infinity±)=±q. Then x(pi(P)) depends on
the hyperelliptic x and y(pi(P))/y is rational in x. [pcc:unverified]

Try reconstructing R=N/Q² and S=M/Q³, with degN≤4,degQ≤2,degM≤3,
so pi(x,y)=(N(x)/Q(x)²,y M(x)/Q(x)³). Use existing polynomial
interpolation/extended Euclidean rational reconstruction or an existing
linear solver. No new field,polynomial or group engine,Python crypto/math
harness,or sampled-output substitution for a global identity.
[pcc:unverified]

Require exact polynomial identity over F101:

    F_T M² = N³ + a N Q⁴ + b Q⁶.

Check gcd(N,Q)=1,smooth curves,nonzero M,separability and actual rational
x-map degree4,including infinity behavior. If this ansatz fails,retain
the failure;do not silently broaden degree bounds or drop points.
Irreducibility of the actual sextic may explain a square denominator:
a degree≤4 denominator cannot have a source Weierstrass root of degree6,
so poles away from source branches should have even multiplicity. Prove
that bridge before relying on it. [pcc:unverified]

## Global identification obligation

The polynomial identity defines a global morphism after proper completion;
it does not alone identify its Jacobian norm with H_i. Independently check
the original global norm/pullback relation H_i H_iHat=[4],and the theorem
that the corresponding curve map has degree4. Establish global equality
by symbolic correspondence algebra or a rigorous degree-bounded certificate.
[pcc:unverified]

Possible finite certificate:for two degree4 curve maps to an elliptic
curve,the theorem of the square should give deg(f-g)≤16. Agreement at
more than16 distinct source points,with an anchored translation,then
forces equality because a nonconstant map's fibre has at most its degree
many distinct points. Read and cite a primary degree/parallelogram theorem,
prove its application including constant/inseparable cases,and independently
replay all certificate points. This is a bounded-degree uniqueness argument,
not a density claim from finite outputs. It remains an obligation.
[pcc:unverified]

## Arbitrary balanced norms and later pullbacks

For a quadratic support u=x²+A x+B with Q a unit modulo u,reduce the
two elliptic coordinates to r0+r1*x and w0+w1*x in the finite support
algebra. Predict slope w1/r1 when r1 nonzero,then symmetric addition
without individual support roots. If r1=0,derive doubling/opposite/branch
cases from the curve identity,including repeated jets. Nonunit Q must
handle every pole:use existing gcd with Q² to detect support multiplicity,
discard only actual elliptic-origin contributions,and explicitly evaluate
any remaining linear support. Both balanced infinities contribute the
actual(2n+m-2)pi(infinity+) correction. [pcc:unverified]

Require comparison on all12317 classes,both components once constructed,
all degree/zero/pole/repeated/infinity categories,group/source/recovery and
geometric controls. The CT27 independent class engine should provide a
second replay. Preserve the old RiHat/norm evaluator as a genuine reference.
Include faithful feasible Lean identities and the exact written geometric
bridges;keep full formalization open if unavailable. [pcc:unverified]

Later test direct degree4 pullback from E into T:U=N-X Q² and
V=Y Q³/M moduloU on finite nonbranch inputs. Reuse the existing balanced
semireduction engine with the actual degree4 baseline2K. Derive dropped
degree/infinity contributions at P=±q and all origin/branch/nonunit cases;
do not force every fibre into degree4. Keep geometric references when the
new rational-input adapter does not cover extension points. This could
replace K_i=R_i P_i in Psi_i=K_i alpha_i H_(i-1)/4,but only after full
correctness and global-identification acceptance. [pcc:unverified]

Freeze sources/receipts/failures/PCC/report/diagram/inspectedPDF and ready
stackedPR/exact-headCI. No performance claim without admitted matched cold
protocols,including construction,conversion,recovery and strong rho/current
index-calculus baseline. S3/livebudget still block benchmarks. Larger
fields/catalog and independent isogeny-map membership remain separate.
[pcc:unverified]
