# Wide-modulus sparse linear algebra: benchmark pack

**Status (2026-11-14): measured on RunPod (2026-11-12/13). The frozen
verdict is "extrapolation unsupported".** The counted size law holds:
β = 1.96–1.97 for every arm, curvature not significant, count independent of
the width. The width-law test failed, because the time per operation depends
on the modulus within one word, not only on the limb count. So the 10⁹-column
cost in `docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` §9 is still not
supported by a fitted law. All figures are `[pcc:unverified]`. Full read-out:
[`REPORT.md`](REPORT.md) ([PDF](REPORT.pdf)).

- [`results.json`](results.json): the frozen analysis, produced by the native
  crate [`analysis/`](analysis/) (`wide-sla-analysis --raw raw --out results.json`;
  cross-check against the Python prototype in
  [`analysis/CROSSCHECK.md`](analysis/CROSSCHECK.md)). Graphs:
  [`figures/`](figures/), drawn by [`plot.py`](plot.py) from `results.json`
  and `raw/`.
- [`raw/`](raw/): 438 attempts from three RunPod pods, one per seed (414 valid,
  24 timeouts at D = 2²⁰ in the wide arms). Launch:
  [`launch.json`](launch.json), [`runpod/`](runpod/); the failed first launch:
  [`launch.attempt1.json`](launch.attempt1.json).
- [`PROTOCOL_AMENDMENT_1.md`](PROTOCOL_AMENDMENT_1.md): RunPod in place of
  Iso Lab, committed before any data.
- [`PROTOCOL.md`](PROTOCOL.md): the measurement §9 promised (counted work per
  column against modulus width 61→255 bits and size 2¹²→2²⁰ columns, for
  filter + block Wiedemann and filter + block Lanczos; the law to fit and how
  it is falsified; commands; records and checkpoint steps), and why it was
  not run in the authoring container.
- Harness: `examples/la_engine_bench.rs`, `ours --modulus-bits B
  [--word u128|u192|u256]`, B ∈ {61, 89, 127, 129, 190, 255}. B = 61 is the
  `u64` control and is the existing `--modulus 2305843009213693951` run with
  extra fields; B = 129 is the ECC2K-130 subgroup order (bit length 130).
  Without `--modulus-bits` the harness output is unchanged.
- [`devcheck.jsonl`](devcheck.jsonl): **local development check of
  correctness, not timings.** 52 runs: D = 256 and 1024, m = 3, seeds 1–2,
  every B, `full-bw` and `full-bl`; plus `--word u128` at 61 bits and
  `--coeffs wide` at 129 bits (D = 1024, seeds 1–2, `full-bw`). Every run
  `verified: true`. Each line is the harness output with one added key,
  `check`. Host: shared 4-vCPU container (Intel Xeon @ 2.30 GHz), commit
  `ac9c937bbe` plus the uncommitted harness change; the `*_s` fields there
  are not measurements.

Bench record: `benchmarks/records/20261112T2153Z-wide-sparse-la.json`;
copies in `benchmarks/20261112-wide-sparse-la/`. There is no admitted
`benchmarks/README.md` row: a phase cost has no matched rho for the
validation gate.
