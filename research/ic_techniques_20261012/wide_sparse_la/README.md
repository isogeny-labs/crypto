# Wide-modulus sparse linear algebra: benchmark pack

**Status (2026-11-12): protocol frozen, harness ready, NOT RUN.** No
measurement exists yet. The 10⁹-column cost in
`docs/ic/IC_TECHNIQUE_INTEGRATION_20261012.md` §9 is still an extrapolation.

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

Still to land after a run: `raw/`, `results.json`, report and graphs, the
`benchmarks/` entry and the `bench.record/v1` record (see PROTOCOL.md).
