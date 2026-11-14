# Nagao Hilbert stop: taking it through the validation gate (2026-11-12)

Task `nagao-hilbert-validation`. It takes the Hilbert-stop result of `../README.md`
(U2 at w = 0: 1.003 / 0.837 / 0.695 at p = 1301 / 1451 / 1601, local L0, unvalidated)
as far through `docs/VALIDATION_GATE.md` as this session can.

Every result below is `[pcc:unverified]`. No pcc claim was filed. Nothing here is a
leaderboard row or a benchmark row.

## Verdict

| gate item | state |
|:--|:--|
| replay certificate | **done, host-independent only.** All 10 records (the A/A control plus 9 Hilbert instances) were re-run on a second host. Every counted field is identical, every logarithm is correct, and the binary was built from the study commit. `replay-certificate.json` |
| rho reference matched to the protocol, same binary, §5a band | **done.** The 126-run pooled reference was re-run with the study binary on the same pod, and all 126 records are identical except `wall_ms`. An independent fresh sample of 126 more walks (run indices 14–27) gives the same verdicts. |
| validator report | **open slot**, for the independent validator (`review/validator/validator-report.json`). |
| S3 raw data and `bench.record/v1` | **blocked.** The AWS identity is rejected, so the benchmark-record rule cannot be met from here. |

`python3 scripts/validation_gate.py evidence research/jv_nagao_hilbert_20261112/validation`
reports exactly one reason: `validator report: review/validator/validator-report.json is not a file`.
The replay certificate and the rho-reference descriptor pass the gate's checks.

The headline is unchanged `[pcc:unverified]`. With the Hilbert stop, the Joux–Vitse Nagao
arm is below matched negation rho in U2 at p = 1451 and 1601, at w = 0, 0.1 and 1. At
p = 1301 it is not resolved at w = 0.

## Exact commands

1. **Binary.** The current `target/release/examples/jv_cover` (a98c009d…) is not the
   study binary, so I rebuilt the study commit.
   - `git worktree add` failed: there was no space left on the device (4.3 GB free).
     Instead, `git archive dd00d429d6bc060085948ded46c3cbabcc60a882 <crate paths + the docs/ and research/ files the crate include_str!s>`
     was unpacked into `/tmp/claude-0/nh_src`.
   - The target directory was a hard-link copy of the shared `target/release/{deps,build,.fingerprint}`,
     so registry crates were reused. The command was `CARGO_TARGET_DIR=/tmp/claude-0/nh_tgt cargo build --release --example jv_cover --offline -j1`
     with rustc 1.97.0, and it took 22 min.
   - Result: sha256 **9ccf9217c717fa58f859fe7d75c2880f240d3a71696c17765dff9254eecab2fc**. The study's
     hash is f35af5d3722fe19fc55e844e88d2637302aed85568245a5cd755627449f74745, so the hash does
     **not** reproduce byte-for-byte. The cause is a different build directory and target state;
     no path strings are embedded. The counts were expected to be identical because the source
     is the same, and the replay confirms they are.
   - Uploaded with `rp.sh put`; the sha256 was verified on the pod.
2. **Pod.** `RP_STATE=/tmp/claude-0/rp_nhval rp.sh create bdr-nagao-hilbert-validation 32 cpu5c 30`
   created pod **jwdf4pnk0velez**: cpu5c, 32 vCPU, AMD EPYC 4564P, kernel 6.8.0, $1.12/h.
   - Created 16:33:48Z, deleted 18:07Z. That is **≈1.56 pod-hours, ≈ $1.74**.
   - A pod listing after the delete showed no `bdr-nagao-hilbert-validation` pod.
3. **Replay.** `pod_run.sh` part 1 is the frozen `../run.sh` command with unchanged flags:
   `JV_FB_LINE=65536 jv_cover --exp dlp --rho-runs 0 --rho-ref 1.3 --check-every 64 [--stop hilbert:16] --sizes P --first-seed S --seeds 1`.
   - It ran sequentially with the default rayon threads (32).
   - Compared with `replay_compare.py`: the JSON is flattened and every field except `*_ms` must be exactly equal.
