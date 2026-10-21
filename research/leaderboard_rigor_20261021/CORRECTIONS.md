# Correction to the slice A report (research/validation_sweep_20261019/A-leaderboard)

Slice A's `REPORT.md`, its figure caption and the `notes` of the six Table A records in `verdicts.jsonl`
attribute the gap between Table A's rho step cost and the canonical step to a **lane-count mismatch**: "this
rho walks 32 lanes while the IC unit is a 1,024-lane batched addition". That explanation was an inference
from the CLI default and was not measured. It is wrong.

Measured since (`rho_lanes/`, `ic price --single-target --rho-lanes L`, n = 47 (4 targets), 53 (2), 61 (1),
L from 32 to 1,024, IC and rho in one process, L0 host, every run verified):

| n | lanes | rho units per step | rho steps (mean) |
|--:|--:|--:|--:|
| 47 | 32 / 64 / 128 / 256 / 512 / 1024 | 8.98 / 9.46 / 12.49 / 8.43 / 9.09 / 11.32 | 61k / 61k / 31k / 33k / 45k / 47k |
| 53 | 32 / 128 / 512 / 1024 | 9.25 / 8.90 / 10.23 / 9.59 | 1.03M / 0.92M / 1.01M / 1.00M |
| 61 | 32 / 1024 | 8.60 / 11.06 | 1.90M / 1.85M |

The walk costs 8.4 to 12.5 units a step at every lane count, with no downward trend, and the cold
IC / rho ratio stays within its run-to-run spread (n = 47: 5.1 to 8.7). So the lane count is not the cause,
and no implemented rho reaches the **canonical step**, which is a model (one batched addition plus the
table canonicalisation, measured separately). What stands from slice A: Table A's reference is the repository's
strongest *implemented* walk by ns per step (about 100 against about 200 ns for the standing fixture), and
priced at the canonical step the x reference would be 1.7 to 2.0 times larger. What does not stand: that the
IC arm "got a kernel the rho side did not" through lane batching. The gap between measured and canonical step
cost is rho overhead (look-ahead, distinguished-point handling, cycle checks, memory), not lanes.

The leaderboard text was corrected in this PR (`scripts/build_ic_leaderboard.py`, Table A reference).
Slice A's files are left as merged, because sealed audit records are write-once; this file is the correction.
