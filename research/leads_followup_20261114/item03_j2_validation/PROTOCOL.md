# PROTOCOL: independent validation of J2 (factor-base constant c_D = 1.60 vs the frozen 2.00, N4v2 DL1)

Written 2026-11-14 by the Validator, before any validator data run (replay, rebuild hash, certificate run, recompute).
Not edited after data; any change is a dated "Amendment" section at the end. Everything the Validator reports is
`[pcc:unverified]` and **unsigned**: no keyring signature is available in this session.

## 0. What is validated, and the frozen artifact set

Claim under validation (producer's words, `research/leads_chase_20261112/RESULTS.md` section J2, bench record
`benchmarks/records/20261112T0348Z-j2-percolation-fb-size.json`): on the N4v2 plane-quartic DL1 solver (GHS-weak family
`E0/F_{q^3}`), lowering the factor-base constant `c_D` from the frozen 2.00 to 1.60 cuts the charged cost to
0.655-0.685x at k = 21 and 22 on held-out seeds 4-8, with the order c = 1.60 < 1.75 < 2.00 holding at w = 0, 0.1, 1;
first-attempt success flips between c = 1.50 and 1.60 at every k; falsified at k = 16 (no gain).

Frozen artifact set: git commit `44d35033fa06b9796c6a4bf7696056d7a6e595a6` (branch claude/cool-davinci-szm3om, HEAD at
validation start). The producer's artifacts (`research/leads_chase_20261112/{PROTOCOL.md,RESULTS.md,tools/j2_*,j2/**}`,
the bench record, `research/quartic_n4v2_20261020/**`, `research/quartic_n4_20261019/**`,
`src/cryptanalysis/plane_quartic_pic.rs`) are committed and unmodified in the working tree at that commit
(`git status --porcelain` is empty for them; the only untracked path is the followup directory). Every file is hashed in
`FROZEN.sha256` in this directory (441 lines) before data. Out-of-git artifacts: `j2/v2.bin` (artifact store, manifest
hash 183810cf...), `quartic_n4v2_20261020/results/grid-20261025/units.tar.gz` (artifact store, manifest hash aac9a25c...;
fetched read-only by SigV4 GET into the scratchpad and hashed).

## 1. Checks, pass criteria, falsifiers

Each check ends pass / fail / not-recomputable. Producer scorers (`tools/j2_score.py`, `tools/j2_band.py`) are NOT used:
the Validator's own code is `tools/` in this directory.

