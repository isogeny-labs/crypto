# Idea records — new relation paradigm synthesis (2026-11-24)

Idea-Synthesist. Proposals only; no experiment run, no result claimed, no
status changed. Every lab number is quoted from a file opened (see SYNTHESIS.md
"Prior art, with paths"). All `[pcc:unverified]`. research-kb/lead-atlas NOT
reachable in-container, so novelty statements are `rg`-scope only and must be
KB-checked before any PR.

---

## SYN-A — BGIP character-design dual functional for the split-Semaev tower

```yaml
id: SYN-20261124-PKM-BGIP-DESIGN
title: >-
  Resolve the PKM degree-slope lower bound by transferring the BGIP Tseitin-mod-q
  character-design to the two-free-block split-Semaev tower, and reconcile the
  two IC lanes' contradictory slope measurements
obstruction_attacked: >-
  solving_degree_tower_frontier_20261124: verdict OPEN-leaning-KILL because "no
  degree-slope lower-bound theorem d_F(m,t) >= c*t" exists, blocked by the
  one-free-block barrier (prime_leads_chase_20261113/L1-pkm/THEORY.md Prop R:
  one free block has t-independent degree O_m(1); a growing bound needs two free
  blocks jointly, unproved). This is the repo's named "missing theorem".
claim: >-
  The quantity is a polynomial-calculus refutation degree (chain_sd_bounds_
  20261115 REPORT.md Thm 2: sd = ref = d_F = LFD, Lean-checked). A co-closed
  dual-functional family Lambda_c built from characters of mu_{2^t} x mu_{2^t},
  weighted by the JOINT 2-adic expansion of the two free blocks (the BGIP
  Tseitin-mod-q "design", named but never carried out in L1-pkm/THEORY.md
  "What is NOT proved"), annihilates U_c(k) and separates 1 at exactly the
  measured LFD staircase 4,4,4,4,5,5,5,5 (chain_sd §8), i.e. SUCCEEDS at
  (k=6,c=4) where the naive boundary-evaluation functional of chain_sd §7.3
  FAILS (188/605 rows, yet 1 not in U_4). Its design-degree is governed by
  Pi(w) (chain_sd §13.2 Corollary A), measured Pi(3)=6, Pi(4)>=10.
mechanism: >-
  The tower normal-form degree IS the popcount of the character index
  (chain_sd Thm 1: deg(NF m) = wt = popcount) -- exactly the degree measure BGIP
  use for PC degree lower bounds mod q. The one-free-block barrier is the
  proof-complexity statement that a single Semaev block does not expand; the
  two-free-block requirement is Tseitin-on-an-expander. BGIP's character-design
  is the known technique that separates where single-source evaluation functionals
  (already tried in chain_sd §7.3 and failed) do not. If the design's attainable
  degree follows the counting heuristic m(w) ~ w*log2(w) (chain_sd §13.4), the
  slope DECAYS, Missing Lemma Pi is FALSE, and the degree-slope route cannot
  close PKM; if it grows linearly, Pi is true and PKM is cleanly KILLed.
prediction: >-
  Direction that discriminates: the character-design SEPARATES 1 at (k=6,c=4)
  (unlike the naive functional) AND its degree tracks Pi(w) sub-linearly
  (Pi(3)=6, Pi(4)=10, predicted Pi(5)=14 -- slope ~1/4 at reachable depth,
  decaying as 1/log t). Magnitude: the design-degree matches the certified LFD
  within +-1 level on the four test cells. Declared interval: design-degree slope
  in [0.20, 0.26] per unit c at k<=12, below 0.20 extrapolated to k>=16.
discriminating_test: >-
  Desk construction + existing native Rust engine
  research/chain_sd_bounds_20261115/native (no Python, no cloud). (1) Build
  Lambda_c as a character-weighted design at m=2 for (k,c) in
  {(6,4),(7,4),(10,5),(11,5)} -- the cells where truth/reachable/Theorem-A'
  already disagree by known constant offsets (chain_sd §13.8 ladder). (2) Check
  annihilation of U_c(k) on the closures ALREADY CACHED by `bnd`/`cofactor`/
  `closure`, and lambda(1)!=0. (3) Read the design-degree slope and compare to
  Pi(w) (chain_sd §13.3) and to solving_degree_tower's sigma=0.125-0.2.
falsification: >-
  Killed if the design fails at (k=6,c=4) with the SAME rank deficiency as the
  naive boundary evaluation (no better separation) -- then BGIP carries no
  further than evaluation functionals already tried and the two-free-block route
  is dead for this system. Separately, a LINEAR design-degree (Pi(w) >= alpha*w)
  makes Missing Lemma Pi TRUE and cleanly KILLs the lever (portfolio-valuable,
  not an exponent route).
cost_band: >-
  Desk + replays of already-computed closures: design construction ~1-2 days
  desk; closure replays at (10,5) ~1 host-hour each on the recorded 4-core/15GB
  host (chain_sd §8), (11,5) via Theorem 4 transfer from k=10 (free). No EKS, no
  paid compute, no new benchmark record. Reference arm (the naive functional) is
  already recorded in chain_sd/reports/evaltest.md at 0 extra cost. Priced at 0,
  0.1, 1 GAE the arms are desk-dominated; GAE band not load-bearing here (no
  group-op comparison in the test).
controls: >-
  Positive control: naive boundary-evaluation functional, which MUST still fail
  at (k=6,c=4) (chain_sd §7.3) -- reproduces the known negative. Negative control:
  the affine/Mobius couplings (chain_sd §13.5) that share slope 1/4 -- the design
  must behave identically, confirming the slope is coupling-independent. A/A:
  two random character seeds per cell (chain_sd uses two seeds/cell in §13.1).
risks: >-
  (1) The design may not be constructible in closed form for S3's (2,2) bidegree
  coupling; mitigated by testing on the simpler affine coupling first. (2) A
  sub-linear degree does NOT give an exponent win on its own: chain_sd §13.6
  shows 1 enters the closure at 84-99.7% of full rank and k=15 needs 2.6M
  columns (~25TB) -- the column count C(2k,<=D), not the degree, is the cost.
  The exponent route needs a sparse/structured solver beating dense Macaulay
  width, which this test does not address. (3) research-kb was unavailable, so
  "BGIP never applied to the Semaev tower" is rg-scope novelty only.
closest_baseline: >-
  chain_sd_bounds_20261115 Theorem A/A' and the Missing-Lemma-Pi reduction
  (same object, dual functionals, but single-source evaluation / cofactor-support
  invariants, not the BGIP two-source character-design). solving_degree_tower_
  20261124 derived KILL from constant sigma (the assumption this test probes).
lean_obligation: >-
  If the design separates: a co-closed-annihilation theorem Lambda_c perp U_c(k),
  dual to chain_sd's Closure.lean/Cofactor.lean family. If Pi sub-linear: a Lean
  upper bound Pi(w) = o(w) closing chain_sd T9 negatively.
probability: >-
  P(separates where naive failed) ~0.4; P(resolves Missing-Lemma-Pi vs constant-
  sigma either way) ~0.7; P(opens an exponent route for ECDLP) ~0.08.
```

