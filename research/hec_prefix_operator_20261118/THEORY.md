# Prefix-circuit relation operator [pcc:unverified]

## Exact factorization

For a cumulative walk R_t=a_t D+b_t T+sum_j n_tj F_j, a split divisor gives signed sparse coefficient row d_i at time t_i. With unknown vector x=(L_0,...,L_(m-1),k,1), the homogeneous augmented row is

A_i x=d_i L-sum_j n_(t_i)j L_j-b_(t_i) k-a_(t_i).

Let each walk increment be a coordinate vector e_(j_t) on the same augmented coordinates: F_j maps to j, T to m, D to m+1. Initial state D+T has coordinate vector z0=e_m+e_(m+1). Then

A_i=d_i-z0-sum_(t<=t_i)e_(j_t).

This proves the prefix operator: initialize h=x_m+x_(m+1), then update h+=x_(j_t) at every trial and return d_i*x-h at each retained event. No cumulative dense row is materialized.

For the transpose let w_t=sum_(i:t_i>=t)y_i. The adjoint is

A^T y=sum_i d_i^T y_i-z0^T sum_i y_i-sum_t e_(j_t)^T w_t.

Compute backward, adding event weights to the suffix before consuming that time's increment. This ordering matters: the event includes its own latest step. Subtract total event weight at each of the two initial coordinates afterward. Multiple events at a time are supported. No signed or constant column can be silently dropped.

## Exact primitive budget

Let T be trials, r retained rows, s total signed decomposition terms (s<=g*r), and n columns. Literal forward uses s products and 1+T+s+r additions/subtractions; transpose uses s products and r+s+T+2 additions/subtractions. Both also traverse/store the trace, allocate outputs, and access vectors; integer indexing/modulo costs are not priced by these primitive counts. An explicitly assembled canonical sparse matrix uses nnz(A) products and nnz(A) additions under the literal reference loops. Cancellations or zero values may lower an optimized explicit implementation's work. The prefix formula does not prove a runtime improvement.

A schematic Krylov sequence of K products costs O(K*(T+g*r)) arithmetic/traversal, not O(K*g*r) total: T is charged on every iteration. Source validation, collection, root splitting, signed decomposition, trace storage, minimal-polynomial generation and reconstruction are additional costs. This factorization changes representation, not rows, rank, target information or relation count. It is classical circuit evaluation and reverse accumulation, not a new discrete-log attack or a novelty claim.

## Solver obligations and falsifiers

This gate does not implement Wiedemann, Lanczos or an end-to-end solve. The existing hybrid dense-step solver and stronger affine-forest pipeline are retained comparators, not replaced by the explicit multiply loop. Do not promote operator product-count ratios into solve speedups. A candidate must handle rectangular matrices, free FB coordinates and target-only determination; preserve all original equations and verify the original scalar. Forming A^T A over an odd finite field can lose information (nonzero vectors can have zero norm), so real-field normal-equation intuition is insufficient. Random diagonal preconditioning needs a specified success/failure/retry rule and exact residual checks; residual validation alone does not certify a target is determined when free variables remain.

Falsifiers: any mismatch in A*x, A^T*y, bilinear adjoint identity, geometry reconstruction or formula counts. Complete-cost admission requires comparing integrated cold costs against the strongest original-panel forest/control, including finite retries/failures and final verification. A multiplication-only gain or one genus does not satisfy10xgenus2+.

Sources checked: counted/native/src/ic.rs cumulative signed rows and explicit prior differenced-row regression; hybrid.rs already separates dense step columns; GROUNDED-DUAL-SCREEN.md exact target functional criterion; cumulative-envelope/RESULTS.md stronger prepared controls prevent using outdated full-base arms as headline. KB revision9b2119e8fbac482b98b50521c0d604bb: source:883ab40565c1f261353efc03ba04473e (research/notes/ecc2k130/RESEARCH_KOBLITZ_INDEX_CALCULUS.md existing block Wiedemann), source:4548c0e85d47f9299ddc900c047c266d (docs/ic/ECDLP_RESEARCH_TECHNIQUES.md existing finite-field preconditioned backend). Primary Wiedemann PDF access at https://www.cs.umd.edu/~gasarch/TOPICS/linalg/Wiedemann.pdf failed in this turn; no external algorithm theorem is imported from it. The displayed operator identities are proved directly above. No Lean proof claimed.
