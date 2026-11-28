# Truncated norm quotient in Jacobian reduction

Every arm verified all 44 scalars. Each audited shortcut satisfied the full exact polynomial identity before normalization, and every resulting group value matched generic composition. The candidates preserved the relation stream.

| Genus | Fixtures | Prior / addition-only wall | Prior / addition-and-doubling wall | Original / addition-and-doubling wall |
|---|---:|---:|---:|---:|
| 2 | 24 | 0.989–1.130× | 0.959–1.094× | 2.362–3.812× |
| 3 | 12 | 1.002–1.115× | 0.992–1.152× | 3.086–4.499× |
| 4 | 8 | 0.994–1.116× | 0.999–1.117× | 4.110–12.573× |

The addition-only change reduced total field reductions on 44/44 inputs and median wall time on 42/44. When extending the shortcut to doubling, the explicit genus-two formula further reduced arithmetic on 24/24, but the general high-diagonal loop increased total reductions relative to addition-only on all 20 higher-genus inputs. For genus four p=31 cases 0 and 1 the combined policy reached 11.720× and 12.573× original time and 9.870× and 10.447× original reduction counts. The other 42 targets remained below tenfold wall time. These measured timings are twelve-observation medians on the existing panel.

RUN.txt identifies the remotely verified prospective commit. CONTROL.txt contains 176 counted calls and 2,112 timings. summarize.rb rebuilds COMPARISON.tsv.
