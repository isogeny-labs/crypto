> Idea Synthesist lane report (2026-11-12). Proposals only: nothing was run, no other file was edited, no status changed. Every statement in this file is `[pcc:unverified]`.

# Wave 5 synthesis: Curve25519 index calculus across characters, descent, regularity and isogeny formulas

## Sources checked

- research-kb: no `kb_status`/`search_knowledge` tool in this session, so there are no `source_id` receipts. The lead atlas bodies are vault-encrypted; novelty is checked against `research/leads_20261112/atlas_lead_ids.txt` (176 ids) and the files below. ecdb was not queried. I had no shell: every "stdlib check" below is hand arithmetic, shown inline, and must be machine-checked before use [pcc:unverified].
- Read: `research/leads_20261112/README.md` and `lanes/synthesis.md`; `research/curve25519_leads_wave5_20261112/lanes/real_parameters.md` (R1-R5); `research/curve25519_exponent_study_20261108/README.md` (base definition, the census table, the gidx test); `research/curve25519_exponent_study_ext_20261110/README.md`; `docs/ecdlp-hardness-corpus/WEAKNESSES.md` §1; `research/odd_n_trace_accounting_20261107/README.md`; `research/lift_dense_torsion_pairing_20261106/README.md`; `papers/ecdlp-index-calculus-treatise/sections/summer.tex` (yield and BKK paragraphs); session logs 2026-10-27 and 2026-11-07 (atlas summaries of LEAD-ICB-cofactor-torsion-quotients and LEAD-ICE-semaev-yield-model-coset-typing). `pipeline.md` did not exist when I finished, so duplicates against it are unchecked.

## Memo: the connection lane-by-lane work could not see

**1. The base the Curve25519 studies use is torsion-structured, and the binary lane has already measured what that structure does.** The exponent study's base is V = lo + g·μ_d with lo = A/3 and g = w^gidx (README "Base"). lo = A/3 is the Montgomery frame, so in Montgomery x the base is exactly the coset g·μ_d. Two facts follow, both standard and both outside the prime lanes' view [pcc:unverified]:
- *Descent character.* On y² = x(x² + Ax + 1) the map α(P) = x(P) mod squares is a homomorphism E(F_p) → {±1} with kernel the image of the dual 2-isogeny (Silverman-Tate III). With E(F_p)[2] = Z/2 (true for Curve25519, whose only rational 2-torsion point is (0,0)) and a cyclic 2-part, ker α = 2E(F_p). For odd d, μ_d consists of squares, so **every point with x in g·μ_d lies in one coset of 2E, selected by the Legendre symbol of g**.
- *T-stability.* Translation by (0,0) acts as x ↦ 1/x. So g·μ_d is mapped to itself exactly when g² ∈ μ_d. **gidx = 0 (g = 1) is T-stable; gidx = 2 (g = w²) is α-homogeneous and not T-stable; gidx = 1 is in the non-trivial α coset.**

The binary lane measured both mechanisms on F_{2^N} lines: trace-homogeneous bases sit on 2E and give 2.08× [1.62, 2.72] per trial on 2E targets (`odd_n_trace_accounting_20261107`, with "0 of 469,256 off-2E counts nonzero"), and T-stable lines lose a factor 2/2^{n-1} in relation rows per column (`lift_dense_torsion_pairing_20261106`: measured 0.54-0.56 at n = 3, 0.28-0.29 at n = 4). The prime-field transfer has not been made: the Curve25519 studies use gidx = 0 [pcc:unverified].

*Consistency with recorded data (hand checks, in-sample).* (a) The k = 20 census "columns" (abscissae of V on the curve) are 2, 2, 4, 12, 8, 16, 34, 28, 36, 76, 108, 96, 120, 216, 274, 278 for the 16 odd d: all even, as x ↔ 1/x pairing predicts when x = 1 does not lift (2^-16 by chance). (b) "covered (m)" at d = 3 (m = 2: 2, m = 3: 0) and d = 7 (m = 2: 2, m = 3: 4) are reproduced exactly by counting signed base points by their Z/8 component τ: τ ≡ 2 (mod 4) for the d = 3 pair, τ ≡ 0 (mod 4) for the d = 7 pair. An odd number of summands with τ ≡ 2 (mod 4) can never sum to τ = 0, which is the ℓ-subgroup. So the three recorded empty m = 3 cells (d = 3, 9 at k = 20; d = 3 at k = 22) need no yield explanation: they are a torsion-parity artifact [pcc:unverified].

