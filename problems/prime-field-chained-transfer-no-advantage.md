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

## Amortized IC, certificates and larger-p genus 3 (2026-11-12 addendum)

See `research/chained_prime_transfer_20261111/LEADS_20261112.md` [pcc:unverified].

- **Amortized IC.** Factor-base logs are computed once and each target needs
  only a descent. Its online cost is 2.0–3.1% of plain rho, but 1.3–11% above
  Bernstein–Lange rho given the same memory. The frozen rule fails in 11 of
  11 rows. The factor base is at least as large as the target group in every
  row (`fb/r` 0.98–125), so the precomputation is a table the size of the
  group. This answers the trapdoor question for IC's own one-time work:
  amortizing helps only when the factor base is much smaller than the group,
  and an elliptic subgroup of order about `p` pushed into a genus-2 Jacobian
  over `F_p` never gives that.
- **Certificates and the real instance** (F4, F5, F14 closed). Logs of the
  pulled-back pair are recovered with oracle-free LA, and 16 of 16
  certificates pass an independent checker. IC/rho is 8.0–40.8× at 0 GAE.
- **Genus 3 to p = 64,007.** IC3/IC2 stays near 3, smoothness follows `1/g!`,
  and the gap to rho grows about as `√p`.

## Cover mechanism and endomorphism census (2026-11-12 addendum)

- **The cover carries no log onto the target** (`research/cover_pushforward_20261112/PUSHFORWARD.md`) [pcc:unverified].
  - All 37,866 genus-2 IC relations on the transferred subgroup (189,330 over 15 seeds) are fibres `P + ιP = φ1^*(Q)`. Their push-forward is the degenerate `[2]Q = Q + Q`.
  - The reason is that the subgroup lies inside `φ1^*(E1)`, whose weight-2 divisors are exactly these fibres.
  - So genus-2 IC on `J(C)` over this subgroup is a birthday collision search on `X(Q)` in `⟨G_E⟩`. The cover only filters to "X(Q) is a nonzero square", which discards about half of the trials. The same search on `E1` alone accepts identical trials and costs identical group operations at every seed.
  - Through the other quotient no pushed relation involves the target, because `φ2_*φ1^* = 0`.
  - The Prym never enters the computation. A cover that helps must supply an explicit map plus a target-side factor base well below `r` on which independent few-term relations occur at a constant rate.
- **Low-degree endomorphisms occur only by construction** (`research/endo_degree_census_20261112/CENSUS.md`) [pcc:unverified].
  - An endomorphism of degree `≤ 11` needs `|d_K| ≤ 44`. Among random curves that happens with probability about `4/√p`, matching the exact Hurwitz-class-number prediction at 12–28 bits (largest |z| = 1.45).
  - The curve itself sits on the crater with probability about `1/p`; the median conductor is about `0.45√p`.
  - The named standard curves that qualify are all `j = 0` (secp*k1, BN/BLS, Pallas/Vesta), except Bandersnatch (`D = −8`).
  - P-256/384/521, brainpool, Curve25519/448, GOST and SM2 have `|d_K| > 44`.
  - The lab registry already holds constructed curves with `D = −7, −8, −11, −19, −43` at 37–61 bits, so the small-`D_E` experiment needs no new construction. Its results would apply to designed curves, not deployed ones.

## Small D_E with an endomorphism-adapted factor base (2026-11-12 addendum)

`research/small_de_endo_fb_20261112/RESULTS.md`. The protocol was frozen by hash before data [pcc:unverified].

- **Solving work does not change.** The S_3/S_4 solving work per trial is identical in the adapted and standard arms (ratio 1.000) at every `D ∈ {−3, −4, −7, −8, −11}`, `m ∈ {2, 3}` and size from 12 to 28 bits. The decomposition probability is unchanged for `D = −3, −4` and 0.09–0.11 lower for the layered `D = −7, −8` bases.
- **The only saving is folded columns.** Columns fall by exactly `|Aut|/2` or the layer count, and the relation-phase cost falls with them.
- **A generic curve gets the same saving.** Its base was folded by a random scalar in the same 4 layers and saves as much: 0.22–0.30 of its unfolded twin, within 0.1 of the layered `D` arms at 24–28 bits. It even costs less than the `j = 0` and `j = 1728` automorphism arms [pcc:unverified].
- **Verdict.** The frozen rule passes for `D = −3, −4, −7, −8` as written, but the control shows the gain is the fold factor, not `D_E`. Small `D_E` does not reduce the actual solving cost.
- **Distance to rho.** No arm is below its matched rho in 120 arm-cells, including rho with the same automorphism group. The smallest ratio is 7.62× at 0.1 GAE, and the gap grows about like `√p`.

## Degree-3 covers and the IC positive control (2026-11-12 addendum)

- **Degree-3 covers** (`research/cover_pushforward_deg3_20261112/PUSHFORWARD_DEG3.md`) [pcc:unverified].
  - On the genus-4 degree-3 cover, all 28,815 IC relations on the transferred subgroup are single full fibres `c^*(Q)`, and they push forward to `[3]Q`.
  - The `J(C)` arm and the matching collision search on `E` accept identical trials and give identical certified logs in 360 of 360 seed-cells.
  - The fibre degeneracy therefore holds at degree 3 as at degree 2.
  - Limits: `p ≤ 251` (the HCurve bound) and `r` 11–83.
