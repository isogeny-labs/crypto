> Agent report, verbatim (2026-11-12). Proposals only; nothing was run; every statement in this file is `[pcc:unverified]`.

# Synthesis: locality, characteristic, number of equations and the graded tower algebra (2026-11-12)

Role: Idea Synthesist. These are proposals only. Nothing was executed. No repository file, result, status or
evidence was edited, and no work was assigned. Every number below is quoted from a file on disk (path given) or
is marked *derived*, with the arithmetic shown. No pcc claim id was looked up, so under the sabotage rule every
cited result is `[pcc:unverified]`, an unverified producer assertion.

## Sources checked

- **research-kb: NOT reachable from this session.** No `kb_status`, `search_knowledge` or `search_theorems` tool
  was available, so there are no `source_id` receipts. The lead atlas bodies are vault-encrypted, so only the ids in
  `atlas_lead_ids.txt` (176) were used. ecdb was not queried, because no curve fact beyond those in the cited files
  was needed. No web search was made. Brownawell–Masser and Hsia–Wang are cited only as N8 cites them.
- Scratchpad (`leads/`): `BRIEFING.md`, `lane_A_falldegree.md`, `lane_B_binary_ic.md`, `lane_C_prime_ext_cover.md`,
  `EXISTING_LEADS.txt` (173), `atlas_lead_ids.txt`, `EXISTING_LEADS.md` (empty), and the sibling outputs
  `out/main_session.md` (all), `out/lane_P_prime.md` (sources, evidence, P3–P5, ranking, considered) and
  `out/lane_J_covers.md` (headings only).
- ml-cryptanalysis: `docs/proofs/N8-growth.md` (all), `docs/proofs/U1-upper-bounds.md` (all),
  `experiments/ic-bounds-k14/RESULTS.md` (all), `experiments/ic-bounds-binary-deep/PLAN.md` (all),
  `experiments/ic-bounds-binary-deep/predictions-2026-11-12.json` (cells C1–C4b),
  `experiments/ic-bounds-binary-law/laws.md`, `experiments/ic-bounds-binary/README.md` (rows for m = 3),
  `scripts/ic_bounds_growth_lib.py` (header and API), `csd/ic_bounds_big.py` (function index).
- crypto: `docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md` (Frontier rows 1–119), `WEAKNESSES.md` §5–6,
  `docs/ic/LEADERBOARD.md` (progress table, Hilbert-stop component, JV sieve section),
  `docs/ic/factor-base-encyclopedia/README.md` (§0.1, §0.2, A10–A13, D4, E2–E6),
  `research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md` (the null, early-exit and staircase passages),
  `research/full_volcano_graphwide_ic_20261017/README.md`,
  `papers/ecdlp-index-calculus-treatise/figures/data-bin-fd-instances.json` (R33a/b, R35a/b), and the `AGENTS.md`
  sections named in the role.
- Not read: the treatise LaTeX, `LEMMA_LINKAGE.md`, `LITERATURE_SEARCH.md`, `fall-degree-lemmas.json`,
  `lit-groebner.json`, and the Rust engines in `/home/user/cryptanalysis`.

---

## Memo

### The connection

N8 proves that the last fall degree (LFD) grows. Its argument is *locality*: a low-weight polynomial vanishing on [pcc:unverified]
the curve `S_3(x1, x2, t) = 0` cannot connect exponents that lie `2^k` apart. The base's defining relation
`x^N = 1` (the *wrap*) is long-range in the coordinate where `S_3` is a quadric, so it can be crossed only by a
derivation whose degree grows with k (`N8-growth.md` §0, T1–T2, §3.2).

Read across the lanes, the theorem needs three conditions that it never names as such:

1. **One equation.** The variety cut out before the grid relations is a curve.
2. **Large characteristic.** In characteristic l, Theorem 3.3 measures exponent distance modulo l, and the
   Frobenius images of `g` are `F0`-proportional to `g` (N8 §3.2 and the control row of §6.1).
3. **A specific presentation.** The standard monomials are 𝒮-units with a bounded 𝒮 (Lemma 3.2).

Binary descents break the first two conditions. They carry n Frobenius-conjugate equations, and in characteristic
2 every pole order `2a` is 0 mod 2. The fourth fact on disk does not fit the third condition. The **Dickson
tower**, whose monomials are not 𝒮-units (each `T_{2^j}(x)` has `2^j` distinct zeros), "gives the same D, width
and step count as kummer at every N" (encyclopedia §0.1). What Kummer and Dickson do share is the associated
graded algebra of the tower: every square is a weight-dropping carry (`y_j² = y_{j+1}` against
`y_j² = y_{j+1} + 2`).

The **Vélu isogeny tower** (flagship 0.2) is the one flagship whose graded algebra differs. Its top-weight
reduction is `y_j² ≡ y_j·y_{j+1}`, with no weight drop below the top level (derived from the Vélu step in
encyclopedia §0.2). This gives one well-posed transfer, testable cheapest-first: is the prime law a function of
(characteristic, number of independent equations, graded tower algebra), and which of the three carries it?

The lanes could not see this alone, for these reasons:

- The fall-degree lane proved locality for one presentation. [pcc:unverified]
- The PKM lane recorded "null equals tower" and "dickson equals kummer" as instrument facts.
  - The null is *predicted* by N8: a random element on the same support `{0,1,2}²` defines another genus-1 (2,2)
    curve.
  - Dickson = Kummer is *not* covered by N8.
- The binary lane fitted laws without reference to either.

### Q1: which factor-base structures could have bounded LFD (falsifiable criterion)

