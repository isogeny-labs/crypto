# Substituting sparse definitions directly

Every arm verified all 44 planted scalars. The matched arms agreed on answers, anchor counts, prefix sizes, finite admissions, rejected rows and joins. The audit checked scalar readiness against the existing linear solver after each insertion.

| Genus | Direct rows across targets | Faster than matched graph | Matched graph / direct | Previous solver / direct | Original normalized / direct |
|---|---:|---:|---:|---:|---:|
| 2 | 5,845 | 24/24 | 1.029–1.114× | 1.022–1.257× | 1.499–2.650× |
| 3 | 141 | 8/12 | 0.968–1.049× | 0.988–1.075× | 1.766–3.153× |
| 4 | 245 | 8/8 | 1.005–1.083× | 1.022–1.179× | 2.727–5.190× |

Ratios use complete-solver median wall times from twelve rotating observations per arm per target. Direct rows include initial definitions, substitutions and rows with solely small coefficients. The matched graph and direct reduction-count ratios rounded to 1.000 on every fixture. Improvements in full time therefore came with essentially unchanged arithmetic in this counting model. Genus three is mixed, with some regressions on its two-large-place schedule. No measured fixture reached the tenfold threshold. This experiment isolates a linear-solver implementation effect on a fixed mathematical relation stream.

RUN.txt identifies the remotely verified freeze. CONTROL.txt includes individual timings, row counts and field counter vectors. summarize.rb regenerates COMPARISON.tsv.
