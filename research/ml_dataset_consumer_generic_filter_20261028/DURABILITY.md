# Canonical software checkpoint

The final source-and-validation checkpoint is:

`s3://setec-astronomy-artifacts/ml-dataset/20261028/software/consumer-generic-filter/1052eb63d104f6346d00e67c29ebf6194f2e073a034313394f52220012c54776/checkpoint.tar.gz`

It contains 62,712,592 bytes, with SHA256
`1052eb63d104f6346d00e67c29ebf6194f2e073a034313394f52220012c54776`.
Conditional PUT, ETag-bound fresh GET, full byte comparison and all 4,088
contained hashes passed. [durability.json](durability.json) records the result.

The archive includes source at
`36050250828d1705e2049256836bf2bc0cb87695`, whose native implementation is
byte-identical to validated `086b4cd590d2e871ca9e119c389470c9a9232f94`.
It retains the standalone crate, incremental Git bundle based on
`88707e16b99f54976cfbcbe78c0f49379ab69da7`, complete frozen inputs, all 40
attempted cells and intermediate files, all historical/final gate logs,
source-bound binaries, independent exports/mutations and the separate
producer-verifier reports. It also contains the restored prior root/policy
fixture packet and the corrected consumer's compatibility replay.

Extract into a fresh directory and verify from its `payload/` root:
`cd payload && sha256sum -c SHA256SUMS`.
The preserved standalone crate builds independently of the root crypto crate.
To recheck the saved generic panel without generating data, set
`ML_GENERIC_FILTER_PANEL` to `payload/validation/panel`, set
`ML_GENERIC_FILTER_REPORT_DIR` to a new absolute output directory, and invoke
`payload/validation/resumed-final/generic-filter-interop-tests-source-bound`
with `--ignored --exact sealed_generic_filter_panel_and_rehashed_tampering`.
That test also passed on the freshly restored archive: all 40 records and ten
rehashed tamper rejections, with an identical aggregate acceptance summary.
The restored Git bundle passed verification against the local parent history.
[restored-replay.json](restored-replay.json) records these checks.

This checkpoint proves controller archive durability for these bounded
software artifacts. It does not establish EKS worker publication permission,
large-corpus completion, a mathematical fall-degree bound or any additional
generation authority. Later delivery notes and the independent correction
source review are separate metadata additions with unchanged native files.
