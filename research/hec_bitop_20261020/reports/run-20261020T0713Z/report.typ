#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
= Run report: primary rows, 2026-10-20 07:13–07:16 UTC

*Study* `research/hec_bitop_20261020` (`PROTOCOL.md`, Instances and runs).

*Command* `run.sh` via task card `cards/CARD-hec-bitop-20261020-1.yaml` at commit `a3c81b2ae`.
- *Receipt:* `cards/CARD-hec-bitop-20261020-1.receipt.json`, exit 0, 155.758 s.
- *Assessment:* `executed_as_specified`; relay verbatim.

*Binary* `target/release/examples/hyperelliptic_ic_vs_rho_bitops`, sha256 `a2569d32…6dce`.

*Host* Intel Xeon \@ 2.10 GHz, 4 vCPU. Counts only, so isolation is not relevant. One thread per genus, with the three genera in parallel.

*Workloads* The 18 round-seven rows, fb-walk configuration:
- genus 2: p ∈ {41, 61, 101, 151, 211, 251};
- genus 3: p ∈ {23, 31, 41, 61, 101, 151, 211};
- genus 4: p ∈ {17, 23, 31, 41, 61}.

*Arms*
- index calculus: seed 20260916, 5 runs per row;
- rho: full-width distinguished-point walk, seed `0xC0FFEE`, 40 runs per row.

*Runs* 90 index-calculus runs and 720 rho runs. Every one recovered and verified its logarithm. There were no failures.

*Measured values* Per-run bitop/v1 per arm and phase are in `raw/g2.jsonl`, `raw/g3.jsonl` and `raw/g4.jsonl`. The derived ratios and fits are in `analysis/analysis.json`, computed by `examples/hec_bitop_analyse.rs` with 2,000 bootstrap resamples, seed 20261020.

#figure(image("ratio_vs_n.svg", width: 95%), caption: [IC/rho per row against log₂ N, bitop/v1, with 95% bootstrap intervals; crosses are round seven's in-situ ratios.])

*Raw records* `raw/g{2,3,4}.jsonl`, with the logs in `raw/g{2,3,4}.txt`.
