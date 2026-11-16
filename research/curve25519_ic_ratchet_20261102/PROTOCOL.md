# Curve25519-family IC ratchet round 1 — protocol

Frozen 2026-11-02 before any data in this directory. The incumbent is the
baseline of [`../curve25519_ic_baseline_20261102`](../curve25519_ic_baseline_20261102/README.md):
`prime-projected-abscissa:size=N_a,scan_cap=16N_a` + `mitm:negation_folded=1`,
with `N_a = 32, 81, 203, 512` at k = 20, 24, 28, 32. Curves are the same
miniatures. Every target seed below is new.

The owner asked for four leads (2026-11-02): (1) retune N, (2) the closed
cofactor fiber for prime curves, (3) a base closed under x ↦ 1/x, and (4) the
algebraic (summation-polynomial) track. This protocol covers all four. Each
part is preregistered before its data is taken.

## Part 1 — base-size retune (generic track, ecbench, end-to-end S)

- **Train.** One session per k with the projected base at
  `N ∈ round({0.35, 0.5, 0.7, 1.0}·N_a)`, plus rho-neg: 16 targets × 1
  round, target seed `2026110400 + k`. `N*` = the lowest mean S at w = 0.
- **Holdout.** One session per k with `inc` (N_a), `sel` (N*), `inc-aa`,
  `rho-neg` and `rho-neg-aa`: 32 targets × 2 rounds, target seed
  `2026110500 + k`. If `N* = N_a`, the holdout still runs and records a null.
- **Decision.** `sel` is an improvement at size k if the paired
  `ecbench compare` ratio sel/inc has its 95% interval below 1 at w = 0
  **and** `ecbench band` ranks sel < inc across w ∈ {0, 0.1, 1}. Otherwise
  the result is relabelling (better at w = 0 only) or null. Prediction: small
  gains (≤ 15%) with N* ∈ {0.5, 0.7}·N_a, since the incumbent's table and
  relation phases are already about balanced (49% / 47% of GAE at k = 32).

## Part 2 — closed cofactor fiber on prime curves (mechanism, exact counts)

The binary-curve `closed` fiber mode stores `P_i ± P_j + K` for all
`K ∈ E[8]`, so one probe covers all 8 cosets. On the blind small-x base this
recovers the full cofactor. **Stated before measuring:** it covers the same
targets per column as the projected base (`S ↦ [8]S` is a bijection from
coset representatives to G), but its table is 8× larger. So against the
incumbent it is predicted to lose: equal yield, 8× table build. Measured with
`curve25519_family fiber` (exact `D` for the blind, closed-fiber and projected
tables at N = N_a, and table entries for each).

- **F1.** `D_fiber / D_proj` ∈ [0.95, 1.05].
- **F2.** `entries_fiber / entries_proj` ∈ [7.5, 8.5].

If both hold, the closed fiber is dominated by projection in the generic
track and gets no ecbench arm.

## Part 3 — base closed under x ↦ 1/x

Already measured for the generic track in
[`../curve25519_t0_symmetry_20261102`](../curve25519_t0_symmetry_20261102/README.md):
2.0× over blind, 4× below projection. In the generic track a projected base
gains nothing from closure, because `[8](P + T₀) = [8]P`. Part 3 therefore
runs inside Part 4, where a small-x base is required and cannot be projected.

## Part 4 — algebraic track

The lab's prime-field algebraic oracle is `pkm-semaev` on a `prime-kummer:t=T`
base (Petit–Kosters–Messeng tower, S_(m+1) solved by F4). Its arms and sizes
are fixed in an amendment written after a feasibility check: one smoke target,
with no timing used for any decision. The comparison is pkm-semaev against
the generic incumbent at equal k, end-to-end S, and the §5a band, on fresh
targets `2026110600 + k`. The T₀-closed variant is in scope only if the
pkm-semaev base admits closure under x ↦ 1/x without changing the solver; if
not, it is named as an obstruction with what would unblock it.
