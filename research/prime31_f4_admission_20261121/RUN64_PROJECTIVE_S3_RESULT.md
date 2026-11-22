# Prime31 run64 — projective Semaev S3 cost ablation

Date: 2026-11-22 UTC. Repository branch: `research/prime31-f4-admission-run51-20261121` / draft PR #4901. Evidence status: **source committed, native Rust CI pending**, no signed PCC.

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

**Native CI is not called passed until its result and logs are inspected.** Timings and complete ECDLP benefit are unknown in this file pending the native gate. Both algebraic arms here evaluate **fixed public pair coefficients** rather than extracting unknown factor-base indices. Therefore even a large component speedup would not qualify as non-generic blind extraction.

## Audited prior leads and decision

- #4901 runs 55–63: rank-aware hybrid 677-point base, 88 public homogeneous free relations, exact two-point support and synthetic 4.699949M-probe T1024 model; run63 x-only batch normalization 21.868/29.928 ns per probe on separate Actions attempts, compared only to within-run arms.
- #4391 already contains Jacobian/batch-inversion methods. #4809/#4846 cover F3 quadratic/discriminant rootfinding (in particular `disc S3 = 16 f(u)f(v)` for rational inputs). Do not duplicate or brand these as inventions.
- #4692, #4812 and #4822 own higher-arity RR9/Horner/F3 blind root extraction. #4891 reports F4 small-base costs of seconds for small inputs; #4816 finds strong large-base nonenumerative complexity penalties. Those are more decisive than low-level per-query costs.
- Peer-reviewed one-relation precedent: Amadori–Pintore–Sala (2018), https://doi.org/10.1016/j.ffa.2018.01.009; original summation-polynomial proposal: Semaev (2004), https://eprint.iacr.org/2004/031.
- Repository research-KB MCP was not available in this execution; available PR sources/literature were inspected. No claim of exhaustive novelty review.

## Next experiment requirements

First collect two source-specific native Rust runs to determine **whether projective S3 actually wins** or if the additional per-pair multiplication makes affine batch normalization superior. Then wire only a winning arithmetic chart into the actual public-only root extractor owned by #4809 or #4822, preserving the frozen 46-positive/18-negative holdout from #4692. Count failures, memory and all non-generic operations, and require signed group-law witnesses and independent target logs. Final 1/32/128/1024 ECDLP cohort must charge setup, rank, individual recovery and all negative probes against the same-runner best complete index calculus, shared BSGS and optimized rho.

No cheaper complete non-generic prime-field ECDLP solver has been established by this run.
