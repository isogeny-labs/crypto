# Native N83 retained-base continuation

Both exact retained primary a=0 bases completed their separately frozen
natural m5 orbit-chain-S3 trials on the same independently replayed public
query. Each reached the 1,000-conflict cap with **zero models, zero relations
and rank zero**. This resolves bounded terminal observability, not natural
useful rank, full cold selection or the full finite-grid sweep.

| Arm | Outer wall (ms, measured, L0) | Native audit | Cleanup |
|---|---:|---|---|
| K2048 | 163378.646436 | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | verified |
| K4096 | 337901.101152 | PASS_NATIVE_BOUNDED_ZERO_RELATION_REPLAY | verified |

Sample size is one per arm; no uncertainty interval or A/A was recorded.
Resource limits match: 1 CPU, 4 GiB, no swap, network none, 64 PIDs,read-only mounts,
task UID,capabilities dropped,no-new-privileges,immutable image and inner
hard timeout. solver_enter/return include preprocessing; no internal cost
split or timing-performance ratio is inferred. The query a,b and coordinates
match across arms and pass independent native polynomial-word group replay.
See [compact original run records](records/native-phase-20261019-v2/exec/v3/),
[protocol](PROTOCOL.md), [cards/specs and verified relays](transport/v3/),
[visual report/PDF](../../reports/run-20261019T0140Z/REPORT.md) and
[integration qualification](INTEGRATION.md).

## Preserved controls and failures

V1's original fixed-coordinate/free-selector control failed its positive
gate at 180 s wall with unknown final counters; its natural cards remained
unlaunched. The distinct v2/v3 control additionally fixes representative and
phase selectors, and passes verified SAT plus independent group/phase
replay. Small-field m5/m6 ordered/unordered original-group assertions and
four traced SAT returns pass in each version. Fully specified N83 controls
prove consistency only, not free-selector N83 witness discovery.

Both v2 natural launches fail before search because a sequential policy was
requested from hash-policy retained manifests. V3 explicitly corrects the
policy, adds native preflight guards and freezes fresh specs/cards. Every
failure, source version, binary, command, receipt and cleanup record remains.
No failed launch is treated as a timed search result or silently retried.
Three [forged-record controls](forgery-controls.json) are rejected, including
selector phases and query coefficients with updated custody hashes.

## Publication and remaining scope

The previously approved raw ZIP is **uploaded and verified**, all 224 entries:
[publication receipt](../raw-approved-publication-20261019/RESULT.md).
The older pending-upload state is superseded, without changing frozen files.

The new native raw archive is complete locally: 49,012,484bytes, SHA256
`bef8a03c355b2238282638e326dd571bd91dd6873f5ed1df5fe12db2f0dcd5c3`.
A fresh local extraction verifies all 308 payload files against the included
SHA256SUMS. [Contents](raw-contents.sha256) and the
[exact archive/storage plan](raw-artifact-plan.json) make the proposed upload
reviewable. **It has not been uploaded.** Automatic approval review rejected
this new source/binary/input/raw-record payload as outside approval for the
earlier exact ZIP. No alternate upload route was attempted. The archive is
preserved at the plan's absolute local path pending explicit approval. The
future native store command will create a tracked logical-name index; no
remote index/object is claimed now. Post-merge test logs are a separate
integration addendum, not retroactively included in the raw archive.

The [bounded observability problem](../../../../problems/n83-native-k4096-terminal-observability.md)
is resolved. [Free-selector N83 witness search](../../../../problems/n83-free-selector-known-witness-search.md),
full native positive-relation/matrix verification, natural useful rank,
column logs, individual-log work and full cold-runtime selection remain open.
No solver process is left running; further diagnostics are not scheduled.

| Requested requirement | Status | Evidence or remaining gap |
|---|---|---|
| Upload the earlier approved ZIP | verified complete | fresh S3 GET, archive hash and all 224 entries |
| Restore Docker and resume retained-base diagnostics | verified complete within declared bounds | both native terminal/query/resource/cleanup audits |
| Preserve and publish new native evidence | local preservation verified; upload blocked | new archive needs explicit approval after auto-review rejection |
| Full factor-base sweep / cold winner | partial | wider declared grid, useful natural rank and full costs unresolved |

Host metadata observed after the runs: Intel Xeon 6975P-C, x86_64,
32 logical CPUs; see [CPU inventory](host-cpu.json) and
[kernel](host-kernel.txt). This supplemental observation and final integration
logs are outside the already frozen raw archive. They do not establish
exclusive-core timing isolation.
