# Runtime to capture attempt binding

Applied after4b99522 passed587 focused tests and strict release Clippy. Binding increment compilation and tests are pending. Sources: capture AttemptBinding.digest/validate, CaptureLimits and CaptureEmitter sticky failure/head; execution LaunchContext original-run validation; solvers solve_dynamic_bound and actual Bound capture/replay control. Existing original-run/typed-capture contracts determine this check; no new mathematical algorithm or performance claim.

Before dynamic work, derive capture limits from the original whole-MiB trace allowance. Require an active Bound sink with the same attempt digest and limits, rejecting stopped/failed sinks. This is a read-only check with unchanged head/event bytes and no sticky refusal on identity mismatch. It does not replace original config/run/execution validation in the parent or independent mathematical replay. Digest/metadata allocation belongs to existing outer admission, not the child's algebra budget.

Extend the actual n47/m4 both-twist runtime test with wrong attempt and wrong trace-limit contexts; assert neither changes the committed head or event stream, then run the correct context. Existing missing-Bound/deadline checks remain. T888 capture scope and T887 adapter scope remain root-owned; no production dispatch changes.
