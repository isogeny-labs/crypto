# Session 2026-10-28

What was done: user explicitly requested the fixed-base regularity rerun. Read
prior runner/design/algebra, fixed base identities and primary S3 paper. Checked
live EKS jobs: no active duplicate. Scoped conductor check passed for this study.

Decisions: preserve96 attempts and40s/3GiB per-process budgets, fix ell27, validate
every planted coordinate against stored point data. Use existing Spot host with
pinned Singular container because EKS shared-bucket failure remains unresolved.

Open threads: freeze source/protocol, execute, preserve all outcomes, publish.

Errors: shared docs/session-logs path reserved by T-671; this study-local log
avoids that scope. Docker requires existing sudo access; no permissions changed.
Singular manual web fetch failed; installed version controls remain available.