**2. The yield heuristic is a character-sum statement, and its one exceptional term is that descent character.** For V = g·μ_d and m = 2, Y(T) = Σ_P 1_V(x(P))·1_V(x(T−P)), and 1_V expands over the e = (p−1)/d multiplicative characters trivial on μ_d. Since e is even, the quadratic character α is among them, and Σ_P α(x(P))·α(x(T−P)) = α(T)·#E exactly (α is a homomorphism). Every other character pair is a sum over a genus-1 curve with 4 zeros and poles, bounded by 4√p (Weil). Hence (derived) Ȳ(T) = (#E/e²)·(1 + α(T)) + O(4√p); for m summands the factor is 1 + α(g)^m·α(T). Two consequences [pcc:unverified]:
- The **mean** is a theorem (conservation plus α), not a heuristic: exactly 2× a random base of equal lift count on the right coset, 0 on the other. This answers the briefing's additive-combinatorics question: small subgroups *are* better than random sets, by exactly one bit, and the reason is a descent character, not sum-product. No other character of F_p^*/μ_d composes with x to a homomorphism, so there is no quartic analogue.
- The **per-target** law is beyond Weil-Deligne in the IC regime. Square-root cancellation needs d ≳ 2p^{3/4} at m = 2 and d ≳ c·p^{(m+1)/(2m)} in general. At d ≈ p^{1/m} it misses by a factor of about p^{(m−1)/2}. Bourgain-type subgroup bounds save only p^{−ε}. So HEUR-DECOMPOSITION-PROBABILITY (attack queue #4) splits into a theorem and a census question.

**3. The solving degree of every presentation on disk is set by its largest univariate degree.** I read this off the recorded tables. It connects the earlier synthesist's "carry" algebra to the real parameters [pcc:unverified].
- *Flat cells.* The top-degree form of S_3(x1, x2, t) in (x1, x2) is the monomial x1²x2². For a flat base the top system (y1^d, y2^d, y1²y2²) is Artinian with regularity d + 1. That is the Caminata-Gorla bound on the solving degree (to be checked against the primary text), and it equals the recorded SD at every prime-d cell (m = 2: 4, 8, 20, 74 at d = 3, 7, 19, 73; m = 3: 4, 8, 20).
- *Chain cells.* At m = 2, all 12 composite cells have SD − e_max(d) ∈ {2, 3}, for example 75 at d = 219 = 3·73 and 22 at d = 513 = 3³·19. At m = 3 the 7 composite cells have SD − e_max ∈ {3, 4, 5}. The top-system bound 2Σ(e_j − 1) − e_0 + 3 is about twice that, and the gap is the carry y_j^{e_j} → y_{j+1} that the top form discards.
- *Consequence for Curve25519.* Every μ_d with d ∤ 12 has a prime factor of at least 65,147, so if the law extrapolates, SD ≥ 65,148 on the curve's own subgroup bases in either presentation. Real-parameters R1/R2 then reduce to constants inside a closed route.
- *The lever this points to.* A presentation whose univariate degrees are about √d for *prime* d.

**4. That lever exists in isogeny cryptanalysis.** √élu (Bernstein-De Feo-Leroux-Smith 2020) writes a kernel as I ± J index sets: x ∈ x(⟨Q⟩) iff S_3^{E'}(x(iQ), x(jQ), x) = 0 for some i ∈ I, j ∈ J, with |I|, |J| ≈ √d/2. More generally a sumset base V = x(A ⊕ B) on an auxiliary curve E' has two dense univariates of degree about √d, coupled by one quadric. It needs no smooth p ± 1 and no smooth d, which are exactly what Curve25519 lacks. On E' = E it is the tried chained-Semaev route, and that case serves as the built-in control [pcc:unverified].

## Idea records

```yaml
id: W5-SYN-1-descent-character-coset-rule
class: measured-improvement candidate (relation phase, miniatures; conditional on the baseline's base choice) + security statement (Curve25519 subgroup bases gain exactly 1 bit)
mechanism: memo 1-2. alpha(P) = (x(P)/p) puts every odd-d mu_d base in one coset of 2E; g^2 in mu_d makes the base T-stable, so each l-relation appears 2^(m-1) times (even subsets shifted by (0,0)) and columns merge in pairs.
claim: >
  Relation-phase solves per needed relation, at identical F4 cost per system (gidx 0/1/2 T equal to 1e-5, recorded), normalised
  to a mu_d coset with (g/p) = +1 and g^2 not in mu_d: gidx 0 (T-stable) = 2^(m-2) (1, 2, 4 at m = 2, 3, 4); (g/p) = -1 with
  l-subgroup targets = infinite at odd m (no decomposition exists), 1 with targets T + tau, tau of order 8; a random set with
  the same lift count and mixed alpha = 2. Up to a chance factor from the tau (mod 4) composition, reported per base.
prediction:
  boundary: "Montgomery miniatures with #E = 8r, E(F_p)[2] = Z/2 (confirm in ecdb per curve), odd d | p-1, m = 2, 3 (m = 4 at d <= 9)"
  exact: "P-A1: 4r*P = O for every base point with (g/p) = +1 (100 %). P-A2: covered(m) = 0 iff no m-multiset has tau-sum = 0 mod 8; the three recorded empty m = 3 cells all have tau = 2 mod 4 on every base point. P-A3: on gidx 0, solution x-multisets of every consistent system come in orbits of size 2^(m-1)."
  ratio: "P-A4 decisive: solves to full rank, gidx 0 / gidx 2 (tau-composition matched): m = 3: 2.0 [1.6, 2.5]; m = 2 control: 1.0 [0.8, 1.25]; random set / gidx 2: 2.0 [1.6, 2.5] at m = 2 and 3"
discriminating_test: >
  Stage 0, about 10 minutes: extend the exponent study's census (scripts/census.py, or its Rust port under the language rule)
  with tau(P) = discrete log of r*P in E[8] for each base point. Check P-A1 and P-A2 on every recorded cell, k = 18..24.
  Stage 1, under an hour: collect relations to full rank by exact enumeration (yield does not depend on the oracle; the F4 cost
  equality across gidx is already recorded) for k = 18, 20; d in {19, 27, 57, 63}; m = 2, 3; gidx 0, 2, 1 (with and without the
  tau shift); a random alpha-mixed set of equal lift count. 3 seeds per arm.
falsification: "Any exception to P-A1 or P-A3, or P-A2 failing on a recorded cell, means the descent model is wrong for these curves. P-A4 at m = 3 outside [1.4, 2.8], or the m = 2 control outside [0.75, 1.33], means the mechanism does not carry to cost."
cost_band: >
  Stage 0 desk CPU minutes; stage 1 under 1 CPU-h (k = 20, m = 3, d = 63: 56 signed points, C(58, 3) = 30,856 triples per base,
  derived). No new F4 runs. If the IC baseline uses gidx 0 mu_d bases at m >= 3: a relation-phase cost x 1/2 at identical
  per-system F4 cost; otherwise 0. Unpriced native work (enumeration, tau classification) is equal in all arms and priced at
  0 / 0.1 / 1 GAE per operation; the order holds across the band because the arms differ only in yield. Matched rho at k = 20:
  1.5*sqrt(131,203) = 543 group operations. The distance to rho is unchanged in exponent; at most a factor 2 (m = 3) comes off.
controls: "positive: binary cross-check (the same counting reproduces 2.08x on an odd-n trace-homogeneous line); negative: gidx with (g/p) = +1 vs a second such gidx -> 1.00; A/A: two seeds."
risks: "If the miniature has full rational 2-torsion, alpha splits into two characters and the table changes (P-A1 then reads 'in one coset of 2E', with 4 cosets). Small M gives chance tau (mod 4) homogeneity (up to 4x or 0), a miniature trap; it is reported, not counted."
nearest_existing_lead_and_difference: "LEAD-ICB-cofactor-torsion-quotients (binary E/2E, closed_obstructed, 'at most 2 bits') and LEAD-ICE-semaev-yield-model-coset-typing (binary trace parity). This is the prime-field Montgomery mu_d transfer, plus the T-stability penalty on the base the Curve25519 studies actually use."
prior: "0.9 for P-A1/P-A3 (standard algebra); 0.7 for P-A4 inside its band."
```

```yaml
id: W5-SYN-2-yield-heuristic-split-weil-threshold
class: security statement + corpus instrument (attack queue #4 HEUR-DECOMPOSITION-PROBABILITY, #5 HEUR-RELATION-RANK)
mechanism: memo 2. Character expansion of Y(T); one exceptional character (alpha); every other term is O(C_W p^((m-1)/2)) by Weil-Deligne, which is informative only above d* ~ c p^((m+1)/(2m)).
claim: >
  (i) Theorem part: mean yield on coset c of 2E = (1 + alpha(g)^m alpha(c)) |A_V|^m / (m! #E), up to diagonal terms.
  (ii) Census part: after (i) and the tau (mod 8) composition, l-relations per target are Poisson (dispersion 1.00) on both
  sides of d*, and twice-orbit counts on T-stable bases have dispersion 2^(m-1) on x-tuples and 1 on orbits.
prediction:
  boundary: "p ~ 2^16..2^18, p = 5 (mod 8), E(F_p)[2] = Z/2; odd d | p-1 spanning p^0.3..p^0.8 (crosses d* = 2p^(3/4) at m = 2); m = 2, 3"
  table: "census mean / formula (i): 1.00 [0.97, 1.03] in every cell; dispersion index 1.00 [0.9, 1.1] below and above d*; max_T |Y - mean| <= 4 sqrt(p) above d* (a theorem; checks the instrument)"
  ratio: "dispersion(below d*) / dispersion(above d*) = 1.0 [0.85, 1.15]"
discriminating_test: "Exact histograms of all m-sums (Rust, O(|A_V|^m) point additions: 2^26 at m = 2, d = 2^13; about 2^30 at m = 3, d = 2^10), shared code with record 1."
falsification: "Census/formula outside [0.95, 1.05] in >= 2 cells: a second exceptional character exists (record 1's 1-bit cap fails). Dispersion > 1.2 below d* only: non-exceptional characters conspire on small subgroups, and the per-target heuristic fails for the bases PKM needs."
cost_band: "Rust histogram tool, about 4 engineering hours (or reuse record 1's); runs under 2 CPU-h; section 5a: no cross-method comparison."
nearest_existing_lead_and_difference: "LEAD-ICE-decomposition-probability-audit and LEAD-EXO-dl-character-sum-pseudorandom (H-PSEUDO Fourier flatness, R ~ 1.005 against a negation-closed null, summer.tex). Neither isolates the exceptional character or tests across the Weil threshold."
prior: "0.85 for (i); 0.7 for Poisson on both sides."
```

```yaml
id: W5-SYN-3-largest-univariate-degree-law
class: security statement (Curve25519 subgroup bases: Groebner degree >= 65,148) + instrument (attack queue #1 HEUR-FIRST-FALL-DEGREE, #2 SEMIREG)
mechanism: memo 3. For a univariate constraint (binomial, dense random, torus trace) the top system is (x_i^d, top S). At m = 2, top(S_3) = x1^2 x2^2, regularity d + 1, which bounds SD (Caminata-Gorla). Chains fall below their top-system bound through carries, landing near the largest prime factor.
claim: "SD(m, d) = e_max(d) + c(m, L) with c = 1 for L = 1 (m = 2, 3), c in {2, 3} at m = 2 and {3, 4, 5} at m = 3 for L >= 2; at m = 4, SD >= 8 (Semaev degree) dominates small d."
prediction:
  boundary: "sym form, frozen binary dc2c30a4, cells already on disk that I did NOT read (k = 22, 24) are the out-of-sample set"
  table: "k = 22, m = 3 (preregistered now): d = 3, 5, 11, 31, 41 -> SD 4, 6, 12, 32, 42; d = 15 (3*5), 25 (5*5) -> [8, 10]; d = 33 (3*11) -> [14, 16]. R2 arms (binomial, dense, random) -> identical SD = d + 1 at m = 2 (theorem-level), m = 3 (law)."
  ratio: "SD_chain / reg(top system) in [0.4, 0.65] for every L >= 2 cell"
discriminating_test: "Desk: read data/tables.md (k = 18, 22, 24) against the preregistered table; compute reg(top system) at m = 3 from top(S_4) with a native CAS (Singular/Macaulay2) in minutes; no new runs."
falsification: "Any out-of-sample prime-d cell with SD != d + 1, or >= 2 composite cells outside e_max + [c_min, c_max]: the law is in-sample only, and the Curve25519 statement reverts to the top-system bound."
cost_band: "about 1 h desk; 0 CPU-h; section 5a not applicable."
consequence_if_holds: "Curve25519: d | p-1, d not dividing 12, forces e_max >= 65,147. Torus d | p+1 = 2*5^2*79*M with e_max <= 79 caps |V| <= 3,950 ~ 2^12, far below the useful 2^64..2^85. Groebner degree on the curve's own subgroup bases >= 65,148 in every presentation."
nearest_existing_lead_and_difference: "LEAD-ICE-prime-field-semaev-solving-degree, LEAD-ICE-first-fall-degree-assumptions, LEAD-ICE-chained-semaev-dreg-vs-semiregular. None states a closed SD law across presentations or ties it to p-1's largest factor."
prior: "0.75 flat (m = 2, 3); 0.55 chain band out of sample."
```

```yaml
id: W5-SYN-4-sumset-base-sqrt-velu-presentation
class: measured-improvement candidate (decomposition-oracle presentation for prime d, m = 3 window), low prior
mechanism: memo 4. Base V = x(A (+) B) on an auxiliary curve E' (e.g. sqrt-Velu I +- J of an order-d point), presented per summand by u (degree |A|), w (degree |B|) and S_3^{E'}(u, w, x) = 0. Under record 3's law, SD ~ sqrt(d)/2 + c instead of d + 1.
claim: "At k = 20, m = 3, prime d in {61, 73, 109}: SD in [sqrt(d)/2 + 2, sqrt(d)/2 + 6]; T_sumset / T_flat <= 0.5 at d = 109; T_sumset / T_flat >= 2 at d = 19 (below the crossover)."
prediction:
  boundary: "column heuristic (9 variables at SD ~ sqrt(d)/2 + c vs 3 at d + 1) gives a window of about 60 <= d <= 1,500 at m = 3 (derived: C(17,9) = 24,310 vs C(77,3) = 73,150 at d = 73; crossover where d^1.5 ~ 9!/3! = 60,480)"
  ratio: "relations per F4 solve x 1/2 vs an alpha-homogeneous mu_d base (record 1): E' points are alpha-mixed on E; priced in"
discriminating_test: "Rust: a Membership::Sumset(A, B, E') variant in subgroup_semaev.rs; flat dense arm from R2 at the same d; E' = E arm as control (the chained-Semaev 2m-system); 3 targets per cell, frozen seeds."
falsification: "SD above sqrt(d)/2 + 8 at d >= 61, or T_sumset >= T_flat at d = 109: close the axis (as R5 tensor would). E' = E arm cheaper than E' != E: the auxiliary curve adds nothing."
cost_band: "about 1 day Rust; runs <= 6 CPU-h (flat d = 109 extrapolated from 7.3e9 at d = 41 by d^4.95: about 1e12 field ops, under 1 h at 2.9e8/s); rho reference 543 group ops; unpriced E' table build (|A| + |B| point operations) at 0 / 0.1 / 1 GAE, below 1e-6 of T."
nearest_existing_lead_and_difference: "Encyclopedia 0.2 / LEAD-ICP-pkm-tower-algebraic-oracle (Velu tower, 2-power d), real-parameters R5 (multiplicative tensor), LEAD-ICE-chained-semaev-dreg-vs-semiregular (E' = E). New: prime d, no smoothness, an additive (group-law) split on a curve other than E."
prior: "0.25 that it beats flat anywhere in the window; 0.6 that the SD prediction holds."
```

## Fields crossed and rejected (one line each) [pcc:unverified]

- **Guruswami-Sudan / elliptic-code list decoding.** A decomposition is a weight-(n − m) word of an [n ≈ 2|V|, m] elliptic code. Agreement m sits below the Johnson radius √(nm) by a factor √(2|V|/m), and the route is tried (LEAD-EXO-/LEAD-ICP-elliptic-code-list-decoding).
- **Sum-product / Bourgain-Glibichuk-Konyagin.** Conservation fixes the mean, and x1 + x2 = x3 in μ_d is field addition, which the group law never sees. The known energy upper bounds (|H|^{5/2}, |H|^{32/13}) exceed the random count |H|³/p for |H| ≤ p^{1/m}. The real structural lower bound is the descent character (record 1).
- **BKK mixed volume.** In every presentation here BKK equals Bézout equals the number of torus roots, so it cannot separate chain from flat. A BKK speedup was already measured at 1.0-1.8× (DEC-20260806-26c0e8). The Gröbner-fan weighted grading makes the chain graded-isomorphic to flat (R1). Record 3 uses top-form regularity instead.
- **HPC (communication-avoiding or out-of-core F4).** This is R4. For d | p − 1 the quotient is split and NTT-diagonal, which is the known enumeration oracle, with total cost about p by conservation.
- **Lattices.** Small-root reach 0.354 < 1/2 closes A12 (LEAD-ICP-coppersmith-smallroot-semaev). Lattice relation construction is tried (LEAD-ICP-lattice-ic-relation-construction).
- **Torsion-point isogeny attacks (Kani, Robert).** They need torsion images under a secret isogeny. Embedding [a] needs a itself, and evaluating it on torsion is a pairing transfer (LEAD-TRF-mov-small-embedding-degree).
- **Blahut/BCH linear complexity on F_p[y]/(y^d − 1).** A generic inverse has full linear complexity, so an early stop below d + 1 has probability about p^{−(d−D)}. Record 3 gives the degree law directly.
- **Lift-fraction coset choice** (the quadratic character of x + 1/x + A). At k = 20, d = 27 the best of 38,836 cosets lifts about 23/27 (derived: Binomial(27, 1/2) tail 1.55e-4 at ≥ 23). But the excess decays as √(ln e/d), to about 2^-38 at real size. It is a miniature trap: report the lift count as a control.

## Ranking

| # | record | decides | cost | class | predicted outcome |
|--:|:--|:--|:--|:--|:--|
| 1 | W5-SYN-1 descent-character coset rule | whether the Curve25519 studies' gidx 0 base pays 2^(m−2); the 1-bit cap; the empty census cells | 10 min + < 1 CPU-h, no F4 | improvement (conditional) + security | P-A1/P-A3 exact; ×2.0 at m = 3 |
| 2 | W5-SYN-3 largest-univariate-degree law | SD law across presentations; Curve25519 Gröbner degree ≥ 65,148 | ~1 h desk, data on disk | security + instrument | k = 22 prime cells SD = d + 1 |
| 3 | W5-SYN-2 yield split at the Weil threshold | turns attack-queue #4 into THM (mean) + HEUR (per-target) | ~4 h code + < 2 CPU-h | security + instrument | Poisson on both sides of d* |
| 4 | W5-SYN-4 sumset (√élu) base | a sub-d-degree presentation for prime d without smooth p ± 1 | ~1 day + ≤ 6 CPU-h | improvement, low prior | wins only for 60 ≲ d ≲ 1,500 at m = 3 |

None of the four moves any row's exponent toward matched rho. Record 1 is the only one with a measured relation-phase gain in reach (≤ 2× at m = 3), and only if the baseline uses gidx 0. The distance remaining is the exponent study's ≥ 3.98e5 field_ops per rho group operation at k = 20. Run order: record 1 stage 0, then record 3 (desk), then record 2 on record 1's tool, then record 4 only if record 3's chain law holds out of sample [pcc:unverified].
