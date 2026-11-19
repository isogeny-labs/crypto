# Prime31 Semaev F3 batch-root extraction: non-novelty audit and controlled Rust comparison
**Date:** 19 November 2026. **Status:** Original two-arm source-specific CI passed; amended three-arm production comparison pending publication of raw evidence. **Claim status:** a complete signed two-factor *component*, **not** a complete ECDLP solver.

## Target and method
Unchanged ordinary prime-order curve from the original Prime31 Rust crate:
`p=2147483647`, `a=847514846`, `b=2087444724`, `#E(Fp)=2147446831`, cofactor 1, G=(625037715,2100627251). These are the actual fixed properties of the source Rust module; no deliberately weak curve substitution, no oracle's hidden scalars in candidate inputs.

For `R=(r,s)` and a factor-base point `P=(u,y)`, the standard Semaev 3-summation polynomial as a quadratic in v has coefficients
```
alpha = (r-u)^2
beta  = -2*((r+u)*(r*u+a)+2*b)
gamma = (r*u-a)^2 - 4*b*(r+u)
F3(u,v,r)=alpha*v^2+beta*v+gamma
beta^2-4*alpha*gamma=16*(r^3+a*r+b)*(u^3+a*u+b)=(4*s*y)^2.
```
Consequently two roots are `v=(-beta±4*s*y)/(2*alpha)` if `alpha!=0`; if `alpha=0` solve the linear `beta*v+gamma=0` and exclude infinity. These are exactly the finite x-coordinates of `R±P`. One Montgomery batch inversion processes every denominator for 31 factors; candidate x only serves as an index into the publicly available *single-factor* list. Every returned signed witness is checked with original Rust elliptic-curve addition; the generic baseline computes public R±P. **No precomputed 2- or 4-sum group table is available to the F3 candidate.**

The exact discriminant identity is a mathematical fact, **not a new F3 algorithm**: direct batched slope x(R±P) computation, S3 rootfinding and special-case denominators already existed in repository code. The amendment adds an explicit **native direct-batched** comparator based on `examples/semaev_rootfind_scaling_bench.rs` and `examples/semaev_rootfind_nist_bench.rs`. This avoids comparing to the slower unbatched affine reference as though it were the index-calculus frontier.

## Frozen original-curve test
- 31 first rational-x points, identical to W8 studies, subgroup generator and arithmetic unchanged.
- SplitMix64 `0x4633524f4f543331`, 64 constructed but blinded two-factor public targets, 256 natural random nonzero multiples of G, 31 exceptional public targets equal to base points. Inputs provided to the extractor are public target coordinates and public base coordinates only.
- Full signed ordered fibre enumeration with public sign replay, including 31 x(R)=x(P) singular cases. Eight alternating wall-time repetitions of the **same** F3, established direct-batched, and generic affine reference programs.
- Field mul counts, one inversion per public target in each batched arm, group additions for witness replay, setup, all negative queries and sorted wall times are charged. Independent Python finite-field and affine arithmetic replays every target and witness. Failing/censored runs cannot be discarded.

