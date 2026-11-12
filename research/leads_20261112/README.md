# Index-calculus techniques by field regime: open cells, new proposals, next steps (2026-11-12)

Owner's request: look at the atlas families for prime fields, Koblitz curves and the rest, think of new
index-calculus techniques, and for the promising ones give next steps. Everything here is a proposal or a desk
computation: `[pcc:unverified]` throughout; nothing was run on a curve; no row, leaderboard or atlas verdict changes.

## 0. Inputs

- The lead atlas brief (image, "Factor-base families tried, by field regime", verdicts as recorded, 2026-11-12).
  The atlas bodies in crypto-autoresearcher are vault-encrypted; only the 176 lead ids were readable
  (`atlas_lead_ids.txt`). Novelty below means: not among those ids, not among the 173 ids of the 2026-11-10 sweep,
  and not among the 13 leads of PR #3938 (`research/leads_20261111/`).
- Today's fall-degree results for the flagship prime family 0.1 (two-summand μ_{2^k} chains), from
  isogeny-labs/ml-cryptanalysis PR #101: LFD ≤ k + 2 (kernel), LFD = ⌊(k+14)/4⌋ measured to k = 15 (kernel relative
  to four GPU closures), LFD unbounded with slope ≥ 1/14 (written proof, reviewed 2026-11-12). The binary deep chain
  (interim, 16 of 57 systems): certified LFD 4 at k = 11 on random subspaces where law B predicted 5.
- `docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` (frontier), `WEAKNESSES.md` §6 (live obstructions),
  `docs/ic/LEADERBOARD.md` (every IC row above matched rho except the GHS-weak family).
- `research-kb` was not reachable from this session (no MCP server, no CLI executable); the knowledge-base pass is
  owed before any of these becomes a task card.

## 1. The bar: Gröbner routes on multiplicative-subgroup bases against brute-force decomposition

With m summands in a coset of μ_N, N = 2^k, the chain presentation has m·k variables. Brute force decomposes one
target in 2^((m−1)k) summand tuples. The Gröbner/XL route must at least write the Macaulay columns of weight
≤ LFD = s·m·k, about 2^(m k H(s)) of them (H the binary entropy). Thresholds (computed this session):

| m | columns < enumeration needs LFD/k < | sparse linear algebra (ω = 2) needs LFD/k < | ω = 2.81 |
|--:|--:|--:|--:|
| 2 | 0.220 | 0.083 | 0.054 |
| 3 | 0.522 | 0.184 | 0.117 |
| 4 | 0.858 | 0.290 | 0.182 |

Measured m = 2 slope: 0.25. The matrix alone already exceeds brute force (2^(1.087 k) columns against 2^k), and the
proved floor 1/14 = 0.071 (written proof, `[pcc:unverified]`) is above the ω = 2.81 line, so at m = 2 the Gröbner route on family 0.1 cannot beat
enumeration even at the proved floor unless the linear algebra is sub-quadratic in the columns. The decisive unknown
is LFD(k) at m = 3: below 0.184·k the route is alive, above it family 0.1 closes for m ≤ 3. No record of a certified
m = 3 prime-chain LFD exists in the files read here. Caveats: equal yield per attempt in both arms; the column count
is a lower bound on F4 work; enumeration priced at one unit per tuple (a quadratic solve and an N-th power test).

## 2. Proposals

Three `idea-generator` lanes (Sonnet) and one `idea-synthesist` (Opus) worked from the same briefing; I read every
record and re-derived the numbers I quote. Lane reports are in `lanes/`. Every claim below is `[pcc:unverified]`.
"Nearest existing" names the closest atlas or sweep id; bodies of the atlas were not readable.

### 2a. Prime fields (open cells 0.1, 0.2, A12)

**P1 `LEAD-P-kummer-penultimate-quotient-law`.** Read off the committed closure profiles (`N8-growth.md` §6.2,
`ic-bounds-k14/RESULTS.md`): at the penultimate degree D = LFD − 1 the codimension q of V_D in M_≤D is 2.5–5.7 times
B = 2^k at every plateau position, for D = 3, 4, 5 (D = 4 and 5 agree to 1%) and at the one D = 6 cell (3.07 B).
Consequence: any solver that forms or eliminates that quotient pays at least about 3B per target, which is the cost
of enumerating the base; the measured closure cost exponent is B^2.24. In-sample; the test is out-of-sample (fresh
primes, D = 3, 4, k ≤ 9, CPU minutes) and the derived D = 6 predictions q = 167,665 / 378,423 / 715,797 at k = 15 / 16 / 17.
Nearest: `LEAD-ICP-pkm-tower-algebraic-oracle`, `LEAD-ICE-prime-field-semaev-solving-degree` (both ask how large D
is; this asks what must be stored at D − 1). If it holds: cell 0.1 → engineering-only, independent of the slope.

