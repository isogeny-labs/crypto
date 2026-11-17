# Fresh S3 fixture replay and training-view portability

The frozen [software fixture](../prime-prefix-restore-fixture/protocol.json) passed
all seven native CLI commands using the separately downloaded binary from source
`06773dcf28f1d77fd43341b7959b7571fa1abbd0`. Driver correction
`06e57fac37a6` changes only the expected archive directory layout.

Three objects were freshly downloaded from `s3://setec-astronomy-artifacts/store`
and checked against frozen SHA256 digests before use: the native binary, the
retained input archive and the earlier export archive. In a new directory:

- Native semantic verification passed for the retained n=7, m=3 curve and twist.
- Both static `features.jsonl` and causal `prefix-features.jsonl` matched the
  previous export byte for byte. The curve has four prefix rows; the twist has
  none because its retained root system is inconsistent.
- Each exported artifact's hash and size, the v5 model-input allowlist, and both
  immutable export retries passed.
- The retained rehashed corruption was rejected with `prime F4 chain input
  differs at domains`. Source inventories before and after were identical.

[passing-result.json](passing-result.json) records exit0 and seven passed
commands. The first attempt passed three native commands but failed its file
comparison because the driver expected an extra archive directory component.
[failed-result.json](failed-result.json), the original driver and complete failed
outputs remain retained; the native implementation was not changed.

[store-index.json](store-index.json) names eight immutable artifacts, and
[readback.json](readback.json) records fresh S3 downloads and byte comparisons
for all eight (189,667 bytes). Both output archives omit only the duplicate
`crypto` executable; its exact bytes remain available through the protocol's
binary digest, `6b3c5204a2fe04c15168288ce7b24b8080a0a4cf21a924225b6a9021af3d7b92`.
The archives retain downloaded input/reference archives, extracted files,
command output, manifests, hashes, protocol and outcome receipts.

This is a fresh-directory software replay using the controller's S3 identity,
not an independent scientific review, the budgeted `dataset restore` publication
path, or a worker IAM test. It validates the stated premerge binary, not the
combined revision still undergoing its separate validation sequence. Scientific
samples added:0; verified cloud dataset cells:0. No trained-model, exact-fall-degree,
complete-native-trace, whole-solver instruction or new ECDLP claim follows.
