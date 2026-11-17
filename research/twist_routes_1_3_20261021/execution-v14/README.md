# Target-blind rank repair, with matched endomorphism augmentation

The single native panel and independent GP replay pass strict admission.
All42 base configurations and252 planned matrix cases are retained;29 bases
fill,13 underfill;174 matrices analyzed,78 excluded by the frozen fill rule.
There are114 target-fixed and60 target-ambiguous cases of one public toy DLP.
Known alpha equations newly fix14 cases, including ordinary controls. This is
a correctness/identifiability census, not a speedup or full-cost experiment.

[Report and quantitative graph](../reports/rank-repair-20261028/RESULTS.md)
· [inspected PDF](../reports/rank-repair-20261028/RESULTS.pdf)
· [frozen plan](PROTOCOL.md) · [admission](ADMISSION.json)
· [all configurations, cells and matched effects](SUMMARY.json)
· [unified export](experiment.export.json)
· [conditional formal criterion](../lean-v9/TwistTargetIdentifiability.lean).

Frozen source `d377251611bc9fd3da22096c3b128862816ef268`; protocol SHA256
`e031cee91eef30ca50d2ddef24cd36c7ca6ee739a3f173bd9885005efb7c61bf`.
`FROZEN.json` pins exact sources, binaries and historical v8/v10 inputs.
`HOST.json`, transport receipts and export record actual processes. The native
prefix remains the archived v8 implementation; new research code is Rust, with
independent PARI verification. Old frozen source/data/failures are unchanged.

Reproduce from the frozen revision into fresh output directories, preserving
any failed or capped attempt. The operational driver refuses existing outputs:

```sh
node research/twist_routes_1_3_20261021/execution-v14/run-producer.mjs
```

The recorded GP transport command runs `runs/rank-v1/verify.gp` into the fresh
`runs/pari-v1` directory with CPU0/4GiB/300s. Use the pinned native binary's
`admit RAW PARI FRESH_ADMISSION` mode for strict verification; the frozen
publication binary's `aggregate`/`export` modes produce the committed summary
and export. Fresh paths are required; never overwrite the archived attempts.

Raw evidence is a managed deterministic archive,61888bytes/ten files:
SHA256 `2fd7a36e7a9755ea9687e93d7cc86bec661943dcf6fb63853968432b52d41e13`.
Hydrate it with maintained storage tooling using the user-confirmed bucket:

```sh
python3 tools/artifacts/artifacts.py pull research/twist_routes_1_3_20261021/execution-v14/runs.tar.xz
python3 tools/artifacts/artifacts.py verify research/twist_routes_1_3_20261021/execution-v14/runs.tar.xz
tar -xJf research/twist_routes_1_3_20261021/execution-v14/runs.tar.xz -C research/twist_routes_1_3_20261021/execution-v14
python3 /workspace/crypto-autoresearcher/tools/experiment_protocol.py check-export research/twist_routes_1_3_20261021/execution-v14/experiment.export.json --protocol research/twist_routes_1_3_20261021/execution-v14/protocol.json --verify-files
```

`RAW_MEMBERS.json`, `PACKING.json` and `RAW_VERIFICATION.json` bind the archive
and every raw member. Development GP shape failures and initial Lean import
failure remain in their separate receipts, not pooled as scientific main runs.
Complete charged discovery/collection/solving, held-out larger toys and the
previous full-root validation gap remain separate open requirements.

| Requested next step | Actual evidence/status |
|:--|:--|
| Useful relations beyond map correctness | V14 fixed42/252-cell diagnostic independently verified; all exclusions retained. |
| Matched ordinary controls and known symmetry | Same alpha information in all arms; newly fixed cases include ordinary bases. |
| Complete charged comparison | Not attempted by v14; original cold-cost cards and concrete instrument requirements retained in NEXT_STEPS.md. |
| Larger held-out curves and NIST relevance | Not attempted; final E28 labels remain unopened, no NIST conclusion. |
| Reproducible publication | Frozen native/GP/export, conditional Lean receipts, report/visual/PDF, managed Setec raw archive; PR2380 reviewed landing pending. |
