# Explicit inversion of linear remainders

All four arms verified the scalar on all 44 targets. The inversion audit checked each returned inverse by multiplication, each failure by gcd, and lifted group outputs against generic Cantor operations. The collector retained identical anchor counts, admissions, cutoff and joins. The combined shortcuts reduced total field reductions on every input relative to the prior lifted solver.

| Genus | Targets | Faster than prior lifted solver | Prior / combined wall | Prior / combined REDCs | Original / combined wall |
|---|---:|---:|---:|---:|---:|
| 2 | 24 | 23 | 0.990–1.205× | 1.008–1.084× | 2.166–3.700× |
| 3 | 12 | 12 | 1.002–1.124× | 1.013–1.032× | 3.071–4.035× |
| 4 | 8 | 8 | 1.029–1.063× | 1.015–1.019× | 4.029–12.207× |

In genus two the quadratic-only and combined arms had identical reduction counts. In genera three and four, adding synthetic termination to the quadratic shortcut reduced counts and median wall time on every fixture. Two genus-four p=31 inputs reached tenfold in both metrics versus the original normalized solver: case 0 had 11.669× wall and 10.072× REDCs, case 1 had 12.207× wall and 10.738× REDCs. No other fixture reached tenfold wall time. Timing ratios are twelve-observation medians in this single registered run.

RUN.txt identifies the remotely verified freeze. CONTROL.txt retains 176 counter captures and 2,112 timings. summarize.rb regenerates COMPARISON.tsv.
