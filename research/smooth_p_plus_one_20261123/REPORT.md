# Distinct Dickson images in four-summand extraction

**2026-11-23 · Theory and development checks · [pcc:unverified]**

On the regular RR5 chart, pushing all four roots to the same Dickson terminal value leaves a Jacobian of rank at most one in the three chart parameters. Stopping at four distinct images gives rank three. An exhaustive C++ check over \(\mathbb F_{127}\) found rank one on all 1,038 regular signed relations after full collapse and rank three on all 180 relations admitted by the distinct-image chart.

Petit, Kosters, and Messeng introduced prime-field factor bases defined by composed maps. The RR5 monic quartic and the compact quotient-product construction are prior work. The coefficient pushforward itself is the familiar Graeffe transform; the earlier norm study explicitly leaves the full terminal Graeffe intersection's reducedness open. This note studies the intermediate cutoff and the local algebra of the even, four-summand chart. The research KB service was unavailable in this runtime and the repository wrapper reported no executable. Sources were checked through the published paper, the repository files, and the live RR5 PR. [PKM](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf) · [Symmetrized summation polynomials](https://www.iacr.org/archive/eurocrypt2014/84410158/84410158.pdf) · [RR5 gate](https://github.com/isogeny-labs/crypto/blob/f0ca664e/research/prime31_rr5_quartic_20261122/REPORT.md) · [Compact controls, PR 4953](https://github.com/isogeny-labs/crypto/pull/4953) · [Graeffe and reducedness review](https://github.com/isogeny-labs/crypto/blob/f0ca664e/research/prime_one_shot_20261029/continuation/reviews/torus-and-reducedness.md)

![The same quartic before and after the last Dickson steps.](diagram.pdf){width=100%}

## Construction

Let \(E\) be a nonsingular short Weierstrass curve of odd order over \(\mathbb F_p\), with \(p>3\). Fix a nonidentity target \(R\). Work on the open chart in which the four input abscissae and \(x(R)\) are distinct. The five points formed by the four summands and \(-R=(u,v)\) sum to the identity. The normalized RR5 function is
\[
F(X,Y)=(X-u)(\alpha+\beta X)+(Y-v)(X+t).
\]
Its norm gives the monic quartic \(Q(X)=-\operatorname{Norm}(F)/(X-u)\). The coefficient of \(XY\) is one. A function with that coefficient zero would have pole order at most four at the identity and could not vanish at the five distinct points.

Let \(q=2^d\mid p+1\) and take a regular norm-one torus fiber \(D_q(X)=c\) with \(q\) simple rational roots. Write \(f(X)=X^2-2\) and choose \(2\le h\le d\). Apply \(f\) to the four roots of \(Q\), with multiplicity, \(d-h\) times to obtain \(Q_{d-h}\). The distinct-image chart is the monic divisibility condition
\[
Q_{d-h}(X)\mid D_{2^h}(X)-c.
\]
The suffix selector splits and is square-free. The condition holds exactly when the four original abscissae are in the full selector and their intermediate images are pairwise distinct. For \(h=2\), it is just the coefficient equality \(Q_{d-2}=D_4-c\). It calls for one root from each of four intermediate fibers and no enumeration of those roots in constructing the equations.

For the pushforward, write a monic quartic as \(Q(Z)=U(Z^2)+ZV(Z^2)\). The quartic with roots \(x_i^2-2\) is \(U(X+2)^2-(X+2)V(X+2)^2\), directly from \(Q(Z)Q(-Z)\). Each pushforward is quadratic in the four coefficients.

## Why the cutoff changes the local algebra

At a regular relation the four simple roots of \(Q\) can be lifted in the completed local ring of the three parameters. On the collapsed encoding, take \(z_i=D_q(x_i)-c\). The four coefficient conditions are the elementary symmetric polynomials \(e_j(z_1,z_2,z_3,z_4)\) for \(1\le j\le4\). The \(z_i\) vanish at the relation. Consequently \(e_2,e_3,e_4\) have zero linear part and the Jacobian has rank at most one.