- **Proved necessary condition.** Proposition 2.2 of N8 needs only a linear lift `f ↦ f̃` to functions on the [pcc:unverified]
  curve with `1 ↦ 1` and `g̃ ∈ 𝔭`. It therefore holds for any tower presentation. If the no-wrap ideal space
  `S_D^B(k)` is closed under the degree-D rule, then `1 ∉ V_D(k)`. **So bounded LFD at degree D requires
  `S_D^B(k)` to be non-closed at every large k.**
  - For Kummer, closedness starts at k = 4, 8, 12 for D = 3, 4, 5 (N8 E1).
  - Testing it costs a kernel of an evaluation matrix, not a closure. At D = 5, k = 12 it took 12,951 monomials
    and 665 s (N8 §6.2), against a closure in 55,455 columns.
- **Structural conjecture to screen.** `S_D^B` closes at finite depth, so LFD is unbounded, whenever the system has
  one equation, characteristic above `2^(k+1)`, and a square-zero graded tower algebra. The briefing's candidates
  map onto this as follows:
  - *Torsion-point images, i.e. the Vélu tower `x(R' + ⟨Q⟩)` (0.2).* These violate square-zero, and their poles
    spread over `2^j` points. **The only flagship candidate.**
  - *Additive subgroups via linearised polynomials.* These exist only in small characteristic, so they violate
    condition 2. This is the binary lane. `F_p` has none.
  - *Images under small-degree isogenies of the target.* These are bijective ("image = all of E₂(F_p): no
    filtering", A10). Genus, 𝒮 and graded algebra are unchanged, so there is no escape. This agrees with A11:
    "solving degree does not differ".
  - *Fixed-degree map images `x = φ(y)`, `y ∈ μ_{2^k}`.* These have one equation, a square-zero algebra, and a
    bounded 𝒮 on the pulled-back curve, so they are predicted to close.
  - *"Small circuits with S_3".* Operationally this is exactly "`S_D^B` not closed", which is the computable form.
- The screen is **LEAD-SYN-nowrap-closedness-screen**. Its discriminating pair is matched on generator weight: a
  degree-2 map image (square-zero, generator weight 4) against the Vélu tower (not square-zero, generator weight 4).

### Q2: why binary steps slowly while prime steps every four depths

**The premise needs one correction: binary is not flat.**

- In the treatise's two-summand binary descent with 2k Boolean variables, R33a/b (n = 33, k = 17) has `1 ∈ W_4`,
  so LFD = 4.
- R35a/b (n = 35, k = 18) has `W_4` without 1, certificate `VALID`, `codim_W4_ge` 5287, so LFD ≥ 5
  (`data-bin-fd-instances.json`).
- At the same variable count 2k, the prime law gives LFD 7 at k = 17 and 8 at k = 18 (law; 8 is conjectural).
  For the prime unit instance, 1 ∉ V_6(17) follows from the k = 14 certificate by the kernel depth restriction
  (`ic-bounds-k14/RESULTS.md`).
- *Derived*: the prime law steps once every 8 chain variables (first variable count with LFD ≥ D is 8D − 28: 12, 20,
  28 for D = 5, 6, 7). The binary steps sit near 12, 16 and 36 variables (lane A, A3: `v_c` ≈ 9–10, 13–14, 33–34).
- Today's k = 11 cell (LFD 4 on 7 affine systems) lies inside binary's D = 4 plateau, which A3 already placed at
  v ≤ 33. The briefing says "laws predicted 5", but only law B did (B, post-hoc, A and A_Z gave 5/4/4/4 in
  `predictions-2026-11-12.json`).
- B's third threshold, k + e ≥ 19, was fitted to the same treatise points that A3 reads as a step in the variable
  count. The deep-chain plan does not cite A3, and A3 does not cite law B.

Three structural explanations compete:

- **F (characteristic):** in characteristic l, locality sees exponents only modulo l. Characteristic 2 annihilates
  every power-of-2 distance, and the n descent equations are the Frobenius orbit of one equation.
- **Dm (number of equations):** prime m = 2 is one equation in two grid coordinates, so its pre-grid variety is a
  curve and Brownawell–Masser applies. A descent is n equations, its zero set is finite, and the closure is
  governed by the regularity of Z. This matches "LFD ≤ 4 when |Z| ≤ 470", which holds on 1458/1458 systems, and
  the k = 10 grid rule, 489/489 (`laws.md`).
- **S (combinatorics):** the law is a characteristic-free property of the support and the ring. N8 is then a valid
  proof device but not the cause.

The discriminating experiment is **LEAD-SYN-char-and-generator-crossing**. It runs the existing μ_{2^k} engine
unchanged with the modulus set to a small prime l < 2^(k+1). The closure needs only the coefficient field of g;
"ζ enters only through V, which the system never contains" (encyclopedia §0.1). A second arm adds a second
generator modulo a large prime. The four hypotheses predict four different patterns.

### Q3: one measurement that closes two open cells

The suggested pairing (0.2 with E3, via moving the instance in its isogeny class) shares no lever under this
picture:

- Every input to the LFD is invariant when the *target* moves within its class: the genus of the Semaev curve,
  𝒮, the graded algebra, the characteristic and the number of equations.
- A11 records "top_form(S₃) is independent of (a, b), and the solving degree does not differ", with floor and
  crater F4 degree about 3 on both.
- The volcano pilot's 19.2× is a "quotient of incomplete cold GAE lower bounds; diagnostic" (LEADERBOARD
  line 28). That is a constant-level number.
