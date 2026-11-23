# Metadata correction: exact identity field

The original `DEVELOPMENT_RECEIPT.json` is preserved byte-for-byte because
PCC-20261123-cff73c pins its hash. Its `cli.identity_field` passed through a
JavaScript JSON Number during metadata assembly, rounding the native integer
2305843009213693951 to2305843009213694000. This is a packaging defect in that
one descriptive field.

`DEVELOPMENT_RECEIPT_v2.json` corrects the field to the exact decimal string
`"2305843009213693951"`, copied from the native certificate. It records the
original path and SHA256. The Rust certificate, its identifier, native
arithmetic, source manifest, test outputs and binary hashes are unchanged.
Use v2 for the current report. No experiment or arithmetic test was repeated
for this metadata correction.

Both records are unsigned producer evidence. The correction does not create a
signed verdict or a proof over the integers. [pcc:unverified]
