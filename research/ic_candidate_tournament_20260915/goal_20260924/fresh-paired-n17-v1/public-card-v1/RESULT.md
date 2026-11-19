# Fresh paired n17 public-point card v1

`icprog target-card-generate` minted one postpublication public-point card
after all four independently checked source publications were present on tip
`a2337c5551b0a187dde2989489eb302c532aade1`. No solver ran, no scalar was
generated, and no online speedup or IC1 identity is claimed.

## Source descriptors

`publications.json` binds four absolute descriptor paths. Descriptor
SHA-256 digests bound into the card:

| role | registration seal | descriptor SHA-256 |
| --- | --- | --- |
| cms | `f92ba60dbd41217ee3e846e1dfead1278e0ead237c4e1d010dd1e52a961db578` | `0e05711802bd21d225f3b532fa083b8751579a2f206999866c5d030d3044dc05` |
| f5 | `bd4c63599b107f97e13d26735bde87296f7800b04ac5df4120c981421df320ee` | `450f66c68d9c4df3242af1c05a35bcc1ef90f11a7470e55655e29fcb4915105d` |
| incumbent | `36923bdb0110841fef8c481b941f134c87076244ed20440684bae3d385c6e6bf` | `fea835bd515e0173c0efb9c679f1717098be7338661d263d04971094c708b5ce` |
| rho | `f74601e7be2ef1c7141fee5a997b38c090e0f19eadbe0d7e843577b59918cf59` | `24d65abf45d33aa566882ec83f0bd73a676af5c2cb431f7a4841dd621b327256` |

SAT and F5 descriptors were already committed. Incumbent and rho
`source-descriptor.json` files were derived from their committed
`SOURCE_SEAL.json` plus `independent-replay.json`
(`registration_sha256`, `source_manifest_sha256`, `archive_sha256`,
`target_free=true`, `source_bound_execution_admitted=false`) under question
`fresh-paired-n17-source-publication-v1`. No seals were invented.

## Card

- card SHA-256 `bd969a4092a329bf093688b9d4715c86d77850c11f115ff47852c9235d7c2379`
- card identity SHA-256 `5cf1214bf7fac7e26bacde23bc5380fef424285caf53eb898748e61228eada83`
- question `fresh-paired-n17-ecbench-public-point-card-v1`
- curve `EC1N17Ckb1hbbe2b5b6b1e6`
- point law `hash_to_subgroup_v1`
- target seed `13942624875975887236`, index `0`, counter `1`
- public target `[87633, 27]`
- ecbench workload id `Wf3040f4f5554`
- ecbench workload SHA-256 `f3040f4f5554a12cd8c8f04bfcf38f021a34e5de5f8248da6f23ee5e88d07927`
- `scalar_generated=false`, `online_speedup=null`, `solver_calls=0`,
  `source_bound_execution_admitted=false`,
  `publication_order_and_freshness_certified=false`,
  `campaign_workload_id=null`

`icprog target-card-audit` returned
`PASS_PUBLIC_POINT_CARD_DATA_ONLY_AUDIT` on the same publications list. This
card does not authorize four-arm dispatch, F5 card adoption, scientific
registration publication, or any speed claim. Those gates remain separate.
