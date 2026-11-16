# A fixed F101 (7,7) transfer between distinct Jacobians

The compatible gluing construction gives an actual separable F101 isogeny
Phi:J0→J1 of degree 49 and polarization multiplier7. Its rational evaluator
and its dual pass both entire12317-class domains and the complete original
order109 source subgroup. The full geometric2-kernel inclusion is checked,
and the global quotient/polarization argument is given below. This is a
written mathematical proof with partial Lean helpers and producer-run
certificates;scientific standing remains unsigned. [pcc:unverified]

## Fixed objects and provenance

All fields in this statement have characteristic101. Put

    C0: y²=x⁶+6x⁴+13x²+56,
    C1: y²=x⁶+26x⁴+42x²+68,
    J0=Jac(C0), J1=Jac(C1).

Both curves are smooth and have two rational infinities. Both rational
Jacobians have12317=109*113 classes. The corresponding short elliptic
factors are

|Factor|Before|After the selected actual7-isogeny|Order/trace|j before/after|
|---|---|---|---|---|
|E1|y²=X³+X+46|y²=X³+26X+58|109/−7|12/87|
|E2|y²=X³+44X+21|y²=X³+69X+50|113/−11|58/92|

The selected kernels,bottom coefficient first,are[72,100,35,1] and
[59,46,8,1]. The maps and normalized duals are the unchanged CT19
Kohel construction already independently replayed with another Montgomery
field/polynomial/kernel engine in cryptanalysisef9a54259/PR1483.
Native inputs stay pinned at crypto75a737b8,separately from102 construction
and five verifier source guards. [pcc:unverified]

The four-model gluing inventory was published during the interruption in
cryptoPR3920,headcc3229d103638190a08d4948b103004cbd567768. Its historical
Python oracle is not rerun here;the native Rust replay corroborates the
four raw sextics and actual quotient identifications. Its E2 A/B labels
reverse our predeclared lexical kernel ordering. Our first supported
lexical pair is[1,0],raw[19,0,86,0,34,0,79],raw w=33Y,and the monic C1
above. The other supported chart has monic[33,0,44,0,92,0,1]. Raw leading
41/46 are nonsquares:their direct monic normalization remains unsupported,
with no silent retwist. This is reuse of HLP gluing,not a new model discovery.
[pcc:unverified]

## Actual quotient coordinates

For a monic even sextic x⁶+a2*x⁴+a4*x²+b,the two long elliptic quotients are

    (U,Y)=(x²,y),       Y²=U³+a2*U²+a4*U+b,
    (V,Z)=(b/x²,b*y/x³),Z²=V³+a4*V²+a2*b*V+b².

They extend across zero and infinity. The first elliptic short coordinate
is X=U+a2/3;the second is X=V+a4/3. For C0 these shifts are2/38 and the
component order is E1,E2. For C1 the shifts are76/14,the component order
is E2',E1',and both short-coordinate identifications multiply(X,Y) by
(21²,21³). Both coefficient equations and complete point-set bijections
are checked;an unchecked scale candidate is never an isomorphism verdict.
[pcc:unverified]

Write N0:J0→E1×E2 for both actual norms and P0 for the dual sum of pulls.
Write N1:J1→E1'×E2' in the matched ORIGINAL component order,and P1 its
dual. The explicit chart swap/scales above are included in N1/P1.
For sigma:(x,y)→(−x,y),the two degree2 covers have involutions sigma and
iota*sigma. The hyperelliptic involution iota acts as−1 on the Jacobian.
The usual pull/norm identities therefore give

    P_i N_i=[2] on J_i,       N_i P_i=[2] on the product, i=0,1.

The cross norm is zero:the opposite involution eigenspaces make its double
zero,and a homomorphism of elliptic varieties killed by2 is zero. These
are global Picard/pull-norm identities,not an inference from sampled points.
Their existing written argument and bounded full-class implementation are
in cryptanalysis:SPLIT_QUOTIENTS.md. Each P_i,N_i has degree4 and the two
are adjoint for the canonical principal polarizations. [pcc:unverified]

## Full geometric kernel inclusion

