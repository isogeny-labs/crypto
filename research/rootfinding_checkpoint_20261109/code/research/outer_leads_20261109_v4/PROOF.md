# A two-parameter dual certificate: amortize h rather than repeat the solver

Status: unexecuted derivation and portable control source. Conditional prime-field component, not original all-roots closure, a new complexity theorem, or novelty priority.

## Falsifiable hypothesis

For a fixed target and two distinct rational anchors, eliminate k before introducing lattice variables. The remaining elementary coefficients are bivariate polynomials of weighted degree at most four in residual sum S and h. A single exact dual certificate can therefore reject a nonempty h interval for every supported residual triple simultaneously. Prediction: on a preregistered full 5-prime anchor panel, at least one width >= 8 h block closes using one independently checked certificate, and the entire block controller costs less than checking its constituent h values separately. Either lack of closure or higher total cost refutes the implementation hypothesis. This is distinct from retrying the fixed-h dual test.

## Exact elimination

Work in Fp for a certified odd prime p, with E:y²=x³+a x+b nonsingular. Let three finite rational points (xi,yi), i=0,1,2 have distinct abscissae; the first is the target-zero point and the next two are anchors. Set Li(X)= product_{j!=i}(X-xj)/(xi-xj). For F=P+Qy with P=X³+rX²+sX+t and Q=hX+k, the three vanishing equations give

P=X³ + sum_i (-xi³-h xi yi-k yi) Li(X).

Writing Li,2 for its leading coefficient, r=A k+B(h), A=-sum_i yi Li,2, B(h)=-sum_i(xi³+h xi yi)Li,2. Let N=P²-(X³+aX+b)Q². Its leading coefficients are 1 and 2r-h². Since each prescribed point lies on E and F vanishes there, N is divisible by product_i(X-xi), even if the function is tangent at a point. The quotient C is monic cubic. If C has residual roots alpha,beta,gamma and S=alpha+beta+gamma, its X² coefficient gives

-S=2r-h²+sum_i xi.

For kappa=2A nonzero,

k=(h²-2B(h)-sum_i xi-S)/kappa.

This is an identity in Fp[S,h]. Substituting gives coefficients r,s,t of weighted degree at most two with wt(S)=2 and wt(h)=1; Q has the same bound. Hence every coefficient of N, and therefore the quotient by the fixed monic cubic, has weighted degree at most four. Write

C=X³-S X²+T(S,h)X-U(S,h).

Both T,U involve only the nine monomials 1,h,h²,h³,h⁴,S,Sh,Sh²,S². The bound deliberately permits coefficients that specialize to zero. Characteristic three causes no division by three and is included. Characteristic two is excluded from this reduction, because 2A is then zero. Every kappa-zero chart, repeated-abscissa/pole chart, nonrational-lift branch, different anchor pair, and global source/auxiliary ordering branch remains separately unresolved; the component must not omit them from any future global controller.

## Whole-h certificate

For a canonical interval h=h0+u with 0<=u<=H and h0+H<p, translate the symbolic polynomials before coefficient lifting. Enumerate the eight nonconstant monomials m=(S²,Sh²,Sh,S,h⁴,h³,h²,h), with h here denoting translated u. Let T=sum ai mi+a0 and U=sum bi mi+b0 modulo p. For any integers gamma,delta choose integer lambda_i congruent to gamma ai+delta bi modulo p and c congruent to gamma a0+delta b0. Every supported coefficient tuple satisfies

J=c+sum_i lambda_i mi-gamma T-delta U in p Z.

For source coordinates in the integer interval [0,M], S is in [0,3M], T in [0,3M²], U in [0,M³], with multiplicity and zero/repeated roots retained. An independent checker computes a rational or integer enclosure [L,R] of J on this entire real box. A monomial enclosure uses each nonnegative mi in [0,(3M)^s H^h]; exact Bernstein coefficients on a rectangle may tighten it without changing the identity. Independent T,U intervals make this an outer relaxation. If ceil(L/p)>floor(R/p), no multiple of p belongs to the enclosure, so no supported triple exists for ANY h in the block. The checker must verify the coefficient congruences and enclosure independently; search/LLL code is untrusted.

This is an exclusion certificate only. Failed certification means unknown, not a root or nonempty component. Its proof does not require C squarefree, and therefore applies to repeated residual roots. An exhaustive verifier must replay known planted positive h: any block containing such an h cannot possess a valid empty certificate.

## Cost, surviving dimension, and counterexamples

There are eight monomial coordinates plus T,U, so the PRIMAL affine value lattice has dimension 10 and two modular congruences T-sum ai mi and U-sum bi mi, determinant p². The separate DUAL CERTIFICATE lattice on (lambda1,...,lambda8,gamma,delta) has eight congruences lambda_i-gamma ai-delta bi modulo p, determinant p^8. Certificate discovery must not use the primal determinant as its own determinant estimate; it is a p-scaled primal-dual lattice up to signs. The true monomial variety has dimension two (S,h), but treating its eight coordinates independently loses six relations. Do not call this a reduction from 4D to 2D LLL: fixed-h primal enumeration was 4D, while this relaxation is wider. Each candidate certificate can be checked with O(1) big-integer polynomial operations; finding a useful one is a separate measured task. A complete disjoint canonical h cover needs ceil(p/(H+1)) blocks per anchor pair and sign branch. Source-pair enumeration can still cost O(|D|²). No block count may be hidden.

A whole-field independent-monomial box has a concrete obstruction. If M>=1,H=p-1 and any h-bearing lambda is nonzero, its coordinate contribution alone has interval width at least p-1, so the inclusive integer enclosure already covers all p residue classes and cannot avoid pZ. Thus full-field monomial-box closure requires cancellation of every h-bearing lifted coefficient (or a stronger dependency-aware enclosure). Small blocks do not imply this cancellation. H=0 is the exact fixed-h special case.

A generic-coverage extrapolation is false: planted repeated-source controls already exhibit h values with actual roots; every block containing one must remain nonrejected or be split. A universal closure claim is also disproved by the abstract coefficient chart T=U=0, which retains the zero triple at every h. The zero triple may be unavailable in a particular actual chart, but it is a valid counterexample to a coefficient-relaxation theorem claimed independently of the geometry. Small h widths might close while all global blocks remain too numerous. Promotion therefore requires both verified block closure and charged total controller benefit, then a separate large-width threshold study.

## Prior art boundary and next experiment

Linear dual certificates, LLL, exact polynomial interval/Bernstein bounds and Lagrange interpolation are classical. This packet proposes their application to the explicit jointly eliminated two-anchor elliptic norm chart. Primary-source novelty review must compare against index-calculus divisor formulations, Coppersmith/Howgrave-Graham modular small-root lattices, and certified polynomial branch-and-bound. No claim that this application is new to the literature is made.

Next experiment: first run oracle.py against independent literal norms and small supported triples; freeze unchanged five planted NIST inputs and a fresh source-independent anchor/h panel. Instrument a fresh block-search worker, widths [1,2,4,8,16], complete disjoint blocks and actual determinant/rank, reject only on verified congruence/range certificates. Compare every block to exhaustive per-h controls, include a planted h block, charge setup/search/checking/serialization and RSS. Budget 90 seconds, 4096 MiB, one core, <=3 experiments; cap states before allocation, unknown on limits. Return null original-root count and null qualified speedup. If width gain fails, next hypothesis is a curved monomial enclosure retaining S² and Sh² rather than the independent ten-coordinate box.
