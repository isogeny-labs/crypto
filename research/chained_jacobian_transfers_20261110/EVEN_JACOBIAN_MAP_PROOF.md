# Whole rational maps on five actual even models

The complete changed suite passes37 tests,zero failed/ignored/filtered,
with oldguards,formatting,strictClippy and all624 source/fixture/engine
hashes unchanged. Whole124,139-byte output is standardXZ10,952 with
raw/archive hashes,source/status and exact roundtrip,3c1cb7. Selected78a854
is diagnostic1/36filtered. The maps below are executed on every12317 rational
C0 class and every image in four target groups. Scientific standing remains
unsigned;native geometric kernel certificates are a separate stageC.
[pcc:unverified]

## 1. Exact curves,covers and the nonmonic chart

Use C0=[56,0,13,0,6,0,1],C10=[89,0,93,0,8,0,61],
C20=[40,0,45,0,75,0,18],Cpi1=[52,0,25,0,77,0,84],
Cpi2=[72,0,85,0,14,0,93],coefficients ascending. The four targets have
intrinsic factors(E1,F0),(E2,F0),(E0,F0),(E0,F0). Their exact cover
identities and entire transported two-kernels are already accepted in
128eb1/d67ccf and native925b02/f04008. They are distinct explicit models
within the fixed72-Jacobian catalog. [pcc:unverified]

For any one of these smooth sextics y²=f(x),choose a simple branch point
W=(a,0),L=f'(a)≠0 and a leading root rho²=c6. Put

    w=L/(x-a),   z=y*w³/L,
    G(w)=w⁶*f(a+L/w)/L².

The cancellation of f(a)=0 gives a monic degree5 G. Smoothness and the
inverse x=a+L/w,y=z*L/w³ identify the smooth projective curves;the unique
odd-model infinity is W. Original infinities become(0,±L²*rho),so their
classes relative to W are q and-q. This changes q.v to L²*rho without
twisting f. Chart::new uses rho=1 on the old monic models,retaining the old
transform arithmetic. Full37 includes all prior35 tests and the new adapter
test,not just checks of the new equations. [pcc:unverified]

Old IndependentMapChart.support_chart_homogenization/175646 and
ordinate_chart_homogenization/fe086f give the full-polynomial substitutions
through degree4 under the cleared relation(x-a)w=L. The new
EvenJacobian.nonmonic_infinity_square/2ac389 proves the infinity-square
identity from rho²=c6. Actual curve/Picard/projective realization and
invertibility are written/native inputs. [pcc:unverified]

All six branch abscissas lie in F101^6:the direct quotient sends x² to an
elliptic2-torsion abscissa in F101^3,so x lies in its quadratic extension.
The factors have irreducible mod2 Frobenius polynomial X²+X+1. Thus a,L
and rho lie in F101^6 (rho already lies in F101²),and the chart is defined
there. Rational divisor classes have unique reduced odd Mumford pairs over
that field. Their degree≤2 supports therefore split over F101^12.
Geometric4/7 inputs admitted over F101^6 have the same bound. This explains
the existing field engine used by the evaluator;it makes no claim that
primitive49 points fit in that field. [pcc:unverified]

## 2. Actual pullbacks,including every exceptional fibre

Undo each checked target scale(s²,s³). For cubic short parameters a,b/3,
the direct degree2 fibre is

    U=x²-(X-b/3)/a,   V=Y/a.

The reciprocal fibre is

    t=a/(X-b/3),   U=x²-t,   V=(Y*t/a)*x.

The code checks f-V² modulo U,including repeated x=0 support. The origin
pulls to identity. Direct origin fibre is the infinity sum;reciprocal
origin fibre is the x=0 sum,and div(x) identifies those sums in Picard.
For a reciprocal signed infinity image,Y=±a*rho and the fibre is twice
the corresponding infinity,so its class is±2q. [pcc:unverified]

At a nonzero elliptic2-point whose fibre contains W,write U=(x-a)h.
Then W+W_other-infinity_sum is W_other-W,because2W is equivalent to the
infinity sum. Its odd-model Mumford pair is u=[L/h(a),1],v=0. This
handles the branch corner without dividing by U(a). The full raw support
transform is used in all other finite fibres. [pcc:unverified]

Degree2 covers are optimal (a degree1 factor would identify genus2 and
genus1). The two elliptic factors are geometrically nonisogenous,so cross
Hom is zero. Bruin–Doerksen0902.3480 Lemma2.6 identifies the product pull
P and proper norm H as adjoints,of degree4,with HP=PH=[2]. The actual
four-point graph is exactly the kernel,not a group inferred from a count.
All sixteen geometric4-pulls are distinct;all256 sums and order4 hold in
each factor embedding. Rational109/113 points,ramified/repeated/pole
fibres,both signed infinity images and all four kernel elements are checked
by the new adapter test. [pcc:unverified]

## 3. Norms and Frobenius from Mumford input

A reduced odd pair of degree d represents ΣP_i-dW. Decode all support
roots with multiplicities through the inverse chart;w=0 uses the original
signed infinity. Proper divisor pushforward along either cover is

    H(D)=sum cover(P_i)-[d]cover(W).

