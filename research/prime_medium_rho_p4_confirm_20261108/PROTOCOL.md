# P4 confirmatory study: the pooled rho.plain / rho.negation ratio on the medium prime tier

**Status: draft, ready to freeze.** Written 2026-11-08, before any data of
this study. The main session freezes `protocol.json` after the commit that
lands it (`experiment_protocol.py freeze --commit auto
--filed-before-affected-data`). After the freeze, results go into `README.md`,
`analysis/result.json` and `reports/`, never into this file. Any later change
is a dated amendment at the end.

Every number quoted from the repositories is `[pcc:unverified]`. Evidence
classes used here: *cited* (a committed file), *derived* (arithmetic on cited
numbers), *heuristic* (a model), *recalled* (literature from memory; it was not
checked, because research-kb is not installed here), *development check*
(local, not a result).

## 1. Question, and why this is a new study

The 2026-11-07 study (`research/prime_medium_tier_rho_20261107`) preregistered
a positive control, P4. It required the paired `rho-plain / rho-neg` S ratio
to be "≈ 1.41, with the 95 % interval excluding 1.2 from below". P4 failed in
all six jobs:

| job | workloads | ratio | 95 % interval |
|:--|--:|--:|:--|
| J1 | 48 | 1.285 | [1.177, 1.404] |
| J2 | 16 | 1.286 | [1.094, 1.512] |
| J3–J6 | 4 each | 1.22–1.66 | wide (3 dof) |

That protocol named no pooling rule, so nothing was pooled, and its verdict
is `inconclusive`. The owner approved a new confirmatory study and ruled out
pooling the old data post hoc (`research/open_leads_execution_20261108/DECISIONS.md`).
The old data enter here only in two places: as the variance for the power
calculation (§7), and as data already seen when the predictions (§12) were
written.

**Question.** On the medium prime tier, what is the pooled paired
`rho.plain / rho.negation` operation-count ratio θ at w = 0? Does its 95 %
interval exclude 1.2 from below? Is it consistent with √2 ≈ 1.414, or with the
old point value of about 1.285?

This study does not re-decide the 2026-11-07 P4. That verdict stays
`inconclusive` under its own protocol. This study reports its own verdict.

## 2. What is reused, and what is new

| item | this study | why |
|:--|:--|:--|
| instrument | **same**: ecbench sha256 `a18af09f9fb875dd09d0399259b5481b6952d9a8f1f52f5fccc7d9ab1a0cd3b9`, image `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:292bdc3d91ef540bc7719f20902c5acc614dab458676225623275bb3a838cae5`, source `dbd587a01aefba0634acc459ea30cfb9bbf49a09` | a confirmation must use the same instrument; no documented reason to rebuild exists (the binary has every arm: `ecbench plan` accepted all ten specs) |
| curves | **same ten** (`curves.json`, copied from the old `curves.json`, whose sha256 is recorded); no superset | the two studies then differ only in seeds and sample size |
| seeds | **new**: prefix `prime_medium_rho_p4_confirm_20261108/v1/` (`seeds.json`) | `build_study.py` asserts that no new seed equals an old one. The resolved plans were also checked: 456 new workload ids against the old 80, overlap 0; 3 192 new algorithm seeds against the old 560, overlap 0 (development check, `ecbench plan --doc`) |
| arms | `rho-neg`, `rho-neg-aa`, `rho-plain`, `rho-neg-v2` in the main jobs; `bsgs-neg`, `bsgs-neg-aa`, `bsgs-tb` in the positive-control job | `bsgs-neg` leaves the main jobs: it is not needed for θ and costs memory and time at 49 bits |
| estimator | **named before data** (§8) | the old protocol named none |

## 3. Arms

| arm | role | method | purpose |
|:--|:--|:--|:--|
| `rho-neg` | reference | `rho.negation` | the denominator of θ |
| `rho-plain` | baseline | `rho.plain` | the numerator of θ |
| `rho-neg-aa` | control (A/A, same seeds) | `rho.negation` | determinism: every pair's counts must be identical (gate G2). It also supplies the wall noise floor the EKS gate needs |
| `rho-neg-v2` | candidate (secondary only) | `rho.negation_v2` | Brent detection of cycles longer than the window, plus lost-walk counters, for the fruitless-cycle analysis (§9a) |
| `rho-split-half` | negative control (analysis-defined, no extra run) | — | the rows of even measured rounds read against those of odd ones, for `rho-neg` and for `rho-plain`. The method is the same and the algorithm seeds are independent, so the expected ratio is 1 (gate G4) |
| `bsgs-tb` / `bsgs-neg` | positive control (job PC) | `bsgs.textbook` / `bsgs.negation` | a known ratio, 1.5 (`methods::expected_s`: 1.5 and 1.0, each up to O(log r)/√r of set-up), through the same estimator (gate G3) |
| `bsgs-neg-aa` | control (A/A of PC) | `bsgs.negation` | the EKS gate needs a control arm; also a determinism check |

