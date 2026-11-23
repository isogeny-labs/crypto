# Coordinator evidence review: P32 after the PROTOCOL §10 validation (XP-20261029-packed-filter)

**Date:** 2026-10-30. **Role:** coordinator. **Scope:**
1. the evidence-review decision for P32 that VERDICT-P32 (DEC-20261029-5eb8e6, decision `replicate`) deferred to the validator's report;
2. the 1.0500 truncation in VERDICT-P32;
3. a defect in the validator's report that the validator did not report;
4. what the final report, `VALIDATION.json`, the leaderboard component section and the `benchmarks/` row may state;
5. the text of the `LEAD-ICP-prime-ic-constant-engineering` update;
6. the 20-bit rerank of DEC-20261029-be9ff6.

This review launches nothing, amends neither `PROTOCOL.md` nor any card, and edits no archived file. The study has no `H-*` or `EXP-*` record, so no hypothesis status changes.

**Decision ID:** `DEC-20261030-04fe6d`. The two corrections are `CORR-20261030-1e501e` (A) and `CORR-20261030-1153f8` (B) and the publication handoff is `TASK-20261030-7f570f`. I had no shell, so none of these ids is allocated. The main session allocates and checks each one (see "Decision record").

**Binding texts:**
- `backlog/XP-20261029-packed-filter/PROTOCOL.md`, sha256 `703bde26…7e51` (freeze `f870408c17`), sections §1, §2, §5, §6.1, §6.2 and §10;
- `cards/VERDICT-P32.md`, sha256 `415a89d7…9ea3` (as `validation/REPORT.md` records it): (A), (B) "What P28 may then report", (D) and (E);
- `TASK-20261029-654f73` (its review_plan, as embedded in VERDICT-P32);
- crypto `docs/VALIDATION_GATE.md`.

**What I read for this review:**
- `validation/REPORT.md`, in full;
- `validation/replay-certificate.json`, in full;
- `validation/validator-report.json`, in part:
  - its top-level keys;
  - the `headline` list (lines 1–200 and 1100–1115), with every `reproduced` flag counted by search;
  - `id`, `joints`, `launcher_requests`, `p28_counted_session_of_record`, `proves_too_much`, `raw_data` and `s3_copy_check` (lines 1115–1510);
  - the tail (lines 2286–2310);
  - line 2, `c2_identity_facts`;
- `validation/code/`:
  - `build_report.py`, lines 1–60 and 110–314;
  - `write_report_md.py`, lines 1–40 and line 165 by search;
  - `out/c2_identity.json`, in full;
  - `out/j4.json`, the `P32-counted` block (lines 866–1025) and the `mean_Sprime_*` keys by search;
  - `out/j1-inpod-crosscheck.json`, its `max_rel_diff` keys;
- `PROTOCOL.md` lines 1–62 and 110–309;
- `scripts/validation_gate.py`, lines 140–241;
- `scripts/build_ic_leaderboard.py`, lines 745–907, with its component sections located by search;
- in crypto-autoresearcher:
  - `agents/coordinator.md`;
  - `templates/research-records.md` (decision, correction, evidence strength);
  - `AGENTS.md`, "Mandatory progress-fidelity gate";
  - `docs/progress-fidelity.md`;
  - `knowledge/leads/ecdlp/ic-prime/LEAD-ICP-prime-ic-constant-engineering.yaml`;
  - `ledger/corrections/CORR-20261029-1aaef6.yaml` (precedent);
  - the record types in `tools/allocate_id.py` and the `LEVELS` set in `tools/progress_fidelity.py`.

I opened no `records.jsonl`, no `analysis/` file, no wall field of a counted session, and no file of the P28 wall session.

**Limits of this review.**
- No shell: I computed no sha256 and saw no git state. The digests below are the validator's, or as dispatched to me. Two inputs are as reported to me by the main session: the snapshot at commit `852515114`, and the post-commit verifier's acceptance of it.
- PR #2826's state (open, not merged) is as reported to me. I did not check it.
- `benchmarks/README.md` is not materialized in this sparse worktree. Its column layout is therefore not prescribed here; see (3.4).

---

## Facts verified (in the validator's files)

- **Status.**
  - `validator-report.json`: `status` "pass", `independent` true, `defects` [] and `section_6_1_all_reproduced` true. `joints` J1–J5 and `proves_too_much` all read "holds".
  - `selftest_negative_controls.all_pass` is true. All eleven perturbations fire.
- **Headline.** It has 122 entries, all `reproduced: true`. The only `reproduced: false` in the file is the separate `descriptive_discrepancies[0]`, the 1.0500 figure.
- **J1, the blind re-derivation.**
  - P32 corrected `dgbf / dgb` 0.960107806240821 [0.9568981834, 0.9633281948], dof 562.64.
  - Condition 9 holds exactly: Σwall(`dgbf`)/Σwall(`dgb-aa`) 0.9601078062408209, and the GAE sums are equal.
  - P24 corrected 1.003199518 [1.000564415, 1.005841562].
  - The in-pod `control-comparison.json` reproduces with `max_rel_diff` 2.25e-16 (P32) and 4.15e-16 (P24). The "4.2e-16" of the summary is the larger value, rounded.
- **J2, the instrument.**
  - Both wall sessions: 1,344/1,344 records re-graded, 0 disagreements, all records L2.
  - `dgbf` and `dgb-aa` sit 144 times at position 1 and 144 at position 2. Each has 144 filtered and 144 unfiltered predecessors.
  - The algorithm seed is shared within each block, and the seeds are the §3 seeds.
  - The exit-42 clause holds for the P32, P24 and P28-r2 counted sessions. The below-L2 set derived from the records equals `l2-gate.json`.
- **J3, the charged work.**
  - 336 matched records per filtered arm, and 0 mismatches, at both sizes.
  - The bitop identity holds on 672/672 filtered records per session, with the difference in `oracle_setup` only. At P32 the difference is 384 × 144 × (1,048,276 + 1,048,295) = 115,931,990,016.
  - Counted records equal wall records at P32, P24 and P28.
  - Condition 10 holds: P32 p ≤ 3,965,125,361 and at most 2,048 signed points.
- **J4, the band.**
  - P32 counted S′, w = 0 / 0.1 / 1:
    - `dgb / rho-neg` 31.4719 / 29.9858 / 23.2775;
    - `dgbf / rho-neg` 31.4719 / 31.4997 / 31.6255;
    - `dgbf / dgb` 1.0 / 1.050488 / 1.358633.
  - `strict_same_order_across_band_every_curve` is true for both IC/rho pairs at P32, P24 and P28, and false for `dgbf/dgb` (a tie at w = 0).
  - P28-r2 (`p28_counted_session_of_record.sprime`):
    - `dgb / rho-neg` 21.044850 / 20.019054 / 15.337880;
    - `dgbf / rho-neg` 21.044850 / 21.116316 / 21.442445;
    - `dgbf / dgb` 1.0 / 1.054811 / 1.398006.
