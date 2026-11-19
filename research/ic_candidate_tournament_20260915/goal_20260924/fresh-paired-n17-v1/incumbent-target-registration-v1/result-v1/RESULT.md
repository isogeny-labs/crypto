# Fresh paired n17 incumbent card-bound scientific registration v1

Tip `icprog incumbent-target-freeze` / `incumbent-target-publish` /
`incumbent-target-replay` bound the public card to the archived
incumbent-source pairinv worker under a one-use registration. Capsule status
is `INCUMBENT_CARD_REGISTERED_NOT_DISPATCHED`. No worker ran. Candidate IC1
remains null. Execute with
`--dispatch-authorization not_granted_freeze_only` was refused.

## Inputs

| item | value |
| --- | --- |
| tip | `codex/ic-native-incumbent-target-registration-20261119` on ordinary-registration tip including #4802 (`5a7892d281cf`) |
| source publication | [incumbent-source-v1](../../incumbent-source-v1/result-v1/RESULT.md) seal `36923bdb…` |
| card | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| public target | `[87633, 27]` |
| four-arm freeze | `a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2` |
| archived worker | `b3d8afacc746a34cf15562452dbec3d75deadbb9faf16051e97ad8fa07cbbd66` |

## Receipts

| field | value |
| --- | --- |
| status | `PASS_DATA_ONLY_INCUMBENT_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY` |
| registration SHA-256 | `65024012786d7d53d196ac47b1a84bcf97f278eab0b6bd311d5d61ef5091d945` |
| archive SHA-256 | `c197f9a0c8e0bcd0749c67801e5c259ae8ee78eb74237716a89e77e79b5538c2` |
| archive members | `13` |
| scientific_worker_calls | `0` |
| archived_binaries_executed | `0` |
| execution_admitted | `false` |
| candidate_id | `null` |
| dispatch_authorization_recorded | `not_granted_freeze_only` |

Independent publish and repository-path replay both returned
`PASS_DATA_ONLY_INCUMBENT_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY`. The original
capsule at `/Volumes/SSD990-2/tmp/ic-incumbent-target-capsule-20261119023505`
remains unconsumed. Sole `incumbent-target-execute` requires
`--dispatch-authorization separately_authorized_incumbent_arm_v1` and the same
tip controller that froze this registration; do not invent that token.
