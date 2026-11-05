# Coordinator verdict: P32, the P28 seal, reruns and the final report (XP-20261029-packed-filter)

**Date:** 2026-10-29, after 23:50Z. **Role:** coordinator. **Scope:** (A) the P32 size verdict under PROTOCOL §6; (B) the seal on the P28 wall session and the procedure that follows it; (C) reruns; (D) what the study's final report may and may not claim; (E) the 20-bit rerank that DEC-20261029-be9ff6 scheduled for this decision. This verdict launches nothing, amends neither `PROTOCOL.md` nor any card, and changes no hypothesis or goal status. The study has no `H-*` or `EXP-*` record (card-analysis header: "not an EXP-* record").

**Decision ID:** `DEC-20261029-5eb8e6`, with the validator handoff `TASK-20261029-654f73`. Neither id is allocated yet: I had no shell in this engagement. The committing session allocates and checks both (see "Decision record").

**Binding texts:**
- `backlog/XP-20261029-packed-filter/PROTOCOL.md`, sha256 `703bde26…7e51` (freeze commit `f870408c17`): §4, §5, §6.1, §6.2, §7, §9, §10;
- `APPROVAL.md`, sha256 `0a35923c…6a40`;
- `cards/RULING-P28.md`, sha256 `bff4e46d…4275`: (a) Seal, (b) B5 and B6, (c) C6;
- `cards/RULING-P28-ADDENDUM-1.md`, sha256 `8bac4596…46da`: (A), (B) D2′, (C);
- `cards/card-analysis.md`, sha256 `69853b64…ff6e`: Parameters, steps 2, 4 and 5;
- `cards/card-launch-v2.md`, sha256 `8fa7610f…d526`: Procedures V and S, "Never retry".

**What I read for this verdict:**
- the texts above;
- `cards/tools/session_facts.py`, `receipt.py`, and the `STOP` conditions of `wall_gate.py`, `records_checks.py`, `work_checks.py`, `sprime.py`, `positions.py`, `phase_sums.py`, `project_compare.py` and `project_facts.py`;
- P32 wall: `analysis/wall_gate.json`, `wall_gate-stdout.txt`, `compare-projection.txt`, `records_checks.json`, `work_checks.json`, `sprime.json`, `phase_sums-stdout.txt`, `positions-stdout.txt`, `receipt.json`;
- P32 counted: `analysis/counted_gate.json`, `sprime.json`, `work_checks.json`, `receipt.json`;
- P24: `wall/analysis/wall_gate.json`, the decision and position-control lines of `wall_gate-stdout.txt`, the `lines` blocks of `wall/analysis/records_checks.json` and `counted/analysis/counted_gate.json`, and `counted/analysis/sprime.json`;
- `docs/session-logs/2026-10-29.md`, entries 18:45Z to 23:50Z;
- `backlog/XP-20261029-fb20-larger-n/DECISION.md`;
- crypto-autoresearcher `agents/coordinator.md`, `templates/research-records.md` (decision and handoff templates), and crypto `docs/VALIDATION_GATE.md`.

I opened no `records.jsonl` of any session. I opened no P28 wall file: the only P28 paths I touched are the names returned by a file listing of `P28/` (`P28-HOSTLOSS.md`, `receipts-P28-rho-r2/`, `s3verify-P28/`).

**Limits of this verdict.**
- No shell: I computed no sha256. The receipt digests in the dispatch (wall `59dfd56b…2635`, counted `5fa0b0b4…0eae`) and the card digest are as reported to me, not checked by me. The per-file sha256 inside the two `receipt.json` files are what this verdict cites.
- I cannot see whether the P32 analysis outputs are committed. Nothing in this verdict is official until they are (see "What makes this verdict effective").

---

## Facts used (verified in the files)

**P32 wall session** `ic-dgbf-p32-20261029`, pod `f4a59943-f564-48d5-ac7c-296929d99a54`, node ip-10-84-75-59 (`P32/wall/analysis/receipt.json`):
- `job-exit.json` exit 0; one pod-uid prefix; the pod uid equals the prefix.
- Audit `ok`, session `complete`, 0 problems, 1,152 of 1,152 replays `identical`, 1,344 verified records.
- L2 gate accepted, 1,344 of 1,344 executions at L2, `below_l2` and `invalid_outcomes` empty.
- `build-info.json` names `3271a25d02…` and ecbench `21701be6…4503`; `binary.sha256` agrees; the input spec is JSON-equal to the frozen `spec-P32.json` (sha256 `9f950f3b…30ab`). The analysis binary hashes to `21701be6…4503` as well. It is the image's binary, recovered by digest at 22:45Z.
- `wall_gate.json`: `W_COND1` to `W_COND10` true, `W_ALL_1_TO_10` true (and its rounded-to-3 twin), `CORRECTED_CI95_UPPER_LT_1` true, `CORRECTED_CI95_LOWER_GT_1` false.
- **Decision block** (`compare-dgb__dgbf-w0.json`, `resample_seed` 20261002, `linearised_t_pairs`, 288 pairs, 0 below level, `self_aa` false, `aa_arms` `["dgb","dgb-aa"]`, `aa_selection` "includes arm A", `aa_pairs` 288, status `admitted`):
  - ns per GAE: `dgb` 84.5526, `dgbf` 81.0446;
  - `ratio_b_over_a` 0.958512 [0.956084, 0.960945]; it decides nothing;
  - `aa_ratio` (`dgb-aa / dgb`) 0.998337 [0.996146, 1.000534];
  - **`corrected_ratio` 0.960107806 [0.956898, 0.963328]**, `corrected_dof` 562.6, `outside_noise` true;
  - the w = 0.1 and w = 1 files carry the identical wall block.
- **Condition 9.** `compare-dgb-aa__dgbf-w0.json` `ratio_b_over_a` = 0.960107806240821, equal to the decision file's `corrected_ratio` (relative difference 0.0). The corrected ratio is therefore Σ wall(`dgbf`) / Σ wall(`dgb-aa`) over arms at the same block positions.
- **A/A controls.**
  - Counted: `ops.ratio_b_over_a` is exactly 1.0, with CI [1.0, 1.0], at w = 0, 0.1 and 1, for both `dgb`/`dgb-aa` and `dgbf-edge`/`dgbf`. That is what "the A/A arms are exactly 1.0" means.
  - The wall A/A contrasts are not 1.0 and are not expected to be: `dgb-aa / dgb` 0.998337 [0.996146, 1.000534] and `dgbf / dgbf-edge` 0.999197 [0.997648, 1.000749]. Both intervals contain 1.