**P2 `LEAD-P-kummer-freeext-lift-k15-18`.** Use the free-extension law (profile recursion, Reduction B) to build
V_6(15..17) from the committed V_6(14) basis with no new elimination, then run the committed D = 6 → 7 lift to decide
1 ∈ V_7(k) at k = 15, 16, 17 and, the decisive cell, 1 ∉ V_7(18). Stage 1 (direct V_6(15) hash = free-extension hash)
is the first test of Reduction B at D = 6. Only 1 ∈ V_7(18) (plateau longer than four) would reopen 0.1 on the slope.
GPU hours on a rented H200 (owner's call on the route); likely outcome is the low-information confirmation.

**P3 `LEAD-P-kummer-radix-presentation`.** Radix-2^ρ chains (ρ ≥ 2) cut the variable count at equal base size;
column exponent 1.087 → 0.775 at ρ = 3 (derived) but the degree cannot drop below the binary chain's (digit sums
in base 2^ρ dominate binary digit sums). Expected null for ECDLP cost, possible matrix-size gain. Cheap at k ≤ 12.
Nearest: `LEAD-ICP-digit-presentation-solving-degree`.

**P5 `LEAD-P-a12-degree-d-reach-and-quotient`.** A12 (height box, degree-d maps): the small-root reach 0.354/d
against the yield threshold 1/2 leaves margins −0.15, −0.32, −0.38 at d = 1, 2, 3 (derived), so the lattice route
closes on paper; one small closure test asks whether the P1 quotient law also holds for degree-2 and degree-3 map
images. Nearest: `LEAD-ICP-coppersmith-smallroot-semaev`.

*Dropped:* P4 (`LEAD-P-isogeny-transport-is-plain-base`) read cell 0.2 as an isogeny into the target's class. The [pcc:unverified]
encyclopedia (§0.2) defines 0.2 as an auxiliary curve E′ with a Vélu x-map tower; the lemma P4 proves belongs to the
already-refuted A11/D7 reading. Its replacement is the synthesist's S2 below.

### 2b. Covers, Jacobians, extension fields (open cells E3, E6; B1–B3)

Structural findings first (quoted from `research/weil_restriction_isogeny_census_20261018/REPORT.md` and
`research/quartic_n4v2_20261020/`): the Joux–Vitse weak class is the l = 2 case of Tian's kernel construction and
exists only with full rational 2-torsion; #E mod 4 is an isogeny-class invariant, so no isogeny walk moves a
prime-order curve into it; the only mechanism on disk that reaches prime-order curves is Tian's odd-l kernel with a
plane-quartic codomain (kernel present on about 0.4 of prime-order classes at q = 7..13; no Tian isogeny has ever been
computed here). E3 (CM class ideals) closes by structure: a closed class walk acts as one scalar on P and Q and
carries no logarithm information (Lean obligation `cm_closed_walk_scalar_action`, elementary).

**J1 `LEAD-J-quartic-dl1-generic-jacobian`.** Run the unchanged N4v2 plane-quartic solver (0.005× rho-on-Jacobian on
the weak family) on random cofactor-one quartics, k = 12..18. Predicts exponent 1.00 [0.93, 1.10] and
DL1 / rho_J ≤ 0.10 at k = 16 across w = 0, 0.1, 1; reports against both rho on the Jacobian and a modelled rho on
E(F_{q³}) (about 100× cheaper per step), so no below-rho-for-E row can come from it. About one hour local. Decides
whether the solver is family-independent before any Tian transport work.

**J2 `LEAD-J-dl1-percolation-fb-size`.** The frozen factor-base constant c_D = 2.0 was picked from {1.5, 2.0, 2.5,
3.0}; c = 1.5 failed at k = 16, 18 and c = 1.75 was never tried first. A percolation model of the line graph puts the
50% threshold at c_50 ≈ 1.55 [1.40, 1.75]; predicts DL1 cost × 0.77 at c = 1.75 and × 0.65 with an adaptive
cycle-rank stop. Under an hour local, 54 units; also the first measurement of `HEUR-RELATION-RANK` and the
large-prime graph assumption on a scaling family. Class if it holds: engineering (1.3–1.5× on the ratchet row).
Nearest: `LEAD-ICE-relation-graph-rank-large-prime`, `LEAD-ICB-large-prime-variants`.