Let alpha=(alpha1,alpha2) be the selected product of the actual degree 7
elliptic maps. Put A=P1 alpha N0. Every factor is an F101 morphism.
The characteristic is prime to2 and7. The six roots of C0 lie in F101³;
their fifteen unordered branch-pair divisor classes plus zero are the
sixteen geometric J0[2] points. All are constructed as actual balanced
divisors and checked distinct/order2. [pcc:unverified]

Let a_i be the roots of a³+6a²+13a+56. The paired elliptic2-torsion
abscissae in short coordinates are a_i+2 and56/a_i+38. The norm of a
branch-pair class is computed by pushing its two ramified finite points;
both infinities push to O. Opposite roots give zero;different indices give
the corresponding remaining elliptic2-torsion pair. The actual image is
the four-element matched graph,with four kernel elements. This is checked
on all sixteen classes,not only F101-rational2-torsion. [pcc:unverified]

Under alpha,the graph becomes the matched graph used by HLP to construct
C1. In actual C1 long quotient coordinates,each nonzero pair(U,0),(V,0)
satisfies UV=68. The first pullback has Mumford support x²−U;the second
has support x²−68/V. These are equal,with ordinate0 and identical balanced
infinity count. They are2-torsion,so their sum is zero. The existing tower
Jacobian engine checks the actual pullbacks/addition on every source2-class;
A annihilates all 16. Since J0[2] is finite etale,annihilating these geometric
points annihilates the whole subgroup scheme. [pcc:unverified]

The extension pull implementation also agrees with the existing independent
coordinate path (reciprocal/balanced engine) on all 444 elliptic points of
both old/new products,including four origins,four ramified U=0 fibers and
four V=0 infinity fibers. For V=0,Z=±b the pullback is the balanced class
2*infinity±−K. No zero multiplicity or infinity contribution is dropped.
This is an arithmetic-path comparison,not a second independently signed
scientific verdict. [pcc:unverified]

## Global factor,degree and dual

Multiplication[2]:J0→J0 is the separable quotient by J0[2]. By the universal
property of the quotient,there is a unique regular homomorphism Phi with

    A=Phi [2].

This uses the full geometric kernel inclusion above. Over an algebraic
closure it follows from Milne AV v2.0 Remark8.12;uniqueness descends the
map to F101,since A and[2] are F101-defined. The factors of A are
isogenies,so Phi is an isogeny as well. Degrees give

    deg A=4*49*4=784,       deg[2]=16,       deg Phi=49.

In particular Phi is separable. This argument supplies a global morphism;
rational halving alone would not. [pcc:unverified]

Taking polarized adjoints gives Ahat=P0 alphaHat N1. Using the actual
elliptic dual identities alphaHat*alpha=[7] and both norm/pull identities,

    Ahat A=P0 alphaHat [2] alpha N0=[28],
    A Ahat=[28].

Since A=Phi[2],the first identity becomes[4] PhiHat Phi=[28],and similarly
in the other direction. The Hom group of abelian varieties is torsion-free:
an integer-killed homomorphism has connected image contained in a finite
kernel,so its image is zero. Canceling[4] thus gives

    PhiHat Phi=[7],       Phi PhiHat=[7].

Equivalently Phi has polarization multiplier7 (type(7,7)),not28.
Cancellation takes place in a Hom group;the geometric point groups DO
have nonzero2-torsion. The Lean cancellation helper explicitly retains
this distinction. [pcc:unverified]

## Rational evaluator and original source

All relevant rational groups have odd order. On J0(F101),the unique global
Phi therefore evaluates as P1 alpha(N0(D)/2),halving the109/113 elliptic
components by55/57. The dual uses P0 alphaHat(N1(Q)/2) with the same
original component ordering. This equals the restriction of the global
factor because norms commute with[2];it is not a separate construction
by scalar labels. Only coordinates are read. [pcc:unverified]

The code checks both entire12317-class domains,both image-set bijections,
both signed dual norms[7],and12317 fixed paired-addition cases in EACH
direction. It checks coefficient/model validity through the balanced APIs.
These are all-class norm/bijection checks,not all12317² addition pairs;
global additivity follows from the morphism construction. The full original
order109 subgroup has a nonzero image,and every scalar relation passes.
[pcc:unverified]

From Phi P0=P1 alpha (cancel[2] in the defining diagram),the source image
f*(P)=P0(P,0) has target elliptic coordinates

    N1 Phi f*(P)=([2] alpha1(P),0).

