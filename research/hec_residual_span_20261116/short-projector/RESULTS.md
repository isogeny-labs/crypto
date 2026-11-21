# Certified short projector lowers component counts, complete-budget gate still fails

[pcc:unverified]. Freeze461a0eed144, input hashes passed, sole120CPU-bounded Rust invocation exit0. Exact cyclic integer witness h-c=sum b_j X^j(P-1)+b_nS holds for every tuple. Every canonical factor-base point passes (P-1)(sigma)Q=0, S(sigma)Q=0 and c(sigma)Q=[h]Q. Both fresh counted runs per arm agree exactly in full16vectors/outputs; all outputs valid/ell-annihilated; REDC identities pass. Scalar reference full vectors reproduce projection-floor exactly. Shared native group arithmetic is not independent proof/review.

|g|p|h|short coefficients c0..c(n-1)|scalar REDC|polynomial REDC|component count ratio|
|---|---|---|---|---|---|---|
|2|251|55|[-2,-1,-2,2,3]|38676|11978|3.228920|
|2|1021|1655|[1,-2,3,2,-4]|328458|63270|5.191370|
|3|211|29|[-5,-3,-2,-4,7,3,2]|31054|25012|1.241564|
|4|181|4445|[-3,1,2,-3,1,-2,-1,6,-2]|112428|21172|5.310221|
|4|271|112347|[-2,1,-2,2,-4,-1,0,3,3]|156816|16764|9.354331|

Ratios describe representative-projection base-field REDC only, not walltime or full solving. Group doublings are replaced in part by joint evaluation with cheap rotated raw-point additions. The counted polynomial includes coordinate rotations/signs, but character construction, reduction search, qualification and orbit/factor-base preparation are FREE in this optimistic comparison. Greedy reduction is not globally shortest and evaluator is not claimed optimal. All exact Frobenius polynomials, witnesses and move counts retained in CONTROL.txt.

## Complete-control budget

All80comparisons remain in COMPARISONS.tsv.70/80satisfy10P>B; ten comparisons with weaker controls do not, so do not claim all80reject this revised kernel. Crucially, all20workloads reject against their strongest equally informed completing arm: each best complete B satisfies10P>B, before any map/collection/LA/final work is added.

|g,p|polynomial P|best complete B for four fixed workloads|strongest-control rejections|
|---|---|---|---|
|2,251|11978|9912,9540,9098,3604|4/4|
|2,1021|63270|26744,30444,40458,38540|4/4|
|3,211|25012|18516,19210,20866,22048|4/4|
|4,181|21172|34966,47278,44678,41388|4/4|
|4,271|16764|46742,12008,43268,30876|4/4|

This rejects10x base-field-count reduction for a full alias solver retaining THIS polynomial setup, even with every remaining phase free. It is not a universal projector/circuit/runtime lower bound. h1original genus-two instance has no projection compression to exploit regardless. No full solver or new cross-base construction was run.

Decision: retain the exact small-coefficient projection kernel and certificates as a component improvement; do not tune it merely to reach a10x component headline. Both scalar and this materially different polynomial setup fail the complete-budget case. The alias-only route needs a genuinely different construction/full-cost case before further solver investment. Classical endomorphism scalar decomposition, no novelty or mathematical-breakthrough claim; full genus2+objective remains active/unachieved.
