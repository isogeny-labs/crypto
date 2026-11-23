# Validation after merging the base

Measured experiment source remains660f0f4b10 with unchanged worker ELF3a31bd2b…;
its inherited4398-pass/118-ignored validation remains separately archived.
The later base merge94d97911e38a48d2f7413a722df5b519e6a2bf09 imports main's
concurrent code. No N83 solver/worker optimization was added in this merge.
Only benchmark/problem index additions conflicted; all distinct entries from
both sides were retained. Earlier Copilot conflict requests were unanswered,
including one over an hour old before the local fallback.

Command (serial, same new integration target):
`cargo test --release --lib -- --test-threads=1`.
The complete run reported4398passed/9failed/118ignored in640.72seconds.
Every failure was `PermissionDenied: Operation not permitted` while binding
local TCP fixtures in pollard_collab, caused by the sandbox's socket limit.
The failed run is preserved in post-merge-full-lib.log; it is not relabeled
a successful single all-tests run.

With local socket permission, reran the affected module:
`cargo test --release --lib cryptanalysis::pollard_collab:: -- --test-threads=1`.
All38module tests passed, including all nine originally denied fixtures,
0failed/0ignored/4487filtered,0.61seconds. No test was skipped, disabled or
changed to obtain a pass. No remote service is contacted by those loopback
fixtures. This is a successful targeted environment-corrected rerun, not a
second full all-tests run. There are no unresolved test failures from that run.

Exporter/model-checker results are recorded in their complete separate logs.

`cargo test --release --example koblitz_n83_factor_base_export -- --test-threads=1`:
19passed/0failed/0ignored,0.86seconds.
`cargo test --release --example koblitz_n83_free_selector_export_control -- --test-threads=1`:
2passed/0failed/0ignored,0.00seconds. Both use the merged source/integration
build target; the measured historical worker target is never rebuilt or overwritten.
The existing native leaderboard render-only consistency check passes.
