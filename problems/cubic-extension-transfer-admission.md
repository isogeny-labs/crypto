# Cubic-extension transfer evidence admission

- **Status:** open
- **Kind:** access/tool limits, implementation gaps, proof and validation gaps
- **Affected work:** [frozen study](../research/cubic_extension_transfer_20261025/README.md)
- **Next action owner:** repository maintainer or a session with the named tools and storage access
- **Last updated:** 2026-10-25

## Problem

The completed Sage prototype cannot yet be admitted as a native, reproducible
performance result because native integration, matched
accounting, formalization, and the repository's release/replay gates are open.
The separate universal one-extra-isogeny coverage conjecture and universal
exceptional-chart handling also remain unresolved. A PR importing evidence
does not settle those research questions.

## Evidence and uncertainty

- **Observed:** retained exact order certificates at p=157; ten complete
  source-verified prototype transfers; seven exceptional-input checks;
  same-endomorphism-order coverage differences at p=109.
- **Recorded finite conclusion:** some gated sources need two total
  degree-three steps in this transverse construction. This does not refute
  one extra step after torsion preparation.
- **Access/publication:** the supplied lab-account AWS identity now passes the
  S3 checkpoint gate, and all five original artifacts are uploaded unchanged
  with fresh-cache size/hash verification. The direct AWS lab bridge still
  denies its required `codebuild:BatchGetBuilds` action; no worker was submitted.
- **Tool status:** Rust 1.99.1 is installed and prior standalone native tests
  pass. The old partial import lacked Cargo.toml; the current continuation uses
  a proper sparse clone. Root release compilation failed under 4-GiB and 12-GiB
  virtual ceilings before tests ran; all attempts are retained in the
  [validation record](../research/cubic_extension_transfer_20261025/native_end_to_end/access/20261028-local-validation.json).
  Conductor and Lean/Lake remain unavailable.
  CLI access is available; the earlier authentication warning was caused by
  sandbox network denial. No root release-test pass is claimed.
- **Missing implementation:** native forward construction and complete inverse
  map. Native obstruction evaluation, source controls, target-stage adapters
  and projective divisor recovery are now implemented, but do not constitute
  a complete elliptic-to-Jacobian transfer.
- **Missing validation:** isolated matched operation-cost runs, the strongest
  applicable rho fold, external solver comparison, cross-host replay, and
  faithful Lean bridges for the structural statements.
- **Not established:** a new exponent, universal coverage, fastest-tool
  status, an admitted index-calculus improvement, or a cryptographic-sized DLP.

## What would unblock it

- [x] Upload the five original files unchanged, verify remote readback against
  PUBLICATION.json and hydrate all five. See the
  [receipt](../research/cubic_extension_transfer_20261025/ARTIFACT_PUBLICATION_RECEIPT.json).
- [ ] Replay every referenced input in the native execution path; byte
  verification alone does not validate the algorithms or complete the map.
- [ ] Preserve all mathematical contracts and fixtures in a native
  implementation; pass source/target equality and exceptional-case controls.
- [ ] Run the required release and touched tests at the PR head and resolve
  all required CI and review gates before merging.
- [ ] Freeze one-target matched inputs, complete operation accounting,
  isolation, strongest rho and external references before any new timing.
- [ ] Attach independent replay and validator artifacts under the repository
  validation schema before promoting any benchmark or leaderboard entry.
- [ ] For each structural theorem, either build a faithful pinned Lean proof
  or retain its exact remaining bridge as partial formalization.

## Next discriminating step

The exact frozen files are now published and hydrated. Next perform a
native exact-fixture replay of the p=157 source, both three-isogenies and
the target subgroup. Matching all recorded answers validates that fixture;
a mismatch blocks further timing until corrected. Only after fixture parity
should a separately frozen, isolated native comparison run. Neither outcome
alone resolves the universal extra-step conjecture.

For the conjecture, a finite all-negative center plus all eligible neighbors
must have every target order independently certified before it is reported
as a counterexample. Otherwise an actual structural proof is needed; more
bounded negative searches are not a proof.


## Native follow-up evidence

See [the follow-up](../research/cubic_extension_transfer_20261025/native_followup/README.md)
and its source-linked report. All historical source/evidence bytes are preserved.

- Exact polynomial calculations give a rational three-torsion translation
  matrix with order three and a factored discriminant; the existing Rust
  checker replays six C++-issued identity records with extra points and
  false-identity controls. No Lean geometric proof is claimed.
- All 232 nonzero torsion actions across 29 saved fixtures match the prior
  construction. The native evaluator reproduces 27 prepared coverage counts.
- A geometric argument bounds bad representative classes by 5*p^2+7*p+6
  under the stated N1/N2 and prime-order hypotheses. It gives common-multiplier
  availability for p>=19, but not totality of the current forward formulas
  or inverter. Native recovery checks all 2,492 rational effective degree-three
  divisors on the three declared test curves; it starts from support coordinates.
- Fifty native solver/control runs plus eight A/A repeats all recover correct
  answers. A separate FLINT implementation checks all source scalar relations.
  Target points are synthetic with the same known answer; no native forward
  map was executed. No qualified wall times: PSI is absent and system threads
  could not all be moved off the selected CPU. Strict preflight refusal and
  limited diagnostic receipts are retained.

The next mathematical exit criterion is a certified character relation for
labelled neighboring obstruction functions, or a source/target-order-certified
counterexample. The exact remaining calculation is level-nine label transport
and modular divisors modulo two, including arithmetic constants. The rational
three-torsion action removes one interpolation dependency; it does not settle
that relation. The next implementation exit criterion is a native forward map
and complete generic inversion on the geometrically guaranteed domain, with
all additional poles counted. The next measurement exit criterion is a host
with PSI/full reservation, then a frozen cold one-target comparison including
every transfer phase, strongest references, pricing, external comparison and
independent replay. No diagnostic substitutes for these requirements.

## Resolution and history

The 2026-10-26 benchmark continuation restored compute access and prepared
[explicit cold-transfer acceptance requirements](../research/cubic_extension_transfer_20261025/native_end_to_end/REQUIREMENTS.md).
The [observed access record](../research/cubic_extension_transfer_20261025/native_end_to_end/ACCESS_STATUS.md)
retains the failed mandatory checkpoint gate, absent runner connection and
missing native-map stages. No new measurement or release-test pass is claimed.

Unresolved. The 2026-10-25 PR preserves existing evidence and names the
publication and admission gaps. No requested research criterion is weakened
or replaced by this problem record.
