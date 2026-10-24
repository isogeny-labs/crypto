# Execution and retained failures

The protocol was committed before implementing or solving the candidate.
The original saved implementation was uncompiled. No old 189+21 case,
BSGS, five-factor scalar census or public-point census is repeated.

## Native build and test continuation, 2026-10-24

| Saved attempt | Observation | Treatment |
|---|---|---|
| `native-gate-v1` managed launch | Exit 2: isolation wrapper excluded by the custom sparse checkout; no compilation started | Retained journal and reconciliation |
| Standard sparse helper | Exit 2: existing checkout has no helper-managed profile | Retained refusal; materialized only the committed wrapper with explicit Git restore |
| `native-gate-v2` | Exit 127 during metadata enumeration: rg absent from managed PATH | Retained partial metadata; added the documented find fallback |
| `native-gate-v3` | Cargo exit 101: committed identity-certificate include fixture absent from sparse checkout | Retained compiler failure; materialized the exact committed fixture |
| `native-gate-v4` | Native test compilation receipt exits 0; managed launch exits 1; no completed test gate | Preserve both observations; never infer tests passed |
| `native-gate-v5` | Test and runner compilation receipts exit 0; traced managed launch refuses pre-existing output | Preserve directory and refusal; no completed test gate inferred |
| `native-completion-20261024T1934Z` | No live science owner/lock remained; successful build files were reconciled and copied; actual native suite passes 84/84 | Before/after source checks pass; only this continuation is credited as the test gate |

The final continuation runs a frozen test binary through the common busy
wrapper, serially, with 8 GiB and a 1,200-second watchdog. Its clock is a
diagnostic, not admitted timing. Production and test executable digests,
the reused compilation directory, owner PID, exact command, source checks,
stdout/stderr and actual exit are retained. The compact receipts are in
`results/native-completion-v1/metadata/`; all original raw directories remain.

The full 42-case-arm panel is not yet reported complete. It will use a new
output directory and the frozen production executable only after its source
and driver are committed. Phase 2's complete one-target price remains open.

## Preservation fix before the first candidate solve

The initial collector accumulated all system records in memory. Before any
candidate solve, durable JSONL input and result journals were added, with an
input synced before F4 and a result synced after decoding. A killed process
therefore leaves its processed prefix and pending system. Incompleteness is
still reported as incompleteness. Diagnostics include this journaling overhead
and are not admitted algorithm timings.

`native-gate-journal-20261024T1940Z` is the new gate for that source. At this
checkpoint its exact busy-wrapper owner is live and waiting for the shared
reservation. The earlier 84-test pass is not credited to the changed source.
The dependent panel controller must see this gate's terminal success before
starting any of the 42 case-arms.

## Existing panel reconciliation and nested reservation

An existing `twist-zero-five-panel-v1.service` was found with frozen inputs
and a runner under `runs/panel-v1`. It held the outer common busy reservation
while its first collector's inner busy wrapper waited for that same lock.
Process/cgroup and wait-channel checks, zero candidate/audit records and the
exact first command confirm that no native collector started. Its pre-stop
service state, journal, runner digest and reconciliation are retained under
`results/nested-reservation-v1/`. Only that verified deadlocked study service
was stopped; the separate live journalled native gate was kept queued.

Additional existing `native-gate-v6`/`v7` receipts were reconciled: v6 has
78 passing and six failing tests; v7 passes 84/84. Both predate the journalled
collector and their source manifests differ at that file, so neither is
substituted for the queued gate. All raw evidence remains unchanged.

The panel driver now refuses an outer busy ancestor with exit 64 before
creating outputs. Its controller verifies that refusal under a bounded
negative control, then launches the driver directly. Individual science
phases still acquire the shared reservation. The new panel uses a fresh
directory; the blocked old prefix is preserved, not called a scientific run.

The queued journalled gate subsequently completes all three phases with exit
zero, 84 passing native tests, zero failures and unchanged source hashes.
Its production executable is SHA-256
`ba3b7c62dd51dbb158a5407e75780cd4065bbd63d9d6baa8400e5468051b4a5f`.
Compact raw metadata is copied unchanged to `results/native-journal-gate-v1/`.
The explicit outer-busy negative control returns 64 and creates no candidate
directory. The corrected panel may now start directly, without an outer lock.

## Terminal polynomial panel

`panel-v2` runs from 20:03:47 to 20:39:43 UTC. All 42 collections and
post-run audits exit zero; the native summary reports all 42 complete with
no unverified case-arm. Each arm has 33,696 complete S4 systems, 504 signed
tuples, 15 projective rows and zero zero-rows in case-local totals. All
per-case ranks match the independently point-replayed reference. The full
case table and exact counter scope are in [the report](REPORT.md).

## Archive permission repair

The first archive is 105,608,344 bytes, SHA-256
`8418084d7e25f6076f69e17d46a20a3ede3f67df73699eb4c7fc4b0df0ef74bd`.
Frozen-source reconstruction, native summary equality, all extracted
checksums and all 1,112 native file comparisons pass. The first repack differs:
the session's umask changes 1,072 extracted file/directory permission modes.
Original archive, first extraction, mismatching repack and exit 1 remain.
The separate `repair-repack-permissions.sh` reuses the original archive,
extracts with preserved permissions and repeats the required integrity check.
Its terminal result remains pending until its own exit receipt exists.

The permission-preserving continuation subsequently exits zero. All 1,112
comparisons and the byte-identical repack pass; its separate raw receipts
remain under `results/pack-permission-repair-v1/`. The original exit 1 and
permission-difference record are not rewritten.

The publication check then identifies an additional packaging-policy issue:
the first archive used epoch-zero tar timestamps instead of the study date
required by AGENTS.md section 13a. The separate `pack-with-study-date.sh`
reuses that verified extracted payload, normalizes only tar timestamp/mode
headers, and checks a fresh extraction and repack. It exits zero with all
1,112 comparisons passing. The canonical candidate is now 105,608,944 bytes,
SHA-256 `86a7f68e331a24c4bb1048aab3932ce9b30901c7f64def2cae1f617f93ce8bb8`.
Both earlier local archives, the permission failure and its successful repair
remain. No cryptographic collector, oracle, target stream or census is rerun.
Remote publication is separate and requires its own successful receipt.
