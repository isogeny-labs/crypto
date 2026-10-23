# `scalar_mul_ct` bit-length timing leak: security-timing evidence

Copies of the visual record of
[`research/scalar_mul_ct_timing_20261022`](../../research/scalar_mul_ct_timing_20261022/README.md),
which stays the source of truth (protocol, raw records, summaries, sources):

- [`REPORT.pdf`](REPORT.pdf): both runs;
- [`fig_timing_run2.svg`](fig_timing_run2.svg) / [`.png`](fig_timing_run2.png): run 2, the shipped fix;
- [`fig_timing.svg`](fig_timing.svg) / [`.png`](fig_timing.png): run 1, the first version.

**Scope.** This is side-channel timing evidence for a constant-time fix in
`Point::scalar_mul_ct`, the affine ladder behind `ecc::ct::scalar_mul_secret`
on every curve but secp256k1 and P-256. It is not a solver-performance result,
compares no ECDLP method and ranks nothing against rho. It is therefore not a
row of the admitted results table: the validation gate (`docs/VALIDATION_GATE.md`)
admits a row only with a matched rho reference, and this study has none to
match.

**Headline, run 2** (as the study reports it; 5 rotated rounds, 15 of 15
processes uncontended, pinned result digests identical across runs):

| | SM2 | GOST-256 test |
|:--|:--|:--|
| before: time of a 224-leading-zero scalar ÷ a full-length one, per round | 0.33–0.45 | 0.37–0.47 |
| after: largest class deviation from full length, worst round | 0.138 | 0.165 |
| A/A noise band δ | 0.296 | 0.296 |
| full-length cost, after ÷ before | 0.96–1.04× (median 1.01) | 0.86–1.09× (median 0.93) |

Run 1 (the subtrahend `[2^L]P` by affine doublings) closed the leak too, at
1.29–1.55× the cost; run 2 computes it with the Jacobian `Point::scalar_mul`.

**Host.** 4 vCPU guest; every process single-threaded on one reserved CPU
under `tools/isolated_bench.py`. The A/A band is wide (23–30 %), so the
deterministic identity-step counter test in `src/ecc/point.rs` is the stronger
evidence that the ladder's work no longer depends on the scalar's length.
