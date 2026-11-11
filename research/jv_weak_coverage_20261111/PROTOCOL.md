# Protocol: how often a curve over F_{p⁶} lands in a Joux–Vitse weak class

Frozen before any run of this study. Smoke runs (p = 1009, 6 curves; a cap sweep on one curve) in the scratchpad
are disclosed in the README and are not data. Scope: the cubic norm-one branch that `jv_cover`'s route uses
(weak test `weak_by_norms`). The quadratic-h branch of the published family (ledger §18.7) is out of scope.

## Measurements

1. **U: uniform curves.** y² = x³ + ax + b over F_{p⁶}, with (a, b) uniform and nonsingular. 2,000 per prime,
   p ∈ {37, 53, 1009, 1511, 1823}. PARI/GP `ellcard` gives the trace t. Recorded:
   - even order;
   - the class passes the proved necessary condition `t ≡ ±(p⁶+1) (mod 16)` (ISO-1 theorem, ledger §18.6;
     `trace_may_hold_weak_curve`), i.e. it is **admitted**.
   Script `uniform.gp`. Seed: GP `setrand(20261111 + p)`.
2. **F: curves with full 2-torsion.** `examples/jv_weak_coverage` (`random_curve`, baby-step giant-step order),
   400 per prime at the same five primes, seed 20261111. Recorded: admitted, as in U.
3. **W: operational reach of the implemented walk.** For every admitted F curve, `walk_to_weak_record` (the
   end-to-end route's walk: 2-isogenies within the full-2-torsion locus, odd jumps 3, 5, 7), cap 100,000 curves.
   Recorded: whether a weak curve is reached.
   - At p = 37 and 53 the truth is known from the exact censuses (ledger §18.6: 98.8–99.4 % of admitted classes
     hold a weak curve). There W measures the walk's recall.
   - At p ≥ 1009 a reached weak curve is a witness. A walk that closes or caps leaves the class unresolved,
     not empty.

## Estimands (fixed now)

- **Coverage upper bound:** P_U(admitted), and P_F(admitted), each with a Wilson 95 % interval.
- **Operational coverage of the implemented route:** P_U(admitted) × P_W(reached | admitted, F). A uniform curve
  whose class holds full-2-torsion curves can be moved onto one by a 2-isogeny path; that step is not run here.
- **Recall:** P_W(reached | admitted) at p = 37 and 53, against the census truth.

## Predictions (before the runs)

- P_U(admitted) is about 1/8 at every prime (2-adic model), with confidence 0.5.
- P_F(admitted) is in [0.55, 0.70] at every prime (ledger: 63 %), with confidence 0.8.
- P_W(reached | admitted) at p ≥ 1009 is below 0.1, with confidence 0.7. The smoke runs closed after fewer
  than 100 curves, because odd jumps exist only where an ℓ-torsion point is rational.

## Consequence for the walk charge (`research/jv_walk_charge_20261111`)

That study's starts were built from a weak curve by the same restricted moves, so they lie in the walk's reachable
set by construction. If W at p ≥ 1009 is low, the walk charge holds only for starts reachable by this walk, and
the README of that study must say so.

## Amendment 1 (2026-11-11, after 8, 6 and 167 F/W records at p = 1009, 53 and 37 existed)

Walking every admitted curve with a 100,000-curve cap would take hours per prime: single walks took up to 195 s.
**W now walks only the first 60 admitted curves of each prime (`--walk-limit 60`).** F, admission over all 400
curves, is unchanged. The seeds are deterministic, so the walked curves are the first 60 admitted ones of the same
sequence. The interrupted first attempt is kept in `runs/attempt1/`.

Disclosed before this amendment: five walks at p = 1009 had been seen, all unresolved, with 6.9·10⁷ to
3.1·10⁹ multiplications. The decision rules and estimands are otherwise unchanged.
