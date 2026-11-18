# Four-dimensional seed, seven-feature exact block certificates

UNEXECUTED changed mechanism. No source/evidence from masked_union_20261111_v1 is altered. This is a falsifiable reduction of certificate-discovery cost, not a novelty or speedup claim.

## Bottleneck and contract

The current candidate pays exact rational 7D LLL before pruning a finite h block. The baseline already solves a 4D fixed-h lattice. Hypothesis: use short coefficient functionals from a cheaper 4D seed and lift only their two output multipliers into the 7D feature chart, avoiding a 7D LLL. This may sacrifice pruning; all surviving fibers must still be solved exactly. The contract remains every ordered conditional residual tuple plus k in a DECLARED rational-anchor/h block. No original-global or binary/odd-extension chart completeness is claimed.

## Direct four-dimensional dual construction

At one preregistered pivot u*, write T=t0 V+t1 S+t2 and U=b0 V+b1 S+b2 mod p, with V=S². Define a dual vector (lS,lV,gamma,delta) by lS=gamma*t1+delta*b1 and lV=gamma*t0+delta*b0 mod p. An integer row basis is

    (p,0,0,0), (0,p,0,0), (t1,t0,1,0), (b1,b0,0,1).

Its determinant is p². Any exact row operations/positive diagonal metric scaling and exact unscale preserve the congruences; independently check every vector before use. Four exact LLL basis rows plus their signed pair sums give16 candidate rows (4+2*C(4,2)). Reduction completion is not required for soundness of an individually checked certificate. Bounds used to choose the pivot metric do not certify the rest of the block; every lifted vector receives freshly computed whole-block bounds.

## Optional reciprocal reuse of a completed primal basis

Let R be an INTEGER row basis for the unscaled homogeneous primal lattice L4 in coordinates (S,V,T,U). Its rows satisfy T-t0 V-t1 S=0 and U-b0 V-b1 S=0 mod p. Check all congruences and |det R|=p². Those imply R spans the entire kernel, because the congruence map to Fp² is onto. In particular p Z4 is a sublattice of L4.

Define A={w in Z4: R*w_column belongs to p Z4}. Since p Z4 is contained in L4, p R^(-1) has integer entries. The rows of p R^(-T) form A; their determinant has absolute value p^4/|det R|=p². Equivalently calculate signed cofactors/adjugate with exact divisibility checks; never truncate a rational inverse. For any row w=(wS,wV,wT,wU), take gamma=-wT and delta=-wU. Then wS=gamma*t1+delta*b1 and wV=gamma*t0+delta*b0 mod p. Native weighted bases must first be unscaled exactly by their actual column weights; using the weighted inverse directly is unsound. A cached primal basis from another public context is not valid input.

A fresh candidate should use EITHER direct4D dual reduction OR charged reciprocal reuse, with distinct named arms; do not pick the faster arm after viewing a holdout. Reciprocal reuse includes the pivot full solve cost and all determinant/divisibility checks. Its benefit could be absent if solving the pivot dominates.

## Exact lift to a whole h block

The fixed public chart supplies five field-feature coefficients a_i,b_i and constants a5,b5 for F=(W²,Wu,u²,S,u), W=u²-S. For any candidate gamma,delta, center them modulo the ACTUAL field prime p, and set lambda_i=center(gamma*a_i+delta*b_i mod p), c=center(gamma*a5+delta*b5 mod p). Then

    J=c+sum_i lambda_i F_i-gamma*T-delta*U

is divisible by p on every actual root, regardless of which pivot produced gamma,delta. This follows immediately by substituting the two field equations and reducing mod p. Even a random gamma/delta is sound after this check; the purpose of the 4D seed is useful size, not correctness. Centering changes only integer representatives and requires recomputing bounds.

For each u, independently compute the exact quadratic range in integer S=0..3M after W=u²-S, then add independent T and U interval bounds. A range with no multiple of p certifies that u empty. All other u remain UNKNOWN until the exact fiber solver completes. Union the retained tuples keyed by u and compare to a full all-h solve. Repeated roots and all permutations are retained. No predicted number of surviving u is assumed.

## Preregistered next experiment

Fresh source directory lifted_4d_certificate_20261112_v1; distinct controls and comparison IDs. Before measurements pin all source/input hashes and reviewer approval. Use the same five exposed positive width8 blocks as TRAINING ONLY, plus five post-result source-independent width64 blocks, and then a separately generated unopened panel if promotion is warranted. No current timing selects the holdout.

Controls: p=5,7,11,13,17, M=2, H=2, all ordered triples and u; verify both constructions, exact reciprocal integrality/determinant/congruences, signed and centered representatives, reciprocal of a deliberately corrupted or still-weighted basis rejected, changed public-key cache miss, repeated/empty supports and mask soundness against direct roots. Include no-pruning cases and a solver cap that yields unresolved rather than empty. Budget<=200000 independent scalar checks,<=128 basis transforms.

Comparison: preregister pivot u*=floor(H/2), fixed16 vector pool; direct4D arm versus prior7D cold discovery versus all-h reference. Three cold paired repetitions in a fixed alternating order, full discovery+root+independent replay+serialization charged. At most3experiments per15minute session, each90seconds4GiB1core. Whole-block completions require every excluded u certified and every survivor complete. Report capped arms with null ratio. Record per-process RSS and whole worker time; compilation common and separately reported.

Falsification: reject the claimed cost improvement if the complete conditional union differs, any true root is masked, or the paid median does not decrease relative to7D with the SAME complete result. Fewer candidate coordinates or apparent vector norms alone are not success. If4D vectors prune poorly, the next changed assumption is a small preregistered multi-pivot pool or exact simultaneous reduction of two pivots; do not merely rerun the same seed longer. A positive component observation still does not qualify the genuine F4/F5/F6 release gate or the user8x target.
