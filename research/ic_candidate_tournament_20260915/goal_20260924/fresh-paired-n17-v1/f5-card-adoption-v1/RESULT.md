# Fresh paired n17 MatrixF5 card adoption v1

`icprog target-control-adopt-card` irreversibly bound public-point card
`bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` to the
existing unconsumed F5 validation capsule at tip
`d973131e6ced3c19bf9122f1a3f5b5e8a7658945` (PR #4792 merge). The capsule's
own frozen controller (`immutable/bin/icprog`, SHA-256
`63b3c0288d845de81e2bf414ae38ca5507eb9db05b29efb292ea1bc3edb30725`) performed
adoption; a newer cache `icprog` is rejected by the frozen-controller check.
No target worker ran, no scalar was recovered, and no online speedup is
claimed.

## Inputs

| item | value |
| --- | --- |
| capsule | `/Volumes/SSD990/llm/tmp/ic-f5-source-validation-20261006-v1` |
| validation seal | `7a59a4a195426f1ece5dbdfa76849547d5258a2bae05d605d19d95a44b92f7da` |
| source publication | [f5-source-v1/result-v1/publication](../f5-source-v1/result-v1/publication/) |
| source seal | `bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee` |
| source descriptor | [source-descriptor.json](../f5-source-v1/result-v1/source-descriptor.json) |
| card | [public-card-v1/card.json](../public-card-v1/card.json) |
| public target | `[87633, 27]` |

## Receipts

Adoption and independent `target-control-audit-adoption` both returned
`PASS_SOURCE_BOUND_F5_CARD_ADOPTION` with identical JSON bytes.

| field | value |
| --- | --- |
| status | `PASS_SOURCE_BOUND_F5_CARD_ADOPTION` |
| scientific registration SHA-256 | `e0b2520c58e6f327202328790b0784c8e305963aab788cb3a4d53c455a0cd93e` |
| card SHA-256 | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| source registration SHA-256 | `bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee` |
| source manifest SHA-256 | `a34f2e9e61c78ab5ae19c75a3b0649dac6f7ae9dd7adf1c497c05da4eb5bae9f` |
| source archive SHA-256 | `cdcd3372d4dc627aaadc630fdcf7284610f1810f3ec30ff9ff83bc8f63ff2446` |
| worker SHA-256 | `f8d64b9b9daf9097dce73d020bb196c3e73be85a765e07985913b6d33c14b749` |
| controller SHA-256 | `63b3c0288d845de81e2bf414ae38ca5507eb9db05b29efb292ea1bc3edb30725` |
| target_worker_calls | `0` |
| source_bound_execution_admitted | `False` |
| online_speedup | `None` |
| adoption-receipt.json SHA-256 | `0ae907f2d4965153fd37a90080805a7e3c4bc1ceabda2d35ae493dea0df1536e` |
| audit-adoption.json SHA-256 | `0ae907f2d4965153fd37a90080805a7e3c4bc1ceabda2d35ae493dea0df1536e` |

Capsule terminal status is `F5_CARD_REGISTERED_NOT_DISPATCHED`. The
scientific registration is local to the capsule; this archive retains the
PASS receipts only. Four-arm dispatch and the sole F5 target run remain
uncreated. Do not redispatch closed registrations.
