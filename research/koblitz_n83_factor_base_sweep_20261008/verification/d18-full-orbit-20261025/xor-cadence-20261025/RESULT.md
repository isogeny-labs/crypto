# Full-D18 native XOR cadence-32 capacity receipt

The source-bound attempt used clean PR commit
`00b3d3929e493587676001e58c3d8bdcc67e3dc0`, release exporter SHA-256
`d3ce79d87241ace0b5af50012e3237192972128c0e14dcc2a64224c66e128d14`,
and a fresh GET of the frozen complete D18 S3 object and wrapper. The
160,747,529-byte object SHA-256 was
`eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`;
the wrapper SHA-256 was
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.
Both fresh GET responses carried full-object SHA-256 checksums.

The constructed worker emitted its `started` phase with the exact
`phase_zero_first_ordered_first_pair_xor_cadence32` policy. It did not emit
`import_checked` or a formula shape. Its own 300-second cap wrote
`UNKNOWN_wall_cap` and exited 124; the independent 310-second supervisor
did not fire. Supervisor wall time was 303.192 seconds. Maximum resident
set was 966,737,920 bytes, and `/usr/bin/time -l` recorded 24.28 user and
6.93 system CPU seconds, with zero swaps. The host's one-minute load average
rose from 100.818 before the worker to 173.849 afterward. These are
shared-host L0 conditions, without a hard memory cgroup.

The initial supervisor labelled the positive arm `POSITIVE_GATE_FAILED`.
The independent audit retains that raw summary but verifies the cap and
classifies the attempt `CAPACITY_CENSORED_BEFORE_MODEL`. It rejected an
altered cap-status control. The runner now classifies a fully checked
internal cap directly, and the auditor handles both the retained historical
label and future `UNKNOWN_wall_cap` summaries. The natural arm was not
launched. No model, solver counters, constructed relation under cadence 32,
or natural relation was observed; this attempt cannot compare cadence-one
and cadence-32 wall time or rank factor bases.

The opt-in solver change passed a 120-case exhaustive mixed CNF/XOR
differential test at cadences 1, 2, 4, and 32, complete-assignment parity
checks, and an expired-deadline check. The full release library suite passed
4,168 tests (117 ignored), the touched example passed 19, and the N83 and
boundary Python suites passed 63 and 52. Those tests validate behavior on
their tested inputs; the N83 correctness gate remains pending a resource-
qualified constructed control. `selected_best_total_runtime` remains null.

`evidence/` contains the raw identity, phase, cap, process-use, supervisor,
summary, independent-audit, changed-cap and S3 GET receipts, plus passing
test logs and their SHA-256 inventory. Do not retry on the same saturated
host condition. Resume with the committed protocol on a resource-qualified
host that can read the exact S3 generation; retain a fresh provider-checked
GET, clean source, release binary, positive independent gate, natural
terminal status, and complete downstream IC and same-target rho accounting.
