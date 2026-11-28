# Reproduce the five-worker qualification

Use fresh output paths, CPU0 and one numerical thread. Mathematical checks and statistics run in Rust. Python only edits source, checks metadata/hashes or lays out recorded results.

Verify `SOURCE_V2.sha256` from the repository root. The old `SOURCE.sha256` records the prequalification fixture before its static synthetic-clock correction; use `initial-source/root-supervised-fixture.rs` when examining that historical context. Keep all original manifests and locks. Follow `build-v2.sh` with new raw log paths, preserving toolchain, compiler profile, dependencies and resource caps. Copy the five normal tools and five libtest executables to fresh read-only paths and hash them before running all unit functions once.

`SYNTHETIC_INPUT_PINS.json` binds five frozen workers and all six original input digests. Workers0/1/2 are historical85/96/flat274; worker3 is the previous pre-scan root; worker4 is the new scan root. Preserve their distinct digests. The qualifier takes the new worker and a corpus with `p{p}-n{n}-frontier/original.json` paths:

```sh
root-transport-qualify NEW_SCAN_WORKER REFERENCE_INPUTS NEW_QUALIFICATION_PINS NEW_TRANSPORT_OUT
```

Follow `run.sh` using fresh paths. It calls only `--qualify`, checks null worker clocks, every complete proof/native-basis/receipt binding and four canaries for each of 432 calls. Keep every failure. Qualification pins cannot authorize the actual timing collector.

To recreate fixtures, use the exact prior240 generic/packed pool, prior281 flat pool, previous four-worker no-clock root pool and new scan root pool:

```sh
root-supervised-fixture RAW240 NEW_FIVE_WORKER_INPUT_PINS FLAT281 OLD_ROOT_QUAL SCAN_ROOT_QUAL NEW_OUT neutral
root-supervised-fixture RAW240 NEW_FIVE_WORKER_INPUT_PINS FLAT281 OLD_ROOT_QUAL SCAN_ROOT_QUAL OTHER_NEW_OUT hypothetical20
```

The fixture checks every reused proof before writing it; content-addressed hardlinks only save storage. The analyzer independently checks every full proof again:

```sh
root-supervised-analyze COHORT COHORT/monitor.jsonl COHORT/prospective-pins.json NEW_ANALYSIS
```

Each cohort contains 15,120 synthetic cells. The neutral fastest control is deliberately an earlier root pair-graph arm. The hypothetical20 cohort gives every eligible control an invented 20× ratio over candidates. Neither contains a host timing or observed monitor state. Threshold arithmetic must reject neutral and accept hypothetical20; admission remains false for both. [pcc:unverified]

Follow `run-canaries-v2.sh` with fresh paths. It copies the neutral fixture using hardlinks and atomically replaces only the clock receipt or duplicate summary. Expected analyzer exit is1, at complete phase/boundary binding and missing/duplicate cell respectively; neither produces `analysis.json`. Verify `CANARY_SOURCE_BEFORE.sha256` from the archive extraction root afterward. No original neutral file may change.

For archived replay, verify the SHA sidecars and extract only into an empty directory. Complete original transport, synthetic and sabotage trees are retained. The frozen tools are unsigned local ELF binaries; hashes establish byte identity rather than execution trust. Read custody/source records before running them.

`root-supervised-compare --print-roster` emits metadata only. Do not invoke the actual collector until the pending canonical-document condition is resolved and the performance compiler/source/control campaign is prospectively qualified. No optimized performance build, comparative observation, full-library pass or 15× result is established by these instructions.
