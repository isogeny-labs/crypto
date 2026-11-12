# Chained-transfer program: proposed next steps (2026-11-12)

This is a proposal for owner review. No experiment here has run. It ranks the
open leads by how much each could change a conclusion, against what each
costs. Every figure cited is `[pcc:unverified]` and comes from the merged
notes named in the right-hand column.

## Where the program stands

| Question | Answer so far | Source |
|---|---|---|
| Does a cover, isogeny or Jacobian chain carry the log somewhere cheaper? | No. Pull-back keeps the subgroup inside `c^*(E)`. All genus-2 IC relations on it are fibres `P + ιP`, so the IC is a birthday search on `E`. | `cover_pushforward_20261112/PUSHFORWARD.md` |
| Does going up the genus tower help? | No. IC3/IC2 ≈ 3 up to p = 64,007, and smoothness follows `1/g!`. | `tower_large/TOWER_LARGE.md` |
| Does a one-time trapdoor or precomputation help? | Online IC costs 2–3% of plain rho, but 1.3–11% more than memory-matched BL rho. The factor base is at least as large as the group. | `amortized/AMORTIZED.md` |
| Does a small `D_E` with an adapted factor base help? | No. Solving work per trial is unchanged; the saving is folding, which a generic control matches. | `small_de_endo_fb_20261112/RESULTS.md` |
| How common is a low-degree endomorphism? | About `4/√p` of random curves at class level, and about `1/p` on the curve itself. Among standard curves only `j = 0` and Bandersnatch qualify. | `endo_degree_census_20261112/CENSUS.md` |
| Cross-characteristic transfer? | None constructed. The only known route lifts to characteristic 0, with a proven success bound of about `C₀/p`. | `ATLAS.md`; `cryptanalysis:research/xedni_cross_field_20261101` |

## Proposed steps, ranked

### 1. Positive control: show the IC instrument can detect a win (do first)

Every row so far is a loss, and nothing yet shows the instrument could report
a win. Genus-3 IC on a **full** Jacobian `J(F_p)`, of order about `p³`, should
cost `Õ(p^{4/3})` with double large primes against rho's `p^{3/2}`. In the literature,
this is the regime where IC costs less than rho.

- **Arms:** genus-3 IC on the whole Jacobian (`hyper64.rs`, `p` to about
  2^16) against matched rho with the negation map.
- **Falsification:** if IC never drops below rho by the largest affordable
  `p`, the instrument is too weak, and every earlier "distance to rho"
  overstates the gap.
- **Cost:** about one agent-day, local, exact counts.
- **Why first:** it calibrates all earlier verdicts. It is not a result about
  the transfer.

### 2. Instrument fixes that affect every recorded gap

- **Sparse LA.** Use structured Gaussian elimination plus Wiedemann, with a
  counter of real operations. The genus-2 counter is currently
  `(fb+2) × rows`, a matrix-size count. The genus-3 LA reaches 2.2·10⁹
  mulmods at p = 64,007.
- **Double-large-prime variation** for g = 2 and g = 3. Without it the IC
  exponent is not the best known.
- **Matched rho.** Use walk steps plus amortized set-up, with the negation map
  everywhere, plus BL with precomputation where IC amortizes. Rho on `E` is
  mostly set-up at these sizes (slope 0.21 against 0.56 for walk steps).
- Then rerun `tower_large` and `small_de` with the fixed instrument and record
  the corrected distances.
- **Cost:** two to three agent-days.

### 3. Cross-characteristic: run the lift measurement

`cryptanalysis:research/xedni_lift_optimizer_20261101` already has
certifying LLL plus Babai lift paths. Its frozen p = 67 comparison never ran,
blocked by the mandatory S3 checkpoint upload (same blocker as this
container).

- **Proposal:** run it, then extend it to a sweep over toy primes. Measure
  lift success per trial against the `C₀/p` bound.
- **Prediction:** success tracks `≈ C₀/p`, with no family beating it.
- **Falsification:** a family of curves whose lift success stays bounded away
  from `1/p` would be the first cross-characteristic lead.
- **Blocker:** checkpoint credentials (a presigned upload token or an AWS
  identity), or an owner waiver of the checkpoint rule for a toy-size run.

### 4. Generalize the fibre degeneracy to degree-`d` covers (cheap)

The push-forward test covered degree 2 only. For a degree-`d` cover the
pulled-back divisors are full fibres `c^*(Q)` of degree `d`.

- **Prediction:** every weight-`d` relation on the transferred subgroup is a
  fibre, so the IC is again a search on `E`.
- **Test:** `tower_ladder` already builds a degree-3 cover to genus 4.
  Reuse `pushforward.rs` on it.
- **Falsification:** any non-fibre relation on the subgroup.
- **Cost:** under an agent-day.

### 5. Proof note, and a Lean statement where it fits

State the obstruction once, with each step tied to the code that measured it:

- `J_C ~ E × P` and `c_* c^* = [deg c]`.
- The target subgroup lies in `c^*(E)`.
- Effective divisors of degree `deg c` in `c^*(E)` are fibres, so a
  decomposition over degree-1 places of `C` gives no relation beyond `E`.
- Isogenies, pairings and Weil restriction keep the characteristic and the
  group order.

Mark what is not proven: that no efficient cross-characteristic map exists is
open. The `c_*c^* = [deg]` step is a candidate for crypto-lean.

### 6. Validation gate and an independent replay

Admit v3, tower_large, cover_pushforward and small_de through the gate:
replay certificates, a validator report and `VALIDATION.json`. Have a validator
agent that did not produce the data replay all four. Until then, no row
reaches the leaderboard.

### Deprioritized

- **More cover families or a wider genus sweep.** Step 4's prediction covers
  them, and the measured mechanism leaves nothing to tune.
- **Small `D_E` at larger sizes.** The saving is folding, which any curve gets.
- **Weil descent.** A prime field has no proper subfield to descend to.

## Recommended order

Steps 1 and 4 can run in parallel now (local, about a day). Step 2 follows,
and step 1 tells whether it is urgent. Step 3 needs a credential or waiver
decision from the owner. Steps 5 and 6 close out the program.
