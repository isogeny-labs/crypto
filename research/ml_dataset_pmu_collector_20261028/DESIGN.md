# Checked native PMU collector: first increment

Scope frozen before code, 2026-10-28. Base c14d4552e513f6e28bda0e0e045f5e64beb492db.
Conductor T-754 owns isolation.rs, its new isolation/ child directory and this
research directory. Parent owns Cargo scheduling. No tests have run for this
increment. Existing maximum USD200 total and all worker owners remain unchanged.

## Prior source and references

The existing HwCounters in src/cryptanalysis/ecbench/isolation.rs records raw
values and enabled/running times but ignores reset/enable/disable failures,
lacks event IDs and RAII cleanup, and discards instructions if cycles cannot
open. Dataset telemetry and its independent consumer already preserve missing
readings and distinguish diagnostic clocks. The retained software fixtures
contain189 phase observations from51 cells; they are not new measurements.

Full read-only map/design archive:
`s3://setec-astronomy-artifacts/ml-dataset/20261028/design/instruction-measurement/49832dbf9a2ce80cff0895caa1242307c74e6f23432111749e944a19290c1876/design.tar.gz`
(29473 bytes, SHA256 in path). Conditional PUT, fresh ETag-bound GET, byte
comparison and13 internal hashes passed2026-10-28T16:14:32Z.

Primary sources read before selecting the contract:

- [Linux perf_event_open](https://man7.org/linux/man-pages/man2/perf_event_open.2.html): hardware events, targeting, inheritance limits, time/ID read format and control operations.
- [Linux perf security](https://docs.kernel.org/admin-guide/perf-security.html): record permission failures; do not widen access.
- Installed /usr/include/linux/perf_event.h and asm-generic/ioctl.h: attr112,
  time-enabled|time-running|ID read format, event-ID ioctl. Native ioctl ABI is
  initially explicit Linux x86_64/aarch64; unsupported targets do not guess.

## Native contract

Add public isolation::checked beside legacy HwCounters. Preserve legacy APIs,
dataset phase/v1 records, reports/exports and historical artifacts. No dataset
phase-v2 integration, live PMU campaign, cloud job or privilege change.

Policy explicitly chooses calling-thread or calling-thread plus new descendants;
cycles may be disabled, required, or optional with a recorded fresh standalone
instruction fallback before work. No implicit inheritance downgrade. Record
owner PID/TID and keep the active collector thread-bound. Pre-existing threads,
kernel/hypervisor work and unproven descendant lifetimes remain excluded.

Open, checked start and consuming finish retain each attempt, including failed
groups. Per-event records include complete requested attrs/bytes, group binding,
kernel event ID, all lifecycle results, raw tuple, checked arithmetic status and
cleanup. Reset/enable/disable failures invalidate interval admission. Reads
require the exact32-byte value/enabled/running/ID tuple and expected ID. Read or
control failure after work never retries a measured interval.

Keep unsupported platform/event, permission, open/ID/control/read failure,
short read, mismatched ID, invalid time, zero running, estimate overflow and
unstarted/cancelled outcomes distinct. Valid measured zero requires positive
running. Multiplexing produces an explicit estimate with raw values retained;
overflow is not silently saturated. Repeated start cannot reset the interval.

RAII disables/closes owned descriptors on drop or unwind. Remove ownership before
close and never retry close: the FD may already have been released. Finish
retains cleanup errors separately. A destructor cannot persist a completed
receipt after panic/SIGKILL. Whole-worker coverage, execution completeness,
independent hardware verification, platform admission and comparative clocks
stay false. Native operations and Callgrind Ir are not PMU instruction proxies.

## Tests and delivery

Use a private injected syscall backend; no new live hardware test. Cover each
open/ID/reset/enable/disable/read/close failure, unsupported/permission reasons,
optional versus required cycles, EINTR and short reads, ID substitutions,
invalid/zero times, measured zero, estimate overflow, repeated start, premature
finish, cleanup errors, prepared/active drop and unwind. Check attrs/read shape,
scope and false completeness flags, and preserve legacy interfaces/tests.

After authoring, ask parent for a Cargo slot, retain failed diagnostics, and run
required native gates before publication. Software receipt consistency will not
prove hardware truth, full execution, UNSAT, fall degree or comparative speed.

## Setup diagnostic

The new --no-checkout worktree initially had an empty index. Sparse setup reported
inherited paths as staged deletions. The staged-check process was stopped before
commit; only this isolated index was initialized from c14d455. That initialization
removed the uncommitted design draft, which was restored here before freezing.
No parent branch/source changed and no Cargo command ran during recovery.