- **A contradiction between records.** Lane P's P4 takes 0.2 to be φ(U) for an isogeny φ: E′ → E. The
  encyclopedia (§0.2) and the briefing label define 0.2 as the abscissa set `x(R' + ⟨Q⟩)` of an *auxiliary* curve
  E′, unrelated to E, presented by a Vélu x-map tower. P4's transport lemma fits the D7/A11 reading, not that one.
  To resolve it, the coordinator reads the atlas body of 0.2 (vault access, a five-minute check). I use the
  encyclopedia definition.

The measurement that does close two cells is **LEAD-SYN-velu-tower-exact-lfd**: the exact closure LFD of the
Vélu tower on unit instances at k = 4..10, with k = 14 as stage 2. It decides two things.

- **(a) The open solver axis of 0.2.** The encyclopedia says "whether D stays bounded as N grows ... remains the
  gate". The only data are F4 degrees D = 5 at N = 4..18 and 6 at N = 20.
- **(b) Whether the prime law is carried by the graded tower algebra or is presentation-free.**
  - If presentation-free, the only escapes are more equations or small characteristic, and neither exists over
    F_p at m = 2. Then the summation-polynomial route is closed on the solver axis for every tower.
  - If carried by the graded algebra, it is the design knob and the Q1 screen becomes predictive.
  - 0.2 is also "the only family that can support a statement about generic primes" (PKM note §4.5), so the
    P-192 route rides on this result.

### Q4: exact fall degree as a speedup, not a lower bound

Mostly it becomes a decision.

- The main session's entropy bar (`out/main_session.md`) puts the column-count threshold at LFD/k = 0.220. That is
  the same constant as U1's counting obstruction, `D*(k)/k → 2·H⁻¹(1/2) = 0.2200` (U1 §6.2).
  - *Derived*: both say the Macaulay matrix at the LFD has at least about |B| columns. For proper ideals with
    p ≈ 2^k this follows from counting (codim V_D = |Z| ≈ 2^k). For unit ideals it follows from locality plus the
    measured slope.
  - On disk: 60,460 = 2^15.9 columns at k = 10 and 1,683,218 = 2^20.7 at k = 14, against |B| = 2^10 and 2^14.
  - This agrees with the root-finding oracles' "4.6·10¹¹× less counted work than F4 at m = 3, N = 18"
    (encyclopedia §0.1).
- Decision: no further tower-F4 compute at m ≤ 4 to rescue 0.1. This avoids the "derived and not run" 130–380 GB
  cells, unless LEAD-SYN-velu-tower-exact-lfd finds a different graded algebra with a flatter law.
- The constructive routes are already proposed elsewhere:
  - the free-extension lift (lane P, P2; main session item 4);
  - the quotient law q ≈ 3–6·B (lane P, P1);
  - the Hilbert stop on the cover arms (lane C, Leads 1–2; held at Nagao scale, S × 0.528).
- I checked two more routes and found nothing new (see "Considered"):
  - a profile-driven F4 stop on the tower, which lane P already rejected as a repeat of P2;
  - a Hilbert stop for the char-2 D4 Macaulay-11 solver, where at most 3 of 387 rows are redundant.
- The one new conversion I can offer is instrumental. The closedness kernels of the Q1 screen certify `1 ∉ V_D(k)`
  without a closure, so the lower half of the law (L_D) can be extended to larger depths cheaply. It is part of
  LEAD-SYN-nowrap-closedness-screen, not a separate record.

---

## Idea records

