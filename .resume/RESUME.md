# Resume: round-2 IC leads (STACK + FEWCOL on Table B), 2026-11-18

- Branch `aburan28/inspiring-dijkstra-ay05qi`, draft PR #4738. Protocol frozen in b0c61aee53: `research/ic_tableb_fewcol_20261118/PROTOCOL.md`.
- Leads: `docs/leads/2026-11-18/ic-leads-round2/PROPOSALS.md`. L2-HRATE is closed (`HRATE.md`).
- To run: `cargo build --release --bin ecbench`, then `python3 tools/benchrec.py run <record> -- research/ic_tableb_fewcol_20261118/run.sh`. The runner writes sessions, audits, compares and bands under that directory.
- Afterwards: summary, graph, PDF and README; close the bench record; session log; tick ACTIVE; mark the PR ready.
- Next leads if these hold: L2-INNER (insert depth-<5 sums, about 40 lines of Rust), L2-FREE (`free=1`), L2-WSIB.
- Blocker carried over: main's malformed record `benchmarks/records/20261117T0334Z-hec-multi-production-cold.json` prevents `benchrec close` from rebuilding progress reports.
