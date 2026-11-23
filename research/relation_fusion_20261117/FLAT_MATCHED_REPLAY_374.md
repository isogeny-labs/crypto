# Replay the flat matched draft qualification

Packet373 contains both complete3600-cell synthetic cohorts, every raw proof/metadata file, source/tool contexts, the initial364failure and exact before365sources, frozen366executables, reports and byte deltas for the two copied-data canaries. It contains zero actual performance observations. Report372 explains scope. [pcc:unverified]

Check the SHA sidecar and extract into a fresh directory. Verify every index.json entry by exact path/size/SHA-256 and require the exact payload roster; index.json itself is the sole unindexed file. The deterministic GNU tar preserves internal hardlinks, regular files and normalized timestamps/ownership. Hardlinks share immutable bytes only; no recorded file is rewritten in place.

Bind every source/tool pin in study/FLAT_MATCHED_SYNTHETIC_SOURCE_366.json to source/ and bin/. Frozen old253source copies are also present and remain byte-identical to their original pins. These are correctness/synthetic tool pins, not prospective performance pins.

Replay both complete analyses with a fresh output directory per cohort:

```sh
taskset -c 0 bin/flat-supervised-analyze neutral-367 \
  neutral-367/monitor.jsonl neutral-367/prospective-pins.json /tmp/flat-neutral-replay
taskset -c 0 bin/flat-supervised-analyze hypothetical20-368 \
  hypothetical20-368/monitor.jsonl hypothetical20-368/prospective-pins.json \
  /tmp/flat-positive-replay
cmp /tmp/flat-neutral-replay/analysis.json study/FLAT_MATCHED_SYNTHETIC_NEUTRAL_367.json
cmp /tmp/flat-positive-replay/analysis.json study/FLAT_MATCHED_SYNTHETIC_POSITIVE_368.json
```

Each invocation must replay all3600complete proofs with exact source/hash/basis/stat/phase/roster bindings. Neutral threshold arithmetic must be false; invented20× arithmetic must be true. Both outputs must declare synthetic input and primary_15x_admitted=false. Synthetic monitor flags are invented, not host observations. [pcc:unverified]

Reconstruct each corruption canary from a new copy/hardlink view of neutral-367 and apply ONLY the files listed in canary-deltas/<mode>/delta-roster.json. Replace copied paths atomically; writing into a shared inode would corrupt the source and invalidate the check. The analyzer must exit1 for clock (complete phase/boundary binding) and duplicate (missing/duplicate cell), without an analysis report. Keep the original complete source byte map unchanged. [pcc:unverified]

For source rebuilding, use source/research/relation_fusion_20261117/Cargo.toml with its Cargo.lock and the captured local dependency sources, or a trusted checkout of published parent4e6364822b plus the four new files. Build only flat-supervised-compare/analyze/fixture, one CPU/build/test/numerical thread. Run their unchanged declared tests; expected9+3+3passes. Frozen original253files are comparison context, never replacements to edit. [pcc:unverified]

The fixture CLI is:

```text
flat-supervised-fixture RAW240 PINS200 FLAT281 NEW_OUT neutral|hypothetical20
```

Original qualified sources240/281 and their earlier archives remain the provenance. To regenerate equivalent synthetic cohorts without those live directories, derive source views from saved AB-round0 cells: old controls use pPRIME-nN-ARM directories, flat arms map sparse/dag and103/231/487 to sparse/dag-frontier103/products231/batch487-launcher as in the captured fixture source. These are clearly synthetic source views, not reconstructed real worker executions; the complete proof/basis/stat bytes are retained and verified before every reuse. All synthetic phase clocks are overwritten by the fixture recipe. No worker/launcher or performance clock is executed. [pcc:unverified]

Do not run a real comparison using this preparatory collector. First read/reconcile the canonical protocol/export/schema, complete its positive/negative controls and publish final prospective source/tool/input pins for a separate executable. Full agreed boundary,36strongest eligible controls/equivalent transport, all six15× thresholds, repeats/A/A, every failure and complete proofs stay required. No fresh/reserved seeds, weaker baseline or hidden setup. LocalCPU0/one thread/zero cloud and persistent checkpoint/S3 waiver remain.
