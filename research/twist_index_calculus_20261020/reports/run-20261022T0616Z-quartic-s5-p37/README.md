# Native S5 pilot, p=37

Run ID `run-20261022T0616Z-quartic-s5-p37`; 22 October 2026; Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. One public known-answer target `[271]G`. Registered [protocol](../../QUARTIC_TWIST_LINE_PROTOCOL.md), [raw record](../../quartic_twist_s5_p37.json), [stderr log](../../quartic_twist_s5_p37.log), and [graph](s5_cost.svg). Source SHA-256: `049e0bbcd34791c920176c32b65181d509d992edd47d05f1cf13e10ff1d5ca31`.

Command: `RAYON_NUM_THREADS=16 TMPDIR=/home/ec2-user/.cache/gls-report-tmp timeout 1800s target/release/examples/quartic_twist_s5_pilot --p 37 research/twist_index_calculus_20261020/quartic_twist_mixed_v1.ndjson`. The process watchdog was 1,800 seconds; both registered queries completed. The exact signed meet-in-the-middle oracle and group addition checked all returned quadruples. This is a two-query oracle pilot, not a full IC solve.

| Query | Index in frozen L0 stream | Exact decompositions | Native S5 solve `Fp M` | L0 solve ms | Oracle agreement |
| --- | ---: | ---: | ---: | ---: | :---: |
| first_success | 0 | 1 | 1,157,849,935,909 | 52,254.2 | yes |
| first_failure | 1 | 0 | 1,158,550,910,260 | 58,213.6 | yes |

Symmetrisation precomputation: 1,226,109,078 `Fp M`, 347.4 L0 ms. Each solve used a degree-29 Macaulay matrix with 41,780 rows and 40,920 columns. L0 timing is a shared-host observation and is not an isolated performance comparison. The S5 system is the single `x∈Fp` line; a tagged two-line or two-dimensional polynomial system was not run. msolve/Magma were unavailable in this environment and were not compared.

![Counted field multiplications for both registered queries](s5_cost.svg)

Graph source: [plot_quartic_followups.py](plot_quartic_followups.py). [PDF report](README.pdf). No failed, extraneous or censored outcome occurred.
