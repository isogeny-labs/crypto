# Fresh whole-cell hypothesis from rejected pair relaxations

This is a theorem and an unexecuted next experiment. No fourth C96 experiment is authorized or run.

For binary prefix D of dimension d, let E=span(D*D), e=min(2d-1,n), and write the exact pair relaxation as M(w)z=B with z the d+e bits of s=x+y and t=xy. Its columns are

    w^2 alpha^(2j)                 0<=j<d
    alpha^(2l) + w alpha^l         0<=l<e.

Every entry of this binary matrix is affine LINEAR in the n bits of w, because squaring is F2-linear. Suppose a rejected w0 has a verified dual lambda with lambda(M(w0)[:,j])=0 for every column and lambda(B)=1. Define the affine set

    C_lambda = {w: lambda(M(w)[:,j])=0 for every j}.

These are at most d+e binary affine equations in w. They contain w0. Therefore C_lambda is nonempty, has exact size2^(n-rank(A_lambda)), and at n83,d18 contains at least2^30 separator values. Every value in the WHOLE cell has no relaxed pair solution: applying lambda to M(w)z=B would give0=1. Consequently it has no original source pair either. A separate verifier can check the cell equations, w0 membership, full affine rank, lambda(B)=1 and annihilation of every original pair column on an affine basis; no enumerating2^30 values is needed.

For the actual B=1 fixture, the t0 column1+w requires lambda(w)=1, while lambda(1)=1. Thus1 is NOT a direction of C_lambda. This avoids the specific free-source unit-direction blindness from C92/C93: the learned cell is generally not a coordinate-prefix cell. It is not a contradiction to those negative controls.

The rank can be as high as53, so a cell may exclude only a2^-53 fraction of the full separator field. Cells overlap, and a large cardinality per certificate does not prove a small global cover, independent rejection rates, or a tractable outer search. Different duals at w0 can yield different ranks; minimizing that rank over the entire dual family is another potentially expensive search. Do not enumerate that family without caps. Product-support rejects inside the relaxed image need a different certificate.

Next fresh experiment: implement independent affine-cell certificate generation/verification; exhaust all w on tiny fields to check the whole set is empty and counts agree. Then freeze128source-independent GF83 separators and at most32deterministic dual choices each. Measure cell rank, construction cost, exact common direction space and certified union counts only when tractable; otherwise retain bounds. Compare these oblique cells with the old coordinate-prefix cells. A value passing the relaxation remains unresolved until exact product support. Initial90s,4GiB,onecore. No sampled rejection rate becomes a global solve or release claim.

For integration, keep the finite rebracketing separator explicit and retain the zero/infinity branch disjunction. Learned exclusions constrain that SAME global separator. Prove full original-chain coverage before adding them to an outer controller; both finite endpoint roots must be accounted for when rejecting an entire block. Ordinary prime fields need a different geometry: a nonzero Fp-linear functional there has no comparable large binary kernel.
