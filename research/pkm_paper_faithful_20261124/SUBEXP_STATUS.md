# Subexponential IC over prime fields: where the exponent actually sits

Date: 2026-11-25. Goal resumed: subexponential index calculus for the ECDLP on
ordinary prime-field curves. Driver `examples/pkm_mixed_ecdlp.rs`; rows in
`results/p3907_*.json`, `p18_b11_full.json`, `p18_b2_rate.json`. All
`[pcc:unverified]`.

## The reduction

PKM's Algorithm 1 costs

    (#relations) × (cost per system) = p^{1/m} × T.

If the per-system solve grows as a power of `p`, `T = p^{γ}`, the whole method
is `p^{1/m + γ}`: rho is `p^{1/2}`, so beating rho needs `1/m + γ < 1/2`, and
subexponential needs `γ → 0`. Everything therefore rests on `γ`.

The density condition fixes the tower: `deg L = p^{1/m}`, so a chain of
`B`-smooth steps has `n0 = log(p^{1/m})/log B` levels and `m·n0 = log p/log B`
tower variables. `γ` is the per-level cost exponent, `γ = log r / log B` with
`r` the cost ratio from one extra level. It is measurable: hold the
construction and vary `p`.

## Measured

Per-target cost at the density condition (`m = 2`, `deg L ≈ p^{1/2}`), two
fields per `B`:

| B | p = 3907 (2^12) | p = 261847 (2^18) | ratio | γ |
|--:|--:|--:|--:|--:|
| 2 | 0.0905 s (74 targets) | 38.1 s (40 targets) | 421× | **1.44** |
| 11 | 0.037 s (60 targets) | 6.47 s (483 targets) | 175× | **1.23** |
| 31 | 0.020 s (5 targets) | 5.98 s (12 targets) | 299× | **1.36** |

`γ = log(cost ratio)/log(p ratio)`, `p` ratio 67.

Both chains verified the planted scalar at both fields (`k = 2683` and
`k = 203179`), so these are costs of runs that work, not of failed ones.

## What that means

* **Total exponent at `m = 2` is `1/2 + γ` = 1.94 (`B = 2`) and 1.73
  (`B = 11`)** — worse than rho (`0.5`) by `p^{1.2}`–`p^{1.4}`, and worse than
  a generic `p^1` exhaustive search.
* **Raising `m` does not rescue it.** The `1/m` term vanishes as `m` grows, and
  the exponent tends to `γ`, still 1.2–1.4. This is the same statement as the
  `m`-ladder's variable-count wall (`m·n0 = log p`), seen in cost.
* **`B` does not move `γ` down.** 1.44 (`B = 2`), 1.23 (`B = 11`), 1.36
  (`B = 31`): flat within the error, not the ~0.02-per-doubling decline the
  first two points suggested. Reaching `γ < 1/2` at this trend would need a
  change of regime, not a bigger `B` — and the `B = 31` chain already needs
  degree-31 equations, where each Vélu step is a degree-31 rational map.
* The `B = 31` pair is a warning about the model: both fields use `n0 = 2`
  levels and the same four variables, yet cost 0.02 s and 5.98 s. Whatever
  drives the second number is not captured by `(n0, B)`, so `γ` from two
  points has wide bars and this table is a screening instrument, not a
  measurement of a law.

So the subexponential goal is not "hard" at this construction; on the measured
evidence it is not reachable through it. What would change the verdict is a
solver whose cost per level grows slower than `B^{1/2}` — the chain system is
the gate, exactly as in `CHAIN_DIAGNOSTIC.md`, now with a number attached to
how much it has to improve.

## Caveats

* Two field sizes, one auxiliary curve each, one planted scalar each; the
  `p = 2^12` figures are single runs on a shared host (wall clocks swing up to
  8×, see `B_END_TO_END.md`), so `γ` is good to roughly ±0.15, not ±0.01.
* `n0` differs between the two fields (6 → 9 at `B = 2`, 2 → 4 at `B = 11`),
  which is the point of the density condition, but it means the ratio estimates
  `γ` for the level-growth law, not for a fixed system.
* `T` includes the oracle's exact grid fallback when F4 does not decide; the
  `B = 2` column leans on it more (15 % of targets at `p = 2^18`).

## Next measurement

`B = 31` was the proposed next point and it came back flat (1.36). The
remaining useful probe is the same pair at `B = 61` and `127`; if those also
sit at 1.2–1.4, the construction is closed for this goal on measured evidence,
and the work belongs on the solver itself (cost per level below `B^{1/2}`)
rather than on the map family.
