# Full dimension-18 direct-orbit factor base: construction and S3 custody

**Status:** the complete 32,643-class factor base was constructed, generically
replayed twice (before upload and after a fresh S3 GET), and published as an
immutable content-addressed object with a hash-bound S3 manifest. The exact
primary curve is `icv1-f2m83-tm6151469093347-debefd74`, Koblitz `a=0`,
prime subgroup order `2417851639230796216685689`. The [protocol](PROTOCOL.md)
and exporter source were committed at `5bac5f1d9221b908f743520aa9039dd48d175f0e`
before construction. The source list is the archived census's complete
32,643-class CSV, rather than its previous first-16,627-class selection.

| Property | Exact result |
| --- | --- |
| Policy | `direct_subgroup_x_d18_full` |
| Full signed-Frobenius columns | 32,643 |
| Distinct point rows | 5,418,738 |
| Compressed bytes | 160,747,529 |
| Compressed SHA-256 | `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b` |
| Compressed BLAKE3 | `c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f` |
| Sorted point-set BLAKE3 | `31aa8e1c1340f4db4898144af59f6f34c87fa63e0d88b175b854f58ce9008a7d` |

The [export manifest](manifest.json), [generic replay](replay.json),
[fresh-object replay](fresh-replay.json), and exact process receipts bind the
source commit and binary SHA-256
`a3f9bce17c8a427a4538e2d4c26031d1300f99a6d33a2c4e6cc442034618e53a`.
Local export took 65.928 seconds; generic replay took 42.632 seconds and the
freshly fetched object replay took 40.792 seconds. These are L0 shared-host
process walls, not matched construction or complete index-calculus times.
The generic multi-limb backend checked all representatives' curve,
subgroup and Frobenius identities, every point/phase/sign/coefficient row,
closure, uniqueness and the sorted point-set hash. Three [semantic and
compressed-byte controls](negative-controls.json) passed: altered header and
first-row labels were rejected despite internally recomputed hashes, and a
changed gzip footer was rejected. The control script is
[`negative_controls.py`](negative_controls.py); it uses `blake3==1.0.8`.

The object is stored at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz`.
The immutable [S3 manifest](s3-manifest.json) is at the same prefix under
`manifest.json`, SHA-256
`1b93351febd47dac4de8c5dd39dbf58564d0057a044054baf448d978f6c0d616`.
Both PUTs used `If-None-Match: *`, SHA-256 checksums and AES256. A fresh GET
matched the object SHA-256 and byte length; fresh HEAD/GET provider responses
also matched their checksum and ETag. The independent
[`verify_s3.py`](verify_s3.py) checked both local and fetched BLAKE3/SHA-256,
the two generic replays, the exact object and manifest provider responses,
curve metadata and the manifest's source/replay hashes. It returned
[`PASS_FULL_OBJECT_AND_MANIFEST`](s3-independent-audit.json).
Two [receipt-tamper controls](s3-tamper-check.json) rejected a false HEAD
length and a changed downloaded manifest point-set hash.

The complete D18 base's four-summand unordered uniform-target counting mean
is about **14.858**, versus **1.000** for the earlier 16,627-class D18 base.
This is a combinatorial support comparison, not an empirical success rate.
The larger base also doubles orbit columns and increases model/import and
linear-algebra costs. There is no natural relation, useful rank, factor-base
log table, unseen-target descent or complete one-target cold-runtime
measurement for this new base. `selected_best_total_runtime` remains `null`.
The next discriminating gate is a source-pinned, same-target four-summand
natural-query comparison with the prior half-size D18 base, independent
group/rank replay, full build/import/failed-search charges and matched rho.