- **Native A/A admission (condition 6).** `control-comparison.json` (`--a dgbf-edge --b dgb-aa`): admitted, 288 pairs, 0 below level, `aa_pairs` 288, `aa_arms` {`dgbf-edge`, `dgbf`}, "includes arm A", `self_aa` false.
- **Positions** (`positions-stdout.txt`): `dgb-aa` and `dgbf` each sit 144 blocks at position 1 and 144 at position 2; `dgb` and `dgbf-edge` each sit 144 at position 0 and 144 at position 3.
- **Condition 8.** Counted `ops` `dgbf / dgb` is exactly 1 at all three weights. `records_checks.json`: every field of the record half matched for `dgbf` and `dgbf-edge` against `dgb` (336 matched each, 0 mismatches in any field). `table_filter_bit_sets` is absent on every `dgb` and `dgb-aa` record and present once, in `oracle_setup`, on every `dgbf` and `dgbf-edge` record. Its values are 1,048,276 on `icv1-fp32-t89141-95879fe3` and 1,048,295 on `icv1-fp32-tm68303-f902d2f0`, one value per curve. In `work_checks.json` the bitop identity holds on all 336 records of each filtered arm.
- **Condition 10.** p ≤ 3,965,125,361 and at most 2,048 signed points (`record.signed_points` on all 1,344 records).
- **bitop/v1** over the measured records: `dgbf / dgb` = 1.0030127, i.e. +0.301%. The difference, 115,931,990,016 bitops, equals 384 × 144 × (1,048,276 + 1,048,295).
- **Phase sums** (288 measured records; `dgb-aa` against `dgbf`, the pair at matched positions):
  - `time.solve_wall_ns`: 48,192.2 ms against 46,269.7 ms (−3.99%);
  - `target_PDP`: 14,047.8 ms against 10,413.1 ms (−25.9%);
  - `oracle_setup`, which includes the filter build: 6,396.3 ms against 8,167.2 ms (+27.7%, +1,770.9 ms, which is 3.67% of `dgb-aa`'s solve wall);
  - `target_query`: 26,776.7 ms against 26,765.8 ms (unchanged).
- **S′ for `dgb`→`dgbf`** (`P32/wall/analysis/sprime.json`, 288 pairs, `S_PRIME_EXACT` true): 1.0 at w = 0, 1.050488 at w = 0.1, 1.358633 at w = 1. Per curve the values are 1.0500 and 1.0509 at w = 0.1, and 1.3547 and 1.3626 at w = 1. Without the bit-set term the ratio is 1.0 at every weight, equal to `compare --weight` to within 1e-9.

**P32 counted session** `ic-dgbf-rho-p32-20261029`, pod `c3786aa2-dc91-438f-912b-cd5552b7b5f3` (`P32/counted/analysis/receipt.json`, `counted_gate.json`):
- `job-exit.json` exit 42, one pod; the launcher's `result.json` records exit 23, as it did at P24-rho and P28-rho-r2.
- Audit `ok`, complete, 1,152 of 1,152 replays identical.
- `GATE_COND1` to `GATE_COND6` true. `EXIT42_CLAUSE` true and `EXIT43_CLAUSE` false.
- `below_l2` has 14 entries: 7 on `rho-neg` and 7 on `rho-neg-aa`, all L1, all "solve too short to admit wall time" (257.0 to 968.9 µs), all waived. 0 IC executions are below L2, and `invalid_outcomes` is empty.
- The `rho-neg`/`rho-neg-aa` A/A `ops` ratio is exactly 1.0 at all three weights.
- `records_checks.json`: `R_COUNTED_EQUALS_WALL` true (336 matched for each of `dgb` and `dgbf`, `table_filter_bit_sets` included, 0 mismatches) and `R_COND10_COUNTED` true. The counted bitop identity holds (`work_checks.json`).
- **S′ ratios** (`P32/counted/analysis/sprime.json`, B over A, 288 pairs):

| pair | w = 0 | w = 0.1 | w = 1 | per curve (t89141 / tm68303) at w = 1 |
|:--|--:|--:|--:|:--|
| `dgb` / `rho-neg` | 31.4719 | 29.9858 | 23.2775 | 21.81 / 24.97 |
| `dgbf` / `rho-neg` | 31.4719 | 31.4997 | 31.6255 | 29.55 / 34.02 |
| `dgbf` / `dgb` | 1.0 | 1.0505 | 1.3586 | 1.3547 / 1.3626 |

  At every weight and on both curves, `rho-neg` is the cheaper arm. The smallest per-curve ratio in the band is 21.81.

**P24** (for comparison; `P24/wall/analysis/wall_gate.json` and stdout, `P24/counted/analysis/`):
- `W_ALL_1_TO_10` true. Corrected `dgbf / dgb` 1.003200 [1.000564, 1.005842], `aa_ratio` 1.000506 [0.999012, 1.002003], `CORRECTED_CI95_LOWER_GT_1` true. The position control `dgbf-edge / dgb` is 1.005304 [1.002677, 1.007938].
- `R_*` lines all true. `EXIT42_CLAUSE` true and `GATE_COND1` to `GATE_COND6` true.
- S′: `dgb / rho-neg` 12.1159 / 11.6498 / 9.4528; `dgbf / rho-neg` 12.1159 / 12.2388 / 12.8184; `dgbf / dgb` 1.0 / 1.0506 / 1.3560.

---

## (A) The P32 size verdict

### What the frozen rule decides

**Ruling: at b = 32 the filter is a wall-clock improvement at N_inc (PROTOCOL §6.2).**

- **§6.1 holds.** Wall conditions 1 to 10 all hold, each read from the job's own files after the `SHA256SUMS` check, as listed above. No noise rerun was involved (see (C)), so "the wall session" is the first launch.
- **The threshold is met.** The upper bound of `wall.corrected_ci95` in the w = 0 file is 0.963328 < 1. The table reading is "improvement: lower wall cost per operation, identical charged GAE, `bitop/v1` higher by the reported ratio". Stated with that ratio: **corrected `dgbf / dgb` 0.9601 [0.9569, 0.9633], i.e. 3.99% less whole-solve wall per charged GAE (95% interval 3.67% to 4.31%), at identical charged GAE, with `bitop/v1` +0.301%.**
- **Position control.** The edge-position estimate `dgbf-edge / dgb` is 0.959282 [0.956531, 0.962040]. That interval overlaps the decision's `corrected_ci95`, so the README makes no block-position statement under §6.2. It reports the two middle-over-edge contrasts given above, both of which contain 1.
- **Counted session.** It satisfies the route through the exit-42 clause. It holds the additional §6.1 equality with the wall records, and condition 10. So it does not block the validation gate at 32, and its `rho-neg` arm is the matched rho reference.
- **No baseline changes** (§6.2). Counted `S` is identical by condition 8, and the headline stays counted `S`. This is a component speed-up of the oracle, not a counted improvement.

### How the §5a band constrains the ranking

- **`dgbf` against `dgb`, counted.** The band never ranks `dgbf` ahead of `dgb`. At w = 0 the two arms tie exactly (S′ ratio 1.0). At w = 0.1 and w = 1, pricing the filter's bit sets on the arm that does them makes `dgbf` 5.05% and 35.86% costlier. The counted order is therefore not strict across the band, and where it is strict it favours `dgb`. The report may claim no counted, GAE or band improvement for `dgbf` over `dgb` at any weight.
- **What the wall claim is.** The wall statistic divides by the identical charged GAE, and the time it measures already includes the bit-set pass (it sits inside `oracle_setup`, +1,770.9 ms). So the 3.99% is net of the filter's own work, measured on this host. The S′ figures say how that same work looks in the counted unit, where one bit set is priced at w GAE. They are a band device, pessimistic at w = 1, and they decide nothing (§5). The two figures always appear together. The wall improvement is never shown without the bitop ratio and the S′ row beside it (§5; APPROVAL E4).
- **Against matched rho.** The order `rho-neg` < `dgb` and `rho-neg` < `dgbf` holds at w = 0, 0.1 and 1, and on both curves. §5a therefore permits the ranking: matched `rho.negation` is cheaper than both IC arms by a factor of 23.3 to 31.6 across the band. `order_holds_across_band` is true for the P32 rho reference.

### Readings that decide nothing

- **Prediction miss.** §7 predicted 0.82 to 0.94 at 32 bits. The whole corrected interval [0.9569, 0.9633] lies above that range, so the gain is smaller than predicted. Two model inputs were wrong, read from the phase sums (descriptive):
  - the set-up rise is 3.67% of the solve wall, not below 2%;
  - `target_PDP`, the phase that carries the probes, is 29% of the solve wall here, not the larger part. `target_query`, at 56%, is untouched by the filter.

  The filter cut `target_PDP` by 25.9%. The predictions decide nothing, and the miss is reported as a model miss. It does not qualify the decision.
- **Pilot.** The L0 pilot ratio of 0.8789 at 32 bits (3 pairs) is not corroborated at L2. It stays descriptive (§3).

### Standing of this verdict

- **The frozen-rule reading is final for this study**, and §9 forbids any amendment that could change it.
- **The official `support` transition waits for validation.** That transition says the result is a validated component speed-up and may enter the leaderboard. It is recorded only after the §10 independent validation reports `pass` (coordinator contract: "A positive result may expand only after an independent verifier passes"). The validator handoff is below.
- **A failed validation** returns the size to me. It is never repaired in prose.

---

## (B) The P28 seal and the procedure that follows

### Successor ranking: "no successor"

RULING-P28 (a) committed me to rank a successor protocol for the 28-bit question at this verdict, and to record either "successor approved for drafting" or "no successor".

**I record "no successor".** The reasons:

- **The value of information is low.** The study's two decidable sizes bracket the effect: a small regression at 24 bits (1.0032) and a 4.0% improvement at 32 bits. At 32 bits the gain fell short of its prediction, and §7 predicted less at 28 bits than at 32. A decision-grade 28-bit point would locate a sign change of a constant cofactor of a few percent. No pending decision turns on that location: the filter stays opt-in (`filter=1`), and nothing proposes making it a default.
- **The roadmap bias.** This is constant-cofactor engineering. Under the exponent-first bias (`docs/target-result-profile.md`), a new protocol, freeze, approval, card set and two EKS jobs for it does not rank.
- **The decision is blind.** It is made before the seal lifts, from the P24 and P32 verdicts alone. Nothing in the P28 wall session influences it. That was the purpose of keeping the seal until now (addendum 1 (C)).
- **It closes nothing beyond this study.** It closes no lane and no lead. A future 28-bit question, for example a size-gated default for the filter, enters through normal intake as a new protocol. It must use fresh curves and seeds (none of `202610296283/4/6/8`), and its §8 must disclose as seen data RULING-P28's (i) to (iii) and the P28 descriptive figures produced under this verdict.

### The seal lifts

- Under RULING-P28 (a), the seal lifts when "no successor" is recorded. Recorded means that **this file and its DEC record have been committed and pushed in a ledger archive and accepted by the post-commit verifier**. Until then the seal stands exactly as written.
- After the lift, the P28 wall session `ic-dgbf-p28-20261029` (pod `cd784143-…`, exit 143) may be processed, **descriptively only**:
  - it stays undecidable (RULING-P28 (a): §6.1 condition 1, and conditions 2, 5 and 6 cannot hold);
  - it is never rerun, never pooled, and enters no table, no `benchmarks/` row and no `VALIDATION.json`;
  - none of its numbers is cited as evidence for or against the filter at 28 bits.
- **README label for P28:** "undecidable (infrastructure: SIGTERM during the in-pod audit, exit 143; no audit, L2 gate or A/A admission); descriptive wall figures processed after the 28-bit question was closed for this study, from per-record isolation grades, without the in-pod replay audit". The C6 disclosures for a launcher-less job apply: no launcher receipt; the image digest was not verified from the pod; the binary is bound by the in-pod check and `S3V_BINARY_SHA` (`P28/s3verify-P28/s3receipt.json`).
- **A local replay is optional.** The validator may replay the P28 wall records locally. Such a replay is labelled descriptive, and it never stands in for the missing in-pod audit (RULING-P28 (a), "No post-hoc cure").

### Procedure for P28 (in this order; the main session dispatches)

1. **Effective lift.** Push the ledger archive that carries this file and its DEC record, and verify it (see "What makes this verdict effective"). Nothing below starts earlier.
2. **B5: which counted session is of record.** It is decided from file existence at the moment of dispatch, before any counted field is read, in one main-session turn with no other dispatch between this step and step 3.
   1. Dispatch card-launch-v2 MODE `VERIFY`, b = 28, UNIT `PAIR`. Read the line `S3_KEYCOUNT ic-dgbf-rho-p28-20261029 <k>`.
   2. If k = 0, the original prefix holds no `job-exit.json`, and **COUNTED is `r2`**.
   3. If k ≥ 1, dispatch MODE `S3VERIFY`, b = 28, UNIT `PAIR`, TAG `P28-rho`, POD_STATUS_SHA256 `NONE`:
      - if it prints `S3V_JOB_EXIT_PRESENT=true`, the original is the session of record and **COUNTED is `first`**. `-r2` is then kept and reported descriptively, never pooled. The frozen card cannot process a second counted session into another directory, so any processing of `-r2` comes back to me for a card decision;
      - if it prints `S3V_JOB_EXIT_PRESENT=false`, **COUNTED is `r2`** (B5: "A prefix that holds objects but no `job-exit.json` stays a host loss");
      - a `STOP:` line, or any other outcome, returns the case to me before any analysis dispatch.

   The expected outcome is k = 0 and COUNTED `r2`: the original prefix listed 0 keys at 21:35Z, the Job deadline has long passed, and B1's bound expired at about 21:13Z.
3. **The analysis dispatch.** card-analysis, with exactly these four values:
   - **b = `28`**
   - **WALL = `first`** (job `ic-dgbf-p28-20261029`, output `P28/wall/`)
   - **COUNTED = `r2`**, or `first` if step 2 decided so (job `ic-dgbf-rho-p28-20261029-r2` or `ic-dgbf-rho-p28-20261029`, output `P28/counted/`)
   - **CARD_SHA256 =** the full sha256 of `card-analysis.md` used on the P24 and P32 dispatches (`69853b64…ff6e`)

   Expected mechanical facts, none of which is a stop condition:
   - Step 2 prints `WALL_JOB_EXIT_CODE ic-dgbf-p28-20261029 143`. 143 is not 42, so the sealing check does not run;
   - Step 4 prints `FILE_PRESENT wall audit.json no`, `… l2-gate.json no` and `… control-comparison.json no`. The card stops only when `session.json` or `records.jsonl` is absent;
   - `wall_gate.py` prints `W_COND1` (and others) false.

   Nobody reads anything into those lines: P28 is undecidable by RULING-P28 (a) whatever they say. The counted session is judged by `counted_gate.py` (`EXIT42_CLAUSE` expected to apply, as at P24-rho and P32-rho), and `R_COUNTED_EQUALS_WALL` compares it with the P28 wall records.
4. **Archive alone.** Commit `P28/wall/` and `P28/counted/` as their own snapshot archive, with large files under the artifact-store rule, and push.
5. **Any `STOP:`** in the analysis dispatch is a `specification_error` returned to me. The director does not improvise.

**What P28 may then report:** the counted distances `dgb / rho-neg` and `dgbf / rho-neg` (S′) at the three weights, as the distance remaining (§6.2: "Every size reports …"), from the counted session of record, provided its §6.1 counted conditions hold. Its wall figures are descriptive, under the label above.

---

## (C) Reruns

**Ruling: no rerun is needed or permitted anywhere in the study. Execution is complete.**

- **P32 wall.** The noise-rerun trigger fails at its condition 1, which needs exit 42 "from a single pod". `job-exit.json` records exit 0. No TRIGGER dispatch is made and no `P32-RERUN.md` is written. A host loss is excluded too, because `job-exit.json` exists. Confirmed: the trigger does not apply.
- **P32 counted.** It exited 42, and it is never rerun:
  - §4: "The counted session is never rerun under this clause";
  - it is not a host loss (`job-exit.json` exists);
  - card v2 "Never retry": "A job that wrote a job-exit.json with a non-zero code is never rerun by any other route".

  Its exit 42 is fully accounted for by the exit-42 clause (`EXIT42_CLAUSE` true; all 14 refusals are waived sub-millisecond rho solves).
- **P24.** The wall exited 0. The counted session exited 42 and passed the exit-42 clause. No rerun.
- **P28.** The wall is never rerun (RULING-P28 (a)). The counted host-loss rerun `-r2` is terminal (addendum 1 (A)), and B6 did not arise.
- **D2′.** Its fallback was not exercised at P32: both launchers wrote `result.json` (W3 `RESULT_JSON_PRESENT` for both tags, 23:50Z). For P32-rho the launcher exited 23, so `verify-build` did not run for it (addendum 1, `run-request` fact). Its image binding rests on the in-pod binary check and `build-info.json`, and the validator states this.

---

## (D) What the final report may and may not claim

### May claim (each with its scope and labels)

1. **At b = 32: a wall-clock component speed-up.** `mitm-prime-blocked:m=2,filter=1` against `mitm-prime-blocked:m=2`, measured as whole-solve wall per charged GAE at N_inc:
   - corrected 0.9601 [0.9569, 0.9633] (−3.99%), at identical charged GAE;
   - `bitop/v1` +0.301%, with the three limits of §5 stated beside it;
   - S′ `dgbf / dgb` 1.0 / 1.0505 / 1.3586 beside it, with the three §5 statements: `dgbf`'s counted cost exceeds `dgb`'s at w > 0 by that ratio; `table_insert_attempts` stays unpriced on every IC arm, identically between `dgb` and `dgbf`, a gap that predates this study; and these figures decide nothing.

   **Scope:** c8a.2xlarge (AMD); image `32144a29…817b`; ecbench `21701be6…4503` from `3271a25d02`; the two §3 curves at 32 bits; N_inc 1,024; `targets = stride-renew-deferred-guard`; `incremental-gauss`; the packed layout (p < 2³², at most 2,048 signed points). One session, 288 pairs.

   **Placement:** the leaderboard component section and a `benchmarks/` row (§10), with the label "whole-solve wall per charged GAE at N_inc, filtered against unfiltered packed table" and the bitop ratio beside it. Both are made **only after** the validation gate holds: a `VALIDATION.json` naming the independent replay of both P32 sessions, the validator report, and the P32 `rho-neg` reference with its §5a band (`order_holds_across_band` true). Until then the README may report the result as "decided under the frozen rule, validation pending".
2. **At b = 24: a small wall regression** (README only): corrected 1.0032 [1.0006, 1.0058]. The predicted 24-bit outcome was "undecidable"; the size was decidable, and the value sits inside the predicted range of 0.97 to 1.01. The phase-sum mechanism of the 19:30Z entry (the probe saving outweighed by the filter build while the table sits in L2) may be given as a descriptive reading.
3. **At b = 28: undecidable for infrastructure reasons.** Use the label of (B), with descriptive wall figures and the counted distances from the session of record.
4. **The distance to matched rho at every size,** from the counted sessions, at w = 0, 0.1 and 1, with S′ for `dgbf`:
   - **P32:** `dgb / rho-neg` 31.47 / 29.99 / 23.28; `dgbf / rho-neg` 31.47 / 31.50 / 31.63;
   - **P24:** `dgb / rho-neg` 12.12 / 11.65 / 9.45; `dgbf / rho-neg` 12.12 / 12.24 / 12.82;
   - **P28:** the session of record, once processed.

   Each figure is labelled as the distance remaining, not a pass/fail gate. `rho-neg` is cheaper than both IC arms at every weight on every curve measured, so §5a permits that ranking. The growth from about 12× at 24 bits to about 31× at 32 bits may be stated descriptively, as a comparison across sessions on different curves.
5. **Descriptive mechanism at 32 bits** from the phase sums: `target_PDP` −25.9%, `oracle_setup` +27.7% (the filter build), and `target_query` unchanged. The prediction miss at 32 bits and its two wrong inputs (set-up share, PDP share) are reported as a model miss.
6. **Across sizes:** "regression at 24 bits, improvement at 32 bits, 28 bits undecidable". It is stated as a list of per-size readings on different curves.

### May not claim

1. **Any counted, GAE or band improvement** of `dgbf` over `dgb`, any baseline change, or any entry in tables A–C. The headline stays counted `S`, identical by construction (§6.2, AGENTS.md §6). Presenting the wall speed-up as a counted gain would be the relabelling the success criterion excludes.
2. **That the filter is free.** It adds native work: +0.301% bitops and, at w > 0, S′ +5.05% and +35.86%.
3. **That `dgbf` closes, or materially narrows, the gap to rho.** In the counted unit `dgbf / rho-neg` is ≥ `dgb / rho-neg` at every weight. No rho wall time and no wall field of any counted session is used (§10).
4. **Anything about 28 bits** beyond "undecidable": no improvement, no null, no regression. That covers any inference from the P28 descriptive figures and any interpolated crossover size presented as measured. "The sign changes between 24 and 32 bits" may not be presented as a located threshold.
5. **Transfer** to other host classes, factor bases, target rules, filter densities, the small-maps layout, larger b, or other curves (§6.2 Scope).
6. **Replication.** Each size is one session. The 32-bit result is "decided, single session", and "validated" once §10 passes. It is never "replicated".
7. **A composition product** `dgbf / dg` with XP-20261029-dg-blocked. §6.2 allows it only where both studies decided an improvement at the same size. dg-blocked decided one only at 28 bits and this study only at 32 bits, so no size qualifies.
8. **The pilot's L0 ratios** as evidence, and any pooling of sessions. That includes P28-rho-r2 with the original counted session if B5 makes the original the session of record.
9. **Any exponent or asymptotic claim.** The class is engineering (constant).

---

## (E) The 20-bit rerank scheduled for this verdict (DEC-20261029-be9ff6)

**Ruling: neither 20-bit residual is admitted now.** The two residuals are (i) the single N 54 challenger of `XP-20261029-fb20-larger-n/DECISION.md` §3 and (ii) the deferred guard at 20 bits.

- **Freezing is not possible.** DECISION.md bars any freeze while a validated 20-bit baseline change waits to merge. I have not verified that PR #2826 has merged since 21:55Z, so I assume that bar still binds.
- **Idle capacity is not a reason to admit.** Both residuals are constant-cofactor work at one toy size with no transfer (DECISION.md §1.4). This study's completion frees execution capacity, and idle capacity does not justify admitting a lane (focus discipline).
- **The order stands:** pipeline first, N second (DECISION.md §1.2). If one is admitted later, it is (ii) before (i).
- **The rerank fires again** when any of these happens, and is recorded as its own decision:
  - PR #2826 merges or closes;
  - a 20-bit deferred-guard result lands;
  - the owner asks for 20-bit tuning.

---

## What makes this verdict effective

The readings above become official only from committed and verified artifacts (AGENTS.md archival rules). The main session runs the archives, each alone:

1. **Snapshot archive:** `research/ic_packed_filter_20261029/P32/` (eks-results and analysis; large files under the artifact-store rule, their sha256 recorded). Push the branch `claude/ic-packed-filter-20261029`, then open or refresh its PR, ready for review with auto-merge on.
2. **Ledger archive:**
   - this file;
   - the DEC and TASK records below, in crypto-autoresearcher `ledger/decisions/` and `ledger/handoffs/`;
   - the session-log entry.

   Push it, open or refresh both PRs (crypto branch; crypto-autoresearcher PR naming `DEC-20261029-5eb8e6` and `TASK-20261029-654f73`), and run the post-commit verifier. **The P28 seal lifts at this push.**
3. **P28:** the procedure of (B), then its own snapshot archive.
4. **Validator:** it works from the committed P24, P28 and P32 directories and writes its own snapshot archive.
5. **After the validator report:** a coordinator evidence review records the official decision for P32 (`support`, scoped as in (D) 1, or a return on validation failure), and the README, figure, PDF, `VALIDATION.json`, leaderboard component and `benchmarks/` row land in one ledger archive and PR.

---

## Decision record (to archive)

I had no shell in this engagement, so I could neither allocate ids nor compute this file's sha256. The session that commits this file must:
- allocate both ids with `python3 tools/allocate_id.py --next coordinator_decision --date 20261029` and `--next handoff --date 20261029`, and confirm each with `--check`;
- replace `<allocate>` in this file's header and in both records with the allocated tokens, in the same commit, before the first push of this file;
- archive the two records below as `ledger/decisions/DEC-20261029-5eb8e6.yaml` and `ledger/handoffs/TASK-20261029-654f73.yaml`;
- record this file's sha256 in the decision record and in the session log;
- run `tools/validate_ledger.py` on the records before pushing.

```yaml
type: coordinator_decision
id: DEC-20261029-5eb8e6
date: 2026-10-29
subject: >
  XP-20261029-packed-filter P32 verdict (improvement under PROTOCOL 6.2, validation pending); P28 successor
  "no successor" and seal lift; no reruns; final-report claim scope; 20-bit rerank of DEC-20261029-be9ff6
context: >
  P32 wall ic-dgbf-p32-20261029 (pod f4a59943-...) exit 0, audit ok 1152/1152 identical, L2 gate 1344/1344,
  W_COND1-10 true; decision block w=0: corrected dgbf/dgb 0.960107806 [0.956898, 0.963328], raw 0.958512,
  aa_ratio 0.998337 [0.996146, 1.000534], 288 pairs; condition 9 identity exact; counted ops A/A exactly 1.0
  at w=0/0.1/1; position control dgbf-edge/dgb 0.959282 [0.956531, 0.962040] overlaps; bitop/v1 +0.301%;
  S' dgbf/dgb 1.0 / 1.0505 / 1.3586. P32 counted ic-dgbf-rho-p32-20261029 exit 42 (launcher 23),
  EXIT42_CLAUSE true, 14 waived sub-ms rho L1 executions, counted records equal wall records; S'
  dgb/rho-neg 31.47 / 29.99 / 23.28, dgbf/rho-neg 31.47 / 31.50 / 31.63; rho-neg cheaper at every weight on
  both curves. P24: regression 1.0032 [1.0006, 1.0058]. P28 wall undecidable (exit 143) and sealed;
  P28 counted -r2 terminal (exit 42), original prefix 0 keys at 21:35Z.
decision: replicate
rulings:
  A: "P32 is a wall-clock improvement at N_inc under PROTOCOL 6.2: 6.1 conditions 1-10 hold and the corrected upper bound 0.963328 < 1. Reading: -3.99% whole-solve wall per charged GAE [3.67%, 4.31%], identical charged GAE, bitop/v1 +0.301%. Counted session satisfies the exit-42 route and the counted=wall equality, so it does not block the gate and its rho-neg is the matched reference. No baseline change; component speed-up only. 5a band: dgbf never ranks ahead of dgb in counted S' (tie at w=0, costlier by 5.05% and 35.86% at w=0.1 and 1); rho-neg ranks ahead of both IC arms at every weight on both curves. Prediction 0.82-0.94 missed (gain smaller); descriptive. Official support waits for the PROTOCOL 10 independent validation (decision 'replicate' = independent recomputation and replay, no new run)."
  B: "'No successor' recorded for the 28-bit question, blind to the sealed session: low value of information for a constant cofactor that the 24- and 32-bit verdicts bracket, no decision depends on the crossover, exponent-first bias. The P28 wall seal lifts when this verdict's ledger archive is pushed and verified. P28 wall is then processed descriptively only (never rerun, pooled, tabled or used as evidence). Procedure: VERIFY b=28 PAIR -> S3_KEYCOUNT ic-dgbf-rho-p28-20261029 k; k=0 -> COUNTED r2; k>=1 -> S3VERIFY b=28 PAIR TAG P28-rho POD_STATUS_SHA256 NONE, S3V_JOB_EXIT_PRESENT=true -> COUNTED first, false -> r2, STOP -> coordinator; then card-analysis b=28 WALL first COUNTED <r2|first> CARD_SHA256 69853b64...ff6e in the same turn; then a snapshot archive of P28/."
  C: "No rerun anywhere. P32 wall exit 0 fails noise-trigger condition 1 and is not a host loss; P32 counted exit 42 is never rerun (4; card v2 Never retry) and is accounted for by EXIT42_CLAUSE; P24 none; P28 wall never (RULING-P28 a); P28-rho-r2 terminal. D2' fallback not exercised at P32."
  D: "Final report may claim: the P32 component speed-up with its scope, bitop ratio and S' row, on the leaderboard component section and in benchmarks/ only after the validation gate; P24 regression (README only); P28 undecidable (infrastructure) with descriptive wall figures; per-size distance to matched rho at three weights (P32 31.47/29.99/23.28 for dgb, 31.47/31.50/31.63 for dgbf; P24 12.12/11.65/9.45 and 12.12/12.24/12.82); descriptive phase mechanism and prediction miss. May not claim: any counted/GAE/band improvement or baseline change; a free filter; a narrower rho gap; anything at 28 bits beyond undecidable or a located crossover; transfer; replication; a dg-blocked composition product (no size decided in both studies); pilot ratios; pooling; any exponent claim."
  E: "DEC-20261029-be9ff6 rerank: neither 20-bit residual admitted. No freeze while PR #2826 is unmerged (not verified as merged here), idle capacity does not admit a lane, both are constant-cofactor; order stays deferred guard before N. Rerank fires when #2826 merges or closes, a 20-bit deferred-guard result lands, or the owner asks."
target_ids: [XP-20261029-packed-filter, LEAD-ICP-prime-ic-constant-engineering, DEC-20261029-be9ff6]
rationale:
  - "PROTOCOL 6.2 decides from wall 6.1 conditions 1-10 and the corrected upper bound alone; wall_gate.json and the decision block show both, and condition 9 makes the statistic the matched-position ratio sum wall(dgbf)/sum wall(dgb-aa)."
  - "Validity was checked before interpretation: run count 1344 per session (336 per arm, 288 measured), audit and replays, L2 grades, seed and spec equality, counted A/A exactly 1, record-level identity of charged work, counted=wall equality, and admission of the native A/A control."
  - "AGENTS.md 5a: ranks only where the order holds across the band. dgb/dgbf is a tie at w=0 and favours dgb at w>0, so no counted ranking of dgbf; rho-neg is cheaper at every weight on both curves, so that ranking is permitted."
  - "Coordinator contract: a positive result expands only after an independent verifier passes; PROTOCOL 10 requires the validator and VALIDATION.json before any leaderboard or benchmarks change."
  - "RULING-P28 (a) fixed the seal's lift to this decision; deciding 'no successor' before reading the sealed session keeps the choice blind. P28 counted analysis needs the wall records, so it follows the lift."
  - "PROTOCOL 4 and card v2 'Never retry' leave no rerun route for any session of this study."
evidence_refs:
  - "backlog/XP-20261029-packed-filter/PROTOCOL.md (sha256 703bde26...7e51) sections 4, 5, 6.1, 6.2, 7, 9, 10; APPROVAL.md (0a35923c...6a40)"
  - "research/ic_packed_filter_20261029/P32/wall/analysis/{wall_gate.json (7e28a6fd...43bf5), wall_gate-stdout.txt (3d3b05ef...02e8), compare-dgb__dgbf-w0.json (e6584542...9ac7), compare-projection.txt (4405b5eb...795b), records_checks.json (3ae5df65...b920), work_checks.json (26055341...fe4b), sprime.json (229b41e7...f2c4), phase_sums.json (18d1b3c8...7f14), positions.json (045c8908...f354), receipt.json}"
  - "research/ic_packed_filter_20261029/P32/counted/analysis/{counted_gate.json (723fc07f...f003), sprime.json (44f09763...57fa), work_checks.json (2362d2a4...fa2c), receipt.json}"
  - "research/ic_packed_filter_20261029/P24/wall/analysis/{wall_gate.json, wall_gate-stdout.txt, records_checks.json}; P24/counted/analysis/{counted_gate.json, sprime.json}"
  - "research/ic_packed_filter_20261029/cards/RULING-P28.md (bff4e46d...4275) (a), (b) B5-B6, (c) C6; RULING-P28-ADDENDUM-1.md (8bac4596...46da) (A)-(C)"
  - "research/ic_packed_filter_20261029/cards/card-analysis.md (69853b64...ff6e) Parameters, steps 2, 4, 5; card-launch-v2.md (8fa7610f...d526) Procedures V and S, Never retry"
  - "docs/session-logs/2026-10-29.md entries 18:45Z-23:50Z (launch, WAIT and analysis facts as reported by the main session)"
  - "backlog/XP-20261029-fb20-larger-n/DECISION.md sections 1.2, 1.4, 2, 3 (rerank inputs)"
limitations:
  - "No shell: no sha256 computed by the coordinator; receipt digests (wall 59dfd56b...2635, counted 5fa0b0b4...0eae) and the card digest are as reported. File sha256 cited above are read from the receipts."
  - "No records.jsonl was opened; record-level facts are the frozen helpers' outputs, to be recomputed by the validator with its own code."
  - "Commit state of the P32 analysis outputs was not visible; the verdict is effective only after the snapshot and ledger archives are pushed and verified."
  - "P32-rho's launcher exited 23, so verify-build did not run for it; its image binding rests on the in-pod binary check and build-info.json."
  - "Single session per size; c8a.2xlarge only; two curves per size. The S' bit-set pricing is a band device and pessimistic at w=1."
  - "PR #2826 merge state not verified in this engagement (E assumes the freeze bar still binds)."
knowledge_promotion:
  promoted: []
  not_warranted: "Not a support or reject_scoped decision: the P32 improvement is one unreplicated session pending PROTOCOL 10 validation, a constant-cofactor component result. Reassessed at the post-validation evidence review; the expected destination then is the lead atlas entry, not a KN-FIND, unless the result is replicated."
decided_by: coordinator
decided_at: 2026-10-29
next_actions:
  - "Main session: snapshot archive of research/ic_packed_filter_20261029/P32/ alone; push; open or refresh the crypto PR (ready for review, auto-merge on)."
  - "Main session: allocate and check DEC and TASK ids; ledger archive of cards/VERDICT-P32.md, the DEC and TASK records and the session-log entry; push, refresh both PRs, run the post-commit verifier. The P28 seal lifts at this push."
  - "Main session: P28 procedure of VERDICT-P32 (B): VERIFY b=28 PAIR (and S3VERIFY TAG P28-rho only if S3_KEYCOUNT >= 1), then in the same turn card-analysis b=28 WALL first COUNTED r2 (or first per B5) CARD_SHA256 69853b64...ff6e; any STOP returns to the coordinator; snapshot archive of P28/ alone."
  - "Validator (TASK-20261029-654f73): PROTOCOL 10 independent validation of P24, P28 (counted of record; wall descriptive) and P32, with replay certificate and validator report."
  - "Coordinator, after the validator report: evidence review of P32 (support scoped to VERDICT-P32 (D) 1, or return on failure), with knowledge_promotion; then the README, figure, PDF, VALIDATION.json, leaderboard component and benchmarks row in one ledger archive; update LEAD-ICP-prime-ic-constant-engineering with the per-size verdicts."
  - "Main session: if this study is attached to an active GOAL, checkpoint it after the ledger archive with exactly one next action: the PROTOCOL 10 validation."
  - "Coordinator: 20-bit rerank of DEC-20261029-be9ff6 when PR #2826 merges or closes, a 20-bit deferred-guard result lands, or the owner asks."
```

```yaml
type: handoff
id: TASK-20261029-654f73
date: 2026-10-29
from: coordinator
to: validator
objective: >
  Independent validation (PROTOCOL.md section 10) of XP-20261029-packed-filter: recompute with own code the
  P32 decision and every section 6.1 wall and counted condition; recheck P24 the same way; recheck the P28
  counted session of record and process nothing of the P28 wall beyond descriptive figures; produce the
  replay certificate and the validator report that the validation gate needs for the P32 rows.
uncertainty_reduced: >
  Whether the P32 improvement (corrected dgbf/dgb 0.960107806 [0.956898, 0.963328]) and its instrument
  conditions survive recomputation by independent code and an independent replay, which gates the leaderboard
  component row and the benchmarks row.
inputs:
  - "backlog/XP-20261029-packed-filter/PROTOCOL.md and APPROVAL.md at freeze f870408c17 (worker context: sections 5, 6.1, 6.2, 10)"
  - "research/ic_packed_filter_20261029/cards/VERDICT-P32.md (worker context: Facts used, (A), (D); the numbers to recompute)"
  - "research/ic_packed_filter_20261029/P32/{wall,counted}/eks-results/ (raw input: records.jsonl, job-exit.json, audit.json, l2-gate.json, control-comparison.json, build-info.json, input-spec.json, SHA256SUMS)"
  - "research/ic_packed_filter_20261029/P24/{wall,counted}/eks-results/ (same files)"
  - "research/ic_packed_filter_20261029/P28/{wall,counted}/ after the VERDICT-P32 (B) procedure, P28/s3verify-P28/s3receipt.json, P28/receipts-P28-rho-r2/, P28/P28-HOSTLOSS.md"
  - "research/ic_packed_filter_20261029/cards/RULING-P28.md (worker context: (a) and C6) and RULING-P28-ADDENDUM-1.md (worker context: (B) D2')"
  - "ecbench binary sha256 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503 (image 32144a29...817b) for the replay only"
constraints:
  - "Own code only for every recomputed figure; do not import or copy research/ic_packed_filter_20261029/cards/tools/ or reuse the analysis/ outputs as inputs (they are the claims under check)."
  - "Read no wall field of any counted session, and no rho wall time (PROTOCOL 10)."
  - "P28 wall is descriptive only: no figure from it enters the report as decision evidence; a local replay of it, if made, is labelled descriptive and never stands in for the missing in-pod audit."
  - "No EKS job, no relaunch, no change to any frozen file, card, spec or request; local replay only."
  - "A defect is reported with the record keys and values; it is never repaired."
deliverables:
  - "research/ic_packed_filter_20261029/validation/validator-report.json (status pass|fail, independent: true, raw_data with sha256, headline list of {name, claimed, recomputed, rel_tol})"
  - "research/ic_packed_filter_20261029/validation/replay-certificate.json (status, independent: true, scalars_verified, runs) for both P32 sessions and both P24 sessions"
  - "research/ic_packed_filter_20261029/validation/REPORT.md (each joint below, its method, result, and the descriptive per-curve and target-clustered intervals and position tables)"
  - "research/ic_packed_filter_20261029/validation/code/ (the validator's own scripts)"
artifact_paths:
  - research/ic_packed_filter_20261029/validation/
archival_task: "one snapshot archive of research/ic_packed_filter_20261029/validation/ only"
inference:
  policy: review-adversarial
  independent_session_required: true
  fallback_allowed: false
  degraded_allowed: false
  delivery: interactive
budget: {}
completion_gate:
  - "validator-report.json and replay-certificate.json exist, conform to docs/VALIDATION_GATE.md, and `python3 scripts/validation_gate.py evidence <dir>` explains them without error once VALIDATION.json is written"
  - "every headline number of VERDICT-P32 (A) and every 6.1 condition for P24 and P32 is marked reproduced or not reproduced, with the recomputed value"
  - "the snapshot archive of validation/ is committed, pushed and accepted by the post-commit verifier"
return_format: "validator report path and sha256, replay certificate path and sha256, overall pass|fail, and the list of defects (empty on pass)"
review_plan:
  claim_under_review: >
    At b = 32, with section 6.1 wall conditions 1-10 holding, the w = 0 decision file gives corrected dgbf/dgb
    ns per GAE 0.960107806 with corrected_ci95 [0.956898, 0.963328] (upper bound < 1, improvement); the counted
    session satisfies the exit-42 clause and equals the wall records; S' dgbf/dgb 1.0 / 1.0505 / 1.3586 and
    dgb/rho-neg 31.47 / 29.99 / 23.28, dgbf/rho-neg 31.47 / 31.50 / 31.63.
  coordinator_prior: >
    Pass. The decision statistic and condition 9 reproduce to 1e-9, and condition 8 holds record by record. If a
    defect exists, I expect it in the S' reconstruction of band.rs's priced GAE, or in the P28 counted/wall
    equality, not in the P32 decision. The disclosure that verify-build did not run for P32-rho (launcher exit 23)
    will be a caveat, not a failure.
  joints:
    - joint: "J1 decision statistic: ns per GAE per arm, ratio_b_over_a, aa_ratio, corrected_ratio and corrected_ci95 (pairs keyed by curve, target, round; L2 or L3 both runs; the section 5 independent-SE combination), and condition 9"
      assigned_to: TASK-20261029-654f73
      attack_plan: "Rebuild the 288 pairs from records.jsonl; check corrected_ratio equals sum wall(dgbf)/sum wall(dgb-aa) exactly; recompute the interval by the linearised-t method and, descriptively, by target-clustered and per-curve bootstraps. A wrong pairing key or an inadmissible pair is where it would break."
      breaking_artifact: "a recomputed corrected upper bound >= 1, or any headline mismatch > 1e-9"
    - joint: "J2 instrument: audit, replays, L2 grades from the records, input-spec equality, block positions (dgbf and dgb-aa 144 at each of positions 1 and 2) and predecessors"
      assigned_to: TASK-20261029-654f73
      attack_plan: "Tabulate positions and predecessors from seq and round; re-grade from each record's isolation block; confirm no admitted record carries an L2 blocker."
      breaking_artifact: "a position imbalance between dgbf and dgb-aa, or an admitted pair with a run below L2"
    - joint: "J3 charged-work identity: condition 8 record half (warm-ups included), table_filter_bit_sets requirements, bitop difference = 384 x table_filter_bit_sets in oracle_setup, condition 10, and counted records equal to wall records"
      assigned_to: TASK-20261029-654f73
      attack_plan: "Diff every listed field of each dgbf/dgbf-edge record against the matched dgb record; run ecbench work independently and check the bitop difference phase by phase."
      breaking_artifact: "any field difference, a counter on the wrong arm or phase, or a bitop difference not equal to 384 x the counter"
    - joint: "J4 counted route and band: exit-42 clause from l2-gate.json and from records, rho A/A exactly 1, S' at w = 0, 0.1, 1 for dgb/dgbf, rho-neg/dgb, rho-neg/dgbf, and the 5a order per curve"
      assigned_to: TASK-20261029-654f73
      attack_plan: "Re-derive priced GAE at w from cost.unpriced as band.rs defines it, add w x table_filter_bit_sets, divide by sqrt(r) per record; check the order rho-neg < IC on each curve at each weight."
      breaking_artifact: "an S' ratio differing beyond 1e-9, or an order reversal at some weight or curve"
    - joint: "J5 independent replay of the P32 and P24 sessions with the image binary on a host other than the pod"
      assigned_to: TASK-20261029-654f73
      attack_plan: "Replay every measured record (warm-ups too, if time allows); compare recovered logarithms and scalars to the records."
      breaking_artifact: "a replay that does not reproduce a recorded scalar or logarithm"
  blindness:
    mutual: true
    lifted_for: []
    rationale: null
  proves_too_much:
    objects:
      - "P32 A/A pairs dgb vs dgb-aa and dgbf-edge vs dgbf (identical methods)"
      - "P24 wall session, where the frozen rule reads a regression"
    failure_signature: "The validator's code must give intervals containing 1 on both A/A pairs, and must reproduce a corrected lower bound > 1 at P24; code that finds a filter improvement on an A/A pair or at P24 has proved too much."
    assigned_to: TASK-20261029-654f73
  blind_rederivation:
    required: true
    quantity: "corrected wall ratio dgbf/dgb at P32: (sum solve_wall(dgbf)/sum total_gae(dgbf)) / (sum solve_wall(dgb)/sum total_gae(dgb)), divided by the dgb-aa/dgb A/A ratio, over pairs whose two runs are at L2 or L3, with a 95% interval treating the two standard errors as independent"
    parameters: "research/ic_packed_filter_20261029/P32/wall/eks-results/session/records.jsonl; arms dgb, dgb-aa, dgbf; pairing by curve, target and round; measured rounds only"
    blind_from:
      - research/ic_packed_filter_20261029/cards/tools/
      - research/ic_packed_filter_20261029/P32/wall/analysis/
      - research/ic_packed_filter_20261029/P24/wall/analysis/
    assigned_to: TASK-20261029-654f73
  procedure_deviations: []
```
