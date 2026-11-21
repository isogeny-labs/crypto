# Paired full-solver timing on 48 public inputs

Both runs completed all 2,304 solves and verified every scalar. Each range below spans the sixteen per-input median paired speedups for one genus.

| Genus | First run | Second run | At least tenfold in both |
|---|---:|---:|---:|
| 2 | 3.08–4.35× | 3.01–4.40× | 0/16 |
| 3 | 2.72–3.61× | 2.71–3.55× | 0/16 |
| 4 | 3.80–7.76× | 3.95–7.88× | 0/16 |

These ratios compare the original solver with the integrated signed order certificate. Each invocation rebuilt its power table when applicable and prepared its catalogue. Public fixture generation occurred outside the timer. The host was shared. The newer graph relation and integer certificate prototypes require a separate integrated comparison. Run ruby summarize.rb in this directory to reproduce RATIOS.tsv.
