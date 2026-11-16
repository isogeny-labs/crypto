# Secant equations and the elliptic-curve group law

The secant system is now formally equivalent to Mathlib's actual elliptic-curve
point addition. The proof includes tangents, repeated target abscissae and flexes.
The identity target is characterized separately by inverse pairs, including
two-torsion. Complete splitting supplies exact factor-base membership and every
ordinate lift is covered by the four sign branches.

See [the proof map and mathematical argument](FORMALIZATION.md),
[verification status](verification/metadata.json), and
[the source-linked illustrated report](reports/REPORT.md).

## Requested scope and evidence

| Requirement | Evidence and status |
| --- | --- |
| Actual elliptic-curve addition, both implications | Locally compiled: `SecantGroupLaw.finite_target_iff`, using Mathlib's nonsingular-point group |
| Tangents and repeated abscissae | Locally compiled: `canonical_slope`, `diagonal_tangent`; no division by x-r or z-r |
| Identity target | Locally compiled: `identity_target_iff` and `SplitFactorBase.identity_signed_iff` |
| Exact split factor base and all four branches | Locally compiled: `selector_iff_group_relation`, `all_four_branches_complete` |
| Dependency and axiom inspection | Lean 4.19.0; pinned Mathlib; twelve printed theorem dependencies use only standard foundations |
| Whole-module repository axiom audit | Not run before the worker became unavailable; existing unchanged CI discovers this Lake project |
| One theorem bundling the prior F17 degree fixture and this group bridge | Not implemented; generic bridge and prior canonical degree theorem are separate checked results |
| Illustrated PDF | Report, editable SVG and Typst source supplied; rendering/inspection blocked by unavailable execution environment |

## Reproduce

From this directory, with elan installed:

```sh
lake --no-cache build
```

The checked development run used `LEAN_NUM_THREADS=4`. Its dependency was built
from pinned source after the cache endpoint returned proxy CONNECT 403. An
online installation may use the normal Mathlib cache; the manifest locks every
Git dependency revision.

For the full unchanged repository gate, run from the repository root:

```sh
tools/lean/proof-gate.sh --out /tmp/secant-group-receipts research/prime_factor_base_degree_20261031/group_law_20261031
```

The gate builds all tracked modules and runs `tools/lean/AuditAxioms.lean`.
The existing `lean-proof-gate.yml` runs on matching main-branch pushes and
manual dispatch, not as a required PR check. No workflow, gate or native-trust
manifest is changed here.

## Source custody

The standalone three-module build completed successfully before the native
worker disconnected. Sources in this continuation were recovered from the
recorded file edits; the unchanged `SecantIdentity.lean` and locked dependency
revisions were recovered from merged #3082. The manifest root package name is
changed and its JSON is reformatted. The hashes supplied here identify these
published files; an original pre-outage whole-source checksum was not captured.

The retained [build-output excerpt](verification/build03-output-recovered.txt)
is recovered tool output, not a copy of the inaccessible on-disk log.
Development failures and an unbuilt optional dependency-layout change are
listed in [the custody note](verification/CUSTODY.md). A fresh whole-module
audit and published-source replay remain explicit verification follow-ups.

No sealed prior proof or experiment record is modified. No new curve, factor
base, numerical benchmark or leaderboard result is introduced.