## Earlier two-arm CI evidence, already completed
[Original source-bound Actions #44555480777](https://github.com/isogeny-labs/crypto/actions/runs/44555480777) PASSED against pinned original commit `1df714645aa683cf975bc76fcacb9842cb4780fd` on Rust 1.100.0: 40/40 library tests, 2/2 focused identity and full-fibre tests, 351/351 complete independent Python affine replays, 64/64 constructed positives, **0/256 random two-factor hits** (descriptive panel, not a population bound), 31 exact exceptional linear-branch cases. This validates a component's correctness. The amended three-arm production comparison supersedes its timing interpretation.


## New exact two-factor feasibility obstruction (not a benchmark)

The first-rational-x31 factor base contains 31 nonzero points and all their negatives, i.e. **62 distinct signed points** because the source group has odd prime order and distinct x values. There are at most `binom(62+1,2) = 1953` unordered two-point multisets (including repetition). The 31 inverse pairs `P_i + (-P_i) = O` all map to one sum, removing at least 30 duplicates. Therefore the complete signed two-factor public-target support satisfies the **rigorous group-independent inequality**

`|S_2| <= 1953 - 30 = 1923`.

On the exact original-curve prime-order group `n=2147446831`, every uniformly randomized public residual (by Q+[b]G with b uniform) hits this two-factor predicate with probability **at most `1923/n = 8.954820078616419e-7`**. Independently randomized residual trials therefore require **at least `n/1923 = 1,116,717.0208` expected queries per success** (unless the actual support is smaller, making it worse), and at least **34,618,227.64 expected queries for 31 successes**, without any rank-independence guarantee. This is an exact upper bound on support; the experiment's zero natural matches out of 256 are consistent but not a sharp population estimate. This does **not** bound nonuniform/adaptive residuals, larger factor bases, eight-factor W8/RR9, algebraic methods that avoid independent random Q shifts, or all ECDLP algorithms.

**Scientific decision:** Completing or micro-optimizing the already-existing degree-2 F3 primitive cannot by itself supply a useful high-yield relation oracle on this fixed B31 support. Index-calculus priority should remain **higher-arity nonenumerative public-only extraction** owned by #4692 and larger factor-base/root-constraint designs, not claiming a faster complete solver from an arithmetic microbenchmark.

## Existing leads and exclusions
- [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns a **different** public-only *eight-factor* RR9/F4/remainder-tower index extractor (46 positive/18 negative frozen checks); do not duplicate it.
- [#4808](https://github.com/isogeny-labs/crypto/pull/4808), [#4807](https://github.com/isogeny-labs/crypto/pull/4807) measured weighted eight-factor **generic group-table grading**, not a solver. K=4 blockscale had 452/512 natural hits versus 440/512 at k=5; paired evidence inconclusive, and **does not** transfer to this two-factor rare-support system.
- [#4532](https://github.com/isogeny-labs/crypto/pull/4532), [#4793](https://github.com/isogeny-labs/crypto/pull/4793) give historical whole-solver comparator: optimized shared 2^20-baby BSGS **170.113ms for 1024 logs**, historical, different runner, and not an end-to-end comparison to this node. Complete costs must be paired on the same target cohort.
- Existing specialized prime-field rootfinding: `examples/semaev_rootfind_scaling_bench.rs`, `examples/semaev_rootfind_nist_bench.rs`, `src/cryptanalysis/prime_fast.rs`; explicitly reject an originality claim for this implementation.
- Cross-repo `crypto-autoresearcher` and `cryptanalysis` contain preexisting batch-inversion/Semaev and related relation-finding leads. Knowledge-base MCP not available in this runtime; bounded GitHub search and public literature are not an exhaustive novelty proof.

## Relevant published prior art
- Petit, Kosters & Messeng (2016), *Algebraic Approaches for the Elliptic Curve Discrete Logarithm Problem over Prime Fields*, https://doi.org/10.1007/978-3-662-49387-8_1. Prime-field factor bases and polynomial systems studied; the complexity of the Gröbner stage is a central unresolved constraint.
- Amadori, Pintore & Sala (2018), *On the discrete logarithm problem for prime-field elliptic curves*, https://doi.org/10.1016/j.ffa.2018.01.009. Published prime-field index-calculus variant; need benchmark the correct complete relation+LA+recovery frontier, not only Pollard rho.

## Next falsifiable experiments
1. **Production baseline ablation (this PR):** Finish source-specific three-arm 351-Q comparison and complete independent replay; report median wall ratio F3/direct_batch and exact `Fp` arithmetic operations. Reject F3 as an optimization if slower than already-shipped batch subtraction.
2. **Unknown-index F4 gate (#4692 only):** If an F3/low-degree node is useful, substitute it *inside* eight-factor remainder F4 with no enumeration of generic 4+4 sums, on frozen 46-positive/18-negative targets; report worst negative wall, degree, memory, forbidden table accesses and exact signs. Current F3 two-factor success does not demonstrate this.
3. **Complete whole-solver gate:** Integrate a qualifying public-only eight-factor extraction into the existing rank31/1024-log original Rust solver. Account setup, failed query tails, relation rank, modular linear algebra and individual logs, then compare the same 1, 64 and 1024 public targets on the same runner to existing IC frontier, optimized shared BSGS and optimized rho. Verify every [k]G=Q, reject preprocessed grader or known-scalar shortcuts.
4. **Scaling gate:** Test at least six growing ordinary prime-order curves without selectively weak coefficients, seed and source hashes frozen; a Prime31 component win is not a subexponential-complexity theorem.

**No full-cost non-generic prime-field ECDLP advantage is established in this record.** Raw three-arm results and signed independent PCC still needed for leaderboard claims.