4. **Concurrency control.** `hilbert_p1301_s1` was re-run with `RAYON_NUM_THREADS=1` (`replay_1thread/`).
5. **Matched rho.** `pod_run.sh` part 2 ran
   `RAYON_NUM_THREADS=1 jv_cover --exp rhostrong --sizes P --first-seed S --seeds 1 --neg 1 --batch 16 --r 256 --dp-bits 16 --run R`
   for R = 0..13 on the 9 instances (126 runs, 30 parallel, 3649 s).
   - `pod_run2.sh` ran the same command for R = 14..27: 126 more runs, 3127 s, overlapping the first batch.
   - Compared with `rho_compare.py`.
6. **Rescore.** The unchanged `negrho_rescore` (`research/jv_cover_negrho_rescore_20261101/fit`, built in
   `/tmp/claude-0/nh_fit`, sha256 5134ba71…) was run on /tmp copies of that study's results tree.
   - `inputs/cover/dlp_p*` was replaced by the committed Hilbert records, as `../README.md` did.
   - The reference sets were laid out as follows:
     - old pooled (as the study ran it)
     - fresh 0–13: runs 0 and 1 in `inputs/old_rhostrong`, runs 2–13 in `new/`
     - fresh 14–27: runs 14 and 15 as "old", runs 16–27 as "new"
     - combined 0–27
   - `band.py` recomputes the absolute §5a (ic, rho) values from the raw records. It matches the
     tool's point ratios to 1e-15.
   - `make_evidence.py` writes `replay-certificate.json` and `VALIDATION.json`.

## Results

### (1) Replay: 10 of 10 records, every counted field identical `[pcc:unverified]`

| record | fields compared | identical | differing (diagnostic only) | total_muls | relation_digest | correct |
|:--|--:|:--|:--|--:|:--|:--|
| aa_line_p1301_s1 | 73 | 72 | f4_muls_per_test_sum | 265,597,174,434 | 4198168094706900228 | yes |
| hilbert_p1301_s1..s3 | 73 | 71 each | f4_muls_per_test_sum, hilbert_check_muls | 140,174,712,426 / 150,234,219,750 / 135,319,708,867 | = committed | yes |
| hilbert_p1451_s1..s3 | 73 | 71 each | same two | 160,577,077,018 / 168,914,503,445 / 162,898,818,503 | = committed | yes |
| hilbert_p1601_s1..s3 | 73 | 71 each | same two | 187,545,267,253 / 180,656,654,191 / 181,461,894,403 | = committed | yes |

The following fields are identical in every record:
- `total_muls` and `s`;
- `f4_muls`, `f4_native_ops` and `f4_stair_ops`;
- `relation_digest`, relations, residuals and decompositions;
- `hilbert_applied` and `hilbert_fallbacks`;
- every other `*_muls` cost component;
- `solved`, `correct` and `mismatches`.

**The two fields that differ are not counted quantities.**
- `f4_muls_per_test_sum` and `hilbert_check_muls` are per-test differences of the process-wide
  `FIELD_OPS` counter (`src/cryptanalysis/f4_fp.rs`), so other threads' work leaks into them.
  On the 32-thread pod they are 5.7–8.8× the committed 4-thread values.
- With one thread (`replay_1thread/`), `f4_muls_per_test_sum` is 80,010,716,496, against
  `f4_muls` 80,024,626,047 (−0.017 %). That is what its doc comment says it should be.
- **Finding:** `hilbert_check_muls` with one thread is **8.875e9**, which is 11.1 % of `f4_muls`.
  The committed 4-thread record has 6.44e10, which would be 80 % of `f4_muls`.
  - The committed `hilbert_check_muls` values are therefore concurrency-inflated and must not be
    quoted as the cost of the Hilbert check.
  - The README does not quote them; `compare.py` carries them only as `hcheck`.
  - Neither field enters `total_muls`, S or the rescore.

