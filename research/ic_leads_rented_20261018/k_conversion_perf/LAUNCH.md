# Launch card P5: K conversion at degree 4, N = 34–44, with hardware counters

**Protocol:** [`research/k_conversion_20261018/PROTOCOL_AMENDMENT_A1_perf.md`](../../k_conversion_20261018/PROTOCOL_AMENDMENT_A1_perf.md), frozen 2026-10-18 before any degree-4 perf datum. Instrument: [`examples/k_conversion_perf.rs`](../../../examples/k_conversion_perf.rs) puts `perf_event_open` interval counters around exactly the intervals the Callgrind toggle measured (the dense full-MutantXL closure; `StrongRhoG::solve`). Driver: [`perf/run_perf.sh`](../../k_conversion_20261018/perf/run_perf.sh); analysis: `perf/analyze.py`.
**Question:** K = (instructions per column) / (instructions per rho step) at degree 4 for N = 34, 36, 42, 44, and its growth with columns. Callgrind gave 4.0–15.6 at degree 3 and 83 at degree 4, N = 26. Class: accounting (a change of unit), not an improvement.

## Host: **not RunPod; the owner's own Linux machine**

The PMU probe (item 5, [`../pmu_probe/README.md`](../pmu_probe/README.md)) found that RunPod containers sit on hosts with a real PMU (`cpu` event source, AMD EPYC 9655), but `perf_event_paranoid = 4`. `perf_event_open` of cycles and instructions then returns EPERM, and a container cannot lower it. This container has no PMU at all (ENOENT). So the run needs bare metal, or a VM with a vPMU, with `perf_event_paranoid ≤ 2` (`sudo sysctl -w kernel.perf_event_paranoid=2`), ≥ 16 GB RAM and `perf` installed for the whole-process null control (`linux-tools-generic`). A rented bare-metal box (outside RunPod) also works; nothing in the job is RunPod-specific.

| | |
|---|---|
| cost | **$0** on the owner's host (no rented hardware) |
| hours | 18.5 h sequential (3 repeats × 16 degree-4 draws, from one calibration point; ±2×), as low as about 2 h if degree 4 builds as few columns as degree 3 did (0.43–0.84 of C(N, ≤D)); `PAR=4` gives about 5–6 h with up to 38 GB |
| memory | ≤ 9.6 GB per job (N = 44, measured ratio 3.4·c²/8) |

## Commands (on the owner's Linux host, from a checkout of `main` after this PR merges)

```sh
sudo sysctl -w kernel.perf_event_paranoid=2        # if above 2
cargo build --release --example k_conversion_perf
target/release/examples/k_conversion_perf probe; echo $?    # 0 = counters work, 3 = this host cannot count
research/k_conversion_20261018/perf/run_perf.sh 2>&1 | tee /tmp/k-perf.log
python3 research/k_conversion_20261018/perf/analyze.py research/k_conversion_20261018/perf/runs/<host>-<UTC>
```

## Results back

Commit `research/k_conversion_20261018/perf/runs/<host>-<UTC>/` (`host.txt`, `raw.jsonl`, `null_whole.jsonl`, `parts/`) and `RESULT.json`. A session then writes the reading by the amendment's §6. The positive control is degree-3 perf instructions within ±5 % of the committed Callgrind `Ir` on the same draws; if it fails, nothing is reported as K.

## Tested before launch (this container, no PMU)

Build, `rustfmt --check`, and clippy on the example are clean. `probe` gives `unsupported` (ENOENT) and exit 3, as do closure and rho mode. With `KPERF_SOFTWARE=1`, the closure on m3-16-6 draw 0 at degree 3 reproduces the committed engine counters exactly (columns 4,861, word ops 9,130,386, products 2,601), and rho n61 seed 1 the committed 609,340 steps, verified. `run_perf.sh --dry` exits 0 (also with `PAR=2`). Without `--dry` it exits 3 and writes an `unsupported` record. Outputs: `research/k_conversion_20261018/perf/test_local/`.
