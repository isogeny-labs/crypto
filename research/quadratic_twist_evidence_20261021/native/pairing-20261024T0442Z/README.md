# Frozen pairing-transfer results

Run `pairing-20261024T0442Z`, under the protocol committed at
`dcd89832fc6169dd71dad048cc59a70f694a66bc`. The 942-model sweep and the
additional F43 fixture are separate populations.

- `results.json`: complete C++17 census, every auxiliary/cofactor/image point,
  direct/shifted value and offset, degeneracy/wrong-subgroup controls, and
  eleven native finite-field IC certificates.
- `audit.json`: independent Rust coefficient-pair/linear-Miller replay, exact
  complete coverage and counts, actual points/scalars, matrix ranks and
  factor-base/target field equations. Producer search-attempt counters are
  retained but are not an independently rerun randomized IC search.
- `certificate/`: source-bound norm-product polynomial IDC1h23c0696a12358b08,
  its 32+16-point receipt and exact false-identity/source-hash controls.

Reproduce with `bash research/quadratic_twist_evidence_20261021/run-pairing.sh /tmp/twist-pairing-fresh`.
That command compares all these JSON bytes and runs negative controls.
The separate repository Lean proof gate checks the actual finite function and
all seven study modules. See the [report](../../reports/pairing-20261024T0442Z/README.md)
and [source/input/output provenance](../../validation/pairing-20261024T0442Z/README.md).
No timing/advantage, generic Weil theorem or general hardness claim is made.
