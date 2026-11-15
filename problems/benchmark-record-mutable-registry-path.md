# Historical benchmark raw path now names a newer registry

- **Status:** investigating
- **Kind:** historical evidence path reuse and validation semantics
- **Affected work:** [closed record](../benchmarks/records/20261105T1826Z-ic-aimed-gather-registry-binding-20261105.json), [benchmark utility](../tools/benchrec.py), shared artifact reader maintenance, Conductor T-6031
- **Next action owner:** T-6031 authoring session and benchmark-record maintainer
- **Last updated:** 2026-11-15

## Problem

The benchmark gate compares a closed record's raw registry digest against a newer file at the same path. Restoring the old canonical registry or editing the closed record would destroy a different contract; neither is a correction.

## Evidence and uncertainty

At main `719ece9f04e68f3c0fff199ff103675aa16922dc`, the closed record declares raw `docs/curves/registry.json` with SHA-256 `1edbd0d717644b047931aba9126431f98862ee96791f4fed3884c40f971c2bc8`. Current registry bytes hash to `33a16da0548b7b0c3482b3ba7198500a0285271794f63e9c46e10f9a02186e4f`. Reading the path at the record's source commit `19b5e74b9af6934ccc0d37f192019337682ed11d` through Git filters reproduces the declared digest exactly.

This establishes availability and byte identity of one archived input. It does not validate the benchmark's execution, costs, other inputs or conclusions. The gate remains failing for this mismatch. The separate open record `20261115T0609Z-genus-ladder-sge-scale` is ongoing work and is preserved; this problem does not close it.

Initial sparse checks also reported missing raw/report/figure paths. Materializing the tracked dependencies and artifact manifests removed those setup findings; the remaining mismatch is present in the committed baseline, independently of the artifact reader patch. This differs from the [full-tree registry rebuild gap](curve-registry-full-tree-rebuild-diverges.md): this record needs its original bytes, regardless of the latest registry's completeness.

## What would unblock it

- [x] Check the declared content-addressed raw object against the archived Git bytes and digest; the [readback receipt](evidence/benchmark-registry-raw-readback-20261115.json) records 22,383,109 matching bytes.
- [ ] Provide an additive, reviewed archival-source binding and a verifier path that checks those exact bytes while continuing to reject changed, missing or unrelated inputs.
- [ ] Demonstrate that arbitrary current-file tampering is still rejected; preserve the original closed record and current canonical registry.

These conditions would resolve the raw-input lookup only. They would not confer a research verdict or close other records. No new exclusion, widened baseline or weaker comparison is proposed.

## Next discriminating step

The declared raw object has been read back and matches. Design the explicit historical binding with positive and substituted-input controls; keep current-file mismatch rejection until that contract is reviewed. Stop if any binding or byte check fails. No scientific campaign is launched by this maintenance task.

## Resolution and history

2026-11-15: reproduced during shared artifact reader maintenance. Archived Git bytes match the old digest. Unresolved; the original record, registry and gate requirements are preserved.

2026-11-15 readback: existing authorized storage access returned the declared object with the same SHA-256 as the archived Git source and closed record. The gate mismatch remains; no record or registry was replaced.
