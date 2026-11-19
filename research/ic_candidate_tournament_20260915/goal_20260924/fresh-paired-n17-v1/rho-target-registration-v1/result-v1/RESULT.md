# Fresh paired n17 rho card-bound scientific registration v1

Tip `icprog rho-target-freeze` / `rho-target-publish` / `rho-target-replay`
bound the public card to the archived rho-source `ecbench` under a one-use
registration. Capsule status is `RHO_CARD_REGISTERED_NOT_DISPATCHED`. No
`ecbench` session ran. Execute with
`--dispatch-authorization not_granted_freeze_only` was refused.

## Inputs

| item | value |
| --- | --- |
| tip | `codex/ic-native-rho-target-registration-20261119` on freeze tip `8a0bd5b3` |
| source publication | [rho-source-v1](../../rho-source-v1/result-v1/RESULT.md) seal `f74601e7…` |
| card | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| public target | `[87633, 27]` |
| four-arm freeze | `a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2` |
| archived ecbench | `60fa3f7a042de090faabffcc09af28a59970c6d112320564b0f0621ecd413d47` |

## Receipts

| field | value |
| --- | --- |
| status | `PASS_DATA_ONLY_RHO_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY` |
| registration SHA-256 | `a849048c792e9819aa2141388e0fe1a1ace3fc9763fc49c2821b3de8737c7287` |
| archive SHA-256 | `4b8f2bedcc489f89a924b69b27625d4e9811e1de733877d78a72f57cb9c6cd77` |
| archive members | `11` |
| scientific_worker_calls | `0` |
| archived_binaries_executed | `0` |
| execution_admitted | `false` |
| dispatch_authorization_recorded | `not_granted_freeze_only` |

Independent publish and repository-path replay both returned
`PASS_DATA_ONLY_RHO_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY`. The original
capsule at `/Volumes/SSD990-2/tmp/ic-rho-target-capsule-20261119013014`
remains unconsumed. Sole `rho-target-execute` requires
`--dispatch-authorization separately_authorized_rho_arm_v1` and the same tip
controller that froze this registration; do not invent that token.
