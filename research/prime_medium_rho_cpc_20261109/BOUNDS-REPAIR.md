# Bound records repair (2026-11-11)

## What was wrong

From commit `40503d051c` (2026-11-09) on, the ecbench CI step
`ecbench bound check --root . --record docs/bounds/records/*.json` failed on the
six `docs/bounds/records/prime-medium-cpc-rho-*.json` records with
"re-derived bound ... differs in: provenance".

Each record's `provenance.audits[]` pinned the sha256 of
`sessions/M{1..4}.audit.json` from an earlier local re-audit (M1 `940d002b…`,
M2 `dfc47534…`, M3 `de2ec538…`, M4 `d558e42f…`). Those receipt bytes were never
committed. The receipts committed later in `1712923dc0` came from a second
re-audit (`analysis.sh`) and hash differently (M1 `cbe5f286…`, M2 `40a7544e…`,
M3 `ad52e6cc…`, M4 `e3e5ff52…`). `analysis/bound-*.json` already pin these
committed receipts. The pinned and committed receipts report the same values:
`ok = true`, the same session ids, and the same replay counts
(1152/1152, 384/384, 96/96 and 96/96 reproduced). The session records did not
change. Every `provenance.sessions[].records_sha256` equals the sha256 of the
committed `sessions/M*/records.jsonl`.

## What changed

The six records were re-fitted with `ecbench bound fit`
(`regen_bound_records.sh`). The fit used each record's own sessions, arm, label,
notes, weight and fit options, and cited the committed audit receipts. Old and
new records differ in `bound_id` and in the four `provenance.audits[].sha256`
values. No other bytes changed. Constants, fits, stages, sizes, admissibility,
`verdict_id` (null) and `improves_on` are unchanged.

| record | old bound_id | new bound_id |
|---|---|---|
| prime-medium-cpc-rho-neg-w0.1 | ECBND1h3be5d3c55c05 | ECBND1hd757bf6022ad |
| prime-medium-cpc-rho-neg-w1 | ECBND1he5f5bb348f82 | ECBND1h134698e0844b |
| prime-medium-cpc-rho-neg | ECBND1h3b357487c59f | ECBND1hb26eeb9edb78 |
| prime-medium-cpc-rho-plain-w0.1 | ECBND1h9a9f2fd12811 | ECBND1h1428394e74a8 |
| prime-medium-cpc-rho-plain-w1 | ECBND1h1d791f3ff0f0 | ECBND1h4c7f06955861 |
| prime-medium-cpc-rho-plain | ECBND1h6263cb36e679 | ECBND1he0cad6abe1b6 |

`docs/bounds/frontier.json` and `FRONTIER.md` were regenerated. After the old
ids are replaced by the new ones, two things differ: the order of the sorted
bound-id list and the frontier id (`ECFR1h5cb122714a06` → `ECFR1h05fc5ea82fcb`).
The leaders, dominance and numbers are unchanged. Only `docs/bounds/`
referenced the old ids on `origin/main`.

## Commands

```
cargo build --release --bin ecbench
OUT=<dir> research/prime_medium_rho_cpc_20261109/regen_bound_records.sh; cp <dir>/*.json docs/bounds/records/
ecbench frontier build --bounds docs/bounds/records --out docs/bounds/frontier.json --markdown docs/bounds/FRONTIER.md
ecbench bound check --root . --record docs/bounds/records/*.json         # 166/166 OK
ecbench challenge check --file docs/bounds/challenges/*.json             # OK
ecbench frontier build ... --check                                       # OK
python3 tools/pcc/pcc.py canary; python3 tools/pcc/pcc.py ledger verify --base origin/main   # OK
```