### (2) Matched rho with the same binary `[pcc:unverified]`

**Runs 0–13 (126).** All 126 are identical to the pooled reference of
`research/jv_cover_negrho_rescore_20261101` (binary 8f62bee…) except `wall_ms`. All are correct
and none is capped.
- The walk is deterministic in (instance, run index), so this is a same-binary replay of the
  pooled reference, not a new sample.
- The rescore against it is therefore byte-identical to the old-reference rescore:
  `rescore_fresh_run0-13/table.md` equals `../rescore/table.md`.
- My /tmp build of the tool also reproduced `../rescore/{table.md,fit.txt,fit.json}` exactly.

**Runs 14–27 (126).** This is an independent sample of new walks, all correct and none capped.
- Homogeneity holds: ANOVA perm-p 0.54, KW perm-p 0.66.
- Positive control: steps/√l 0.8909 ± 0.0375 against theory 0.8880 (z +0.08).
- The reference is cheaper than the pooled one: U2 0.3707 against 0.3879.

U2 (textbook F_p multiplications), S/rho with 95 % bootstrap interval, estimator (b):

| reference | p | w = 0 | w = 0.1 | w = 1 | order holds across band |
|:--|:--|:--|:--|:--|:--|
| pooled = fresh 0–13 (n 126) | 1301 | 1.003 [0.916, 1.104] not resolved | 0.861 [0.786, 0.949] below | 0.831 [0.758, 0.915] below | **no** |
| | 1451 | 0.837 [0.771, 0.910] below | 0.718 [0.662, 0.782] below | 0.693 [0.638, 0.754] below | **yes (below)** |
| | 1601 | 0.695 [0.642, 0.756] below | 0.597 [0.551, 0.649] below | 0.576 [0.532, 0.626] below | **yes (below)** |
| fresh 14–27 (n 126) | 1301 | 1.050 [0.957, 1.155] not resolved | 0.901 [0.822, 0.993] below | 0.870 [0.793, 0.958] below | **no** |
| | 1451 | 0.876 [0.807, 0.956] below | 0.752 [0.693, 0.821] below | 0.725 [0.668, 0.792] below | **yes (below)** |
| | 1601 | 0.728 [0.672, 0.793] below | 0.625 [0.577, 0.681] below | 0.603 [0.557, 0.657] below | **yes (below)** |
| combined 0–27 (n 252) | 1301 | 1.026 [0.955, 1.108] not resolved | 0.881 [0.820, 0.952] below | 0.850 [0.791, 0.919] below | **no** |
| | 1451 | 0.856 [0.805, 0.912] below | 0.735 [0.691, 0.783] below | 0.709 [0.666, 0.756] below | **yes (below)** |
| | 1601 | 0.711 [0.670, 0.756] below | 0.611 [0.576, 0.650] below | 0.589 [0.555, 0.627] below | **yes (below)** |

U1 (group additions) at w = 0, 0.1 and 1 is below at every prime with every reference.
- Pooled: 0.417 / 0.348 / 0.289 at w = 0.
- Fresh 14–27: 0.437 / 0.364 / 0.303.
- Combined: 0.427 / 0.356 / 0.296.

The order holds across the band in all nine U1 cells. The full tables are in
`rescore_*/table.md`.

The modelled U2 crossover p\* is 1327 [1278, 1381] with the pooled reference, 1356 [1305, 1413]
with fresh 14–27, and 1342 [1305, 1382] combined.

**U3** is not defined by the unchanged `negrho_rescore` tool, which emits only U1 and U2. No U3
figure is reported, rather than inventing one.

