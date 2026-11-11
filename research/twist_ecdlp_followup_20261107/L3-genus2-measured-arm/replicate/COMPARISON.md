# L3-genus2-measured-arm: replication comparison

Scope: mechanical replication of the frozen commands in VERIFICATION.json. No interpretation.
All outputs were redirected to `replicate/`; the originals (`out/`, `out_reduced/`, `results.json`,
`results_reduced.json`, `RESULTS.md`, `out/summary.txt`, `out/posthoc.txt`) were not written
(mtimes unchanged, 20:37 to 20:46 from the original run).

## Frozen commands and completion

| # | Frozen command | Redirected to | Status |
|---|---|---|---|
| 1 | `sha256sum -c FROZEN_SHA256.txt` | n/a | completed, all 11 files OK |
| 2 | `run.sh` FULL (k=8..16, all controls) | `replicate/full/` (OUT, RES, SUM) | completed rc=0, 736 s, no INCOMPLETE size |
| 3 | `KMAX=12 CTRLSET=reduced run.sh` | `replicate/reduced/` | completed rc=0, 37 s, KLIST=8..12 |
| 4 | `analyze.gp` (re-run, original `out/` raw files) | `replicate/analyze/` (RES, SUM) | completed rc=0 |
| 5 | `posthoc.gp` + `make_results.sh` | `replicate/full/posthoc.txt`; RESULTS.md assembled in `replicate/assembly/` | completed rc=0 |

Every frozen command ran to completion. No process started by this run was left running; the
chain process exited after `ALLDONE`.

## Comparison with the originals

Timing rule: the last field of each run row (RHO, IC, and the control arms PCA, PCC, NCA, NCB, NCC)
is the wall-clock time and was excluded. All other fields were compared exactly.

| Item | Compared | Result |
|---|---|---|
| Full-run raw files `out/main_k*_h*.txt`, `out/ctrl_*.txt` | 34 files | 34/34 match on all non-timing fields (18 main + 16 ctrl; control files differ only in the wall field, 0 non-timing diffs) |
| Reduced-run raw files `out_reduced/main_*`, `ctrl_*` | 20 files | 20/20 match on all non-timing fields |
| `results.json` (full, from replicate run) | whole JSON, wall keys removed | identical |
| `results_reduced.json` (reduced) | whole JSON, wall keys removed | identical |
| `results.json` (analysis-only re-run on original raw files) | whole JSON | identical |
| `out/summary.txt` (full run) | byte-level | identical |
| `out/summary.txt` (analysis-only re-run) | byte-level | identical |
| `out/posthoc.txt` | byte-level | identical |
| `RESULTS.md` (assembled by make_results.sh in replicate/assembly) | byte-level | identical |

Mismatches: none in non-timing numbers.

Timing-only differences (not compared, expected under contention): wall-clock fields in the
full run and reduced run differ from the originals (for example, control rows in the replicate
differ only in the last field).

## Notes

- The reduced run's raw files were compared against the original `out_reduced/` files, not against
  the full run's files.
- Raw replicate outputs (`replicate/full/main_*`, `ctrl_*`, `replicate/reduced/main_*`, `ctrl_*`) and the
  replicate plots were deleted after comparison to free disk (disk was at 97%). Kept: summaries, post hoc
  text, JSON, RESULTS assembly, `chain.log`, and the per-job logs.
- The comparison script is at `research/twist_ecdlp_followup_20261107/L3_compare.py`.
- A `run.sh` process belonging to a different arm (`L2-binary-xonly-twist-filter`, PID 18889) was running
  at the start. It was not touched.