| id | check | pass criterion |
|:--|:--|:--|
| V0 | provenance: commit, clean tree, artifacts exist and hash to recorded values (v2.bin vs BUILD.txt and `.artifacts.json`; raw-units.tar.gz vs bench record; N4v2 units.tar.gz vs manifest; raw-units.tar.gz members equal committed unit files) | all equal |
| V1 | rebuild `v2` from `research/quartic_n4v2_20261020/v2.rs` with `rustc --edition 2021 -O` (three invocation forms: from the source directory, from the repo root with a relative path, with an absolute path), compare sha256 with 183810cf... | PASS if any recorded form reproduces the hash bit for bit; otherwise reported as "not bit-identical" and the check falls back to V2 equivalence, labelled as such |
| V2 | replay: ALL 84 IC units (54 grid + 30 held-out) re-run with the rebuilt binary, plus the A/A unit; every record compared field by field. Compared: rec, arm, q, seed, attempt, c, ell, k, k_found, ok, fail, every `<phase>.{mul,inv,add,lmul,ladd,hash,cmp}` and every `s.*` stat; and the ratios record's ell, order, gae_mul, gae_inv. Ignored (timing, host): any key ending `wall` or `cpu`, `load1`, `search_s`, `r_inv`, `r_lmul` of the `ratios` record, `s.controls_*`. The six mandated units (q=2097091 seed 4 at c = 1.60 and 2.00 and seed 5 at 1.75; q=4194271 seed 4 at c = 1.60 and 2.00 and seed 5 at 1.75) are inside the 84. If V1 is not bit-identical, at least those six are also replayed with the producer's `v2.bin` | 84/84 identical (record count, every compared field) |
| V3 | control the producer's protocol required but RESULTS does not show: c = 2.00 reproduces the committed N4v2 records. Compare DL1, D, DL counters at c_D = 2.0 (J2 `ic_<q>_<seed>_c2.00`) with the N4v2 grid `ic_<q>_<seed>_SD.jsonl` (raw units from the artifact store) for q in {65479, 262111, 1048507, 4194271}, seeds 1-3, and ell/order with `results/instances.jsonl` | every compared field identical; ok and attempts identical |
| V4 | certificate discipline, independent code (C, `tools/certcheck.c`, own xorshift64*, own F_q / F_{q^3} / Weierstrass arithmetic; no solver code): for every (q, seed) of the 22 instances re-derive E0 from the recorded RNG construction, its order by baby-step giant-step, `order = 4 ell`, `ell` prime, a generator G of the ell-part with ell G = O, the planted k from the seed (compare with the record's k), and check `k_found G = k G` on E0 for every record with ok = true; also j(E0) not in {0, 1728} and j not in F_q (so that negation is the only symmetry for rho) | every instance reproduces the recorded ell and order; every ok record has k_found = k and k_found G = k G |
| V5 | recompute the priced cost from raw counters: median of three `v2 calib` records per q, M-eq = sum over the 7 phases of (mul + r_inv inv + r_lmul lmul) over every DL1 attempt record; recompute every paired ratio, every success count, every range in RESULTS J2 and the bench record, and `j2/summary.json`. Cross-check of my pricing code: with the N4v2 calibration, my M-eq of DL1 at c = 2.00 must reproduce the committed N4v2 `summary_supp.json` DL1 mean at the same q | recomputed values equal the claimed values to the claimed rounding (3 decimals for ratios) |
| V6 | band: per (q, seed) the order c = 1.60 < 1.75 < 2.00 at w = 0, 0.1, 1 (unpriced = hash + cmp, GAE = gae_mul + r_inv gae_inv in M-eq, the N4v2 frozen rule). Sensitivities, labelled as such: (a) strict reading, unpriced also counts `add` and `ladd`; (b) calibration (r_inv, r_lmul) in {(15, 1.1), (30, 2.5), the N4v2 host values} | claimed order reproduced at k >= 18 and at the held-out k = 21, 22; the k = 16 failure reproduced; sensitivities reported without altering the verdict |
| V7 | threshold law: first-attempt success per (q, c) and the flip between 1.50 and 1.60; exact one-sided binomial bound for c = 1.60 first-attempt success over all c = 1.60 units | counts equal the producer's table |
| V8 | held-out discipline: amendment commit `0056acedae` time precedes the first held-out launch; held-out seeds 4-8 do not occur in N4v2's instance list or J2 grid; held-out predictions (success 5/5 each size at c = 1.60, 1.75; ratio in [0.60, 0.72] at 1.60 and [0.72, 0.82] at 1.75; order at every w) scored from my recompute; `failed-binary-missing/` units are empty or error records and none of them enters a reported number | as stated |
| V9 | matched rho reference: the N4v2 frozen rho records (raw per-run counters, plain and negation map) at q = 2097091 and 4194271 (and the committed summaries). Recompute rho mean M-eq at w = 0, 0.1, 1 with (i) N4v2's calibration (must reproduce the committed `summary_supp.json`), (ii) J2's calibration; build the table IC(c = 1.60, 1.75, 2.00) / rho-neg and / rho-plain at w = 0, 0.1, 1; list every unpriced counter of every arm with its count (AGENTS.md 5a); rho calibration rule status; replay of a subset of rho runs (k = 16, 18 full seed 1; k = 21 seed 1 runs 0-1 plain and neg; k = 22 one neg run if time allows) in the rebuilt binary against the N4v2 raw rho records | table complete; order holds across the band or is reported unresolved |
| V10 | reporting contract (AGENTS.md "The rule: boundary, table, ratio", 5a): one table, one unit, ratio, correctness column, phases separated, every arm priced including the reference, matched rho named before measuring, m = 83 / ECC2K-130 disclosure | pass / gap list |

## 2. Predictions and falsifiers (stated before data)

- Prediction: V2 passes 84/84 (the solver is deterministic; the producer reports an A/A repeat identical and the N4v2
  README reports S/L/D/DL records replaying bit for bit). Falsifier: any compared field differs in any unit: then the
  record is not reproducible and the J2 numbers are `invalid_measurement` for that unit; a rebuilt binary that differs
  while counters agree does not falsify.
- Prediction: V3 passes (D-family code unchanged since the N4v2 grid, whose source hashes in ADDENDUM-20261025.md match
  today's ic2.rs, la2.rs, solver2.rs, rho2.rs, common.rs, ghs.rs, ic.rs, solver.rs, plane_quartic_pic.rs; only v2.rs
  (gained `homcheck`) and summ2.rs differ from the addendum's hashes). Falsifier: any counter difference at c = 2.00.
- Prediction: V4 passes for all 22 instances (every unit's `ok` is the solver's own `k_found G = Q`, and `k_found` is
  computed from the descent logs, not from `k`: `solver2.rs` `downstream`). Falsifier: a recorded ell/order not
  reproduced, a k not reproduced from the seed, or any ok record with k_found G != k G: the run is
  `invalid_measurement`, not a negative result.
- Prediction: V5 reproduces the producer's table to its printed rounding; V6 holds at k >= 18 and the held-out sizes; the
  strict band (a) preserves the order because every phase shrinks with N_fb.
- Prediction: the matched rho (negation) is about 300x above DL1 in M-eq at k = 21, 22 (N4v2: DL1 / rho-neg 0.0028 at
  k = 21), so the order IC < rho holds across the band; the J2 own protocol names no rho reference and the rho arm ran in a
  different binary on a different host; the gate fields `matches_protocol` and `same_binary_or_process` are therefore
  expected to be false unless V9 shows otherwise.

## 3. Execution rules

At most 2 threads (4 vCPU shared); `ulimit -v 6000000` around every solver run; runs longer than 8 minutes under
`nohup` with a log and status file; binaries and the raw N4v2 extract live in the scratchpad (not committed); only code,
manifests and reports are written under this directory. No evidence of the producer is edited; no experiment is rerun to
manufacture a passing artifact: every replay result, pass or fail, is kept.

## 4. Dispositions

`validated` only if V0, V2, V3, V4, V5, V6 (as claimed), V7, V8 pass and V9 is complete; `invalid_measurement` if V2 or
V4 fails for any unit; `insufficient_evidence` if a required arm or control cannot be priced; `refused` if the artifact
set is not frozen. The gate descriptor `VALIDATION.json` states the true values of `matches_protocol`,
`strongest_available`, `same_unit`, `same_binary_or_process`, and is written unsigned.

## Amendment 2026-11-15 (after data; the protocol text above is unchanged)

Additions and execution notes, all made after the first recompute and labelled post-hoc:

1. **Added checks (post-hoc).** The recompute of V6 showed that the strict reading (additions counted as unpriced work) [pcc:unverified]
   reverses the c = 1.60 < c = 2.00 order. Four analyses were added to understand it: the price-of-an-addition sweep and its
   break-even (`add_ladd_weight_sweep`), the measured CPU / wall ratios of the same solves in the producer's records and in
   the validator replay (`cpu_wall_ratios_to_c2.00`), an in-situ least-squares price per counted operation on the
   linear-algebra phase (`in_situ_LA_regression`), and a fresh `v2 calib` re-measurement at q = 2097091 and 4194271
   (`evidence/calib_recheck/`). These form check V11 in the report. None was pre-registered.
2. **Disposition departs from section 4's mechanical rule, on judgment.** Section 4 would give `validated` for the
   measurement in the N4v2 unit (V0-V5, V8 pass, V6 passes under the frozen rule, V7 passes with a scope limit, V9 table
   complete), because section 1 listed the strict reading as a sensitivity that does not alter the verdict. The
   validator nevertheless reports `insufficient_evidence` for the improvement claim: V11 shows the gain does not appear in
   measured CPU time, the omitted cost class (counted additions) moves the other way, and AGENTS.md "What does not count"
   excludes a ratio improved by dropping a cost from the budget. A reviewer who prefers the mechanical rule can read the
   numbers in `evidence/recompute.json`; the departure is flagged in `validator-report.json` (`disposition_reason`).
3. **Execution slips, kept.** (a) `certcheck` first parsed the group order as u64; at k = 22 the order 4 ell exceeds 2^64, so the
   first k = 22 instance reported `matched: false`; fixed (u128 parse), the whole run repeated, all 22 instances pass. (b) The
   first launch of the rho replay passed one argument too few (the first job name became the tag); killed after about 3 minutes,
   relaunched with the correct arguments, no output of the aborted launch is used. (c) `v3_compare.py` first flagged the
   `tag` label of the `ratios` record ("S,D" in N4v2's process, "D" here); the label names the arms the process ran, not a
   counter, and is ignored in V3 only (V2 compares it).
4. **Store access.** `aws` and `boto3` are absent; the N4v2 raw units (`units.tar.gz`) and, as a probe, the J2 raw tarball and
   `v2.bin` were fetched read-only from the artifact store with a stdlib SigV4 GET (`tools/s3get.py`, region us-east-2, the
   session's existing AWS environment), into the scratchpad, and hashed against the manifests. Nothing was uploaded or deleted.
