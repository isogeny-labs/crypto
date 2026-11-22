# W8 sign-vector Fourier/Pluecker experiment — prospective native Rust gate (2026-11-18)

Tracking: isogeny-labs/crypto PR #4728. Source curve (unchanged): y^2=x^3+847514846x+2087444724 over p=2147483647; n=2147446831, cofactor 1. Reuse the pinned original Prime31 Rust crate at `1df714645aa683cf975bc76fcacb9842cb4780fd`. This is a **given-eight-abscissae sign-vector component**, not an unknown-public-target extractor and not a complete ECDLP solver.

## Hypothesis and derivation frozen before native Rust timings

Let x_i,y_i denote the **positive representatives of the eight distinct scalar-weighted slot points**, i=0,..,7; public R=(r,s), and eps_i in {+1,-1}. A row of the target-conditioned 8x8 RR9 matrix is

[(x_i-r)*(1,x_i,x_i^2,x_i^3), (eps_i*y_i+s)*(1,x_i,x_i^2,x_i^3)].

For every four-subset S of {0,...,7}, define V(S)=prod_{i<j in S}(x_j-x_i). Laplace expansion across the first four columns gives

D(eps) = sum_{|S|=4} (-1)^{sum_{i in S} i} V(S)V(S^c) [prod_{i in S}(x_i-r)] [prod_{j in S^c}(eps_j*y_j+s)] mod p.

For each S, distribute the last 4-factor product into 16 sign monomials. Accumulate coefficients c_T for all 256 sign masks T (only |T|<=4 may be nonzero); one 8-stage in-place Walsh-Hadamard transform evaluates D(eps) at **all 256 sign choices** without 256 separate Gaussian eliminations. No inversions or elliptic-curve additions are required *inside* this fast determinant evaluator. Root-finding over 8 unknown x-slot choices is **not** provided; x points are supplied as test inputs.

## Frozen test panel (source-independent local Python feasibility pilot already completed, not a prospective discovery)

Use the first 31 affine rational-x base points with nonzero y, canonical min(y,p-y), and weights (1,3,5,7,11,13,17,19). Assert all 248 positive weighted x-coordinates distinct. For fixtures t=0,...,15, choose slot j point index (3t+7j+j^2)%31. Its planted sign is positive iff (t+3j)%4<2. Set R to the sum of the eight signed points. Two target arms: (A) R; (B) R+G with G=(625037715,2100627251). Check R and R+G are nonidentity.

For both target arms and every sign mask 0..255:
1. Fast 70-minor/256-slot Walsh result equals independently computed 8x8 Gaussian determinant **as a residue modulo p**, not just by zero-set.
2. Determinant vanishes iff the eight signed original-curve points sum to the target; distinguish sign-bit ordering and reject missing/duplicate roots.
3. Positive arm must include planted sign; negative arm must have no solutions on this fixed small panel, or any unexpected solution is reported (not suppressed).
4. Record **all** field multiplication and inversion counts in both implementations, cold setup, and alternating-order A/B whole-sweep diagnostic wall times. Never mistake a component speedup for a full ECDLP speedup.
5. Independently reproduce all fixture x values, targets, all 8192 zero/nonzero truth decisions, residue checksums and all reported witnesses in Python arbitrary-precision group arithmetic, without importing the native Rust library.

## Publication, novelty audit and stop gate

Publish original-Rust example, independent Python checker, source-bound CI workflow, report and exact evidence, with a commit SHA, on the *existing* #4728 lane. No four-sum grader tables or unknown scalars are available to the proposed fast algebraic evaluator, but fixture generation and independent checks use group arithmetic and know the planted signs: keep this leakage boundary explicit. Sign-vector Fourier evaluation is a classical multilinear polynomial evaluation (not a new ECDLP algorithm). The 4+4 grader in #4728 must never be counted as a non-generic extractor. The negative baseline in crypto-autoresearcher SA-04 and SA-18 and prior #4692/#4717/#4714/#4720/#4722 must appear in the report.

Progression to B8 46-positive/18-negative frozen **unknown x-index** panel requires a separate public-only selector/F4 extraction, without enumerating x_i tuples or 3+3/4+4 group sums. B31 rank31 and 1024 blinded full solves are subsequent gates; compare total costs with best matched mixed IC, shared BSGS and rho; independent signed PCC is a separate validation gate. No paid compute.
