# Separately prospective successor after output sampler failure

The immutable original attempt failed after20.784042s with FileNotFoundError; no tests ran. Frozen inputs remained unchanged and the owned compiler group was killed/reaped/empty. Original repr lacks traceback/path, so the dynamic target-tree race is an inferred source diagnosis.

Only runner changes: recursively sample target entries with no-follow stat, tolerate only disappearing ENOENT entries/directories, reject unexpected symlinks, retain full failure traceback and observed cleanup exit. Other errors still fail. Candidate and controls are unchanged. Fresh private source/vendor copy and empty target; explicit offline/locked single-job command. Proposed one240seconds total235+5/4GiB with same exact test/outputs/counters and sampled2GiB target bound; source validation only. No retry without root release.
