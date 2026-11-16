# Same-request controller supervision

The first c1 controller ended with CodeBuild status `FAILED` at
06:27:08.972 UTC on 2026-10-30; its BUILD phase reports `BUILD_TIMED_OUT`.
AWS automatically started its successor at 06:27:09.164 UTC. The root
executor did not issue another StartBuild or scientific submission.

The durable state before and after preserves the request, worker instance,
Pod UID, worker attempt 0 and restores 0. Only controller ownership changes.
These are infrastructure observations, not completed mathematical results.
Worker time is charged continuously, including the ownership transition.
Controller costs remain separate; no cost is inferred to be zero.

The pre-discovery manual takeover review is preserved, but its proposed
StartBuild was never executed. AWS AutoRetryLimit 2 means two additional
retries, potentially three controller builds; the recorded earlier limit
was two total attempts. The version-5 protocol prospectively distinguishes
four maximum controller invocations from the unchanged two maximum
scientific/worker attempts and one restore. It must be reviewed, committed
and separately admitted before the third controller starts. Version 4 and
all earlier observations remain retained. This changes no scientific input,
instrument, analysis rule, cap, six-hour allowance or absolute deadline.

A fourth controller is conditional on terminal predecessor and matching
live state, and must disable AWS automatic retries for that invocation.
There is no shared-project or IAM/RBAC update. The current shared project
reports a 1,500-minute timeout whereas actual builds report 45 minutes;
the discrepancy is retained without an unsupported explanation.

Primary reference: [AWS AutoRetryConfig](https://docs.aws.amazon.com/codebuild/latest/APIReference/API_AutoRetryConfig.html).
Repository references: `.agents/skills/isolab-eks/references/recovery.md`,
the pinned controller `cloud.rs` ownership/CAS path and `model.rs` request
binding, and the experiment-protocol amendment rules.

Native protocol validation is in `v5-native-validation.json`. An initial
metadata candidate used a timestamp where the amendment schema requires a
date; it failed validation and was corrected before review or admission.
No scientific program was executed by these metadata checks.
