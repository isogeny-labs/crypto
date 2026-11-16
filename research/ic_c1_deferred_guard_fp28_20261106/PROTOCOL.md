> **Frozen 2026-11-06T10:11:25Z.** `protocol.json` freeze: protocol_sha256 `43b1ca918c034433f3584f37db439613d11a618c0eec13d3d7831a1fa96d092f`, commit `339292abed16`, filed before affected data. This file is the Coordinator draft at revision 6, unchanged; `protocol.json` governs where they differ. Not yet approved; no EKS launch without the approval decision DEC-20261106-c69de4 and the owner's go-ahead.

# C1, route A: deferred repeat guard with a smaller factor base fixed in advance, at fp28 (counted only)

**Status: DRAFT, revision 6. Not frozen, not approved, not dispatched.**

> **Revision 6 (Coordinator, 2026-11-06, pre-data; section 21).**
> - **Reconciled against `freeze/protocol.json`** (sha256 `5d83e72e…`), field by field. The draft is fixed where it disagreed, and seven changes to `protocol.json` are listed in section 21 (C1–C7). They must be applied, and `validate` rerun, before the freeze.
> - **Route ruling:** keep `runbook`, option (i), in section 18:
>   - `controls: []`, with the reason stated;
>   - the in-job gates are checked by the frozen `analyze-c1.sh`, run as a bound verification step S03 after S2;
>   - every failure stops, with no automatic retry;
>   - a `-r2` runs as a new runbook session on a frozen `-r2` request.

> **Revision 5 rulings (Coordinator, 2026-11-06, pre-data; section 20).** These supersede the revision 4 deadline and budget in the note below.
>
> **A. The job deadline goes back to 3,600 s.** `remote/validate-request` caps `.job.seconds` at 3,600, at ea8e9ef7 and on main.
> - The shared launcher validator is not changed for this study. That would be an infrastructure change outside this protocol, and it is not needed.
> - **Projected session:** 10,800 × 0.1206 s (the measured local rate, P9r) ≈ 1,302 s of measurement, plus about 230 s fixed, ≈ 1,530 s. That leaves a 2.35× margin (3,600 / 1,530), so a node up to 2.59× slower per execution still fits: (3,600 − 230) / 1,302.
> - **Holdback** returns to USD 0.93108 per job. The gate is in section 10.
>
> **B. The execution route is `runbook`.** The ruling and the files it needs are in section 18. This matches the only ICPERF precedent: EXP-ICPERF-283a70 and EXP-ICPERF-284bc4 were approved on lint receipts with `executor_tier: mechanical`. It also matches the model tiers, under which a frozen card is run by executor-mechanical. There is no EKS precedent for either route kind. Pre-approval check R1 (section 18) therefore confirms that the supervisor can run a networked, long trial before the packet is built. If it cannot, the fallback is `implementation`, with a stated reason.

> **Revision 4 rulings (Coordinator, 2026-11-06, pre-data; section 19).** Two rulings and one deletion supersede every count, deadline and budget figure written before this note. Sections 7, 9, 10, 12 and 16 are updated to match.
>
> 1. **P8 power rule.** The section 12 rule was set before any data: "power at a 3 % effect below 0.8 at any weight → raise `targets_per_curve`". It applies, and it is followed, not relaxed.
>    - The new value is **T = 60 targets per curve**, giving 120 workloads and 10,800 executions per session.
>    - **Fallback:** T = 72, if the Executor's P8 calculation, rerun at T = 60 with the conservative SD, shows power below 0.8 at any weight for either variance input (power1 or power2).
>    - If T = 72 also fails, the protocol returns to the Coordinator.
> 2. **Deadline and budget.** The job deadline becomes 5,400 s, and the gate is recomputed in section 10. *(Superseded by revision 5 ruling A: 3,600 s, HB USD 0.93108.)*
> 3. **V removed.** The EKS replay holdback V is deleted. The Executor ran `verify --replay-all` locally with the pinned binary, so the validator replays locally.



