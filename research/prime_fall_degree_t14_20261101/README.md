# t = 14 cell of the PKM-tower fall-degree study (in progress)

**Status, 2026-11-01: cap-5 cells done, cap-6 cells owed (route gap).** At `t = 14` cap 5 does not refute on either
draw and neither draw is decomposable, so `d_F ≥ 6` (a lower bound). C-step at `t = 14` stays undecided until the
cap-6 cells (`run_cells.sh T 6 0`, `T 6 1`) run on a 112 GiB Iso Lab job. See `REPORT.md` / `REPORT.pdf`.

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
- `PLAN.md` (frozen at `aa374b555`), `runs/`, `preflight/`, `REPORT.md`, `REPORT.pdf`, `cost_vs_t.svg`: the record.

Route: AGENTS.md §9a sends x86 experiments to Iso Lab on EKS Spot, and Amendment 7 A sizes each `t = 14`
system at 112 GiB. The session that wrote this had 4 vCPUs, 15 GB and no AWS access, so any row it produces is
off-route and is reported separately. `run_cells.sh` is route-independent: `BIN`, `BUDGET`, `DENSE`, `VMEM_KB`
and `THREADS` are environment overrides.

## Routing the cap-6 cells (2026-11-01): blocked by the integration, not launched

The owner asked for `run_cells.sh T 6 0` and `T 6 1` on Iso Lab. None of the paths in
`.claude/skills/isolab-eks` carries this workload as it stands:

- `isolab.eks.request/v1` and `isolab-experiment.yml` take an `ecbench.spec/v1` and cap `seconds` at 3,600
  (`taskq/deploy/eks-karpenter/contracts/request.schema.json`). These cells are a custom native binary
  (`pkm_tower_pilot`), 86,400 s per system, 112 GiB.
- `isolab_k8s_run` (argv plus a digest-pinned image) is the custom-command path. Its stdio MCP binary lives on the
  lab host, not in the container that wrote this, which also has no `aws` CLI. No workflow builds an image holding
  `pkm_tower_pilot`.
- Spot can interrupt a 24 h job; the non-resumable path would restart it. A resumable run needs
  `isolab.eks.resumable/v1` and a recovery image, which is a separate request.

Needed from the owner or the lab host: (1) the route (`isolab_k8s_run` on the lab host, or an extension of the
request schema or workflow), (2) a digest-pinned image with `pkm_tower_pilot` built from a pinned commit, (3) whether
the run is resumable. `run_cells.sh` is route-independent (`BIN`, `BUDGET=86400`, `DENSE`, `VMEM_KB`, `THREADS`).
`DENSE` for a 112 GiB request is 2.79·10¹⁰ (27,900,000,000; the largest multiple of 10⁸ with `4·C <= 112 GiB - 8 GiB`, Amendment 7 A).
