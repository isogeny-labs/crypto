# Chained same-characteristic transfers do not cheapen a prime-field ECDLP

- **Status:** `open` (derived + measured boundary for the isogeny → bielliptic [pcc:unverified]
  cover → genus-2 Jacobian route)
- **Kind:** proved/empirical obstruction (relation-count lower bound of
  genus-2 index calculus), plus implementation gap (no cross-characteristic
  map)
- **Affected work:** PR #3877; `research/chained_prime_transfer_20261111/`;
  `src/cryptanalysis/chained_transfer/`; the transfer survey closures
- **Next action owner:** unassigned
- **Last updated:** 2026-11-11

## Problem

We cannot cheapen the discrete logarithm on a prime-order subgroup of [pcc:unverified]
`E/F_p` by transporting it, through an explicit chain of same-characteristic
algebraic maps, to a genus-2 Jacobian `J(C)`: every destination reached by the
verified chain `E → E' → J(C)` (a Vélu isogeny then a bielliptic pull-back
`φ^*`) solves the DLP at a cost at or above solving it directly on `E`.

Instance: `E1/F_p` a quotient of a bielliptic `C : y² = F(x²)`, prime-order [pcc:unverified]
subgroup `r`, `Q = [x]P`; the chain is built and each hop verified to satisfy
`h([x]P) = [x]h(P)`, `ord(h(P)) = r`, `h(P) ≠ 0`.

## Evidence and uncertainty

- **Observed** (`chained_transfer_bench`, counted group operations,
  `[pcc:unverified]`): rho on the isogenous curve `E1'` matches rho on `E1`
  (same `#E`); rho on the order-`r` subgroup of `J(C)` matches `E1` rho in
  group-op count (same `√r` walk) and is dearer in field operations (a Cantor
  step costs several elliptic additions); genus-2 index calculus on `J(C)`
  with the full degree-1 factor base collects about `p` relations, its measured
  relation-phase group-op count growing like `p` while rho grows like `√p`.
  See `research/chained_prime_transfer_20261111/RESULTS.md`.
- **Derived:** for a degree-2 morphism `φ : C → E`, `φ_*φ^* = [deg φ] = [2]`,
  so `φ^*` preserves the odd-order DLP; the second quotient gives
  `φ2_* φ1^* = 0` (a vanishing cross-composite). Both are verified in code on
  the unramified locus and on random classes. The usual small-genus
  index-calculus estimate is `Õ(p^{2-2/g})`, exponent 1 at `g = 2`. That is a
  limit of this approach, not a lower bound on the problem: rho on the
  original `r ≍ p` subgroup already costs about `√p`.
- **Suspected explanation:** the chain is a group isomorphism onto its image,
  so it transports the hardness unchanged; the only structure it exposes
  (a second elliptic quotient) communicates with the first through a map that
  is provably zero, and the higher-genus destination raises, not lowers, the
  index-calculus exponent.
- **Missing capability:** no efficiently computable cross-characteristic map
  (prime → binary) is supplied; the one explored elsewhere is bounded below by
  a global lift of degree `≥ m` (`cryptanalysis:research/xedni_cross_field`).
- **What this evidence does not establish:** the operation counts are exact
  and replayable but are a local development check, not an isolated timing
  measurement; no claim about curves beyond the tested bielliptic family, and
  no impossibility theorem for all algorithms.

## What would unblock it

A transfer whose destination has a *lower* DLP exponent than `√p` and whose
every map is efficiently computable on encoded points without their logs.

- [ ] A same-characteristic destination with index-calculus (or other) cost [pcc:unverified]
  below rho's `√p` on the same instances, verified end to end with the full
  cost accounting and a growing-size check.
- [ ] Or a computable cross-characteristic subgroup homomorphism on arbitrary
  encoded points, with a construction cost below the direct solve.

Meeting either would be a cryptanalytic improvement; failing to, the boundary [pcc:unverified]
stands. Neither is implied by the existence of the intermediate maps.

## Next discriminating step

- **Action:** extend the measurement to a split-Jacobian census — which
  complement curve `E'` appears in `Jac(C) ~ E1 × E2`, and whether any
  bielliptic family yields `E2` with a lower-cost representation than `E1`
  (it cannot lower the exponent, but the constant is worth measuring).
- **Inputs and controls:** fixed bielliptic curves, matched rho per arm,
  counted group operations.
