# Quotient-form doubling and coprime addition

All four arms verified all 44 targets. The three fused arms agreed on the scalar, anchor count, cutoff, accepted rows, rejections and joins. Audited table entries and partial sums matched the generic group law. The full lifted arm reduced complete median wall time and total field reductions relative to the prior signed shared-table solver on every input.

| Genus | Targets | Original / lifted time | Shared table / lifted time | Shared table / lifted REDCs | Targets reaching 10× original time |
|---|---:|---:|---:|---:|---:|
| 2 | 24 | 2.254–3.300× | 1.006–1.216× | 1.023–1.217× | 0 |
| 3 | 12 | 2.924–4.061× | 1.065–1.239× | 1.084–1.200× | 0 |
| 4 | 8 | 4.100–12.072× | 1.134–1.246× | 1.145–1.191× | 2 |

Genus four p=31 case 1 reached 12.072× in complete time and 10.574× in field reduction count versus the original normalized control. Case 0 reached 11.019× in time and 9.927× in reductions. These timings are from one registered run on the same public panel used to develop the preceding shared-table implementation. The remaining 42 cells stayed below tenfold time. The incremental coprime-addition comparison against lifted doubling alone was faster on 43/44 targets and reduced field reductions on all 44.

RUN.txt identifies the prospective source commit verified against the GitHub PR head before execution. summarize.rb regenerates COMPARISON.tsv from the 176 counted calls and 2,112 timed observations in CONTROL.txt.
