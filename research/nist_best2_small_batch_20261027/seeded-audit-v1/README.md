# BEST2 seeded-walk and replay audit - 2026-10-27

**Partial implementation and correctness audit; DP bias unresolved.** BEST2
remains disabled by default. No new GPU performance or solve-cost result is
claimed. The historical 0.947 estimate is a combination of two different
experiments, not a measured end-to-end ratio or an end-to-end 95% interval.

Base inspected: PR #2689, source branch head
`bec16151f92b5764e94579bf10b6680557d663ff` (actual source branch, not PR merge ref).
Source and binary hashes for the changes tested here are in `checks/SHA256SUMS`
and `checks/source-sha256.txt`. The amended walk protocol is [PROTOCOL.md](PROTOCOL.md).

## Verified observations and corrections

- `best2_ab.sh` previously changed ordering, not starts or jump tables. Its
  repetitions are timing controls over one deterministic sample per W/curve.
  New seed arguments derive distinct reproducible table/start namespaces, with
  a fixed-table/varying-start control. Seed zero retains the historical seeds.
  Pseudorandom namespaces do not constitute a mathematical independence proof.
- `rho_phase_b` returns cycle-detected/seek states that the batched engine does
  not DP-test. `rho_hash` and the cycle escape's minimum-hash rule read limb 0,
  overlapping the partition/DP coordinate. Score limb 1 being different does
  not establish independence of the selected trajectory from the DP predicate.
  On the one-limb toy field the score explicitly mixes the same coordinate.
  These are mechanisms to measure, not a proved explanation of the deficit.
- The old GPU counter included four warmup iterations' DPs but its denominator
  excluded warmup. The driver now clears DP count after warmup. This correction
  decreases a measured count; it cannot explain an existing low count. Warmup
  trajectory changes remain and are logged.
- The GPU numerator is physical lane-iterations and the timed kernel already
  executes cycle work. Combining it with an iteration model requires that
  model's steps to be converted to physical iterations. Do not add cycle cost
  again to already measured device time. The earlier approximately 0.4% is
  a heuristic differential, not a validated correction to end-to-end cost.
- **Replay defect fixed:** `RhoHost::seed` advanced a local restart when its
  start was infinity, then returned only the point/coefficients. The caller's
  restart could name the wrong start. It now updates the caller's restart by
  reference, matching `rho_seed_walk`. A constructed cancellation-start
  regression checks the host/device restart identities and points agree.
- Source audit of standalone `rho_walk_leads` `e4`: `Inst::score` and DP tests
  hash the coordinate with separate keys; the kernel reads coordinate limbs.
  The harness detects only a two-cycle and jumps directly to an escape, and
  keeps walking after a DP; the kernel seeks through detected cycles of length
  2..6 and restarts after a DP. Matching r=256 alone does not equate these walks.

## Tests actually executed

Development unit tests ran locally, without an isolation qualification. Their
elapsed times are not reported as benchmarks. Raw outputs are in `checks/`:

| Requirement | Evidence and status |
|---|---|
| CPU field/curve/rho suites, BEST2=0 and 1 | Both passed (`cpu-b0.log`, `cpu-b1.log`); runtime-W vs compile-time-W now includes W=64 |
| Independent full-size point arithmetic | OpenSSL named P-256/P-384 groups independently compute P+M/P-M and verify selection/fold codes on 2048 physical steps per arm, two seed namespaces, W=16, 32 iterations (`openssl-b*.jsonl`) |
| Batched DP replay identity | Same unit runs compare every batch state/history and emitted DP's x, walk, restart and logical steps to per-phase/OpenSSL-checked trajectories |
| Scalar replay including suppressed steps | 4000 toy phases per arm check P=[a]G+[b]Q after every non-infinity phase; subtract, escape and skipped-step paths are covered (`cpu-b*.log`) |
| Determinism | BEST2=1 OpenSSL test rerun was byte-identical (`cmp`, exit 0); two seed namespaces yield different records |
| Native summary uncertainty/repetition checks | Synthetic 8-seed fixture produces rate ratio 1.1; duplicate fixture is rejected with exit 1. This is parser arithmetic testing, not DP evidence |
| Native ecbench model bookkeeping | Real model source tested through dependency-free shim: 24 synthetic cyclic-group recoveries (12 seeds x 2 arms), identical deterministic counters on replay (`model-unit.log`) |
| Root library integration | Blocked: available extracted Cargo 1.75 cannot read this repository's lockfile v4 (`cargo-test.log`, exit 101); no root build success claimed |
| CUDA compilation / independent-seed GPU A/B | Not executed: no nvcc, GPU device/CLI or authenticated GPU client in this runtime |
| EKS statistical panel / ecbench sealed session | Not submitted: no EKS/AWS connection or credentials; GitHub connector has read and repository-write tools but no workflow-dispatch operation |
| Effective cost per successful GPU collision | Not implemented or measured; bounded collision workload, GPU DP transfer/replay and full-size scalar recovery remain missing |

