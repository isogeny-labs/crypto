# Amendment 1: raw pairs and coefficient-row collisions

The first six-curve invocation stopped on `b11_c0` before writing a result.
It preserved `raw/b11_c0.json`. The initial screen asserted that every
unordered signed-point pair produced a distinct coefficient row. At the
identity point, every `P + (-P)` pair has the empty coefficient row, so this
assertion is false even when group arithmetic is correct. The failure was a
screen-ledger error, not a map failure or relation result.

Before rerunning the full six-curve panel, amend the output to retain both
raw pair multiplicity and distinct coefficient-row count. Reconcile the sum
of raw multiplicities with `N(N+1)/2` signed pairs, and report identity raw
pairs and distinct rows explicitly. Define multi-row support using distinct
coefficient rows. Keep the frozen roster, equal-size controls, target points,
source and extractor pins, and other metrics unchanged. The initial failed
invocation remains part of the audit trail.