```yaml
id: LEAD-SYN-char-and-generator-crossing
title: >
  Is the prime LFD law a property of one equation in large characteristic? Run the mu_{2^k} closure modulo small
  primes l < 2^(k+1), and with a second generator modulo a large prime
claim: >
  Hypothesis L: the N8 locality mechanism is what produces the measured unit-instance law
  LFD(k) = floor((k+14)/4) of the normalised two-summand mu_{2^k} chain (generator NF(S_3(x1, x2, t)), t != 0,
  nonsingular curve). The law then holds only when (i) the system has one equation, so its pre-grid zero set is the
  curve S_3 = 0, and (ii) the characteristic exceeds 2^(k+1).
  Pre-registered decisive cells (the law's answer in brackets):
    C-c17:  over F_17,    inputs {g, sigma_17(g)},  k = 6:  1 in V_4(6)   [law: 1 not in V_4(6)];
    C-c97:  over F_97,    inputs {g, sigma_97(g)},  k = 6:  1 in V_4(6)   [not];
    C-c257: over F_257,   inputs {g, sigma_257(g)}, k = 10: 1 in V_5(10)  [not];
    C-d1:   over F_12289, inputs {g, h}, where h is a random element supported on exponent pairs {0,1,2}^2 with no
            common zero with g on mu_{2^k}^2, k = 10: 1 in V_5(10) [not];
    C-d2:   over F_12289, inputs {g, sigma_s(g)} with s = 1 + 2^(k-2) (exponent multiplication), k = 10:
            1 in V_5(10) [not].
  Here sigma_l(g) = g^l in F_l[G], the Frobenius of a commutative group algebra of characteristic l (coefficients in
  F_l). It multiplies exponents by l mod 2^k. It lies in the ideal and vanishes on the same curve, because
  S_3(x1^l, x2^l, t) = S_3^l, but it links exponents about 2^log2(l) apart at weight <= 4.
  Derived: x^17 = u_0 u_4 and x^34 = u_1 u_5 at k = 6; 97 = 33 mod 64, so x^33 = u_0 u_5 and x^66 = u_1;
  257 = 2^8 + 1, so x^257 = u_0 u_8 and x^514 = u_1 u_9 at k = 10.
  Three competing hypotheses are registered with their own patterns: Dm (only the number of equations matters),
  F-links (only long-range low-weight ideal elements matter), and S (the law is free of characteristic and of
  generator count).
mechanism: >
  N8 Theorem 3.3 bounds ||2(a - a')||_p for the exponents of a circuit. Theorem 3.1 uses F0 = F^p, so in
  characteristic l the wrap distance N = 2^k is seen only mod l, and for l < 2^(k+1) it becomes short (derived:
  2*64 = 9 mod 17; 2*1024 = 249 mod 257, which is -8). The control row of N8 section 6.1 shows the S_3(x^p)
  circuit for the same reason. Binary descents are the extreme case l = 2 (every 2^j distance is 0 mod 2) and carry
  n conjugate equations. Separately, with a second independent equation the pre-grid variety is finite and there is
  no curve for Brownawell-Masser to act on.
  Engine fact: the closure in A_k = K[(Z/2^k)^2] needs only the coefficient field of g (encyclopedia section 0.1:
  zeta "enters only through V, which the system never contains"). The kernel bound LFD <= k+2 holds "every field
  with 2 != 0" (briefing). Arms (b) and (c) therefore run on the existing engine with the modulus set to l.
  Interpreting an F_l closure as the LFD over F_{l^n}, where mu_{2^k} lives, rests on field-extension invariance of
  rank and needs the bridge stated for that field (see lean_obligation).
prediction:
  boundary: "arm (a) control: the law (k: 4,5,6,7,8,9,10 -> LFD 4,4,5,5,5,5,6); unit instances only"
  table: |
    cell     | L (curve AND large char) | Dm (equations only) | F-links (long-range elts) | S (free)
    C-c17    | 1 in V_4(6)              | not (law)           | 1 in V_4(6)               | not
    C-c97    | 1 in V_4(6)              | not                 | 1 in V_4(6)               | not
    C-c257   | 1 in V_5(10)             | not                 | 1 in V_5(10)              | not
    C-d1     | 1 in V_5(10)             | 1 in V_5(10)        | not                       | not
    C-d2     | 1 in V_5(10)             | 1 in V_5(10)        | 1 in V_5(10)              | not
    reported, not scored: arm (b) (one generator mod l) at k = 4..10, which under L may or may not fall below the
    law depending on whether the closure derives sigma_l(g) itself; arm (d) at k = 6..9.
  ratio: "per cell, (min D with 1 in V_D) minus the law value; under L the ratio is <= -1 on every decisive cell"
  scoring: >
    A cell reads "below law" if 1 enters below the law degree on >= 3 of 4 instances, "law" if it does not enter on
    >= 3 of 4, and "mixed" otherwise (mixed cells are not scored). A hypothesis is supported if it matches every
    scored decisive cell. If none matches, the result is "mechanism open" and the pattern is reported.
discriminating_test: >
  Engine: scripts/ic_bounds_growth_lib.py (closure, profile, verify_closed) for <= 12,616 columns, and
  csd/ic_bounds_big.py (cpu or H200) for 21,700 columns (k = 10, D = 5) and 60,460 columns (k = 10, D = 6, needed
  only to certify unit status where 1 is not in V_5(10)). Four curves per l with all nine S_3 coefficients nonzero
  mod l and t != 0.
  Arms:
    (a) mod 12289 and mod 40961, one generator (reproduce the recorded profiles: V_5(10) = (0,0,2,138,3163,18569));
    (b) mod l in {17, 97, 257}, one generator, k = 4..10;
    (c) the same with sigma_l(g) added;
    (d) mod 12289, k = 6 and 10, with h (d1) or sigma_s(g) (d2).
  Unit status: 1 in V_D at some D <= k+2.
falsification: >
  L is falsified if any scored decisive cell reads "law". Dm is falsified if C-c17, C-c97 or C-c257 reads "below
  law". F-links is falsified if C-d1 reads "below law" or a C-c cell reads "law". S is falsified if any decisive
  cell reads "below law" while arm (a) reproduces the law on the same engine build. If arm (a) fails to reproduce
  the recorded profile hash, the run is void: an engine fault, not evidence.
cost_band:
  arms:
    - "(a) control: 8 closures <= 21,700 columns; reuse the recorded GPU profiles as reference; < 1 CPU-h or minutes on H200"
    - "(b)+(c): 3 l x 2 arms x 4 instances x k in 4..10 at D <= 5; columns 386 to 21,700; about 6-10 CPU-h single-thread (the k = 10 cells dominate)"
    - "(d): 2 arms x 4 instances x k in {6, 10}; about 2 CPU-h"
    - "unit certification at k = 10, D = 6 (60,460 columns) for cells reading 'law': CPU reference engine about 3,129 s each (RESULTS.md: k = 10, D = 6 reference engine) or about 73 s on H200 at $5.29/h; at most 24 cells, so <= 21 CPU-h or <= $5 GPU"
    - "engine change: multi-input option for (c)/(d) if the drivers take a single generator (about 20 lines); the language rule for landed code applies, so the director decides whether this is a research-engine patch in ml-cryptanalysis or a Rust port"
    - "section 5a: no cross-method ECDLP comparison is made; not applicable (no rho arm)"
  total: "<= 35 CPU-h or <= 15 CPU-h + <= 1 GPU-h; memory <= 4 GB per k = 10 cell (dense 21,700^2 int64 = 3.8 GB, derived)"
controls:
  positive: "a generator-count ladder (2k random weight-2 elements) must make 1 enter at low D (instrument power, as PKM section 5.3)"
  negative: "(d0) h = x1*g (a multiple of g, weight 3) must leave the law unchanged: adding an input in itself changes nothing"
  aa: "two independent instance draws per cell; the recorded instance-independence of profiles (three primes, unit and proper) predicts equal profiles"
risks:
  - "Prior against L at k = 6: the PKM generator ladder at N = 12 gave isogeny D = 5,5,5,5,4 for g = 1,2,4,8,12 (encyclopedia section 0.2), so one extra random quadric did not lower F4's D. That was F4 on a proper instance, so the k = 10 cells carry the decision."
  - "N8's quantitative range starts at k = 39 (D = 3). Whether the law's early steps at k = 6 and 10 are locality-driven at all is what is being tested; a uniform 'law' outcome is informative (S)."
  - "For small l, accidental coefficient vanishing can make S_3 degenerate; the instance filter requires all nine coefficients nonzero."
  - "A driver may assert 2^k | p-1 when building instances; call the library closure directly (only the modulus is used)."
closest_baseline: >
  The law and its data: N8 section 6.2, ic-bounds-k14/RESULTS.md, U1 section 2.7. The characteristic-p circuit test
  in N8 section 6.1 (ic_bounds_growth_charp.py, p = 101, 103, 127) tests Theorem 3.3 on circuits, not the LFD.
nearest_existing_lead_and_difference: >
  LEAD-ICE-curve-presentation-invariance and LEAD-ICP-extension-field-embedding (presentation and field changes);
  main_session item 2 and lane P's LEAD-P-kummer-radix-presentation (radix). None of them varies the characteristic
  below 2^(k+1) or the number of equations at fixed presentation, and none uses the four-pattern scoring.
lean_obligation:
  - "Closure dimensions are invariant under field extension (rank over F_l equals rank over F_{l^n}), and ChainClosureBridge.chain_mem_closure_iff holds for any field containing mu_{2^k} with 2 != 0. Needed to read an F_l closure as the LFD over F_{l^n}."
  - "sigma_l(g) = g^l in F_l[G] (Frobenius on a commutative characteristic-l monoid algebra; ring identity)."
  - "None for the measurements, which are certified computations."
next_steps:
  - "1. Core (frontier): freeze the instance lists, the scoring rule and the multi-input decision; 2 h."
  - "2. Experiment director (Sonnet): write the frozen card, arms (a)-(d) at k = 4..10, D <= 5, D = 6 only for unit certification; the director chooses and freezes the route (local CPU or isolab-eks for CPU cells, the rented H200 for k = 10 if used); fill the resume checkpoint first; 0.5 day."
  - "3. Executor (Haiku): run the card unchanged; <= 35 CPU-h; records with profile and basis hashes."
  - "4. Experiment director: score the pattern table; research-visuals report (table, graph, PDF); 2 h."
  - "5. Core: if L or Dm wins, write the mechanism note for N8 section 7 and the binary lane; if S wins, card the binary multiplicative-base cell (mu_{3^k} cubing tower over F_{2^n}, one generator) as the follow-up; 0.5 day."
```