**Against the preregistered prediction** (1.03 / 0.86 / 0.71 in U2 at w = 0):
- With the fresh independent reference the values are 1.050 / 0.876 / 0.728, closer to the
  prediction than the pooled 1.003 / 0.837 / 0.695.
- The verdicts are unchanged: not resolved at p = 1301, below at p = 1451 and 1601.

### (3) Gate evidence

The draft `VALIDATION.json` (schema `validation-gate/v1`) contains:
- `replay_certificate` → `replay-certificate.json`, with its sha256 pinned;
- `validator_report` → `review/validator/validator-report.json`, with sha256 `TO-BE-FILLED-BY-VALIDATOR`;
- `rho_reference`:
  - kind `negation`, unit U2, same binary 9ccf9217…, same pod;
  - `strongest_available`, based on red-team row 7 of `research/jv_cover_fbline_20261102`:
    j ∉ F_{p²} ∪ F_{p³} on all 9 instances, so there is no Frobenius or automorphism fold;
  - `gae_band` for the U2, p = 1601 cell: ic 0.2697 / 1.1757 / 9.3297 against rho 0.3879 / 1.9689 / 16.1973;
  - `order_holds_across_band: true`.
- The other five cells are in `band_fresh_run0-13.json`, `band_fresh_run14-27.json` and `band_combined_run0-27.json`.

Raw data for the validator:
- `../results/*.json` (committed records);
- `replay/*.json` and `replay_1thread/`;
- `rho_fresh_run0-13/*.json` and `rho_fresh_run14-27/*.json`, with their logs and `walls.txt`;
- `replay/host.txt`.

## What still blocks a row

1. **Validator report.** An independent validator must recompute the headline from the raw data
   in the repository and file `review/validator/validator-report.json` (status `pass`, `independent: true`).
   It should also re-sign or redo the replay certificate. That certificate is *host*-independent:
   a second machine ran the frozen command. It is not *analyst*-independent, because a sibling
   agent of the producing programme wrote it. The certificate says so in `independence`.
2. **Benchmark record and S3.** The rule needs a `bench.record/v1` record, raw data in S3 by SHA-256
   and a regenerated `benchmarks/progress/` report. None of this is possible from this session,
   because its AWS identity is rejected. The Iso Lab stage-3 route (lead
   `LEAD-C-jv-nagao-hilbert-at-scale`) remains the way to a gated row.
3. **pcc claim.** None was filed. Every number is `[pcc:unverified]`.
4. **Known limits, carried over.**
   - The cover count is a lower bound: cover F_p additions are not emitted.
   - The result covers only the weak class of curves over F_{p^6} with a Joux–Vitse cover, at p ≤ 1601.
   - The change is classified as engineering: counted cost falls while the native counter stays flat.

## Next step

Hand this directory to the independent validator: recompute S, the U1/U2 ratios and the band
from the raw records, and fill the validator-report slot. Then run the stage-3 Iso Lab job once
an AWS identity is available. Separately, fix `hilbert_check_muls` to use a thread-local counter
(the `FIELD_OPS_THREAD` / `F4_FP_THREAD_OPS` path already exists), so the Hilbert-check cost is
reported correctly under concurrency. The fix must be opt-in, or a diagnostic-only change.

## Files

- `pod_run.sh`, `pod_run2.sh` (run on the pod)
- `replay_compare.py`, `rho_compare.py`, `band.py`, `make_evidence.py`
- `replay/`, `replay_1thread/`, `rho_fresh_run0-13/`, `rho_fresh_run14-27/`
- `rescore_fresh_run0-13/`, `rescore_fresh_run14-27/`, `rescore_combined_run0-27/`
- `band_*.json`, `replay-certificate.json`, `VALIDATION.json`

The rescore logs print "consistency controls FAILED" for the reason `../README.md` gives: the
old cover records were replaced on purpose.

No repository source was changed. No git state was changed: the `git worktree add` attempt failed
before creating anything, and `git worktree prune` found nothing to prune.
