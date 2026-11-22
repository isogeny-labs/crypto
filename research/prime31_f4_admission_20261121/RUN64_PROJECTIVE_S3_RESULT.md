# Prime31 run64 — projective Semaev S3 cost ablation

Date: 2026-11-22 UTC. Repository branch: `research/prime31-f4-admission-run51-20261121` / draft PR #4901. Evidence status: **two source-identical native Rust CI attempts passed** ([Actions #45032930310](https://github.com/isogeny-labs/crypto/actions/runs/45032930310)); signed PCC remains unverified.

## Mathematical result: a division-free public S3 zero predicate

For `E: y²=x³+ax+b`, the standard three-point Semaev polynomial can be written

`S3(u,v,x)=c2*x²+c1*x+c0`, with

- `c2=(u-v)²`;
- `c1=-2[(u+v)(uv+a)+2b]`;
- `c0=(uv-a)²-4b(u+v)`.

For a nonidentity Jacobian point `(X:Y:Z)`, `x=X/Z²`. Clearing the denominator gives

`S3_hom(u,v;X,Z)=(u-v)²*X² - 2[(u+v)(uv+a)+2b]*X*Z² + [(uv-a)²-4b(u+v)]*Z⁴ = Z⁴*S3(u,v,X/Z²)`.

Since `Z != 0` on the finite chart, **zero testing is equivalent** and can avoid batch inversion if the algorithm only needs a fixed set of S3 tests. Projective charting is a standard algebraic operation; no novel index-calculus algorithm is claimed.

With coefficients prepared once, using the reused Mersenne-field multiplication, affine Horner evaluation requires 2 field multiplies *per fixed pair* after x normalization; homogeneous evaluation can share four preproducts (`X²`, `Z²`, `XZ²`, `Z⁴`) per query and then requires 3 multiplies per pair. Thus a projective win is **not automatic**: avoiding Montgomery normalization competes with an extra multiply per factor-pair, and may lose sharply at K=16. This is precisely why the K=1/4/16 crossover is measured rather than asserted.

## Independent local validation (completed)

A separately authored Python big-integer affine group-law implementation on the unchanged `p=2147483647, a=847514846, b=2087444724, G=(625037715,2100627251)` checked:
- **5,625/5,625** pair sums of the first 75 positive multiples of G satisfy the explicit S3 polynomial;
- **4,096/4,096** seeded independently rescaled public projective representations satisfy `S3_hom=Z⁴*S3_aff`.

These are algebraic correctness controls, **not Rust throughput or blind unknown-index relation recovery**. A matching independent verifier has been committed for the native CI job.

## Reused Rust and source-specific gate

- [Frozen protocol](RUN64_PROJECTIVE_S3_PROTOCOL.md)
- [Original-crate Rust 7-arm experiment](../prime31_ic_20261115/native/examples/prime31_run64_projective_s3.rs): imports `prime31_algebraic_ic::{check_curve,Arithmetic,Base,PRIME31}`, uses the existing run63 mixed-Jacobian and batch-X arithmetic design, and gets the 16 public S3 pairs from the actual depth-10 Dickson base (509 points).
- [Independent BigInt verifier](run64_independent_verify.py)
- [Bounded CI workflow](../../.github/workflows/prime31-run64-projective-s3.yml)

CI gate: Rust 1.100.0 original-crate compilation, 40 × 192 original-arithmetic group-law controls (7,680), 16 genuine factor-base pair sums and 16 doubling certificates, and 5 rotations of 250,000 public shifts per arm. Compare shift-only; projective S3 and batch-affine-X S3 at K=1,4,16. Record all seven raw times and medians, not just favorable ratios. Infinity, input-invariant and final-scalar mismatches fail closed.

**Native CI passed twice**, with matched component timings frozen below and in `RUN64_RESULTS.json`. Complete ECDLP cost remains unmeasured. Both algebraic arms here evaluate **fixed public pair coefficients** rather than extracting unknown factor-base indices. Therefore even a large component speedup would not qualify as non-generic blind extraction.

## Audited prior leads and decision

- #4901 runs 55–63: rank-aware hybrid 677-point base, 88 public homogeneous free relations, exact two-point support and synthetic 4.699949M-probe T1024 model; run63 x-only batch normalization 21.868/29.928 ns per probe on separate Actions attempts, compared only to within-run arms.
- #4391 already contains Jacobian/batch-inversion methods. #4809/#4846 cover F3 quadratic/discriminant rootfinding (in particular `disc S3 = 16 f(u)f(v)` for rational inputs). Do not duplicate or brand these as inventions.
- #4692, #4812 and #4822 own higher-arity RR9/Horner/F3 blind root extraction. #4891 reports F4 small-base costs of seconds for small inputs; #4816 finds strong large-base nonenumerative complexity penalties. Those are more decisive than low-level per-query costs.
- Peer-reviewed one-relation precedent: Amadori–Pintore–Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009; original summation-polynomial proposal: Semaev (2004), https://eprint.iacr.org/2004/031.
- Repository research-KB MCP was not available in this execution; available PR sources/literature were inspected. No claim of exhaustive novelty review.