```yaml
id: LEAD-SYN-velu-tower-exact-lfd
title: >
  Exact closure LFD of the Velu isogeny tower (flagship 0.2) on unit instances at k = 4..10 (stage 2: k = 14):
  one measurement for 0.2's solver axis and for whether the prime law is presentation-free
claim: >
  Normalised Velu tower x(R' + <Q>), |V| = 2^k, on an auxiliary curve E': the supersingular y^2 = x^3 + x with
  p = 3 mod 4 and v2(p+1) >= 15 (one draw, PR #1947), and one ordinary E'. Target curve E random over the same
  p, about 2^31; unit instances only. The exact LFD (local closure with certificate, as in RESULTS.md) is reported
  against two references on the same engine: the Kummer law (steps at k = 6 and k = 10) and the weight-matched,
  square-zero map-image base x = y^2 + alpha*y (generator weight 4, like the Velu generator; derived: x1^2 x2^2
  reduces to weight 4 in both).
  Pre-registered three-way reading of the step pattern of LFD_velu(k) - LFD_mapimage(k) over k = 4..10:
    H_same:  constant, i.e. the same steps;
    H_flat:  LFD_velu has at least one step fewer in k = 4..10, and at stage 2 no step at k = 14 where Kummer
             steps 6 -> 7;
    H_steep: at least one step more.
  Prior from F4 (an upper bound on the LFD under THM-F4-CONTAINMENT): Velu D = 5 at N = 4..18 and 6 at N = 20,
  while Kummer has 4 (N <= 10), 5 (N = 12..18) and 6 (N = 20) (encyclopedia section 0.2). F4 thus shows one step
  where Kummer shows two. Registered probabilities: H_same 0.45, H_flat 0.35, H_steep 0.20.
mechanism: >
  The Velu step y_{j+1}(y_j - xi_j) = y_j^2 - xi_j y_j + tau_j gives y_j^2 = y_{j+1} y_j - xi_j y_{j+1}
  + xi_j y_j - tau_j. The top-weight reduction keeps weight 2, so the associated graded algebra is not square-zero
  (y_j^2 = y_j y_{j+1} in gr), unlike Kummer and Dickson, where every square is a carry. As functions on the Semaev
  curve the Velu tower variables have poles at the 2^j kernel abscissae, so N8's S-unit hypothesis fails (the
  locality proof says nothing). The Dickson tower also fails the S-unit hypothesis yet equals Kummer in D, width
  and steps, because it shares Kummer's graded algebra. So the Velu tower is the one available presentation that
  separates "the law is carried by the graded algebra" from "the law is presentation-free".
prediction:
  boundary: "unit instances, two summands, normalised presentation, p about 2^31, k = 4..10 (stage 1) and k = 14 (stage 2)"
  table: "LFD_kummer = 4,4,5,5,5,5,6 (k = 4..10, law); LFD_mapimage measured on the same engine (SYN-screen run); LFD_velu measured; the H_* reading is on step positions, offsets allowed"
  ratio: "dim V_5^velu(10) / dim V_5^kummer(10) = x / 18,569; under H_same with an equal graded closure, 1.00"
discriminating_test: >
  Port: a tower closure engine whose shifts are the reduced tower products (sparse, not permutations) and whose
  certificate is the codim / |Z| criterion. It reuses the f4_tower_generic tower tables (Rust; the no-Python rule
  for landed code), with Kummer as the parity case: it must reproduce V_4(k) dims 163, 386, 605, 829, 1061, 1301
  (k = 4..9) and V_5(10) = 18,569 with profile (0,0,2,138,3163,18569).
  Stage 1: k = 4..9 at D <= 6, and k = 10 at D = 5, 6, for 3 instances x 2 auxiliary curves.
  Stage 2, only if stage 1 is H_flat or H_same: k = 14, D = 6 closure plus the D = 6 -> 7 lift, as in RESULTS.md,
  on 2 instances.
falsification: >
  0.2 is closed on the solver axis on the same footing as 0.1 if H_same holds at k = 4..10 and stage 2 gives
  1 not in V_6^velu(14) with a step at 14 (in which case the law is presentation-free and SYN-screen's structural
  conjecture loses its knob). H_flat is falsified by any step of LFD_velu at a k where the map-image control does
  not step, or by a step at k = 14. Engine fault: the parity case fails to reproduce the Kummer dims and hashes.
cost_band:
  arms:
    - "port (Rust, tower shifts, evaluation-point kernel shared with SYN-screen): core coding session, 1-2 days; counted once for both records"
    - "stage 1 runs: k <= 9 minutes each; k = 10, D = 5 (21,700 columns) and D = 6 (60,460) about 1-3 CPU-h per instance (denser shifts than Kummer, estimate 1-4x); 6 instances, <= 18 CPU-h"
    - "Kummer parity and map-image control on the same build: <= 4 CPU-h"
    - "stage 2 (conditional): k = 14, D = 6 closure, Kummer 6,573-8,077 s on H200 (RESULTS.md), Velu estimated 1-4x, plus lift 677-751 s x 1-4; 2 instances, about 4.5-18 GPU-h at $5.29/h = $24-95, plus setup; needs the shift port in the GPU engine (+1 day)"
    - "unit verification by enumeration of V x V: 2^20 pairs (k = 10) seconds; 2^28 pairs (k = 14) minutes"
    - "section 5a: no cross-method ECDLP comparison; not applicable"
controls:
  positive: "the Kummer tower and the Dickson tower on the new engine reproduce each other and the recorded law (Dickson = Kummer is the on-disk control)"
  negative: "the map-image base (square-zero, weight-4 generator) must follow a Kummer-like step pattern (SYN-screen prediction); if it does not, the weight-matched reference is void and the comparison is against Kummer only"
  aa: "supersingular and ordinary E' at equal (p, k): the PR #1947 finding (supersingular D = ordinary D) predicts equal LFD"
risks:
  - "The generator weight differs from Kummer's (4 against 2), so absolute LFD values are not comparable; the reading is on step positions, and the map-image arm is the weight-matched reference."
  - "Stage 1 sees at most two Kummer steps; a difference in the first step alone (k = 6) may be a small-k offset. Stage 2 is the confirmatory cell."
  - "The tower port is real implementation work; if it slips, F4 with THM-F4-CONTAINMENT can only bound the LFD from above."
closest_baseline: >
  Encyclopedia section 0.2 (F4 D on the raw Velu presentation, p0 = 786433, N <= 20; ecbench pricing 21.0-136.5x
  rho) and PR #1947 (supersingular auxiliary curve). The Kummer law: N8, U1, RESULTS.md.
nearest_existing_lead_and_difference: >
  LEAD-ICP-pkm-tower-algebraic-oracle and LEAD-ICP-isogeny-transported-relation-yield (F4 degree and yield);
  lane P's LEAD-P-isogeny-transport-is-plain-base reads 0.2 as phi(U) under an isogeny into E's class, which
  contradicts encyclopedia section 0.2 (see memo Q3); its class-wide invariance test does not touch the Velu
  presentation. This record measures closure-exact LFD on the Velu presentation against a weight-matched
  square-zero control, which none of these does.
lean_obligation:
  - "N8 Proposition 2.2 for tower presentations with a linear lift (shared with SYN-screen): small, a few hundred lines, no computation."
  - "The closure bridge for tower quotients (ordinary degree-D closure in the polynomial ring iff the normal form lies in the quotient closure), the tower analogue of ChainClosureBridge.chain_mem_closure_iff: needed before any LFD value here is cited as a kernel-backed lower bound."
next_steps:
  - "1. Core coding session: tower closure engine in Rust with the Kummer parity test and the Dickson control; 1-2 days."
  - "2. Experiment director: frozen card for stage 1 (instances, p, E' choices, the H_* reading rule); resume checkpoint; 0.5 day."
  - "3. Executor: run stage 1; <= 22 CPU-h."
  - "4. Experiment director: assess; research-visuals report; decide stage 2 only on H_flat or H_same; 2 h."
  - "5. Core: on H_same plus stage 2 step, record 0.2 closed on the solver axis on the same law as 0.1 (the coordinator changes status, not this role); on H_flat, card the SYN-screen tier-2 families and a third-step test at k = 18 on the GPU."
```

