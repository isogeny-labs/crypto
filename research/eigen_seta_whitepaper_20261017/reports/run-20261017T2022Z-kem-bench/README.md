# Run report: run-20261017T2022Z-kem-bench

Observations only; interpretation is in the white paper
(`crypto:papers/eigen-seta-whitepaper/`) and [`../../REPORT.md`](../../REPORT.md).
PDF: [`report.pdf`](report.pdf).

* **Command.** `bench_kem <SET> <keys> <trials> <jobs>` (eigenx
  `examples/bench_kem.rs`, commit `d89d790`, merged as eigenx#4), in order:
  `L1 5 10 1`, `L3 5 5 1`, `L5 5 5 1`, then under `taskset -c 0`
  `L1 2 5 1`, `L3 1 3 1`, `L5 1 3 1`, then `Q3 2 3 4`, `Q5 1 3 4`.
* **Host.** Intel Xeon @ 2.10 GHz, 4 vCPU shared cloud VM, Linux 6.18,
  rustc 1.97.0, release build. Isolation L0: all-core rows unpinned,
  single-core rows pinned to CPU 0. Started 20:22 UTC, finished 22:10 UTC.
  Interference: during Q5 key generation (21:29–22:02) a LaTeX build, a
  plotting script, a short ML-KEM run and a cargo download ran on the
  host. They may have lengthened Q5 key generation and did not overlap
  any other timed operation.
* **Requested / completed.** All requested keys and operations completed;
  every decapsulation, PKE decryption and public-key degree test returned
  the right value. No failures, panics or timeouts.
* **Values.** Medians [ranges], seconds, all cores:

| set | n keys | keygen wall | keygen CPU | encaps | decaps | n ops |
|---|---|---|---|---|---|---|
| L1 | 5 | 4.6 [3.6–6.5] | 5.4 [4.5–7.5] | 0.204 [0.165–0.304] | 1.20 [1.08–1.80] | 50 |
| L3 | 5 | 84.1 [76.5–94.0] | 87.8 [79.8–97.3] | 1.14 [0.80–1.57] | 5.83 [5.40–6.30] | 25 |
| L5 | 5 | 50.6 [39.2–80.3] | 60.2 [48.6–89.2] | 2.30 [1.74–3.06] | 12.78 [11.81–14.90] | 25 |
| Q3 (jobs=4) | 2 | 815 [690–941] | 2592 [2395–2790] | 5.88 [5.38–6.56] | 37.3 [36.1–38.5] | 6 |
| Q5 (jobs=4) | 1 | 2005 | 6909 | 13.7 [13.5–14.1] | 88.7 [87.8–89.7] | 3 |

  Single core (median): L1 encaps 0.240, decaps 1.88; L3 1.06 / 9.61; L5
  2.94 / 22.2. Field arithmetic, PKE and per-step chain cost: see the TSVs
  and the white paper's Tables 5 and 7.
* **Records.** [`../../records/run-20261017T2022Z-kem-bench/`](../../records/run-20261017T2022Z-kem-bench/)
  (one TSV per set and mode, `host.txt`).
* **Graph.** [`../../figures/perf.svg`](../../figures/perf.svg).