- **J5, the replay.**
  - `replay-certificate.json` has `status` "verified", `scalars_verified` true and `independent` true.
  - Four required runs (P32 wall and counted, P24 wall and counted): each 1,152/1,152 identical with 0 problems, and the validator's own scalar check holds on 1,344/1,344 records, warm-ups included.
  - The replay host is an Intel Xeon VM, of a different class from the pods' AMD EPYC 9R45.
  - P28-counted is an additional run. P28-wall is a descriptive run, labelled `descriptive_only: true`.
- **Proves-too-much control.**
  - Both P32 A/A intervals contain 1, and the P24 corrected lower bound is above 1.
  - In the label-swap null on (`dgb-aa`, `dgbf`), the null median is 0.99982, 0.99999 and 0.99999 at n = 72, 144 and 288. Its 95% band narrows from ±0.0095 to ±0.0053.
  - The observed ratio stays at 0.962, 0.959 and 0.960, with a null fraction ≤ observed of 0.0 at every n.
- **Blindness.** `not_read` lists `cards/tools/`, every `analysis/` directory, `compare.rs`, every counted-session wall field, every rho wall time and every P28 wall field. `blind_from_respected` is true.

**The coordinator prior** (review_plan) was "pass", with any defect expected in the S′ reconstruction or the P28 counted/wall equality, and the verify-build disclosure expected as a caveat.
- The pass and the caveat came out as I expected. My prior concurs with the outcome; it was recorded before the report.
- The one discrepancy found is an S′ figure, but in my own prose, not in the reconstruction.
- The defect in (2b) was not anticipated by the prior.

---

## (1) The evidence-review decision for P32

**Ruling: `support`, scoped exactly to VERDICT-P32 (D) item 1.**

The supported statement: at b = 32, `mitm-prime-blocked:m=2,filter=1` against `mitm-prime-blocked:m=2`, measured as whole-solve wall per charged GAE at N_inc:
- corrected 0.9601 [0.9569, 0.9633], that is 3.99% less wall per charged GAE (95% interval 3.67% to 4.31%);
- identical charged GAE;
- `bitop/v1` +0.301%;
- S′ `dgbf / dgb` 1.0 / 1.0505 / 1.3586 at w = 0 / 0.1 / 1.

It is a component speed-up of the oracle. It is not a counted, GAE, band or baseline improvement.

**Scope:**
- host class c8a.2xlarge (AMD);
- image `32144a29…817b`, ecbench `21701be6…4503` from `3271a25d02`;
- the two §3 curves at 32 bits, `icv1-fp32-t89141-95879fe3` and `icv1-fp32-tm68303-f902d2f0`;
- N_inc 1,024; `targets = stride-renew-deferred-guard`; `incremental-gauss`; the packed layout;
- one wall session, 288 pairs;
- claim tier `toy`.

**Not transferred** (PROTOCOL §6.2 "Scope"; VERDICT-P32 (D) "may not" 5): other host classes, factor bases, target rules, filter densities, the small-maps layout, larger b, or other curves.

**Validity before interpretation.** These are the items checked, each by the validator's own code; none fails.
- **Run count.** 1,344 executions per session, 336 per arm, 288 measured pairs.
- **Manifests.** `SHA256SUMS`, the artifact store and the S3 copies are byte-equal. Each job has one pod prefix, equal to `job-exit.json`.
- **Seeds and specs.** The seeds are the §3 seeds, and the input specs are JSON-equal to the frozen specs and to every launcher request.
- **Raw against summary.** Every claimed figure is recomputed from `records.jsonl`, and the in-pod comparison agrees to 4.2e-16.
- **Controls.**
  - The counted A/A is exactly 1 at every weight, and the wall A/A intervals contain 1.
  - The native A/A admission conditions hold.
  - The positions are balanced.
  - The label-swap null behaves as a null, and the observed ratio as an effect.

**Why `support` and not a return.**
- All five joints hold. The breaking artifact each joint was assigned was sought and not found.
- The decision statistic was re-derived blind, and condition 9 holds exactly.
- The proves-too-much control has the signature a correct method must show.
- The two items in (2) and (2b) are a printed truncation and a mislabelled field. Neither touches any joint, any headline, any §6.1 entry or any field `validation_gate.py` reads.

**The caveats, and why none blocks.**
1. **`ecbench work` was not run.** §10 asks the validator to check "the bitop clauses" with its own code. It did so: it re-priced each record's in-pod `work` block from primitive counts under its own reading of `BITOPS.md`, and checked the 384 × counter identity phase by phase. That is independent of the producer's `ecbench work` call, not weaker than it. The producer's `ecbench work` totals remain the reported §5 figures.
2. **Warm-ups were not replayed.** The binary's replay modes refuse warm-ups. They are covered instead by the validator's own [d]G = Q check on 1,344/1,344 records, and by the counted session's second-pod execution of every `dgb` and `dgbf` input. Condition 8 already pairs warm-ups record by record. No claim rests on a warm-up's wall time.
3. **The image digest of the three counted jobs.** For P32, P24 and P28-r2 the launcher exited 23, so `verify-build` did not run. The binary is bound by four things: `binary.sha256`, `build-info.json`, every record's `binary_sha256`, and the auditor binary. VERDICT-P32 (C) already disclosed this, and the review plan expected it as a caveat. For all six jobs the image digest itself is unverifiable from S3 without pod status. This is disclosed, not cured.
4. **The report id is null.** The gate names the report by path and sha256 (`docs/VALIDATION_GATE.md`). `tools/allocate_id.py` has no validation-report type. So this review, `VALIDATION.json` and the lead name it as `validation/validator-report.json` (sha256 `df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a`). That is complete without an id.
   - Other studies cite `VAL-*` ids (PROTOCOL §1 cites VAL-20261029-33a286). I could not locate the allocator that mints them, and minting one is not required.
5. **Independence.** The validator was dispatched by the main session that also relayed the producer's cards. It shared no code and no outputs with the producer, read no `analysis/` file and replayed on a host of another class. This is disclosed and accepted.

**Evidence strength: `preliminary`.**
- In the program's vocabulary, `replicated` requires an independent re-measurement. This result is one wall session on one host class.
- The independent replay re-executes the deterministic counted records, not wall time. The counted session's second-pod execution replicates counted records, not the wall statistic.
- The label does not measure decision grade. The result is pre-registered, decided under the frozen rule, far from 1 relative to its interval, and independently validated. But it is not replicated, and VERDICT-P32 (D) "may not" 6 forbids calling it so.

**knowledge_promotion: none; `not_warranted`.**
- The strength is below `replicated`: one session.
- It is a constant-cofactor engineering component whose durable content is a scoped measurement, not a transferable finding.
- Its place is the lead atlas entry (4), as VERDICT-P32 anticipated.
- If an independent second wall session ever reproduces the interval below 1, that decision promotes a `KN-FIND`.

**Promotion gates for asymptotic claims:** not applicable. No asymptotic or exponent claim is made (VERDICT-P32 (D) "may not" 9).

**Heuristic-validation pairing:** not applicable. The result is not conditional on a heuristic.

**Other sizes.**
- **P24 regression.** The reading is reproduced (corrected 1.003200 [1.000564, 1.005842]). It is recorded as the study's per-size reading, README only, with no row: the gate concerns rows shown as results.
- **P28.** It stays undecidable (infrastructure). Its counted session of record (`-r2`) satisfies the route and the counted=wall equality, so its counted distances are reportable as distance remaining (VERDICT-P32 (B) and (D) 3).

