# Retained costs in the tenfold budget

Both full solvers recovered and verified the planted scalar on all 44 targets. The other three arms are nested accounting controls. Below, each count is the number of inputs on which the control median equals or exceeds one tenth of the corresponding original solver median. The REDC column applies the same comparison to exact total field reduction counts for the complete control.

| Genus | Targets | Eligibility | Plus preparation | Plus verification | Complete control exceeds REDC budget | Complete control / fused time |
|---|---:|---:|---:|---:|---:|---:|
| 2 | 24 | 8 | 11 | 16 | 20 | 9.9–64.3% |
| 3 | 12 | 4 | 5 | 10 | 12 | 26.1–69.5% |
| 4 | 8 | 1 | 1 | 6 | 7 | 47.9–58.2% |

On 32/44 inputs the observed eligibility, preparation and final scalar check together consume the entire tenfold wall budget even when supplied with the correct scalar. On 39/44 inputs their exact REDCs alone exceed the tenfold reduction-count budget. Eligibility alone consumes the wall budget on 13/44. The public checks and group operations warrant independent optimization alongside relation production. Lower retained overhead alone also cannot solve the problem: on the least overhead-bound genus-two cell, the combined control used only 9.9% of the current median. Scope includes fresh per-target preparation, public eligibility and original-group verification throughout.

Illustration: genus two p=101 case 0 has original median 107,000 ns, fused median 58,542 ns and combined control median 34,000 ns. Its tenfold time budget is 10,700 ns. The complete control uses 6,410 REDCs against 12,786 in the original solve. These control medians measure separate calls, not instrumented subintervals or unconditional algorithmic lower bounds.

The full solver achieved 1.636–2.731×, 1.750–2.942× and 2.549–7.606× respectively in this timing replay. No target reached tenfold. The control arms did not compute any scalar. The prospective source commit is in RUN.txt; summarize.rb regenerates BUDGET.tsv from all 3,300 individual timings and 220 counter snapshots.