This is the algorithm's input norm. It neither receives a planted scalar
nor uses a corpus index. Lazy norm caches are keyed by actual(u,v),and pull
caches by actual elliptic coordinates;on a miss the stated arithmetic runs.
The complete corpus uses these caches for correctness testing,not as
cold-cost evidence. [pcc:unverified]

Since cover(W) is a nonzero elliptic2-point,the omitted correction changes
every admitted degree1 example. On degree2 it vanishes,so the rational
corpus alone would miss that mutant. The code checks every finite F101
curve point and both extension infinities as degree1 classes. Witness counts
per factor are120 for C0/Cpi1 and122 for C10/C20/Cpi2.
EvenJacobian.degree_two_basepoint_cancels/636360 and
degree_one_omission_detected/88bf4f prove the exact module consequences;
actual cover(W) is supplied by the checked geometry. [pcc:unverified]

Coordinate Frobenius on the odd coefficients would change the chart and
basepoint. Instead decode support,apply coordinate Frobenius on the original
sextic,return to the same W chart,and subtract[d](W^pi-W). Indeed

    pi(D)=sum(pi(P_i)-W)-[d](pi(W)-W).

EvenJacobian.frobenius_basepoint_rebasing/324cb4 proves this accounting at
explicit module hypotheses. The native adapter checks it on degree1 classes
and all geometric4-pulls;every whole-domain input and output is checked
Frobenius fixed through this actual transport. [pcc:unverified]

## 4. Four transfers,their adjoints and recovery

Let phi on the product be(alpha,id),(alpha2*alpha,id),(pi_E,id) or
(pi_E²,id),and let F be the global map defined by the previously identified
stable kernels. The quotient identity F*P0=Pt*phi implies globally

    [2]F=Pt*phi*H0.

On rational groups of order12317,the inverse2 is55 on order109 and57 on
order113. The executable formula is therefore proper H0 norm from the
Mumford input,those public halves,actual factor map,and actual target pulls.
For partial Frobenius the rational factor map is identity. This identifies
the evaluated function with the global isogeny's rational restriction,
using injectivity of[2] only on that odd rational domain. [pcc:unverified]

Adjoints use the actual normalized SevenEdge.reverse maps,with calibrated
scale29;two-hop reversal applies them in reverse order. For Frobenius,V*pi
=[101] and pi=id on rational points give V=[101] there. Hence the adjoint
formula is P0*phi^dagger*Ht/2 on the same domain. Global norms descend
diag(d,1),with d=7,49,101,10201,as written in128eb1/d67ccf. Recovery applies
the public inverse of d on the E109 coordinate:78,89,68,46,respectively,
and leaves F113 unchanged. [pcc:unverified]

EvenJacobian.public_unbalanced_norm_inverses/7908b9 checks the7/49 CRT
representatives6329/1357 and inverses11414/2487. Earlier
PartialFrobenius.public_moduli_and_norms/72739d checks101/101² and their
inverses. The new uniform_mutant_coordinates/8ba779 proves in Fin113 that
d*b=b iff b=0 for all four d and checks109*112=12208. The actual rational
product identification is a written/native bridge,not encoded as an AV axiom.
[pcc:unverified]

## 5. Complete finite-domain acceptance and the chain control

All109×113 actual source elliptic point pairs give12317 valid,distinct,
Frobenius-fixed C0 classes,so they exhaust its rational group. For each of
four targets,the evaluator sees only those Mumford inputs. Product-point
witnesses are used separately to assert expected outputs. Each target has
12317 distinct valid Frobenius-fixed images,actual adjoint norms and inverse
recovery. The wrong uniform norm differs on12208 inputs and agrees on109
in every case,matching the preregistered prediction. [pcc:unverified]

Every identity/negation and translation by both source generators is
checked. Those generators span the order109 and113 factors;translation
identities on the whole finite domain therefore imply full additivity.
All109/113 embedded scalar images and exact injective orders are checked
explicitly as well. These establish DLP preservation at the actual bounded
fixture scope;they do not constitute a new DLP solver or growing-size result.
[pcc:unverified]

The actual divisor chain C0→Cpi1→Cpi2→C0 is evaluated on every input and
returns it. Applying the extra E-negative involution gives actual Cantor
scalar6103 on all12317 classes. Both Frobenius cases also satisfy the
cleared direct-bridge identity[2]F=PtH0 on every class. The earlier negative
comparison remains:that direct bridge already preserves the odd-prime
logarithm and has identity factor stage. No additional factor-evaluation
advantage over it is established. [pcc:unverified]

Six new exactly bound EvenJacobian declarations pass locally,gate02/zero
trusted native;project census183 written/36 modules. Actual AV/Picard,
kernel/degree and field-realization bridges remain written/native. Formal
attempt01/044c8c failed on variable-scalar normalization;its source/diagnostic
and compiler-recovery sorryAx are retained and are not accepted proofs.
The corrected proof uses explicit zsmul_sub and additive cancellation with
unchanged statement/hypotheses. [pcc:unverified]

StageC still requires actual geometric7 kernel images/Frobenius/sums and
the degree24 cyclic49 kernel-polynomial/division/gcd certificate. Full37
does not enumerate primitive49 points or prove a cost advantage. Source-linked
reports,raw custody,protocol6f1e93 and the shared immutable engines preserve
the exact remaining scope. No paid experiment or benchmark was launched.
[pcc:unverified]
