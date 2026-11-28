# Canonical field for binary Gröbner and Semaev benchmarks

New binary Gröbner/Semaev benchmark runs use exactly the profile in
[`groebner-field-profile.json`](groebner-field-profile.json):

- `GF(2^17)` in the polynomial basis `GF(2)[z]/(z^17 + z^3 + 1)`;
- field UID `f2m-17-237ceae1`;
- the standard-panel curve `K1-17`, ICV1
  `icv1-f2m17-tm101-00378d4e`, when an elliptic curve is required.

The profile ID is `semaev-groebner-f2m17-v1`. `ecbench plan`, `ecbench run`
and Iso Lab job construction refuse a new binary `ic.pipeline` run using a
Gröbner solver or algebraic Semaev oracle unless the spec names this ID and
every workload has the exact degree, modulus and curve identity. The native
benchmark harness also constructs every case from this profile; individual
cases no longer carry their own field degree.

This is an evidence-comparability rule, not a restriction on the generic
algebra library. Small-field correctness tests, frozen historical corpora and
explicit scaling studies can still exercise other fields, but they are not
admitted into the canonical benchmark series. Historical evidence is never
rewritten: a legacy spec can still be replayed and verified, while it cannot
be launched as a new canonical run without adopting the profile.

## Why this profile

`K1-17` is already on the frozen standard panel, has exhaustive decomposition
ground truth, and is the single cell selected by the earlier Gröbner solver
shootout after its pre-data review. Both two- and three-summand Semaev systems
fit the same field and curve. The previous FL diagnostic used `GF(2^17)` for
its `m=2` cell and `GF(2^15)` for its `m=3` cell; those two frozen rows remain
valid separate diagnostics, but they are not one field-matched series.

## Sources checked

- [`STANDARD_PANEL.md`](STANDARD_PANEL.md) and
  [`standard_panel.json`](standard_panel.json): frozen `K1-17` identity and
  exhaustive role.
- [`../curves/ICV1.md`](../curves/ICV1.md) and
  [`../curves/registry.json`](../curves/registry.json): exact field modulus,
  field UID and curve identity.
- [`../../research/groebner_solver_shootout_20261116/AMENDMENT1.md`](../../research/groebner_solver_shootout_20261116/AMENDMENT1.md): the reviewed
  `K1-17`, `m=3`, dimension-five Semaev cell.
- Semaev's summation-polynomial construction and the Faugère--Lachartre F4
  split cited by the implementation modules. They require a fully specified
  finite-field representation for identical inputs; equality of extension
  degree alone is not an input identity.

The shared research-KB service and repository wrapper were unavailable in the
implementing runtime; four recorded queries returned `no executable found`.
