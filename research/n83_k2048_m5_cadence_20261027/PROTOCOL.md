# Primary N83 K2048 five-summand XOR-cadence diagnostic

Frozen before source compilation or measured runs on 2026-10-27. This card
tests whether deferring native XOR elimination from every decision to every
eighth decision lowers counted solver work on the *same natural public point*
in the exact U and D K2048 chained-S3 model. The current solver default is
cadence 1. `Solver::set_xor_decision_cadence(8)` retains XOR checks at root
and complete assignment; the repository's exhaustive mixed CNF/XOR tests
cover both settings. The previous [m4 XOR-cache card](../n83_prebuild_cost_m23_20261019/reports/run-xor-cache-natural-20261020/RUN_REPORT.md)
reduced repeated row visits, but did not test this m5 cadence choice. The
[m5 natural card](../n83_k2048_m5_pair_20261027/PROTOCOL.md) supplies the
exact model and bounded baseline; the [constructed control](../n83_k2048_m5_planted_20261027/RESULT.md)
establishes satisfiability when the intermediate chain is supplied.

## Frozen workload

- Curve `EC1N83Ckb0h252dacc102a4`, polynomial basis
  `z^83+z^45+z^2+z+1`, `a=0`, `b=1`, subgroup order
  `2417851639230796216685689`.
- U `public_x_hash`, 2,048 signed-Frobenius columns, 339,968 points,
  compressed object SHA-256
  `aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a`.
- D `direct_subgroup_x_d18_prefix2048`, equal counts, compressed SHA-256
  `2406a7105bff162fc80e448d441709cc9f725e36da9751a2e9feb6f3e23512c7`.
- Public corpus SHA-256
  `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`;
  fixture 0 point x/y
  `1834913121178197242856238` /
  `8797964190732846213981377`. This point is common to both bases.
- Chained S3 with five summands, zero split bits, signed-Frobenius orbit
  domain, 300,000-variable and 8,000,000-domain-clause construction limits.
  Expected pre-search model: 111,478 variables, 1,140,557 initial clauses,
  4,980 native XOR rows. Reject any dimension mismatch.

## Eight fixed executions

Run one fresh process per row, sequentially on the same shared Apple M4 Pro
Darwin arm64 host. Use this fixed order, without replacing failed rows:

| Position | Base | XOR decision cadence |
| ---: | --- | ---: |
| 1 | U | 1 |
| 2 | D | 8 |
| 3 | D | 1 |
| 4 | U | 8 |
| 5 | U | 8 |
| 6 | D | 1 |
| 7 | D | 8 |
| 8 | U | 1 |

Each search stops at 5,000 conflicts or 240 seconds measured from the start
of the worker, whichever occurs first. Preserve `Sat`, `Unsat`, `Unknown`,
timeout, model error, input error, and process failure distinctly. Record
input/model/root/search/whole-process wall separately, solver decisions,
conflicts, restarts, XOR passes/row operations/scans, CNF visits, total
`word_ops`, and SAT-model validity. One worker runs at a time; no host-level
isolation or guaranteed memory cap is asserted. Record host OS/CPU and
background-noise caveat. The exact code and binary hashes are to be frozen
in an amendment after compilation and before execution.

Primary diagnostic: compare cadence-8 to cadence-1 `word_ops` for each base
at a matched `Unknown` 5,000-conflict endpoint; report both repeat values
and their spread. Search wall and phase times are descriptive L0 observations
without a qualified CPU speed ratio. A `Sat` result is a separate outcome:
check its complete SAT model, independently lift its five x values to subgroup
points, verify their group sum equals the public target, then count a verified
relation only after that replay. A setting that returns `Unknown` has no
verified natural relation, regardless of lower work. Do not use these
one-point stage observations to rank U versus D or to fill an IC candidate
online-time field.

Afterward, a calibrated Tier 1 or Tier 0 same-point comparison must include
relation yield/rank and full target-log recovery with a paired rho solve to
select a factor base. Preserve this card and all eight rows even if cadence
8 performs worse or ends at a different status.
