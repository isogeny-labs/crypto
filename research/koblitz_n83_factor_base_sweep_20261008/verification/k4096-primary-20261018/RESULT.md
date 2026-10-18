# Primary N83 K4096 base: construction, full replay and S3 custody

The frozen `a=0`, `public_x_hash`, seed `2026100801` construction produced a
**4,096-column signed-Frobenius base with 679,936 distinct point records**.
Its 20,621,466-byte compressed object has BLAKE3
`a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779`
and SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`.
The point-set BLAKE3 is
`671c2d4d3e2f13276b474ae5e011d89c15084521eaa38df86ae989369395e3ba`.
The exact object is stored at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/v2-size-frontier/a0/objects/a84572e6ca52efa7867082261bf0543723d52c0d136061a45c970970716aa779.jsonl.gz`.
The uploader downloaded it and checked BLAKE3, then an independent fresh S3
HEAD/GET matched SHA-256, BLAKE3, 20,621,466 bytes, content-addressed
metadata and HEAD/GET ETags. S3 reported AES-256 encryption.

The [protocol](PROTOCOL.md) froze one cell and three separate caps. The
construction supervisor returned `PASS_construction_only` from source commit
`f61b33c1b1bb98efa22177a615270c160859a605` under a live-inspected
one-CPU, 4-GiB, zero-swap, networkless, read-only-repository Docker guard.
Its static Linux worker SHA-256 was
`21817b5218b59b5227de5c705e44710583ef70bc8863a5c544d93853a98b5eed`.
The native arm64 replay/upload worker SHA-256 was
`5881dc22b3367b6076ef5c3d452eb270bfa3c4013ec63782a0276b4670cdc094`.
The generic multi-limb replay checked every point, representative, subgroup
projection, Frobenius identity, label, closure and point-set hash before S3
publication. A separate Python audit scanned every freshly downloaded row,
confirming 679,936 unique points and each sign, coefficient, Frobenius step
and orbit closure. The [same audit on a retained metadata copy](evidence/retained-copy-audit.json)
passed after rehydration with the S3 bytes. A forged replay receipt with an
altered point
count and a recomputed outer hash was [rejected](evidence/negative-control.json).

| Stage | Receipt | Observed wall | Frozen cap |
| --- | --- | ---: | ---: |
| Guarded construction and cleanup | `PASS_construction_only` | 233.702 s | 900 s |
| Generic full-object replay | `PASS_generic_replay` | 6.337 s | 1,800 s |
| Native upload and download check | `PASS_s3_round_trip` | 5.632 s | 600 s |
| Fresh S3 byte audit and independent row scan | `PASS_CONSTRUCTION_REPLAY_S3` | Separately retained; no combined IC timing | — |

The three supervised stage walls sum to 245.672 seconds. Build, preparation,
fresh S3 audit, and earlier sweep work are separate; the protocol's
conservative stage envelope is 3,300 seconds. The Linux exporter ran under
emulation, so construction wall time does not rank native IC variants.
An initial attempt was [rejected before container launch](evidence/preflight-failure.json)
because its scratch output was inside the frozen repository; the corrected
run used external scratch and retained that first failure.

The [source-derived next-model projection](evidence/source-projection.json)
is 131,958 total SAT variables, 1,773,110 orbit-domain clauses and
2,021,197 total clauses for a five-summand all-affine K4096 circuit. It
reproduces the earlier K2048 domain count, but is not a constructed K4096
SAT model or a solver timing. The nominal five-summand multiset count divided
by subgroup order is about 500.882 here versus 15.653 at K2048; this is not
an observed relation probability. No natural search, useful rank, column
logs, individual-log phase or complete cold comparison was run in this gate.
The [independent audit](evidence/audit.json), [validation record](VALIDATION.md)
and [evidence manifest](evidence/evidence-manifest.json) retain the exact
scope. `total_index_calculus_runtime_ms` and
`selected_best_total_runtime` remain `null`.