---

## (2) The 1.0500 truncation

**Ruling: it is a correction note, recorded as an additive `correction` record (CORR A below). VERDICT-P32.md stays byte-unchanged.**

- **The defect.**
  - VERDICT-P32 "Facts used", S′ bullet (line 61), prints the per-curve S′ `dgbf / dgb` at w = 0.1 on `icv1-fp32-t89141-95879fe3` as 1.0500.
  - The value is 1.0500605204138123 (`validation/code/out/j4.json`, `P32-wall` and `P32-counted`, `dgbf/dgb`, `0.1`, `per_curve`). At four decimals it rounds to **1.0501**. 1.0500 is a truncation.
- **Why a correction record and not a silent fix.**
  - The verdict is archived, and results are immutable: corrections are new records (AGENTS.md rule 4).
  - A wrong printed number in an archived decision report is exactly what a `CORR-*` is for (precedent `CORR-20261029-1aaef6`).
- **Effect: none.** The figure is descriptive. It is in no claim of (A) or (D), in no field of the DEC-20261029-5eb8e6 YAML, and in no headline. The aggregate 1.050488 and the other curve's 1.0509 reproduce.
- **Downstream use.** Any README, PDF or section that quotes per-curve S′ prints 1.0501 and cites the CORR.

## (2b) A defect the validator did not report: `c2_identity_facts`

**Ruling: a non-blocking defect in the validator's report assembly, recorded as CORR B. The report stays as written, and VALIDATION.json pins it.**

- **What it is.** `validator-report.json` line 2 reads `"c2_identity_facts": "1.3626"`. The field should carry RULING-P28 C2's identity facts for the six jobs.
- **The cause, in `validation/code/build_report.py`:**
  - line 57 binds `c2 = J("c2_identity.json")`;
  - line 123 reuses `c2` as a loop variable (`for pair, c1, c2 in (…, ("dgbf/dgb", "1.3547", "1.3626"))`), so after the loop `c2` is the string "1.3626";
  - line 303 writes `"c2_identity_facts": c2`.

  No other name used in the report dict is rebound.
- **The intended content** is committed and correct, in `validation/code/out/c2_identity.json`:
  - for all six jobs, `SHA256SUMS_OK`, `MANIFEST_COMPLETE`, `JOB_EXIT_PRESENT`, `JOB_EXIT_IDENTITY`, `S3_TERMINAL`, `INPUT_SPEC`, `BUILD_INFO` and `BINARY_SHA` are true, `POD_UID_COUNT` is 1, and `IMAGE_DIGEST` is `unverifiable_without_pod_status`;
  - `EXIT_CODE` is 0 for P32-wall and P24-wall, 42 for P32-counted, P24-counted and P28-counted, and 143 for P28-wall.

  REPORT.md's sentence on the C2 facts (J2, "S3 and launch binding") is fixed prose consistent with that file.
- **Why it does not block.**
  - No joint, headline, §6.1 entry, status computation or gate check reads the field. `check_validator_report` reads `status`, `independent`, `raw_data` and `headline`.
  - The facts it should hold are all disclosed as caveats in (1) 3.
- **Why no re-issue is required.** A validator re-issue would change the pinned digest and the REPORT.md header to fix a field nothing consumes. The validator's report is its record, as in `CORR-20261029-1aaef6`.
- **If a re-run happens.** Re-running `build_report.py` reproduces the defect until the loop variable is renamed. Any re-issue is the validator's, under a new file name, and then needs a new VALIDATION.json pin.

---

## (3) What may be stated

VERDICT-P32 (D) stands: its "may claim" items 1–6 and "may not" items 1–9. This section makes P32's standing final and fixes the wording and the evidence files.

### 3.1 Distance to matched rho (counted S′, w = 0 / 0.1 / 1; "distance remaining", never a pass/fail gate)

| size | standing | `dgb / rho-neg` | `dgbf / rho-neg` | `dgbf / dgb` | per curve at w = 1 (`dgb` ; `dgbf`) |
|--:|:--|:--|:--|:--|:--|
| 24 | regression (decided) | 12.12 / 11.65 / 9.45 | 12.12 / 12.24 / 12.82 | 1.0 / 1.0506 / 1.3560 | 10.00, 8.96 ; 13.68, 12.03 |
| 28 | undecidable (infrastructure); counted from `-r2`, session of record, **descriptive** | 21.04 / 20.02 / 15.34 | 21.04 / 21.12 / 21.44 | 1.0 / 1.0548 / 1.3980 | 15.33, 15.34 ; 22.20, 20.71 |
| 32 | **validated component speed-up** | 31.47 / 29.99 / 23.28 | 31.47 / 31.50 / 31.63 | 1.0 / 1.0505 / 1.3586 | 21.81, 24.97 ; 29.55, 34.02 |

Sources: validator `code/out/j4.json`, `validator-report.json` → `p28_counted_session_of_record.sprime`, and REPORT.md J4. Every published figure is copied programmatically from those keys, not retyped from this table.

**The §5a band** (AGENTS.md §5a; PROTOCOL §5):
- **IC against rho.** `rho-neg` is cheaper than both IC arms at w = 0, 0.1 and 1, on every curve at every size, so that ranking is permitted.
- **`dgbf` against `dgb`.** The counted order is not strict across the band: a tie at w = 0, and `dgbf` costlier by 5.05% and 35.86% at w = 0.1 and 1 at P32. So `dgbf` is never ranked ahead of `dgb` in the counted unit.
- **The growth across sizes.** The ratio rises from about 12× to about 21× to about 31× at w = 0. It may be stated only descriptively, as a comparison across sessions on different curves, not as a fitted scaling.

**What the band figures may not imply.** Wherever S′ appears, the three §5 statements go beside it:
- `dgbf`'s counted cost exceeds `dgb`'s at w > 0, by the ratio shown;
- `table_insert_attempts` is unpriced on every IC arm, identically between `dgb` and `dgbf`, a gap that predates this study;
- these figures decide nothing.

No figure may say that `dgbf` narrows the gap to rho.

### 3.2 The final report (`research/ic_packed_filter_20261029/README.md`, its figure and PDF)

It may state VERDICT-P32 (D) items 1–6, with these changes.
- **P32 standing.** The phrase becomes: "decided under the frozen rule, **validated** (independent recomputation and replay, TASK-20261029-654f73; DEC-20261030-04fe6d), single session, not replicated". "Validation pending" is no longer used.
- **P28.** The README gives the counted distances of 3.1, under VERDICT-P32 (B)'s P28 label, and its wall figures descriptively only.
- **Validation section.** It names the following:
  - `validation/validator-report.json` (sha256 `df9624c2…7a6a`, status pass);
  - `validation/replay-certificate.json` (sha256 `b357028b…d533`, verified);
  - `validation/REPORT.md`;
  - the five joint verdicts and the proves-too-much control;
  - the caveats of (1) 1–5, and CORR A and B.
- **The descriptive intervals** of REPORT.md J1, all labelled "decide nothing":
  - per curve: 0.959215 [0.954834, 0.963617] and 0.961017 [0.956294, 0.965762];
  - target-clustered bootstrap: [0.958482, 0.961943];
  - the per-round corrected ratios;
  - the label-swap null table.
