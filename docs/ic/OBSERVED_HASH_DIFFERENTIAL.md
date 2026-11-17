# Observed hash differentials

`crypto cryptanalysis hash-differential-observed` measures output bit distances
for caller-supplied pairs of equal-length messages with one fixed XOR input
difference. It uses the same hash dispatch as `crypto hash`, including SHA-256,
the SHA-3 family, and SHAKE. SHAKE needs an explicit `--output-bytes` length.

```sh
crypto cryptanalysis hash-differential-observed --algorithm sha256 \
  --input docs/ic/fixtures/hash_differential_sha256_public.json
crypto cryptanalysis hash-differential-observed --algorithm sha3-256 \
  --input docs/ic/fixtures/hash_differential_sha3_256_public.json
```

The input JSON has `input_difference_hex` and `pairs`. Each pair has `m0_hex`
and `m1_hex`. The command requires a nonzero difference of 1–4096 bytes,
1–10,000 pairs, distinct messages, and exact XOR agreement for every pair.
The file is limited to 64 MiB. `--input` accepts a local path or a checked
`store:` reference. The report gives the bit-distance histogram, minimum,
maximum, mean, and equal-digest count. A zero-distance pair would be reported
as observed; the command does not infer a collision attack from a small sample.
The output digest is limited to 128 bytes for this measurement.

The two public fixtures use one-byte messages `(00,01)` and `(02,03)` with
input difference `01`. Independently computed Python `hashlib` distances are
139 and 116 bits for SHA-256, and 132 and 125 bits for SHA3-256. Their means
are 127.5 and 128.5 bits respectively. These four fixed observations are
command checks, not statistical evidence about arbitrary inputs.
With `--algorithm shake128 --output-bytes 32`, the same messages give 115
and 128 output-difference bits; those distances were checked against Python
`hashlib.shake_128`.

Both fixture names passed live Setec S3 upload or content-addressed reuse,
HEAD, remote download verification, fresh-cache `store:` replay, byte
comparison, explicit pull, and exact-object cleanup. They contain identical
message pairs, so the store used one object for the two names. The isolated
prefix and checks are in [the acceptance receipt](../store/HASH_OBSERVED_S3_ACCEPTANCE_20261022.json).
