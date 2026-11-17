# Collector implementation status

## Current correction: little-endian native ABI guard validated

PR2801 review comment4404274309 is addressed by tested source
`4a57e45ec74ad878a4ed0ce7ccd68abb4f4d7647`. Native support now requires
little-endian 64-bit Linux x86_64 or AArch64; other targets return unsupported
before opening counters. All four complementary cfg gates use that condition.
The new backend-eligibility regression joins the existing mock tests.

The correction passed26 focused tests, strict release Clippy, and the full
library suite: **5209 passed,0 failed,123 ignored**, exit0. Original failed
host-migration validation and its repair are retained, as described in
[ENDIAN_REVIEW.md](ENDIAN_REVIEW.md). Current source, executable and terminal
logs have verified canonical S3 receipts in
[ENDIAN_VALIDATION_RECEIPTS.json](ENDIAN_VALIDATION_RECEIPTS.json).
The following earlier validation is historical and remains unchanged.

## Earlier validation: full library, focused tests and strict lint passed

Compiled runtime source: `9ab6260a2aa57ca4df3729d6d17f10fefea312f5`.
Toolchain 1.98.0, release profile, clean/full-commit build metadata. All 25 checked
collector mock tests passed, with 0 failed and 0 ignored. Strict Clippy for the
release library, all test targets and crypto binary passed with warnings denied.
Source hashes were checked before/after and the worktree remained clean.

The immutable library test executable is 248471168 bytes, SHA256
`fcf2d574774547a204c2a19f2c17ffe97796c34039f35435d0d24e5073040d57`.
The parent completed one full-library run against that copy: **5208 passed,
0 failed, 123 ignored**, terminal exit 0, 1402.62 seconds. The completed log and
exit record are in the final archive. The historical focused archive deliberately
excluded the then-live full log. No duplicate full-suite run or new live PMU/fork
campaign was started here. The unchanged full suite may attempt legacy counter
calls; these are not a new collector measurement campaign.

No CLI or dataset caller invokes the new CheckedCounters API. CLI integration
tests are therefore inapplicable to this increment; the source reference search
and applicability note are retained alongside the native logs. Legacy API and
dataset records remain unchanged. Hardware-independent, whole-worker and full
execution claims remain false.

The source bundle, executable, completed logs and metadata are preserved in the
canonical S3 archive described by `FINAL_VALIDATION_RECEIPT.json`: 37794116 bytes,
SHA256 `9c01de42001e2368089651eeb6c83ae64c92aef49db579383243e76fbc806a7e`.
Conditional PUT, fresh ETag-bound GET, archive SHA-256/length/byte comparison,
all 35 inner hashes, bundle verification and restored executable byte comparison
passed. The bundle contains tested runtime 9ab6260; documentation head 564638d
has identical runtime, build and test files. This final receipt/status update
also changes documentation only. The archived source STATUS.md intentionally
retains its pre-validation wording; the packet's VALIDATION.json records the
completed gates. `FOCUSED_VALIDATION_RECEIPT.json` preserves the earlier packet.

The delivery is stacked on `codex/ml-binary-root-trace-20261028`, verified at
base `c14d4552e513f6e28bda0e0e045f5e64beb492db` (PR #2782). Hosted checks and
review of the new PR remain separate from these local gates; no main merge is
performed. Phase-schema integration and independent consumer support for such
future records remain pending. This increment creates no scientific dataset
cells or new worker publication evidence.

Preparation diagnostic: sparse materialization succeeded; the first hash list
included a nonexistent build.rs. That list was corrected before Cargo began.
No compilation, focused-test or lint failure occurred.
At final documentation delivery, an explicit rustfmt +1.98.0 check reported that
its rustfmt component was not installed. The already-installed default rustfmt
1.10.0-stable completed the scoped check successfully; no source changed and no
component was installed. This tooling diagnostic occurred after final archive
creation and is retained in the local publication readback packet.

## Implementation history

2026-10-28: native implementation authored after design commit3c7964ae0a7.
This is an unvalidated implementation checkpoint, not a measurement result.

The isolated branch adds isolation::checked beside the unchanged HwCounters API.
The native backend is limited explicitly to Linux x86_64/aarch64; other targets
return unsupported. Per-event records preserve full requested attrs, event IDs,
open/control/read/close outcomes, raw tuples and checked estimates. Policy can
require cycles or allow a separately recorded standalone-instruction fallback
before work. A cleanup failure prevents that fallback. Active state is thread
bound and RAII owns descriptor cleanup, including preparation unwind.

Nineteen mock test functions cover syscall failures, inheritance scope, optional
cycles, ID substitution, tuple math, repeated start, read lengths, cleanup and
unwind. They have been authored but have not yet been compiled or run. Scoped
rustfmt and git diff --check passed. Parent owns Cargo scheduling and final
required native gates. No new live PMU test has been introduced.

Dataset phase schemas and existing historical records are untouched. The new
collector record keeps platform admission, independent hardware verification,
complete execution, descendant lifetime coverage, whole-worker coverage and
comparative clocks false. Kernel control success is reported evidence, not an
independent proof of hardware truth. Cleanup failures remain separate from a
successfully obtained raw tuple; consumers must inspect both.

No PR, cloud worker, fresh scientific measurement or privilege change was made
for this checkpoint. Original maximum USD200 total campaign cap remains in force.

## Pre-build source-review follow-up

The parent identified that Rust thread affinity alone does not prevent use after
fork, and that marking finalization complete before reads could hide owned FDs
from unwind cleanup. The follow-up records and checks original/current PID/TID at
start and finish, rejects missing identities, and preserves sticky typed context
failures. A mismatched context closes only its local FD references, without
resetting/disabling/reading the original owner's event. This matters for inherited
descriptors after fork. No live fork or hardware campaign is used to test it.

Finalization is now marked complete after cleanup; disable attempts are not
repeated while unwinding. Six additional mock tests cover identity changes or
absence before start/finish, unchanged original identity requirements, sticky
failures, mismatched-context drop and one-time control/read unwind. Total authored
test functions:25; still awaiting compilation. The private backend's identity
and close functions must not unwind. Native implementations return OS errors;
cleanup itself panicking, process abort and SIGKILL are outside the RAII promise.

The native ABI guard also explicitly requires64-bit pointers. An x86_64
architecture label alone does not exclude the x32 ABI, whose event-ID ioctl
pointer-size encoding differs. Other ABIs take the unsupported backend.