- **The position table** (§10), from REPORT.md J2, for P24 and P32. For P28 it comes from the producer's descriptive processing, labelled descriptive.
- **Memory beside time** (descriptive): median max RSS at P32 is 62,350 KiB for `dgb` and `dgb-aa`, 63,412 KiB for `dgbf` and 63,380 KiB for `dgbf-edge`.
- **The mechanism and the prediction miss** as in VERDICT-P32 (D) 5.

**It may not state, in addition to (D) "may not" items 1–9:**
- that the replay "reproduced" or "confirmed" the speed-up. The replay reproduces deterministic counters and logarithms. The wall statistic was measured once.
- that the counted session's second-pod execution replicates the wall result. It replicates counted records only.
- any wording in which "validated" reads as "replicated", "general" or "transferable".

### 3.3 `VALIDATION.json`: where it lives and what it holds

**Location: two files, identical except for their relative paths.** Each display has its own evidence directory under `docs/VALIDATION_GATE.md`. Both are depth-2 directories, which is where `raw_data` resolves (its paths begin `../../`). Neither file goes in `validation/`: from depth 3, `../../research/…` does not resolve.

1. **`research/ic_packed_filter_20261029/VALIDATION.json`.** This is the evidence directory of the leaderboard component section. It follows the precedent of `prime_base_size_component()`, which runs `gate.check_evidence_dir(REPO / study, REPO)`. The pinned paths are `validation/replay-certificate.json` and `validation/validator-report.json`.
2. **`benchmarks/20261029-ic-packed-filter/VALIDATION.json`.** This is the evidence directory of the benchmarks row ("for a benchmarks row, `benchmarks/<dir>/`"). The pinned paths are `../../research/ic_packed_filter_20261029/validation/replay-certificate.json` and `…/validator-report.json`.

**Content of both files:**
- `"schema": "validation-gate/v1"`.
- `"rows": ["icv1-fp32-t89141-95879fe3", "icv1-fp32-tm68303-f902d2f0"]`: P32 only. No P24 or P28 curve may appear. P24 is a regression, P28 is undecidable, and neither is a row (VERDICT-P32 (B)).
- `replay_certificate`: the path above, sha256 `b357028b1c2187dc67abfbafe8999bf5f1b1a79e9c74c2471304b16e14f4d533`.
- `validator_report`: the path above, sha256 `df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a`.
- `rho_reference`:
  - `kind`: `"negation"`.
  - `protocol`: "PROTOCOL.md §2 and §6.2 (backlog/XP-20261029-packed-filter, freeze f870408c17): arm rho-neg (rho.negation), with A/A rho-neg-aa, in the counted session ic-dgbf-rho-p32-20261029, on the same curves, targets and seeds as dgb and dgbf".
  - `unit`: "counted S′ = (band.rs priced GAE at w + w × table_filter_bit_sets) / √r, w GAE per unpriced native operation (PROTOCOL §5)".
  - `binary_or_process`: "ecbench sha256 21701be6c420c3507a81767ed20f759e230f1fe5ec002ef3f7bb63a9c0b64503 from 3271a25d02, image 32144a29…817b; rho-neg, dgb and dgbf in the same counted pod; the counted dgb and dgbf records equal the wall records".
  - `matches_protocol`, `same_unit` and `same_binary_or_process`: `true`.
  - `strongest_available`: `true`. Basis, stated in a sibling key `strongest_available_basis`: the curves are prime-field curves with A ≠ 0 and B ≠ 0 (`replay-certificate.json` `curve_binding`), so j ≠ 0, 1728. No automorphism fold exists, and Frobenius is trivial over a prime field, so negation is the strongest reference.
  - `gae_band`: keys `"0"`, `"0.1"` and `"1"`. `ic` is the candidate `dgbf`'s mean S′ and `rho` is `rho-neg`'s. Both are copied by program from `code/out/j4.json` → `P32-counted` → `dgbf/rho-neg` → `0.0`, `0.1` and `1.0` → `mean_Sprime_b` (ic) and `mean_Sprime_a` (rho). The values, for checking:

    | w | `ic` (`dgbf`) | `rho` (`rho-neg`) |
    |:--|--:|--:|
    | 0 | 31.693595555358375 | 1.0070458230707695 |
    | 0.1 | 34.87384979037822 | 1.1071161527651106 |
    | 1 | 63.49613790555689 | 2.00774912001418 |

    An optional extra key `ic_baseline_dgb` per weight carries `dgb`'s mean: 31.6936, 33.1978 and 46.7353. The gate reads only `ic` and `rho`.
  - `order_holds_across_band`: `true`. It holds for `rho-neg` against `dgbf`, and against `dgb`, on both curves at every weight (`strict_same_order_across_band_every_curve`).
  - A sibling key, `ic_vs_ic_order`, carries: "dgbf is not ranked against dgb in the counted unit: tie at w = 0, dgbf costlier at w > 0".
- **Gate.** `python3 scripts/validation_gate.py evidence <dir>` must say valid for both directories before anything is staged.

### 3.4 The leaderboard component section (outside tables A–C)

**Placement.**
- The section is its own section, built by a new builder function modelled on `prime_base_size_component()`. Its standing comes from `gate.check_evidence_dir(REPO / "research/ic_packed_filter_20261029", REPO)`.
- It moves no row of tables A–C and is ranked against nothing in them.
- It states the track as `docs/ic/SWEEPER.md` classifies these pair-table pipelines at the time of publication.

**Title:** "Component speed-up: membership filter on the packed pair table (2026-10-29, engineering)".

**It may state, and must state all of:**
1. **Label:** "whole-solve wall per charged GAE at N_inc, filtered against unfiltered packed table", `mitm-prime-blocked:m=2,filter=1` against `mitm-prime-blocked:m=2`, at b = 32.
2. **The figure:** corrected 0.9601 [0.9569, 0.9633], that is 3.99% less wall per charged GAE (95%: 3.67% to 4.31%), at identical charged GAE.
3. **The bitop ratio beside it:** `bitop/v1` +0.301%, with the three §5 limits:
   - the rise is build-side only;
   - the probe-side filter test is not counted separately;
   - the build's pass over the empty slots is not counted.
4. **The S′ row beside it:** `dgbf / dgb` 1.0 / 1.0505 / 1.3586 at w = 0 / 0.1 / 1, with the three §5 statements of 3.1.
5. **Distance remaining at 32 bits:** `dgbf / rho-neg` 31.47 / 31.50 / 31.63 and `dgb / rho-neg` 31.47 / 29.99 / 23.28, with the §5a sentence of 3.1.
6. **Scope:** the scope of (1), "one session, 288 pairs".
7. **Standing:** "validated with caveats: replay certificate, validator report and the matched rho reference with the §5a band (research/ic_packed_filter_20261029/VALIDATION.json); single session, not replicated". The caveats are (1) 1 and 3 in a short form, with a link to the README.
8. **Other sizes:** the sentence "At 24 bits the same comparison read a regression (1.0032 [1.0006, 1.0058]); 28 bits is undecidable (infrastructure); see the README". No P24 or P28 row.
9. **Links:** the README and its PDF.

