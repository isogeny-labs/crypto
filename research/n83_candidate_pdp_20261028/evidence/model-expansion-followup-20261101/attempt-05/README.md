# N83 full-domain fixed-base model run

This directory preserves the seven fixed factor-base input panels, top-form certificates and replay checks, and the grouped model outputs. Large files are in the content-addressed artifact store: 196 `.sing` systems and 140 certificate JSON files (9,968,332,030 bytes); smaller `.sing` files and metadata remain tracked directly in Git. Tracked `.artifacts.json` manifests and adjacent `.gitignore` files provide hashes and hydration mapping. Remote verification checked all 336 stored entries with zero problems. From the repository root, restore with:

```sh
python3 tools/artifacts/artifacts.py pull research/n83_candidate_pdp_20261028/evidence/model-expansion-followup-20261101/attempt-05
```

## Layout

- `base-sources/`: source index and per-base manifests for spike27–spike33.
- `inputs/spikeNN/`: 48 generated inputs for each base; `inputs.json` records sampled factor-base points, selectors, and source shard hashes. `systems.json` records the system map; individual `.sing` systems hydrate from the artifact store.
- `generation-summary.json` and `generate-spikeNN.log`: generator version, checks, and per-base run records.
- `top/spikeNN/certificates/`: 48 top-form interval/positive-dimensional certificates per base. `replay-summary.json` and `replay.log` record independent replay.
- `top/top-summary.json`: aggregate count and all-replayed gate.
- `model/rows.json`, `predictions.json`, and `result.json`: row-level model inputs, grouped out-of-base predictions, and summary metrics.

The frozen protocol is v3 (`280a78bd11e6fc20ae274c47525070a2cb6e455401593774b1688dbdd9537e94`). All 336 top-form certificates replayed. The model found identical status and interval bounds for all 48 family/m/target configurations across the seven bases; see the sibling report directory `../reports/model-full-domain-20261101/` for results, scope, grouped-error graph, and inspected PDF.
