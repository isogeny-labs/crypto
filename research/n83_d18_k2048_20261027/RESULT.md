# Primary N83 D18 K2048 factor base: full export and replay

The sorted direct-subgroup D18 census yields a verified nested K2048 base on
the primary `a=0` N83 curve. The committed Linux arm64 exporter constructed
2,048 signed-Frobenius columns and **339,968 distinct subgroup point rows**;
the generic multi-limb replay checked every representative, coefficient,
point, orbit closure and point-set digest. The 10,071,459-byte object and its
manifest and replay receipts passed a fresh S3 download and independent
SHA-256/BLAKE3 comparison. The [custody receipt](evidence/s3-custody.json)
records `PASS`.

| Item | Exact result |
| --- | --- |
| Curve | `EC1N83Ckb0h252dacc102a4` (canonical record: `research/ecdlp_surfaces_n83_20261012/runs/run-20261012T0331Z/identities/curve_record_a0.json`); legacy slug `icv1-f2m83-tm6151469093347-debefd74`; subgroup order `2417851639230796216685689` |
| Factor-base policy | `direct_subgroup_x_d18_prefix2048`; first 2,048 rows of the fully hashed 16,627-row sorted census selection |
| Columns / point records | 2,048 / 339,968 |
| Producer source commit | `a1e6b778dcee8f4ec72f359df7ce3728b302b78a` |
| Linux arm64 binary SHA-256 | `a0f9b7fe67ffea4816d077b8017e3ed4512cc36fd04689e3eae1257975831a5d` |
| Container image | `sha256:7c4ae649a84014c467d79319bbf17ce2632ae8b8be123ac2fb2ea5be46823f31` |
| Compressed object SHA-256 / BLAKE3 | `2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7` / `a3c06a35db07b7aa0d9e7f45e8b747d5c136f2ea1a0fdc0e8822ffed84659f68` |
| Point-set BLAKE3 | `2b5ac37ec3779026278f9debf1a3be6fdfdcff6ff7838f78f1eed4bee2f04b0b` |
| S3 object | `s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-direct-subgroup-k2048-v1/a0/objects/a3c06a35db07b7aa0d9e7f45e8b747d5c136f2ea1a0fdc0e8822ffed84659f68.jsonl.gz` |
| S3 HEAD | 10,071,459 bytes, AES256 |

The frozen [protocol](PROTOCOL.md) used the pinned Linux image, one CPU,
4 GiB cgroup memory and swap ceiling, no network, read-only source, separate
export/replay containers, and a 900-second inner and host watchdog. Both
exited zero without OOM. Export took 4.419 s inside the process and 5.594 s
outer wall, with 74,735,616 bytes cgroup peak. Replay took 2.624 s inside
and 3.602 s outer wall, with 85,483,520 bytes cgroup peak. Compilation,
process launch, S3 transfer and the full library suite are outside these
informational setup measurements. The exact [manifest](evidence/manifest.json),
[replay](evidence/replay.json), and export and replay guard receipts are
retained in `evidence/`.

The matched U control is the retained `public_x_hash`, seed `2026100801`,
K2048 object with compressed BLAKE3
`82990cda5d57b1c4c4b635673a881ad233c24f707e7ac644a11478aeddddf851`
and point-set BLAKE3
`d528f5828a05a00add4252212bc909ece481f33093e4074ff7dd8c6a51d3c97e`.
The two bases have equal point and orbit-column counts, so a future U/D
comparison can isolate their point-set choice. The exact five-summand
uniform-target subset count is `binomial(339968,5)/r = 15.6518808172` for
either size; this count does not measure solver yield.

A fresh GET of the U archive matched its retained compressed BLAKE3. The
[independent structural audit](evidence/structural-overlap.json) parsed and
uniqueness-checked all 339,968 rows in each archive. Their point-set
intersection is **zero points and zero signed-Frobenius orbits**. Thus the
matched K2048 comparison changes the entire base point set while preserving
its count and curve; the overlap result is a structural identity, not a
relation-yield or runtime preference.

At commit `9e3a23f0e3607fa6ead297367b8d0c5c1905ae1a`, the full macOS
`cargo test --release --lib --locked` run finished with 4,898 passed, four
failed and 122 ignored in 912.75 seconds. The retained
[exact-head log](evidence/lib-test-postmerge.log.gz) identifies two GPU bench
tests that invoke absent `/bin/true` on macOS and two `jv_cover` assertions.
No library source, `Cargo.toml`, or `Cargo.lock` differs from the parent
branch in this study. These inherited suite failures are separate from the
K2048 exporter, whose touched example test and Linux export/replay passed.

The next measurement is a source-pinned U/D K2048 importer and a frozen,
same-point natural-query panel with independent relation/rank replay. The
candidate ID, natural yield, final matrix, target logarithm, same-point rho
and end-to-end timing fields remain null in this construction receipt. The
prior K16627 U/D m5 panel ended with four `UNKNOWN_budget` natural arms at
600 seconds each; this smaller base tests a different solver-domain size.

The [native amendment](NATIVE_AMENDMENT_1.md) records a Rust guarded
export/replay rerun and independent Rust custody and full-row overlap audits.
Their results match the original construction identities; the original
Python source snapshots and timing receipts remain historical evidence.
The [visual report](reports/run-20261027T1915Z/REPORT.md),
[figure](reports/run-20261027T1915Z/factor_base_overlap.svg), and
[PDF](reports/run-20261027T1915Z/REPORT.pdf) present the construction result.
