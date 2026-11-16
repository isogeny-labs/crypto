#set page(paper:"a4",margin:(x:17mm,y:16mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.55em)
= Koblitz cover transfers: Round26
*Prym direction, exact cusp transport and a specified RR target* \
2026-10-28 UTC. Native exact computations,kernel proofs and labelled written
consequences; no IC cost,relation-rank or official promotion claim.

== Which direction preserves the intended factor
R15/R16 already identify the original involution
$sigma(lambda,nu)=(lambda+1+c,nu+b)$ with the primitive diamond involution.
All conjugate characters act by minus1. Native and kernel checks confirm
all60 units,10 equal character fibers,and twelve nontrivial subgroup norm
sums0,with four trivial controls1. Under the recorded modular/specialization
bridge,the invariant quotient norm and every nontrivial diamond quotient
norm annihilate the intended CM factor. This is not a newly computed full
Jacobian lattice matrix.

On actual minus11-torsion v,the difference acts by2 and6 recovers:
$v+sigma(v)=0$, $v-sigma(v)=2v$, $6(v-sigma(v))=v$.
The next map direction is therefore Prym/divisor differences. A nonzero
actual factor inclusion remains unconstructed. In characteristic2,this is
not a distinct minus-eigenfunction space. The AS generator $theta=nu/b$
satisfies $sigma(theta)=theta+1$ and $theta^2+theta=V/b^2$.

#image("round26.svg",width:100%)

The native source orbit is exact,not a fixture-only iteration. Universal
Tate and line transformations,base translation and all five original affine
lambda/nu coefficient residuals are checked. The written QR/source-normal
curve bridge supplies the proper chart interpretation and divisor pullback.

#pagebreak()
= Complete base-derived divisors and the next construction
Let $D_P=pi^*(P)$ be the two unramified source points plus7 times each of
the two tame points above a rational base cusp; degree16. The exact source
orbit transports verified O charts around O,A0,C0,D0,B0. Each has four
rational branches of indices1,1,7,7. Let $E_Z=pi^*(Z_5)$ be the retained
R23 points of degree10,weighted1 and7; degree80.

#table(columns:(1.8fr,.55fr,.55fr,.55fr,.55fr,.55fr,1fr),inset:4pt,stroke:.4pt+gray,
 [*Function*],[*A*],[*B*],[*C*],[*D*],[*O*],[*Extra*],
 [b],[-7],[1],[1],[2],[3],[none],
 [delta],[-16],[-4],[8],[5],[2],[+E_Z],
 [c],[-4],[-1],[1],[2],[2],[none],
 [b(b+c)],[-14],[0],[3],[6],[5],[none],
 [b+c+c²],[-8],[-2],[3],[5],[2],[none])

These are complete lifted divisors under the R22 base and explicit source
normalization bridges. They cover registered entries1,12,2,2,20,37. Their
pole degrees are112,320,80,224,160. Original lambda,nu,S,N valuations and
additional zero supports remain incomplete. The failed raw-Z C/D chart
assumed an incorrect t14 constant term;its residuals are retained and its
conditional series call is not launched.

The next prescribed divisor is $D=7D_A+D_B$,degree128. With the recorded
genus61,128 exceeds120,so Riemann-Roch gives $dim L(D)=128+1-61=68$.
This is a written geometric dimension deduction,not a constructed basis.
The next experiment must produce all68 basis elements and verify completeness
before searching for maps/correspondences. Another small monomial ansatz does
not satisfy that requirement.

== Verification and retained failures
Six native invocations:3accepted,1implementation rejection,1failed coordinate
ansatz,1expected corrupted-CRT rejection. Changing actual delta34 to identity1
causes errors,while native exit0 and a late marker still occur;wrapper1 rejects.
The native cap remains4GiB virtual/1200s;there is no timing comparison.

Lean4.34.1/pinned mathlib compiles16 named theorems. The transitive audit
checks17 declarations,including one generated equation,with0failing and
0trusted-native. CRT/character arithmetic,conditional norm/Prym recovery,
nonzero detection,equivariance and quadratic field identities are kernel
proofs. The modular,proper-chart,divisor and RR bridges remain written.
Lean01's elaboration errors and failed source are retained;see PROOF_MAP.md.

Root source is unchanged from7fcc0e26b. Its full library run hit1200s without
a final summary;the targeted replay was26pass/1inherited registry failure.
No green full-suite claim or weakened test is introduced. Shared daily log
scope is reserved by another task;SESSION.md retains this lane's facts.
All source/raw hashes and fresh artifact checks are separate publication
records. Both PDF pages and editable SVG are inspected before publication.

Next: original-coordinate Laurent valuations using the exact T orbit;
complete68-dimensional RR basis;actual Prym/F2 map and nonzero order11 image;
node187 membership/complete coupled arc;proper polarized embedding;arbitrary
reverse evaluation and ordinary460/367 images. R22 recovery d=1 is preserved.

Primary references: #link("https://jvoight.github.io/articles/cmfs-mf-101222-scrubbed.pdf")[Best et al.,§4.1],
#link("https://antsmath.org/ANTSXV/papers/ANTS-XV_molin-page.pdf")[Molin-Page,Thm37],
#link("https://www.ocw.mit.edu/courses/18-782-introduction-to-arithmetic-geometry-fall-2013/5859c5f7d2ad1eae5339d87dce144823_MIT18_782F13_lec22.pdf")[Sutherland,Cor22.23].
See CM.md for prior source records and every bridge.
