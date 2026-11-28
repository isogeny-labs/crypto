#set page(paper:"a4",margin:17mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:false,leading:0.6em)
#text(size:18pt,weight:"bold")[High-genus prime-field covers]
#v(4pt)
#text(size:12pt)[Algebraic transfer and the explicit factor-base cost]
#v(8pt)
An explicit cover can meet the geometric conditions of the published
Enge–Gaudry–Thome (EGT) algorithm and preserve a large elliptic subgroup.
Its standard full factor base still requires exponentially many entries in
the original elliptic input size. This is a source review and hand argument,
with no new native experiment, formal geometry proof or novelty claim.
*PCC-20261115-c4aaf8 remains unverified.*

#image("pipeline.svg",width:100%)

Let E/Fp have equation y²=f(x)=x³+A x+B, with p>3, nonsingular f and
prime rational group order r comparable to p. Let k=ceil(log2 p), u the least
power of two at least k, v=u+1 and choose f(s)≠0. Eventually p>v. The map is
#raw("C: Y^(2u)=f(X^v+s);   pi(X,Y)=(X^v+s,Y^u)",block:true)
The shifted polynomial is squarefree. The tame Kummer model has one rational
infinity point and genus g=(2u−1)(3u+2)/2. A function-field tower gives
cover degree δ=uv. Thus g and δ are Θ(k²); the plane degrees are Θ(k).

The precise EGT parameter M=log(g log p)/log p is Θ(log k/k). Both plane
degrees divided by (g/M)^(1/3) grow as (log k)^(1/3); their product/g is≤3,
and g≥(log p)² eventually. These meet Theorem7's geometric conditions.
Heuristics5/6, order handling, arithmetic and target descent remain separate.

The standard divisor identity gives #raw("pi_* pi^*=[delta]"). Since δ is less than r eventually,
pullback is injective on the r-subgroup. Pullback support need not be fully
rational. The existing conditional Lean group theorem has its earlier
October22 receipt; it does not formalize this cover or its divisor maps.

#pagebreak()
#text(size:16pt,weight:"bold")[Charge the original input scale]
#v(7pt)
The paper's full factor base includes affine non-inert places of degree≤µ,
with µ≥1 eventually. Every affine Fp-rational point is such a place.
Hasse–Weil gives
#raw("#C(Fp) ≥ p+1−2g*sqrt(p)",block:true)
With g=Θ(k²), the affine population is at least p−2g sqrt(p)=p(1−o(1)).
Distinct rational points give distinct Abel–Jacobi classes for g>0, by
Milne Proposition2.3. Explicitly materializing this source base therefore
takes Ω(p) records. This is a bound on the named explicit procedure,
without an information lower bound against implicit representations.

Even merging projected duplicates does not remove the exponent. A degree-δ
map has at most δ distinct geometric points in a fibre. Identifying opposite
elliptic points removes at most another factor of two. Thus, eventually,
#raw("signed projected columns ≥ p/(4*δ) = exp(k*log(2)−O(log k))",block:true)
This lower bound for the full projected base requires no relation-yield,
random-rank or timing assumption.

Separately, the published heuristic guarantee is expressed at Jacobian scale
#raw("L_(p^g)(1/3,c+o(1));  log(T_bound)=Theta(k*(log k)^(2/3))",block:true)
after substitution of this family's parameters. That upper-bound expression
does not certify exp(o(k)) work for the elliptic input; it is not itself a
runtime lower bound. The explicit factor-base count above supplies the
separate charged-stage obstruction.

*Disposition.* The algebraic transfer exists. Applying this standard full
factor-base algorithm does not meet the requested subexponential prime-field
criterion. A smaller implicit base, compressed projected columns or a
different projected-norm/descent method would require another algorithm and
its own effective yield and complete cost. None is excluded by this review.
No high-genus benchmark, curve census or additional solver-cap run is launched.

*Sources checked (KB unavailable).* EGT arXiv:0905.2177v2, sections2–5,
Proposition3, Heuristics5/6, precise factor base on page9 and Theorems7/8;
Milne, Jacobian Varieties, Proposition2.3 and Corollary6.3; the existing
October24 prime-cover/norm report and October22 conditional transfer proof.
The historical group theorem assumes maps/composition; no fresh Lean build
or formal geometric bridge is claimed. PDF hashes and repository blob
bindings are in sources.txt, and the full hand argument is COVER-REVIEW.md.

This is a symbolic review, so its visualization is a proof-and-cost diagram,
not a numerical scaling graph. No canonical leaderboard or performance
value changes. All new scoped statements remain PCC unverified.
