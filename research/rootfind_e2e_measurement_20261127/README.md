# Rootfind production end-to-end timing (2026-11-27)

This is a local diagnostic reproduction of the frozen Prime31 production panel,
not claim-grade isolated evidence. The runner was pinned to CPU 0 on an Intel
Xeon Platinum 8573C, but CPU and memory PSI were unavailable, so `ecbench`
required `--allow-busy` and graded every record L0. The frozen input is the
legacy v1 spec `research/rootfind_production_panel_20261126/p31-production-panel.json`.

## Result

All 35 executions verified. Excluding one warmup per arm, the six measured
`rootfind-floor-batch-v1` solves had a median cold solve time of **100.022 ms**
(range **81.981–169.959 ms**). Median method-call time was **101.867 ms**,
median process time was **111.295 ms**, and median target-dependent online time
was **58.883 ms**.

Median wall-time phase breakdown for the batch arm:

| Phase | Median ms |
|---|---:|
| Factor base | 0.653 |
| Oracle setup | 35.148 |
| Relations | 50.364 |
| Linear algebra | 8.045 |
| Final verification | 0.007 |

The protocol analyzer reports a paired median legacy/batch ratio of **7.736x**
(descriptive bootstrap 95% interval **5.914–11.935x**) and a paired median
subtract/batch ratio of **2.864x** (**2.161–4.344x**). The 50x gate is not met.
The legacy A/A paired median was 1.096x with a symmetric observed upper bound
of 2.651x, so these timings remain descriptive under the unavailable external
admission gates.

## Correctness and custody

- Session: `ECBS1h02da92923676`; 35/35 records verified.
- Full deterministic replay: 30/30 measured records reproduced identically.
- Audit receipt SHA-256: `1a27050ea0a71e6099bfd5a6123f8340aeedafdaf3110df5a5cff3fd1fd12be4`.
- Records SHA-256: `8467aa21d144761c858b0910c609565835bf9f81bbed821f297771a1ac84db23`.
- Binary SHA-256: `bab7623438724c109fab0bb041736e9234388b4ef73f89e2276597ad6fd51785`.
- Rust: 1.99.1, release, `-C target-cpu=native`.

Raw session files, analyzer output, and the audit receipt are retained under
`local-run-busy/`.

## Commands

```sh
target/release/ecbench plan \
  --spec research/rootfind_production_panel_20261126/p31-production-panel.json \
  --json

taskset -c 0 target/release/ecbench run \
  --allow-legacy-spec --allow-busy --cpus inherit \
  --spec research/rootfind_production_panel_20261126/p31-production-panel.json \
  --out research/rootfind_e2e_measurement_20261127/local-run-busy

python3 research/rootfind_production_panel_20261126/analyze.py \
  research/rootfind_e2e_measurement_20261127/local-run-busy/records.jsonl \
  --output research/rootfind_e2e_measurement_20261127/local-run-busy/analysis.json

taskset -c 0 target/release/ecbench verify \
  --dir research/rootfind_e2e_measurement_20261127/local-run-busy \
  --replay-all --replay-seed 20261127 \
  --out research/rootfind_e2e_measurement_20261127/local-run-busy/verify-receipt.json
```
