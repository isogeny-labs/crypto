# Progress custody development contract

T-1752. The existing producer buffers all raw counts until all three arms complete; a failure before output therefore does not identify attempted target calls. Add deterministic stderr markers immediately before each arm and query, after verified query return, after teardown and after complete panel output. Markers run outside accounting scopes. Preserve producer arithmetic, target set, A/A checks and raw output format.

One local selected native correctness test build (90s) and run (60s), same two development fixtures as predecessor. This is instrumentation validation, not the full20-target comparison. Failure parser tests use synthetic prefixes of the fixed schedule, including truncation before every marker, malformed sequences and partial final lines. No cloud job, no retry. Sources: matched producer, capture wrapper and preserved export schema/linter. Original4563-case cohort unchanged.
