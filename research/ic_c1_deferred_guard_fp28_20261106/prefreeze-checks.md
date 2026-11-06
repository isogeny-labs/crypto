# C1 route A: pre-freeze checks P1-P7, P4 (local, free; facts only)

Executor, 2026-11-06. Nothing here is study evidence; no hypothesis, lead or goal status is touched. No EKS, no AWS, no remote host. Local solves ran on this container with `--cpus none --allow-busy` (L0) on **non-study** curves (seeds 2026110699101, 2026110699201, 2026110699202; target seeds ...99103, ...99203).

Work dir: `/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/next-candidate/checks/` (below `checks/`). `E=../../v6/bin-pinned/ecbench`.

## 0. Identity checks

```
$ sha256sum v6/bin-pinned/ecbench v6/counted-untrusted/session/records.jsonl
73362eed4165d6db20fe1d398baa8fabefa2497aff6e78dab9ed8f31af31dd01  ecbench                 (matches the pin)
1a7f3faad6f5ebada35f4546d150e0f35aadf0fae3df9d84234832851b8ec00b  records.jsonl           (matches)
```
Records were read with `/usr/bin/python3 -I`, scripts in `checks/`, after copying nothing into the untrusted dir (`checks/v6copy/session` is a copy used for `band`).

Spec files used (built by `checks/mkspec.py`, v6 counted spec as template, 12 targets per curve, 2 curves, rounds 8, warmup 1, `alternate_measured`, `isolation_required: L2`, `timeout_seconds` 600):
- S1: `checks/spec-s1-final-order.json` (seeds 2026110628201/202, target 203, measurement 204)
- S2: `checks/spec-s2-final-order.json` (seeds ...211/212, target 213, measurement 214)
- Arm order (final): incumbent, candidate, walk-eager-n304, srenew-eager-n407, srenew-deferred-n407, srenew-eager-n304, pc-fb2x, rho-neg, rho-neg-aa, incumbent-aa.

## P1. `ecbench plan` validity: PASS

```
$ $E plan --spec checks/spec-s1-final-order.json     -> exit 0
spec ECS1hc027a4dab62e (...): 2160 executions
$ $E plan --spec checks/spec-s2-final-order.json     -> exit 0
spec ECS1hab11b5206d1d (...): 2160 executions
```
All 10 arm strings are valid: `prime-abscissa:size=407|304|814`, `targets=walk|stride-renew|stride-renew-deferred-guard`, `rho.negation`. The ten method ids:

| arm | method id (ECM) |
|---|---|
| incumbent, incumbent-aa | ECM1h1729b9b88605 (same id: params identical) |
| candidate | ECM1h871b51ac7b8a |
| walk-eager-n304 | ECM1h5caae1adafba |
| srenew-eager-n407 | ECM1hfb08123d17dc |
| srenew-deferred-n407 | ECM1h06aa2f4e545c |
| srenew-eager-n304 | ECM1h0c5d4fe610d3 |
| pc-fb2x | ECM1h5d5c4637a846 |
| rho-neg, rho-neg-aa | ECM1hefea4e0ebe79 |

