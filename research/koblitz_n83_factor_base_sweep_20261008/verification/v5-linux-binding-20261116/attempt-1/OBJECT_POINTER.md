# Exact S3 input and build-binary pointers

The [frozen protocol](../PROTOCOL.md) specifies the exact
read-only K600 input. This run performed one fresh GET through AWS
profile `ecc-challenges-new` (account `104186811093`), then
verified 3,025,798 bytes and SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.
The durable object is:

```text
s3://setec-astronomy-artifacts/store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf
```

Retrieve the bytes with a read-authorized identity and compare the
digest:

```sh
AWS_PROFILE=ecc-challenges-new aws s3 cp \
  s3://setec-astronomy-artifacts/store/sha256/3e/3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf \
  ./3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz
shasum -a 256 ./3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf.jsonl.gz
```

The [retained v3 manifest](../../k600-v3-s3-adoption-20261115/attempt-1/manifest.json)
has SHA-256
`d2095084767804af1525665307c9ad1c9eb8dbee2fbdb5adcac9f849e765d119`.
Its `store_name` is a logical name, not the physical S3 key.

GitHub artifact automation moved the 9,884,664-byte
`koblitz_n83_factor_base_export` file from the
[build packet](../../v5-linux-exporter-build-20261116/attempt-5/RESULT.md)
after commit `34b076792b8c`. The tracked
[artifact manifest](../../v5-linux-exporter-build-20261116/attempt-5/.artifacts.json)
binds it to SHA-256
`87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`
and this content-addressed S3 key:

```text
s3://setec-astronomy-artifacts/store/sha256/87/87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236
```

The exact binary remained at the build output path during this Linux
binding run and passed its pre-run SHA-256 gate. The S3 binary pointer
comes from the tracked automation manifest; this run did not perform
a separate S3 readback of the binary.
