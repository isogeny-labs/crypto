# Local survey resume after the clean 4,037 checkpoint

`envelope.json` freezes the operational change before launch; `launch.json`
records the actual PID and timestamp; `verified.json` confirms a detached,
non-zombie process (PPID 1, own session) and newly audited native progress.
Outer guards increased from 24 hours / 2 GiB to 72 hours / 6 GiB after verified
redundant local cache removal. Frozen driver, queue, solver and all child
mathematical/search/watchdog budgets are unchanged. It remains a bounded
survey, not the full original matrix. New runtime epoch; clocks unqualified.
No builds, audits or compression were run concurrently after this launch.

The Drive checkpoint remains 4,037 vertices. A process is not a durable
scheduler; verify current process/ledger state after environment recovery.
There is no guarantee of host survival or unattended upload. PR #2106 has a
one-off follow-up check scheduled around 21:04 UTC, subject to its CI/review
rules; this is not a mathematical completion or upload scheduler.