- **Positive control** (`research/ic_positive_control_20261112/RESULTS.md`) [pcc:unverified].
  - Double-large-prime genus-3 IC on a full Jacobian is below rho in group ops from p = 257 on (0.114× at p = 65,537).
  - At 0.1 and 1 GAE it is 5.8× and 31× rho, so the §5a band does not show this known win at lab sizes.
  - The verdicts here are unaffected: every transfer row loses at 0 GAE or reduces to a search on `E`. The general consequence is filed as `problems/section5a-band-cannot-show-a-known-ic-win.md`.

## Proof note (2026-11-12 addendum)

`research/chained_transfer_proof_20261112/PROOF.md` states the obstruction once.
- **Established in the note** (argument given there, unverified by pcc) [pcc:unverified]: `c_*c^* = [d]`, `J_C ~ E × P`, and pull-back injective on `E[r]` for `r ∤ d`.
- **Fibre rigidity (T5).** For hyperelliptic `C`, `d ≤ g` and `c ∘ ι_h = [−1] ∘ c`, every `Q` outside an explicit finite set has `c^*Q` as the reduced representative of `c^*[Q − O]`.
- **Collision equivalence (T7).** Every dependency among the fibre rows is a collision `Q_i = ±Q_j` on `E`. So an IC whose relations lie in the transferred subgroup cannot beat the matched collision search on `E`.
- **`φ2_*φ1^* = 0` (T8).**
- **Open:** a cross-characteristic map, lifting (only the conditional `C₀/p` bound), and an IC that draws relations from all of `J_C`.
- **Correction:** the ATLAS "iff" is now an "only if". Tate gives an isogeny when the orders agree, but not necessarily one of degree prime to `r`.

## Instrument fixes (2026-11-12 addendum)

`research/instrument_fixes_20261112/RESULTS.md`; the protocol and Amendment 1 were frozen before the recorded run [pcc:unverified].
- **Matched rho.** The recorded rho on `E` spent about 400 group ops on jump-table set-up. The negation-map walk with set-up separated is 5–9× cheaper at v3 sizes. So every recorded IC/rho distance was understated.
- **IC baseline.** Walk collection plus double large primes cut IC group ops 13.4× at p = 1619 (34,521 to 2,583). For genus 3 the new arms beat the old IC in total from p ≥ 2003. This is a measured improvement of the index-calculus baseline. Its gap to the matched rho stays 28.1×, 247× and 1,715× at 0, 0.1 and 1 GAE.
- **LA counter.** The genus-2 counter was a matrix-size count, but within 2% of the real multiplication count.
- **Verdicts.** One recorded verdict changes: tower genus 2 at p = 251 goes from "unresolved" to "above" at every price. No arm in v3, tower or small_de is below any matched rho.

## Independent validation (2026-11-13 addendum)

`research/validation_20261112/REPORT.md` [pcc:unverified]:
- **Admitted:** small_de passed the gate, and `research/small_de_endo_fb_20261112/VALIDATION.json` is written.
- **Not admitted:** v3, tower_large and cover_pushforward. Their measurements reproduce in independent code, but their matched rho is plain (negation-free) and its native work is not emitted, so the gate fails on the reference.
- **Next step for those three:** re-measuring them with the negation-map reference from `instrument_rerun` would make them admissible.

## Negation re-measurement (2026-11-13 addendum)

`research/negation_remeasure_20261113/RESULTS.md`; validation evidence in `research/validation_20261113/REPORT.md` [pcc:unverified].
- **Setup.** v3, tower_large and cover_pushforward were re-measured against the negation-map rho in the same process, with rho's native counters emitted.
- **Result.** 0 of 162 arm-rows are below rho. The plain rho total was 4.35–9.22× the negation total, so the recorded gaps were understated at 0 GAE. At priced weights this is not uniform: at the tower g3 headline cell the w = 1 distance shrinks.
- **Verdict changes.** Fifteen cover first-collision rows had been recorded below plain rho; that came from plain rho's set-up and does not hold against the negation rho. The tower g2 p = 251 row moves from unresolved to above.
- **Validation.** Admitted 2026-11-13: `research/negation_remeasure_20261113/VALIDATION.json`, with the commit binding confirmed against `8bf77ba333`.
- **Verdict-change count.** Of the 16 verdict changes, 10 go to "above" and 6 to "unresolved (lower bound)".

## Resolution and history

- 2026-11-11: opened from `research/chained_prime_transfer_20261111`
  (native Rust framework, PR #3877 follow-up).
- 2026-11-11: transfer-atlas addendum — cover sweep, census and down-edge
  controls; the obstruction holds across every arm measured.
- 2026-11-11: tower-up addendum (genus 1 → 2 → 3 on one instance); the ladder raises IC cost per rung.
- 2026-11-12: ladder/formula/smoothness/replay addendum; replay corrections
  applied (F1–F3, F6, F8–F12) and the bench rerun as `results_v2.json`.
- 2026-11-12: amortized-IC / certificate / larger-p genus-3 addendum; the
  obstruction holds against matched precomputed rho.
- 2026-11-12: cover push-forward and endomorphism-degree census addendum; the
  transferred subgroup's genus-2 IC is a birthday search on `E`.
- 2026-11-12: small-D_E addendum; the adapted base saves only by folding, which a
  generic control matches.
- 2026-11-12: degree-3 fibre and positive-control addendum.
- 2026-11-12: proof note added; ATLAS iff narrowed to only-if.
- 2026-11-13: instrument-fix addendum; the recorded gaps were understated.
- 2026-11-13: validation addendum; small_de admitted, three targets not (rho reference).
- 2026-11-13: negation re-measurement addendum.
