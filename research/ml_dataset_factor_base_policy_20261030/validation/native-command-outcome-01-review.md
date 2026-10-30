# Parent static review

The supervisor preserves ordinary exit codes and represents signal termination
with -1. The new process predicate explicitly excludes negative status before
comparing the success expectation. Timeout and cancellation exclusions remain.
The table covers both expectations, statuses -1/0/1/2/137, and both independent
supervisor flags. An explicit normal exit137 is distinct from a signal death;
semantic diagnostic admission remains a required later gate for negative controls.

Only run.rs process admission and its unit tests change. No command arguments,
budget, timeout, supervisor cleanup or dataset source is changed. The installed
rustfmt check passed. Native compilation/test is pending. This is parent static
review, not independent review. Preserve source06 and current snapshot08.
