# Two-parameter section sieve: incidence and rank admission

Status: derived candidate screen, not a measured gain or priority claim. The preceding goal turn is progress: it qualifies and measures the factored circuit, closing that implementation's tenfold cost case. This screen moves to useful relation yield while retaining the genus2/3/4 objective.

## Construction and exact relation

Let R=(u,v) be a tracked degree-g anchor on y²=f(x), monic deg f=2g+1, odd characteristic. For parameters (a,b), let w=v+u*(a+b*x). Since u divides f-v², H_ab=(f-w²)/u is a polynomial. For b!=0 it has degree g+2 and leading coefficient -b². For b=0 it is the existing degree-g+1 pencil. The principal divisor of y-w gives R+residual=0 in the original Jacobian, with pole order2g+2 at infinity in the b!=0 case. Exact multiplicities, shared support and source/target coefficient bookkeeping remain necessary. Enlarging the family is not a new known-log source.

For a rational point P=(x,y), u(x)!=0, set tP=(y-v(x))/u(x). It lies in the residual precisely on the parameter line a+x*b=tP. Thus two points with distinct abscissas determine one parameter pair:

    b=(tP-tQ)/(xP-xQ), a=tP-xP*b.

Equal-abscissa points do not produce a transverse pair; rational ramification points occur only once. This is an exact incidence reformulation, not a speed result.

## Pair-visit identity

Let m_ab be the number of distinct rational residual abscissas away from u=0, counting only one compatible point per abscissa. For fixed parameters and u(x)!=0, w(x) chooses exactly one y; hence no opposite-sign duplicate at the same x occurs. Over all finite parameter pairs,

    sum_ab binom(m_ab,2)
      = number of unordered point pairs with distinct abscissas.

Proof: each transverse pair determines exactly one parameter pair, and each such pair inside a bucket is counted once. For b!=0 a completely split squarefree disjoint residual has m_ab=g+2 and is consequently visited binom(g+2,2) times by naive pair enumeration. A scheme processing only a canonical pair must either avoid generating the other pairs or charge rejecting them; output deduplication does not erase pair-generation work.

## Conditional cost screen, not a theorem about yield

Under a random-polynomial splitting heuristic, there are approximately q²/(g+2)! fully split two-parameter residuals and q/(g+1)! one-parameter residuals. This heuristic may fail for these structured families; it is not a proved count or an independence assertion. With about q affine rational points, a naive all-pairs incidence pass has about q²/2 pair visits. Its estimated visits per full split residual are (g+2)!/2, versus (g+1)! point visits for the original pencil. The ratio is (g+2)/2:2,2.5,3 for genera2,3,4. Pair visits also require solving/interpreting parameters and are not equivalent-cost to original point visits.

Therefore a naive line-arrangement implementation has no current tenfold cost case; more raw parameter values alone are insufficient. This does not rule out avoiding most pairs, biased useful subfamilies, fast incidence algorithms, large-prime processing or different structured yield. Such a proposal needs an explicit generation-cost and rank argument rather than assuming generic split probability.

## The anchor still needs normalization

All these residuals represent -R. In the linear model let X be unknown factor-base logs and R=a0*D+b0*T. Every row has r_i*X+a0+b0*k=0 after normalization by D. Without an independently supplied source equation, the transformation X'=sX and k'=s*k+(s-1)*a0/b0 preserves every row when b0!=0. If the anchor projection a0+b0*k is nonzero, nonzero choices of s change k. This is the already qualified single-anchor ambiguity, not a new lower bound. More dimensions in the section family do not by themselves remove it. A smooth public-source row can break this symmetry, with its cost and availability charged equally to controls; multiple anchors can also break it, without guaranteeing full rank.

## Fixed-point slicing

Forcing a particular residual point P makes a=tP-b*xP. The resulting one-dimensional family is an affine slice of the two-parameter family. Removing the known linear root leaves degree g+1, but the relation also retains P's factor-base column. It is not automatically a known-log row and cannot be credited with a free normalization. The full cost includes selecting P, evaluating tP, scanning the slice, exact residual validation and elimination. This is a possible classical subfamily, not a tenfold construction.

## Decision and next falsifier

Do not execute a full two-parameter all-pairs campaign on the strength of family size. Admission requires a mechanism that avoids enough transverse pairs or provably concentrates useful independent relations, while retaining original targets, source-normalization costs, repeated/support cases, exact rank and final scalar checks. If a proposed filter only changes output order while still enumerating all pairs, the incidence count must remain in its ledger. No S3 or paid resource is required for this desk result; no scientific producer ran.

## Source boundary

Primary Improved sieving on algebraic curves, https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf, sections2.1–2.3, read: classical Mumford smoothness, Riemann–Roch pencil and rational-point bucket sieve. Existing PENCIL-NEXT.md already records this collision; it is not novel. The two-parameter incidence/count derivation above is our explicit algebraic screen, not an attribution of a performance result to that paper.

KB lexical query `hyperelliptic index calculus sieving polynomial pencil resultant relation generation`, revision9b2119e8fbac482b98b50521c0d604bb: source:6b2208c18d3ecd6afd0494c0c44c1a36 is a literature note on Nagao/Riemann–Roch decomposition over extension fields, not a same-instance execution result. Broad ecbench and elliptic-surface hits do not settle novelty. No absence/priority claim follows. [pcc:unverified]
