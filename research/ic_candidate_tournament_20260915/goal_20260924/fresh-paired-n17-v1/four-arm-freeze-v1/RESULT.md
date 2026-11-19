# Fresh paired n17 four-arm freeze v1

Pre-dispatch freeze of arm order, policy pins, source/scientific seals, and the
shared public-point card for the first fresh-paired n17 campaign. No arm ran.
No registration was consumed. No online speedup is claimed.

## Freeze

| field | value |
| --- | --- |
| status | `FOUR_ARM_FROZEN_NOT_DISPATCHED` |
| freeze.json SHA-256 | `a94df27cd3ed73974787e4cf87d755bbb062f5431645704a61b8c4d45f761aa2` |
| arm order | `rho`, `f5`, `sat`, `incumbent` |
| arm-order method | Fisher–Yates with OS CSPRNG (`secrets.randbelow`) before any arm result |
| arm-order entropy SHA-256 | `e4c7ba783b2650a23cd7049325353b8c3216f7512e1113734df2a06baefeafb3` |
| card SHA-256 | `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379` |
| public target | `[87633, 27]` |
| dispatch authorization | `not_granted_freeze_only` |

## Bound publications

| arm | registration / source seal | archive SHA-256 |
| --- | --- | --- |
| F5 scientific | `e0b2520c58e6f327202328790b0784c8e305963aab788cb3a4d53c455a0cd93e` | `ce9b58765bcfb60f8ab76d4914670b8dca36125821d552db9f9abfadc162ccc7` |
| SAT scientific | `f92ba60dbd41217ee3e846e1dfead1278e0ead237c4e1d010dd1e52a961db578` | `bc0ca591ae634537fab84e04ffef3de7db4f44990edb96490dec1f7ec2204745` |
| incumbent source | `36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf` | (see source descriptor) |
| rho source | `f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59` | (see source descriptor) |

Descriptor digests match the public card's `source_publications` map. F5 and
SAT data-only scientific replays during this freeze returned
`PASS_DATA_ONLY_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY` and
`PASS_DATA_ONLY_SAT_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY`. Both capsules
remain unconsumed.

## Policies (bounds, not retries)

- F5: ≤8 seeded queries, algorithm seed `2026100501`, 900 s outer watchdog
- SAT: ≤8 seeded queries, 1e6 CMS conflicts, 120 s solver watchdog/query, 900 s outer
- Rho: `rho.signed_frobenius_strong`, 32 lanes, `dp_bits=4`, `step_cap_factor=2000`
- Incumbent: qualified pair-table; IC1 stays null until exact pre-card inventory

## Not authorized

One-use `target-control-execute` / `sat-target-execute`, incumbent/rho target
runs, isolated-host promotion, and any closed-control redispatch remain
outside this freeze. Machine-readable record: [freeze.json](freeze.json).
