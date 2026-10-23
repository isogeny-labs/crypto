# Full-tree curve registry rebuild diverges

- **Status:** open
- **Kind:** generated-source scope and provenance gap; unresolved EC1 representation
- **Affected work:** [`docs/curves/registry.json`](../docs/curves/registry.json), [`build_curve_registry.py`](../scripts/build_curve_registry.py), the `names` CI check, curve-linked dashboards, and [PR #2304](https://github.com/aburan28/crypto/pull/2304)
- **Next action owner:** curve-registry maintainer and source reviewers
- **Last updated:** 2026-10-23

## Problem

A full, clean checkout of crypto PR #2304 at
`cc4185f3da586e28a8e83982699910777817bcbc` cannot reproduce the
committed curve registry. The committed file has 2,771 curve entries; the
same builder run against all 130,239 tracked files writes 15,990 and exits 1
because a pre-existing n73 EC1 claim has no matching representation. The
13,219-entry difference has not been reviewed, so a wholesale generated-file
replacement is not an accepted correction. This blocks a claim that all
curve identifiers and the dashboards that join on them are complete.

The narrower `names` CI failure has a concrete source: three prime-curve
slugs in the frozen volcano validator report are absent from the committed
registry. Their original slug maps exist and passed an independent finite
point/label check, but that does not resolve the full-tree harvest discrepancy
or independently count each curve's full group order.

## Evidence and uncertainty

- **Observed:** `python scripts/build_curve_registry.py --check` on the full
  PR-head checkout exits 1 and names both `docs/curves/registry.json` and
  `src/cryptanalysis/curve_aliases.json` as stale. Its complete
  [check log](../docs/curves/audit-20261023/full-registry-check.log) has
  SHA-256 `4154da67fe449acb540da1f7afae83a9fce508015770df54eaa5f91146dbe133`.
- **Observed:** A separate `python scripts/build_curve_registry.py` in that
  isolated checkout wrote 15,990 entries and aliases, then exited 1 on
  `research/ic_single_target_20260930/curve_ids.json`:
  `EC1N73Ce0h243f847bccdd (7b45cede73d3) is not a representation in the registry`.
  Its [build log](../docs/curves/audit-20261023/full-registry-build.log)
  has SHA-256 `efab579bef3317fbdee74513404cb56b86e5174cb9acf18587624afa8db7c270`.
  The generated scratch registry SHA-256 is
  `1b738da25380b9fa4940e5dd2cf7b3e64fe2721efcb42d29a12f2e5ffb96725c`;
  it was **not** copied over the committed file. The committed PR-head file
  SHA-256 is `e7bc24a974598923c4c7abd7153998eff8251bd78ca6d7682d54b062e5e0136c`.
- **Observed:** The n73 source record is present unchanged on base `main`
  `d34b9b3908df5d3feafe6189f83b2ce3244d63c3` at line 349 of the cited
  file. Its failure is not established as a new #2304 defect.
- **Observed:** The original
  [`V3-ctl-L0-c0` slug map](../research/volcano_ecdlp_census_20261017/runs/slugmap/V3-ctl-L0-c0.json)
  (SHA-256 `16f77524079584e040d6f4d64ca5234728ed0ecafbbec251e24fb6a593546ebb`)
  and [`V3-D-L0-c0` slug map](../research/volcano_ecdlp_census_20261017/runs/slugmap/V3-D-L0-c0.json)
  (SHA-256 `5230ffc1a2899a87ebab87a949e0a7b46ca24bc18608163fc57ac5ff82347c51`)
  bind the three `icv1-fp26-tm6998-*` slugs to exact `p,a,b`, subgroup
  order, and base points. The [independent checker](../docs/curves/audit-20261023/check_volcano_slugs.py)
  passed under `python -O`: both `p` and `r` are prime, the recorded group
  order is `16r`, each base point lies on its stated curve, `[r]G=O`, and
  each model digest gives its archived slug. A one-field source mutation was
  rejected by the pinned hash; see [positive](../docs/curves/audit-20261023/source-check.log)
  and [negative](../docs/curves/audit-20261023/negative-control.log) output.
- **Derived:** The full-tree builder recognizes those three models, while the
  committed registry lacks them. The resulting `names` failure on #2304,
  #2357 and #2391 is reproducible from the current base report and registry.
- **Suspected explanation:** Sparse-worktree generation may have omitted
  tracked source files, and the broad JSON harvester may admit unrelated
  fixtures into the 15,990-entry output. Both need record-level review;
  neither explanation is established by the count alone.
- **Missing capability:** An agreed authoritative input scope for the
  generator, a source-backed resolution of the n73 representation, and a
  full-checkout generated artefact that passes its own freshness check.
- **What this evidence does not establish:** It does not show that the 2,771
  registered models are false, that all 13,219 additional generated models
  are valid, or that a performance result is wrong. The finite subgroup check
  does not independently certify the three full group orders.

## What would unblock it

- [ ] Review and pin the registry builder's accepted source classes; classify
  the 13,219-entry delta against original curve records and fixtures without
  silently admitting or discarding a class.
- [ ] Resolve `EC1N73Ce0h243f847bccdd` from a matching source record or a
  truthful source correction, preserving the original claim. Then regenerate
  registry and aliases in a full clean checkout with exit code 0.
- [ ] Require `build_curve_registry.py --check`, `check_curve_names.py`, and
  the curve-traits freshness check to pass on the same exact head; verify the
  three volcano models are present with source links and reviewed subgroup
  representations.

Those conditions would establish source/identity consistency of the registry.
They would not independently replay every experiment or prove its performance.

## Next discriminating step

- **Action:** Diff the builder's full-tree input inventory against the
  committed registry's source list, starting with the three volcano slug maps
  and the n73 mismatch. Make a reviewed source-class rule before accepting a
  regenerated registry.
- **Inputs and controls:** The two pinned full-tree logs, original slug-map
  hashes, current registry and aliases, and a clean checkout of the named PR
  head. Run the finite checker with `python -O` and a mutated-source negative
  control. Keep the old generated files intact while reviewing the delta.
- **Execution limit:** No new scientific run is required for this triage;
  stop a proposed record correction when its original curve identity is
  unavailable rather than creating a placeholder.
- **Outcomes:** A clean, reviewed delta permits regeneration; an incorrect
  source class calls for a builder fix; an unresolved n73 record keeps the
  freshness gate red; an infrastructure failure is recorded separately.
- **Recheck trigger:** Any change to the builder's source scope, source
  records, registry, aliases, or n73 record.

## Resolution and history

- 2026-10-23: Full-tree replay and source-bound volcano-slug check completed
  as above. No generated 15,990-curve replacement was committed. Open.