**Defaults expanded by plan (ic.pipeline arms):** `linalg=incremental-gauss`, `solver=""`, `solver_budget_seconds="0"`; params given: `factor_base`, `oracle=mitm:negation_folded=1`, `max_trials=26479085`, `targets`. **rho.negation:** `cap_multiple="64"`. `stride-renew` and `stride-renew-deferred-guard` expand with **no extra parameters** (nothing else appears in the arm's params). Note: `ecbench methods` documents `targets` as "random, walk, or walk-prime-fast" only; the parser (`stages.rs:675`) accepts `stride-renew-deferred-guard` and its error text lists `stride, stride-renew, stride-renew-deferred-guard`. The help text is stale; the strings work.

Slugs and r (S1): `icv1-fp28-t10999-67500d78` r=2^27.72 (12 workloads), `icv1-fp28-tm6425-08e2435f` r=2^27.50 (12). (S2): `icv1-fp28-t14969-7ca8e8a5` r=2^27.29, `icv1-fp28-tm4717-89c926fe` r=2^27.62. All `registered=false`; plan notes "24 slug(s) are not in docs/curves/registry.json".
Counts: 2 curves x 12 targets x 9 rounds (1 warmup + 8) x 10 arms = 2160 executions. OK.

## P2. Roles: PASS, with one ordering consequence

```
$ $E plan --spec checks/spec-badrole.json     (arm role "descriptive")
ecbench: spec: unknown variant `descriptive`, expected one of `reference`, `baseline`, `candidate`, `control` at line 1 column 1029     exit 2
```
- Role vocabulary (`spec.rs:29`): exactly `reference`, `baseline`, `candidate`, `control`. **`descriptive` is invalid** (exit 2). Draft arms 4-8 must be `candidate` (or `control`).
- Several `candidate` arms: accepted (plan exit 0 with 7 `candidate` arms). Only unique-name (`[A-Za-z0-9._-]+`) is enforced (`spec.rs:288,296`). The old XP-20261029 specs also use several `control` arms for non-twin arms, and plan accepts them.
- The A/A twin `incumbent-aa` can only take `control` (or `candidate`/`baseline`; there is no "twin" role). Roles are labels; no cross-check that a control repeats another arm's method.
- `run-ecbench` (`taskq/deploy/eks-karpenter/run-ecbench` @ea8e9ef7, lines 70-73 and 138-143) pre-check, run verbatim with `jq -e` on both spec files: `true`, exit 0 (needs schema, `isolation_required == "L2"`, rounds >= 5, warmup >= 1, at least one `reference`, at least one `control`).
- **Pairing:** `reference=$(first role==reference)`, `control=$(first role==control)` in spec order. With the order above: `reference=rho-neg control=rho-neg-aa` (checked with jq for S1 and S2). If `incumbent-aa` (role control) precedes `rho-neg-aa`, the pair becomes `rho-neg` against `incumbent-aa` (verified on `spec-s1-try1.json`: `reference=rho-neg control=incumbent-aa`). So: **`incumbent-aa` must be after `rho-neg-aa`**, as the draft's fallback says; I used that order.
- The launcher's admission step then reads `.wall.status=="admitted"` (exit 42/43 handling unchanged). Not exercised here (no L2 run).

## P3. Phase decomposition of v6 counted records: PASS for set-up and trial counts; two small numeric differences from the draft

Source: `records.jsonl` (576 records = 4 arms x 144 incl. 16 warmup each; 128 measured per arm). Scripts `checks/p3.py`, `p3b.py`; output `checks/p3-out.txt`.

**Per-run means, arm `incumbent` (measured, n=128; N=407; the v6 `candidate`, `layout=lines`, has the identical counted numbers):**

| phase | charged adds | doubles | scalar mults | charged GAE | share of GAE | uncharged native counters (per run) |
|---|---:|---:|---:|---:|---:|---|
| factor_base | 0 | 0 | 0 | 0 | 0 | legendre_symbols 834.5; sqrt_solves 407.0 (work_native: index_inserts 1221, index_sort_word_ops 4070) |
| oracle_setup | 166,056 | 0 | 0 | 166,056.0 | 54.01 % | work_native: table_insert_attempts 165,649; lookups 407 (in `work_native`, NOT in `native`, not in the band sum) |
| relations (trials) | 139,697.5 | 1,679.8 | 65.2 | 141,377.3 | 45.98 % | lookups 138,582.6; target_guard_probes 138,624.0; trials 138,624.0; relations 209.3; repeated_targets_skipped 41.4; walk_steps 138,581.6 |
| linear_algebra | 0 | 0 | 0 | 0 | 0 | row_ops 2,515.6 (rows 209.3, columns 408) |
| verify | 12.9 | 25.8 | 1.0 | 38.7 | 0.01 % | none |
| **total** | | | | **307,472.0** (= sum of phase GAE in all 128 records) | | native (5 priced counters) 280,963.6 |

- Set-up charge = N(N+1): 407x408 = 166,056 in every record (observed set `{166056}`). Also observed locally: N 304 -> 92,720 = 304x305; N 814 -> 663,410 = 814x815.
- Trials per run: mean 138,624.0 (min 33,243, max 200,954). The draft's T407 = 138,582.6 is the **lookups** mean (`native.relations.lookups`, 17,738,567/128), and guard probes are 17,743,870/128 = 138,624.0 = trials. Same to 0.03 %.
- The five counters named in `cost.unpriced` (`sqrt_solves, lookups, row_ops, legendre_symbols, target_guard_probes`) are all in `phases[].native`; the band's `native_sum` equals their sum exactly: curve t15757: 20,191,358 = 54,464 + 26,048 + 160,973 + 9,973,470 + 0 + 9,976,403; curve tm10425: 15,771,983 (both match `band-v6.json` `native_sum`). So `target_guard_probes` **is** charged by `band`/`compare --weight` in the v6 run (P11 evidence, not the source read).
- Native per run (priced at w): lookups 138,582.6 + guard probes 138,624.0 + row_ops 2,515.6 + legendre 834.5 + sqrt 407.0 = 280,963.6; so per-trial native k = 277,206.6/138,624.0 = 2.0 (the draft's k_eager = 2 holds).
- **Recomputing the draft's constants:** F (charged, not set-up, not per-trial): relations GAE 141,377.3 minus lookups 138,582.6 = 2,794.7, plus verify 38.7 = 2,833.4 GAE = **0.191 S** at sqrt(r_eff)=14,832 (draft: 0.25 S). G (the non-trial native, row_ops + legendre + sqrt) = 3,757.1 per unit w = **0.253 S per unit w** (draft: "about 0.25 S at N=407" for G, matches). The draft's F = 0.25 S looks like G's value; the measured F is 0.19 S. Charged relations GAE per trial is 141,377.3/138,624.0 = 1.020, not 1.0 (adds 1.0077/trial, doubles 0.012/trial).
- Mean per-run S at the recorded weight 0: 20.77 (incumbent). `band` at (0, 0.1, 1): t15757 20.572/22.571/40.557 (incumbent), tm10425 20.970/22.751/38.778; rho-neg 0.846/0.928/1.668 and 0.960/1.053/1.893. Identical for `candidate` (layout=lines) and for `rho-neg-aa`. `band --ranks` says incumbent vs candidate "unresolved inside the band".
- **Trials against N (local, not v6):** the v6 session has only N=407, so trials(N) cannot be checked from it. From the local non-study run `dev/run2` (2 curves x 24 targets x 3 measured rounds = 144 measured runs per arm): trials ratio walk N304/N407 = 1.3415, stride-renew eager N304/N407 = 1.3693, pc-fb2x(814)/407 = 0.4887, against 407/N = 1.3388, 1.3388, 0.5000. Model assumption (trials proportional to 1/N) is within about 0.2-2.3 % in these samples. Set-up N(N+1) is exact.
- Model against dev measurement, ratios of mean GAE-equivalents to the incumbent (descriptive, non-study curves, ratio of means, not the paired estimator): candidate 0.9135 / 0.8931 / 0.7979 at w 0 / 0.1 / 1 (draft projection 0.915 / 0.894 / 0.798); walk-eager-n304 0.920 / 0.955 / 1.119 (draft 0.915 / 0.949 / 1.113); pc-fb2x 2.365 / 2.208 / 1.473 (draft 2.39 / 2.23 / 1.50); srenew-eager-n407 0.984 / 0.984 / 0.983; srenew-deferred-n407 0.984 / 0.943 / 0.750. Table: `checks/p3c-out.txt`. Candidate and srenew-eager-n304 have identical mean GAE at w=0 (283,986.0).
- Observation to carry: in this local sample stride-renew at N407 used 0.9805 of the incumbent (walk) trials (`dev/run2`), but the draft's F4 smoke found S 21.066 stride-renew against 16.289 walk on one target; I did not rerun F4. n here is small and the curves are non-study.
- The rule's inputs hold (set-up = N(N+1), trials ~ 1/N), so by the draft's own clause N_small = 304 is not changed by P3. Only F (0.19 against 0.25 S) differs; the draft says the N minimiser does not depend on F.

## P5. Seed and curve collisions: PASS (limited scope, see end)

| session | seed | slug | log2 r |
|---|---|---|---|
| S1 | 2026110628201 | icv1-fp28-t10999-67500d78 | 27.72 |
| S1 | 2026110628202 | icv1-fp28-tm6425-08e2435f | 27.50 |
| S2 | 2026110628211 | icv1-fp28-t14969-7ca8e8a5 | 27.29 |
| S2 | 2026110628212 | icv1-fp28-tm4717-89c926fe | 27.62 |

Compared (all by slug computed with `plan`, i.e. ecbench's own `prime_search`):
- the four slugs are pairwise distinct;
- v6: tm10425-13a2119a (seed ...28101), t15757-68bdca4a (...28102): no match; third v6 curve `icv1-fp28-t10131-f8b12347` (named in the draft): no match;
- XP-20261029 specs on disk (`spec-P24/P28/P32.json`): P28 seeds 20261029283/284 -> `tm13337-3a482fe3`, `tm5065-1146b4a5` (the P24 and P32 curves are fp24/fp32): no match. XP-P28 uses `max_trials` 26479085, N 407 and 267, 24 targets per curve, `alternate`, rounds 6;
- F4 smoke seed 2026110628901 -> `icv1-fp28-tm14825-8419afc7`: no match;
- `docs/curves/registry.json` @ea8e9ef7 (20.9 MB): none of the four slugs appears (0 matches each); the XP-P28 slugs are registered (2 matches each) and the three v6 slugs are **not** in the registry (so the registry is not a complete list of used curves);
- `docs/ic/LEADERBOARD.md` @ea8e9ef7: none of the four slugs; its only fp28 slug is `icv1-fp28-t28201-65099091`.
- Fallback seeds (+100) would give: S1 `t7075-59d612a9`, `t12031-c0912d73`; S2 `t26137-730c77fc`, `tm8753-e0a59910` (informational; not triggered).

**Not checked (not on disk or not pathspec-safe):** F6 curves, holdout-P28 curve slugs beyond the one in LEADERBOARD.md, "every committed crypto session". I did not run repo-wide `git grep`. A first pathspec attempt over `docs research/... backlog` was started by mistake and killed (it was too broad); no result was used from it.

## P6. `algorithm_seed` shared across arms: PASS

```
$ $E plan --spec checks/spec-s1-try2.json --doc > checks/plan-s1-doc.json   (same arms/seeds as final-order; spec id ECS1hc027a4dab62e)
executions 2160; groups (workload,round) 216; groups with >1 distinct algorithm_seed across the 10 arms: 0
distinct seeds 216, no seed shared by two different (workload,round) groups; 9 distinct seeds per workload (1 warmup + 8 rounds)
10 arms per (workload,round) group
```
Seed is a function of (workload, round) only: arms with N 304/407/814 and walk/stride-renew*/rho share it. Confirmed on real records: v6 `records.jsonl` has 144 (workload, round) groups, 0 with more than one seed; and the local `dev/run2` `compare incumbent -> candidate` (N 407 against 304, walk against deferred stride-renew) reports `same_seeds: true`, `unpaired_runs: 0`, `pairs: 12` (run1; `checks/cmp-dev1-w*.json`). Pairs in the real session: 8 rounds x 24 workloads = 192. (The warmup round has its own seeds, as does v6.)

## P7. Counters named per phase; deferred emits the uncharged markers: PASS (P7a and P7b)

Local solve: `checks/spec-dev1.json` (1 non-study curve, 6 targets, 10 arms + twin, 2 measured rounds + warmup, 180 executions): `session ECBS1ha931980889d1 complete (180 records: {"verified": 180})`, 23 s. `verify --replay-all`: `audit OK: 180 records, 180 verified, 120 replays reproduced, 0 problems`, exit 0.
Larger one: `checks/spec-dev2.json` (2 curves, 24 targets, 3 rounds + warmup, 8 arms, no twins) 1536 records, all verified.

- **P7b:** for `candidate`, `srenew-deferred-n407`, `srenew-eager-*`, the record's `cost.unpriced` = `["sqrt_solves_uncharged","lookups_uncharged","row_ops_uncharged","legendre_symbols_uncharged","target_guard_probes_uncharged"]`. So the deferred arm lists both `target_guard_probes_uncharged` and `lookups_uncharged`, and the counters exist in `phases[relations].native` (`lookups`, `target_guard_probes`, `trials`, `relations`, `repeated_targets_skipped`, plus `stride_steps`, `stride_renewals`, `stride_restarts`). `rho.negation`: `["canonicalisations_uncharged"]`, no `phases[].native` IC counters; a `counters` block with `targets_tried` is absent for rho records (a script must skip rho).
- **P7a:** records expose per-phase counters **by name**: `phases[]` = `factor_base`, `oracle_setup`, `relations`, `linear_algebra`, `verify`, each with `adds, doubles, scalar_mults, gae, wall_ns` and `native{}` / `work_native{}` (counters). Plus top-level `counters` (`targets_tried, relations_found, matrix_rank, matrix_rows, matrix_work, matrix_dependent`), and `work.phases[].prims` (bitop/v1). Note: `oracle_setup` carries its 407 `lookups` only in `work_native` (not `native`), so `band` does not charge them.
- Walk arms have `walk_*` counters; stride arms have `stride_*` and no `walk_mix_hashes`.

## P4. Deferred against eager under stride-renew at equal N: PASS on stream identity; the parity bound needs to be corrected

**Source read (@ea8e9ef7).** `ic_boundary.rs` 7715-7727 (doc), 8620-8655 (loop), 7761 (`renews_stride`), `stages.rs:636`:
- The target stream is generated before the guard and is the same code for both (`Stride | StrideRenew | StrideRenewDeferredGuard` arm, line 8559). Eager: the guard probes every trial; a repeat does `continue` and, for `renews_stride()` sources, does **not** set `walk_fresh` (no restart) and does not renew the stride.
- Deferred: skips the guard, calls the oracle; if it returns no relation, `continue`; only for a decomposing target it probes `seen`, and on a duplicate does `continue` **before** the renewal block. A fresh decomposition then renews the stride. So the guard firing never renews the stride in either variant, and a repeat never restarts: the stream is the same by construction. What changes: eager skips the oracle lookup of every repeat; deferred does the lookup of every repeat (non-decomposing ones return nothing; a repeat of a decomposing target returns relations and is then dropped by the deferred guard).

**Measured (local, non-study curves; `checks/p4-out.txt`, `p4b-out.txt`, `p3c-out.txt`).** Paired by `algorithm_seed` (same workload and round), 192 pairs each in `dev/run2` (144 measured + 48 warmup), 18 in `dev/run1`:

| pair (eager -> deferred) | pairs | differing: recovered log, status, trials, relations found, rank, matrix_work, stride_steps, stride_renewals, target_rng_draws, per-phase adds/doubles/scalar_mults | differing: total GAE |
|---|---:|---|---|
| srenew-eager-n407 -> srenew-deferred-n407 | 192 | none (0 of 192) | 10 of 192, each +1.0 GAE (one add in `relations`) |
| srenew-eager-n304 -> candidate (N 304, deferred) | 192 | none (0 of 192) | 10 of 192, each +1.0 GAE |
| same two pairs, `dev/run1` | 18 | none | 0 of 18 (all Δ = 0) |

- All 20 differing pairs: the deferred record's `target_guard_probes` equals its `relations` + 1 and its relations-phase adds are +1. i.e. the deferred run did the oracle for one repeat of an already-decomposed target (which the eager guard skipped), paid 1 addition for it, then dropped it at the deferred guard. In the other 172 pairs: Δ GAE = 0.
- Deferred guard probes = relations (+ the above) in every pair; eager guard probes = trials.
- Δ lookups (deferred - eager) = eager `repeated_targets_skipped` in all 192 pairs of both comparisons (sum 8,499 at N407; 16,512 at N304 -> about 44 and 86 per run). Δ guard probes (`dev/run1`, 18 pairs): -71,276 to -177,403 per run (N407), -107,001 to -246,919 (N304).
- **Corrected bound for the parity rule:** Δ GAE at w = 0 (deferred - eager) is in {0, +1, ...}: it equals the number of skipped repeats whose oracle call is charged (observed one add each), i.e. at most (deferred `target_guard_probes` - deferred `relations`) additions, here 0 or 1 per run; the draft's working bound ("at most one addition per repeat the eager guard skipped", repeats = eager guard probes - eager lookups) holds but is much looser than what was observed (max Δ of 1 add against 19-136 eager-skipped repeats per run in the differing pairs). Largest observed relative difference: 1/283,986 = 3.5e-6 (N304).
- At w=0.1 and 1 the weighted difference is the guard-probe saving: srenew-deferred-n407 / srenew-eager-n407 mean-GAE-equivalents: 1.0000 (w=0), 0.9584 (w=0.1), 0.7636 (w=1) (from `checks/p3c-out.txt`, ratio of means, non-study curves). The expected w = 0 difference stated in the draft (about 41 additions) was not seen: it was 0 or +1 per run, never 41 in a run, and zero on average of the 192 pairs (10/192 = 0.052 per run).

## Summary table

| check | result |
|---|---|
| P1 | PASS: 10 arm strings valid, S1/S2 plan exit 0, 2160 executions each; defaults `linalg=incremental-gauss`, `solver=""`, `solver_budget_seconds=0`, rho `cap_multiple=64`; stride-renew variants expand with no extra params |
| P2 | PASS with changes: roles are only reference/baseline/candidate/control (`descriptive` is invalid); multiple candidates OK; `incumbent-aa` is `control` and must follow `rho-neg-aa`; launcher pre-check passes |
| P3 | PASS: set-up = N(N+1) exactly; 54.0 % / 46.0 % GAE split set-up/trials; trials ~ 1/N within 0.2-2.3 % locally; draft F should read 0.19 S not 0.25 S; N_small unchanged |
| P4 | PASS on stream identity (0 of 192 + 0 of 192 pairs differ in log, relations, trials, rank, stride counters); parity bound needs correction to Δ GAE in {0, 1} (10/192 pairs +1), tied to guard probes - relations |
| P5 | PASS for the checked sources (v6, XP-20261029, F4 seed, registry, LEADERBOARD.md); F6 / holdout-P28 / other committed sessions not checked |
| P6 | PASS: one seed per (workload, round) for all 10 arms; 216 seeds / session; `same_seeds: true` across N and target source |
| P7 | PASS (a and b): deferred emits `target_guard_probes_uncharged` and `lookups_uncharged`; per-phase counters by name in `phases[].native` / `work_native` |

## Files

All under `/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/next-candidate/checks/`: `spec-s1-final-order.json`, `spec-s2-final-order.json`, `spec-badrole.json`, `spec-dev1.json`, `spec-dev2.json`, `plan-s1-doc.json`, `plan-s1-final.txt`, `plan-s2-final.txt`, `band-v6.json`, `p3-out.txt`, `p3c-out.txt`, `p4-out.txt`, `p4b-out.txt`, `p7-out.txt`, `cmp-dev1-w*.json`, `dev/run1/`, `dev/run2/` (records), `dev/audit1.json`, scripts `mkspec.py p3.py p3b.py p3c.py p4.py p4b.py p7.py`, source copies `ic_boundary.rs`, `run-ecbench.src`. I did not modify the draft or any repo file; two `git config gc.auto 0 / gc.autoDetach false` settings were written in the sparse clone `v6/repo/.git/config` to stop background `git gc` while reading blobs (a clone config, not a tracked file).


# Part 2: P5r, P8, P9, P10 (draft revision 3; draft mtime 08:25:30Z unchanged throughout)

Work files: `checks/` (as before), `checks/p9/`, `freeze/` (the freeze package), `gen/build_protocol.py`. All paths below are under `/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/next-candidate/`.

## P5r. Residual collision check: PASS (by GitHub code search; the `git grep` was not run)

**Holdout-P28** (`git -C v6/repo show origin/main:research/prime_base_oracle_tuning_20261029/sessions/holdout-P28/spec.json`, origin/main = a2e8fbc1d08f3369ddcd647311a4a6faf408b986; saved as `checks/holdout-P28-spec.json`): one curve, `prime_search` 28 bits seed 20261018202, target seed 20261018211, 48 targets. `ecbench plan` slug: **`icv1-fp28-tm1499-56171ddc`** (r = 2^27.57). Not one of the four C1 slugs.

**F6 seeds** (for the record): 2026110628904 -> `icv1-fp28-t6151-13657c27` (2^27.39); ...905 -> `icv1-fp28-t10663-31bf0393` (2^27.43); ...906 -> `icv1-fp28-t27971-b8709714` (2^27.94). None is a C1 slug. (F4 seed ...901 earlier: `tm14825-8419afc7`.)

**Repository search.** Size estimate: `git ls-tree -r --name-only origin/main -- research backlog benchmarks docs/ic | wc -l` = **250,011 files**, all blobs absent from the blobless clone. Blob sizes cannot be read without fetching them (`ls-tree -l` is barred), so the fetch cannot be bounded below the 1 GB limit, and with 6.2 GB free I did **not** run the `git grep`. I used GitHub code search instead (`gh search code ... --repo isogeny-labs/crypto --limit 100 --json path`; the repo name `aburan28/crypto` in the clone's remote returns nothing, `isogeny-labs/crypto` works). Positive controls first: `run_pipeline`, `tm10425-13a2119a` (a v6 slug), `2026110628101` (a v6 seed) and `tm1499-56171ddc` (the holdout slug) all return files (`checks/gh/search-isogeny.txt`; the earlier empty results in `checks/gh/search.txt` and `search-control.txt` came from the wrong repo name and are void). Results for the C1 strings:

| string | files found |
|---|---|
| t10999-67500d78, tm6425-08e2435f, t14969-7ca8e8a5, tm4717-89c926fe | none |
| icv1-fp28-t10999 | none |
| 2026110628201, ...202, ...203, ...204, ...211, ...212, ...213, ...214 | none |
| 2026110628904 (F6 seed, positive control for the F-seeds) | 5 files, all in `research/ic_line_layout_tournament_20261031/` |

Limits of this method: code search covers the default branch only, may lag recent pushes, skips large files, and I read paths but not match context. Nothing in this study is committed yet, so "no match" is the expected result. P5r result: **no collision** within those limits; the fallback seeds are not triggered.

## P8. Power: RAN; passes is false at the design size (3 % effect)

**The tool.** `python3 car/tools/experiment_protocol.py power --n 192` -> `{"rank_test_n":192,"min_attainable_two_sided_p":0.01036,"threshold":0.05,"passes":true,"n_needed":40}`. Its docstring: "Check add-one empirical-null p-value resolution, not statistical power." It says nothing about this design, so `rank_test_n` is null in `protocol.json`.

**Computation (mine, method `other`).** Variance from two **design-size local runs on non-study curves** (2 curves x 12 targets x 8 rounds + 1 warm-up; arms incumbent, candidate, srenew-eager-n407/n304, srenew-deferred-n407; `checks/dev/power1`, `power2`, 1080 records each, all verified), the real `ecbench compare --weight w` intervals (`checks/p8b-power1.txt`, `p8b-power2.txt`):

| run | w | ratio cand/inc | t-CI | bootstrap CI |
|---|---|---|---|---|
| power1 | 0 / 0.1 / 1 | 0.8666 / 0.8486 / 0.7621 | [0.8470,0.8867] / [0.8284,0.8693] / [0.7392,0.7858] | [0.8375,0.8963] / [0.8189,0.8793] / [0.7295,0.7968] |
| power2 | 0 / 0.1 / 1 | 0.9094 / 0.8920 / 0.8085 | [0.8805,0.9392] / [0.8632,0.9218] / [0.7781,0.8401] | [0.8764,0.9435] / [0.8584,0.9261] / [0.7715,0.8457] |

(The intervals differ by a factor of about 1.5 between the two replicates, so the width itself is uncertain.) Model: `hat ~ N(true, sd)`, `U = hat + max(t half-width, bootstrap half-width)`, half-widths scaled by true/observed; sd from the t-CI (optimistic) or the bootstrap CI/1.96 (conservative). One session, P(U_w < 1) (`checks/p8c.py`, `p8c-out.txt`, `p8c-rep2-out.txt`):

| targets/curve (workloads) | 3 % effect, w = 0 / 0.1 / 1 | projected effect (8.6 / 10.6 / 20.2 %) |
|---|---|---|
| 12 (24) = draft | power1 0.38-0.42 / 0.33-0.39 / 0.17-0.27; power2 0.34-0.37 / 0.33-0.35 / 0.21-0.26 | 0.998-1.000 |
| 24 (48) | power1 0.71-0.80 / 0.66-0.74 / 0.45-0.47; power2 0.63-0.65 / 0.61-0.63 / 0.45-0.46 | 1.000 |
| 36 (72) | power1 0.87-0.96 / 0.83-0.93 / 0.64-0.70; power2 0.80-0.85 / 0.78-0.83 / 0.63-0.66 | 1.000 |

Reading as facts: at the draft design the power for a 3 % true effect is below 0.8 at all three weights (0.17 to 0.42); the rule in section 12 ("below 0.8 at any weight -> raise targets_per_curve, rerun P1 and P6") is met. A 3 % effect at w = 1 is the least natural case (the model projects 20 %); even at 36 targets per curve the 3 % power at w = 1 is 0.63-0.70. Both sessions must pass, so joint power is about the square. A first, cruder simulation on `dev/run2` (3 rounds, mean log ratio, `p8.py`, `p8-out.txt`) gave 0.39 / 0.35 / 0.22 and a minimum detectable ratio (80 %) of 0.950 / 0.948 / 0.933; it is superseded by the design-size runs. v6 cross-check (`p8v6-out.txt`): incumbent per-run CV of S 0.109 / 0.120 / 0.172 at w 0 / 0.1 / 1, within-workload SD of log S 0.101 / 0.112 / 0.167. The note written into `protocol.json` is `gen/p8-note.txt`; `sample_size.power.passes` is **false**. No change was made to the specs: the Coordinator decides whether to raise `targets_per_curve` (that would change the specs, the requests, the 2,160 count, section 7.2 and this P1/P6 evidence).

## P9. `analyze-c1.sh`: PASS

File: `analyze-c1.sh` (copy in `freeze/analyze-c1.sh`). **sha256 `1b1f44505916f94b636e9f0f3be445033e46e566b7cad52b4deba72168f8b72f`**. Usage: `analyze-c1.sh ECBENCH OUT_DIR SPEC_S1 SPEC_S2 SESSION1 SESSION2`. Implements section 7.3 and section 8: integrity (verify --replay-all exit 0, audit fields, 2,160 records, planned = written = 2,160, status complete, no invalid outcome, `l2-gate.json` invalid_outcomes `[]` when present, session `spec.json` and `input-spec.json` canonical-JSON equal to the frozen spec, binary sha256 = pin in session.json and every record); controls (incumbent-aa and rho-neg-aa exact at 3 weights; pc-fb2x ratio > 1 and ci95[0] > 1); parity rules 1-4 over all 216 pairs for both comparisons; primary compare at w = 0, 0.1, 1 with `status ok`, `same_seeds`, 192 pairs, 0 unpaired, 0 censored; U_w = max, L_w = min of the t and bootstrap bounds; C1-guard (equivalence [0.995,1.005] at w = 0, U_w < 1 at 0.1 and 1); the section 4.2 factorial and rho -> each IC arm at 3 weights; band --ranks; per-phase table; arms by name, rho skipped for IC counters; no wall field read; exit 2 on a wrong binary hash or usage error. It also writes `reports/c1/s{1,2}/native-supplement.jsonl`, one row per record with every `phases[].native` counter (keyed `instance_id` = workload_id, `replicate` = round), for the export (from-ecbench drops them).

Exercised:
1. **Real end-to-end** on a full-size 10-arm local session on non-study curves (`checks/spec-dev-full.json`, seeds ...99401/402, `checks/dev/full-job/session`, 2,160 records, all verified; L0): `checks/p9/real-run.txt`, 6 m 23 s, overall `supported_within_scope`, S1 = S2 (same session passed twice), all integrity, control and parity gates true, parity 216 pairs per comparison with 12 and 17 pairs having dGAE = 1; U_w 0.858 / 0.849 / 0.801; C1-guard pass; route B trigger false. Descriptive, from non-study data, no scientific content. A first run exposed a variable-shadowing bug in my new supplement loop (session verdict printed as an integer); fixed, the whole run repeated, and the sha above is of the fixed file.
2. **Exit 2**: wrong binary (pin + 1 byte) -> `ecbench sha256 is not 73362eed...`, exit 2; too few arguments -> usage, exit 2; non-session directory -> exit 2.
3. **Invalid session end-to-end**: the 180-record `dev/run1` as both sessions -> `invalid_measurement` (records_2160, planned = written and spec equality false; control comparisons not valid).
4. **Stubbed failure paths** (`checks/p9/stubs.py`, output `checks/p9/stubs-out.txt`, 29 of 29 as expected): non-identical twin -> invalid_measurement; rho twin with same_seeds false -> invalid_measurement; pc-fb2x ci95[0] <= 1 at w = 0.1 -> invalid_measurement; censored pair -> invalid_measurement; pairs 191 -> invalid_measurement; unpaired runs -> invalid_measurement; build != pin and spec != frozen -> invalid_measurement; parity failure -> implementation_defect (integrity failure takes precedence over it); U_0 >= 1 -> fail; only the bootstrap U_1 >= 1 -> fail; L_0 > 1 recorded as fail_adverse ['0']; overall mapping for all rows of the section 8 table (pass+pass, fail+fail common adverse or not, mixed, defect, single_unreplicated pass/fail, both invalid); parity pair rules: dGAE 0 and dGAE = d = 1 pass; dGAE > d, a `relations_found` mismatch and d lookups != eager repeats each fail with the right rule.
Not exercised (no input): `input-spec.json` and `l2-gate.json` next to a session (the code reads them only when present; the dev job has neither), a real censored pair, and a failing real-session twin (stub only).
**Note for the draft:** `session.json` `invalid_outcomes` is `null` in ecbench sessions (v6 counted too); the `[]` value of section 7.3 lives in the launcher's `l2-gate.json`. The script computes invalid outcomes from the records and also requires the `l2-gate.json` value to be `[]` when that file is present.

## P10. Freeze package: validate PASS; preapproval packet NOT BUILDABLE (blocked on Coordinator ids)

Files in `freeze/` (to be copied under `research/ic_c1_deferred_guard_fp28_20261106/`):

| file | sha256 |
|---|---|
| spec-c1-s1.json | 3067767ad7f9b095a7ece8d9714c7fffb3eeee28f9c6d63bbfdb43755720c166 |
| spec-c1-s2.json | ea0e430f42025d4baa64c5525374f5e8a5707510d2cbd673ee398127a8b84510 |
| launch/request-s1.json | 160e56bdc04a02b7ad4ffd85d82633f8ed7bca4fa7efaeb537f29dfbcd09d231 |
| launch/request-s2.json | 90493da496afdcb2c1cf70b417b76f79ed7ac3b98447cf515c92a2409e638fe5 |
| analyze-c1.sh | 1b1f44505916f94b636e9f0f3be445033e46e566b7cad52b4deba72168f8b72f |
| protocol.json | cd47ec98f86063e2ae695e6c0461107edf56c2642348f4e592a8a6317328904d |

- **Specs.** Copies of `checks/spec-s{1,2}-final-order.json` with only `label` and `description` rewritten (the pre-freeze wording is not meant for a frozen file). Verified: `ecbench plan` spec ids unchanged (`ECS1hc027a4dab62e`, `ECS1hab11b5206d1d`), 2,160 executions, exit 0 (labels are not part of the id).
- **Requests.** `isolab.eks.request/v1`, `job_name` `ic-c1-dg-s1-20261106` / `ic-c1-dg-s2-20261106` (the date is a placeholder: change it if the launch day differs, before the freeze), `instance_type` c8a.2xlarge, `seconds` 3600, `build` = the pin (image digest, source ea8e9ef7..., ecbench 73362eed...), no `memory` (default 4Gi as in v6). Provenance harness and source_revision set. **`jq -S -c .job.spec request` equals `jq -S -c . spec` for both** (note: the spec sits at `.job.spec`). **`taskq/deploy/eks-karpenter/remote/validate-request` exit 0 on both** (it prints the defaulted request; saved in `checks/validate-s*.out`).
- **protocol.json** (`gen/build_protocol.py`; follows section 16). `python3 car/tools/experiment_protocol.py validate freeze/protocol.json` -> `OK: experiment.protocol/v1 freeze/protocol.json (0 finding(s))`, exit 0. It accepted `cmp-recipe` as `one_factor` over the bundled `recipe` variable (no fallback needed). First attempt had 3 findings (derived metrics need a `formula`); fixed. Set as instructed: protocol roles (incumbent baseline; candidate and 4 descriptive arms candidate; pc-fb2x control_positive; rho-neg reference; two A/A twins control_aa), every arm instrument = repo isogeny-labs/crypto, full 40-character commit ea8e9ef7..., `build: null`, binary sha256 73362eed..., image digest, `threads: 1` (confirmed: session `child_env` OMP_NUM_THREADS = RAYON_NUM_THREADS = 1 and every record's `time.solve.threads_before/after` = 1), and `same_instrument_as: "incumbent"` on every other arm; `tests_in_family` 6 with `correction: "other"` in power and decision_rule; `measurement.isolation_required` "none", clock counted; backend isolab-k8s, timeout 3600, checkpoint none; interleaved order; `harness_drawn` seeds with the rule named; `preflight_runtime` non-null; `amendments` []; `freeze.status` "draft" (**not frozen**, `freeze` not run); everything else under `extensions`. Fields I could not fill: `links.experiment_id`, `hypothesis_ids` and `prs` are null (no EXP-/H- id is allocated; I allocate none); `sample_size.power.passes` is false (P8).
- **Preapproval packet.** `experiment_preapproval.py --packet` needs `experiment_id`, a reserved `approval_decision_id`, canonical paths for specification.yaml, trial-plan.json, queue.json and a handoff, plus preview copies. None exist. With a placeholder packet (`freeze/preapproval-packet-PLACEHOLDER.json`, nulls) the tool answers `{"approvable": false, "blockers": ["packet: valid experiment_id required"]}`, exit 2. So `approvable: true` cannot be reported until the Coordinator allocates the ids and the experiment specification, trial plan, queue and handoff exist; the packet is not an Executor product.


# Part 3: P1r, P6r, P8r, P9r, P10r (draft revision 4; draft mtime 08:51:15Z at the start, checked again at the end)

Paths under `/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/next-candidate/`. The revision-3 package is kept in `freeze-r3-backup/`. Nothing frozen, committed or launched.

## P1r: PASS
`freeze/spec-c1-s{1,2}.json` with only `targets_per_curve` changed 12 -> 60. `ecbench plan` exit 0, **10,800 executions** each (`checks/plan-s1-T60.txt`, `plan-s2-T60.txt`).

| | spec id | sha256 |
|---|---|---|
| S1 | ECS1h3229b941623b | d2af59dd9f62d46f28c0287a475becfe468e8e0a17eabe5f96681e2abf09e8ad |
| S2 | ECS1h1e755d9669a9 | 688ec9a004648933231302935704d9307cfc441aaa231fc6efb25205a302cf34 |

Slugs unchanged: S1 `t10999-67500d78` (60 workloads), `tm6425-08e2435f` (60); S2 `t14969-7ca8e8a5` (60), `tm4717-89c926fe` (60).

## P6r: PASS
`plan --doc` on S1 (`checks/plan-s1-T60-doc.json`): 10,800 executions, **1,080 (workload, round) groups**, 0 groups with more than one `algorithm_seed`, 10 arms in every group, 1,080 distinct seeds.

## P8r: PASS at T = 60 (the T = 72 fallback is not needed)
`checks/p8r.py`, `p8r-out.txt`. Same model as P8 (U = hat + max of the t and bootstrap half-widths from the two design-size local runs, scaled by sqrt(24/120)). Single-session P(U_w < 1) for a 3 % effect, w = 0 / 0.1 / 1:

| variance input | conservative sd (bootstrap) | optimistic sd (t) |
|---|---|---|
| power1 | 0.977 / 0.963 / 0.845 | 0.999 / 0.997 / 0.942 |
| power2 | 0.951 / 0.944 / 0.838 | 0.977 / 0.972 / 0.890 |

Minimum 0.838 (power2, w = 1, conservative) >= 0.8: passes. (T = 72 would give a minimum of 0.897.) Two sessions must both pass, so the joint figure at the minimum is about 0.70. Caveat: the t quantile is kept at 23 degrees of freedom (conservative at 119), and both variance inputs are small local samples. Cross-check on real data: the T = 60 local session (below) gave candidate/incumbent upper bounds U_w 0.914 / 0.897 / 0.818, half-widths about 0.014-0.032, in line with the model's U half-widths 0.015-0.020 (those are for a 3 % effect, the real run's effect is about 12 %). `sample_size.power.passes` is true and the note is `gen/p8-note.txt`.

## P9r: PASS
`analyze-c1.sh` no longer has literal 2160 / 192 / 216 (grep finds none). `set_counts(spec)` derives them from the frozen spec: workloads = curves x targets_per_curve; records = workloads x (warmup + rounds) x arms = 10,800; pairs = workloads x rounds = 960; parity pairs = workloads x (warmup + rounds) = 1,080. If the two frozen specs imply different counts the script exits 2. Integrity keys renamed `records_count_matches_spec` and `planned_eq_written_eq_spec_count`.
**New sha256: `66aa2ffd91e10f5d1f1a65be086cd17c3bf388b4299157f646b75ea49d994dcc`** (`freeze/analyze-c1.sh` is the same file).

- **Stubs:** all 29 pass with counts derived from the frozen T = 60 spec (10,800 / 960 / 1,080) (`checks/p9/stubs-T60-out.txt`).
- **Full-size real local session at T = 60**, non-study (`checks/spec-dev-full-T60.json`: curve seeds 2026110699501/502, target seed ...503, measurement ...504; L0; `--cpus none`): 10,800 records, all verified, `ecbench run` 21 m 43 s = **0.1206 s per execution** (against the 0.177 s used for the budget). Then `analyze-c1.sh` on it (same session passed as S1 and S2, spec = the dev spec) took 34 m 57 s (`checks/p9/real60-run.txt`, `checks/p9/out-real60/`): overall `supported_within_scope`; per session all integrity and control gates true; parity 1,080 pairs per comparison, 0 failed (dGAE = 1 in 67 and 54 pairs); primary compare `pairs` 960, `workloads` 120, dof 119, censored 0; U_w 0.914 / 0.897 / 0.818, L_w 0.886 / 0.869 / 0.786; C1-guard pass; route B trigger false; `native-supplement.jsonl` 10,800 rows. Descriptive on non-study data; no scientific content.
- Wrong binary hash -> exit 2 (rerun with the new script).

## P10r: validate PASS; **validate-request FAILS at 5,400 s (cap 3,600)**
- **`validate-request` caps `.job.seconds` at 3,600.** Line 16: `(.job.seconds // 900)<=3600`, in the pinned-commit copy (ea8e9ef7) and at origin/main (`git show origin/main:taskq/deploy/eks-karpenter/remote/validate-request`). Both requests with `seconds: 5400` exit 2 ("Invalid isolab.eks.request/v1 request"; `checks/validate-s*-T60.err`). With `seconds` 3600 the same request validates (exit 0); 3601 and 5400 fail (`checks/req-test-*.json`). The bridge's `MaxSessionDuration` is 7200 (bridge.yaml:155) but that is a different limit. **Returning to the Coordinator as instructed.** The requests are written with 5400 and are therefore not launchable as they stand; the 3,600 s alternative would need S1 to fit: at the measured 0.1206 s per execution a session is about 1,300 s of measurement plus fixed time, which is inside 3,600 s, but the draft chose 5,400 on the 0.177 s figure (about 2,140 s) plus provisioning.
- Requests: `freeze/launch/request-s{1,2}.json`, job names `ic-c1-dg-s1-20261106` / `ic-c1-dg-s2-20261106`, `seconds` 5400, spec at `.job.spec` embedded `jq -S -c`-equal to the spec file (both). sha256: s1 `a4a68e03f60a2acc1431b4294456e6dfb03f98a4d19a0416ac67304a67921cc7`, s2 `035612631c2e2d227bfaf0246b30631295c5bdc1a5c757f4038d35a360e73dd1`.
- **`protocol.json`** (`freeze/protocol.json`, sha256 `99698a24a37e7e22b0649999343b4307d8f93c784bfcdea1bce8065d9f2f8b5e`; `gen/build_protocol.py`): `experiment_protocol.py validate` -> **OK, 0 finding(s), exit 0**. Updated: `sample_size.instances` 120; `preflight_runtime` fixed 230 s, per-sample 0.177 s, total 2,142 s (measured rate in the note); `execution.timeout_seconds` 5400; `power.passes` true with the P8r note; `analysis_plan.script.sha256` 66aa2ffd...; spec hashes in `extensions`; budget note (HB 1.14662, S1 and S2 only); `links.experiment_id` EXP-ICPERF-127dbd; `links.decisions` [DEC-20261106-1d23b6, DEC-20261106-c69de4]; parity count 1,080; `links.hypothesis_ids` still null (not allocated). `freeze.status` "draft". Note the inconsistency: `execution.timeout_seconds` 5400 as instructed, while `validate-request` refuses 5400.

## How an earlier EKS-routed EXP-ICPERF trial plan was set up (read-only, car clone)
- **No EXP-ICPERF experiment in the car clone is EKS-routed.** `git grep -i -E "launch-remote|isolab-eks|eks-karpenter"` over `experiments/EXP-ICPERF-*/trial-plan.json` and `specification.yaml` (all 33 ICPERF directories at HEAD) returns nothing.
- The two ICPERF experiments I read (sparse-checkout added `experiments/EXP-ICPERF-284bc4` and `EXP-ICPERF-283a70`, now present in the car clone) are local stage-0 instruments on the **runbook route**: `ledger/goals/GOAL-ICPERF-e6b6a4/checkpoints/TASK-20261029-0a9023.yaml` records, per experiment, the specification sha, the `runbook.yaml` sha, the `runbook_lint` sha, the executor handoff sha and `lint_output: LINT OK experiments/<EXP>/runbook.yaml sha256 ...`; approvals DEC-20261029-ca33b3 and DEC-20261029-0e40f3; `execution_state: ready` read from `tools/newest_experiments.py --experiment <EXP> --json` with "the trial plan experiments/<EXP>/trial-plan.json". Preapproval probe reports at `coordination/design/TASK-20261029-0a9023/<EXP>/preapproval-report.json`.
- **trial-plan.json lives at `experiments/<EXP>/trial-plan.json`** (schema `crypto.autoresearch.trial_plan.v1`; keys: approved_by, experiment_id, frozen, goal_id, hypothesis_id, input_sha256, queue = `coordination/goals/GOAL-ICPERF-e6b6a4/batches/BATCH-38f344/dispatch_queue.json`, source_sha256 of `implementation/run.py` and `check.py`, specification + sha256, task_id, `trials[]` with argv, artifacts, check_argv, depends_on, id, memory_mb). Experiment directory contents: specification.yaml, runbook.yaml, runbook-lint.txt, trial-plan.json, implementation/, runs/, runbook-sessions/, audits/, review/, amendments/, reports/, analysis.md.
- The EKS-routed ICPERF work of 2026-10-29 (XP-20261029 packed-filter, P24/P28/P32) was a **backlog XP card** in the crypto repo (`research/ic_packed_filter_20261029/.../eks-results/`), handed to a validator by TASK-20261029-654f73 with "No EKS job, no relaunch"; I found no `EXP-ICPERF-*` trial plan for it in the car clone. I did not find a handoff `execution_route.kind` of `runbook` or `implementation` for an EKS run in the ledger. The only `execution_route` blocks I found were formal-verification kinds (TASK-20261101-*). So the evidence in the clone supports `runbook` (lint receipt, local instrument) for the ICPERF experiments that exist; it contains no EKS precedent for either kind. The preapproval tool's own docstring lists the two kinds: `implementation` (needs a reason and an executor-implementation handoff) and `runbook` (needs a committed lint receipt).


# Part 4: P10r2, P10r3, R1 (draft revision 5; draft mtime 09:57:34Z)

Paths under `/tmp/claude-0/-home-user/fe537fb0-7f2e-504d-8df2-39e551cfffea/scratchpad/next-candidate/`. Nothing frozen, committed, run on EKS or written to AWS.

## P10r2: PASS
Both requests set to `seconds: 3600` (job names `ic-c1-dg-s1-20261106`, `ic-c1-dg-s2-20261106`; `.job.spec` still `jq -S -c`-equal to the spec files). `taskq/deploy/eks-karpenter/remote/validate-request`: exit 0 for both (`checks/validate-s*-3600.out`).

| request | sha256 |
|---|---|
| launch/request-s1.json | 683cc67d136ab6a2e5f1f3fbc7f6ae20ba981194f4781418340d883620b6c779 |
| launch/request-s2.json | d58a601e3a5503bd37578105763f05c35ec6b98503faa55628781fd61f8c4bf1 |

## P10r3: PASS
`freeze/protocol.json` rebuilt (`gen/build_protocol.py`, `gen/p8-note.txt`): `execution.timeout_seconds` 3600; `preflight_runtime` fixed 230 s, per-sample 0.1206 s, total 1,533 s (measured_on: the P9r local rate); budget note in `extensions.budget_gate` (HB(3600 s) = USD 0.93108, gate lines 1.86 / 2.79 / 2.79 / 3.72, worst case 3.72, expected about USD 0.6-1.0, no holdback V); power note now states the joint two-session power at the minimum single-session value, 0.838^2 = 0.702; new stopping rule for the deadline hit (request is not rerun, no result read, returns to the Coordinator; kind `invalidation` because the schema enum has no `deadline` kind); request sha256 values in `extensions.requests`; `links.hypothesis_ids` ["H-ICPERF-4cb536"]. `experiment_protocol.py validate freeze/protocol.json` -> **OK, 0 finding(s), exit 0**. **protocol.json sha256: `5d83e72ebc42517c359d12bc27b6240f67eb83157cac518fd60f658a188468b9`.** Unchanged: spec and script hashes (spec-c1-s1 d2af59dd..., spec-c1-s2 688ec9a0..., analyze-c1.sh 66aa2ffd...); `freeze.status` "draft".

## R1: can a runbook carry the C1 session? (read-only; no runbook written)
Method: read `tools/runbook.py` (2,970 lines), `templates/runbook.yaml`, `docs/runbook-execution.md` (added to the car sparse checkout, as are `/examples/runbooks`); `python3 tools/runbook.py lint --help` and `--help` run. `tools/supervisor.sh --help` is not the trial supervisor (it is an autoresearch loop and aborts "opencode is not on PATH"); the trial supervisor is `tools/experiment_execution.py` (`--help` works; the runbook's `trials` step with `engine: supervisor` runs it). To test the lint on real shapes I copied `experiments/EXP-ICPERF-284bc4` into a scratch repo (`checks/r1/repo`, with a stand-in dispatch queue because `coordination/` is not on disk) and linted variants with `python3 tools/runbook.py --repo checks/r1/repo lint` (`checks/r1/lint-out.txt`): the unmodified runbook lints OK (sha e5850bc6...), so the stand-in queue is valid for this purpose.

| # | Question | Result | Evidence |
|---|---|---|---|
| a | a 4,500 s trial | **PASS** | `timeout_seconds` is accepted from 1 to 604,800 (runbook.py:697); variant with `timeout_seconds: 4500` and `claim.ttl_minutes: 80` lints OK (lint requires ttl x 60 >= timeout). A trial in the plan may also carry its own `watchdog_seconds` (used instead of the step timeout, runbook.py:1717). |
| b | network access plus the ambient AWS identity | **PASS in the tool, untested at runtime** | The runbook runner does not sandbox: children get `dict(os.environ)` plus the declared `environment.env` (`_subprocess_env`, runbook.py:1271); I found no network restriction in `runbook.py` or `experiment_execution.py`. A runbook may not set credential-named variables: `environment.env.AWS_SECRET_ACCESS_KEY` fails lint ("may not be set by a runbook", the regex rejects names with TOKEN, SECRET, PASSWORD, API_KEY, CREDENTIAL); `AWS_DEFAULT_REGION` and `programs: [python3, aws]` lint OK. So the ambient identity (profile, IMDS, role) is inherited, not declared. Whether the executor's host allows network is outside the tool. |
| c | a replay setting that does not re-run a trial | **PASS** | `replay.mode: exempt` with `exempt_reason` lints OK (with or without `sample`); the audit then lists the step as `not_covered` ("replay exempt (reason)"), runbook.py:2363. |
| d | controls that are arms within a job | **FAIL as stated; workaround exists** | `controls` is a list of trial ids from the plan (lint: "controls lists trial ids from the plan"), and `control_validity` is computed from those units only. Arms inside one ecbench session are not units, so they cannot be named controls (an empty list gives `control_validity: not_applicable`; lints OK). The A/A twin and `pc-fb2x` gates can be stated only as `validity` assertions on files the job produces (for example a gate JSON with `assertions` on its pointers), or in a separate verification command step. |
| e | stop on timeout, retry on named infrastructure failures | **PASS in lint; run-time behaviour not exercised** | `on_failure` accepts `{when: {timed_out: true}, class: infrastructure_error, action: stop}` followed by `{when: {exit_code: N}, class: infrastructure_error, action: retry, max_attempts: 2}`; the variant lints OK. First matching rule wins (so the timeout stop goes first). Only `infrastructure_error` and `resource_exhaustion` may retry, with `max_attempts` 2 to 3. Without a rule a supervisor timeout is classed `resource_exhaustion` (runbook.py:1597) with no retry. Limits: `when` is exactly one of exit_code / timed_out / validity per rule; on a `trials` step the exit code is the supervisor's (2 = refused to launch, 3 = trials remain, 130 = interrupted), not the job's 42/43 or a `-r2` host-loss signal, so "named infrastructure failures" must be those codes or a `validity: false` rule. A retry re-runs the step's supervisor on the same plan; I did not test how the write-once unit directories behave on a retry. |

Other constraints seen while linting (for the runbook author): the plan's task must be in the plan's `queue` file as an `executor` task whose `artifact_paths` list every run file (including supervisor-generated ones) and whose `write_scope` covers `experiments/<EXP>/runbook-sessions` (first lint failures were exactly these); runbook text read by the operator may not contain vague words (`VAGUE` list); approval needs a committed decision naming `runbook_sha256`; the engine for an `EXP-*` runbook is `supervisor` (`local` only for `EXAMPLE-*`).


# Part 5: P10r4, P10r5 (draft revision 6; draft mtime 10:08:36Z, unchanged at the end)

Nothing frozen, committed, run on EKS or written to AWS. Paths under `.../scratchpad/next-candidate/`.

## P10r4: PASS
`freeze/launch/request-s{1,2}-r2.json`: copies of the 3,600 s requests with only `job.job_name` changed (diff of the two files with `job_name` removed is empty): `ic-c1-dg-s1-20261106-r2`, `ic-c1-dg-s2-20261106-r2` (23 characters). `validate-request` exit 0 on both; `jq -S -c .job.spec` equals `jq -S -c . spec-c1-sK.json` on both.

| file | sha256 |
|---|---|
| launch/request-s1-r2.json | fd9c9c8fd5e64a333a4f650c1e8ff34dc9309193f290240b39f47845652a3e44 |
| launch/request-s2-r2.json | 816d85f0b37089c090cadce804d6ced760e9de255de39030deceaf2b84819050 |

(first-attempt requests unchanged: s1 683cc67d..., s2 d58a601e...)

## P10r5: PASS
C1-C7 of section 21.2 applied in `gen/build_protocol.py` with the section's wording; `experiment_protocol.py validate freeze/protocol.json` -> **OK, 0 finding(s), exit 0**.
- C1: `extensions.spec_ids` = {S1 `ECS1h3229b941623b`, S2 `ECS1h1e755d9669a9`} (they now agree with the T = 60 spec hashes in `extensions.specs`).
- C2: the full-size local-session entry appended to `data_seen` of all seven predictions (P-pc keeps its earlier entry too, so it has two).
- C3: `preflight_runtime.measured_on` names `checks/spec-dev-full-T60.json` and the 230 s source (v6 qualification launch 04:37:46Z-04:41:32Z).
- C4: `literature[3].claim` as worded.
- C5: `links.requests` and `extensions.requests` carry the two `-r2` requests (hashes above); `invalidation_rules[2]` extended with the rerun sentence.
- C6: `randomization.blinding.how` as worded.
- C7: `extensions.execution_route` added (kind runbook, steps S01-probe, S02-trials, S03-verify, controls, replay, on_failure, fallback).
- Unchanged per the section: `execution.retries`, `measurement.order`, the exit-code stopping rule. Other hashes unchanged (specs d2af59dd... / 688ec9a0..., script 66aa2ffd..., first-attempt requests).

**protocol.json sha256: `a2ad290045ebcd867f780c45a58d842ec2c9cfda7445256ccdf51ba9f9bd5b31`** (previous `5d83e72e...`). `freeze.status` "draft".
