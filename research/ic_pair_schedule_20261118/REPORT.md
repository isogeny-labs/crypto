# IC R3 — balanced partial pair-table order (falsified toy hypothesis)

**2026-11-18; EMPIRICAL (toy only); [pcc:unverified].** Frozen protocol SHA-256 `5199afb75054606659da5c7428f0f5e067830aa7b4507ab4758ce2323537464e` before R3 execution. Producer `run.py` and independent `audit.py` committed on this branch have Git blob IDs `7c937196c94fa2f4d3e058cf65f38a67abf841f6` and `c802a58d781f2e96a8d5def9dc7cf8beb95f4691`, exactly matching the locally tested files.

## Design and hypothesis

On the same public curve `E(F_1009): y²=x³+4x+10`, prime order `r=1069`, base `G=(0,162)`, compare seeded random-prefix selection to a deterministic cyclic-distance/chord-order prefix of exactly the same unordered factor-base pair count. No factor-base scalars or curve points influence the order. At `B=16, f=25%`, prespecified pass criterion: ≥10% lower median **cold counted EC group additions** across 24 paired seeds, and ≥22/24 independently verifiable DLP solves **in both arms**. Secondary `B=32, f=50%`: candidate ≤5% regression, 24 seeds. Exact random A/A, full-table and matched seeded 4-partition rho controls were also run. Any failed/censored target is not counted as a successful low-cost solve.

## Measured result and frozen verdict

| B / table fraction | random completed | balanced completed | median paired balanced/random G | 95% paired bootstrap | balanced median G | full-table median G | balanced / matched measured rho G median |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16 / 25% | 21/24 | 24/24 | **0.9319** | [0.6766, 1.0114] | 881.5 | 555.5 | 10.39× |
| 32 / 50% | 24/24 | 24/24 | **0.9961** | [0.9504, 1.0478] | 917.5 | 1069.5 | 12.81× |

For the three B16 random failures, the median's fractional ratio convention uses candidate cost / infinite baseline cost = 0. **Among the 21 both-solved pairs, median ratio is 0.9757**. The 10% threshold and the baseline success floor both fail; **H1 is falsified**. B32 nonregression passes but shows no meaningful gain. Full-table B16 remains faster, while partial-table B32 remains faster than full; no curve-scaling or asymptotic improvement. No IC leaderboard row promoted.

## Cold phase accounting

Values below are **mean counted group additions** per seed, including unsuccessful runs as incurred work (not complete solve costs). Matrix phase separately timed, linear algebra reported as modular `F_r` ops.

| B | arm | FB G | pair table G | relation collection G | matrix median ms | LA Fr operations mean | descent G | total G mean | rho G mean |
|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | random 25% | 190.875 | 34 | 748.542 | 0.00309 | 121.875 | 24.375 | 997.8* | 83.67 |
| 16 | balanced 25% | 190.875 | 34 | 637.5 | 0.00276 | 131.917 | 37.042 | 899.4 | 83.67 |
| 16 | full | 190.875 | 136 | 224.292 | 0.00309 | 167.583 | 8.25 | 559.4 | 83.67 |
| 32 | random 50% | 380.75 | 264 | 290.5 | 0.00593 | 378.042 | 3.333 | 938.6 | 82.38 |
| 32 | balanced 50% | 380.75 | 264 | 283.792 | 0.00529 | 380.625 | 3.708 | 932.2 | 82.38 |
| 32 | full | 380.75 | 528 | 169.417 | 0.00574 | 369.833 | 1.042 | 1079.2 | 82.38 |

All phase timers, `w=0,.1,1` uncalibrated G+Fr sensitivities, and per-seed raw witnesses reside in the preserved conversation archive (see `RAW_MANIFEST.md`). Balanced schedule was itself slower to build in Python (B32 median 0.310 vs 0.108 ms), so no wall-time speedup claim. Ideal generic rho `√(πr/2)≈40.98` G is *not* the observed rho; the actual 4-partition rho on the same target is the comparator.

## Validation and literature

48 seeds × 4 IC arms = **192**, with **186 successful target recoveries and 6 censored arms** (three distinct B16 random cases plus their A/A replays). An **independent audit**, not importing the producer, enumerated 1,069 curve points; independently reproduced all factor-base points, chosen pair sums, every accepted relation, rank by dense modular elimination, successful logs, per-stage group counts and **48 actual rho traces**. Zero invalid successful relations. One deliberately corrupted row failed audit as expected. Earlier R2 random arm was replayed exactly on all 8 overlapping B16 and 24 overlapping B32 seeds. Initial audit erroneously omitted an extra final group addition on censored walks; the audit was repaired after a recorded failed output, producer and frozen protocol unchanged, and all 192 arms rechecked. This is local software validation, **not** PCC signed CI/independent hosted replication.

Prior work: Semaev (2004, https://eprint.iacr.org/2004/031), Gaudry (JSC 2009), Corrigan-Gibbs–Kogan (EUROCRYPT 2018, https://eprint.iacr.org/2017/1113), closure `research/ic_mega_pass_20261031/{brief/BRIEF.md,report/REPORT.md}`, prime frontier `research/prime_ic_collection_frontier_20261117/README.md`, preceding R2 partial-table PR #4763. This changes **table selection engineering only**, not the generic-group exponent or ECDLP difficulty.

## Blockers and next experiment

Official workspace checkout failed GitHub DNS; `bin/ws route` was run before `bin/ws need crypto` via an explicitly marked offline subset, not a pretend upstream checkout. The official `AGENTS.md` and latest lead/closure/frontier files were read through GitHub. Conductor claim tools are absent, and Rust/native, EKS and PCC execution are unavailable in this runtime. The tested scripts are committed, and the complete unmodified raw archive is separately preserved with checksums; raw JSON is not in this PR due to connector transfer limitations.

**Next:** do not promote this null. Continue already queued **native partial random-table** experiment from R2 PR #4763 in the official trunk with Conductor scope; matched 24/28-bit Rust `prime_fast` batched full/half/A/A, cold phases including rank and descent, independent replay and strongest matched rho at calibrated `w=0,.1,1`. Reject if no complete single-target cost improvement.
