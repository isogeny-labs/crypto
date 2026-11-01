# t = 14 cell of the PKM-tower fall-degree study (in progress)

**Status, 2026-11-01: tooling only. No study row exists and no result is claimed.**

The decisive cell of `research/prime_fall_degree_20261027/PREREGISTRATION.md` (Amendment 7 B): Kummer,
`m = 2`, `P1 = 2013265921`, `t = 14`, two random draws, capped F4 at the degree that brackets the
registered prediction `C-step` (7). This directory holds what a run needs; its plan is committed
before any row.

- `run_cells.sh`: the cells (`R` regression, `T <cap> <draw>` the bracket, `G` decomposability), with the
  frozen options of `run.sh` and Amendment 7 A's limits.
- `../../examples/pfd_grid_zeros.rs`: counts the zeros of `S₃` on the grid `V × V` in O(K) memory
  (Amendment 7 C: draws with a zero are decomposable and excluded). It agrees with
  `prime_fall_degree_tools ns-degree` on 8 draws at `p = 786433`, `t = 10` (counts 0, 0, 2, 2, 2, 2, 4, 2),
  gives exactly 2 on a planted draw, and 0 on the random draws at `t = 6 … 9`.
- `PLAN.md`, `runs/`, `REPORT.md`: to follow.

Route: AGENTS.md §9a sends x86 experiments to Iso Lab on EKS Spot, and Amendment 7 A sizes each `t = 14`
system at 112 GiB. The session that wrote this had 4 vCPUs, 15 GB and no AWS access, so any row it produces is
off-route and is reported separately. `run_cells.sh` is route-independent: `BIN`, `BUDGET`, `DENSE`, `VMEM_KB`
and `THREADS` are environment overrides.