```yaml
id: LEAD-SYN-nowrap-closedness-screen
title: >
  A computable criterion for bounded LFD: the closedness depth kappa_B(D) of the no-wrap ideal space S_D^B(k),
  screened over six base presentations
claim: >
  (Proved necessary condition; generalisation of N8 Proposition 2.2.) For any tower presentation B with a linear
  lift f -> f~ to functions on the Semaev curve (1 -> 1, g~ in the curve ideal), if S_D^B(k) is closed under the
  degree-D rule then 1 is not in V_D(k). Hence bounded LFD at degree D requires kappa_B(D) = infinity, where
  kappa_B(D) = min{k0 : S_D^B(k) closed for all k >= k0}.
  (Conjecture to screen.) kappa_B(D) is finite whenever B has one equation, characteristic > 2^(k+1) and a
  square-zero graded tower algebra.
  Pre-registered table:
    Kummer, g = 1 (control):     kappa(3) = 4, kappa(4) = 8, kappa(5) = 12 (recorded, N8 E1);
    Kummer coset, g != 1:        identical (N8 Remark 2);
    Kummer, t = 0:               kappa(3) = 5 (recorded: "S_3(k) closed from k = 5 instead of 4"), kappa(4) in {8, 9, 10};
    map image x = y^2 + alpha*y: finite; kappa(4) <= 10, kappa(5) <= 14 (weight-4 generator; D >= 4 only);
    Dickson tower:               equal to Kummer (same graded algebra);
    Velu tower (supersingular and ordinary E'): the discriminating row. G (the graded algebra carries the law)
      predicts kappa_velu(D) >= kappa_mapimage(D) + 2, or not closed through k = 12 at D = 4, 5. S predicts
      |kappa_velu(D) - kappa_mapimage(D)| <= 1.
mechanism: >
  N8 Proposition 2.2 uses only that S_D contains g, that it is closed, and that 1 is not in the curve ideal; the
  multiplicativity of the lift is never used (N8 section 2). Closedness is a statement about low-weight functions
  on one curve, computable from the kernel of an evaluation matrix at more points than the degree bound (N8 section
  6). The structural conjecture names the three conditions behind the locality proof (memo Q1). The weight-matched
  pair (map image against Velu) isolates the graded algebra from the generator weight.
prediction:
  boundary: "two summands, unit and proper instances alike (S_D is instance-level), p about 12289 to 2^31, D in {3, 4, 5}, k <= 9 for D <= 4 and k <= 12 for D = 5"
  table: "the pre-registered table in the claim"
  ratio: "kappa_B(D) - kappa_kummer(D) per family; 0 for coset and Dickson, <= 2 for map image and t = 0; Velu is the measured row"
discriminating_test: >
  Tier 1 (generator change only, existing kernels: ic_bounds_growth_ideal.py and ic_bounds_growth_fei.py style):
  Kummer g != 1, t = 0, and the map image (evaluation points on S_3(phi(y1), phi(y2), t) = 0 by root-finding). Lane
  P's P5(b) runs the same map-image generator for its quotient test; share the instances.
  Tier 2 (tower port shared with LEAD-SYN-velu-tower-exact-lfd): Dickson, and Velu with both auxiliary curves.
  Each cell reports dim S_D^B(k), closed or not, and the number of failing rows (as N8 section 6.2 does: "12 rows
  fail under u_{i,6}").
falsification: >
  The structural conjecture is falsified by any square-zero, one-equation, large-characteristic family (coset, t = 0,
  map image, Dickson) with S_4 not closed for every k <= 12 (D = 4) and S_5 not closed through k = 12. The Velu
  escape route is closed if kappa_velu(D) = kappa_mapimage(D) +- 1 at D = 4 and 5. The necessary condition itself
  cannot be falsified by data (it is a proof); a bug shows up as 1 in V_D(k) at a k where S_D was certified closed,
  which a closure on the same instance detects.
cost_band:
  arms:
    - "Kummer control: recorded (N8 section 6.2, FEI table); one re-run per depth for engine parity, < 1 CPU-h"
    - "tier 1: 3 families x D in {3,4} x k <= 9 (<= 4,048 monomials) minutes each; D = 5 at k = 10..12 (<= 12,951 monomials, 665 s at k = 12 recorded) about 1 CPU-h per family; total <= 5 CPU-h"
    - "tier 2: 3 presentations x the same grid, after the port (counted in LEAD-SYN-velu-tower-exact-lfd); <= 6 CPU-h"
    - "section 5a: no cross-method ECDLP comparison; not applicable"
controls:
  positive: "Kummer reproduces kappa = 4, 8, 12 and the recorded dims of S_3(k) and S_4(k) (16, 41, 74, 111, 126, 130, 134 and 105, 270, 564, 1025, 1657, 2403, 3163, 3569)"
  negative: "a deliberately wrong lift (monomials evaluated with x replaced by 2x) must make g~ not in the ideal and the check fail at once"
  aa: "two instances per family; the recorded instance-independence of dims predicts equality"
risks:
  - "Necessary is not sufficient: a family with kappa = infinity in range may still grow (Crit_D or other certificates); the screen kills candidates, it does not certify them."
  - "For Velu and Dickson the Bezout point count grows with the rational degree of the tower monomials (about 2^k); at k = 12 this means about 10^4-10^5 points over F_{p^2}."
  - "If LEAD-SYN-char-and-generator-crossing returns S, the structural conjecture's motivation weakens, but the screen still decides the Velu row."
closest_baseline: >
  N8 sections 2 and 6.2 (Kummer only: closedness, FEI, Crit_D), encyclopedia A10/A12 (map images; no closedness
  data), and the PKM note's null and Dickson controls (F4 D only).
nearest_existing_lead_and_difference: >
  LEAD-ICE-curve-presentation-invariance, LEAD-ICP-algebraic-membership-factor-base (yield of map images) and
  lane P's LEAD-P-a12-degree-d-reach-and-quotient (quotient size q/B for degree-d images). This record uses
  Proposition 2.2 closedness as a presentation-agnostic screen across families, with a weight-matched
  discriminating pair, which none of them does.
lean_obligation:
  - "Proposition 2.2 (and 2.1 where a wrap formula exists) for an arbitrary prime ideal and a linear lift of a tower quotient; N8 section 8 sizes the Kummer case as 'a few hundred lines, no computation'."
  - "The structural conjecture is not to be relied on; no Lean."
next_steps:
  - "1. Core: fix the lift and point-count rules for each family, and the shared instance list with lane P P5(b); 2 h."
  - "2. Experiment director: frozen card for tier 1; resume checkpoint; 0.5 day."
  - "3. Executor: tier 1 runs; <= 5 CPU-h."
  - "4. After the tower port (LEAD-SYN-velu-tower-exact-lfd step 1): executor runs tier 2; <= 6 CPU-h."
  - "5. Experiment director: table of kappa_B(D) with failing-row counts; research-visuals report; every future prime-field factor-base proposal then reports kappa_B(4) and kappa_B(5) at intake (a coordinator decision)."
```