**J3 `LEAD-J-tian-n3-transport-and-reach`.** Staged: census of Tian kernels to q = 61, l ≤ 13 (hours); then the
real obstruction, an explicit (3,3,3)-isogeny to the quartic in a theta model (weeks); then end to end against rho.
Starts only if J1 passes.

**J4 `LEAD-J-k4-genus2-nagao4`.** E(F_{p⁴}) through a genus-2 cover over F_{p²} with a four-point Nagao descent
(Bézout number 16 against 64 for genus 3); derived crossover p* ≈ 2·10³ [4·10², 1.2·10⁴]; only the four-point test is
measured first. Nearest: the `LEAD-ICP-gfp5-*` family.

### 2c. Cross-lane synthesis

The synthesist's connection: the growth proof (`N8-growth.md`) silently needs one equation (a curve), characteristic
above 2^(k+1), and monomials that are S-units for a bounded S. Binary descents break the first two. The Kummer and
Dickson towers share a square-zero graded algebra (every square is a weight-dropping carry) and measure the same D;
the Vélu tower of flagship 0.2 is the one presentation whose graded algebra differs (y_j² ≡ y_j·y_{j+1} at top
weight). It also corrects the briefing: binary is not flat (treatise R35, k = 18: LFD ≥ 5 under a valid certificate),
it steps at variable counts near 12, 16, 36 against the prime law's every 8; and the entropy bar's 0.220 is the same
constant as U1's counting obstruction 2·H⁻¹(1/2), a consistency check between two independent derivations.

**S1 `LEAD-SYN-char-and-generator-crossing`.** Run the existing μ_{2^k} closure engine with the modulus set to a
small prime l ∈ {17, 97, 257} below 2^(k+1) (where the locality theorem does not apply), with and without the
Frobenius image g^l as a second input, and modulo 12289 with a second generator. Four hypotheses (locality L, number
of equations Dm, characteristic links F, characteristic-free combinatorics S) predict four patterns on five
preregistered cells at k = 4..10. At most 35 CPU-hours, engine unchanged. This is the mechanism test for the whole
fall-degree program and the cheapest discriminator between "prime" and "binary" behaviour.

**S2 `LEAD-SYN-velu-tower-exact-lfd`.** Exact closure LFD of the Vélu tower (0.2) on unit instances at k = 4..10,
weight-matched against a degree-2 map-image base, k = 14 on a GPU as stage 2. Decides 0.2's open solver axis and
whether the prime law is carried by the graded algebra. Prior lowered by the encyclopedia's F4 degrees for this
tower (m = 2: 4,5,5,…,5,6 at N = 2..20 against Kummer's 4,4,4,4,4,5,…,6: "rises at the same level"), which are F4
degrees, not certified LFDs. Needs a Rust tower-closure port (1–2 days).

**S3 `LEAD-SYN-nowrap-closedness-screen`.** The closure-free criterion from Prop. 2.2 of the growth draft: bounded
LFD at degree D requires the no-wrap ideal space to stay non-closed at every large k; closedness is an
evaluation-matrix kernel (12,951 monomials and 665 s at D = 5, k = 12, against a 55,455-column closure). Screen six
presentations (Kummer, coset, t = 0, map image, Dickson, Vélu) at D = 3, 4, 5. A kill screen, not a certificate; every
future prime-field base proposal would report its closedness depth at intake.

### 2d. Main-session items not covered above

**M1 arity grid at m = 3 (prime).** Certified LFD(k) for three-summand μ_{2^k} chains at k = 3..7 with the existing
closure engines; the one record on disk (p = 7681, k = 6: LFD ≥ 8, V_7 closed at dim 19,958 against 62,956) is
already above the sparse-linear-algebra line 0.184·k = 1.1 by a wide margin, so the preregistered expectation is
that m = 3 closes too; the measurement costs CPU hours and turns the §1 table from an entropy bound into a
measured row. Nearest: `LEAD-ICP-full-width-arity-grid`.

**M2 free-extension solver.** Subsumed by P2 (the lift seeded from the free extension) and S3.

## 3. Ranking and next steps

## 4. Proposed atlas verdict changes (proposals only)

- 0.1 Kummer g·μ_{2^t}: "open" → "open only for m ≥ 3 with LFD/k < 0.184, or slope < 0.083 at m = 2" (§1).
- C5 trace-zero: today's k = 11 binary measurement (LFD 5, every law) is consistent with "engineering".
- A4 subgroup/coset: if its refutation is cited through the fall degree, the coset case of the k + 2 bound is written
  only, not kernel-checked.

## 5. Considered and not proposed

## Sources checked