- **Outcomes:** a measured constant-factor gap that never crosses rho
  confirms the boundary; a crossing would reopen the lead with full
  accounting.
- **Recheck trigger:** a new computable map that changes the destination's
  genus or characteristic.

## Transfer atlas (2026-11-11 addendum)

Covering a *fixed* target curve and sweeping the cover maps the discrete log to
a rich spectrum of different-order complementary curves, but the log follows to
none (`research/chained_prime_transfer_20261111/ATLAS.md`) [pcc:unverified]:

- Sweep workflow: 5,641 covers; the complement takes up to 141 distinct orders
  across the Hasse interval; 3,479 complements are smoother than the target (smaller largest
  exactly-once prime);
  the cover map `φ2_*φ1^* = 0` wherever sampled (first 40 covers ×
  16 samples per report, one direction; replay F9); no reachable complement is
  strictly easier (reachable ⇒ same order ⇒ identical hardness).
- Census: `φ2_*φ1^* = 0` in both directions on 300/300 curves per prime. The
  "no reachable easier node" counters are zero by construction. They restate
  the theorem below and are not independent evidence (replay F8).
- Positive control: Smart's additive transfer recovers the log on a genuinely
  anomalous curve over `F_43`, so the easy node is real — just unreachable from
  a non-anomalous start (an isogeny preserves the trace; no edge changes the
  characteristic, so no binary node is reachable either). Isogeny transfer still
  works over prime fields; what is missing is a smaller base field. A weak
  prime-field family discovered later could still be a transfer target.

The load-bearing theorem: a DLP-preserving `E → E'` exists iff `E ~ E'`
(isogenous) iff `#E = #E'`, so a map to a different-order curve cannot preserve
the log. This strengthens, not resolves, the obstruction: the reachable set is
confined to `E`'s isogeny class.

## Tower up (2026-11-11 addendum)

Carrying the same elliptic log *up* the cover tower makes the destination
harder (`research/chained_prime_transfer_20261111/tower/TOWER.md`)
[pcc:unverified]. One target E/F_p is covered by a genus-2 and a genus-3
curve (`C : y² = F(x²)`), and the pull-back is verified by homomorphism
checks. On the same instance, counted index calculus on genus 3 costs 2.3–3.4×
index calculus on genus 2 at every p in 53–251 (about the `1/g!` smoothness
ratio). Genus 3 is above rho at every §5a price. Genus 2 cannot be ranked
against rho at 0 GAE, and is above rho at 0.1 and 1 GAE. The IC slope in p is
0.85 (genus 2) and 1.06 (genus 3), against 0.44 for rho. Up, across and down
are now each measured or proved closed.

## Ladder, formula, smoothness and replay (2026-11-12 addendum)

Four parallel experiments under
`research/chained_prime_transfer_20261111/{tower_ladder,tower_formula,tower_smoothness,replay}`
[pcc:unverified]:

- Ladder (genus 1 → 2 → 3 → 4 on one instance, genus 4 by a degree-3 cover
  `y² = c(h(x))`): IC per rung grows by about 2×, 3× and 4×. Genus 4 costs
  3.7–19.6× rho at 0 GAE. No rung ranks below rho across the band.
- Formula scan to 2^128: the best genus is g = 2, still about 64 bits above
  rho. No subexponential regime beats rho at any size scanned
  [pcc:unverified].
- Smoothness: measured trials per relation are 1.00, 1.94, 5.91 and 22.2 for
  genus 1–4, against `g!` = 1, 2, 6, 24.
- Independent replay: every raw file is byte-identical. One figure was wrong:
  the cover-spectrum note said 470/504 smoother, and the correct count is 87/504
  (fixed). The replay also found that the IC counter overcounted by 8–13%,
  that rho was not checked against the planted log, and that LA work was not
  emitted. These are fixed in code and rerun as `results_v2.json`. The band
  is one-sided, because rho has no unpriced counters.

## Resolution and history

- 2026-11-11: opened from `research/chained_prime_transfer_20261111`
  (native Rust framework, PR #3877 follow-up).
- 2026-11-11: transfer-atlas addendum — cover sweep, census and down-edge
  controls; the obstruction holds across every arm measured.
- 2026-11-11: tower-up addendum (genus 1 → 2 → 3 on one instance); the ladder raises IC cost per rung.
- 2026-11-12: ladder/formula/smoothness/replay addendum; replay corrections
  applied (F1–F3, F6, F8–F12) and the bench rerun as `results_v2.json`.
