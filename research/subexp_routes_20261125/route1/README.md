# Route 1 — T-independent solver for the PKM m=2 tower membership system

Prototype + benchmark. **Read `ROUTE1_MEMO.md` first**; it has the tables,
the claim boundary, and the next step.

Nothing here writes to the git repo. Public/toy instances only
(`p ∈ {12289, 40961, 65537}`, `T ≤ 8`). No keys, no wallets, no network.

## Environment

Python 3.10+ only (tested on 3.12.8). No Sage, no third-party packages.
Optionally exercised with Sage 10.9's Python; the script imports only
`json`, `random`, `sys`, `time`.

## Run

```bash
cd /Volumes/SSD990/llm/tmp/opencode/subexp_routes/route1

python3 route1_tower_solver.py            # ~4 s: T=5..8, join tables, 2 extra moduli
python3 route1_tower_solver.py --quick    # ~0.6 s: T=5,6 only
```

Writes `route1_results.json` next to the script.

## What it does

1. `self_test` (always runs first, before any timing):
   - closed-form quadratic `A,B,C` of `S_3(X,x2,Y)` vs a direct `S_3` evaluation;
   - generator walk of `μ_{2^t}` vs literal square-root back-substitution;
   - tree solver vs the naive `V × V` grid, for all of
     {plain, batched} × {hash, chain} membership.
2. `benchmark`: per-system op counts and wall time for T = 5,6,7,8.
3. `join_cost`: full pair table, coverage of `F_p`, cost/relation.
4. prints the comparison against the programme's measured F4 numbers.

## Key result

Per-system cost `c · 2^T` with `c = 127` **flat in T** (121/127/127/127 at
T=5/6/7/8), vs F4's 127.4 M / 2.003 G / 79.75 G. Growth per level of T falls
from 15.72×/39.82× to 2.11×/1.99×/2.00×. Total relation collection is still
`Θ(p^1.0)` — the join, not the solver, is now the binding term.
