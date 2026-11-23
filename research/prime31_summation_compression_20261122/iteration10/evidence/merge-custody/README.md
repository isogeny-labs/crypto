# External merge custody: PRs #4928 and #4962

This checkpoint records the two externally merged research PRs and the remaining autonomy-ledger gap. The mathematical goal remains open. No benchmark, native correctness pass, signed validation, formal proof or asymptotic speedup is established here.

## Canonical lesson outcomes

| PR | Outcome | Result |
|---|---|---|
| #4928 | `O-cb8790c26032` | `partial`: research artifacts published; both native builds failed before candidate tests or identity issuance |
| #4962 | `O-494cc8c43d40` | `blocked`: all five independent command stages failed compilation in unchanged binary HEC source |

The maintained `tools/lessons.py` generated both entries and validated them with zero errors. Validation covers these two additions only. The source tool identities and per-entry hashes are in `receipt.json`.

## Autonomy registration remains incomplete

The maintained autonomy tool requires valid first-prompt/episode and human-message timestamps. This task has PR opening and merge times, but no authoritative episode boundaries or message times. Those values are not interchangeable. The tool also represents incident recovery only as `SELF` or `HUMAN`; the three retained native-build failures had not recovered before the respective external merges. No canonical episodes or interventions were fabricated.

`intervention-drafts.json` contains only hashes, provisional classifications and missing-value markers for two visible short continuation messages. It contains no message text, claims no separate classifier run and does not claim to be an exhaustive intervention ledger. A mixed instruction/environment context block lacks an authoritative complete human-message boundary and is explicitly left unclassified and unhashed.

The PR metadata confirms the merges. The scientific native failures and unchanged-source finding come from retained evidence supplied by the parent task; this child did not rerun them. Complete merge-check sets were not reconstructed, and no fourteen-day revert check is claimed.

## Integration

All files in this custody bundle are new paths. Add the two canonical lesson entries and this evidence directory onto the current checkpoint while preserving the existing tree. Confirm each new path is absent before integration. No existing autonomy entry, lesson entry, shared report or PCC was changed, and no remote write was performed by this child.

`manifest.json` lists the bundle files and their content identities. It excludes itself to avoid a self-referential digest. The maintained tools were fetched at the pinned base and are recorded by identity; duplicate tool source is not part of the published bundle. Full Git-prefix verification was unavailable in the partial materialization and is not claimed.