- **Author:** Coordinator, 2026-11-06, under DEC-20261106-1d23b6 (`selected_for_design: C1`, `c1_route_ruling.route_A`; crypto-autoresearcher PR #3084).
- **Revision 2** applies the Executor's pre-freeze checks P1–P7 in `next-candidate/PREFREEZE-CHECKS.md` (evidence files in `next-candidate/checks/`). Those checks are design input only. The changes are listed in section 15.
- **Revision 3** closes P14. It re-reads the draft against the canonical documents and adds section 16, the mapping onto `experiment.protocol/v1` that `protocol.json` (P10) must follow. The changes are listed in section 17.
- **What this file does not do:** it changes no hypothesis, lead or goal status. It allocates no `H-*`, `EXP-*`, `DEC-*` or `TASK-*` id, and it authorizes no spend.
- **What it becomes:** a crypto-repo study directory and an `experiment.protocol/v1` record (section 13), and then only after the checks still open in section 12 pass.
- **What a launch needs:** the conditions in section 11.

Proposed protocol id: `IC-C1-DEFERRED-GUARD-FP28-20261106`. Proposed study directory: `isogeny-labs/crypto:research/ic_c1_deferred_guard_fp28_20261106/`.

## 0. Inputs, and what was not read

**Read for revision 1:**
- DEC-20261106-1d23b6, and `next-candidate/RANKING.md`.
- From `v6/repo/research/ic_line_layout_tournament_20261031/`:
  - `PROTOCOL-V6.md` (all);
  - `PROTOCOL-V6-AMENDMENT-1.md`, item 6;
  - `DECISION-V6-REDESIGN-20261106.md`, sections 3.4, 3.8, 12.1 and 14.1;
  - `spec-fp28-v6-counted.json`;
  - `QUALIFICATION-V6-20261106.md`;
  - `eks-results-v6/counted/job-exit.json`;
  - `reports/v6/validation/VALIDATION-REPORT.md`, section 8;
  - parts of `reports/v6/analysis-a/counted-compare-*.json`.
- The crypto `AGENTS.md`, sections 5 and 5a.
- The crypto-autoresearcher `AGENTS.md`, "Experiment protocol, export and review" and "Approval is bounded by execution".

**Read for revision 2:**
- `next-candidate/PREFREEZE-CHECKS.md`.
- `checks/p3c-out.txt`, `checks/p4b-out.txt`, and the roles and seeds of `checks/spec-s1-final-order.json`.

**Read for revision 3 (P14), all in the crypto-autoresearcher checkout `car/`:**
- `agents/coordinator.md`: standing authorization, responsibilities 11 and 12, dynamic dispatch, review architecture, the closure gate and the prohibitions;
- `docs/experiment-protocol.md` (all);
- `docs/agent-runtime-core.md`, the protocol and receipt lines;
- `docs/claims-and-verification.md`, "Refutation artifacts";
- `.claude/skills/experiment-protocol/SKILL.md` (all);
- `schemas/experiment-protocol.v1.json` (all);
- the `validate`, `from-ecbench` and `lint_export` logic of `tools/experiment_protocol.py`.

**Not read:**
- `v6/repo/backlog/XP-20261029-srenew-deferred-guard/` is now on disk. The Executor used its specs for P5. I did not read it myself; it is listed as data seen (no run found).
- `src/`. The Executor read `ic_boundary.rs` and `stages.rs` at ea8e9ef7 for P4.
- The worked example `examples/experiment-protocol/ecbench-l2-smoke-subset/`. P10 should start from it.

**Data already seen.** This is design input only. None of it is ever pooled with C1, and none of it is a test curve:

| Source | What was seen | Use here |
|---|---|---|
| v6 counted session (`1a7f3faa…`, curves `icv1-fp28-tm10425-13a2119a` and `icv1-fp28-t15757-68bdca4a`) | aggregates and the per-phase decomposition (P3) | calibrates the N model (section 3) |
| prime-base holdout-P28 row (`docs/ic/LEADERBOARD.md`, eager guard, N 288 against 407) | 0.929 / 0.974 / 1.193 at w = 0 / 0.1 / 1 | checks the N model |
| F4 pre-freeze smoke (non-study seed 2026110628901, curve `icv1-fp28-tm14825-8419afc7`, one target, L0) | `stride-renew` S 21.066 against `walk` 16.289 | risk statement only |
| Executor's local development solves (`checks/dev/run1`, `run2`; non-study seeds 20261106991xx and 20261106992xx; L0; ratio of means) | candidate/incumbent 0.9135 / 0.8931 / 0.7979; deferred/eager at equal N 1.000 / 0.958 / 0.764; table in section 3 | model check and parity rule. **No claim** |
| Executor's design-size power runs (`checks/dev/power1`, `power2`; non-study curves; 24 workloads × 8 rounds; L0) | `ecbench compare` intervals for incumbent → candidate at w = 0, 0.1 and 1 | P8 and P8r power only. **No claim** |
| Executor's full-size local sessions (`checks/dev/full-job`, T = 12, 2,160 executions; `checks/spec-dev-full-T60.json`, T = 60, 10,800 executions; non-study curve seeds 2026110699501/502; L0) | runtime of 0.177 and 0.1206 s per execution; `verify --replay-all`; at T = 60 the `analyze-c1.sh` output U_w 0.914 / 0.897 / 0.818, L_w 0.886 / 0.869 / 0.786, with 0 parity failures (P9r) | deadline, the P9r script exercise and the P8r cross-check. **No claim** (revision 6) |
| packed-filter `dgb` P28 counted r2 | dgb/rho-neg 21.04 / 20.02 / 15.34 | not used |
| XP-20261029-srenew-deferred-guard | specs only. The main session found no run (git, branches, S3 prefixes) | its seeds and curves are excluded (P5) |

## 1. Claims

**C1-primary (confirmatory).** It concerns exactly one arm, the arm named `candidate`.
- **Recipe:** `prime-abscissa:size=304` with `mitm:negation_folded=1` and `targets=stride-renew-deferred-guard`.
- **Comparator:** the walk incumbent, `prime-abscissa:size=407` with `mitm:negation_folded=1` and `targets=walk`.
- **Claim:** the candidate has lower counted cost S than the incumbent at every weight of the section 5a band (w = 0, 0.1 and 1 GAE per native operation), on fresh fp28 curves.
- **Scope:** `ic.pipeline`, `max_trials` 26479085, `linalg=incremental-gauss` (with `solver=""` and `solver_budget_seconds=0`, the plan defaults), the pinned executable (section 2), and accounting as emitted by that executable.
- **Kind of claim:** a recipe-level, constant-factor, counted improvement of the prime-table reference baseline. It is not an algebraic-IC row, not asymptotic, not about wall time, and not rho-beating.

**Spec roles carry no meaning.** ecbench accepts only `reference`, `baseline`, `candidate` and `control` (P2), so the five descriptive IC arms and the positive control also carry `role: "candidate"`. This protocol, not the spec role, decides the use of every arm. Only the arm named `candidate` is tested for C1-primary. The other arms with role `candidate` (`walk-eager-n304`, `srenew-eager-n407`, `srenew-deferred-n407`, `srenew-eager-n304`, `pc-fb2x`) are descriptive, parity or positive-control arms. No verdict is drawn from them except C1-guard and the control gates. The frozen analysis addresses every arm by name.

**C1-guard (secondary, declared now so that it may be reported as a component result).** Within `stride-renew` at equal N (both 407 and 304), the deferred guard has lower counted S than the eager guard at w = 0.1 and w = 1, and is equivalent at w = 0 (section 7.3). If supported, it is reported as a native-work component saving, outside tables A–C. It does not rank as an improvement of the IC baseline under section 5a, because it does not lower S at w = 0.

**C-rho-gap (reported comparative).** The distance remaining between the incumbent and the candidate on one side and matched `rho-neg` on the other, at each weight, with CIs. It ranks only where the order holds across the band, and it is reported as a distance, not as a gate.

**Descriptive only (no verdict):**
- the N effect, the target-source effect and the guard effect from the factorial (section 4.2). The `C-attribution` entry in section 16 records attribution as out of scope;
- the positive-control magnitude;
- `band --ranks`.

**Confound, stated before data (DEC c1_route_ruling).** Against `walk`, the candidate changes three things: the guard placement, the target source and N. So C1-primary is a claim about the recipe. Attributing it to its factors is descriptive, through the factorial.

The single F4 smoke reading suggested that `stride-renew` costs S at w = 0. The Executor's local non-study sample points the other way: `stride-renew` used 0.98 of the walk trials at N 407. Neither is evidence. The risk is carried into section 8.

## 2. Fixed environment (the pin)

Every arm of every job, and the analysis, uses the same pin.

- Image: `104186811093.dkr.ecr.us-east-2.amazonaws.com/isolab/tasks@sha256:3c4ad8bf8ee1bd1fdc327cb4c0e2597a1fc5ef1cdd889aa21e1b1294bdd22914`
- Source: `ea8e9ef71c3c82027dd9e01535302725519c3fca`
- ecbench sha256: `73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01`. The local copy `v6/bin-pinned/ecbench` was confirmed to match (PREFREEZE §0).

The analysis binary is extracted from the image and checked against `73362eed…`. Any other hash is a usage error (exit 2) and produces no verdict.

`ecbench methods` lists `targets` as "random, walk, or walk-prime-fast". That help text is stale: the parser (`stages.rs:675`) accepts `stride-renew` and `stride-renew-deferred-guard`, and `plan` exits 0 (P1).

**Host.** c8a.2xlarge, EKS Spot, through `isolab-eks`, with `checkpoint: none`. Counted cost does not depend on the host. Host identity (Job, Pod, node) is still recorded.

## 3. The smaller N, fixed now by a stated rule (confirmed by P3)

**Model**, using the per-run GAE of the walk pipeline as calibrated on the v6 counted records (P3, measured per phase):

```
M(N, w, g) = N(N+1) + (1 + w*k_g) * T407 * 407 / N + F + w * G(N)
```

| Term | Meaning and value |
|---|---|
| N(N+1) | oracle set-up, in additions. **Exact in every record** (P3): 166,056 at N 407; 92,720 at N 304 and 663,410 at N 814 in the local solves |
| T407 | 138,582.6 lookups per run, or 138,624.0 trials per run. The two differ by 0.03 %, which leaves the result unchanged |
| T407 × 407 / N | trials at N. In the local samples, trials scale as 1/N to within 0.2–2.3 % (P3) |
| k_eager = 2.0 | measured: 277,206.6 per-trial native operations / 138,624.0 trials |
| k_deferred = 1 | one lookup per trial. The deferred guard probes only decomposing targets (guard probes = relations, about N per run) |
| F | 0.19 S, measured: the charged remainder (relations GAE − lookups, plus verify) is 2,833.4 GAE at √r_eff 14,832. Revision 1 used 0.25 S, which was G's value |
| G(N) | row operations + Legendre symbols + square roots = 3,757.1 per run at N 407, that is 0.253 S per unit w, scaled ∝ N |

Charged relations GAE per trial is 1.020, not exactly 1. The extra 0.02 per trial is inside the model error.

**Ratio and rule:**

```
R(N, w) = M(N, w, deferred) / M(407, w, eager)
N_small = the integer N in [150, 407] that minimizes max over w in {0, 0.1, 1} of R(N, w);
          ties go to the smaller N
```

**Result: N_small = 304, unchanged by P3.**
- Throughout N = 288–320, R(N, 0.1) and R(N, 1) are below R(N, 0). The minimax is therefore the minimizer of N(N+1) + T407·407/N.
- That minimizer does not depend on F or on √r: the objective is 278,256.6 at N = 304 and 278,258.3 at N = 305.
- R(N, 0) varies by less than 0.3 % over 288–320, so the choice is insensitive to model error of that size.
- P3 confirmed the rule's inputs: set-up is exactly N(N+1), trials scale as 1/N, and k_eager is 2. Under the rule's own clause N is therefore not recomputed. It is frozen as 304.

**Projected ratios against the local development solves (design input only, no claim).** The projection is pre-data, with F = 0.19 S, √r_eff = 14,832 and the target source assumed to have no effect on trials. The local column is non-study, L0, ratio of means (`checks/p3c-out.txt`).

| Arm / incumbent | Model, w = 0 / 0.1 / 1 | Local development, w = 0 / 0.1 / 1 |
|---|---|---|
| `candidate` (deferred, N 304) | 0.914 / 0.894 / 0.798 | 0.9135 / 0.8931 / 0.7979 |
| `walk-eager-n304` | 0.914 / 0.949 / 1.113 | 0.920 / 0.955 / 1.119 |
| `srenew-eager-n407` | 1.000 / 1.000 / 1.000 (assumed) | 0.984 / 0.984 / 0.983 |
| `srenew-deferred-n407` | 1.000 / 0.959 / 0.764 | 0.984 / 0.943 / 0.750 |
| `srenew-eager-n304` | 0.914 / 0.949 / 1.113 | 0.9135 / 0.949 / 1.116 |
| `pc-fb2x` (eager, N 814) | 2.39 / 2.24 / 1.50 | 2.365 / 2.208 / 1.473 |
| deferred/eager at equal N (stride-renew, N 407) | 1.000 / 0.959 / 0.764 | 1.000 / 0.958 / 0.764 |

**Model check against the holdout row (other curves):** eager N 288 / 407 is 0.918 at w = 0 (measured 0.929) and 1.150 at w = 1 (measured 1.193).

## 4. Arms

### 4.1 Arm table (spec order is the table order; confirmed by P1, P2 and P6)

All IC arms use `ic.pipeline` with `oracle` `mitm:negation_folded=1` and `max_trials` "26479085". The plan expands the defaults `linalg=incremental-gauss`, `solver=""` and `solver_budget_seconds="0"`. `stride-renew` and `stride-renew-deferred-guard` expand with no extra parameters. The rho arms are `rho.negation`, which the plan expands to `cap_multiple="64"`.

| # | Arm name | Spec role | Protocol use | `factor_base` | `targets` | Method id (P1) |
|---|---|---|---|---|---|---|
| 1 | `incumbent` | baseline | comparator for C1-primary | `prime-abscissa:size=407` | `walk` | ECM1h1729b9b88605 |
| 2 | `candidate` | candidate | **the only C1-primary arm** | `prime-abscissa:size=304` | `stride-renew-deferred-guard` | ECM1h871b51ac7b8a |
| 3 | `walk-eager-n304` | candidate | descriptive: N effect under walk, eager | `prime-abscissa:size=304` | `walk` | ECM1h5caae1adafba |
| 4 | `srenew-eager-n407` | candidate | descriptive: target source at N 407; parity pair with 5 | `prime-abscissa:size=407` | `stride-renew` | ECM1hfb08123d17dc |
| 5 | `srenew-deferred-n407` | candidate | C1-guard at N 407; parity pair with 4 | `prime-abscissa:size=407` | `stride-renew-deferred-guard` | ECM1h06aa2f4e545c |
| 6 | `srenew-eager-n304` | candidate | C1-guard at N 304; parity pair with 2; completes the 2×2 | `prime-abscissa:size=304` | `stride-renew` | ECM1h0c5d4fe610d3 |
| 7 | `pc-fb2x` | candidate | positive control (a counted slowdown by construction) | `prime-abscissa:size=814` | `walk` | ECM1h5d5c4637a846 |
| 8 | `rho-neg` | reference (the first reference) | matched rho; the launcher's admission reference | none | none | ECM1hefea4e0ebe79 |
| 9 | `rho-neg-aa` | control (the **first** control) | rho A/A twin; the launcher's admission control | none | none | ECM1hefea4e0ebe79 |
| 10 | `incumbent-aa` | control (the second control) | counted A/A twin of 1, negative control | same as 1 | `walk` | ECM1h1729b9b88605 |

**Order constraint (P2).** `run-ecbench` takes the first arm with role `reference` and the first arm with role `control`, in spec order. `incumbent-aa` must be `control` and must come **after** `rho-neg-aa`. Otherwise the admission pair becomes `rho-neg` against `incumbent-aa`; that was checked on `checks/spec-s1-try1.json`. With the order above, the pair is `rho-neg` against `rho-neg-aa` in both S1 and S2 (checked with jq). The launcher's `jq -e` pre-check returns `true` on both final specs.

### 4.2 What the factorial supports

The three factors are guard placement, target source and N. The walk deferred-guard cells do not exist on ea8e9ef7; they belong to route B.

| Effect (descriptive unless marked) | Comparison (a → b) |
|---|---|
| N under walk, eager | 1 → 3 |
| N under stride-renew, eager | 4 → 6 |
| N under stride-renew, deferred | 5 → 2 |
| Target source at N 407, eager | 1 → 4 |
| Target source at N 304, eager | 3 → 6 |
| Guard placement at N 407 (C1-guard; parity) | 4 → 5 |
| Guard placement at N 304 (C1-guard; parity) | 6 → 2 |
| Guard × N interaction (point estimate only) | (2/6) / (5/4) |
| Recipe (C1-primary) | 1 → 2 |

## 5. Controls

- **A/A control: counted A/A twins** (protocol role `control_aa`).
  - `incumbent-aa` against `incumbent`, and `rho-neg-aa` against `rho-neg`. Each pair differs only in the nuisance factors of arm label and position in the execution order. Each twin has the same method id as its original (P1), and seeds depend only on (workload, round) (P6).
  - Each twin is therefore expected to be record-for-record identical in counted fields: `ops.ratio_b_over_a == 1`, `ops.ci95 == [1, 1]`, `same_seeds`, `unpaired_runs == 0` at every weight.
  - The noise floor of a counted metric is therefore exactly zero. The between-seed spread that the CIs carry is sampling variance over workloads, not noise.
  - A twin that is not identical makes the session `invalid_measurement`.
- **Negative control: not applicable.** The reason is recorded in `controls.negative.not_applicable_reason`: C1 compares the deterministic counted cost of fixed recipes on fresh instances. It is not a detector searching a population for a property, so there is no null population to sample.
  - The "no difference" null is represented in two places: exactly, by the A/A twins; and at w = 0, by the equivalence of the equal-N guard pairs (section 7.3 step 4), which is predicted by construction (P4).
  - Revision 2 called the twins a "negative control". Under `docs/experiment-protocol.md` section 1 they are the A/A control.
- **Positive control: `pc-fb2x`** (protocol role `control_positive`; its spec role is `candidate`).
  - It runs through the same code path as the incumbent: the same oracle and target source, with only N changed.
  - The set-up is exactly 663,410 against 166,056.
  - Gate: for `incumbent` → `pc-fb2x`, `ops.ratio_b_over_a > 1` and `ops.ci95[0] > 1` at every weight (locally 2.365 / 2.208 / 1.473).
- **Equal-N parity (an implementation check, not science; the rule is fixed by P4).** Pairs: `srenew-eager-n407` → `srenew-deferred-n407`, and `srenew-eager-n304` → `candidate`. The pairing is by `algorithm_seed` and includes warm-ups, so there are 1,080 pairs per comparison per session at T = 60. In every pair, all of the following must hold:
  1. These fields are equal: `status`, the recovered logarithm and its verification, `counters.targets_tried`, `counters.relations_found`, `counters.matrix_rank`, `counters.matrix_rows`, `counters.matrix_work`, and from `phases[relations].native`: `trials`, `relations`, `stride_steps`, `stride_renewals` and `stride_restarts`. The same holds for target RNG draws if they are emitted. The phase `adds`, `doubles` and `scalar_mults` are equal in every phase except `relations`.
  2. Let d = deferred `target_guard_probes` − deferred `relations`, the number of decomposing repeats dropped by the deferred guard. Then 0 ≤ Δ total GAE (deferred − eager, at w = 0) ≤ d, and Δ `relations`-phase adds = Δ total GAE, with Δ doubles = Δ scalar_mults = 0. Locally, Δ was 0 in 364 of 384 pairs and +1 in 20, and d = Δ in every pair. This replaces revision 1's looser bound (repeats = eager guard probes − eager lookups, about 41 per run).
  3. Δ `lookups` (deferred − eager) = eager `repeated_targets_skipped`, exactly. This held in all 384 local pairs.
  4. Deferred `target_guard_probes` ≤ deferred `relations` + d (true by the definition of d), and eager `target_guard_probes` = eager `trials`.

  **Mechanism (P4 source read).** The stream is identical by construction:
  - the target stream is generated before the guard, by the same code for both variants;
  - in neither variant does the guard firing renew or restart the stride;
  - only a fresh decomposition renews.

  A failed parity check is an implementation defect (DEC closes_when). It is never a negative result. It stops the C1 verdict and opens a fix.

## 6. Variable ledger

| Class | Variable | Value or handling |
|---|---|---|
| Manipulated | guard placement | eager (`walk`, `stride-renew`) or deferred (`stride-renew-deferred-guard`) |
| Manipulated | target source | `walk` or `stride-renew` |
| Manipulated | N (`prime-abscissa:size`) | 407, 304 (section 3), or 814 (positive control only) |
| Held constant | oracle | `mitm:negation_folded=1` (no `layout=lines`) |
| Held constant | `max_trials` | 26479085, a cap and not binding (the local maximum was about 265k trials at N 304). The decision requires `censored_pairs == 0` |
| Held constant | linear algebra and solver | `incremental-gauss`, `solver=""`, `solver_budget_seconds=0` (the plan defaults, P1) |
| Held constant | rho parameters | `cap_multiple=64` (the plan default) |
| Held constant | executable | the section 2 pin |
| Held constant | accounting | as emitted at ea8e9ef7. The band charges exactly the five named IC counters: `sqrt_solves`, `lookups`, `row_ops`, `legendre_symbols` and `target_guard_probes`. P3 confirmed this from the v6 data, since the band's `native_sum` equals their sum, which settles P11. Rho is charged `canonicalisations`. `table_insert_attempts` and the 407 set-up `lookups` sit in `work_native` and are not charged on any IC arm (conservative; it favours the larger N). No accounting change inside the study |
| Randomized | curves | `prime_search`, 28 bits, from the frozen curve seeds (section 7.1) |
| Randomized | targets | 60 per curve (revision 4), public, from the frozen target seed |
| Randomized | algorithm seeds | derived by ecbench from (workload, round) only, shared by all 10 arms (P6 at T = 12: 216 seeds per session, no group with more than one seed; P6r reruns it at T = 60, expecting 1,080) |
| Measured, primary | counted S at w = 0, 0.1 and 1 | `ecbench compare --weight w`: the `ops` block, and weighted per-arm S from the per-workload means |
| Measured, secondary | unpriced counters, per arm | `cost.unpriced` and `unpriced_counts`, all listed (section 5a). IC arms list 5 counters; rho lists `canonicalisations_uncharged` |
| Measured, diagnostic | per-phase adds, doubles, scalar_mults and gae, and `native{}` and `work_native{}` by counter name | `phases[]` (P7a). Rho records have no IC `counters` block and no IC `phases[].native` counters, so the scripts skip rho for these reads |
| Recorded, not read | every wall-time field (including `phases[].wall_ns`) | never read or reported |
| Nuisance | host, Spot interruption | recorded; no effect on counts; host loss is handled by section 9 |

## 7. Seeds, curves, sample size and analysis

### 7.1 Seeds and curves (fixed; slugs from `ecbench plan`, P1 and P5)

| Purpose | Session S1 | Session S2 (replication) |
|---|---|---|
| curve seeds → slug (log₂ r) | 2026110628201 → `icv1-fp28-t10999-67500d78` (27.72); 2026110628202 → `icv1-fp28-tm6425-08e2435f` (27.50) | 2026110628211 → `icv1-fp28-t14969-7ca8e8a5` (27.29); 2026110628212 → `icv1-fp28-tm4717-89c926fe` (27.62) |
| target seed | 2026110628203 | 2026110628213 |
| measurement seed | 2026110628204 | 2026110628214 |
| counted bootstrap resample seed | 20261002, the pinned default | same |
| plan spec id (T = 60, P1r; revision 6) | ECS1h3229b941623b | ECS1h1e755d9669a9 |
| plan spec id at T = 12 (void) | ECS1hc027a4dab62e | ECS1hab11b5206d1d |

**Curve rule.**
- Each curve is ecbench's deterministic `prime_search` output, at 28 bits, for its seed, with no analyst choice. The four slugs above are pairwise distinct.
- They do not collide with any of these: the three v6 curves, XP-20261029's P28 curves (`tm13337-3a482fe3`, `tm5065-1146b4a5`), the F4 curve, `docs/curves/registry.json` at ea8e9ef7, or the only fp28 slug in `docs/ic/LEADERBOARD.md` (`icv1-fp28-t28201-65099091`).
- Fallback, written now: the session's seeds move up by +100, once. The slugs are already computed and are informational: S1 `t7075-59d612a9` and `t12031-c0912d73`; S2 `t26137-730c77fc` and `tm8753-e0a59910`. The fallback is used only if the residual P5 check finds a collision. A second collision stops the freeze.

**Residual P5 ruling (Coordinator).**
- **Holdout-P28 slugs: blocking.** The holdout row is data seen that shaped this design, so its curves must be shown not to be C1 test curves. The check is free (P5r below).
- **F6 curves: not blocking.** The F6 seeds (2026110628904–906) are documented as development-only, they are distinct from every C1 seed, and no design figure came from F6. Its slugs are still computed alongside P5r for the record, at no cost.
- **"Every committed crypto session": blocking**, but only as a slug-string search (P5r). The registry is known to be incomplete: the v6 slugs are not in it.

### 7.2 Sample size

**Per session (revision 4):** 2 curves × **60 targets** × (1 warmup + 8 measured rounds) × 10 arms = **10,800 executions**.
- That is 1,080 per arm, of which 960 are measured.
- It gives 960 measured pairs per arm-pair comparison, over 120 workloads (dof 119).
- Parity uses 1,080 pairs, warm-ups included.
- ecbench derives 1,080 algorithm seeds per session.

**Fallback at T = 72:** 12,960 executions; 1,296 per arm, of which 1,152 are measured; 144 workloads.

S1 and S2 are identical except for seeds and job names. The curve seeds, and therefore the four slugs, are unchanged. The first 12 targets per curve may coincide with the revision 2 set, but none of them has data.

**Power (P8, Executor, Part 2).** The calculation is the Executor's single-session P(U_w < 1). Its variance comes from two design-size local non-study runs (`checks/dev/power1` and `power2`).

| T (targets per curve) | Power at a 3 % effect, w = 0 / 0.1 / 1 | Power at the projected effects |
|---|---|---|
| 12 | 0.34–0.42 / 0.33–0.39 / 0.17–0.27 | 0.998–1.000 |
| 24 | 0.63–0.80 / 0.61–0.74 / 0.45–0.47 | — |
| 36 | 0.80–0.96 / 0.78–0.93 / 0.63–0.70 | — |

- **Expected at T = 60** (√n scaling of the T = 36 values, which is my extrapolation, not a measurement): about 0.84 or more at w = 1, and higher at w = 0 and 0.1.
- **Confirmation:** P8r reruns the calculation at T = 60 before the freeze. `sample_size.power.passes` is set from that rerun.
- **Joint power** for both sessions is about the square of the single-session value.
- **Why 3 % is the effect to power for.** A false closure is the expensive error here. The owner criterion counts every verified measured improvement, and closes_when closes on any U_w ≥ 1. An underpowered session would close a real small gain.

### 7.3 Frozen analysis (script `analyze-c1.sh`, sha256 frozen in `protocol.json`)

The script runs on each session separately, with the pinned binary.

**1. Integrity:**
- `ecbench verify --replay-all` exits 0;
- the audit is OK;
- `records.jsonl` holds 10,800 records (12,960 at T = 72), with `executions_planned == records_written` equal to that count, all verified and every replay reproduced;
- there are no invalid outcomes. The check computes them from the records. The launcher's `l2-gate.json` `invalid_outcomes` must be `[]` when that file is present; ecbench's `session.json` carries `invalid_outcomes: null` (P9 note);
- `input-spec.json` and `session/spec.json` are `jq -S -c`-equal to the frozen spec, and the build equals the pin.

**2. Controls (section 5):**
- the A/A twins are exact at w = 0, 0.1 and 1;
- the `pc-fb2x` gate passes at all three weights;
- equal-N parity passes, rules 1–4, in all 1,080 pairs of each parity comparison (1,296 at T = 72).

**3. Primary comparison.** `ecbench compare --a incumbent --b candidate --weight w` for w ∈ {0, 0.1, 1}. From `ops` it reads:
- `ratio_b_over_a`;
- `ci95` (linearised t over workloads);
- `ci95_bootstrap` (cluster);
- `status`, `same_seeds`, `unpaired_runs`, `pairs`, `censored_pairs` and `band`.

Required: `status == "ok"`, `same_seeds`, `pairs == 960` (1,152 at T = 72), `unpaired_runs == 0` and `censored_pairs == 0`.

**Weighted S.** The top-level `a.mean_s` and `b.mean_s` stay at the w = 0 value at every weight (P7, and revision 1's reading of the v6 files), so they are never used as weighted S. Weighted per-arm S is the mean of the per-workload `a_mean_s` and `b_mean_s` in that weight's compare file, or the `band` output.

The session's upper bound at weight w is U_w = max(`ci95[1]`, `ci95_bootstrap[1]`), and its lower bound is L_w = min(`ci95[0]`, `ci95_bootstrap[0]`).

**4. Secondary comparison (C1-guard).** The same reads for `srenew-eager-n407` → `srenew-deferred-n407` and `srenew-eager-n304` → `candidate`.
- Equivalence at w = 0 means both CIs lie inside [0.995, 1.005]. The margin is fixed now; the locally observed difference is at most 3.5 × 10⁻⁶ relative.
- Superiority at w = 0.1 and w = 1 means U_w < 1.

**5. Descriptive:**
- every comparison in section 4.2 at three weights;
- `rho-neg` → each IC arm at three weights, which gives the distance remaining with its CI;
- `ecbench band --ranks` over all arms;
- every arm's unpriced counters, listed;
- per-phase tables from `phases[]` (IC arms only).

**What the script writes:** `reports/c1/s{1,2}/…` and `verdict.json`. It exits 0 whatever the verdict, and 2 on a usage error or a binary that is not the pinned one.

## 8. Decision rule (matches DEC-20261106-1d23b6 closes_when)

**Per session, in order of precedence:**
1. An integrity or control gate fails → `invalid_measurement`. The session is not evidence; it goes back to the Executor with the defects listed. A timeout, crash or host loss is infrastructure signal.
2. Equal-N parity fails → `implementation_defect`. This is not a negative result. The verdict is stopped and a fix is opened.
3. U_w < 1 at every w ∈ {0, 0.1, 1} → `pass`.
4. Otherwise (U_w ≥ 1 at some w) → `fail`. This is closes_when: "the candidate/incumbent CI upper bound is ≥ 1 at any of w = 0, 0.1 or 1". The weights at which L_w > 1 are recorded as `fail_adverse@w`.

**Overall (both sessions are pre-registered and run unconditionally; nothing is stopped on the outcome):**

| S1 | S2 | Verdict | Coordinator action admitted |
|---|---|---|---|
| pass | pass | `supported_within_scope` | evidence review `support`, after the independent validator. Replicated, so a KN-FIND is warranted. Leaderboard and `benchmarks/` only after `VALIDATION.json` |
| fail | fail, with `fail_adverse` at a common w in both | `refuted_within_scope` at that weight | `reject_scoped`, after archiving an `empirical_only` refutation note (claims-and-verification, "Refutation artifacts"); KN-FIND for the boundary |
| fail | fail (otherwise) | `not_supported_within_scope` | close C1 route A at fp28 at the stated scope (closes_when): `weaken`, no `reject_scoped` |
| pass | fail | `inconclusive` | no third session under this protocol; a successor needs a new protocol |
| fail | pass | `inconclusive` | as above |
| valid | invalid after its permitted rerun (or the reverse) | single unreplicated | a pass gives `analyzed`, never `supported`; a fail gives `weaken` plus replication under a new protocol, never `reject_scoped` |
| `implementation_defect` in either session | (either) | `implementation_defect` | fix and re-run under an amendment; no scientific verdict |

**Closure obligations (coordinator.md, "Closure gate"; claims-and-verification, "Refutation artifacts").**
- Before any adverse transition (`weaken`, closing C1 route A, or `reject_scoped`), the analysis archives an `empirical_only` refutation note in a snapshot commit, before the decision. The note records `proof_status: empirical_only` and lists the S1 and S2 compare files in `proof_refs`.
- A `reject_scoped`, or a closure resting on "no route remains", also needs an `obstruction` block on its evidence record. The block gives the quantity (candidate/incumbent counted S at weight w), its measured value with its CI from both sessions, the runs it is read from (session ids and `records.jsonl` sha256), and the scope of section 14, no wider.
- It also needs a `resource_check` that genuinely asks which theory takes the measurement as its hypothesis. The obvious reading to examine is route B: the target source masking a guard gain.
- A closure whose obstruction is prose, or measured over a narrower scope than the closure asserts, is refused.
- A `weaken` that leaves C1 open for route B needs the refutation note only.

**Promotion gates.** C1 makes no asymptotic-complexity claim, so the four promotion gates for asymptotic claims (coordinator.md) do not apply. The section 3 model is a pre-data design aid for choosing N. No claim depends on it as a heuristic.

**C1-guard (secondary).** The same table, read on the section 7.3 step 4 criteria. It needs both N levels and both sessions.

**Route B trigger (roadmap, not a verdict).** Route B is recommended to the owner when all three hold:
- C1-primary fails;
- C1-guard passes;
- the target-source effect `incumbent` → `srenew-eager-n407` has L_0 > 1 in both sessions.

**§5a.** The candidate ranks against the incumbent only if the pass holds at all three weights. An IC arm ranks against rho only where the order holds across the band. Distance to rho is reported as distance, not as a gate.

## 9. Stopping rules and execution

1. **Pre-freeze.** Every blocking item in section 12 is closed and recorded in `prefreeze-checks.md`. That file is the Executor's PREFREEZE-CHECKS.md plus the residual items. Then:
   - `experiment_protocol.py validate`, and `freeze --filed-before-affected-data`;
   - the merge;
   - the Coordinator approval DEC;
   - `experiment_preapproval.py` reporting `approvable: true`;
   - `newest_experiments.py` reporting `execution_state: ready`;
   - the owner's go-ahead (section 11).
2. **S1, once.** Launched from a frozen experiment-director card relayed verbatim to executor-mechanical. After `job-exit.json` is written, only transport and identity integrity is read before S2. That is the `check_job.py` set (section 18): SHA256SUMS, `job-exit.json` and its exit code, image, source and ecbench identity, spec and build equality, job name, and the record count.
   - Audit, replay, `invalid_outcomes` and the A/A exactness are not read before S2 (revision 6). They run, with every other gate, in the post-S2 verification step S03 (section 18). This reads strictly less before S2 than revision 5 did.
   - No `compare` is run, and the parity fields are not opened, until S2 has written `job-exit.json`.
3. **S2, once,** whatever S1's outcome, unless S1 shows a spec or launcher defect that would equally affect S2. In that case the director stops and returns to the Coordinator, and the fix needs a `protocol_amendment`.
4. **Analysis, export and review.**
   - Run `analyze-c1.sh` on S1 and S2.
   - Write one `experiment.export/v1` per session, beside the raw records and never inside the sealed session directory: `experiment_protocol.py from-ecbench <session> --protocol protocol.json --out export-s{1,2}.json --export-id <id>`, then `check-export --verify-files`.
   - The `flow` table goes in the report.
   - The adapter carries only the record's top-level `counters` block and `field_ops`, not the `phases[].native` uncharged counters. The analysis therefore supplements each export's rows with the five uncharged counters (and `canonicalisations` for rho), so that a reviewer can apply the section 5a band (docs/experiment-protocol.md section 3). Where the adapter cannot carry them, the export notes the gap and binds `records.jsonl` by hash.
   - The Coordinator writes the `review_plan` (coordinator.md, "Review architecture") **before** any reviewer runs:
     - **prior:** the section 3 projection, a pass at all three weights;
     - **joints, one owner each:** replay and determinism; parity rules 1–4; the weighted-band arithmetic and the use of per-workload means; curve freshness; the integrity of the budget and route;
     - **worked attacks:** for example, recompute U_w from `records.jsonl` without ecbench `compare`, and perturb one parity field;
     - **blindness;**
     - **a blind re-derivation of the primary ratio** at each w, from `records.jsonl` by a script that has not read `analyze-c1.sh`.
   - Then the independent validator: replay certificate, recomputation and validator report. Then `tools/check_review_independence.py --batch <dir>`. Then the Coordinator's evidence-review decision.
5. **Stop.** No third session, and no change to seeds, N, arms, weights, thresholds or margins.

**Exit codes.** Exit 0, 42 or 43 is accepted only if every section 7.3 step 1 condition holds. 42 is expected, because rho at fp28 is graded below L2; v6's counted job exited 42.

**Host loss.** A job that writes no `job-exit.json`, or one whose non-zero exit other than 42 or 43 the director classifies as infrastructure before any record is opened, is rerun once, whole, as the same request named `-r2`. Counts are deterministic given the seeds, so a rerun cannot select outcomes. Any further failure ends that session.
- **The `-r2` requests are frozen now (revision 6).** They are `launch/request-s1-r2.json` and `launch/request-s2-r2.json`, byte-equal to the first-attempt requests except `job.job_name` (`ic-c1-dg-s{1,2}-20261106-r2`). Each must pass `validate-request`, and both are bound in the freeze. No request is edited at launch time.
- **How a `-r2` runs.** The runbook does not retry automatically (section 18). A host loss stops the runbook session. The director classifies the failure from `raw-result.json` and the launcher log, and the Coordinator records an approval DEC for a new runbook session. That session's trial plan names the frozen `-r2` request. It records the section 10 gate line, and it is bound to the same EXP and protocol sha.

**Spec.** The Executor's `checks/spec-s1-final-order.json` and `checks/spec-s2-final-order.json` are the basis of `spec-c1-s1.json` and `spec-c1-s2.json`: `isolation_required: "L2"`, `rounds` 8, `warmup` 1, `order` `alternate_measured`, `timeout_seconds` 600, and the arm order and roles of section 4.1. Their sha256 values are recorded at P10.

**Route.**
- Requests: `launch/request-s1.json` and `launch/request-s2.json` (`isolab.eks.request/v1`), each embedding its spec `jq -S -c`-equal to the spec file, with the pinned `job.build`.
- Job names: `ic-c1-dg-s1-20261106` and `ic-c1-dg-s2-20261106`, fixed as the protocol date whatever the launch day, so the frozen requests carry no placeholder. Each has a **3,600 s** deadline (`seconds` 3600). This is revision 5; revision 4's 5,400 s is refused by `validate-request`.
- **Deadline arithmetic.**
  - The Executor's full-size local T = 60 session ran 10,800 executions in 21 min 43 s, 0.1206 s per execution. That projects to about 1,302 s of measurement plus about 230 s fixed, about 1,530 s in all. The margin is 2.35×.
  - Revision 4's 0.177 s figure came from the T = 12 local run and is superseded.
- **CodeBuild.** The build waits up to `seconds` + 180 = 3,780 s, which is inside the launcher's 100-minute (6,000 s) build timeout.
- **Deadline-hit rule (new in revision 5).** If a session ends at its deadline, it is an infrastructure outcome, not evidence, whether or not it writes a `job-exit.json`. In that case the same request is **not** rerun as `-r2`, because the same request would hit the same deadline. The director stops and returns to the Coordinator. The fix is a `protocol_amendment` filed before any record of that session is opened, for example splitting each session into two jobs by curve.
- Launch: `taskq/deploy/eks-karpenter/remote/launch-remote` from the agent, with the executor's existing AWS identity.
- The executor verifies the image digest, the source, the ecbench hash and the S3 `SHA256SUMS`.
- Before each run, `.resume/RESUME.md` is filled with `scripts/checkpoint.sh init` and pushed to S3.
- No L2, replay or A/A wall admission is needed, because no wall field is read.

## 10. Budget (hard cap USD 5.00; the §14.1 rolling hard-bound gate from v6)

- `HB(job) = (job seconds / 3600) × 0.43108 + 100 × 0.005` USD. For the revision 5 job of 3,600 s that is 0.43108 + 0.5 = **USD 0.93108**, the same as v6 and revision 3. Revision 4's USD 1.14662 (5,400 s) is withdrawn.
- Before each launch L, including any `-r2`, the card records `actual_spend + HB(L) + HB(every first-attempt job not yet launched)` and requires it to be at most USD 5.00. `actual_spend` is read from the billing or cost record of each finished job. Where that is not yet available, the job's HB is used, never an estimate.
- The first-attempt jobs are S1 and S2 only. There is no holdback V (revision 4): the validator replays locally with the pinned binary.

| Launch | Gate line (USD) | Result |
|---|---|---|
| S1 | 0 + 0.93108 + 0.93108 (S2) = 1.86 | pass |
| S1-r2 (S1 at hard bound) | ≤ 0.93108 + 0.93108 + 0.93108 = 2.79 | pass |
| S2 (after S1 and S1-r2 at hard bound) | ≤ 2 × 0.93108 + 0.93108 = 2.79 | pass |
| S2-r2 (everything above at hard bound) | ≤ 4 × 0.93108 = 3.72 | pass |
| a fifth launch | 5 × 0.93108 = 4.66 would pass the arithmetic | **forbidden anyway.** `retries.max_attempts` is 2 per session, `fixed_n` allows no third session, and the deadline-hit rule (section 9) forbids rerunning a request that timed out. The director stops and returns to the Coordinator |

Worst case committed by the protocol: **USD 3.72**, leaving USD 1.28 of the cap unused.

**Expected spend: about USD 0.6–1.0 in total.**
- **Per job:** about 1,830 s of node time (about 1,530 s run plus about 300 s for provisioning and image pull). At the USD 0.43108 on-demand rate that is about USD 0.22. The CodeBuild launcher waits for the job, about 32 minutes at USD 0.005 per minute, so about USD 0.16. That is about USD 0.38 per job.
- **Total:** two jobs, about USD 0.76. Spot pricing lowers the node share. The top of the range allows for one `-r2`.
- These figures assume the EKS node runs at the local rate of 0.1206 s per execution (P9r, L0). A node up to 2.59× slower per execution would still finish inside 3,600 s, and the deadline-hit rule (section 9) covers anything slower. Revision 4's figure of 0.177 s per execution and its USD 0.8–1.2 range are superseded.

**Comparison with T = 12.** Raising T to 60 multiplies the measurement time by 5. CodeBuild and provisioning do not scale, so cost rises from about USD 0.2 to about USD 0.4 per job. Both figures are far inside the cap.

**Prices.** The card records the current on-demand and Spot prices before launch. That is P13, a launch-time item, not a freeze blocker.

## 11. Owner gate

The run needs **the owner's explicit go-ahead for EKS spend before any launch** (DEC-20261106-1d23b6 `owner_gate`). That covers S1, S2 and any `-r2`. The go-ahead is in addition to:
- this protocol frozen and merged;
- a Coordinator approval DEC that passes "Approval is bounded by execution".

None of the following is that go-ahead: drafting this file, the pre-freeze checks, the local development solves, or any agent message. Relaying the request to the owner is the main session's task.

**What the gate covers.** The gate is the cloud-spend limit that DEC-20261106-1d23b6 records under "ECC comes first within owner cloud limits" ("owner cloud limits bind each launch"). It is not a request to approve the protocol. Under the standing user authorization (AGENTS.md; coordinator.md; agent-runtime-core.md), the Coordinator approves a complete, runnable protocol without asking the user. The approval DEC cites that authorization.

## 12. Pre-freeze checks: status, and what still blocks

**Closed by the Executor (PREFREEZE-CHECKS.md):**

| # | Check | Outcome |
|---|---|---|
| P1 | plan validity of all arm strings and both specs | PASS: 2,160 executions each, exit 0; defaults recorded (section 4.1) |
| P2 | roles and launcher pre-check | PASS with changes, applied in sections 1 and 4.1 |
| P3 | v6 phase decomposition | PASS: set-up = N(N+1), k_eager = 2.0, F = 0.19 S; N 304 stands |
| P4 | equal-N parity and stream identity | PASS: the stream is identical, and the bound is tightened (section 5) |
| P5 | seed and curve collisions | PASS for v6, XP-20261029, F4, the registry and LEADERBOARD. The residual is P5r below |
| P6 | shared `algorithm_seed` | PASS: 192 measured pairs per comparison |
| P7 | counter emission and per-phase names | PASS (a and b); rho lacks the IC `counters` block |
| P11 | the band charges guard probes | PASS from data (P3: `native_sum` equals the sum of the five counters) |
| P12 | walk renews on a repeat | not needed for route A; deferred to route B |
| P14 | re-read against the canonical documents (Coordinator, revision 3) | CLOSED: gaps fixed; schema mapping in section 16 |

**Revision 4 status (Executor, PREFREEZE-CHECKS.md Part 2).**

| Check | Status |
|---|---|
| P5r | PASS. The holdout-P28 slug is `icv1-fp28-tm1499-56171ddc`; the F6 slugs differ from ours. GitHub code search on `isogeny-labs/crypto` found no C1 slug or seed. Limits: default branch only, possible index lag, large files skipped. **Accepted:** nothing of this study is committed, and the positive controls returned hits |
| P8 | ran; it triggered the T = 60 ruling. **P8r** must rerun it at T = 60 (conservative and optimistic SD, power1 and power2 variance). Pass condition: at least 0.8 at every weight at a 3 % effect; otherwise T = 72 |
| P9 | PASS at T = 12 (`analyze-c1.sh` sha256 `1b1f4450…`) |
| P10 | `validate` PASS with 0 findings (`cmp-recipe` accepted as `one_factor`); specs and requests built; `validate-request` exit 0 |

**Revision 5 status (Executor, PREFREEZE-CHECKS.md Part 3).**

| Check | Status |
|---|---|
| P1r | PASS. 10,800 executions per spec, slugs unchanged. S1 `ECS1h3229b941623b`, sha256 `d2af59dd…09e8ad`; S2 `ECS1h1e755d9669a9`, sha256 `688ec9a0…02cf34` |
| P6r | PASS. 1,080 groups, one seed per group, all 10 arms in every group |
| P8r | PASS at T = 60. The minimum is 0.838 (power2, w = 1, conservative SD), so T = 72 is not used. **The joint two-session power at that minimum is about 0.70.** This is accepted, not hidden. The pre-registered rule is per session (≥ 0.8 at each weight). Section 8 maps pass+fail or fail+pass to `inconclusive`, never to a refutation, and `protocol.json` agrees. Revision 5 wrongly wrote `not_supported_within_scope` here; revision 6 corrects it. The note must state the joint figure |
| P9r | PASS. `analyze-c1.sh` sha256 `66aa2ffd91e10f5d1f1a65be086cd17c3bf388b4299157f646b75ea49d994dcc`; counts derived from the spec; 29 stubs pass; full-size non-study T = 60 session run end to end. Its ratios are development data with no claim, and they do not move N, T or any threshold |
| P10r | `validate` PASS with 0 findings (`protocol.json` sha256 `99698a24…2f8b5e`). **`validate-request` FAILS at 5,400 s** (the cap is 3,600), so revision 5 sets 3,600 s. `protocol.json` therefore needs the P10r2 updates below, and its sha changes |

**Revision 6 status (Executor, PREFREEZE-CHECKS.md Part 4).**

| Check | Status |
|---|---|
| P10r2 | PASS. Both requests have `seconds` 3600, and `validate-request` exits 0. s1 `683cc67d136ab6a2…c779`, s2 `d58a601e3a5503bd…4bf1` |
| P10r3 | PASS. `validate` reports 0 findings. `protocol.json` sha256 `5d83e72ebc42517c359d12bc27b6240f67eb83157cac518fd60f658a188468b9`, with `links.hypothesis_ids` [H-ICPERF-4cb536]. The deadline-hit rule is stored as kind `invalidation`. **This sha changes again with C1–C7** |
| R1 | (a) PASS. (b) PASS in the tool and untested at runtime; S01 tests it. (c) PASS as `exempt`. (d) the empty form passes and in-job arms are refused, so `controls: []` is used. (e) PASS in lint, but the supervisor's exit codes make automatic retry unusable, so every failure stops. Ruling in section 18 |
| Reconciliation | done (Coordinator, revision 6). The draft is fixed in sections 0, 7, 9, 12, 13, 16 and 18. `protocol.json` needs C1–C7 (section 21) |

**Rework that blocks the freeze, revision 6 (all free):**
- **P10r4 (Executor).**
  - Write `freeze/launch/request-s{1,2}-r2.json` as copies of the first-attempt requests with only `job.job_name` changed to `…-r2`.
  - Run `validate-request` on both, check `cmp` of the `jq -S -c` output with `.job.job_name` deleted, and record their sha256 values.
- **P10r5 (Executor).** Apply C1–C7 to `protocol.json` through `gen/build_protocol.py`. Then run `validate`, which must report 0 findings, and record the new sha256.
- **Freeze.** It follows these two items, using the section 12 freeze procedure below. The `-r2` requests are part of the frozen package.

**Rework that blocked the freeze in revision 5 (closed by Part 4):**

- **P10r2: requests.**
  - Set `seconds: 3600` in `freeze/launch/request-s{1,2}.json`, changing nothing else.
  - Rerun `validate-request` on both; each must exit 0.
  - Recheck `jq -S -c .job.spec` against the spec files.
  - Record the new request sha256 values. The revision 4 values (`a4a68e03…`, `035612631c…`) are void.
- **P10r3: `protocol.json`.**
  - `execution.timeout_seconds` 3600;
  - `preflight_runtime` with the revision 5 values in section 16.8;
  - the budget note: HB USD 0.93108, the gate in section 10, worst case USD 3.72, and the deadline-hit rule;
  - the `power.note` stating the joint two-session figure, about 0.70;
  - the request sha256 values in `extensions`.
  - Rerun `validate`, which must report 0 findings, and record the new sha256.
  - `links.hypothesis_ids` stays null until the Coordinator allocates the H- id, which happens before the freeze (below).
- **Hypothesis id (Coordinator, before the freeze).** Allocate one `H-ICPERF-…` with `allocate_id.py --next hypothesis --date 20261106` and confirm it with `--check`. Set `links.hypothesis_ids`, then rerun `validate`. The protocol cannot freeze with a null hypothesis link, because the specification and the trial plan bind it.
- **Draft reconciliation (Coordinator).** Re-read this draft against the final `protocol.json`, field by field over section 16, before the freeze. The freeze is of `protocol.json`, and any difference is fixed in the draft, not in the frozen file.

**Rework that blocked the freeze in revision 4 (all closed by Part 3, except P10r's `seconds`, which P10r2 replaces):**

- **P1r.** Set `targets_per_curve: 60` in `freeze/spec-c1-s{1,2}.json`, changing nothing else. Then `ecbench plan` must exit 0 with 10,800 executions per spec. Record the new spec ids and sha256 values, and confirm the four slugs are unchanged.
- **P6r.** Run `plan --doc` on S1. Pass: 1,080 (workload, round) groups, none with more than one seed, all 10 arms per group.
- **P8r.** As in the table above.
- **P9r.** Make the counts in `analyze-c1.sh` parameters derived from the frozen spec, or set them to 10,800 / 960 / 1,080, and replace any literal 2,160 / 192 / 216.
  - Rerun the 29 stubs with the new counts.
  - Run one full-size real local session at T = 60 on non-study curves (about 32 minutes at L0).
  - Record the new sha256.
- **P10r.**
  - Set `seconds: 5400` in `freeze/launch/request-s{1,2}.json`, and fix the job names to `ic-c1-dg-s1-20261106` and `ic-c1-dg-s2-20261106`. Rerun `validate-request`; it must accept `seconds` 5400. If it caps below that, return to the Coordinator. (Returned: the cap is 3,600, and revision 5 rules 3,600 s.)
  - Update `protocol.json`:
    - `sample_size.instances` 120 (144 at T = 72);
    - `preflight_runtime` (per-sample 0.177 s, fixed 230 s, total about 2,140 s);
    - `execution.timeout_seconds` 5400;
    - `power.passes` and `power.note` from P8r;
    - `analysis_plan.script.sha256` from P9r;
    - the spec hashes in `extensions`;
    - the budget note (section 10);
    - `links.experiment_id` = `EXP-ICPERF-127dbd`;
    - `links.decisions` adding `DEC-20261106-c69de4`;
    - `links.hypothesis_ids` once allocated.
  - Then `validate` must report 0 findings.

**Freeze, once all of the above pass.** Copy the package to `research/ic_c1_deferred_guard_fp28_20261106/` in crypto. Then run `experiment_protocol.py freeze protocol.json --commit auto --filed-before-affected-data`. Then commit, push, open a ready PR with auto-merge, and merge.

**Approval blockers after the freeze (crypto-autoresearcher; section 18 gives the exact files).**
1. Allocate the two task ids, the run ids and the runbook session id. The hypothesis id is allocated before the freeze. Write `runbook.yaml`, run check R1, and commit the LINT OK receipt (section 18).
2. Commit the review_required specification, the goal lane and the goal record, and push them.
3. Write the four preview files.
4. `experiment_preapproval.py --packet` reports `approvable: true`.
5. Write the Coordinator approval DEC `DEC-20261106-c69de4`, citing the standing authorization, and archive it.
6. `newest_experiments.py --experiment EXP-ICPERF-127dbd --json` reports `execution_state: ready`.
7. The owner's go-ahead for spend (section 11).

**Superseded record (revision 3's list, kept for history):**

1. **P5r: residual collision check (Executor, free).**
   - Compute the holdout-P28 slugs, and the F6 slugs for the record:
     ```
     E=v6/bin-pinned/ecbench
     git -C v6/repo show origin/main:research/prime_base_oracle_tuning_20261029/sessions/holdout-P28/spec.json > checks/holdout-P28-spec.json
     $E plan --spec checks/holdout-P28-spec.json        # record slugs
     python3 checks/mkspec.py ...  # F6 seeds 2026110628904/905/906, or read the F6 spec from prefreeze-checks-v6.md, then $E plan
     ```
   - Then search the committed crypto tree at `origin/main` for the four C1 slugs and the eight C1 seeds. Use one `git grep` with a narrow pathspec, run on a blob-complete clone, or GitHub code search on `isogeny-labs/crypto`:
     ```
     git -C <full-clone> grep -n -F -e t10999-67500d78 -e tm6425-08e2435f -e t14969-7ca8e8a5 -e tm4717-89c926fe \
       -e 2026110628201 -e 2026110628202 -e 2026110628203 -e 2026110628204 \
       -e 2026110628211 -e 2026110628212 -e 2026110628213 -e 2026110628214 \
       origin/main -- research backlog benchmarks docs/ic
     ```
   - Pass condition: no match, apart from this study's own files.
2. **P8: power (Executor runs it; Coordinator reads it).**
   - The tool's `power` command checks only the p-value resolution of a rank test (2/(N+1) < alpha/m). It does not establish power to detect an effect (docs/experiment-protocol.md section 1), and C1 uses no rank test.
   - What P8 must supply is therefore a test-specific power calculation for the paired, workload-cluster ratio. It goes in `sample_size.power.note`, with `method: "other"` and `rank_test_n: null`. `passes` is set from that calculation.
   - Run `python3 car/tools/experiment_protocol.py power --help` only to confirm that no other form applies.
   - Then compute power for the paired workload-cluster ratio at 24 workloads × 8 rounds:
     - **variance input:** the per-workload SD of the log ratio, measured on `checks/dev/run2` (the `incumbent`→`candidate` and eager→deferred pairs, at w = 0, 0.1 and 1), with the v6 counted records' per-run SD at N 407 as a cross-check;
     - **effects:** 3 % and the projected 8.6 % at w = 0, and the projected values at w = 0.1 and 1.
   - Record the result in `prefreeze-checks.md`.
   - Rule, unchanged: if power at a 3 % effect is below 0.8 at any weight, `targets_per_curve` is raised before the freeze, and P1 and P6 are re-run on the new specs.
3. **P9: analysis script (Executor or design lane; Coordinator reviews).**
   - Write `research/ic_c1_deferred_guard_fp28_20261106/analyze-c1.sh`, implementing section 7.3 exactly. It must include:
     - parity rules 1–4 of section 5, over all 216 pairs, warm-ups included;
     - addressing arms by name and never by role;
     - skipping rho for `counters` and `phases[].native`;
     - weighted S from per-workload means, never the top-level `mean_s`;
     - U_w and L_w as max and min over `ci95` and `ci95_bootstrap`;
     - the verdict mapping of section 8;
     - exit 2 if the binary hash is not `73362eed…`.
   - Exercise it on the local `checks/dev/run1` and `run2` sessions, and on synthetic or stubbed inputs for each failure path:
     - a non-identical twin;
     - `pc-fb2x` with `ci95[0] ≤ 1`;
     - a parity violation, with Δ GAE greater than d and a mismatch in `relations_found`;
     - a censored pair;
     - `pairs` ≠ 192;
     - a wrong binary hash.
   - Record each expected verdict, then `sha256sum analyze-c1.sh`.
4. **P10: freeze package (main session or design lane).**
   - Copy `checks/spec-s{1,2}-final-order.json` to `spec-c1-s{1,2}.json` and record their sha256.
   - Build `launch/request-s{1,2}.json` (`isolab.eks.request/v1`, pinned `job.build`, 3,600 s). Check `jq -S -c .spec request-sK.json` against `jq -S -c . spec-c1-sK.json`.
   - Run `taskq/deploy/eks-karpenter/remote/validate-request` on each request.
   - Write `protocol.json` (`experiment.protocol/v1`) from this file, field by field as section 16 specifies, starting from `examples/experiment-protocol/ecbench-l2-smoke-subset/`. Then run `python3 ../crypto-autoresearcher/tools/experiment_protocol.py validate research/ic_c1_deferred_guard_fp28_20261106/protocol.json`, which must exit 0.
   - Build the preapproval packet. Run `python3 ../crypto-autoresearcher/tools/experiment_preapproval.py --packet <PACKET.json>`, which must report `approvable: true`. This is needed before the approval DEC, not before the protocol freeze itself.
5. **P14: canonical-document re-read. CLOSED in revision 3.** The gaps found are fixed in sections 5, 8, 9 and 11. P10 must now build `protocol.json` to section 16. P10 is not complete until `validate` exits 0 on a `protocol.json` that carries every section 16 field.

**Not blocking the freeze:** P13 (prices), which the launch card records before each launch; and the F6 slugs, which are informational.

## 13. Artifacts at the freeze (proposed paths, crypto repository)

All paths are under `research/ic_c1_deferred_guard_fp28_20261106/`:
- `PROTOCOL.md` (this draft, completed);
- `protocol.json` (`experiment.protocol/v1`);
- `spec-c1-s1.json` and `spec-c1-s2.json`;
- `analyze-c1.sh`;
- `launch/request-s1.json` and `launch/request-s2.json`, plus `launch/request-s1-r2.json` and `launch/request-s2-r2.json`, which are used only under the section 9 host-loss rule (revision 6);
- `prefreeze-checks.md` (the Executor's PREFREEZE-CHECKS.md Parts 1–4: P1–P14, P1r, P5r, P6r, P8r, P9r, P10r–P10r3 and R1);
- after the run: `eks-results-c1/{s1,s2}/` (gate files only), `reports/c1/` (analysis, research-visuals report, graph and PDF) and `reports/c1/validation/`.

The local development records in `checks/dev/` are not committed as study data. If they are kept at all, they go under `prefreeze/dev/` and are labelled non-study.

**Archives.** One snapshot archive for the freeze. One for the results before the independent review. One ledger archive after the review. Each stages only its declared paths and passes the post-commit verifier, and each is pushed to a branch whose PR to `main` is ready for review with auto-merge enabled.

**If C1-primary is supported,** the leaderboard regeneration and the `benchmarks/<YYYYMMDD>-c1-deferred-guard/` row land in the same PR as `VALIDATION.json`.

## 14. Scope and transfer

The verdict covers only:
- fp28;
- the four seeded `prime_search` curves of section 7.1;
- 60 public targets per curve (revision 4);
- the section 2 executable;
- the stated recipe and accounting.

No transfer is assumed to other sizes, other target sources, the route B walk deferred guard, other N values, later commits, or wall time. A side-effect on wall time is plausible, but it is neither claimed nor read.

**Roadmap caveat (DEC).** This is constant-factor engineering. Even full success leaves the distance to rho at about 19–21× (projection), and it must not displace exponent-targeting work.

## 15. Changes in revision 2 (pre-data; driven by P1–P7)

- **Roles (P2):**
  - the descriptive, parity and positive-control arms now carry `candidate`;
  - the arm named `candidate` is designated as the only C1-primary arm;
  - `incumbent-aa` is now `control`, placed after `rho-neg-aa`;
  - the final arm order is the order of `checks/spec-s1-final-order.json`.
- **F is 0.19 S (P3), not 0.25 S.** k_eager = 2.0 and set-up = N(N+1) were confirmed. N stays 304. The projected ratios moved by 0.001 at most.
- **Band charging (P3).** The band sums the five named counters, including `target_guard_probes`. That closes P11 from data.
- **Parity rule (P4), tightened:**
  - 0 ≤ Δ GAE ≤ deferred guard probes − relations (observed 0 or 1 per run);
  - Δ lookups = eager `repeated_targets_skipped`, exactly;
  - the full set of fields that must be equal;
  - the stream is identical by construction, and the guard never renews the stride.
- **Curves (P1, P5):** the slugs are recorded, and the residual check is ruled on: holdout-P28 and the repository search block the freeze; F6 does not.
- **Pairing (P6):** 192 measured pairs per comparison; parity uses 216 pairs, warm-ups included.
- **Counters (P7):** names and emission confirmed; rho is skipped for IC counters; the rule on top-level `mean_s` is restated.
- **Data seen:** the local development solves are added as design input, with no claim.
- **The C1-guard equivalence margin is unchanged ([0.995, 1.005]).** Its revision 1 rationale (about 41 additions) is replaced by the observed figure: at most 3.5 × 10⁻⁶ relative.
- **Expected runtime** updated from the local timing. The budget gate is unchanged.

## 16. Mapping onto `experiment.protocol/v1` (what `protocol.json` must carry; P10)

**How to read this section.** It is binding on whoever writes `protocol.json`.
- The schema is `car/schemas/experiment-protocol.v1.json`, which sets `additionalProperties: false` at every level. Anything not listed in the schema goes under `extensions`.
- The cross-checks are those of `car/tools/experiment_protocol.py validate`.
- A field marked (P8) or (P9) is filled from that check's output.

### 16.1 Top level

| Field | Value |
|---|---|
| `schema` | `"experiment.protocol/v1"` |
| `protocol_id` | `"IC-C1-DEFERRED-GUARD-FP28-20261106"` |
| `version` | 1. Draft revisions 1–3 precede the freeze, so they are not amendments |
| `created` | `"2026-11-06"` |
| `repository` | `"crypto"` |
| `study_dir` | `"research/ic_c1_deferred_guard_fp28_20261106"` |
| `authors` | `["coordinator", "executor (pre-freeze checks)"]`, roles only |
| `links` | `goal_id` GOAL-ICPERF-e6b6a4; `lead` LEAD-ICP-prime-ic-constant-engineering; `decisions` [DEC-20261106-1d23b6, DEC-20261106-c69de4 (reserved approval)]; `experiment_id` **EXP-ICPERF-127dbd**; `hypothesis_ids` [**H-ICPERF-4cb536**] (allocated; revision 6); `specs` [`spec-c1-s1.json`, `spec-c1-s2.json`]; `requests` [`launch/request-s1.json`, `launch/request-s2.json`], plus the two `-r2` requests (change C5); `narrative` `PROTOCOL.md`; `prefreeze_checks` `prefreeze-checks.md`; `prs` null until the freeze PR exists |
| `question` | "On fresh fp28 curves, does `prime-abscissa:size=304` + `mitm:negation_folded=1` + `targets=stride-renew-deferred-guard` have lower counted cost S than the walk incumbent (`size=407`, `targets=walk`) at every weight w ∈ {0, 0.1, 1} of the section 5a band, on the ea8e9ef7 executable?" |

### 16.2 `claims` (each in one class; a comparative claim decided only by `one_factor`, `factorial` or `absolute`)

| `id` | `class` | `decided_by` | Note |
|---|---|---|---|
| `C1-primary` | comparative | [`cmp-recipe`] | section 1 text; `if_supported` and `if_not_supported` from section 8 |
| `C1-guard` | comparative | [`cmp-guard-n407`, `cmp-guard-n304`] | component result only |
| `C-rho-gap` | comparative | [`cmp-rho-incumbent`, `cmp-rho-candidate`] | distance remaining at each w; a ranking only where the order holds across the band |
| `C-attribution` | out_of_scope | [] | "No sentence attributes C1-primary to guard placement, target source or N beyond the descriptive factorial table" |
| `C-wall` | out_of_scope | [] | no wall-time claim |

### 16.3 `hypotheses`: predictions with provenance

| Hypothesis | Prediction | `metric` | `direction` | `minimum_effect` |
|---|---|---|---|---|
| `H-C1` | P-w0 | `m-s-w0` | `lower` | "U_w < 1 in each session; projected 8.6 %" |
| `H-C1` | P-w01 | `m-s-w01` | `lower` | "U_w < 1 in each session; projected 10.6 %" |
| `H-C1` | P-w1 | `m-s-w1` | `lower` | "U_w < 1 in each session; projected 20.2 %" |
| `H-C1-guard` | — | `m-s-w0` | `equal_within` | "[0.995, 1.005]" |
| `H-C1-guard` | — | `m-s-w01`, `m-s-w1` | `lower` | "U_w < 1" |

- **`H-C1` `mechanism`:** the section 3 model.
- **Provenance of every prediction:** `written_at` 2026-11-06, `first_recorded_in` this draft, `sample_status: "out_of_sample"` (the test curves are fresh, P5/P5r), `role: "decisive"`.
- **`data_seen` entries (path and what was measured):**
  - the v6 counted `records.jsonl` (`1a7f3faa…`);
  - the holdout-P28 row of `docs/ic/LEADERBOARD.md`;
  - the F4 smoke;
  - `checks/dev/run1` and `checks/dev/run2`;
  - the XP-20261029 specs (specs only).
- **Optional `calibration`-role prediction:** `pc-fb2x` higher at every w.

### 16.4 `arms` (protocol roles; these differ from the ecbench spec roles)

| `id` | protocol `role` | ecbench spec role |
|---|---|---|
| `incumbent` | `baseline` | baseline |
| `candidate` | `candidate` | candidate |
| `walk-eager-n304`, `srenew-eager-n407`, `srenew-deferred-n407`, `srenew-eager-n304` | `candidate` (descriptive by this protocol) | candidate |
| `pc-fb2x` | `control_positive` | candidate |
| `rho-neg` | `reference` | reference |
| `rho-neg-aa` | `control_aa` | control |
| `incumbent-aa` | `control_aa` | control |

- **Why the roles may differ.** `from-ecbench` takes arm roles from the protocol for every arm the protocol names. The `ECBENCH_ROLES` fallback applies only to arms the protocol does not name. `lint_export` then compares export roles with protocol roles. A spec role and a protocol role that differ do not conflict.
- **`instrument` of every arm:**
  - `repo` `"isogeny-labs/crypto"`;
  - `commit` `"ea8e9ef71c3c82027dd9e01535302725519c3fca"`;
  - `build` null, because the image is prebuilt and pinned;
  - `binary_sha256` `"73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01"`;
  - `threads` 1. **P10 confirms** this from `plan --doc` or `host.json`;
  - `image_digest`: the full image reference from section 2.
- **`parameters`:** the expanded ecbench parameters of section 4.1.
- **`same_instrument_as: "incumbent"`** on every other arm. The checker then compares commit, binary and threads.

### 16.5 `controls`

- **`aa`:** `{present: true, comparison: "cmp-aa-incumbent", not_applicable_reason: null}`.
- **`positive`:** `{present: true, arm: "pc-fb2x", expected_detection: "incumbent→pc-fb2x ops.ratio_b_over_a > 1 and ops.ci95[0] > 1 at w = 0, 0.1 and 1 in each session", not_applicable_reason: null}`.
- **`negative`:** `{present: false, arm: null, null_population: null, not_applicable_reason: <the section 5 text>}`.

### 16.6 `variable_ledger`

**`independent` (name and levels):**
- `recipe`: [`walk/eager/N407`, `stride-renew/deferred/N304`];
- `guard_placement`: [`eager`, `deferred`];
- `target_source`: [`walk`, `stride-renew`];
- `factor_base_size`: [304, 407, 814];
- `method_family`: [`ic.pipeline`, `rho.negation`].

**`factors`** (each with `treatment` and `how`):

| Factor | Treatment | How |
|---|---|---|
| `instrument` | fixed | the section 2 pin |
| `oracle` | fixed | |
| `max_trials` | fixed | |
| `linalg_solver` | fixed | |
| `accounting` | fixed | as emitted at ea8e9ef7 |
| `curves` | randomized | seeded `prime_search` |
| `targets` | randomized | |
| `algorithm_seed` | randomized | shared across arms (P6) |
| `arm_label_and_position` | crossed | `alternate_measured` |
| `warmup` | fixed | 1 per workload, discarded |
| `host` | measured | it has no effect on counts |
| `memory_layout_environment` | measured | it has no effect on counts |
| `spot_preemption` | measured | rerun rule, section 9 |
| `analyst` | fixed | frozen script, section 7.3 |

**`comparisons`** (`differs_in` must name an `independent` or `factors` entry):

| `id` | `arms` | `differs_in` | `design` |
|---|---|---|---|
| `cmp-recipe` | [incumbent, candidate] | [`recipe`] | `one_factor` |
| `cmp-guard-n407` | [srenew-eager-n407, srenew-deferred-n407] | [`guard_placement`] | `one_factor` |
| `cmp-guard-n304` | [srenew-eager-n304, candidate] | [`guard_placement`] | `one_factor` |
| `cmp-factorial` | [the six IC cells] | [`guard_placement`, `target_source`, `factor_base_size`] | `factorial` |
| `cmp-positive` | [incumbent, pc-fb2x] | [`factor_base_size`] | `one_factor` |
| `cmp-aa-incumbent` | [incumbent, incumbent-aa] | [`arm_label_and_position`] | `aa` |
| `cmp-aa-rho` | [rho-neg, rho-neg-aa] | [`arm_label_and_position`] | `aa` |
| `cmp-rho-incumbent` | [rho-neg, incumbent] | [`method_family`] | `one_factor` |
| `cmp-rho-candidate` | [rho-neg, candidate] | [`method_family`] | `one_factor` |

- **`cmp-recipe` `reason` (a Coordinator judgement):** "The claim is recipe-level by DEC-20261106-1d23b6 `c1_route_ruling.route_A`. `recipe` bundles guard placement, target source and N. No sentence attributes the effect to a sub-factor (`C-attribution` is out_of_scope); attribution is descriptive, through `cmp-factorial`."
- **Fallback for `cmp-recipe`.** If the validator or a reviewer refuses `one_factor` on a bundle, the fallback is still pre-data and must not use `confounded`, because a `confounded` comparison decides nothing. It is: design `factorial`, with `differs_in` the three sub-factors, the six-cell arm list, and the decisive contrast incumbent → candidate named in `reason`.
- **`cmp-factorial` `reason`:** "a partial factorial; the walk/deferred cells do not exist on ea8e9ef7 (route B); it is descriptive and decides no claim".
- **Also set:** `crosses_map: false` (no claim moves the problem across a map), so no `transfer_obligations`; and `aa_not_applicable: null`.

### 16.7 `randomization`

- **`arm_order`:** `"interleaved"`. ecbench `alternate_measured` interleaves every arm in every round.
- **`seed_derivation`:** `{method: "harness_drawn", seeds: [the eight section 7.1 seeds], nonce: null, note: …}`. The `note` must say: ecbench 73362eed derives each curve from `workloads.curves[].seed` by `prime_search`, each target from `target_seed`, and each `algorithm_seed` from `measurement.seed` as a function of (workload, round) only (P6: 216 seeds per session, shared by all 10 arms). The bootstrap resample seed is 20261002.
- **`instance_sampling`:** `{rule: "120 workloads = 2 seeded fp28 prime_search curves × 60 public targets from target_seed, fixed before any data; the same workloads for every arm", independent_of_arm: true}`.
- **`blinding`:** `{analysis_labels_masked: false, how: …, unmasked_at: null}`. The `how` text: "Not masked. The metric is a deterministic count, so no analyst choice remains: `analyze-c1.sh` is frozen by sha256 before the data, and S1 is read for integrity only until S2 writes `job-exit.json` (section 9). The blind control is the reviewer's re-derivation of the primary ratio (section 9 step 4)."

### 16.8 `sample_size`

- **Counts:** `instances` **120** (revision 4; 144 at T = 72; per session, since each session has its own export), `replicates` 8, `warmups` 1.
- **`power`:** `{method: "other", alpha: 0.05, tests_in_family: 6, correction: "other", rank_test_n: null, min_attainable_two_sided_p: null, passes: (P8), note: (P8)}`.
  - The `note` must state: the intersection-union test over 3 weights × 2 sessions needs no multiplicity correction, because each one-sided test at 0.025 must reject (Berger, 1982); the effect sizes assumed; the variance source; and the power at 3 % and at the projected effect.
  - The checker rejects `correction: "none"` with `tests_in_family` > 1, so the value is `"other"`, explained in the note.
- **`preflight_runtime`:** `{fixed_seconds: 230, per_sample_seconds: 0.13, threads: 1, estimated_total_seconds: 511, measured_on: …}`.
  - `measured_on`: "local L0 non-study `checks/dev/run1`: 180 executions in 23 s; v6 qualification launch to launcher result 04:37:46Z–04:41:32Z".
  - The estimate is 230 + 2,160 × 0.13. P10 may replace it with a per-arm timing.
  - **Revision 4 values (superseded):** `{fixed_seconds: 230, per_sample_seconds: 0.177, threads: 1, estimated_total_seconds: 2142}`.
  - **Revision 5 values:** `{fixed_seconds: 230, per_sample_seconds: 0.1206, threads: 1, estimated_total_seconds: 1533}`, that is 230 + 10,800 × 0.1206 ≈ 1,532.5, rounded up.
    - `measured_on`: "local L0 non-study full-size T = 60 session, spec `checks/spec-dev-full-T60.json`, session `checks/dev/full60-job`: 10,800 executions, `ecbench run` 21 m 43 s (P9r); the 230 s of fixed time comes from the v6 qualification launch, 04:37:46Z–04:41:32Z".
    - Revision 6 reconciliation: `protocol.json` names the session but not the spec or the source of the fixed time. Change C3 in section 21 adds them.
    - The note states the margin to `timeout_seconds` 3600, which is 2.35×.

### 16.9 `metrics`

**`primary`** (aggregation `paired_ratio`; uncertainty `paired_bootstrap_ci95`):

| `id` | Kind | Formula | Note |
|---|---|---|---|
| `m-s-w0` | derived | `total_gae / sqrt(r)` | |
| `m-s-w01` | derived | `(total_gae + 0.1 × Σ priced native counters) / sqrt(r)` | the IC counters are the five of section 6; rho is `canonicalisations` |
| `m-s-w1` | derived | `(total_gae + 1 × Σ priced native counters) / sqrt(r)` | |

- **`definition`:** "charged GAE of one complete verified solve (factor base, oracle set-up, relations, linear algebra, verify), plus w times the priced uncharged native operations, divided by sqrt(r)". Byte counters are excluded.
- **`source`:** `records.jsonl` `cost.total_gae`, `phases[].native`, the workload r, and `ecbench compare --weight w` `ops`.
- **`resolution`:** "1 GAE (about 7×10⁻⁵ S)".
- **The interval:** a linearised-t interval over workloads is also used. It is recorded in `decision_rule.interval`.

**`secondary`** (counted, `per_row`, `none_exact`):
- `m-total-gae`;
- one metric per uncharged counter: `sqrt_solves`, `lookups`, `row_ops`, `legendre_symbols`, `target_guard_probes`, `canonicalisations`;
- `m-trials`, `m-relations`, `m-repeated-targets-skipped`;
- `m-parity`, kind `exact`, the section 5 rules.

### 16.10 `measurement`

- **`isolation_required: "none"`.** No clock is a metric. The ecbench spec still carries `isolation_required: "L2"`, because only the `run-ecbench` pre-check requires it. That difference is recorded in `extensions.ecbench_spec_isolation`. With `"none"`, `from-ecbench` writes `wall_valid: false` on every row, which is the intended outcome.
- **The rest:** `warmups` 1, `order: "alternate"`, `clock: "counted"`, `timed_primary_justification: null`.
- **`counters`:** every native counter name of section 6.
- **`record`:** CPU model, kernel, binary sha256, isolation level and blockers, node name, pod uid, job name, image digest, and threads.

### 16.11 `execution`

| Field | Value |
|---|---|
| `backend` | `"isolab-k8s"` |
| `queue` | `"EKS isolab namespace, Karpenter Spot c8a.2xlarge"` |
| `placement` | `{route: "taskq/deploy/eks-karpenter/remote/launch-remote", identity: "executor's existing AWS identity", checkpoint: "none", budget_note: "section 10 rolling gate"}` |
| `measurement_class` | `"measurement"` |
| `timeout_seconds` | **3600**, the job deadline. This is revision 5; revision 4's 5400 is refused by `validate-request`. The spec's per-execution `timeout_seconds` stays 600 |
| `checkpoint` | `{policy: "none", interval_seconds: null, grace_seconds: null, restore_validation: "none", after_restore: {wall_time: "invalid", counted_units: "valid"}}` |
| `retries` | `{infrastructure_only: true, max_attempts: 2}`, the `-r2` rule |

### 16.12 Stopping, invalidation, decision and analysis

- **`stopping_rules`** (each `preregistered: true`):
  - `fixed_n`: S1 once; S2 once, whatever S1 shows; 10,800 executions each (revision 4); no third session;
  - `budget`: the section 10 gate;
  - the section 9 deadline-hit rule, stored as kind `invalidation`, since the schema has no `deadline` kind (P10r3). A session at its 3,600 s deadline is an infrastructure outcome: it is not rerun, no result is read from it, and it returns to the Coordinator;
  - `invalidation`: section 7.3 steps 1 and 2, and the exit-code rule.
- **`invalidation_rules`:** the section 8 precedence items 1 and 2, written as strings.
- **`decision_rule`:**
  - `text`: section 8;
  - `comparison_ids`: [`cmp-recipe`];
  - `threshold`: "U_w < 1 at each w ∈ {0, 0.1, 1} in each of S1 and S2";
  - `statistic`: "ratio of mean counted S, candidate/incumbent, paired by algorithm_seed over 120 workloads × 8 rounds";
  - `interval`: "95 % CI, linearised t over workload clusters and cluster bootstrap (4000 resamples, seed 20261002); U_w = max of the upper bounds, L_w = min of the lower bounds";
  - `multiple_comparison_correction`: `"other"` (intersection-union);
  - `effect_size_measure`: "1 − ratio at each w, with its CI";
  - `verdict_vocabulary`: [`supported_within_scope`, `refuted_within_scope`, `not_supported_within_scope`, `inconclusive`, `single_unreplicated`, `invalid_measurement`, `implementation_defect`].
  - The C1-guard rule and the route B trigger go in `extensions`.
- **`analysis_plan`:** `{script: {path: "analyze-c1.sh", sha256: (P9)}, inputs, outputs, blinded: false, frozen_before_data: true}`.
- **`reporting`:** `{export_schema: "experiment.export/v1", review_schema: "experiment.review/v1", graphs, report_dir: "reports/c1"}`.

### 16.13 `inadmissible` (at least one entry; these are the entries)

- changing N, seeds, curves, targets, arms, weights, thresholds, margins or accounting after any C1 test-curve data;
- pricing `table_insert_attempts` or any other accounting edit inside the study;
- pooling with v6, holdout-P28, F4/F6, the local development runs or XP-20261029;
- a third session or any selective rerun;
- reading top-level `mean_s` as weighted S;
- using a guard-only or N-only ratio as C1-primary;
- attributing the recipe effect to a sub-factor;
- reading or reporting any wall field;
- treating a parity failure, timeout or crash as a negative result;
- `reject_scoped` on one unreplicated session.

### 16.14 The rest of the record

- **`literature`.** Every item has `provenance: "internal"`:
  - DEC-20261106-1d23b6;
  - PROTOCOL-V6 and the v6 validation report;
  - the holdout-P28 LEADERBOARD row;
  - `PREFREEZE-CHECKS.md`;
  - crypto AGENTS.md section 5a.

  The intersection-union reference (Berger, 1982) is `"recalled"`, unless it is retrieved first.
- **`amendments`:** `[]`.
- **`freeze`:** `{status: "draft", protocol_sha256: null, commit: null, frozen_at: null, filed_before_affected_data: null}` until `experiment_protocol.py freeze … --commit auto --filed-before-affected-data`.
- **`extensions`:**
  - the spec paths and sha256 values;
  - the request paths;
  - `ecbench_spec_isolation` ("L2", required by the launcher only);
  - the map from spec roles to protocol roles (16.4);
  - the parity rules 1–4;
  - the C1-guard decision rule and margin;
  - the route B trigger;
  - the budget gate, and the owner gate (DEC-20261106-1d23b6);
  - the closure obligations of section 8;
  - the curve slugs.

### 16.15 Export notes (after the run; not freeze blockers)

- There is one export per session (`instances` 120).
- `from-ecbench` maps `instance_id` to `workload_id` and `replicate` to `round`. It derives `wall_valid` from `isolation_required`, which is `"none"` here, so every row gets false. It copies the top-level `counters` and `field_ops`, but not the `phases[].native` counters. Those must be supplemented (section 9 step 4).
- `check-export --verify-files` must be clean before the review.

## 17. Changes in revision 3 (pre-data; P14)

- **Controls.** The A/A twins are now the A/A control (`control_aa`), not a "negative control". The negative control is declared not applicable, with its reason, as `docs/experiment-protocol.md` section 1 requires.
- **Closure obligations added to section 8:**
  - an `empirical_only` refutation note archived before any adverse decision;
  - an `obstruction` block and a `resource_check` for `reject_scoped` or any closure (coordinator.md, "Closure gate").
- **Promotion gates.** Section 8 states that the asymptotic promotion gates do not apply.
- **Section 9 step 4 now requires:**
  - one `experiment.export/v1` per session, with `check-export` and the `flow` table;
  - the uncharged counters supplemented in the export, because the adapter omits them;
  - a `review_plan` with a recorded prior, joints and attacks, and a blind re-derivation of the primary ratio;
  - `check_review_independence.py`.
- **Owner gate (section 11).** It is restated as a cloud-spend gate. It does not ask for protocol approval, which the standing authorization covers.
- **P8.** It is restated as a test-specific power calculation. The tool's `power` command checks only rank-test p-value resolution.
- **P14 closed. Section 16 added:** the field-by-field schema mapping, including:
  - the protocol roles that differ from the spec roles;
  - the bundled `recipe` factor for `cmp-recipe`, with a pre-data fallback;
  - `isolation_required: "none"` in the protocol against `"L2"` in the spec;
  - `correction: "other"` (intersection-union), because the checker rejects `"none"` for a family of tests;
  - the `inadmissible` list.

## 18. Preapproval packet: the files to write (crypto-autoresearcher; requirements read from `car/tools/experiment_preapproval.py`)

**Ids.**
- Allocated: `EXP-ICPERF-127dbd`, and `DEC-20261106-c69de4` (reserved approval).
- Still to allocate, each with `allocate_id.py --next … --date 20261106` and confirmed with `--check`:
  - the hypothesis is now allocated as `H-ICPERF-4cb536` (revision 6), so it is not allocated again;
  - an executor task `TASK-20261106-<a>`;
  - a Coordinator archive task `TASK-20261106-<b>`;
  - a lane/batch id `BATCH-…`, if the goal has no open C1 lane;
  - two run ids `RUN-<x>-<6hex>`, one per trial, unique with a random suffix;
  - a runbook session id `RBS-ICPERF-<6hex>` (revision 5, runbook route), as in EXP-ICPERF-283a70.
- The hypothesis id is now allocated **before the freeze** (section 12), not here.

**Committed and pushed at `source_ref` before the packet runs** (the tool reads them with `git show origin/<branch>:…`):

| Path | Content and the tool's requirement |
|---|---|
| `ledger/hypotheses/H-ICPERF-4cb536.yaml` | H-C1 (section 16.3), status `proposed`. The specification's `hypothesis_id` names it |
| `experiments/EXP-ICPERF-127dbd/specification.yaml` | the **canonical draft**: `status: review_required` (or `specified`), with no `approved_by`. Its bytes at `source_ref` must equal the working copy |
| `coordination/goals/GOAL-ICPERF-e6b6a4/lanes/BATCH-….lane.json` | `status: open`, `dispatch_queue_path` equal to the queue path below. The number of open lanes must not exceed `campaign_budget.max_concurrent` in the goal record |
| `ledger/goals/GOAL-ICPERF-e6b6a4/goal.yaml` | the committed goal record. Its bytes at `source_ref` must equal the working copy |
| every `input_bindings` path and every `source_sha256` / `input_sha256` path that is relative | committed with exactly those bytes |

**Preview files** (temporary paths in the packet's `preview` block; their sha256 values must later equal the canonical archive):

1. **`experiments/EXP-ICPERF-127dbd/specification.yaml`, the approved form.**
   - It binds the approval: `id: EXP-ICPERF-127dbd`, `status: approved`, `frozen: true`, `approved_by: DEC-20261106-c69de4`, `execution_authorized: true`, `goal_id: GOAL-ICPERF-e6b6a4`.
   - It must have non-empty `hypothesis_id`, `version`, `budget` (section 10), `controls` (section 5), `stopping_rules` and `invalidation_rules` (sections 9 and 16.12), `success_criterion` and `falsification_criterion` (section 8), and `metrics` (16.9).
   - `decision_impact: {on_positive: …, on_negative: …}`, and the two must differ. On a positive result: support plus validation gate, leaderboard and benchmarks row. On a negative result: close or weaken C1 route A at fp28, and evaluate the route B trigger.
   - `input_bindings`: `{path, sha256}` for `protocol.json` (the frozen file), `spec-c1-s{1,2}.json`, `launch/request-s{1,2}.json` and `analyze-c1.sh`.
   - None of the outcome fields `outcome`, `result`, `observed_result`, `conclusion` or `verdict` may be present.
2. **`experiments/EXP-ICPERF-127dbd/trial-plan.json`.**
   - Binding fields: `schema: crypto.autoresearch.trial_plan.v1`, `experiment_id`, `frozen: true`, `approved_by: DEC-20261106-c69de4`, `specification: experiments/EXP-ICPERF-127dbd/specification.yaml`, `specification_sha256` (the sha256 of preview 1), `task_id: TASK-…<a>`, and `queue` (the path in item 3).
   - `source_sha256` and `input_sha256`: non-empty maps from path to sha256.
   - `trials`: T1 = S1, and T2 = S2 with `depends_on: [T1]`. Each trial has `id`, `run_id`, `argv` and `check_argv`; each of the two argv arrays must name a bound source path. Each also has `artifacts`, which include `manifest.yaml` and `raw-result.json`, and a positive `memory_mb`. A watchdog of 4,500 s is optional, with a reason: 3,600 s deadline + 180 s CodeBuild slack + about 720 s for provisioning and the result download (revision 5).
   - The archive will contain `experiments/EXP-ICPERF-127dbd/runs/<RUN>/` holding the supervisor's generated files (`launch.json`, `execution-receipt.json`, `command.txt`, `environment.json`, and the stdout, stderr, check-stdout and check-stderr logs) plus the declared artifacts.
3. **`coordination/goals/GOAL-ICPERF-e6b6a4/batches/BATCH-…/queue.json`.**
   - `schema: crypto.autoresearch.dispatch_queue.v1`, `goal_id: GOAL-ICPERF-e6b6a4`.
   - **Executor task:** `id` TASK-<a>, `role: executor`. Its embedded `handoff.execution` is `{mode: "experiments", kind: "run", experiment_id, trial_plan: experiments/EXP-ICPERF-127dbd/trial-plan.json, trial_plan_sha256 (of preview 2), approved_by: DEC-20261106-c69de4}`. `archived_by` is TASK-<b>. `artifact_paths` covers every required run path.
   - **Coordinator archive task:** `id` TASK-<b>, `role: coordinator`, `archive.source_task_ids: [TASK-<a>]`, and a `read_scope` covering those paths.
   - It must pass `research_dispatch.validate_queue`.
4. **`ledger/handoffs/TASK-…<a>.yaml`.**
   - `id`, `role: executor`, and the identical `execution` binding and `archived_by`.
   - Objective, constraints, deliverables, completion gate and budget, as the handoff template requires.
   - For an implementation route, `inference.policy: executor-implementation`.

**The packet** (`crypto.autoresearch.preapproval_packet.v1`):
- the ids above;
- the four canonical paths and the four preview paths;
- `source_ref: origin/<pushed branch>`;
- `lane_id`;
- `execution_route`, chosen by one of the two rules below.

**Coordinator rulings for these files:**
- **Execution route (revision 5 ruling): `kind: "runbook"`.**
  - *Why.* The Executor found no EKS-routed `EXP-ICPERF-*` experiment for either route kind (PREFREEZE-CHECKS.md Part 3). The only ICPERF precedent is the runbook route: EXP-ICPERF-283a70 and EXP-ICPERF-284bc4, each with a committed `runbook.yaml`, a `runbook-lint.txt` LINT OK receipt, `executor_tier: mechanical` and `approval.mode: decision_record`, approved by DEC-20261029-0e40f3 and DEC-20261029-ca33b3, and checkpointed in `ledger/goals/GOAL-ICPERF-e6b6a4/checkpoints/TASK-20261029-0a9023.yaml`.
    - The run itself needs no inference. Each trial launches one frozen request, waits, downloads, and checks hashes. That is exactly the frozen-card-to-executor-mechanical tier, and a lint receipt makes the card checkable before approval.
    - `implementation` would put an inference-capable executor in charge of a paid launch for no benefit.
  - *Files (experiments/EXP-ICPERF-127dbd/, crypto-autoresearcher; written by the design lane, not the Coordinator):*
    - `runbook.yaml`, schema `crypto.autoresearch.runbook.v1`, `executor_tier: mechanical`, `approval.mode: decision_record`. It contains:
      - **`binds`** (path + sha256): `specification.yaml`, `trial-plan.json`, the read-only `inputs/` copies of `protocol.json`, `spec-c1-s{1,2}.json`, `request-s{1,2}.json` and `analyze-c1.sh`, and `implementation/launch_eks.sh` and `implementation/check_job.py`.
      - **`spec_bindings`:** each frozen request and spec sha256, the job names, `seconds` 3600, the image digest, source `ea8e9ef7…`, the ecbench sha256 `73362eed…`, and the runbook session id.
      - **`environment`:** the programs used (bash, python3, aws, jq, git), with no secrets in the file. The executor's existing AWS identity is read from the environment.
      - **`S01-probe`** (command): records `aws sts get-caller-identity`, confirms the crypto checkout HEAD is the pinned source, checks the sha256 of the local pinned ecbench, and recomputes every bound request sha256. If any value differs, exit nonzero, which stops the session.
      - **`S02-trials`** (`engine: supervisor`): runs T1 (S1), then T2 (S2) with `depends_on: [T1]`.
        - `timeout_seconds` 4,500 per trial; `claim.ttl_minutes` ≥ 90.
        - Each `argv` is `implementation/launch_eks.sh <bound request>`. It runs `taskq/deploy/eks-karpenter/remote/launch-remote` from the pinned crypto checkout, downloads the job prefix, and writes `raw-result.json` and `manifest.yaml`.
        - Each `check_argv` is `implementation/check_job.py`. It verifies `SHA256SUMS` and `job-exit.json`, the image and source identity, the ecbench sha256, 10,800 records, and the job name.
        - The validity assertions pin those fields, `attack_claimed: false`, `experiment_id`, `hypothesis_id`, `task_id` and `approved_by`.
      - **`on_failure`:** a named infrastructure class (Spot interruption, provisioning or image-pull failure, launcher error before the job starts) → `retry`, `max_attempts: 2`, which is the `-r2` rule. `timed_out` or a job at its deadline → `stop`, which is the deadline-hit rule of section 9. Anything else → `stop`. `on_invalid: stop`.
    - `runbook-lint.txt`: the `python3 tools/runbook.py lint experiments/EXP-ICPERF-127dbd/runbook.yaml` output, `LINT OK … sha256 …`, committed with its own sha256 recorded in the packet's `execution_route`.
    - `implementation/launch_eks.sh` and `implementation/check_job.py`: thin wrappers, with no study logic. Analysis stays in the frozen `analyze-c1.sh` and runs locally after the archive (section 9 step 4).
  - *Pre-packet check R1 (design lane, free, no launch).* Before the packet is built, read `tools/runbook.py` (lint and supervisor) and lint a draft `runbook.yaml`. Confirm each of the following:
    - (a) a trial may run 4,500 s, with a claim TTL long enough to cover it;
    - (b) the step may use the network and the ambient AWS identity;
    - (c) `replay` can be omitted, or set to a mode that does **not** re-execute a trial. Re-executing a trial would relaunch a paid EKS job. The study's replay is the local `ecbench verify --replay-all` in section 7.3;
    - (d) `controls` may be empty, or may name in-trial arms, since the controls here are arms inside each job and not separate trials;
    - (e) `on_failure` can stop on `timed_out` while retrying named infrastructure classes.
  - *Fallback, ruled now.* If lint or the supervisor refuses any of R1 (a)–(e):
    - Use `kind: "implementation"`, with the reason "EKS launch of the frozen requests via launch-remote; runbook lint refuses <item>; card relayed verbatim from the experiment director", and `inference.policy: executor-implementation` in the handoff.
    - Record the lint output in the approval DEC's evidence.
    - Neither `tools/runbook.py` nor the shared launcher may be changed to make this study pass. That is out of scope here, and needs its own task.
    - No further Coordinator ruling is needed for the switch. Under both kinds the trial argv, check_argv, budget gate and stopping rules are the same.
  - *Budget inside the runbook.* Every launch that `max_attempts: 2` permits passes the section 10 gate even at hard bound (worst case USD 3.72 ≤ 5.00). So the gate cannot fail inside the runbook's limits. The card still records each gate line before each launch, and the director checks those lines when it assesses the run.

- **Execution route, revision 6 ruling (R1 results, PREFREEZE-CHECKS.md Part 4): keep `kind: "runbook"`, option (i).** This supersedes the revision 5 `on_failure`, the S02-only layout and the "analysis after the archive" line above, wherever they differ.
  - **Why the fallback is not triggered.** R1 (d) read "`controls` may be empty, *or* may name in-trial arms". Lint accepts the first form (`controls: []` gives `control_validity: not_applicable`) and refuses only the second. The fallback applies when lint refuses an item, and it does not refuse this one. The other items pass: (a), (b) in the tool, (c) as `exempt`, and (e) in lint.
  - **Why (i) over (ii).**
    - *New code is the same.* Both routes need the same two thin wrappers, `launch_eks.sh` and `check_job.py`. Option (i) adds only YAML and a lint receipt.
    - *The gates stay exactly as frozen under (i).* Every control and integrity gate already exists once, in the frozen `analyze-c1.sh` (sha256 `66aa2ffd…`), exercised by 29 stubs and a full-size session. Option (i) runs that script unchanged, as a bound verification step. Option (ii) would leave the gate run to an inference-capable executor, and would give no lint receipt before approval.
    - *Re-implementation is ruled out.* Stating the A/A or `pc-fb2x` gates afresh in `check_job.py` would be a second implementation that could drift from the frozen script. So it is ruled out.
  - **Layout (replaces S02 alone):**
    - **`S01-probe`** as above. Calling `aws sts get-caller-identity` also tests runtime network access and the ambient identity at no cost. R1 (b) was untested at runtime.
      - No credential-named variable is set. `AWS_DEFAULT_REGION` may be.
    - **`S02-trials`** as above: transport and identity integrity only, the section 9 step 2 set.
      - `controls: []`, with the reason in the step's `explain` text, or in a reason field if lint offers one: "C1's controls are arms inside each ecbench job (`incumbent-aa`, `rho-neg-aa`, `pc-fb2x`; protocol section 5), not plan trials. Their gates are evaluated by the frozen analyze-c1.sh in step S03."
      - `replay: {mode: exempt, exempt_reason: "re-executing a trial relaunches a paid EKS job; the study replay is ecbench verify --replay-all inside analyze-c1.sh (S03, section 7.3 step 1)"}`.
    - **`S03-verify`** (`kind: command`; it runs after S02, so after S2 has written `job-exit.json`; `timeout_seconds` 5,400; local; no spend).
      - It runs the bound `analyze-c1.sh` on the two downloaded sessions, exactly as section 7.3 prescribes, and writes into its step output directory.
      - **Validity assertions cover only the gate fields:**
        - each session's integrity gates: replay, audit, record count, invalid outcomes, spec and build equality, and binary hash;
        - each session's control gates: A/A exactness for both twins, and the `pc-fb2x` gate;
        - the parity rules 1–4 result.
      - The design lane takes the exact JSON pointers from the script's existing output (`checks/p9/out-real60/`). They must not be renamed.
      - **No assertion may name the verdict, U_w, L_w or any pass/fail of a claim.** A scientific outcome must never become a runbook validity failure.
      - A gate failure therefore makes S03 invalid. Its outputs are kept, and they are read as `invalid_measurement` or `implementation_defect` under section 8, never as a negative result.
      - `replay: {mode: exempt, exempt_reason: "deterministic frozen script over sealed records; the independent validator re-derives the ratio (section 9 step 4)"}`.
    - Section 9 step 4 (export, `review_plan`, validator, review) still runs after the snapshot archive, outside the runbook. `analyze-c1.sh` is re-run there only by the validator, who uses its own derivation.
  - **`on_failure` (replaces the revision 5 rule): stop on every failure, with no automatic retry.**
    - The first rule is `{when: {timed_out: true}, class: infrastructure_error, action: stop}`. Every other failure also stops.
    - The reasons are three:
      - On a `trials` step the exit code is the supervisor's (2, 3 or 130), not the job's 42 or 43, so no exit-code rule can single out a host loss.
      - Classifying a failure as infrastructure is the director's judgement (section 9, "Host loss"), not a mechanical rule.
      - How a retry interacts with write-once unit directories is untested.
    - A `-r2` therefore runs as a new runbook session on the frozen `-r2` request, after a Coordinator DEC (section 9). Each runbook session makes at most two launches, so the section 10 gate holds at hard bound.
  - **Exit codes inside the wrappers.**
    - `launch_eks.sh` exits 0 only when the launcher has returned, `job-exit.json` and `SHA256SUMS` are downloaded, and the job's exit code is 0, 42 or 43.
    - Otherwise it exits nonzero and writes `raw-result.json` with `outcome_class` ∈ {`infrastructure_candidate`, `deadline_hit`, `other`} and the launcher's evidence, for the director to read.
    - The section 9 acceptance of 0, 42 and 43 is judged from `job-exit.json` by `check_job.py` and `analyze-c1.sh`, never from the supervisor's exit code.
  - **Lint constraints the author must meet (Part 4):**
    - the queue's executor task lists every run file, including the supervisor's generated files, in `artifact_paths`;
    - its `write_scope` covers `experiments/EXP-ICPERF-127dbd/runbook-sessions`;
    - no `VAGUE` words in operator text;
    - `engine: supervisor`;
    - the approval DEC names `runbook_sha256`.
  - **If lint refuses this layout** (for example an exempt replay on a command step, or S03's assertions), the revision 5 fallback applies unchanged, under the same "no shared tool edits" rule.
- **Cross-repository binding.** The frozen study files are canonical in crypto and must not be duplicated as editable copies. The specification binds them in one of two ways:
  - (a) by absolute path in a clean crypto checkout at the freeze merge commit, which the tool checks by local digest; or
  - (b) as committed byte-identical, read-only copies under `experiments/EXP-ICPERF-127dbd/inputs/`, bound by relative path.

  The crypto merge commit is recorded in the specification either way. **(b) is preferred**, because a fresh session can verify it at `source_ref`.
- **Trial argv.** Each `argv` runs the bound wrapper `implementation/launch_eks.sh`, which calls `launch-remote` on the frozen request (revision 5; the request is a bound input). Each `check_argv` runs the bound `implementation/check_job.py` over that job's downloaded `SHA256SUMS` and `job-exit.json`. Both scripts are implementation work for the design lane; the Coordinator does not write them.
- **Approval DEC.** `DEC-20261106-c69de4` is written only after the packet reports `approvable: true`. It cites the standing user authorization and this protocol's frozen sha256. It must pass "Approval is bounded by execution". It is followed by `newest_experiments.py --experiment EXP-ICPERF-127dbd --json` reporting `execution_state: ready`, and by the owner's spend go-ahead.

## 19. Changes in revision 4 (pre-data)

- **T raised from 12 to 60 targets per curve, under the pre-registered P8 rule**, with a fallback of 72. The new counts are 10,800 executions, 960 measured pairs, 1,080 parity pairs and 120 workloads. They are applied in sections 5, 6, 7, 14 and 16.
- **The deadline is now 5,400 s, and V is removed.** The gate is HB USD 1.14662 per job; the worst case is S1, S1-r2, S2 and S2-r2 at USD 4.59. Expected spend is about USD 0.8–1.2. *(The deadline and budget are superseded by revision 5, section 20. The removal of V stands.)*
- **The job names are fixed** as `ic-c1-dg-s{1,2}-20261106`.
- **Section 7.3 integrity:** invalid outcomes are computed from the records and from `l2-gate.json`, because `session.json` carries `invalid_outcomes: null` (P9 note).
- **Section 12** records the Part 2 results and lists the rework P1r, P6r, P8r, P9r and P10r.
- **Section 16 links:** `EXP-ICPERF-127dbd` and `DEC-20261106-c69de4`.
- **Section 18 added:** the preapproval packet files, their paths, and the rulings on execution route, cross-repository binding, trial argv and the approval DEC.

## 20. Changes in revision 5 (pre-data; driven by PREFREEZE-CHECKS.md Part 3)

No C1 test-curve data exists. Every change below is driven by infrastructure limits or by non-study measurements. None moves N, T, the seeds, the arms, the weights or the thresholds.

- **Deadline 3,600 s** (ruling A). `validate-request` caps it at 3,600, and the shared validator stays unchanged.
  - Runtime is projected from the measured 0.1206 s per execution at about 1,530 s, a 2.35× margin. CodeBuild waits 3,780 s, inside its 6,000 s limit.
  - Applied in sections 9 and 16.11.
- **Deadline-hit rule added** (section 9). A session that reaches its deadline is an infrastructure outcome. It is not rerun as `-r2`, and it returns to the Coordinator for a pre-data `protocol_amendment`.
- **Budget recomputed** (section 10). HB is USD 0.93108. The gate lines are S1 1.86, S1-r2 ≤ 2.79, S2 ≤ 2.79 and S2-r2 ≤ 3.72; a fifth launch is forbidden by the stopping rules. Expected spend is about USD 0.6–1.0, about 0.38 per job.
- **`preflight_runtime`** is updated to per-sample 0.1206 s and an estimated 1,533 s in total (section 16.8).
- **Execution route `runbook`** (ruling B, section 18).
  - The runbook layout is specified, using `launch_eks.sh` and `check_job.py` wrappers.
  - Pre-packet check R1 (a)–(e) is added, covering trial length, network and identity, a replay that does not relaunch, in-trial controls, and stop-on-timeout.
  - The `implementation` fallback is ruled in advance, and no shared tool may be edited to make the study pass.
  - The runbook session id is added to the ids to allocate.
- **Section 12.**
  - The Part 3 results are recorded: P1r, P6r, P8r and P9r pass; P10r fails only on `seconds`.
  - The joint two-session power of about 0.70 is stated and accepted under the per-session rule.
  - New freeze blockers: P10r2 (requests at 3,600 s, `validate-request`, new sha values), P10r3 (`protocol.json` fields, `validate`, new sha), the hypothesis id allocated before the freeze, and draft reconciliation against `protocol.json`.

## 21. Changes in revision 6 (pre-data; reconciliation against `freeze/protocol.json` `5d83e72e…`, and the R1 route ruling)

No C1 test-curve data exists. Nothing below moves N, T, the seeds, the arms, the weights, the thresholds or the decision mapping.

### 21.1 Where the draft was wrong, and is now fixed to agree with `protocol.json`

- **Section 12, P8r row.** Revision 5 said a one-session failure maps to `not_supported_within_scope`. Section 8 and `decision_rule.text` both say pass+fail or fail+pass is `inconclusive`, and the row is corrected.
- **Section 16.12.** The deadline-hit rule is kind `invalidation`, not `deadline`, because the schema has no such kind.
- **Section 16.2 `links`.** It now has `hypothesis_ids` [H-ICPERF-4cb536], `requests`, `narrative` and `prefreeze_checks`. Section 18 names the hypothesis file `ledger/hypotheses/H-ICPERF-4cb536.yaml`.
- **Section 7 seed table.** It now shows the T = 60 spec ids, `ECS1h3229b941623b` and `ECS1h1e755d9669a9`. The T = 12 ids are marked void.
- **Section 0 data-seen table.** It adds the power runs and the two full-size local sessions, which `protocol.json` partly lacks; see C2.
- **Section 13.** The `prefreeze-checks.md` description now covers Parts 1–4.

The rest agrees between the two documents and needed no change: claims, hypotheses, the 10 arms with their roles and instruments, controls, the variable ledger and comparisons, seeds, sample size, power, metrics, measurement, execution, stopping and invalidation rules, decision rule, analysis plan, reporting, `inadmissible`, `literature`, `extensions` gates and slugs, and `freeze.status` draft.

### 21.2 Changes `protocol.json` needs before the freeze (P10r5; then `validate`, 0 findings)

| # | Field | Now | Change to |
|---|---|---|---|
| C1 | `extensions.spec_ids` | `{S1: "ECS1hc027a4dab62e", S2: "ECS1hab11b5206d1d"}`, the T = 12 ids | `{S1: "ECS1h3229b941623b", S2: "ECS1h1e755d9669a9"}` (P1r). **This is a real defect.** The sha values in `extensions.specs` are already the T = 60 ones, so the file disagreed with itself |
| C2 | `hypotheses[*].predictions[*].provenance.data_seen`, all seven predictions | no entry for the full-size local sessions | Append `{path: "next-candidate/checks/dev/full-job (T = 12, 2,160 executions) and checks/dev/full60-job (spec checks/spec-dev-full-T60.json, T = 60, 10,800 executions); local L0, non-study curve seeds 2026110699501/502", measured: "runtime 0.177 and 0.1206 s per execution; verify --replay-all; at T = 60 analyze-c1.sh gave candidate/incumbent U_w 0.914 / 0.897 / 0.818 and L_w 0.886 / 0.869 / 0.786, with 0 parity failures; script exercise only; no C1 test curve"}` |
| C3 | `sample_size.preflight_runtime.measured_on` | names `checks/dev/full60-job` only | Also name the spec `checks/spec-dev-full-T60.json`, and the source of the 230 s fixed time, "v6 qualification launch 04:37:46Z–04:41:32Z" |
| C4 | `literature[3].claim` (`prefreeze-checks.md`) | "P1-P11 pre-freeze check results" | "pre-freeze checks P1–P14 and P1r, P5r, P6r, P8r, P9r, P10r–P10r5, R1 (PREFREEZE-CHECKS.md Parts 1–4)" |
| C5 | `links.requests`, `extensions.requests`, `invalidation_rules[2]` | first-attempt requests only | Add `launch/request-s1-r2.json` and `launch/request-s2-r2.json` with their sha256 values (P10r4). Append to `invalidation_rules[2]`: "; a rerun uses the frozen launch/request-sK-r2.json, identical to request-sK.json except job.job_name, as a new runbook session after a Coordinator decision" |
| C6 | `randomization.blinding.how` | "S1 is read for integrity only until S2 writes job-exit.json" | "Before S2 writes job-exit.json, S1 is read only for transport and identity integrity (the check_job.py set, PROTOCOL section 9 step 2); audit, replay, invalid outcomes, A/A exactness, the pc-fb2x gate and parity run after S2 in the frozen analyze-c1.sh (runbook step S03)" |
| C7 | `extensions.execution_route` (new) | absent | `{kind: "runbook", repo: "crypto-autoresearcher", runbook: "experiments/EXP-ICPERF-127dbd/runbook.yaml", steps: ["S01-probe", "S02-trials", "S03-verify"], controls: "[] (in-job arms; gates in analyze-c1.sh at S03)", replay: "exempt (no paid relaunch)", on_failure: "stop on every failure; -r2 is a new runbook session", fallback: "implementation, if lint refuses the layout, with no shared-tool edits"}`. This is informational, and no gate reads it |

**Not changed, after checking:**
- `execution.retries` `{infrastructure_only: true, max_attempts: 2}` stays. It counts attempts per session, whichever runbook session carries the attempt.
- `measurement.order` `"alternate"` stays, with the spec's `alternate_measured`, as section 16.10 already records.
- The exit-code stopping rule (0, 42 or 43) stays. It is judged from `job-exit.json` (section 18), so the supervisor's own exit codes do not affect it.

### 21.3 Route (section 18)

- **Ruling:** keep `runbook`, option (i).
- **Fallback not triggered.** R1 (d) had an "empty" alternative, and lint accepts that form.
- **Code.** It needs the same new code as option (ii): two thin wrappers.
- **Gates.** These are kept exactly, because the gates are only ever evaluated by the frozen `analyze-c1.sh`, run as the bound step S03 after S2. Validity assertions cover gate fields only, never the verdict.
- **Failures.** Every failure stops. There is no automatic retry, because:
  - the supervisor's exit codes 2, 3 and 130 cannot single out a host loss;
  - the infrastructure call is the director's judgement;
  - retry against write-once unit directories is untested.
- **`-r2`.** A rerun runs as a new runbook session on a frozen `-r2` request, after a Coordinator DEC.
- **Change to section 9.** Before S2, strictly less is read than in revision 5. The cost is that a defect visible only in A/A or audit is first seen after S2 has been spent, at most about USD 0.4 at the expected rate.
