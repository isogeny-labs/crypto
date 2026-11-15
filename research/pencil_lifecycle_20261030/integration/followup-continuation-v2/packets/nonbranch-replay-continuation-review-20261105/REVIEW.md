# Prospective continuation source review

Result: revision required for published d2682c2340865a8b82afcc71d1bf7f2eac90b84b, freeze 74b03e071fab8fea1727ec451d635e0a55ba55ebdfafe42ac19f2cca09ffaa1c. This independent review follows the preread PLAN and makes no execution or new kernel claim.

All twelve frozen files match their hashes and exact published Git objects. The continuation preserves four compiler calls, ten producer plus twenty reviewer named readbacks, the mathematical negative control, full fresh 16,736-file / 3,111,892,318-byte custody in each pre/post phase, runtime pins and fail-closed partial status. Historical PLAN/REPLAY are intentionally retained verbatim; CONTINUATION changes timing and diagnostics. No native dependency corpus was rehashed by this review.

## R1: compiler receipt failure skips required post custody

At launch.py lines 45–46, read_text, json.loads or native['status'] may throw after compiler execution. The outer handler then exits before recording the compiler outer row and before post custody. This is not false passage, but it violates the stated post-after-any-compiler-attempt requirement. A malformed receipt, unreadable receipt, JSON null or missing status supplies a concrete source-level path. Guard decoding and shape validation, retain a failed compiler row, then attempt post when budget and confirmed service termination permit. Otherwise record explicit incomplete custody.

## R2: termination allowance and launcher wait are not bounded explicitly

At launch.py lines 33–44, RuntimeMaxSec is set, but TimeoutStopSec is not, and subprocess.run(systemd-run --wait) has no timeout. Elapsed acceptance checks run only after return. A stuck process or shutdown grace can exceed the advertised phase/cumulative ceiling. Pin a short stop grace inside the phase budget; bound launcher waiting and cleanup; reconcile service ownership before post or any later work. Uninterruptible filesystem I/O can outlive SIGKILL, so record unresolved ownership and stop without overlap rather than promising physical termination.

## Preserved limits and next review

The 46,800-second campaign retains a 300-second debit for the original pre failure (280.005 seconds, zero compiler calls), leaving 46,500 before this proposed 2,400-second attempt. Deadline remains 2026-11-06 04:00 UTC, one CPU, 8 GiB and no retries. The owner waiver concerns dollar-accounting facts only. Source correctness of the unchanged theorem is outside this operational review. Publish a minimal revised wrapper/freeze, then independently review the delta before execution. The parent retains admission and scientific-promotion authority.
