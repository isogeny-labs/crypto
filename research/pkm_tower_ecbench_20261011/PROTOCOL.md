# F4, F5 and an F6 analogue on prime-field curves: protocol

Status: **preregistered**. This file, the four specs and the code were
committed before any measured run. The smoke runs that tested the plumbing
are disclosed below.

## Question

The binary round, `research/f4_f5_f6_cross_family_20261011/`, compared the
repository's Gröbner decomposers on Koblitz and generic binary curves. It
could say nothing about prime-field curves: `ecbench` had no algebraic
decomposition oracle for them, and F6-IC's gate encodes a factor base that
is an `F_2`-subspace, which `F_p` does not have.

This round builds that oracle and measures, in `ecbench`'s counted unit
and against same-target rho, how the following compare on prime-order
curves over `F_p`:

- tower F4;
- signature F5;
- an F6 analogue;
- a no-algebra control.

## What was built

All of it is in the `ecbench` binary's new plug-in,
`src/bin/ecbench/pkm_tower.rs`, which reaches `ic.pipeline` through a new
library hook, `methods::register_prime_plugins`. The hook mirrors the
binary-curve one.

- **Base: `prime-tower:t=T,seed=S`.** A Petit–Kosters–Messeng Kummer tower
  `V = { g·ζ^k : k < 2^T }`, with `ζ` of exact order `2^T` (so `2^T | p − 1`).
  Every `x ∈ V` that carries a point contributes both points above it,
  folded by negation into one column.
- **Oracle: `pkm-tower:t=T,seed=S,engine=E`.**
  - It finds two-point decompositions (`m = 2`).
  - The system is `S₃(g·y_{1,0}, g·y_{2,0}, x_R)` in the tower ring
    `F_p[y_1, y_2]/(y_{j+1} = y_j², y_{T−1}² = 1)`, so `N = 2T` unknowns.
  - The search is a splitting tree over block 1's tower levels:
    - a node fixes one level by a linear constraint, and the levels above
      it follow from the rules;
    - its two children are the two square roots;
    - an engine that proves a node's system inconsistent prunes it;
    - a leaf fixes `x_1`. `S₃` is then a quadratic in `x_2`, its roots are
      looked up in the base, and the signs are lifted by group arithmetic.
- **Engines.** All four run on the same tree:
  - `f4`: `f4_fp_tower` at each node.
  - `sig-f5`: `sig_fp_tower` at each node, signature F4 with the F5/GVW
    criteria under position-over-term. The node's constraint is listed
    first, because under that order a later generator is not reached
    until the first one's ideal is finished (smoke run 2).
  - `f6`: as `f4`, but a node whose remaining candidate set for `x_1` has
    at most `cap = 4` elements is closed by geometry. For each candidate
    point `P_1` the oracle computes `R ∓ P_1` (two charged additions) and
    looks the abscissae up in the base. This is F6-IC's idea, exact
    factor-base geometry closing a branch of the algebraic search, with
    no subspace encoder.
  - `enum`: the root is closed by geometry. There is no algebra, and it is
    the control.
- **Degree cap.** 20 for both tower engines. F4 never exceeded degree 5
  in the smoke runs; signature F5 needs up to 14.
- **Tests.**
  - `tests/ecbench.rs::pkm_tower_runs_every_engine_on_a_prime_curve`: every
    engine verifies, F4 and signature F5 visit identical trees, F6 charges
    geometry, `enum` calls no solver, and `verify --replay-all` passes.
  - A unit test of the tower construction.

## Curves (registered in `docs/curves/registry.json` by this change)

Prime order (cofactor 1), from `find_prime_order_curve(bits, seed)`. Seeds
were chosen by `examples/pkm_tower_curve_seeds.rs` as the first with
`2^t | p − 1`.

| cell | bits | seed | p | r | t (abscissae 2^t) |
| --- | --: | --: | --: | --: | --: |
| P5 | 9 | 17 | 257 | 233 | 5 (32) |
| P6 | 11 | 20 | 1,409 | 1,367 | 6 (64) |
| P7 | 13 | 35 | 7,681 | 7,723 | 7 (128) |
| P8 | 15 | 115 | 18,433 | 18,253 | 8 (256) |

ICV1 slugs: `icv1-fp9-t25-33fa1ca2`, `icv1-fp11-t43-4d73e7f7`,
`icv1-fp13-tm41-756f569a`, `icv1-fp15-t181-50ff72d2`.

`t` grows with `r` so that a random target decomposes with constant
probability (`|V|²/2 ≈ r`). The tower system therefore grows with the
curve: `N = 2t` = 10, 12, 14, 16.

## Specs

- `SPEC-P5`, `SPEC-P6`, `SPEC-P7`: 8 public one-target workloads each.
- `SPEC-P8`: 4 workloads, because a smoke run put t = 7 at 35–48 s per
  tower run and t = 8 is expected to cost several times that.
