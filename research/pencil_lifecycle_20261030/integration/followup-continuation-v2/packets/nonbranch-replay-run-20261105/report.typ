#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Independent norm replay: retained attempt
2026-11-05 · T-1622 · observed native receipt, PCC unverified

The existing frozen replay was invoked once under systemd limits of one CPU, 8 GiB memory, zero swap and 128 tasks. Its source and two restored manifest hashes matched before launch; the exact configuration was published at commit 38485b69d1.

#image("phases.svg", width: 100%)

The pre-custody phase reported the work-deadline assertion after 280.005022528 seconds. The outer command exited 1; systemd reported 4 minutes 40.105 seconds and 6.653 CPU seconds. No Lean compiler was invoked. Compiler and post phases were not started because the precondition failed and no compiler had run.

The intended input closure contains 16,736 files and 3,111,892,318 bytes. Those are declared totals, not completed checks: the failed wrapper preserved no completed-file count. Inputs reside on NFS4. The exact I/O or metadata bottleneck was not isolated by this attempt.

The campaign conservatively charges the full 300-second phase allowance. This leaves 46,500 seconds of the newly authorized 46,800-second worker-time ceiling, subject to other charged work and the November 6, 04:00 UTC stop. Historical charges remain separate. The owner's instruction waives dollar-fact admission; it does not create a mathematical result.

The failure, native output, supervisor exit and live/terminal resource readbacks are retained. A separate continuation proposal may add bounded progress checkpoints and a prospectively reviewed custody allowance; this record and its source remain unchanged.

No theorem passage, new cohort answer, comparative timing or cost ratio follows from this failed custody check. Machine verification status: not verified.