## Ranking

1. **LEAD-SYN-char-and-generator-crossing.** It uses an existing engine essentially unchanged and costs at most [pcc:unverified]
   about 35 CPU-hours. It is the only test whose four outcomes separate four mechanisms, and it answers the
   binary-against-prime question directly.
2. **LEAD-SYN-velu-tower-exact-lfd.** The real two-for-one. One closure series decides 0.2's open solver axis and
   whether the prime law is presentation-free. It needs a tower port, and stage 2 needs a GPU.
3. **LEAD-SYN-nowrap-closedness-screen.** It turns Q1 into a computed table with a proved necessary condition and
   gives every future prime-field base proposal an intake test. Its decisive row (Vélu) shares the port with
   record 2, so it ranks behind it.

Run order: record 1 now. Records 2 and 3 share the port. If record 1 returns S, record 2 becomes the only remaining
discriminator.

## Considered and not proposed (with the reason)

- **0.2 with E3 via moving the instance in its isogeny class (the briefing's example).**
  - Every LFD input is invariant under target isogeny moves. A11 already measured "solving degree does not
    differ", and the 19.2× volcano figure is a diagnostic quotient of lower bounds.
  - On E3, a desk argument and no new measurement: if E3's product is relations among CM ideal classes, those are
    endomorphisms commuting with Frobenius. On a cyclic prime-order group they act as scalars, so "103 relations,
    all scalar" is the expected outcome. If its product is a walk to a weaker curve, Tate's theorem fixes #E
    across the class over F_p (lane C, "Considered").
  - Statuses are left as recorded. The Lean obligation, that an endomorphism of a cyclic group of prime order is
    multiplication by a scalar, is small.
- **Profile-driven pair stop for tower F4.** This was my first Q4 candidate. Lane P rejects it as a repeat of P2
  (free-extension lift with the instance-independent profile), and the PKM F4 already has a within-step early exit
  (39% at m = 4, N = 12) and a staircase stop. A separate record would duplicate them.
- **Hilbert early stop for the char-2 D4 Macaulay-11 solver** (cross-lane transfer of the cover-lane stop).
  *Derived*: 448 columns minus a 64-dimensional quotient gives rank 384, so at most 3 of the 387 rows are
  Hilbert-redundant, under 1% of the echelon. The cost sits in charpoly (45.8%, lane B).
- **Free-extension assembly plus the degree lift as a solver for U_7 (1 ∈ V_7(17)).** This is lane P's P2 and
  main-session item 4, so it is not re-proposed. The closedness kernels of record 3 are its cheap complement for
  the lower half.
- **Asymmetric-depth hybrid** (guess the top bits of one block; N8 Remark 1 with U1's LFD ≤ min(k1, k2) + 2).
  For m = 2 the optimum is j = O(1), which is fibre enumeration at 2^k·poly, with no gain over the rootfind oracle.
  For m ≥ 3 the fibre is still a two-summand system at depth k. Nearest: LEAD-ICE-hybrid-guess-determine-l7.
- **Manufacturing a second equation over F_p for m = 2.** Every polynomial relation valid on all decompositions
  lies in the curve ideal, and by N8 its low-weight elements are local. Under L or Dm this route is closed; under S
  it is moot. Record 1 decides which.
- **A3's binary cells** (T-type n = 35, k = 17; R-type n = 34, k = 17), and an exact D = 5 closure of R35
  (*derived*: 443,704 columns at 2k = 36, about 1,000 s nominal at the plan's 0.28·N³ / 25 TFLOP/s). These are
  lane A's lead A3, not new. I recommend the director append them to the binary deep chain, because they decide
  the variable-count reading that today's C1 result already favours.
- **The binary multiplicative-base cell** (μ_{3^k} cubing tower over F_{2^n}, one generator). It is the follow-up
  if record 1 returns S. It is not proposed now because it needs a new radix-3, characteristic-2 engine, and record
  1 may make it unnecessary.
