# Historical benchmark raw path now names a newer registry

- **Status:** resolved
- **Kind:** offline archival-input availability; existing guarded verifier
- **Affected work:** [closed record](../benchmarks/records/20261105T1826Z-ic-aimed-gather-registry-binding-20261105.json), [benchmark utility](../tools/benchrec.py), shared artifact reader maintenance, Conductor T-6031
- **Next action owner:** T-6031 authoring session and benchmark-record maintainer
- **Last updated:** 2026-11-15

## Problem

The offline gate initially lacked the archived bytes needed to check a closed record whose raw path now contains a newer registry. The existing archival verifier resolves this lookup when supplied with the original content-addressed bytes; the current canonical registry and closed record remain intact.

## Evidence and uncertainty

At main `719ece9f04e68f3c0fff199ff103675aa16922dc`, the closed record declares raw `docs/curves/registry.json` with SHA-256 `1edbd0d717644b047931aba9126431f98862ee96791f4fed3884c40f971c2bc8`. Current registry bytes hash to `33a16da0548b7b0c3482b3ba7198500a0285271794f63e9c46e10f9a02186e4f`. Reading the path at the record's source commit `19b5e74b9af6934ccc0d37f192019337682ed11d` through Git filters reproduces the declared digest exactly.

This establishes availability and byte identity of one archived input. It does not validate the benchmark's execution, costs, other inputs or conclusions. The initial offline gate failed on this mismatch before the guarded archive cache was hydrated; the online CI check already verifies the original stored bytes. The separate open record `20261115T0609Z-genus-ladder-sge-scale` is ongoing work and is preserved; this problem does not close it. [pcc:unverified]

Initial sparse checks also reported missing raw/report/figure paths. Materializing the tracked dependencies and artifact manifests removed those setup findings; the remaining mismatch is present in the committed baseline, independently of the artifact reader patch. This differs from the [full-tree registry rebuild gap](curve-registry-full-tree-rebuild-diverges.md): this record needs its original bytes, regardless of the latest registry's completeness.

## What would unblock it

- [x] Check the declared content-addressed raw object against the archived Git bytes and digest; the [readback receipt](evidence/benchmark-registry-raw-readback-20261115.json) records 22,383,109 matching bytes.
- [x] Use the existing `archived_raw_matches` path, which checks the original record's canonical URI, byte length and SHA-256. The existing archive-control tests cover missing, corrupted, truncated, oversized and unavailable originals.
- [x] Replay the three existing offline/online archive controls, including size/hash corruption and refusal to upload substituted current bytes; preserve both the original record and live registry. These three tests pass locally.

These checks resolve the archived-input lookup only. They confer no research verdict and close no other record. No new exclusion, widened baseline or weaker comparison was introduced.

## Next discriminating step

For future offline worktrees, hydrate the guarded `.benchrec/raw-archive-cache/<sha256>` with the original verified object, or use the existing online `check --verify-s3` path. The verifier rejects missing, corrupt, wrong-size, noncanonical or escaped/symlink cache inputs. Inspect the separate ongoing record's completion under its own protocol; this maintenance task does not close it. [pcc:unverified]

## Resolution and history

2026-11-15: reproduced during shared artifact reader maintenance. Archived Git bytes match the old digest. Unresolved; the original record, registry and gate requirements are preserved.

2026-11-15 readback: existing authorized storage access returned the declared object with the same SHA-256 as the archived Git source and closed record. The gate mismatch remains; no record or registry was replaced.

2026-11-15 correction and resolution: CI job132770260983 explicitly verified the original raw bytes from the S3 archive and failed only on the unrelated open record. Source inspection confirmed that `archived_raw_matches` already implements the required guarded path; the initial local diagnostic had overlooked it. The exact readback was supplied to the existing local cache, whose digest and length checks pass. Three existing archive tests passed. The raw-input lookup is resolved without changing verifier semantics, the closed record, the newer registry or any research status. [pcc:unverified]
