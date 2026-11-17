## What was done
Prepared prospective v4 worker/launcher patches, qualification source and plan; copied immutable source recipe and unchanged snapshot helper. Parsed Python syntax only, executed no functions/tests/compiler/payload. T-2031 owns scope.

## Decisions made
Keep10s permit,15s lease margin, all root test/resource limits and old failures. One expired-only bounded reread; publish owner-bound permit ahead of cumulative checkpoint after current identity validation. Actual ledger/cache/lease remain unset.

## Open threads
Independent reviewer, source publication, one prospective software qualification, fresh post-v3 cache snapshot and parent manual release.

## Errors
One source-preparation helper had a missing closing parenthesis and failed before changes; corrected helper. No runtime receipt or experiment altered.

## Independent review corrections
Required attempted launch and nonempty current cgroup before permit publication; added missing/changed cgroup, prelaunch unit and wall-time-expiry controls (24 total). Replaced stale copied proposal with prospective v4 null bindings. No execution.