All parameters are at their registered defaults (`rho.*` `cap_multiple = 64`).
One binary serves every arm and job.

**Why this positive control.** The question is whether the harness and this
estimator can resolve a known ratio of about the right size against the 1.2
threshold. bsgs.textbook / bsgs.negation is the only pair in the registry with
an exactly derived ratio (1.5). It runs on the same curves, the same harness,
the same paired-workload layout and the same estimator. BSGS is deterministic
for a given target. A development check (local, 41-bit curve, two targets) saw
`S_tb − S_neg = 0.500` exactly for both targets, so its interval reflects
target variance only; the split-half A/A (G4) checks the estimator at rho's
own variance. The PC job uses the six 33–41-bit curves. At 45–49 bits BSGS
would cost tables of 2–5 × 10^7 points and add no new code path.

## 4. Workloads, rounds, order, seeds, jobs

- Every job: planted targets (`uniform_scalar_sha256_v1`), one target per
  workload. Measurement: `rounds: 6`, `warmup: 1`, `order: alternate`,
  `isolation_required: L2`, `timeout_seconds: 300` per execution. Six rounds
  is the old setting: `alternate` needs an even count, and the EKS gate needs
  at least five.
- **36 targets per curve** in the main jobs, so 10 × 36 × 6 = **2 160 pairs**
  (§7). A curve that is split across jobs uses a different target block in
  each job, so the workloads differ.
- Seeds (`seeds.json`): `seed(tag) = int(SHA-256(prefix + tag)[0:12 hex], 16)`
  with `prefix = prime_medium_rho_p4_confirm_20261108/v1/`. The tags are
  `target_seed/block=K01..K10`, `measurement_seed/job=M1..M9,PC`, and
  `analysis/bootstrap`, `analysis/bootstrap-large`,
  `analysis/bootstrap-positive-control`.

| job | curves | targets/curve | block | executions | spec id (`ecbench plan`) | modelled pod wall |
|:--|:--|--:|:--|--:|:--|--:|
| PC | six 33–41-bit curves | 16 | K10 | 2 016 | `ECS1hb04ab3761ef6` | 459 s |
| M1 | six 33–41-bit curves | 36 | K01 | 6 048 | `ECS1hc7fbcc20114d` | 907 s |
| M2 | both 45-bit | 18 | K02 | 1 008 | `ECS1hb31521d0e1ea` | 1 281 s |
| M3 | both 45-bit | 18 | K03 | 1 008 | `ECS1h63b7706cd10f` | 1 281 s |
| M4 | 49-bit curve 1 (`…a355a6d8`) | 12 | K04 | 336 | `ECS1h27a59486b9c8` | 1 564 s |
| M5 | 49-bit curve 1 | 12 | K05 | 336 | `ECS1h48f8455cdc1c` | 1 564 s |
| M6 | 49-bit curve 1 | 12 | K06 | 336 | `ECS1hd02181fb3af9` | 1 564 s |
| M7 | 49-bit curve 2 (`…11a5de23`) | 12 | K07 | 336 | `ECS1h35a87e9814ce` | 1 921 s |
| M8 | 49-bit curve 2 | 12 | K08 | 336 | `ECS1he72ce6f8a2e3` | 1 921 s |
| M9 | 49-bit curve 2 | 12 | K09 | 336 | `ECS1hfd3fcb2396dc` | 1 921 s |

The job split follows the old pattern: the six small curves share one job, the
45-bit pair shares jobs, and each 49-bit curve is split into target blocks.
Every job stays under 1 921 s of modelled pod time, at least 47 % under the
3 600 s cap (§11).

## 5. Metrics

**Primary (counted, `ecbench.gae`).** θ (`theta_w0`): §8.

**Secondary (predeclared, §9a; never the verdict).**
- θ at w = 0.1 and w = 1, the §5a band.
- `theta_steps`: θ with `steps / √r` in place of S.
- The overhead of `rho.negation` per step: look-ahead additions, escape
  doublings, set-up, starts, verification.