- Shared settings:
  - target seed 202610112301, measurement seed 202610112302, one round;
  - arms `rho-negation` (reference), `f4` (baseline), `sig-f5`, `f6`
    (`cap=4`) and `enum`;
  - `max_trials` 200000, a 3,600 s cap per run, and isolation L1 requested.

## Units

- **`W`.** Counted `F_p` multiply-adds of the tower engines per run
  (`solver.ops`; deterministic).
- **`S`.** Charged group-addition equivalents over `√r`.
  - For the tower arms `S` is a **lower bound**: their multiply-adds are
    counted but unpriced. `ecbench` has no pinned price for that unit.
  - Geometric closure additions and sign-lifting additions are charged.
  - `enum` runs no solver.
- **Wall time.** Relation-phase wall time is a practicality note only.
- **Floor.** `√(π/4)` (negation only; these curves have no other
  automorphism used here).

## Questions and decision rules (per cell; medians and ranges over targets)

1. **F5 against F4.**
   - Statistic: `W(sig-f5) / W(f4)`.
   - **"F5 cuts F4's work"** if the ratio is below 1 on every target and
     both engines visit the same tree on every target (equal nodes,
     refutations, leaves, trials and relations).
   - **"F5 costs more"** if the ratio is above 1 on every target.
   - Otherwise **"mixed"**.
2. **F6 analogue against F4.**
   - Statistic: `log₂(W(f4) / W(f6))`, with the thresholds of the binary
     round:
     - **"F6 saves solver work"** if the median is at least 0.1 and every
       target is above 0;
     - **"no saving"** if the median is within ±0.05;
     - otherwise **"mixed"**.
   - Also reported: the geometric additions F6 charges, and `S(f6)/S(f4)`.
3. **Each arm against rho.**
   - Statistic: `S(arm) / S(rho-negation)` on the same target.
   - **"Slower than rho"** if above 1 on every target.
   - A tower arm's lower bound below rho proves nothing, because its
     multiply-adds are unpriced.
   - `enum` charges all of its work, so its ratio is not a lower bound
     unless the record says so.
4. **Algebra against geometry.** `S(f4)/S(enum)` and the relation-phase
   wall ratio, reported as is with no rule. A tower arm whose lower bound
   is below `enum`'s is not cheaper, because its multiply-adds are
   unpriced. The report prices them as a derived figure at 1/63 GAE each,
   the `F_p`-multiplication-per-affine-addition ratio measured in
   `RESEARCH_RESIDUAL_WALKS.md` §11. That figure is labelled derived and
   decides nothing.
5. **Size.** Per arm, the median `S` and `W` at each `t`. A descriptive
   least-squares slope of `log₂ S` against `log₂ r` over the four cells is
   reported for each arm and for rho. With four small sizes it is no
   exponent claim.

**Stop conditions.**
- A run that fails verification fails its cell; the cell enters no ratio.
- A cell with more than half its targets timed out gets no median.

## Smoke runs (before this file; plumbing only, not in any decision)

1. **`prime_search` 11 / 20, t = 6, one target (target seed 1).**
   - All five arms verified.
   - Before the input-order fix, `sig-f5` refuted no node: 1,011 nodes
     against F4's 175. A unit comparison showed that the constraint,
     listed second, never entered its basis below degree 14.
2. **The same target after the fix, degree cap 20.**
   - F4 and signature F5 visited identical trees: 175 nodes, 43 refuted,
     34 leaves.
   - F4: 2.54 × 10⁹ multiply-adds. Signature F5: 0.69 × 10⁹.
   - F6 (`cap=4`): 2.53 × 10⁹ multiply-adds, 111 nodes, 128 geometric
     additions.
   - `enum`: 0 multiply-adds, 679 additions.
   - `S`: rho 4.25, F4 and signature F5 16.4, F6 16.9, `enum` 31.6.
   - Relation-phase wall: F4 4.7 s, signature F5 4.5 s, F6 4.3 s, `enum`
     0.13 ms.
3. **`prime_search` 13 / 35, t = 7, one target.**
   - F4: 3.69 × 10¹⁰ multiply-adds, 35 s.
   - Signature F5: 1.11 × 10¹⁰, 48 s.
   - Both visited the same tree, 330 nodes.
   - `enum`: 0.27 ms. All verified.

## Commands

```sh
cargo build --release --bin ecbench
for s in P5 P6 P7 P8; do
  target/release/ecbench run --spec research/pkm_tower_ecbench_20261011/SPEC-$s.json \
    --out research/pkm_tower_ecbench_20261011/sessions/$s --wait
  target/release/ecbench verify --dir research/pkm_tower_ecbench_20261011/sessions/$s \
    --replay-all --out research/pkm_tower_ecbench_20261011/audits/$s-replay-all.json
done
```
