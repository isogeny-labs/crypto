# Pre-registered native B677 monic-S3 public-root gate (2026-11-23 UTC)

## Purpose
Independently test a *non-table, algebraic*, public-`z` **x-only m=2 root extractor** on the **unchanged ordinary prime-order** Prime31 curve at factor-base sizes approaching the required project size B=677. This complements [passing small-cell Run74](NATIVE_F4_RUST_RUN74_20261123.md) and does not replace the stronger RR9/Horner/F3 m>=3 index-calculus program.

## Source identity and invocation
Code: `examples/prime31_monic_s3_pair_oracle.rs`, first `--scale` source commit `80da3c1a0c2d231ddc5f6cb93bec8596c3f537f0`; workflow step attached by `fb850179c3847fd6950dbd91015c4e3d7c6da62d`. Run:

```bash
cargo test --locked --release --example prime31_monic_s3_pair_oracle
cargo run --locked --release --example prime31_monic_s3_pair_oracle -- --scale
```

CI: [#45088863033](https://github.com/isogeny-labs/crypto/actions/runs/45088863033), source `fb850179c3847fd6950dbd91015c4e3d7c6da62d`. At registration this source-specific CI was **still in progress**; never infer success from merely launching it.

## Frozen experiment
- Group: `p=2147483647, a=847514846, b=2087444724, n=2147446831, h=1`. Original `crypto_lib::cryptanalysis::semaev_prime_poly::{Fp,Curve}` and existing Prime31 generator x.
- Factor base `W_B`: distinct rational curve x-coordinates consisting of `GX, X2G`, then ascending eligible abscissae from 0; `B∈{64,128,256,677}`. The search bound is 10,000, and failure to obtain B points is a hard failure, not a success.
- Public target abscissae: fixed `GX` and `X10G` controls plus **four predetermined, distinct, quadratic-residue-eligible curve x-coordinates** from the in-code LCG seeded `0xC31BA7ED20261122`. The oracle is not supplied target logarithms or factor-base indices. These four controls are deterministic and not claimed to be statistically uniform nor a signed-point basis.
- Candidate: m2 Newton trace moments reduced modulo **monic t** S3, univariate `D(s), N(s)`, polynomial gcd, `F_p` rational-root finding, exact back-substitution `t` and `x1,x2` recovery. Degeneracies with identically vanishing D=N or root-split cap must fail loudly. Report `Ddeg,Ndeg,gcd_deg,online_ms`.
- Independent reference: only *after* the candidate returns, enumerate all unordered pairs in `W_B`, require original `Curve::s3` equality, and compare exact sorted pair lists. Separately report `reference_scan_ms`, **not included** in candidate online time. Check balanced product-tree pair coefficients agree exactly with the independent Newton recurrence.
- CI timeout: five minutes for this scaling step, plus full original Rust base/circuit/F4 stages in a 30-minute job. A timeout means **censored**, not failure of mathematical method or proof of no roots.


## Explicit post-registration amendment — positive-control coverage
The initial 24-cell protocol had only `GX` as a guaranteed positive per B; the four LCG curve-x values are legitimate *negative controls* but unlikely to be decomposable as 2-sums from small W. To prevent a trivial all-empty pass, the source at commit `116801035be0ce84624c4df07b9a4b6294095dde` now includes **two additional planted nontrivial S3-positive x targets per B** (from pairs of public factor-base abscissae at `(B/3,B/2)` and `(B/4,B-3)`). An exact quadratic discriminant formula reconstructs each z and checks `Curve::s3(u,v,z)==0`. The candidate `extract()` still receives only the abscissa z and W, not the pair indices. These are *planted correctness controls*, not truly blind random positives.

Updated 32 total cells: B=64/128/256/677 x 8 distinct targets (GX, X10G, 4 LCG curve-x, 2 algebraically planted positives). An independently written Python EC/S3 grader of the initial source found all four LCG x targets negative for these B values (i.e., prior panel would exercise mostly empty sets). The expansion was made openly **after** the first `--scale` workflow launch; it is not represented as originally preregistered. Newly triggered source-bound CI required. The original 24-cell historical run remains separately attributable.

## Success and failure interpretation
**Correctness pass:** every one of the 32 (B,z) cells has exact pair-set agreement, including true empty cells, no hidden generic pair dictionary in the candidate. A correct returned x pair still requires a signed group certificate to qualify as an ECDLP relation.

**Performance-gate pass:** demonstrate reproducible same-host cold time lower than the strongest project index-calculus-frontier *after* full signed group extraction, independent rank, 1/32/128/1024 target recovery, exceptional cases, failed probes, amortization and optimized negation-map rho/BSGS comparison. This CI does NOT implement that gate.

**No-advantage observations:** any `--scale` cell becomes slower than matched pair scanning once validation scan cost is isolated, exceeds its step timeout, or has prohibitive memory. Record all, do not cherry-pick successes. Even a successful m2 algebraic oracle can be asymptotically inadequate because signed-pair decomposition density is O(B²/n) under uniform-target sampling.

**No claim of cheaper complete ECDLP solver or asymptotic speedup.**
