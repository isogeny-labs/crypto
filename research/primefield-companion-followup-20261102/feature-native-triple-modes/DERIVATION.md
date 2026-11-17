# Three selected modes from one anchor stream

Let h=(r-1)/2 and c_n=[X^n](X-1)^h=(-1)^(h-n) binom(h,n), with r caller-certified prime. For 1<=n<h,

    c_(n-1) = -n*c_n/(h-n+1),
    c_(n+1) = -(h-n)*c_n/(n+1).

Restrict to m>=3 dividing h and let L=h/m. A descending mode1 stream visits n=1+jm, j=L-1,...,0. Both neighbor denominators are nonzero modulo r. Each triple supplies (c_(jm),c_(jm+1),c_(jm+2)). Mode0 additionally has the known leading coefficient c_h=1, so its polynomial is V^L plus the mode0 component of the triple stream. No scalar/root search enters this construction.

A refill has count<=cap and steps=min(count,anchor.left-1). It requests the same2*steps numerator/denominator interval products as the mode1 stride stream, then jointly inverts steps+2*count nonzero denominators: stride denominators and both neighbors. The existing checked full-product inverse and terminal reciprocal helper are reused. The buffer holds count triples; denominator storage is at most3*cap, with a prefix of one extra slot. The mode1 initialization remains charged. The reader performs L-1 stride transitions and ceil(L/cap) refill inversions; no full coefficient vector is retained.

The quotient evaluator shares the prepared variable powers across all three components and restores the leading V^L term to mode0. It applies to arbitrary monic scalar quotients, including repeated-root and nilpotent fixtures; it does not require a root enumeration or a squarefree quotient.

For a direct-image update, let g be the old branch polynomial and g1=gcd(g,C0). Evaluate F0,F1,F2 in K[V]/g before the C0 gcd. Since g1 divides g, every representative modulo g remains valid modulo g1. Therefore gcd(g1, left*F2-right*F1^2), using representatives formed modulo g, equals the staged result that first reduces the quotient to g1 and then evaluates F1,F2 there. The zero-candidate condition is unchanged. Monic normalization, round cap, source query order, materialization and public-point verification stay on the existing path.

The optional recover_triple adapter retains the original initialization and substitutes only subsequent direct-image updates. The staged adapter is preserved. If the staged C0 step retains any branch, its separate F0 and F1 anchor traversals use L and L-1 stride transitions; the fused path uses L-1. If C0 eliminates every branch, the staged path skips the F1/F2 evaluation entirely. Fusion also adds a neighbor denominator per coefficient, a third evaluation component, a leading-power restoration and potentially more quotient work before pruning. Thus fewer stride traversals alone do not establish lower total work or elapsed time.

Qualification compares coefficients to compiled modes, quotient values to explicit scaled polynomials modulo monic fixtures, branch supports/materializations to staged updates, and public recovery outcomes/routes/calls to the original adapter. Scalar enumeration appears only in finite correctness fixtures. Producer checks are unsigned pending independent signed verification. [pcc:unverified]