## Next experiment requirements

First collect two source-specific native Rust runs to determine **whether projective S3 actually wins** or if the additional per-pair multiplication makes affine batch normalization superior. Then wire only a winning arithmetic chart into the actual public-only root extractor owned by #4809 or #4822, preserving the frozen 46-positive/18-negative holdout from #4692. Count failures, memory and all non-generic operations, and require signed group-law witnesses and independent target logs. Final 1/32/128/1024 ECDLP cohort must charge setup, rank, individual recovery and all negative probes against the same-runner best complete index calculus, shared BSGS and optimized rho.

No cheaper complete non-generic prime-field ECDLP solver has been established by this run.

## Native source-bound outcome — two independent CI attempts PASS

The original Rust 1.100.0 example passed on source commit `41644f56ae334d7498d49f28753f51bd315b1895`, in two source-identical Actions executions: [job 137240289733 and rerun 137240814780](https://github.com/isogeny-labs/crypto/actions/runs/45032930310). Both independently ran 5,625/5,625 Python S3 sum checks and 4,096/4,096 projective identities, plus 7,680 original Rust EC group-law comparisons, 122,848 Rust homogeneous/affine S3 identities, 16 genuine public Dickson pair sums, 16 doubling checks, and 35 benchmark endpoint checks.

The **two runs differ materially in absolute speed**, so compare within each runner only. Each arm evaluates the same 250,000 public target shifts per repetition, five repetitions, with rotated execution order. Medians are nanoseconds per public shift *plus fixed-pair algebraic zero tests*, not the cost of finding an unknown pair.

| Timed arm | First CI median ns/query | Same-source rerun median ns/query |
|---|---:|---:|
| Original fixed-G Jacobian public shift only | 14.1086 | 22.9277 |
| **Projective S3, 1 public pair** | **14.6644** | **22.5697** |
| Affine-X batch256 + S3, 1 pair | 19.8130 | 31.4991 |
| **Projective S3, 4 public pairs** | **16.2973** | **27.8859** |
| Affine-X batch256 + S3, 4 pairs | 23.8253 | 38.0794 |
| **Projective S3, 16 public pairs** | **34.6238** | **58.7928** |
| Affine-X batch256 + S3, 16 pairs | 39.8900 | 63.9528 |

**Within-run projective-to-affine ratios:** K=1 **0.740/0.717** (26.0% / 28.3% lower), K=4 **0.684/0.732** (31.6% / 26.8% lower), K=16 **0.868/0.919** (13.2% / 8.1% lower). This is an independently repeated lower-level **algebraic evaluation chart optimization**. There is no observed complete solver improvement; the fixed-pair K16 cost already consumes most of the *historical different-host* 36.19ns/query BSGS sensitivity in one run and exceeds it in the second. It would be misleading to extrapolate this positive component result to the many unknown factor-base indices of actual blind extraction.

In the K=1 rerun, projective S3 appears numerically slightly faster than the shift-only arm (22.570 vs 22.928ns), which cannot establish a negative cost for S3; instruction scheduling, optimization and noise cause this. Treat these as pipeline benchmarks, not additive operation-counter lower bounds.

Raw **all-five-repetition** times, source, input data, curve certificate and two-job IDs are frozen in [RUN64_RESULTS.json](RUN64_RESULTS.json). The proof/instrumentation is source-specific and [pcc:unverified]. A runner-isolated repeat and real F4 oracle remain the next gate.

### Revised algorithmic decision

Projective S3 testing is an actionable candidate to **avoid normalizing the public target per query** in an algebraic oracle using a small set of fixed S3 constraints. For a *truly blind* extractor, the leaf abscissae remain unknown algebraic variables, and the corresponding coefficient expressions must be retained symbolically; testing K precomputed pairs or using a pair-sum lookup is not permitted as a non-generic oracle. The next concrete integration is a homogeneous-public-target variant of the already existing four-variable RR9 or 14-variable F3 chart, on the exact registered positive/negative holdout; measure F4 rows/cols/degree, memory, failed roots and recovered log witnesses, not S3 test speed alone.
