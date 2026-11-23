# Degree limit versus degree-fall closure

This is a separate bounded diagnostic of the existing mixed-component
extension-field zero-relation study. The full 42-arm certificate panel and
85-configuration recovery panel retain their own inputs, sources and owners.

The [prospective protocol](PROTOCOL.md) fixes six systems: indices 0, 1, 2
from p37 seed0 in each original phi/doubling packet. Both `raw16` and
`closure8` run on each, first plus one deterministic repeat: 12 configurations,
24 producers, 12 first audits and 12 parity checks. These are polynomial
systems from one public synthetic fixture, not independent secret samples.

## What this tests

The raw degree-8 certificate method found no certificate on the first
completed matched pair. That observation alone does not identify why.
`raw16` raises the original Macaulay degree bound. `closure8` multiplies
newly derived lower-degree rows while retaining an exact representation
over the four original equations. Its original-product cap is 16, which
must not be confused with its intermediate degree cap of 8.

Every returned certificate receives independent sparse coefficient
expansion and comparison with the sealed original roots and physical-relation
set. Raw degrees 4..8 must reproduce the previous record exactly, including
rank and arithmetic counters. The producer receives equations and peeled
abscissa only; prior outcomes and costs belong to the later auditor.

## Implementation and evidence

- [Native implementation](native/src/probe.rs); the copied
  [algebraic core](native/src/algebra.rs) records its inherited raw method.
- [Control checkpoint](BUILD_AND_CONTROLS.md), its
  [PDF](BUILD_AND_CONTROLS.pdf) and editable [figure](figures/flow.svg).
- Current native gate: `../runs/degree-closure-probe-native-gate-v2/` (five
  tests pass). V1's four-test build and report remain preserved; prelaunch
  review corrected a cap/actual-degree reporting error before any case ran.
- Input preparation: `../runs/degree-closure-probe-inputs-v1/`.
- Completed guarded production directory: `../runs/degree-closure-probe-panel-v1/`.
  Native final status complete, terminal exit 0 at 2026-10-25T07:20:28Z.
- [Managed launch](RUNNING.md): one owner started 2026-10-25T04:08:23Z,
  after source/input freeze `c30a3dc6068b9907681e46634df14c52a8a40f1e`.
  All 24 producers, twelve first audits, twelve parities and final seals
  pass. See the [full result table](REPORT.md), [inspected three-page
  PDF](REPORT.pdf) and [native summary](../results/degree-closure-probe-v1/final/summary.json).
- [Existing rejection proof and IDC](../collision_certificate/BUILD_AND_PROOF.md).
  The native/field/group bridges remain open. No new general theorem is claimed.

The baseline 189+21 study remains complete and is not repeated. Unique
relation yield, factor-log matrix rank, complete cold cost and matched
strongest IC/rho comparisons are the original acceptance criteria. This
diagnostic collects no new relations, performs no target recovery and does
not discharge those criteria. Full price remains null, including memory,
I/O and uncalibrated primitives. Shared-host times are diagnostic only.

The atlas match is partial: `LEAD-ICE-descent-closure-degree-at-scale`
already distinguishes raw Macaulay spans from degree-fall closure in a
different binary-descent setting; `KR-IC-fbdb61` is a related match. No
novelty or atlas-saturation claim follows.

Keep all failed, incomplete and censored outputs. Publish this round in a
separate versioned archive after its production owner becomes terminal;
never overwrite the prior zero-fiber or historical archives.

## Complete versioned archive

`runs.tar.xz` retains all ten degree/closure raw directories: both gates,
both driver-control versions, prepared inputs, both prelaunch layouts,
the complete production tree, the historical partial snapshot and the
inspected final layout. It includes the earlier failures and quiescence.
Its paths are rooted at the parent collector. From that parent:

```sh
(cd degree_closure_probe && sha256sum -c runs.tar.xz.sha256)
tar -xJf degree_closure_probe/runs.tar.xz
```

Use `archive-results.sh FRESH_PATH` from the repository root to reproduce
the deterministic options and exact closed-directory list. The prior
original reference corpus remains a prerequisite. Compact final receipts
and the native table remain loose under `../results/degree-closure-probe-v1/final/`.
The final PDF/SVG/150 dpi PNG have identical unlisted benchmark copies.
Prelaunch graphics and their PDFs remain unchanged. This records counted
construction work for both maps; it makes no complete-cost or novelty claim.
