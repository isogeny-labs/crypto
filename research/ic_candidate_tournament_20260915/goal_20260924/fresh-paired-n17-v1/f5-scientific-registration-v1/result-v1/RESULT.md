# Fresh paired n17 MatrixF5 scientific registration publication v1

`icprog target-control-publish-registration` published the unconsumed
card-adopted F5 capsule as data-only scientific custody. The capsule remains
`F5_CARD_REGISTERED_NOT_DISPATCHED`. No target worker ran, no claim was
consumed, and no online speedup is claimed.

## Inputs

| item | value |
| --- | --- |
| capsule | `/Volumes/SSD990/llm/tmp/ic-f5-source-validation-20261006-v1` |
| tip | `dfd0de8909f690509012bab47b9f013e5c8706eb` (PR #4796 merge) |
| adoption archive | [f5-card-adoption-v1](../../f5-card-adoption-v1/RESULT.md) |
| scientific registration SHA-256 | `e0b2520c58e6f327202328790b0784c8e305963aab788cb3a4d53c455a0cd93e` |
| card SHA-256 | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| public target | `[87633, 27]` |
| frozen controller | capsule `immutable/bin/icprog` SHA-256 `63b3c0288d845de81e2bf414ae38ca5507eb9db05b29efb292ea1bc3edb30725` |

## Receipts

Independent publish and repository-path replay both returned
`PASS_DATA_ONLY_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY` with identical JSON
bytes. Appending one byte to a separate archive copy made replay reject it as
`target custody archive bytes differ`.

| field | value |
| --- | --- |
| status | `PASS_DATA_ONLY_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY` |
| scientific_registration_published | `true` |
| archive SHA-256 | `ce9b58765bcfb60f8ab76d4914670b8dca36125821d552db9f9abfadc162ccc7` |
| archive members | `6979` |
| archive bytes | `37105324` |
| PUBLICATION.json SHA-256 | `c0ed0d1bd7842cb86feef41cea77376331aea2de99dab91d4959164ce489a8f4` |
| source commit | `bed4864cca41a214e8907264fb3f1d493036b80f` |
| source manifest SHA-256 | `a34f2e9e61c78ab5ae19c75a3b0649dac6f7ae9dd7adf1c497c05da4eb5bae9f` |
| scientific_worker_calls | `0` |
| execution_admitted | `false` |
| source_bound_execution_admitted | `false` |
| online_speedup | `null` |

The publication retains card, adoption start/terminal, precard validation
sidecars, registration, seal, host context, build receipts, and the immutable
source/binary tree. The original capsule is still unconsumed. Sole
`target-control-execute --publication ...` and four-arm dispatch remain
uncreated. Do not redispatch closed registrations.
