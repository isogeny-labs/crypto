# S3 custody readback for full-fixture validation

GitHub artifact automation added commit `df8f6a602ca3` after the [library and touched-example reports](REPORT.md) were pushed. It replaced the 116,853-byte compressed library log and both graph PNG copies with immutable SHA-256 pointers in [this manifest](.artifacts.json) and the [example manifest](examples/.artifacts.json). The local branch integrated that bot commit without changing code.

At **2026-11-16 04:48 UTC**, a fresh `aws --profile ecc-challenges-new s3api get-object` readback from `s3://setec-astronomy-artifacts/store/sha256/<first-two-hex>/<sha256>` returned the following exact byte counts and `shasum -a 256` values:

| Artifact | Bytes | SHA-256 | Readback |
| --- | ---: | --- | --- |
| `lib-test.log.gz` | 116,853 | `448f858797ba7f9d7351d9b7efd1253aa01990e48452180effba6b5e18ad13dc` | exact |
| `fixture-validation.png` | 87,850 | `4c5656bcc8cc1cdb16c29b0081bc7023d09090ac1f3ae927c8ad0daba441e8ce` | exact |
| `examples/example-tests.png` | 88,055 | `fab683bd6ef41f98a58bc9907b958d9a3093145eb63ad66bd7b8658c81296f1c` | exact |

The local readback files are disposable verification copies; the content-addressed S3 keys and committed manifests are the durable pointers. This checks custody of validation artifacts. It is not a new N83 factor-base publication, native v3 S3 push/fresh import, or IC runtime measurement.
