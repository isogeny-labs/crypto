# Frozen two-shard native generate pilot

The official `experiment.protocol/v1` draft passed the pinned checker and was
committed at `70fb1c122356cdad3308da258c936cd65fcc4e88`. The official
freeze set `filed_before_affected_data=true` at2026-10-31T00:02:03Z;
canonical body SHA-256 `4eb882c13ef3d5b38338eac642cab9af677c5857fe902da6a8eacfd81cbb6783`,
file SHA-256 `4c6bd720bc32330cbb1e5ece05ed267960fa080cc8f49b57adb1bfd693964e21`.
The native plan has64 cells and file SHA-256
`98fc2099a3a32c9212cb1ab124e536523c5366fc914d8545dcb878ec9d290ce6`.
Preflight checked the clean source/binary/protocol/plan joins. Frozen files
were committed at `7d9a9a9be49c` before either generate invocation.

Each invocation selected exactly one shard, index62 untwisted or63 twisted,
from the same plan; `--max-cases 1`, one thread, 2GiB worker memory,
64MiB trace cap, 30s inner worker cap and 90s outer command cap. These were
local non-publishing attempts on source commit
`c41bec150e16e4c4f9a00634002b633b821de262` and binary SHA-256
`b8e0881ec5e46f60c23e6bbb49ad7c3677b9168199f5787b68d89169bfaaa77e`.

| Shard | Full command duration | Sealed cell outcome | Independent `dataset verify` | S3 archive SHA-256 |
| --- | ---: | --- | --- | --- |
| 62, untwisted | 31,397,332,868 ns | Timeout at inner30s; 0 solved | Hash replay true; 1 timeout, 0 independently solved | `19fd40be5bdc03eaf207a826c7fc1cf5ddb3aea1e52bc1c71f256c6c2e0cf8c9` |
| 63, twisted | 43,926,784,367 ns | Verified BSGS scalar recovery | Hash replay true; 1 known-answer-verified and independently solved | `b272d3813fddc507d90b389e56cb6ebee352cea1ed05ee081b69edb60a9b30fd` |

The outer commands both exited0: `generate` records a bounded timeout as a
sealed outcome, so exit0 is not a solved-cell count. The native partial report
was written for each shard. Both strict exports produced one allowlisted
feature row each, with restricted targets kept apart; one row is censored by
timeout and one is verified. The exports assign both n23 rows to the same
whole-field-size train component. No model training or broad accuracy claim
is implied. Whole-worker retired instruction counts remain null. Full
campaign completion remains false and verified cloud dataset cells remain0.

The complete run directories include frozen plan/protocol copies, process
receipts, cell manifests, telemetry, solver artifacts, reports and split
exports. Their restricted S3 objects passed fresh GET byte comparisons.
The 30s cap demonstrably under-provisioned the untwisted parent path relative
to its7.58s direct-worker preflight, so subsequent runtime design needs
parent-path observations and an explicit amended cap; this timed-out attempt
is retained without retry.