- `theta_v2` = plain / v2, and v2's `lost_walk_steps` share.
- The per-curve θ_c, the slope of ln θ_c against log2 r, and Cochran Q.
- R = S / declared S for each rho arm.
- Fruitless 2-, 4- and other-length cycles, capped walks and abandoned walks,
  each per walk and per curve.
- The PC ratio and the split-half ratios, which are the gates.

Wall time is not a metric of this study. The pod's L2 gate governs only
whether wall time could be quoted, and none is quoted (§10).

## 6. Variable ledger (summary; full in `protocol.json`)

| factor | treatment | how |
|:--|:--|:--|
| instrument | fixed | one image, binary and source for every job; the script checks `binary_sha256` and `spec_id` |
| curve, size | fixed | the ten old curves; curve-equal weights |
| target | randomized | ten new target blocks |
| algorithm seed | randomized | ecbench derives one seed per (workload, round), shared by every arm (paired); disjoint from the old seeds |
| arm order | randomized | `alternate`, balanced over six rounds |
| job | measured | sizes are nested in jobs. Counts do not depend on the job; the workload bootstrap resamples target blocks |
| host, Spot | measured | counts do not depend on the host; an interrupted session is excluded and rerun once |
| analyst | fixed | the frozen script, with its sha256 pinned in `protocol.json` |
| pricing weight | crossed | w = 0 decides; w = 0.1 and w = 1 are reported (§5a) |

## 7. Sample size, from the old variance only

`analysis/power_from_old_variance.py` (stdlib; output `analysis/power.json`)
reads the per-workload mean S of `rho-neg` and `rho-plain` from the six old
comparisons (80 workloads × 6 pairs). That is variance only; no old ratio
enters θ. The steps:

- **Old variance** (derived): the curve-centred per-run CV is 0.41
  (`rho-neg`) and 0.56 (`rho-plain`). The pooled within-curve correlation of
  the two arms' workload means is −0.22, with 95 % interval [−0.43, 0.02]. A
  negative correlation widens a ratio's interval, so the design is sized at
  the lower end.
- **Check against the old run**: the delta method with these numbers gives a
  J1 half-width of 0.1134 for 288 pairs. The observed half-width was 0.1135.
- **Target half-width ≤ 0.05.**
  - √2 − 1.285 = 0.129 > 2 × 0.05, so one interval cannot contain both
    references.
  - 1.285 − 0.05 = 1.235 > 1.2, so at the old point value the interval
    clears 1.2.
  - Pairs needed at the lower-end correlation: 1 729 at θ = 1.285, 1 908 at
    1.35, 2 094 at √2.
  - 36 targets per curve gives 2 160 pairs. The delta-method half-width is
    then 0.041–0.046 at ρ = −0.22, or 0.045–0.049 at ρ = −0.43.
- **Simulation of the exact design** through the frozen estimator. It uses
  lognormal pairs with the old means, CVs and correlation; 200 designs per
  truth; 2 000 resamples each:

| truth θ | ρ | median half-width | P(half-width ≤ 0.05) | coverage | outcome probabilities |
|:--|--:|--:|--:|--:|:--|
| 1.285 | −0.22 | 0.041 | 1.00 | 0.930 | exceeds_1.2_below_sqrt2 0.98, inconclusive 0.02 |
| 1.35 | −0.22 | 0.043 | 1.00 | 0.955 | exceeds_1.2_below_sqrt2 0.865, confirm_sqrt2 0.135 |
| √2 | −0.22 | 0.045 | 0.98 | 0.935 | confirm_sqrt2 0.935, exceeds_sqrt2 0.045, exceeds_1.2_below_sqrt2 0.02 |
| 1.285 | −0.43 | 0.044 | 1.00 | 0.955 | exceeds_1.2_below_sqrt2 0.965, inconclusive 0.035 |
| 1.35 | −0.43 | 0.047 | 0.94 | 0.945 | exceeds_1.2_below_sqrt2 0.76, confirm_sqrt2 0.24 |
| √2 | −0.43 | 0.048 | 0.75 | 0.935 | confirm_sqrt2 0.935, exceeds_1.2_below_sqrt2 0.035, exceeds_sqrt2 0.03 |

  The Monte Carlo standard error of each probability is about 0.015–0.035.
  A truth between the two references, near 1.35, is the hard case: there,
  "consistent with √2" depends on where the interval falls. The table says so
  in advance.

## 8. The pooled estimator (frozen; `analysis/p4_confirm.py`)

