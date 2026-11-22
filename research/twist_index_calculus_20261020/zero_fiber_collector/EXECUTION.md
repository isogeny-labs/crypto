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

## Publication transport and sparse-index corrections

The first transport stops on terminal input in the busy wrapper's child
process group. It has no AWS child and no publication receipt; the verified
publication process group is stopped and its state/exit 247 retained in
`results/publication/transport-attempt-v1/`. A foreground reader now pipes
input through the wrapper without writing credentials to disk.

The first actual publication attempt authenticates successfully, then fails
with `FileNotFoundError` because `store/index.json` is tracked but absent
from this sparse worktree. The failed `store-publication-v1.json` is kept.
No remote-head or push command ran in that attempt. Materializing the exact
tracked index does not change its contents. A new attempt uses its own v2
receipt and rechecks the alias and immutable remote object before pushing.

The v2 publication passes. The previously absent alias and immutable object
are reconciled before the single native push. Native cache/remote verification
and S3 whole-object SHA-256 and size checks match the canonical archive.
All unrelated index entries remain byte-value equivalent. Failed v1 and
terminal-transport records remain alongside the successful v2 receipt.

## Phase 2 build and positive-control history

The prospective [execution supplement](PHASE2_EXECUTION.md) was committed
before any Phase 2 target solve. The new public-only pipeline and separate
post-run prefix audit use the old complete physical sets solely as replay
references; they do not rerun either oracle or the completed 189+21 panel.

The first two `phase2-check` compilations exit 101: v1 requires parentheses
around an array expression inside `json!`; v2 requires an explicit integer
type for a conversion used by `AddAssign`. V3 exits zero. Their logs and
pre-compilation source seals are retained under separate raw directories.
The first v1 source-copy overlaps the one-line syntax repair and fails its
seal check; `reconstructed-source-v2` reverses exactly that edit and matches
the complete pre-compilation manifest. Both the failed copy and successful
reconstruction remain, with the explicit reconciliation. V2's original
source snapshot matches. These are compile failures, not solver outcomes.

`phase2-bootstrap-build-v1` builds successfully. Native baseline selection
and compact p37 replay-fixture projection both exit zero in
`phase2-preregistration-v1`; their outputs are committed before comparison
runs. The immutable baseline selection reuses 20 registered plain controls
and adds only p97 seed1's `union-l2`, giving 85 configurations rather than
105 duplicate executions.

`phase2-native-gate-v1` is terminal with all three phases exiting zero,
89 passing tests, no failures and unchanged compiled-source seals. Its
actual p37 seed0 first-run records cover zero/plain phi/doubling, four
independent physical-set/RREF replays and two extra zero-phi A/A records.
They recover and verify the target and factor points. All raw records remain
under the gate's `test-artifacts/p37-roundtrip/`. The runner is SHA-256
`daeb82aec36b876927d98bd48051fc4bf9b28d64c3e5a18b6c5f8e02c04dad37`.

A subsequent replay review finds that converting an array to a set can hide
duplicate tuples. The auditor is tightened to reject that corruption, with
an explicit mutation control. `phase2-native-gate-v2` is a fresh gate for
that source revision; v1 remains unchanged. A separately built Rust
workload/reducer tool and a driver with per-phase reservations prepare the
full frozen comparison. At this checkpoint those builds are queued behind
the common busy reservation; no full Phase 2 panel has started, and no new
timing admission is claimed.

The fresh `phase2-native-gate-v2` subsequently completes with all three
phase exits zero, **89 tests passed, zero failed**, and stable compiled-source
seals. Its tightened auditor rejects duplicated physical tuples. The same
actual p37 recovery and A/A controls pass again. The independent panel-tool
build then starts under the next reservation. No full panel is launched
until its build, frozen workload and current launcher controls pass.

The standalone metadata build, full public/reference preparation and final
launcher controls subsequently all pass. The controls reject nested outer
busy with 64 before output creation and an existing phase with 65 without
changing its marker. The prepared 85-configuration workload and all binary
hashes are committed in `af66996a19` before any registered panel execution.
The freeze refers to that full panel; earlier p37 test-build positive
controls are separately preserved. Native reduction of those retained
records gives 153 zero-plus-phi systems, 288 zero-plus-doubling systems and
72 for each plain arm, all at one target query and rank 3/3.

The launch first verifies no existing unit or output directory, both frozen
binary hashes, workload seals and equality with the tested driver. The
managed `twist-zero-pipeline-panel-v1.service` starts with one owner and the
original resource/deadline envelope. Its receipt is in
`results/phase2-panel-v1/`; the full comparison's terminal outcome remains
pending. No old source-side waiting lane, Diem generator or EKS job restarts.

The control report's first diagram inspection finds labels extending beyond
two boxes. V1's original PDF/diagram/source remain, v2 corrects the layout
and is actually inspected, and the final follow-up adds all four native
p37 count rows. A queued v3 document-layout reservation is stopped before
any renderer or output directory exists; its exit 143 remains. Lightweight
v4 artifact rendering completes directly, from copied sources, with both
actual PDF pages inspected and a 150 dpi graph PNG. No science process is
stopped and no scientific computation is repeated for that layout repair.

The source-reuse check confirms the root library/examples remain identical
to the previously tested integration source. Its first protected-parent
check uses the wrong relative working directory and returns 1; the separate
correct-directory v2 returns 0 with both original hashes unchanged. Raw
receipts keep their original whitespace. Full Phase 2 archive publication
remains pending with the managed panel, as [recorded](phase2/ARTIFACTS.md).
