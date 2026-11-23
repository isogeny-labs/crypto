# Causal group projection review against d6f3865

Reviewed original external patch SHA256 `ae30df34d3440dd4402c390c1826cb731a791acec46bd4c0ac201c59e0a304d9` against committed source `d6f3865a9301cc852e8d32f2693f62e18b1a2b80`. Original and adjusted patches both pass `git apply --check` against that worktree. No patch applied, no worktree edits, builds, Rust tests, CLI or cloud actions performed.

## Integration adjustment

Found a remaining read/swap race: `verify_live_committed_prefix_visit` authenticates against its own head reread but returns only payload receipts. The projector staged signed events without binding the bytes it actually visited to the initially saved head. A different internally valid prefix could be present during the visitor and original files restored before final manifest/head verification. Public original byte caches already have explicit descriptor hashes; the stream needed an equivalent snapshot anchor.

Prepared replacement full patch `koblitz-causal-group-projection-adjusted.patch`, SHA256 `d5588e14775109f8d75cef0b2054cb6f718450d2404f53e3d5f67165563b5292`. It retains the original draft and adds:

- A cursor initialized from the admitted original binding, updated with the exact visited raw line length and SHA256, including newline.
- Checked cumulative byte/event arithmetic and incremental64MiB projection cap.
- Equality of observed event count, byte count and last-event digest to the initially saved committed head, with explicit presence/absence agreement, before any row emission.
- Private deterministic before/after-visitor hooks. Production passes no-ops; the real original-bound fixture swaps to a valid shorter committed prefix during the visitor, proves that alternate prefix independently validates, then restores original stream/head and proves the manifest validates. The projector must reject the staged observations and emit zero rows. The test removes only the final solver_return event, keeping payload custody valid.

External adjusted Rust files are in `koblitz-causal-watermark-adjustment/`. Rustfmt parsing through stdout passed without modifying files. The added controls are uncompiled/unexecuted. This adjustment closes the snapshot mismatch without changing capture.rs or producer accounting.

## Compatibility and leakage audit

Current native projection receipts eliminate the duplicate generic native-events payload, not the signed-domain event stream. The group projector selects strict `koblitz_m3` / `signed-domain` records and deliberately ignores `koblitz_m3_native`; its group feature schema therefore needs no native-receipt format migration. Central `verify_worker_cell` now supplies the updated parent/native projection checks before staging. Original public, factor-base, binding and limits parse buffers remain hash/length-bound to unique manifest descriptors.

All source authentication, final manifest checks, initial-watermark equality and full bounded group arithmetic replay precede the first exporter callback. No positive witness or completed outcome is required for group-row admission. Features are constructed field-by-field from authenticated public curve/domain/target, deterministic tuple/summand query and previous independently replayed accumulator. Current sum, infinity result and target-match flag stay in the restricted label. Capture sequence, original binding/head, witness, terminal result, solver identity and completion do not enter public features. Witness publication interleaving does not advance feature state. Existing tests cover prefix/suffix invariance, shifted provenance sequences, inverse-pair infinity, positive target labels, unrelated future outcome fields and forged late state causing zero emissions.

Summary provenance remains restricted. The exporter must never merge it or record-level source metadata into model inputs. Current API reconstructs static inputs internally and intentionally does NOT accept the older design's arbitrary `public_features` parameter. This is the correct integration signature.

## Remaining implementation scope

This implements only independent signed-group transition features/labels. Matrix candidate observations, original equation/support features, construction projection and native basis derivation are not implemented; their summary statuses remain explicit. Do not infer them from the canonical native receipt or terminal rows.

The original design's graceful bounded-suffix export is also not implemented: above stream/record limits this draft rejects with zero rows rather than emitting a capped partial suffix. This is a conservative admission limit, and must be stated in exporter policy. The192 group-addition count covers only `verify_signed_prefix` in the projection pass; earlier central replay, reconstruction, parsing/hashing and I/O are excluded explicitly.

The foreign exporter integration is still outstanding: invoke the native seam in its verified-source branch, use existing scratch/atomic publication behavior for callback failure, content-deduplicate feature bytes separately from restricted source occurrences, retain existing field-size split grouping, version the native feature policy and include restricted summaries/counters. No new cloud generation is needed to test this integration on existing smoke fixtures. End-to-end CLI/export controls and a real authenticated suffix-extension comparison remain required after the native draft compiles.

## Integration

Native projector and controls applied after terminal d6f3865 validation, under Conductor T1148 for the child module and existing T1024/T887 wrappers. No producer dispatch or exporter activation changed. Historical T828 export reservation has since expired; fresh coordinated reservation and feature-policy validation are still required before exporter integration. Rust controls remain pending.
