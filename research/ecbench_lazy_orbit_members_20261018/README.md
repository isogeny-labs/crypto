# Lazy orbit-member index in the Frobenius-folded pair table (2026-10-18)

**Result (engineering, measured):** the frozen Koblitz orbit index calculus costs
1.12×–1.44× less in counted, fully priced GAE on seven curves (n = 13–37) and
1.014× less at n = 41, with the lower end of every paired 95 % interval above 1.
Trials, relations, rank and recovered logarithm are identical on all 856
baseline/lazy run pairs. At n = 31 the distance to the matched strong rho went
from 1.221 to 0.985 [0.908, 1.062]; at n = 23 from 1.342 to 1.116. No ranking
against rho is claimed.

| file | what |
|:--|:--|
| [`REPORT.pdf`](REPORT.pdf) / [`REPORT.typ`](REPORT.typ) | results, graphs, scope and limits |
| [`PROTOCOL.md`](PROTOCOL.md) | frozen before the data (commit `0334afb11`) |
| [`reports/run-20261018T0540Z/`](reports/run-20261018T0540Z/) | the run report: command, host, sessions, completion |
| `sessions/` | 8 `ecbench` sessions, loose (CI re-audits committed sessions); `audits/`, `tables/`, `logs/` |
| `figures/` | `plot_figures.py` (lays out recorded values only), `summary.json` (derived phase means), `ratios.json` (copied comparisons, read by the leaderboard) |
| [`../../benchmarks/20261018-lazy-orbit-members/`](../../benchmarks/20261018-lazy-orbit-members/) | PDF, SVG and PNG copies |

Change: `FrobeniusPairTable::build_lazy` and oracle `mitm-frobenius-lazy-counted`
(`src/cryptanalysis/ic_boundary.rs`, `ic_framework/plugins.rs`,
`ecbench/methods.rs`). The eager oracle and every committed session are unchanged.

Reproduce: `cargo build --release --bin ecbench`, `bash make_specs.sh` (rewrites the
specs byte for byte), `bash run.sh`; `ecbench verify --dir sessions/<s> --replay-all`
re-audits a committed session. Next experiment (not part of this claim): fewer
columns under the lazy oracle (exploratory scratch sessions favoured c = 2 at
n = 17, 29, 31).