- **Pair**: a (workload, measured round) of jobs M1–M9 with a record for each
  of `rho-neg`, `rho-neg-aa`, `rho-plain` and `rho-neg-v2`. Warm-ups are
  excluded. A censored record (`exhausted`, `timeout`) is kept, at its
  recorded `total_gae`.
- **S** = `(total_gae + w · Σ counters[k] for k in cost.unpriced) / √r`. The
  primary uses w = 0.
- **θ** = (1/10) Σ_c mean_{pairs of c} S_plain ÷ (1/10) Σ_c mean_{pairs of c}
  S_neg.
  - The weights are curve-equal. In a ratio of total GAE the two 49-bit
    curves would carry about 76 % of the weight (their share of Σ√r), because
    √r grows about 320-fold over the tier.
  - With the balanced design, this equals the pair-weighted ratio of mean S.
  - It is the pooling across all nine jobs that the old protocol lacked.
- **Interval**: a stratified cluster bootstrap.
  - Strata are curves; all ten are kept in every resample.
  - Clusters are workloads. Each curve's 36 workloads are resampled with
    replacement, and a workload's six pairs stay together.
  - Pairs are never split across arms.
  - **20 000** resamples, drawn from `random.Random(seed)` with
    `seed = derive("analysis/bootstrap")` = 47 393 111 417 401 (`seeds.json`).
  - Indices are drawn curve by curve (sorted slug) and workload by workload
    (sorted workload id). Every statistic is evaluated on the same resamples.
  - The interval is the percentile interval with linear interpolation
    (Hyndman–Fan type 7).
- **Tie-break**: if an endpoint lies within 0.002 of 1.2 or √2, the interval
  is recomputed with 200 000 resamples (`analysis/bootstrap-large`), and that
  interval decides.
- **Sensitivities**: none of them decides the verdict. They are the per-curve
  θ_c, Cochran Q on ln θ_c (inverse bootstrap variances, χ² on 9 dof), and the
  size slope.

## 9. Decision rule (primary: θ at w = 0)

**Gates** (any failure → `inconclusive`, with the gate named):

| gate | condition |
|:--|:--|
| G1 | every session `complete`, record count = planned, local `ecbench verify --replay-all` audit `ok` with no problems, `binary_sha256` = the frozen one, `spec_id` in `specs/spec-ids.json` |
| G2 | `rho-neg-aa` counts identical to `rho-neg` in every pair |
| G3 | PC: bsgs-tb / bsgs-neg, same estimator (seed tag `analysis/bootstrap-positive-control`). The 95 % interval's lower end must be above 1.2, **and** 1.5 must lie inside the 99 % interval. The PC A/A must be identical. At least 90 % of PC pairs must be complete |
| G4 | split-half A/A: the even/odd-round ratio of `rho-neg` and of `rho-plain` must each have 1 inside its 97.5 % interval (Bonferroni over two) |
| G5 | censored share ≤ 1 % in every main arm |
| G6 | complete pairs ≥ 90 % of the planned 2 160 |

**Verdict** for θ with 95 % interval [L, U], when every gate passes:

| verdict | condition | reading |
|:--|:--|:--|
| `confirm_sqrt2` | L > 1.2 and L ≤ √2 ≤ U | the detector clears 1.2, and the saving is consistent with √2 |
| `exceeds_1.2_below_sqrt2` | L > 1.2 and U < √2 | the detector clears 1.2, but the saving is smaller than √2 |
| `exceeds_sqrt2` | L > √2 | named for completeness; unexpected |
| `below_1.2` | U < 1.2 | the ratio is below 1.2 |
| `inconclusive` | L ≤ 1.2 ≤ U, or any gate fails | — |

In the request's words:
- "**confirm**" is `confirm_sqrt2`;
- "**reject 1.2**" is the ratio provably above 1.2, with or without √2: either
  of the two `exceeds_*` verdicts;
- "**inconclusive**" is `inconclusive`;
- `below_1.2` is the remaining named outcome.

**Reported beside the verdict, never changing it**:
- whether 1.285 lies inside [L, U] (the "consistent with ≈ 1.29" flag);
- whether the half-width is ≤ 0.05;
- θ at w = 0.1 and w = 1, with the band order. A rank is stated only where the
  order holds at every weight (§5a).

There is one decisive interval, so no correction is needed. G4 uses
Bonferroni over its two tests.

### 9a. Is √2 the right reference at these sizes? (predeclared secondary)

