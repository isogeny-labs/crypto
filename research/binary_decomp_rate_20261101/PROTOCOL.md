# Decomposition rate against base size and summand count on Koblitz E_0: protocol

Frozen before any measured run (2026-11-01). Owner request: "A and b";
(b) is the decomposition-rate lead.

## Question

Every complete index-calculus arm on the Koblitz E_0 ladder sits above
same-target strong rho, and the gap widens with size: the m = 3 root finder
at 2.75× (n = 13) and 17.1× (n = 19), the m = 2 one at 2.01× and 11.0×
(`research/ecbench_rootfind_engine_20261012`). Those used one base per size.
The cost of a root finder is its table, plus |F| probes per trial (m = 3)
or one (m = 2), times the trials one relation takes, times the relations the
base needs. The base dimension sets the decomposition rate and the number of
relations needed. This round measures that trade directly.

- **Q1 (primary).** Per size, which (summands, base dimension) cell has the
  lowest median `S`, and is it lower than the incumbent cell (the
  rootfind-engine study's: d = 5 at n = 13, d = 7 at n = 19, and d = 8 at
  n = 23 from the F6 ladder)? Statistic: per-target `log₂(S(incumbent)/S(cell))`.
  Rule: **improves** when its median is > 0 and the paired bootstrap 95%
  interval of the median excludes 0.
- **Q2 (mechanism, derived).** Per cell: trials per relation (decomposition
  rate), relations found, table additions and probe additions, from the
  records.
- **Q3 (distance remaining).** `S(cell)/S(rho-strong)` per target, median
  and range. Not a gate.
- **§5a.** The root finders' unpriced native work (lookups, Frobenius maps)
  is priced at 0, 0.1 and 1 GAE per operation (`ecbench band`); a cell
  ranks above another only where the order holds at every weight.

## Workloads

| size | target seed | measurement seed | base dimensions d | timeout |
| --- | --- | --- | --- | --- |
| n = 13 | 202611011301 | 202611011302 | 4, 5, 6 | 1800 s |
| n = 19 | 202611011901 | 202611011902 | 6, 7, 8 | 1800 s |
| n = 23 | 202611012301 | 202611012302 | 7, 8, 9 | 3600 s |

Koblitz E_0 (a = 0), 8 public targets per size, 3 rounds, 1 warmup,
alternating order, isolation L2 required. Fresh target seeds (dated
today) so the cells are not tuned on earlier targets.

## Arms (per size)

- `rho-strong` (reference) and `rho-strong-aa` (A/A): `rho.signed_frobenius_strong`, 32 lanes, dp 4.
- `rf-m2-d{d}`: `pdp3-koblitz:m=2,engine=rootfind` on `koblitz-standard-subspace:dimension=d`, for each d.
- `rf-m3-d{d}`: `pdp3-koblitz:m=3,engine=rootfind` on the same bases.

Six IC arms and two rho arms per size. Specs: `SPEC-n13.json`,
`SPEC-n19.json`, `SPEC-n23.json` (this directory).

## Route, accounting, stop

As in `research/f6_ic_pair_index_v2_20261101/PROTOCOL.md`: isolab-eks, L2,
replay-all before analysis, `ecbench.gae` at the frozen commit, wall time a
note only. A size with fewer than 6 complete targets per arm reports no
decision. This is parameter selection by AGENTS.md's classes: a better cell
is an improvement of the IC baseline at that size, not a new method.
