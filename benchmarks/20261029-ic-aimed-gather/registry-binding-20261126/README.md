# Prospective registry custody, 2026-11-26

The inherited validation report's registry pin became stale as the catalog
grew. This successor preserves all of the original report's scientific JSON
values exactly and changes one raw binding to the current registry. It adds
file-custody metadata; no benchmark or scalar replay was executed.
[pcc:unverified]

The unique target's complete record **differs** from the original: its top-level
`sources` and its single representation's `sources` list changed. The receipt
records both lists before and after. Every other target field is equal under
type-sensitive exact-decimal JSON comparison. This includes its curve, field,
generator, order, cofactor, and identity. A duplicate target, a non-string source,
or any change beyond those two provenance fields is rejected. All 211 other
raw subjects were present and their bytes match their original pins.
[pcc:unverified]

`refresh_custody.py` obtains the original registry from the pinned Git commit,
checks the original registry/report SHA-256 values, and checks all raw pins.
It constructs the successor directly from the original report's text, preserving
inherited numeric lexemes and booleans. The independent exact-decimal checker
compares all original fields after restoring the original registry pin and
removing the new receipt. The seven fidelity controls include observed rounding,
sub-rounding decimal drift, boolean/type drift, and raw-pin tampering; six
custody controls check the narrow provenance exception. Neither validation
tolerances nor the validation gate or freeze files change.
[pcc:unverified]

Reproduce from the repository root (hydrate the original report's raw subjects
first using the repository artifact reader if absent):

```sh
python3 benchmarks/20261029-ic-aimed-gather/registry-binding-20261126/refresh_custody.py
python3 benchmarks/20261029-ic-aimed-gather/registry-binding-20261126/test_numeric_fidelity.py
python3 benchmarks/20261029-ic-aimed-gather/registry-binding-20261126/test_custody.py
```

The first command refuses to overwrite differing evidence. Its optional
`--publish-pointer` updates only the live manifest's validator-report reference;
`previous-manifest.json` retains the predecessor manifest. Earlier report
directories remain unchanged. The receipt is specific to the hashed current
registry: a later catalog change requires a new prospective custody review.

Sources: the original `../validator-report.json`, pinned historical registry,
`../registry-binding-20261123/` exact-decimal checker and controls, and the
unchanged `scripts/validation_gate.py` raw-subject rules. This is a CI custody
repair accompanying the P-192 map-verification stack, with zero new PCC claims.