On the distinct-image chart, any first-order variation of a monic divisor of the fixed square-free suffix polynomial is zero. Equivalently, the root-to-coefficient Vandermonde is invertible at the four distinct images and the suffix roots cannot move. The map \(f^{d-h}\) has nonzero derivative at each original abscissa, since the full selector \(D_q-c\) is separable. Thus each original root has zero first-order variation. The ordinates also have zero first-order variation, since they are nonzero on this odd-order curve. The corresponding variation of \(F\) lies in \(L(4O)\) and vanishes at the four summands and \(-R\). A nonzero function in \(L(4O)\) can have at most four zeros. Therefore the three parameter variations all vanish. The three-column Jacobian has full rank and the selected local scheme is reduced.

In fact, the collapsed scheme has local length at least six at each regular solution. It is finite on this chart because the possible selector roots and their signs form a finite set, and the normalized function with a given divisor is unique. If the first equation has a nonzero linear part, eliminate one parameter. The remaining three generators have orders at least two, three, and four in two parameters. The degree-two piece of the associated graded quotient has dimension at least two and the degree-three piece at least one. Together with the constant and the two tangent directions, this gives length at least six. If the first equation has zero linear part, at most two independent quadratic initial forms can occur, leaving at least four degree-two dimensions in three parameters. This case also has length at least six. Graph variables that uniquely record the pushforwards preserve these local rings. This is a handwritten local-algebra argument with a native finite-field differential replay. A checked Lean proof remains open.

## Exact replay and bounded development runs

The fixture is \(Y^2=X^3+X+7\) over \(\mathbb F_{127}\), of prime order \(109\), with \(G=(1,3)\). The recovery probe uses public \(Q_{\text{target}}=(11,98)\). The norm-one group has order 128. The C++ verifier generates the selector, enumerates signed quadruples, constructs their RR5 functions, and checks coefficients against independently transformed roots. The symbol \(q\) denotes selector size before lifting.

| Selector size | Unsigned base size | Regular signed relations | Distinct-image signed relations | Collapsed rank 1 | Distinct rank 3 |
|---|---:|---:|---:|---:|---:|
| 8 | 3 | 0 | 0 | 0 | 0 |
| 16 | 8 | 1,038 | 180 | 1,038 | 180 |

The eight lifted abscissae for \(q=16\) had intermediate pool sizes \(2,2,1,3\). Their support admits \(12\) of the \(\binom84=70\) distinct unsigned quadruples before target checks. More generally, a regular selector of size \(q\) has \(2^h\) equally sized intermediate fibers. Before elliptic lifting, the admitted fraction is \(\binom{2^h}{4}(q/2^h)^4/\binom q4\). At fixed \(h=2\), it tends to \(3/32\). [Native receipt](evidence/native_replay.txt)

The following run records are scoped development diagnostics from the original standalone Python equation builder driving single-thread msolve 0.7.5. The JSONL projections preserve exact outcome and workload fields. In the four-query \(q=16\) panel, stopping at distinct images returned exactly \(1,0,3,3\) signed relations and the full expanded remainder returned \(9,10,13,13\). The compact remainder ladder and the collapsed coefficient chain each timed out on all four at the three-second cap. The compact ladder tested here is a sequence of remainders of the Dickson iterates. PR #4953 uses a different compact quotient-product system. The \(q=8\) run has only three lifted points and serves as an empty-chart control.

In a single public mixed-target \(q=16\) development run, the separated chart recovered and verified scalar 69 after eight oracle calls; the expanded remainder recovered scalar 69 after thirteen calls, four of which timed out. The direct pair-index control also verified 69. The compact ladder exhausted its 30-draw budget. The historical checks include setup, signed recovery, and left-kernel elimination, but lack an admitted native performance comparison across curves and sizes. No leaderboard or performance graph changes follow from these development records. [Four-query panel](evidence/f127_q16_msolve_development.jsonl) · [Empty control](evidence/f127_q8_msolve_development.jsonl) · [Mixed-target recovery](evidence/f127_q16_mixed_target_development.jsonl)

## Reproduce the exact native check

Run the following from this directory.

    g++ -std=c++20 -O2 -Wall -Wextra -Werror verify_four_summand.cpp -o /tmp/verify-dickson-cut
    /tmp/verify-dickson-cut

The native source is the exhaustive checker. The next cold benchmark needs a native discovery path and identical queried curves and budgets across the distinct-image chart, the compact quotient-product and oriented S3 controls, and the direct baseline. Coverage loss and every failed call enter the total.
