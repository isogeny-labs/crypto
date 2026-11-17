# Execution assessment: koblitz_m5_m6 campaign (attempts 1-5), director assess mode

Date: 2026-11-08. Branch `claude/arity-koblitz-a5-20261108`, repository HEAD `af9addcc07e2`.
Scope: execution only. No K verdict is interpreted here and no improvement is claimed.

```yaml
card_id: none (campaign-level assessment; the attempt-5 card CARD-arity-launch-koblitz-a5 has no receipt, see U6)
relay_verbatim: not_assessed (no executor report text was supplied; no card receipt exists to run check-relay against)
outcome: executed_as_specified_with_deviations   # classification: COMPLETE-WITH-DEVIATIONS
stopped_at_step: null
recommendation: proceed_to_analysis   # with the open protocol questions P1-P3 below referred to the core tier
reasons:
  - every one of the 148 grid cells has exactly one terminal record (a)
  - max attempts per cell is 2, under retries.max_attempts = 3 (b)
  - every m5/m6 cell ran on Intel Xeon Platinum 8488C, Part A on one node, Part B on two nodes of that model (c)
  - no process_error, no invalid_model, no A/A disagreement; seconds are diagnostic only (rule 6) (d)
  - analysis.txt, table.tsv and ANALYSIS.json are reproduced by the frozen script, differing only in the printed raw path (e)
interpretation: not_assessed
```

## Evidence checked (own commands)