√2 is the ratio of the declared floors: √(π/2) for a random walk on r points,
and √(π/4) on r/2 classes. That is the ideal-walk constant, and this
protocol keeps it as the primary reference, together with the old 1.285 as a
flag. It is not necessarily the expectation for ecbench's charged counts.

What `rho.negation` does (read at `dbd587a01a`, `src/cryptanalysis/ic_boundary.rs`,
`RhoWalk::negation`, `TunedWalker::walk`, `rho_jumps_for`, `WalkShape::of`;
cited):

- **The walk.** It is an r-adding walk on the classes {P, −P}; the
  representative is the one with the smaller key.
  - It uses 16 jumps for r above 2^22.
  - Distinguished points have ⌊bits(r)/4⌋ bits: 8 at 33 bits, 12 at 49 bits.
    Walks are therefore about 2^8 to 2^12 steps long, and the walk cap is
    20 · 2^dp + 64 steps.
  - A start costs one addition (stride starts).
- **Wiener–Zuccherato look-ahead.** If the canonicalisation flipped the sign
  and the new point would take the same jump again (a fruitless 2-cycle), the
  next jump is tried instead. Each rejected addition is **charged**
  (`look_ahead_additions`).
  - Heuristic: about ½ · 1/16 = 1/32 ≈ 3.1 % extra additions per step.
  - The development check saw 3.3 % at 33 bits (local, not a result).
- **Short-cycle detection.** Each step is compared with the last 16 points. A
  detected cycle (2, 4, …, 16 points) is left by **doubling** its least-key
  member. That is one charged doubling; the steps spent going round the cycle
  are charged as steps. A walk that falls back into the cycle it just left is
  **abandoned**, and its steps stay charged.
- **No Brent detection in `rho.negation`.** A fruitless cycle longer than 16
  points runs to the walk cap: 20 · 2^dp + 64 steps, all charged, then the walk
  is lost. `rho.negation_v2` adds Brent detection and counts lost steps.
- **Accounting.** Canonicalisations (one negation and one key comparison) are
  uncharged and counted (`canonicalisations_uncharged`, about one per GAE).
  At w = 1 this roughly doubles S_neg, so θ falls to about 0.65 (the old J1
  band gave 0.643).

What this predicts for θ at w = 0. These are heuristic or derived, and are
predictions, not results:

1. The look-ahead alone gives θ ≈ √2 / (1 + 1/32) ≈ **1.371**
   (`lookahead_heuristic_reference`).
2. 4-cycles occur at about (r − 1)/(4r³) ≈ 9 × 10⁻⁴ per step for r = 16
   (Bernstein–Lange–Schwabe heuristic, recalled; the development check saw
   about 1 × 10⁻³). Each costs a few steps plus a doubling, about 0.5 %. The
   count per *walk* grows with 2^dp, from about 0.2 at 33 bits to about 4 at
   49 bits. Cycles longer than the window should be rarer still. The old
   v2/neg ratio was 1.002 [0.934, 1.074] at J1.
3. Finite-size set-up (the jump table, about 360 operations; verification;
   the distinguished-point overshoot of about 2^dp steps) is under about 1 %
   of S at 33 bits and under 0.02 % at 49 bits (derived from the counters of
   the development check and from 2^dp/√r). Because it is about the same
   absolute size in both arms, it lowers θ by roughly 0.1–0.2 % at 33 bits
   and by less above. Starts cost one addition per walk in both arms.
4. Walk randomness: Teske's heuristic penalty for an r-adding walk is about
   1/√(1 − 1/r) ≈ 1.03 at r = 16 (recalled). The old run had `rho.plain` at
   R ≈ 1.014 and `rho.negation` at R ≈ 1.093 (cited, 2026-11-07). A larger
   randomness penalty on the class walk is the remaining candidate. Bos,
   Kleinjung and Lenstra reported practical negation-map gains below √2
   (recalled; to be checked against the knowledge base before it is cited).

**Predeclared attribution rule** (`sqrt2_attribution`; reported, not the
verdict):

| label | condition |
|:--|:--|
| `charged_overhead` | the `theta_steps` 95 % interval contains √2 and θ's U < √2: the shortfall is the charged look-ahead, escapes and set-up |
| `walk_level_shortfall` | the `theta_steps` U < √2: the negation walk itself takes more steps than √2 predicts |
| `no_shortfall` | θ's interval contains √2 |
| `unresolved` | otherwise |

