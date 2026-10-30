# Source, evidence and fresh restore

The validated source/evidence archive was conditionally uploaded to the canonical
bucket and freshly downloaded on2026-10-29. The downloaded archive matched every
byte and SHA-256; all733internal file hashes and the source Git bundle verified.

`s3://setec-astronomy-artifacts/ml-dataset/20261029/consumer-checked-phase/final/03bfd537bf9809f9102bdfd9b5ecea91e5dfde404a5ee2d117a1f7ee83d99441/checkpoint.tar.gz`

- Archive SHA-256: `03bfd537bf9809f9102bdfd9b5ecea91e5dfde404a5ee2d117a1f7ee83d99441`
- Archive size:2943449bytes.
- Source/evidence commit: `8558e2b8edad6a9635e8532c077fb1a52d2c22f5`.
- Validated runtime commit: `0ceec70eb18d0b0c8642e09b68879f527cd23bdf`.
- Restored consumer executable: `2512b52b8d4061febd068746b0e15fffb72cc5c1bfb8e4f54f45469236d8523c`.

The archive includes the standalone source, dependency lock, executable, final
tests/lint logs, all49exact retained producer fixture files and their separate
producing-source/executable provenance, both source reviews and original failures,
40legacy differential outputs, and eight rehashed checked tamper copies/errors.
Historical legacy input bytes remain in the already verified archive named in
the embedded receipt. The small progress checkpoint is retained separately.

After the fresh download, the restored executable ran actual
`audit --arithmetic` and `export --arithmetic --allow-unreplayed-preview` against
both restored fixture roots. All four commands exited0. Both exports were
byte-identical to their pre-upload counterparts:

| Fixture | Starts / ends | State | Export SHA-256 |
| --- | --- | --- | --- |
| sealed-worker | 4 / 4 | completed_recorded_prefix | `22c971a476c3e86861fb48f8101864d0c4d891ebbadf1227180be2351529ffa7` |
| sealed-registryless-prefix | 1 / 0 | interrupted_or_unfinished | `344134db26f2cac2479200b3b5f61508f44607489a827fc030eb9ac0231dd259` |

The exact readback and restored-replay receipt is [durability.json](durability.json).
This is controller archival and local mocked interoperability. It is not worker
publication, PMU truth, a newly generated scientific corpus, admitted training
cost, or verified partial-snapshot lineage. No producer was invoked by the
consumer, and all corresponding false/null flags remain unchanged.