- Frozen files unchanged since freeze commit `99285a6e86`: `git diff --stat 99285a6e86 HEAD` on protocol.json, PROTOCOL.md, analyze.py, grid.tsv, run_shard.sh is empty.
- sha256: analyze.py `b556bbea...3bcfe` (matches `analysis_plan.script.sha256`), grid.tsv `c5ff1c2b...`, run_shard.sh `c25c097f...`. The pod's `frozen-files.sha256` (a4 and a5) equals these two.
- Instrument identity: pod `binary.sha256` (a4 = a5, byte-identical): `n83_msweep_cell` `bdf52eee79a6...e12ebe1b`, `pkm_tower_pilot` `5b6d8c9c...`. Image digest `sha256:dd5249556468...e644b05`, same in request a4 and a5. `provenance.source_revision` = `4fa25efc0e2392b2e02e4c84253059ebc7c67e02`, `protocol_sha256` = `ac8027f0...e394c9`.
- Attempt 4 pod prefix downloaded from S3 (`runs/arity-koblitz-m56-20261104-a4/da7313ba-.../`): 287 files, SHA256SUMS all OK. Attempt 5 `run-a5/SHA256SUMS`: 483 lines, all verified against `raw/` and `run-a5/` on disk (the 26 raw files not listed are all 0-byte stubs; SHA256SUMS lists non-empty files only, same as a4's 176 empty files).
- a4 raw (448 files) vs committed raw (494 files): 446 byte-identical; the 2 that differ are the `.time` sidecars of `liftB n83k1 m5 union-3 chain-s3-orbit public` and `liftB n83k1 m6 union-2 chain-s3 public`; no a4 file is missing from the committed raw. This matches the dispatch.
- Working tree: `git status --porcelain` shows only untracked `koblitz_m5_m6/{ANALYSIS.json,analysis.txt,table.tsv,raw/,run-a5/}` (inside write scope). Nothing else is modified.

## (a) One record per cell

Expected set computed from `grid.tsv` (24 lines, 23 data rows): 148 distinct file names (Part A: negatives 10, public 40, A/A 40, planted 40 = 130; Part B: negatives 2, public 8, planted 8 = 18; total 148). Found 148 `.json` records: 0 missing, 0 extra, 0 duplicates, every record in the shard directory for its m (`shard2`: 12, `shard5`: 68, `shard6`: 68). Terminal states: 52 complete, 84 process_cap (all exit 124), 12 skipped_stopping_rule.
By shard: m2 12 complete; m5 21 complete / 41 process_cap / 6 skipped; m6 19 / 43 / 6. The .time sidecar exists for the 136 cells that ran (all but the 12 skips).

## (b) Attempts per cell

Reconstructed from `launch/receipts/attempts/koblitz-a1`, `koblitz-a2`, `receipts/CARD-arity-launch-koblitz-a{2,3,4,5}`, the a4 S3 prefix and `run-a5/`:

| attempt | job / pod | host | what ran |
|---|---|---|---|
| 1 | `arity-koblitz-m56-20261104` / a6009182 | AMD EPYC 9R45 (c8a) | m2 only; 6 negatives complete; `negative icv1-f2m37-tm534059-32aad96b chain-s3-orbit` in flight, Spot loss (143) |
| 2 | `-a2` / 03123eb7 | AMD EPYC 9R45 | m2 only; 4 more negatives; `negative n83k1 m2 chain-s3-orbit` in flight, Spot loss |
| 3 | `-a3` | none | never scheduled (Pending), deleted by `delete-command`; S3 prefix holds only `launcher/`, no records |
| 4 | `-a4` / da7313ba | Intel Xeon Platinum 8488C (m7i.2xlarge) | m2: the 2 n83k1 negatives; Part A m5/m6 complete; 3 Part B cells; killed by `activeDeadlineSeconds` with 2 cells in flight |
| 5 | `-a5` / e87cf35a | Intel Xeon Platinum 8488C (m7i.2xlarge) | 13 Part B cells (the 2 in-flight cells fresh, 11 new); m2 shard resumed with nothing to do |

Per-cell maximum is 2 (`negative icv1-f2m37-tm534059-32aad96b chain-s3-orbit`: a1 lost, a2 complete; `negative n83k1 m2 chain-s3-orbit`: a2 lost, a4 complete; `liftB n83k1 m5 union-3 chain-s3-orbit public` and `liftB n83k1 m6 union-2 chain-s3 public`: a4 killed, a5 complete (process_cap)). Every other cell has 1 attempt. All are under `retries.max_attempts = 3` (README deviation 6, per-cell reading). The a1 and a2 records that survive are byte-identical in the final raw (6 of 6 and 10 of 10 m2 records). The first launch card (`CARD-arity-launch-koblitz`) failed its source-identity gate and never submitted; it ran nothing.

## (c) Host model of the m5/m6 cells

- Part A (m5, m6; all 120 cells incl. 12 skips) and 3 Part B cells: pod da7313ba, node `ip-10-84-108-136`, `model name: Intel(R) Xeon(R) Platinum 8488C`.
- 13 Part B cells: pod e87cf35a, node `ip-10-84-100-118`, same model string.
- Part A is on one host model (one node). Part B is on one host model (two nodes of it). Part A and Part B are also the same model as each other. Host identity per cell is derived from the pod that wrote the record (no per-record host field); the two pods' `host.txt` are the evidence.
- The m=2 negatives are not on one model: the 10 Part A negatives ran on AMD EPYC 9R45 (a1, a2); the 2 n83k1 negatives ran on the Xeon in a4 (README deviation 8 mentions the ten Part A controls only). Not an m5/m6 cell; reported because the protocol matches the negative control to the measured cells on "host class".

## (d) Invalidation and stopping rules applied mechanically

Invalidation rules (protocol.json order):

1. Decided planted cell with proved_unsat or fewer than four verified witnesses: NOT triggered. The 6 decided planted cells (chain-s3: icv1-f2m29-tm40309-30c52b96 m5/m6 S=2, icv1-f2m37-tm534059-32aad96b m5/m6 S=2/3) all have 4/4 witnesses, 4 verified, 0 proved_unsat. The other 8 completed planted cells are all `incomplete` (not decided: 6 chain-s3, 2 chain-s3-orbit), and 30 planted cells are process_cap (23 Part A, 7 Part B). See P1.
2. Decided public cell with verified_relations != relations: NOT triggered (0).
3. invalid_model: NOT triggered (0).
4. Negative control (m=2 union:2) not deciding all queries inside its cap: TRIGGERED for engine `chain-s3-orbit` at icv1-f2m23-t5197-69e76b73, icv1-f2m29-tm40309-30c52b96, icv1-f2m31-tm90707-c95f16f5, icv1-f2m37-tm534059-32aad96b, icv1-f2m41-tm2308219-7f48b14a (4/4 incomplete each; icv1-f2m29-tm40309-30c52b96 at 0 conflicts) and n83k1 (1/1 incomplete, 200000 conflicts). Per the rule and AM1, that engine's capped cells at those n are `not_evaluable` for K2 and K4. `chain-s3` negatives all decide (proved_unsat 4/4, and 1/1 at n83k1): not triggered for it. See P2 for how analysis.txt prints this.
5. A/A pair with differing complete records: NOT triggered. 36 non-skipped pairs: 0 disagreements on outcomes, relations, verified_relations, base_points, columns; 0 one-capped-one-complete noise flags (analysis.txt line "A/A pairs with disagreement or noise flag: 0").
6. Binary sha256 differs from the frozen value / no L2 admission: TRIGGERED, two grounds. (i) No L2 admission for a command Job (README deviation 1): every second is diagnostic; the process_cap class is a wall-clock event on the same tier. Decided/capped outcomes in conflict terms, D values and witnesses are exact. (ii) The only binary hash recorded at freeze is `smoke/binary.sha256` = `424382e5...82d3bd2cd36898eba5a07f12ebe1b` (authoring-VM preflight; protocol.json carries no `binary_sha256` for this study). The binary that ran is `bdf52eee...` (README "Source identity": the 08:27Z worktree build reproduced 424382e5 but was lost; the 15:41Z rebuild differs). The README discusses this but does not number it as a deviation; see U1.
7. process_error (exit not 0/124/137): NOT triggered. Exit codes: 84 x 124, 52 complete, 12 skipped; no process_error record.

Stopping rules:

- Fixed grid / per-cell cap: all 148 cells present; caps 600 s (Part A) and 1800 s (Part B) as in grid.tsv; no cap changed; no capped cell re-run with a longer cap. Capped Part A wall times 600.0x s, Part B 1800.0x s.
- Part A skip rule, recomputed independently from the public records of the two previous curves, per (m, engine): fires exactly at (m=5, chain-s3-orbit, icv1-f2m41-tm2308219-7f48b14a) and (m=6, chain-s3-orbit, icv1-f2m41-tm2308219-7f48b14a) (icv1-f2m31-tm90707-c95f16f5 and icv1-f2m37-tm534059-32aad96b, S=2 and 3, all process_cap). Cells skipped = the 6 per m (public, A/A, planted x S=2,3) = 12 total; 0 mismatches against all 120 Part A cells for both m and both engines (no cell skipped where the rule does not fire, none run where it fires). chain-s3 never fires (icv1-f2m37-tm534059-32aad96b decided). No Part B or negative cell was skipped.
- Invalidation stop (shard stop for a failed positive control, process_error, binary sha256 difference, process-capped/errored negative): no process_error, and no negative was capped or errored (all 12 complete). A "failed positive control" and a "binary sha256 different from the frozen one" are conditions the frozen `run_shard.sh` does not test and the shards did not stop on; see P1 and U1. The m2 gate (`gate-m2.json` passed true) and the m5/m6 shards exited 0.

## (e) analysis.txt reproduction

`python3 -I analyze.py raw --grid grid.tsv --json $S/ANALYSIS.json --tsv $S/table.tsv` re-run on the committed raw (output in scratchpad). `table.tsv`: identical (cmp). `analysis.txt`: 2 differing lines of 287 (diff), only line 1, which prints the raw directory (`under raw` in the re-run vs `under /tmp/.../koblitz-a5/raw` in the committed file). `ANALYSIS.json`: identical after removing the per-cell `path` fields (which carry the same directory difference). So the committed outputs were produced from a copy of the records at a scratch path, not from `raw` as stated in the dispatch; the content is what the frozen script produces from these records.

## Deviations from the frozen protocol (README numbers)

Applying to the Koblitz campaign:

1. No L2 admission for command Jobs (seconds diagnostic; rule 6).
2. m2 shard runs before, not concurrently with, the m5/m6 shards; m5 and m6 run concurrently as 2 single-thread processes inside one 4-CPU pod.
4. Executor host replaced by a pod; cell commands reproduced in `job-koblitz.sh`; director launch cards bind the request, launcher scripts and image digest.
5. Host type c8a.2xlarge to m8a.2xlarge (attempt 3, never ran). Superseded by 8; no record depends on it.
6. Attempt limit counted per cell (max observed 2).
8. Attempt 4 pinned to m7i.2xlarge (Intel Xeon 8488C) after Spot loss and capacity refusal; the m5/m6 cells all ran on it.
9. (Koblitz-relevant part) Koblitz stays pinned to m7i.2xlarge while Pending; this is the cause of the 8.9 h Pending ahead of attempt 4.
10. Attempt 5 is a plain command Job, not CRIU (matches `checkpoint.policy: none`); interrupted cells re-run fresh and counted against max_attempts = 3; Job deadline 64800 s; image unchanged; the attempt-5 card binds a `job-lib.sh` copy (`f03e57cf...`) different from the one in the image (`7c1e760e...`), so that bind records the tree, not what ran.

Not applicable to Koblitz: 3 and 7 (prime tower only).

Observed, not numbered in the README (referred to the main session; I did not edit the README):

- U1. Binary sha256 `bdf52eee...` differs from the freeze-recorded `424382e5...` (rule 6 ground ii; stopping rule 4 would stop the shard). Documented in README "Source identity" prose only.
- U2. Mixed host for the m=2 negatives (EPYC 9R45 for 10 Part A negatives, Xeon 8488C for 2 n83k1) versus a single Xeon for the cells they control.
- U3. Job `seconds` 60900 (a4) and 64800 (a5) exceed `execution.timeout_seconds` 57600; a4 ended on its deadline after the m5/m6 shards had finished Part A. `job-exit.json` for a4 reads `exit_code 0` (known TERM-trap defect, a1 NOTE), not 143.
- U4. The frozen analyze.py was run from a copy at a scratch path (see (e)).
- U5. The m=2 shard's a2 pod and a4 pod resumed from the previous attempt's S3 prefix; the m2 gate in a4 passed on the resumed records.
- U6. The attempt-5 card produced no `.receipt.json` (runner killed after submit, `launch/receipts/CARD-arity-launch-koblitz-a5/NOTE.md`) and the benchrec record has no `runs` entry; the evidence is the launcher result copied from S3, the Job's S3 prefix and its SHA256SUMS (all verified above). No executor relay text was given, so `relay_verbatim` could not be checked.

## Open protocol questions for the core tier (not resolved here)

- P1. K3 / positive control wording. Rule 1 and the K3 claim speak of "decided" planted cells; the positive-control text says every planted cell that completes returns 4/4 witnesses and treats proved_unsat or an unverified witness as a miss, but is silent on a completed planted cell whose outcomes are all `incomplete`. analyze.py counts those as misses (prints K3 falsified: chain-s3-orbit 0/2, chain-s3 6/12). By the literal rule 1, no planted cell invalidates anything. Whether stopping rule 4 ("failed positive control") should have stopped a shard depends on this reading; run_shard.sh has no such stop.
- P2. Rule 4 / AM1 say the orbit engine's capped K2/K4 cells at the n where its negative is undecided are `not_evaluable`; the frozen analyze.py prints `supported_partial` (K2) and `supported` (K4) for chain-s3-orbit with a NOTE instead. The rule's mechanical consequence is stated in (d) above.
- P3. Rule 6: the `process_cap` class and its cap-boundary behavior are wall-clock events on a non-L2 tier (protocol.measurement); how that bears on the cap-based classes is a core-tier decision.

## Classification

COMPLETE-WITH-DEVIATIONS. Every grid cell is recorded once, attempts per cell are within the limit, m5/m6 are on one host model, no cap or grid cell was changed, the frozen scripts and image are unchanged, the stopping-rule skips match the rule, and the analysis output is reproducible. The deviations above (no L2 admission, binary hash, host and route changes, CRIU not used) make all seconds diagnostic and attach the invalidation-rule consequences listed in (d). Not `invalid`: no rule that voids the records outright (process_error, invalid_model, failed A/A, decided-planted miss) applies.

## Could not check

- The attempt-5 pod's L2 isolation, steal and PSI (no host telemetry besides `host.txt`); the node and the pods' `host.txt` are the only host evidence.
- Per-record host identity (records carry no host field; host is inferred from the pod prefix). The A/A pairs for Part A ran in the same pod, so same-host.
- Whether `bdf52eee...` differs from `424382e5...` only by non-reproducible code generation (README says not investigated).
- Executor relay verbatim (no report text, no card receipt).
- S3 object hashes for the attempt-1/2 pods (checked on disk against the copied files only; their m2 records are byte-identical to the final raw).