Reported with it:
- the overhead fractions;
- whether 1.371 lies inside θ's interval;
- the size slope of ln θ_c, and whether its interval lies entirely below 0
  ("decreasing with size": the fruitless-cycle growth hypothesis) or entirely
  above 0;
- `theta_v2` and v2's lost-walk share;
- the per-curve cycle, cap and abandon rates.

None of these moves the thresholds of §9. A secondary that "explains" the
result does not change the verdict.

## 10. Stopping rules, failures, inadmissible

- Fixed N: ten jobs, once each, PC first. A failing positive control is
  reported, and the main jobs may still run, but C2 is then `inconclusive`.
- A complete session with a passing audit is admitted for counts even when
  the pod's L2 wall gate fails (exit 42/43). Levels gate wall time, never
  counts. No wall time is reported.
- An incomplete session (Spot loss, launcher failure, job timeout) is kept,
  excluded, and rerun **once** under a new job name with the identical spec.
  The rerun is recorded as an amendment with the failed session's identity.
  If the rerun also fails, the job is left out. G6 then decides whether the
  analysis is admissible (≥ 90 % of pairs).
- Censored rows stay rows (G5).
- Inadmissible:
  - pooling any old session, record or ratio;
  - changing curves, seeds, targets, rounds, arms, unit, accounting, weights,
    estimator, resample count or seed, or thresholds after data;
  - dropping or rerunning executions, or rerunning a complete session;
  - reading a secondary as the verdict;
  - local or non-L2 wall time;
  - the script's `--dev` mode for the reported result.

## 11. Execution and cost

- **Placement.** Iso Lab on EKS Spot (AGENTS.md §9a, `isolab-eks`): queue
  `cpu-l2-spot` (8 vCPU / 32 GiB), `c8a.2xlarge`. Each job requests 4 CPU and
  4 Gi with `seconds: 3600` and no checkpoint (`policy: none`; a lost job is
  an incomplete session, §10). At most two jobs fit the pool at once. Counts
  do not depend on co-location.
- **Requests.** `requests/<job>.request.json` (`isolab.eks.request/v1`). Each
  carries `job.build` bound to the frozen image, and
  `provenance.source_revision = dbd587a…`, because the validator requires it
  to equal `job.build.source_revision`. The protocol's own commit is bound by
  the `check-experiment-git` publication receipt and the freeze.
- **Cost model** (`analysis/cost_model.py`, `analysis/cost.json`; derived).
  - It uses the old S and the largest per-arm c8a ns/GAE of the old jobs. It
    reproduces the six old session walls within factors of 0.83–1.06.
  - Total pod time is 14 383 s. Adding an assumed 300 s of provisioning per
    job gives 17 383 s ≈ 4.8 c8a.2xlarge instance-hours.
  - Spot price was USD 0.099–0.158 per hour (describe-spot-price-history,
    us-east-2, 2026-11-07T20Z–11-08T03Z), so compute costs about
    **USD 0.5–0.8**.
  - Proposed ceiling: **USD 10**, covering ten jobs, up to three reruns,
    CodeBuild launchers and S3.
- **Before launch** (main session): commit and push; `check-experiment-git`;
  `freeze`; `scripts/checkpoint.sh init` + push; one `bench.record/v1`
  (`tools/benchrec.py init`). See `README.md`, "Ready-to-launch".

## 12. Predictions (written before any data of this study)

Data already seen (in-sample for the old study, not for this one): the old
RESULTS.md and comparisons (§1); the old bound constants (S `rho.plain` 1.270,
`rho.negation` 0.969 at w = 0, so a ratio of constants of 1.311); the old band
(J1: 1.285 / 1.169 / 0.643 at w = 0 / 0.1 / 1); and the development checks
named in `README.md`.

| id | quantity | prediction |
|:--|:--|:--|
| P1 (decisive) | θ, w = 0 | about 1.30; interval within [1.2, √2), so verdict `exceeds_1.2_below_sqrt2`; 1.285 inside |
| P2 | `theta_steps` | about 1.35, interval below √2 → `walk_level_shortfall` |
| P3 | PC ratio | 1.50 ± 0.05; lower end above 1.2 |
| P4 | A/A | identical counts; split-half intervals contain 1 |
| P5 | band | about 1.18 at w = 0.1 and about 0.65 at w = 1; order unresolved inside the band |
| P6 | size slope | interval contains 0 (no detectable trend over 32–49 bits) |
| P7 | `theta_v2` | within 0.05 of θ |

## Amendments

None.