**It may not state:**
- anything in "may not" of VERDICT-P32 (D) or of 3.2;
- any per-curve or descriptive interval presented as the result;
- any composition with XP-20261029-dg-blocked.

**After the build.** Regenerate `docs/ic-leaderboard.html`, `docs/ic/LEADERBOARD.md` and `docs/ic/leaderboard.json` with `python3 scripts/build_ic_leaderboard.py`, with the lab browser, and run `scripts/check_ic_leaderboard_paths.py` for the workflow filter.

### 3.5 The `benchmarks/20261029-ic-packed-filter/` row

**The directory holds:**
- `VALIDATION.json` (3.3, item 2);
- copies of the README's PDF, SVG graph(s) and PNG render(s);
- a short `README.md` pointing to the study.

**The row in `benchmarks/README.md`** carries the same content as items 1–7 of 3.4, in the table's existing columns:
- the date 2026-10-29 and the directory;
- the label (item 1);
- 0.9601 [0.9569, 0.9633];
- `bitop/v1` +0.301%;
- S′ `dgbf / dgb` 1.0 / 1.0505 / 1.3586;
- `dgbf / rho-neg` 31.47 / 31.50 / 31.63 as the distance remaining;
- c8a.2xlarge, 32 bits, two curves, 288 pairs, single session;
- validated with caveats.

Any item a column cannot hold goes in the directory's README, and the row links it. The bitop ratio and the S′ row are never dropped from the row.

**The row goes in only when** `validation_gate.py check --base origin/main` accepts it, in the same PR as the leaderboard change.

---

## (4) The `LEAD-ICP-prime-ic-constant-engineering` update

**Ruling: update the lead.**
- The disposition stays `open_next_step`.
- The progress-fidelity gate (AGENTS.md) applies. No `claims/*.json` record exists for this result, so in this repository the figure is labelled UNQUALIFIED, in the lead's existing style.
- Filing a `claims/*.json` record at `level: local` stays an open item of the lead. It is not assigned now.

The text below goes in the crypto-autoresearcher ledger archive of this decision. The archive runs the lead-atlas checker and leaves the other fields unchanged.

`next_action.step`:
- Replace the clause "Held for the Coordinator's rerank at the XP-20261029-packed-filter P32 verdict (crypto backlog/XP-20261029-fb20-larger-n/DECISION.md): a single challenger N 54 against the baseline at freeze time, after the deferred guard has been tested at 20 bits." with:
  > Reranked at the XP-20261029-packed-filter P32 verdict (DEC-20261029-5eb8e6 (E)) and again at its evidence review (DEC-20261030-04fe6d (5)): neither 20-bit residual is admitted while PR #2826 is unmerged; if one is admitted later, the deferred guard at 20 bits comes before a single challenger N 54. The rerank fires when #2826 merges or closes, a 20-bit deferred-guard result lands, or the owner asks.
- Replace the sentence that begins "The other open constant is the hash-table floor of the relation trial" with the following. The derivation of the floor and the `layout=lines` patch are kept unchanged.
  > The hash-table floor was attacked first with a one-bit-per-slot membership filter in front of the packed pair table (crypto XP-20261029-packed-filter, `mitm-prime-blocked:m=2,filter=1`, opt-in; branch claude/ic-packed-filter-20261029; REVIEW-P32, DEC-20261030-04fe6d): a validated whole-solve wall component speed-up at 32 bits only, a regression at 24 bits, and 28 bits undecidable, with no 28-bit successor (DEC-20261029-5eb8e6 (B)). At 32 bits the filter cut target_PDP by 25.9 % but that phase is 29 % of the solve wall, and target_query (56 %) is untouched (descriptive phase sums), so further work on the probe side is bounded by the PDP share. Still open on this item: software prefetch, or a compact lane layout (crypto research/prime_ic_program_20261029/hashtable_prefetch, uncompiled layout=lines patch and draft protocol).

