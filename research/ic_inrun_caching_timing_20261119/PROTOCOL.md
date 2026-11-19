# Protocol: timing the in-run caching of PR #4756 (old vs new binary)

Version 1, drafted 2026-11-19. It is frozen when this directory is committed
on both source revisions named in §4 and before the first dispatch. Any later
change is a dated amendment below §12, made before the data it affects.
Every number in this file is a prediction or a design parameter, not a
result: `[pcc:unverified]`.

## 1. Question and class

Do the default-on changes of PR #4756 ("Remove repeated target-independent
work inside one cold IC run") lower the cost of one cold IC run on matched
instances? The cost is counted work first and L2 wall time second.

- **Class:** engineering (AGENTS.md §3). The algorithms, relations, factor
  bases and recovered logarithms are meant to be identical. No algorithmic
  or asymptotic claim is made. No claim to beat rho is made, and no claim
  tier above T0 is made (`docs/ic/SUCCESS_CRITERIA.md` §3).
- **Not measured, and not to be reported as measured:** the opt-in paths
  `skeleton=1`, `trace=1`, `log_store=…` and `schedule=…`. The accounting
  correction of `is_reusable_phase` (round 2 #2) changes no total; it is
  not timed (§3, class *accounting*). Four default changes cannot be reached
  from `ecbench`; they are listed in `BLOCKERS.md` and are out of scope.

## 2. Components, reachability and hypotheses

| # | change (default on) | reached from ecbench by | in-binary toggle | hypothesis |
|:--|:--|:--|:--|:--|
| C5 | prime `subtract`/`mitm` batch inversion (`ic_boundary::add_each_batched`) | `ic.pipeline`, `oracle=subtract` / `mitm:negation_folded=1`, prime curve | `batch=0` (new binary); ignored on the old binary, so both arms are serial there | H5: same relations, adds, lookups and logs; fewer field inversions; lower wall in `oracle_setup` (table) and `relations` |
| C2b | one CADO filter shared by the two seeded sparse solves (`SparseCado::try_solve` → `solve_seeds`) | `ic.pipeline`, `linalg=filter+wiedemann` | none | H2b: same logs and relation counts; `linear_algebra` row_ops fall by the second solve's filter work; small wall drop |
| C4 | `descent-algebraic` S_{m+1} template and τ-slot bases built once, charged to `oracle_setup` | `ic.pipeline`, `oracle=descent-algebraic:m=2[,slots=all]`, Koblitz curve | none | H4: same relations and logs; per-target descent work (bitop/v1) falls, `oracle_setup` rises, total falls; lower wall |
| C1 | ecdl-tensor-ic MITM left-half table | not reached (`ic.tensor_contraction_v1` leaves `Solver::mitm = false`) | — | out of scope (BLOCKERS B1) |
| C2a | incremental dense retries in `ec_index_calculus` | not reached (`ic`/`icx` binaries only) | — | out of scope (B2) |
| C3 | sweep x-index hoist (`IC_HOIST_SWEEP=0`) | not reached; and the workflow cannot set environment variables | — | out of scope (B3) |
| R2-3 | `petit_quisquater` `PqLookup` hoist | not reached | — | out of scope (B4) |

The `batch=0` and `IC_HOIST_SWEEP=0` toggles were named as A/A toggles. Only
`batch=0` is usable here (§5).

## 3. Instances (standard panel, AGENTS.md §11)

- **`P24a`** = `icv1-fp24-tm5099-a6f137c6` (EC1 `EC1P24Cfph095cb65c0da0`),
  given as `prime_explicit` (p = 16777199, a = 16777196, b = 31,
  r = #E = 16782299, G = (2, 5412647); checked on the curve). It is the
  smallest prime rung, so `subtract`, whose cost is about r additions, fits
  the budget.
- **`K0-13`** = `icv1-f2m13-t181-515ee569` (EC1 `EC1N13Ce0h1dfa32cdf623`,
  r = 2003, cofactor 4), the panel's smoke rung. It is used because
  `descent-algebraic:m=2` with `f4-f2` on `binary-subspace:dimension=6`
  verifies there in the committed yield-sweep session
  (`research/ecbench_yield_sweep_20261004`). A larger rung would make one F4
  solve per trial too costly for 5 rounds in one 4,800-second job.
- **Targets:** 4 public targets per curve (`target_kind: public`,
  `target_seed` 20261119). Each is its own one-target cold workload ("IC
  measurement rules"); there is no shared setup across targets. The
  measurement seed is 20261120 and the order is `alternate`.

## 4. Comparison design

**Source revisions.** The workflow reads `spec_path` from the tested commit
(`isolab-experiment.yml`, "Read the experiment spec from the tested commit"),
so both revisions must contain this directory, byte for byte.

- **OLD** = `ad2a014c0022cf000845014499a9649b6c2cef2d` (PR #4756's base, `main`)
  plus one commit that adds only `research/ic_inrun_caching_timing_20261119/`.
  `GET compare/ad2a014…OLD` must list no other path. Its `src/`, `crates/`
  and `Cargo.lock` are then ad2a014's.
- **NEW** = the frozen PR #4756 head that contains this directory. Its code
  must equal `a0b215b0d77125539e9cc902a1a3bc7424874c73`, or a later head
  named in an amendment before dispatch. Any later commit that touches
  `src/`, `crates/` or `Cargo.lock` voids the run (BLOCKERS B6).
- The spec bytes are identical on both revisions. The recorded SHA-256 is in
  `DISPATCH.md`. The workflow builds each revision with the same pinned
  toolchain (Rust 1.99.1, static musl, `-C target-cpu=x86-64`), on the same
  instance type (c8a.2xlarge, no SMT), at L2.

**Jobs.** 2 specs × 2 revisions × 2 replicates = **8 jobs**, dispatched
one at a time so that no two run on one node. The order is ABBA per spec,
with the specs interleaved:
O-P24-1, N-P24-1, N-K13-1, O-K13-1, N-P24-2, O-P24-2, O-K13-2, N-K13-2.
The second replicate gives a between-job A/A for each binary
(O-…-1 vs O-…-2, N-…-1 vs N-…-2). That is the host-to-host noise that a
cross-binary wall ratio is read against.

**The rule that allows comparing two sessions.** `ecbench compare --b-dir`
pairs arm B from another session for **operations only**
(`src/bin/ecbench.rs`, `Compare.b_dir`). AGENTS.md §8 requires matched runs
on identical curves, factor bases, targets and seeds. §10 requires identical
outputs and counted units between baseline and candidate. So:

1. **Counted units across revisions are admissible.** Pair records by
   `(workload_id, method_id, round)` with the same algorithm seed. Every
   arm's `method_id` is a hash of the same spec parameters on both binaries.
   The new binary declares no default for `batch` in the method registry,
   and `optional_param` covers only `log_store`.
2. **Wall time is decided only inside one session.** It is read against that
   session's A/A arm (`ecbench compare --dir S --a … --b …`, L2-admitted).
   Here that is only the `batch` pair on the new binary.
3. **A cross-session wall ratio is descriptive.** It decides nothing. It is
   reported as the paired geometric mean of `solve_wall_ns` new/old over
   matched `(workload, round)`, then divided by the same ratio for the
   untouched rho arm in the same two sessions (rho code is not changed by
   the PR). It is printed beside the between-job A/A spread. It is never
   pooled with the in-session decision.

## 5. Arms, controls and references

**P24a spec** (`specs/p24a-batch-sparse.json`; 10 arms × 4 targets × 6 = 240 children):

| arm | role | what it is for |
|:--|:--|:--|
| `rho-neg`, `rho-neg-aa` | reference, control | matched counted rho reference (`rho.negation`) and its A/A; untouched-code control for cross-session drift |
| `sub`, `sub-b0`, `sub-aa` | candidate, baseline, control | C5 on `subtract`, `prime-abscissa:size=256` |
| `mitm`, `mitm-b0`, `mitm-aa` | candidate, baseline, control | C5 on the folded pair table, `size=1024` (the table build dominates) |
| `cado`, `cado-aa` | candidate, control | C2b: `filter+wiedemann` over the serial oracle (`batch=0`), so that only the linear algebra differs between revisions |

**K0-13 spec** (`specs/k0-13-descent-template.json`; 5 arms × 4 × 6 = 120 children):

| arm | role | what it is for |
|:--|:--|:--|
| `rho-strong`, `rho-strong-aa` | reference, control | `rho.signed_frobenius_strong`, the Koblitz reference (AGENTS.md §12) and its A/A |
| `da`, `da-aa` | candidate, control | C4: plain S_3 template |
| `da-slots` | candidate | C4: the τ-slot bases (`slots=all`) |

**Controls.**

- **A/A** (noise floor): the `-aa` copies, in every session.
- **Negative control** (the instrument must not see an effect that is not
  there): on the **old** binary, `sub` vs `sub-b0` and `mitm` vs `mitm-b0`
  run identical serial code. Their corrected ratio must contain 1. If it
  does not, the C5 decision is void (undecidable: instrument).
- **Positive control** (the instrument can see a real effect): the counted
  identity checks are exact. For wall time, H5's in-binary comparison is the
  only decided reading. No separate planted-slowdown arm is used (BLOCKERS
  B7 records this gap).
- **Identity control:** every counted quantity that a change is not meant to
  move must be identical between old and new: recovered logarithm, relations,
  trials, adds, doubles, lookups and `table_insert_attempts`. That holds for
  every arm. The rho arms must also be identical in every counter.

**Matched rho.** Each session carries its rho reference and A/A. The §5a
band is read at w = 0, 0.1 and 1 GAE per unpriced native operation, with the
default table-work pricing (2026-11-08). The IC-to-rho ratio is reported as
the distance remaining, not as a gate. `w_cal` cannot be measured inside
these jobs. The calibrated reading is therefore omitted and labelled so, and
the band alone governs (BLOCKERS B8). `rho.prime_fast`, the §12 prime wall
reference, is not an ecbench method in either binary. So `rho.negation`
serves as both counted and wall reference, and this is labelled.

## 6. Metrics

1. **Primary (counted, one target, cold):** per record `S = total GAE / √r`
   and `ecbench work --counted` bitop/v1 totals by phase. The deterministic
   ratio new/old is taken per matched record, then summarised by the median
   and range over the 4 targets × 5 rounds. Native counters named per change:
   field inversions and multiplications (C5), `linear_algebra` row_ops
   (C2b), and descent work in `oracle_setup` vs `relations` (C4).
2. **Secondary (wall, L2):** `solve_wall_ns` per record, and the five
   exclusive online phases. The cold interval is the headline. The online
   interval is a separate `online-only` row.
3. **Beside them:** peak RSS per arm, contended-run count, L2 admission,
   replay counts.

## 7. Predictions and acceptance criteria (falsification targets, AGENTS.md §4)

**Correctness gates (all must hold, or the job is invalid; it is recorded and
not retried silently):**

- G1. Every record has status `verified` with `[d]G = Q` on a public target.
  The in-pod audit, replay, L2 gate and A/A admission all pass. The S3
  payload hashes verify.
- G2. The identity control of §5 holds for every arm across revisions, and
  for every `X` vs `X-b0` pair inside the new session. Any other difference
  stops the analysis: the change is then not an equivalence (§10) and is
  investigated, not reported as a speedup.

**Decisions:**

| id | statistic | improvement | regression | prediction |
|:--|:--|:--|:--|:--|
| D5s | new session, `ecbench compare --a sub-b0 --b sub`, corrected by `sub-aa`, 95% CI | upper < 1 | lower > 1 | 0.55–0.90 |
| D5m | the same for `mitm-b0` → `mitm` | upper < 1 | lower > 1 | 0.45–0.85 |
| D5n | old session, the same two pairs | must contain 1 | — | contains 1 |
| D2b | counted `linear_algebra` row_ops `cado` new/old, every record with a paired solve | < 1 on every such record, and total bitop/v1 new ≤ old | any > 1 | < 1; size not predicted |
| D4 | counted bitop/v1 total `da`, `da-slots` new/old (and its per-phase split) | total < 1 on ≥ 95% of records and the pooled total < 1 | pooled > 1 | < 1; `oracle_setup` > 1 |
| S | cold `S` at w = 0, 0.1, 1, every IC arm, new/old | lower at every weight | higher at any weight | C5: flat at w = 0 (adds identical); C2b: flat at w = 0 |

A decided D5 result is a **stage (wall) improvement** of the oracle. D2b and
D4 decided in counted units are **stage (counted) improvements** of the
linear algebra and the descent. Any of them becomes a **pipeline
improvement** only if row S (the cold end-to-end total) falls at every band
weight on every target with G1 and G2 holding. A counted move with S flat at
w = 0 is reported as a stage result, never as a pipeline one. Relabelling
(the headline falls and the total rises) and accounting changes are recorded
as such (§3). A null, a regression, a timeout or an L2 failure is retained
and reported.

**Abandonment.** If G2 fails for a component, its timing is not reported as
an improvement until the cause is fixed and a new protocol version is
frozen. If both replicates of a job fail L2 admission, the wall readings for
that spec are undecidable (infrastructure). No target, round or job is
dropped after results are seen.

## 8. Cost chain (AGENTS.md §3a), to be filled from data

```text
C(old) = Σ phases(old binary, arm X)                       def., records O-*
       → C5 : replace serial add loop by add_each_batched   measured, D5 / field-op counters
       → C2b: replace two filters by one per paired solve   measured, row_ops
       → C4 : replace per-target S_3 expansion by template  measured, bitop/v1 by phase
       = C(new) = Σ phases(new binary, arm X)               def., records N-*
```

Each replace line concerns a different arm, so no line both restructures and
replaces.

## 9. Isolation, resources and stopping

- c8a.2xlarge Spot, 4 CPUs, 4 GiB, L2 (`cpu-l2-spot`), zero measurement
  retries, Kubernetes deadline 4,800 s. Child timeouts are 15 s (P24a) and
  30 s (K0-13), so the worst-case sum is 3,600 s per job.
- Ordinary `isolab.eks.request/v1` jobs only. They are not resumable. An
  interrupted job is a censored record, and its rerun is a new job with a
  new record.
- Stop launching when the ceiling in `DISPATCH.md` is reached or its
  remainder is unknown.

## 10. Analysis (frozen commands; no new code)

Run per downloaded session `S` (the `ecbench` built from NEW reads both
revisions' records):

```sh
ecbench verify --dir S --replay-all --exit-code             # local re-audit
ecbench work S --counted --summary                          # bitop/v1 by phase
ecbench band --dir S --weights 0,0.1,1 --json S.band.json --ranks
ecbench compare --dir N-P24-r --a sub-b0 --b sub --save     # D5s (and the -aa A/A)
ecbench compare --dir N-P24-r --a mitm-b0 --b mitm --save   # D5m
ecbench compare --dir O-P24-r --a sub-b0 --b sub --json     # D5n (same for mitm)
ecbench compare --dir O-…-r --a X --b-dir N-…-r --b X --json            # counted, w = 0
ecbench compare --dir O-…-r --a X --b-dir N-…-r --b X --weight 0.1 --json
ecbench compare --dir O-…-r --a X --b-dir N-…-r --b X --weight 1 --json
```

The descriptive cross-session wall ratio of §4.3 is computed by the
independent validator (§11) from the raw records. It is not part of any
decision.

## 11. Evidence and publication (after the run, same PR)

- Raw sessions are kept as one archive per job in the store (§13a), with
  `bench.record/v1` records (one per job), the progress report, and a
  `research-visuals` report, graph and PDF. A copy goes to
  `benchmarks/20261119-ic-inrun-caching/`.
- A `review-run`/independent validator recomputes D5, D2b, D4 and S from
  the raw records with no shared code, and writes `VALIDATION.json`. The
  validation gate applies before any leaderboard or `benchmarks/README.md`
  row.
- A pcc `measurement` claim (and a `speedup` claim only for a decided item)
  is filed. Until CI signs it, every number is `[pcc:unverified]`.
- The frozen regression suite of AGENTS.md §8 is not a launch gate here. A
  claim that C4 improved the IC pipeline still needs it (BLOCKERS B9).

## 12. Amendments

None.
