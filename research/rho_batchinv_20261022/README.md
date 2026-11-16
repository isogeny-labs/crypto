# Strong rho, round 4: the batched inversion (2026-10-22)

`Gf2::batch_inv_clmul`, the walk's Montgomery batch inversion, walks its
slices as `as_chunks::<4>()` arrays instead of zipped variable-length
`chunks`. The same products are formed, and every walk output is identical to
main's (`412b60289`).

| measure | main | round 4 | ratio |
|:--|--:|--:|--:|
| Callgrind Ir / walk step, n41 (4 fixtures) | 706.5 | 679.0 | 1.040 |
| Callgrind Ir / walk step, n53 (4 fixtures) | 707.2 | 679.7 | 1.040 |
| native ns / step, n53 (median of 5 A/B rounds; A/A 7.0%) | 110.6 | 99.0 | 1.117, **not established** (see below) |
| native ns / step, n61 (median of 5; A/A 14.4%) | 117.1 | 108.8 | 1.076, not established |

**The result is the instruction count.** The native n53 figure does not
survive validation:
- *Baseline drift.* In these runs main's A/B median (110.6) sits 7.5%
  above its own A/A median (102.9). Measured against the A/A median, the
  pooled baseline or the median paired ratio, the gain is 1.039–1.058,
  inside the 7.0% spread.
- *No replication.* An independent rerun found no gain: 16 paired rounds
  with the same binaries, a median paired ratio of 0.989, and 6 of 16
  rounds favouring the change. That rerun's own A/A spread was 26.5%, so
  it cannot exclude a gain of a few percent.

Validator findings F1–F3 cover both points.

Counted operations are identical in both arms, so the §5a band does not rank
them.

- Protocol: [`PROTOCOL.md`](PROTOCOL.md). It was written after the Callgrind
  count and before any timing.
- Report: [`report/report.pdf`](report/report.pdf) (source
  `report/report.typ`, graphs `report/plots.py`).
- Raw data: `runs/callgrind/`, `runs/identity/`, `runs/time/`.
- Reduction: `summarise.sh` produces `summary.json`.
- Timing: `run_time.sh`, which is round 3's script.
- Validation: [`benchmarks/20261022-strong-rho-batchinv/VALIDATION.json`](../../benchmarks/20261022-strong-rho-batchinv/VALIDATION.json).