The reversed source norm is N0 PhiHat Phi f*(P)=([14]P,0). Recovery uses
39 mod109;the Jacobian source-class inverse7 is78. Every109 scalar,
including zero,is checked. The direct elliptic7 map remains an explicit
control for future solving-cost comparisons. [pcc:unverified]

## Unpolarized F101 Jacobians are distinct

Each elliptic pull into J0/J1 has kernel contained in elliptic2-torsion,
because its matching norm/pull composition is[2]. All three nonzero
geometric2-torsion images of each old/new factor are explicitly distinct
and nonzero. The kernels are therefore trivial as group schemes,and the
elliptic factors embed. [pcc:unverified]

There is no nonzero F101 homomorphism between the two trace types−7/−11:
a nonzero homomorphism of elliptic curves is an isogeny,and isogenous
curves over F101 have the same Frobenius trace. Hence any F101 elliptic
subvariety of trace−7 in J0 lies in the identity component of the other
norm's kernel. That component is one-dimensional and contains the embedded
E1,so E1 is the unique such elliptic subvariety. The same reasoning applies
to J1 and E1'. An F101 isomorphism of the unpolarized Jacobians would
restrict to E1≅E1',contradicting j12≠87. Thus J0 and J1 are not isomorphic
as F101 abelian varieties. No geometric-over-the-algebraic-closure
nonisomorphism claim is made. [pcc:unverified]

## Formal/evidence scope and next chain

Consumer PCCef4a7b/6a1231/c78fb3 bind the native inventory,full rational
evaluations and geometric kernel/embedding receipts. Source snapshots and
failed toolchain/Clippy diagnostics remain retained. Root PCCb320bb binds
eight SevenJacobian.lean helpers:cleared/matched supports,kernel fiber
constancy,unique FUNCTIONAL descent/additivity,conditional torsion-free
Hom cancellation and order109 recovery. The gate audits8 declarations,
0 trusted native uses;the project total is80 written lemmas. Functional
descent does not formalize regular algebraic morphisms;actual finite fields,
Picard functor,pull/norm identities,finite-etale kernels,polarized quotient
theorem and elliptic-factor uniqueness remain the written bridge above.
No weaker Lean helper is presented as the full geometric theorem.
[pcc:unverified]

Polynomial records IDC1hef947795d0f8d235 and IDC1he970a46be821e787 certify
the coordinate scaling and support-clearing identities through the unchanged
native identity checker (32 issue+16 extra points,modulus2^61−1),with exact
SHA256 pointers to separately checked Lean CommRing proofs. The checker
does not execute Lean or express noncommutative Hom composition. It does
not certify the global geometric argument. Root PCC3ed6c9 records this
exact scope;all scientific standing remains unsigned. [pcc:unverified]

Next compute Phi on all geometric2-torsion via64 product4-torsion lifts,
using Phi P0=P1 alpha. Transport the accepted R0 cross-kernel to select R1.
Only after actual kernel/normalization/duals checks may R1 Phi R0Hat factor
through[2] to a genuine next odd hop Psi from the accepted six-cycle target.
That continuation is not yet implemented by this checkpoint. S3/livebudget
admission still blocks benchmark/solver/paid work;the root frozen ledger/
archive gates and locked-atlas/KB gaps remain open. [pcc:unverified]

## Primary sources checked

- Howe–Leprévost–Poonen,arXiv:math/9809210v1,Section3.2 Propositions3/4
  and proof,https://arxiv.org/pdf/math/9809210 . Retained PDF SHA256
  1730ca25f09f57eeda0a703ed6917b58c9c924d497e132f2e6565b2dbacd095c.
- J.S.Milne,Abelian Varieties,version2.0,16March2008,Remark8.12 and the
  prime-to-characteristic quotient discussion,https://www.jmilne.org/math/CourseNotes/AV.pdf .
  Retained PDF SHA256
  f5ca4e63e5092a4b102daad1470e4cbed5fe8f82115e3a28c8881e3f67f6aaef.
- Existing SPLIT_QUOTIENTS.md and balanced/nonsplit code for global
  involutions and actual zero/infinity bookkeeping;CT19 exact elliptic
  identities/independent replay;external3920 inventory. No new prior-art
  absence or novelty claim. The KB CLI remains unavailable and atlas locked.
