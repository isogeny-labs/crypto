# More hits per anchor and their linear cost

All five arms verified the scalar on all 44 inputs. Every wider policy reduced anchor visits on all 24 genus-two and 12 genus-three targets. Even the smallest expansion improved wall time on only 7/24 and 4/12 respectively. Each cell below divides the original aligned-width solver's median wall time by the wider solver's median. Only the four genus-four p=61 targets changed width; the p=31 scan already covered its entire catalogue.

| Genus | Width policy | Distinct-width fixtures faster | Distinct-width fixtures using fewer anchors | Baseline / candidate wall across whole genus |
|---|---|---:|---:|---:|
| 2 | 1.5 W | 7/24 | 24/24 | 0.771–1.086× |
| 2 | 2 W | 1/24 | 24/24 | 0.606–1.104× |
| 2 | 3 W | 0/24 | 24/24 | 0.351–0.959× |
| 3 | 1.5 W | 4/12 | 12/12 | 0.718–1.085× |
| 3 | 2 W | 1/12 | 12/12 | 0.571–1.016× |
| 3 | 3 W | 0/12 | 12/12 | 0.307–0.949× |
| 4 | 1.5 W | 4/4 | 4/4 | 1.016–1.196× |
| 4 | 2 W | 4/4 | 4/4 | 1.030–1.232× |
| 4 | 3 W | 4/4 | 4/4 | 1.034–1.249× |

At genus two p=4001 case 0, widths 90 and 270 consumed 156 and 30 anchors respectively. Base-field REDCs fell from 85,088 to 66,066, while subgroup REDCs grew from 15,370 to 231,154. This quantifies how a fivefold reduction in anchors can increase the full arithmetic bill. The experiment leaves open a wider collector paired with a different elimination algorithm or a different small/large partition.

Two genus-four p=31 cases exceeded tenfold time over the original normalized solver in all three wider arms. Those cases had identical effective widths to the aligned control. Timing differences between these width policies on p=31 cannot be credited to more scanned places. No genus-two or genus-three expanded arm reached tenfold. The full-solver goal remains open.

RUN.txt identifies the Git freeze. CONTROL.txt holds 220 counted calls and 2,200 rotating full-solver timings. summarize.rb regenerates COMPARISON.tsv including requested-arm identifiers and effective widths.
