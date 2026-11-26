# PKM native gate source repin after CI assertion failure

Frozen 2026-11-26 before any qualifying cold panel timing. This amends
`AMENDMENT_FORCE_DESCENT_20261125.md` and leaves the nine-curve roster, target
scalars, arm order, resource limits, and complete-cost accounting unchanged.

CI run `45722437269` on `aab505dd2d24cd5c98f9be1a2bd2243793a2001b`
ran 34 PKM tests: 33 passed and one failed. The independent F97 census and
`descent_roots_match_ground_truth_at_m3` passed. The forced public-path test
recovered and verified its planted relation, then failed at an assertion that
`pkm_basis_dfs_root_split_branches > 0`. A zero branch count is valid when
basis specialization forces every tower variable before the DFS reaches a
root split. That counter is diagnostic, not a requirement for basis descent.

Source commit `12663f43ee3f03169e54683cb4998b89474c86ab` removes only
that assertion from the public-path test. The test still requires visited
nodes, a terminal leaf, one verified tuple, a group-law match, and zero grid
deferral. The source commit also contains upstream PKM engine work merged
after the previous `4d5019bba92f075e2abdc961b1a86bcd07738cdf` pin.
Consequently, **all** candidate, control, and rho arms in this panel must be
rebuilt and measured at the new source pin. Do not pool timings or gate
receipts across the two source pins.

The previous native gate store contains a prelaunch abort caused by a busy
host and no test verdict. Keep it as a historical receipt. Use a fresh
append-only gate store for the new source. Re-run all three required witness
tests under the 240-second and 2-GiB caps, then independently replay before
promotion. The hosted CI run is a correctness check and does not replace the
resource-capped native gate or a cold same-target cost comparison.