`best_measured`, appended:
> Packed-table membership filter (crypto XP-20261029-packed-filter; UNQUALIFIED in this repository, no claims/*.json record; validated in crypto by TASK-20261029-654f73, validation/validator-report.json sha256 df9624c2…7a6a): at 32 bits, corrected whole-solve wall per charged GAE dgbf/dgb 0.9601 [0.9569, 0.9633] (-3.99 %), identical charged GAE, bitop/v1 +0.301 %, S' dgbf/dgb 1.0 / 1.0505 / 1.3586 at w = 0 / 0.1 / 1 (no counted improvement; the filter's bit sets make dgbf costlier at w > 0); one session, c8a.2xlarge, two curves, not replicated. 24 bits: regression 1.0032 [1.0006, 1.0058]; 28 bits undecidable. Distance to matched rho.negation, counted S' of dgbf at w = 0 / 0.1 / 1: 12.12 / 12.24 / 12.82 (24 bits), 21.04 / 21.12 / 21.44 (28 bits, descriptive), 31.47 / 31.50 / 31.63 (32 bits); different curves per size.

`evidence`, appended:
- `{repo: crypto, path: research/ic_packed_filter_20261029/validation/REPORT.md, note: "PROTOCOL 10 validation of P24/P28/P32: pass, J1-J5 hold, 1152/1152 replays identical per session"}`
- `{repo: crypto, path: research/ic_packed_filter_20261029/cards/REVIEW-P32.md, note: "support scoped to VERDICT-P32 (D) 1; strength preliminary; CORR A (1.0500 -> 1.0501) and CORR B (c2_identity_facts)"}`
- After the publication archive: `{repo: crypto, path: benchmarks/20261029-ic-packed-filter/VALIDATION.json, note: "P32 rows only"}`

The rest of the update:
- `ledger_refs`: add DEC-20261029-5eb8e6 and DEC-20261030-04fe6d.
- `last_reviewed`: `'2026-10-30'`.
- `forecloses`: unchanged. A measured, scoped component gain forecloses nothing.

**Success criterion.** This is a verified, measured component improvement over the current IC baseline's oracle (crypto CLAUDE.md, "Success criterion"). It is reported as such, in its own section. It is not a counted IC row, and not relabelling.

---

## (5) The 20-bit rerank of DEC-20261029-be9ff6

**Ruling: it does not fire now. Neither 20-bit residual is admitted.**
- **The triggers.** VERDICT-P32 (E) fixed three: PR #2826 merges or closes; a 20-bit deferred-guard result lands; the owner asks. None has occurred. PR #2826 is reported to me as open and unmerged, and no 20-bit deferred-guard result and no owner request is in what I read.
- **The freeze bar still binds.** DECISION.md bars a freeze while a validated 20-bit baseline change waits to merge. What was assumed at the verdict is now reported as fact, though not checked by me.
- **This study's completion is not a trigger.** VERDICT-P32 (E) was itself the completion-triggered rerank, and its inputs are unchanged. The capacity that this study frees does not admit a lane (focus discipline).
- **The order stands:** the deferred guard at 20 bits before the single N 54 challenger.

---

## What makes this review effective

Each archive runs alone. Nothing becomes official until it is committed, pushed and accepted by the post-commit verifier.

1. **Decision ledger archive.**
   - **Crypto** (branch `claude/ic-packed-filter-20261029`): this file and the session-log entry.
   - **crypto-autoresearcher:**
     - `ledger/decisions/DEC-20261030-04fe6d.yaml`;
     - `ledger/corrections/CORR-20261030-1e501e.yaml` (A) and `ledger/corrections/CORR-20261030-1153f8.yaml` (B);
     - `ledger/handoffs/TASK-20261030-7f570f.yaml`;
     - the lead update of (4).

   Push both, open or refresh both PRs (ready for review, auto-merge on) naming the records, and run the post-commit verifier. **The `support` transition becomes official at this push.**
2. **Publication** (`TASK-20261030-7f570f`): README, figure and PDF; the two `VALIDATION.json`; the builder section and regenerated leaderboard; the `benchmarks/` directory and row. It lands in its own ledger archive after its completion gate.

**Procedure deviation.** VERDICT-P32's "What makes this verdict effective" (5) planned the decision and the publication artifacts in one ledger archive. I split them, for two reasons:
- the rows must cite an official, committed decision;
- the publication artifacts do not exist yet, and their completion gate needs this decision's (3) to check against.

Nothing else in that plan changes.

---

## Decision record (to archive)

**What the committing session must do:**
- Allocate four ids and confirm each with `--check`, as dispatched:
  - `python3 tools/allocate_id.py --next coordinator_decision --date 20261029`;
  - `--next correction --date 20261029`, twice;
  - `--next handoff --date 20261029`.

  The decision date is 2026-10-30. If the main session's convention binds the id date to the decision date, it allocates under `20261030` instead, and uses the same date everywhere.
- Replace every `<allocate>` in this file and the four records in the same commit, before the first push of this file.
- Record this file's sha256 in the decision record's `evidence_refs` and in the session log.
- Run `tools/validate_ledger.py` on the records before pushing.

```yaml
type: coordinator_decision
id: DEC-20261030-04fe6d
date: 2026-10-30
subject: >
  XP-20261029-packed-filter P32 evidence review after the PROTOCOL 10 validation (TASK-20261029-654f73):
  support scoped to VERDICT-P32 (D) 1; corrections A (1.0500 truncation) and B (validator-report
  c2_identity_facts); publication scope; lead update; 20-bit rerank not fired
context: >
  Validator TASK-20261029-654f73 reported pass (validation/validator-report.json sha256 df9624c2...7a6a;
  replay-certificate.json b357028b...d533, verified, scalars_verified; snapshot commit 852515114 as reported).
  J1-J5 hold: blind corrected dgbf/dgb at P32 0.960107806 [0.956898, 0.963328], condition 9 exact, in-pod
  comparison to 4.2e-16; positions 144/144 at middle positions for dgbf and dgb-aa; 0 condition-8 mismatches
  over 336 records per filtered arm and the 384 x counter bitop identity on 672/672; S' and the 5a order
  reproduced at P32, P24 and P28-r2; 1152/1152 replays identical in P32 and P24 wall and counted sessions on
  an Intel host, own [d]G=Q check 1344/1344 per session. Proves-too-much: A/A intervals contain 1, P24
  regression reproduced, label-swap null centred on 1 with the observed 0.96 outside it at every n. 122/122
  headline entries reproduce. One descriptive truncation in VERDICT-P32 line 61 (1.0500 for 1.0500605);
  one undetected field defect in the validator report (c2_identity_facts = "1.3626", a shadowed variable in
  build_report.py lines 57/123/303).
decision: support
target_ids: [XP-20261029-packed-filter, DEC-20261029-5eb8e6, TASK-20261029-654f73, LEAD-ICP-prime-ic-constant-engineering, DEC-20261029-be9ff6]
rulings:
  "1": "support, scoped to VERDICT-P32 (D) 1: at b = 32, mitm-prime-blocked:m=2,filter=1 against mitm-prime-blocked:m=2, whole-solve wall per charged GAE at N_inc corrected 0.9601 [0.9569, 0.9633] (-3.99%, 95% 3.67-4.31%), identical charged GAE, bitop/v1 +0.301%, S' dgbf/dgb 1.0 / 1.0505 / 1.3586; component speed-up only, no counted/GAE/band/baseline improvement. Scope: c8a.2xlarge, image 32144a29...817b, ecbench 21701be6...4503 from 3271a25d02, two 32-bit curves, N_inc 1024, stride-renew-deferred-guard, incremental-gauss, packed layout, one session, 288 pairs, claim tier toy; no transfer. Strength preliminary (single session, not replicated). Caveats accepted: ecbench work not run by the validator (bitop identity re-priced independently); warm-ups covered by scalar check and second-pod execution, not by replay; image digest unverifiable from S3 for all six jobs, verify-build absent for the three counted jobs (launcher exit 23), binary bound by binary.sha256, build-info, record and auditor binary sha; report id null, named by path and sha256; validator dispatched by the relaying main session."
  "2": "VERDICT-P32 line 61 per-curve S' dgbf/dgb at w = 0.1 on icv1-fp32-t89141-95879fe3 is 1.0501 (1.0500605204138123), not 1.0500: additive correction CORR A; VERDICT-P32.md unchanged; no effect on any claim, decision field or headline."
  "2b": "validator-report.json c2_identity_facts holds the string 1.3626 instead of code/out/c2_identity.json (loop variable c2 at build_report.py line 123 shadows line 57): additive correction CORR B; non-blocking (no joint, headline, 6.1 entry, status or gate check reads it; the facts are committed in code/out/c2_identity.json and disclosed); report stays pinned as written; no re-issue required."
  "3": "Publication per REVIEW-P32 (3): README with P32 'validated, single session, not replicated', P24 regression, P28 undecidable with counted distances from -r2 (descriptive); VALIDATION.json in research/ic_packed_filter_20261029/ (leaderboard component evidence directory) and benchmarks/20261029-ic-packed-filter/ (benchmarks row), rows = the two P32 curves only, pins b357028b...d533 and df9624c2...7a6a, rho_reference negation with gae_band from code/out/j4.json P32-counted dgbf/rho-neg (ic) and rho-neg (rho), order_holds_across_band true for rho-neg against both IC arms; component section outside tables A-C and benchmarks row each carrying label, corrected ratio, bitop ratio with three limits, S' row with three statements, distance remaining dgbf/rho-neg 31.47/31.50/31.63 and dgb/rho-neg 31.47/29.99/23.28, scope and 'validated with caveats'. Distances: P24 dgb 12.12/11.65/9.45, dgbf 12.12/12.24/12.82; P28 dgb 21.04/20.02/15.34, dgbf 21.04/21.12/21.44; P32 as above. May not: VERDICT-P32 (D) may-not 1-9, a replay 'confirming' the wall result, second-pod counted equality as wall replication, 'validated' read as replicated or transferable."
  "4": "Lead LEAD-ICP-prime-ic-constant-engineering updated with the text of REVIEW-P32 (4): per-size verdicts, the filter as the first attack on the hash-table floor (32-bit validated component gain, 24 regression, 28 undecidable), the PDP-share bound on further probe-side work, distances to matched rho, UNQUALIFIED label (no claims/*.json), 20-bit rerank status; disposition open_next_step."
  "5": "20-bit rerank of DEC-20261029-be9ff6 does not fire: PR #2826 reported open and unmerged, no 20-bit deferred-guard result, no owner request; this study's completion is not a trigger; idle capacity admits no lane; order deferred guard before N 54 stands."
rationale:
  - "Validity before interpretation: run counts, manifests and S3 copies, seeds and spec equality, raw/summary agreement and the A/A and position controls were all checked by independent code, and none fails."
  - "The review plan's five joints each had a named breaking artifact; none was found. The decision statistic was re-derived blind from records.jsonl and matches the instrument to 4.2e-16."
  - "The proves-too-much control shows the required signature: no improvement on identical arms or at P24, and a label-swap null centred on 1 that narrows as 1/sqrt(n) while the observed ratio holds near 0.96."
  - "Coordinator contract: a positive result expands only after an independent verifier passes; it has passed, so the scoped support and the publication of PROTOCOL 10 may proceed."
  - "Neither discrepancy touches the claim: one is a descriptive figure truncated in prose, the other a field no check reads; results are immutable, so both are additive corrections."
  - "Strength preliminary because the wall statistic is one session on one host class; the knowledge-promotion rule applies at replicated or strong, and VERDICT-P32 already placed the durable record in the lead atlas."
  - "AGENTS.md 5a: rho-neg ranks ahead of both IC arms at every weight on every curve; dgbf is never ranked ahead of dgb in counted S'."
evidence_refs:
  - "crypto research/ic_packed_filter_20261029/validation/validator-report.json (sha256 df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a): status, joints, headline (122/122), section_6_1, proves_too_much, p28_counted_session_of_record, c2_identity_facts"
  - "crypto research/ic_packed_filter_20261029/validation/replay-certificate.json (sha256 b357028b1c2187dc67abfbafe8999bf5f1b1a79e9c74c2471304b16e14f4d533)"
  - "crypto research/ic_packed_filter_20261029/validation/REPORT.md (sha256 a2d8892c...8f47, as dispatched)"
  - "crypto research/ic_packed_filter_20261029/validation/code/build_report.py lines 57, 123, 303; code/out/c2_identity.json; code/out/j4.json P32-counted; code/out/j1-inpod-crosscheck.json"
  - "crypto research/ic_packed_filter_20261029/cards/VERDICT-P32.md (sha256 415a89d7...9ea3) Facts used line 61, (A), (B), (D), (E); DEC-20261029-5eb8e6; TASK-20261029-654f73 review_plan"
  - "crypto backlog/XP-20261029-packed-filter/PROTOCOL.md (703bde26...7e51) sections 1, 2, 5, 6.1, 6.2, 10; docs/VALIDATION_GATE.md; scripts/validation_gate.py check_validator_report, check_rho_reference; scripts/build_ic_leaderboard.py prime_base_size_component"
  - "crypto research/ic_packed_filter_20261029/cards/REVIEW-P32.md (sha256 recorded at archive)"
limitations:
  - "No shell: no sha256 computed and no git state seen; validator digests and the snapshot commit 852515114 are as reported. PR #2826 state as reported."
  - "Single wall session per size, c8a.2xlarge only, two curves per size; the replay reproduces counters and logs, not wall time; S' at w = 1 is a pessimistic band device; table_insert_attempts unpriced on every IC arm."
  - "Image digest unverifiable from S3 for all six jobs; verify-build absent for the three counted jobs."
  - "benchmarks/README.md not materialized in the reviewing worktree; the row's column layout is left to the table's existing format."
knowledge_promotion:
  promoted: []
  not_warranted: "Strength preliminary: one wall session on one host class, not replicated (VERDICT-P32 (D) may-not 6); a scoped constant-cofactor engineering measurement, recorded in LEAD-ICP-prime-ic-constant-engineering. A KN-FIND follows only if an independent second wall session reproduces the interval below 1."
procedure_deviations:
  - "VERDICT-P32 'What makes this verdict effective' 5 planned the decision and the publication artifacts in one ledger archive; split into the decision archive and a publication archive (TASK below) so that the rows cite an official decision and their completion gate can check against REVIEW-P32 (3)."
decided_by: coordinator
decided_at: 2026-10-30
next_actions:
  - "Main session: allocate and check DEC, two CORR and TASK ids; archive this file and the session-log entry (crypto), and DEC, CORR A, CORR B, TASK and the lead update (crypto-autoresearcher) as one decision ledger archive; push both branches, refresh both PRs (ready, auto-merge on), run the post-commit verifier. Support becomes official at this push."
  - "Executor (TASK-20261030-7f570f): publication per REVIEW-P32 (3); its own ledger archive after its completion gate."
  - "Main session: where the round has a review batch directory, run tools/check_review_independence.py --batch <dir>; a failure returns to the coordinator."
  - "Main session: if this study is attached to an active GOAL (the lead lists GOAL-ICPERF-e6b6a4), checkpoint it after each ledger archive with exactly one next action: the publication task, then, after it lands, the 20-bit rerank when it fires."
  - "Coordinator: 20-bit rerank of DEC-20261029-be9ff6 when PR #2826 merges or closes, a 20-bit deferred-guard result lands, or the owner asks."
```

```yaml
# Additive correction A. Changes no byte of cards/VERDICT-P32.md or DEC-20261029-5eb8e6.
correction:
  id: CORR-20261030-1e501e
  id_provenance: Minted with tools/allocate_id.py --next correction --date 20261030 and confirmed free with --check.
  recorded_at: '2026-10-30'
  recorded_by: coordinator
  decided_under: DEC-20261030-04fe6d
  record_type: claim
  record_type_note: >-
    One descriptive per-curve figure printed in the decision report of DEC-20261029-5eb8e6; the DEC record's own
    fields do not contain it.
  record_id: DEC-20261029-5eb8e6
  also_corrects:
    - "crypto research/ic_packed_filter_20261029/cards/VERDICT-P32.md (sha256 415a89d7...9ea3), Facts used, S' for dgb->dgbf (line 61)"
  field: "P32 S' dgbf/dgb at w = 0.1, per curve icv1-fp32-t89141-95879fe3"
  prior_value: "1.0500"
  corrected_value: "1.0501 (unrounded 1.0500605204138123)"
  reason: >-
    Truncated rather than rounded at four decimals. Found by the PROTOCOL 10 validator (TASK-20261029-654f73,
    validator-report.json descriptive_discrepancies[0]) and confirmed by the coordinator in code/out/j4.json
    (P32-wall and P32-counted, dgbf/dgb, 0.1, per_curve).
  effect: >-
    None on any decision: the figure is in no claim of VERDICT-P32 (A) or (D), no DEC field and no headline; the
    aggregate 1.050488 and the other curve's 1.0509 reproduce. Later quotations print 1.0501 and cite this record.
  evidence_artifacts:
    - crypto research/ic_packed_filter_20261029/validation/validator-report.json
    - crypto research/ic_packed_filter_20261029/validation/code/out/j4.json
    - crypto research/ic_packed_filter_20261029/validation/REPORT.md (Discrepancies)
```

```yaml
# Additive correction B. Changes no byte of validation/validator-report.json, REPORT.md or code/.
correction:
  id: CORR-20261030-1153f8
  id_provenance: Minted with tools/allocate_id.py --next correction --date 20261030 and confirmed free with --check.
  recorded_at: '2026-10-30'
  recorded_by: coordinator
  decided_under: DEC-20261030-04fe6d
  record_type: claim
  record_type_note: >-
    One field of the independent validator's report for TASK-20261029-654f73, a review artifact with no ledger
    id (its id is null; it is named by path and sha256).
  record_id: "crypto research/ic_packed_filter_20261029/validation/validator-report.json (sha256 df9624c2849a9496c44598e1b27d5c671bd4275bcfe4dba87740ada8c8797a6a)"
  field: c2_identity_facts
  prior_value: "\"1.3626\""
  corrected_value: >-
    The object in crypto research/ic_packed_filter_20261029/validation/code/out/c2_identity.json: for P24-wall,
    P24-counted, P28-wall, P28-counted, P32-wall and P32-counted, SHA256SUMS_OK, MANIFEST_COMPLETE,
    JOB_EXIT_PRESENT, JOB_EXIT_IDENTITY, S3_TERMINAL, INPUT_SPEC, BUILD_INFO and BINARY_SHA true, POD_UID_COUNT 1,
    IMAGE_DIGEST unverifiable_without_pod_status; EXIT_CODE 0, 42, 143, 42, 0, 42 respectively.
  reason: >-
    validation/code/build_report.py line 57 binds c2 to c2_identity.json; line 123 reuses c2 as a loop variable
    over per-curve S' strings, leaving it "1.3626"; line 303 writes that value. Found by the coordinator in the
    evidence review; not reported by the validator.
  effect: >-
    None on the validation verdict or the decision: no joint, headline, section_6_1 entry, status computation or
    validation_gate.py check reads the field, and REPORT.md's C2 sentence matches c2_identity.json. The report
    stays as written and pinned by VALIDATION.json. A re-run of build_report.py reproduces the defect until the
    loop variable is renamed; any re-issue is the validator's, under a new file name, with a new VALIDATION.json pin.
  evidence_artifacts:
    - crypto research/ic_packed_filter_20261029/validation/code/build_report.py
    - crypto research/ic_packed_filter_20261029/validation/code/out/c2_identity.json
```

```yaml
type: handoff
id: TASK-20261030-7f570f
date: 2026-10-30
from: coordinator
to: executor
objective: >
  Publish XP-20261029-packed-filter under DEC-20261030-04fe6d: the study README with figure and PDF
  (research-visuals), the two VALIDATION.json files, the leaderboard component section outside tables A-C with
  the regenerated leaderboard, and the benchmarks/20261029-ic-packed-filter/ directory and row, each stating
  exactly what cards/REVIEW-P32.md (3) permits.
uncertainty_reduced: >
  None scientific: this is publication of a decided and validated result. It removes the risk that a published
  row overstates the scoped claim or lacks its gate evidence.
inputs:
  - "crypto research/ic_packed_filter_20261029/cards/REVIEW-P32.md (worker context: (3) in full, (2), (2b))"
  - "crypto research/ic_packed_filter_20261029/cards/VERDICT-P32.md (worker context: (B) P28 label, (D))"
  - "crypto research/ic_packed_filter_20261029/validation/{REPORT.md, validator-report.json, replay-certificate.json, code/out/*.json} (number source for validation figures)"
  - "crypto research/ic_packed_filter_20261029/P24/, P28/, P32/ analysis/ outputs (number source for the producer's figures, phase sums and ecbench work totals)"
  - "crypto docs/VALIDATION_GATE.md; scripts/validation_gate.py; scripts/build_ic_leaderboard.py (precedent prime_base_size_component); .claude/skills/research-visuals/SKILL.md"
  - "crypto-autoresearcher ledger/corrections CORR A and CORR B of DEC-20261030-04fe6d"
constraints:
  - "Every number is copied by program from the files named in inputs, never retyped; per-curve S' at w = 0.1 on t89141 prints 1.0501 (CORR A)."
  - "No statement beyond REVIEW-P32 (3) and VERDICT-P32 (D) may-claim; every may-not item is honoured. A required statement that cannot be made as written returns to the coordinator; nothing is reworded to fit."
  - "No change to any frozen file, card, spec, run directory or validation/ file; no EKS job, no run; local builds and checks only."
  - "Obtain or attach to a Conductor task for the paths below before editing (crypto CLAUDE.md); report any scope expansion before editing outside them."
  - "No P24 or P28 row anywhere; P28 wall figures descriptive only."
deliverables:
  - "research/ic_packed_filter_20261029/README.md, its figure(s) (SVG and PNG) and README.pdf"
  - "research/ic_packed_filter_20261029/VALIDATION.json and benchmarks/20261029-ic-packed-filter/VALIDATION.json per REVIEW-P32 (3.3)"
  - "scripts/build_ic_leaderboard.py component function and Markdown/HTML section; regenerated docs/ic-leaderboard.html, docs/ic/LEADERBOARD.md, docs/ic/leaderboard.json and the lab-browser files the builder writes"
  - "benchmarks/20261029-ic-packed-filter/ (VALIDATION.json, PDF, SVG, PNG, README.md) and its row in benchmarks/README.md"
  - "research/ic_packed_filter_20261029/PUBLICATION-CHECK.md: each required statement of REVIEW-P32 (3.2)-(3.5) mapped to the file and line that makes it, and each may-not item to the check that it is absent"
artifact_paths:
  - research/ic_packed_filter_20261029/README.md
  - research/ic_packed_filter_20261029/README.pdf
  - research/ic_packed_filter_20261029/figures/
  - research/ic_packed_filter_20261029/VALIDATION.json
  - research/ic_packed_filter_20261029/PUBLICATION-CHECK.md
  - benchmarks/20261029-ic-packed-filter/
  - benchmarks/README.md
  - scripts/build_ic_leaderboard.py
  - docs/ic-leaderboard.html
  - docs/ic/LEADERBOARD.md
  - docs/ic/leaderboard.json
  - "the lab-browser outputs and any workflow filter path that scripts/check_ic_leaderboard_paths.py names, listed by exact path in the receipt"
archival_task: "one ledger archive of exactly the artifact_paths above plus the session-log entry, after the completion gate"
inference:
  policy: executor-implementation
  independent_session_required: false
  fallback_allowed: false
  degraded_allowed: false
  delivery: interactive
budget: {}
completion_gate:
  - "python3 scripts/validation_gate.py evidence research/ic_packed_filter_20261029 and evidence benchmarks/20261029-ic-packed-filter both say valid"
  - "python3 scripts/build_ic_leaderboard.py --check and python3 scripts/validation_gate.py check --base origin/main pass; scripts/check_ic_leaderboard_paths.py reports no missing filter path"
  - "PUBLICATION-CHECK.md maps every required statement and every may-not item, with no item open"
  - "the ledger archive is committed, pushed, the crypto PR refreshed (ready, auto-merge on) and accepted by the post-commit verifier"
return_format: "paths and sha256 of README.pdf, both VALIDATION.json and PUBLICATION-CHECK.md; validation_gate and builder check outputs; the archive commit; any item returned to the coordinator"
```