---

## SYN-C — ruling out the genus-3/cover transfer for the pinned targets

```yaml
id: SYN-20261124-GENUS3-DIM-NOGO
title: >-
  The genus-3 exponent-1 IC advantage cannot be transferred to ECC2K-130
  (n=131 prime) or to prime-field curves (n=1); the obstruction is the prime
  extension degree, not a missing construction
obstruction_attacked: >-
  The pinned goal names "exponent 1 via a transfer to a plane quartic and Diem's
  index calculus, against 3/2 for rho" (docs/ACTIVE.md). The question: can any
  isogeny/cover/correspondence carry an ELLIPTIC DLP into a genus-3 setting where
  that exponent applies?
claim: >-
  No. The genus-3 Diem/Smith exponent-1 (O~(q) vs rho q^{3/2}; SUCCESS_CRITERIA
  §5 table) is a property of a DLP in a group of dimension 3 / order ~q^3. An
  elliptic DLP is dimension 1, order ~q. Transfer preserves order (hence hardness:
  prime-field-chained-transfer-no-advantage.md T7, Tate), so it cannot synthesize
  the missing dimension. The only dimension-raising mechanism (Weil restriction /
  GHS of E/F_{q^n}, genus ~2^{n-1}) needs n COMPOSITE with a small factor.
  ECC2K-130: n=131 PRIME, only subfield F_2, genus >= 2^130 (koblitz-modular-
  carrier lane reaches genus 45,589 at n=131 with NO evaluated subgroup map in 29
  rounds). Prime-field curves: n=1, no extension. So the exact object
  "computable hom from <P> onto an order-r subgroup of Jac(C), C/F_{2^m}, m|131,
  m<131, g small enough to beat rho" does NOT exist.
mechanism: >-
  Measured three ways across lanes: (i) all IC relations on a transferred elliptic
  subgroup degenerate to fibres = collision search on E (chained-transfer:
  189,330 genus-2 + 28,815 genus-3 relations, phi2_* phi1^* = 0 on 300/300);
  (ii) pushing an order-r~q subgroup into a genus-3 Jacobian of scale q^3 lands
  in a strict subvariety where Gaudry's relation analysis fails and q^{4/3} is
  70.17 bits ABOVE rho (n83_genus3 DESTINATION_CALCULUS_SCREEN.md, Gaudry
  ePrint 2004/073 Remark 2); (iii) SUCCESS_CRITERIA lines 284-286 state descents
  needing an intermediate subfield do not apply to prime n=131.
prediction: >-
  Any attempted elliptic->genus-3 transfer on the pinned targets either (a) fails
  to produce a subgroup-preserving map (koblitz-modular, 29 rounds, still no
  evaluated map), or (b) produces one whose destination IC cost is >= rho on E by
  the dimension argument (n83: 70 bits above). No finite-size crossover exists.
discriminating_test: >-
  No new experiment is warranted -- this is a ruling-out grounded in three
  recorded lanes. The checkable restatement: enumerate m | 131 with m < 131
  (only m=1), compute the forced GHS genus 2^{131/m - 1} (astronomical), and
  confirm no §8-fidelity ECC2K-130 instance has a small-genus cover. For prime
  fields, confirm n=1 admits no Weil restriction. Both are arithmetic facts,
  already in SUCCESS_CRITERIA §5 and the GHS problem record.
falsification: >-
  REOPENED only by a curve family where the extension degree n has a small factor
  AND the instance passes the ECC2K-130 §8 fidelity gate (or a prime-field analogue
  with a genuine cover) -- none exists for the pinned targets. A new weak
  prime-field subfamily discovered later (chained-transfer §Atlas positive control
  note) would be a result for THAT subfamily, not ECC2K-130.
cost_band: >-
  Zero compute. Desk ruling-out from recorded lanes. No benchmark, no run.
controls: >-
  Positive control (the exponent-1 result is real where it applies): ic_positive_
  control_20261112 -- genus-3 double-LP IC below rho in group ops from p=257, on a
  genus-3 SOURCE (dimension 3). Negative control: the n83 strict-subvariety screen
  (dimension-1 source into genus-3 ambient = 70 bits above rho).
risks: >-
  None to the ruling-out. The risk is MIS-reading it as "genus-3 IC is useless":
  it is not -- it is the engine of the pinned hyperelliptic genus>=2 goal, whose
  source is ALREADY dimension >=2. This record sharpens why that goal is the live
  one and the elliptic-transfer hope is not.
closest_baseline: >-
  problems/prime-field-chained-transfer-no-advantage.md (T7/Tate, measured),
  n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md (Gaudry Remark
  2, q^{4/3} = +70 bits), problems/ghs-higher-magic-descent-not-executable.md,
  problems/koblitz-modular-carrier-transfer.md (genus 45,589 at n=131).
lean_obligation: >-
  None created (a no-go grounded in existing records). If formalized: "n prime =>
  GHS genus >= 2^{n-1}" and "DLP-preserving E->E' => #E=#E'" (Tate), both standard.
probability: >-
  P(exponent route for ECC2K-130 or prime-field curves via genus-3/cover
  transfer) ~ 0 for the pinned targets. Obstruction is the prime extension degree.
```