The OpenSSL trajectories are intentionally small unit tests, not statistically
powered DP samples. Do not compute a bias verdict from their DP totals. They use
the host field implementation; they do not test PTX compilation/device arithmetic.
`RhoHost` still supports only orders below 2^63. Full-size coefficient replay and
an independent recovery certificate are not provided by those unit tests.

## What was added

`gpu/nist/test_best2.cpp` is a native full-size seeded DP diagnostic. Each record
contains exact seed IDs/namespaces, W, iterations, DP bits, all physical-step
categories, eligible advances, emitted DPs, plus/minus/selected DP marginals and
score-bucket contingency counts. Categories satisfy
`attempts = advanced + cycle_detect + seek + inf`; escape doublings are a subset
of advances, not another additive category. It matches CPU batched emission in
unit mode; sample mode avoids the OpenSSL-per-step oracle overhead.

`gpu/nist/best2_stats.cpp` recomputes physical and eligible DP rates separately
and uses a deterministic paired seed-cluster bootstrap. It rejects duplicated
samples rather than treating timing repetitions as independence. The intervals
are exploratory/unadjusted; a small event count is not a proof of no bias.
A large-seed statistical run and independent recomputation remain pending.

The native ecbench registry gains `rho.plus_model_v1` and `rho.best2_model_v1`
with explicit jumps/walks/DP-bit/cap parameters. Both charge setup, every start,
physical walk step, candidate addition and escape through `CountedGroup`;
scalar recovery is returned for the runner's independent check. Non-group work
is retained in `_uncharged` counters. Existing method IDs are unchanged.
[The spec](../ecbench-model.json) includes the strong negation reference and A/A.
This is a generic model with hashed group keys, not the CUDA coordinate-limb
walk, and charges two full additions rather than assuming a shared inversion.
Its sealed-session/replay evidence is missing; it cannot validate the NIST ratio.

## Sources and scope

Read PR #2689 metadata and review history; the study README and source branch;
`gpu/nist/{nist_rho.cuh,nist_rho_host.hpp,bench.cu,best2_ab.sh}`;
`research/rho_walk_leads_20261014/harness/src/main.rs` (score and e4 DP loop);
`src/cryptanalysis/ecbench/{spec,methods,gpu}.rs` and `ic_boundary.rs`;
repository ecbench/report-evidence/isolab/research-visuals instructions.
These sources establish the implementation differences and accounting above.

Literature search: Bernstein, Lange and Schwabe, *On the correct use of the
negation map in the Pollard rho method*, https://eprint.iacr.org/2011/003 .
The primary PDF retrieval was blocked (403); the author's page and an IACR
mirror also failed. No new theorem, novelty claim or paper-derived quantitative
claim is made here. The empirical and source-level checks do not rely on an
assumption that coordinate limbs are statistically independent.

No benchmark/leaderboard row is added: no new speed was measured and no replay
certificate/independent validator supports promotion. Existing study-local
iteration/GPU graphs are retained as historical diagnostic plots; their numeric
inputs have not changed. The new accounting diagram and this report supersede
their performance-verdict wording. Classification: **accounting and correctness**,
not an established engineering speed gain.

## Original scope still open

[Problem record](../../../problems/nist-best2-dp-bias-and-gpu-validation.md):
the statistically powered bias-vs-sampling test, authenticated GPU runs, native
ecbench integration build/session, independent second-host replay, GPU DP
coefficient recovery and complete successful-collision accounting remain open.
A PR or passing unit test does not discharge those requirements.

[Audit PDF](REPORT.pdf) | [Accounting diagram](accounting.svg).
