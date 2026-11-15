> Agent report, verbatim (2026-11-12). Proposals only; nothing was run; every statement in this file is `[pcc:unverified]`.

# Lane J: Jacobians, covers, transfers and extension fields (2026-11-12)

Idea Generator output. Proposals only; nothing was run (this session had no shell, only file reads and web search), no repository file was edited. Every number marked "disk" is read from the path given and is producer-reported, `[pcc:unverified]`, unless a VALIDATION.json is named. Every number marked "derived" is arithmetic on those numbers (shown). Scope: weak, undeployed classes, plus prime-order curves over cubic extension fields as a mathematical class; no deployed curve and no key recovery is addressed.

## Sources checked

- research-kb MCP: NOT available in this session (no kb_status / search_knowledge / search_theorems tool was exposed). Novelty below is checked against the id lists in the briefing directory (atlas_lead_ids.txt, EXISTING_LEADS.txt, lane_C_prime_ext_cover.md) and repository files, plus web search. The atlas bodies are vault-encrypted and were not read, so every "nearest existing lead" below is matched by id only; the Coordinator should check the vault bodies before landing any of these. [pcc:unverified]
- Repository (read): docs/ecdlp-hardness-corpus/ATTACK_GRAPH.md (Frontier rows 2, 26, 39, 48, 76, 82), WEAKNESSES.md section 6 (rows E-RP-054, E-RP-056, E-WD-049, E-WD-070); research/weil_restriction_isogeny_census_20261018/REPORT.md (Theorem 4, Part H counts); research/tian_n3_quartic_route_20261024/REPORT.md; research/quartic_n4v2_20261020/{README.md,PROTOCOL.md,PARAMS.md}; research/jv_cover_fbline_20261102/README.md; research/jv_cover_crossover_20261101/README.md; research/ic_cover_weakclass_20261023/REPORT.md; research/joux_vitse_n1_20261015/README.md; research/jv_cover_next_leads_20261102/LEADS.md; research/weak_isogeny_classes_20261013/REPORT.md; research/pdp_literature_leads_20261022/leads/LEAD-15.verify.md; research/full_volcano_graphwide_ic_20261017/README.md; research/synthetic_volcano_ecdlp_20261018/README.md (invented data, excluded; the "19.2x" in docs/ic/LEADERBOARD.md line 28 is a distance to rho of a bounded volcano pilot cell, "quotient of incomplete cold GAE lower bounds; diagnostic", i.e. IC is 19.2x ABOVE rho, not a gain); treatise sections transfer_ext.tex (lines 1195-1520, 1640-1860), prog_openproblems.tex (problems 8, 9, 11), prime_ext.tex (standard-curve audits); docs/ic/LEADERBOARD.md (lines 1-70); research/jv_cover_next_leads_20261102/LEADS.md; lane_C_prime_ext_cover.md (to avoid overlap).
- Not found on disk: research/jv_nagao_hilbert_20261112 (the Hilbert-at-scale result of today is not on disk here); /home/user/cryptanalysis is absent from this container, so the Rust sources jv_cover.rs and jv_sieve.rs named in the briefing were located only as /home/user/crypto/src/cryptanalysis/{jv_cover,jv_sieve,gaudry_quartic,ghs_*,smith_*,hyperelliptic_index_calculus}.rs and not read in detail.
- Literature (web, standard search; abstracts and catalogue entries only, no full text):
  - S. Tian, "Cover attacks for elliptic curves with prime order" (J. Cryptology 36, 2023; titled "Cover Attacks for Elliptic Curves over Cubic Extension Fields" in some listings), arXiv 2012.07173: an F_q-rational (l,l,l)-isogeny from Res_{F_{q^3}/F_q} E to a genus-3 Jacobian, DLP in O~(q) for "some" prime-order E/F_{q^3}. https://arxiv.org/abs/2012.07173 . The one published example is q = 523, l = 3, #E = 143069629 prime (as transcribed in research/weil_restriction_isogeny_census_20261018/REPORT.md section 1, not re-read at source).
  - C. Diem, "Index calculus with double large prime variation for curves of small genus with cyclic class group", arXiv math/0606607 (verified by the 2026-10-22 mechanical check); Diem-Thome, J. Cryptology 21(4), 2008 (plane quartic, heuristic O~(q)); Gaudry-Theriault-Thome-Diem, double large prime for hyperelliptic genus 3 (ePrint 2004/153, abstract only).
  - Vitse-Wallet, "Improved sieving on algebraic curves", LATINCRYPT 2015 (about 3x on relation search of Diem-Kochinke; abstract only). https://hal.sorbonne-universite.fr/hal-01203086
  - Joux-Vitse ePrint 2010/157 and Eurocrypt 2012 (cover and decomposition); search snippets only. A search for a Nagao-type six-point decomposition on a NON-hyperelliptic genus-3 curve and for a genus-2-cover plus Nagao-descent route to E(F_{p^4}) returned nothing; that is a negative search, not a novelty proof.
  - Theta-model isogeny algorithms (Lubicz-Robert, Compositio 2012; Dartois-Maino-Pope-Robert, Asiacrypt 2024) and Weber/Fiorentino bitangent reconstruction of plane quartics are cited from memory only; they were not retrieved in this session and Proposal 3 relies on them as a plan, not as verified fact.
  - No 2024-2026 follow-up stating an existence condition or a proportion for Tian kernels was found (the 2026-10-18 census round reports the same negative search).

## What the evidence on disk says about where this lane can move

1. The weak class (E(F_{p^6}), full rational 2-torsion over F_{p^3}, #E = 4l) is exactly the l = 2 case of Tian's construction: the census proves a Tian kernel at l = 2 exists iff phi = I, i.e. full rational 2-torsion, "the exact statement behind the cover attacks' three rational points of order 2" (weil_restriction_isogeny_census REPORT, "Covers never reach them"). The invariant that confines every hyperelliptic cover to the weak class is therefore #E mod 4: it is an isogeny-class invariant (Tate), so no walk can move a prime-order curve into it. Over F_{p^6} the programme already walks non-weak CURVES inside weak CLASSES (whole-route rows 0.0098 and 0.058 of rho, treatise transfer_ext.tex lines 1785-1795; "86-95% of deeper classes hold a weak curve", line 1836), so curve-level non-weakness is solved; class-level and odd-order non-weakness is not. [pcc:unverified]
2. The only mechanism on disk that reaches prime-order curves is Tian's odd-l kernel (plane quartic codomain). Disk facts: Theorem 4 (kernel exists iff chi_phi(T) = T^2 - tT + q^3 has a root in F_l and every root is a cube, Jordan needs l = 1 mod 3); prime-order classes with a Tian kernel at some odd l <= 7: 0/8, 0/18, 24/61, 57/150, 144/323 at q = 3, 5, 7, 11, 13 (24/61 = 0.39, 57/150 = 0.38, 144/323 = 0.45, derived); a smooth quartic in the class of Res E: 26 of 26 flagged classes at q = 7..17; Jacobian mass per flagged class 24, 74, 117, 272 (about q^2.7). Missing, in the report's own words: "No Tian isogeny Res E -> Jac(C) was computed, so no attack on E/F_{q^3} was run" and "Item (b) uses generic quartics, not the item (a) classes" (tian_n3_quartic_route REPORT, "What is not established").
3. The downstream solver exists and is fast but was measured only on the GHS-weak family: N4v2 DL exponent 0.983 [0.975, 0.993] over k = 12..22, DL / rho-neg = 0.005 [0.004, 0.008] at q = 2097091, one DL solve 12.3 CPU-s against 616 CPU-s (rho-neg) at q = 4194271, DL1 = 0.536 of DL (quartic_n4v2 README). Its scope line reads "GHS-weak family E0/F_{q^3}, #E0 = 4 ell ... No generic-curve, prime-order ... claim". The Tian-route measurement on generic quartics stopped at q = 8191 with dense linear algebra and an unresolved band (0.7-8.0x rho at w = 0.1, 5.7-32.5x at w = 1; IC 180-340 GAE at w = 0) (tian_n3 REPORT, item b).
4. The Diem arm's frozen FB constant c_D = 2.0 was picked from {1.5, 2.0, 2.5, 3.0}; at k = 16 the cost was 3.87e7 (c = 1.5, "one failed attempt each, charged"), 3.34e7 (2.0), 3.97e7 (2.5), 5.07e7 (3.0); at k = 18: 2.09e8, 1.81e8, 2.15e8, 2.70e8 (quartic_n4v2 PARAMS.md lines 16-19). c = 1.75 (the retry step) was never run as a first attempt.
5. At the scale of the weak class (q = p^2 = 2^21), the validated sieve row is 0.0065x matched negation rho at p = 1511 (VAL-20261103-c31c76, quoted in lane C), and N4v2 DL is about 0.005x at q = 2^21: the same order of magnitude, on groups of the same size, by two different routes.

---

## Proposal 1 (cheapest, decisive for the transfer lane)

```yaml
id: LEAD-J-quartic-dl1-generic-jacobian
title: N4v2 DL1 on random cofactor-one plane quartics, to see whether the 0.005x-rho solver depends on the GHS-weak family
obstruction_attacked: >
  The Tian route's downstream cost on non-weak (cofactor-one, prime-order) Jacobians is unmeasured at scale.
  tian_n3 item (b) stops at q = 8191 with dense linear algebra (IC/rho unresolved across the band:
  0.7-8.0 at w = 0.1, 5.7-32.5 at w = 1), while N4v2 (SGE + scalar Lanczos + double large primes) is measured only
  on E0/F_{q^3} with 4 | #E0 up to q = 4.2e6. Without this row, any claim that a transfer to a plane quartic
  would put a prime-order group below rho rests on an extrapolation across a family boundary. Maps to atlas cell E6
  (genus-3 cover, IC with two large primes) and to HEUR-WD-DT-ASSUMPTION12 (frontier row 26) on non-weak instances.
claim: >
  On random smooth plane quartics over F_q (q prime, q = largest prime below 2^k, k = 12, 13, ..., 18, 3 curves
  per k, Jacobian order #J with a prime factor l >= #J / 8, planted DLP in the order-l subgroup), the unchanged N4v2
  DL1 pipeline (c_D = 2.0, double large primes, SGE, scalar Lanczos) solves and verifies 100% of instances, its
  cold M-equivalent exponent in q over k = 12..18 lies in [0.93, 1.10], and DL1 / matched negation rho
  is at most 0.10 at k = 16 at each of w = 0, 0.1 and 1 with the order holding across the band.
mechanism: >
  Diem's algorithm uses only lines through pairs of rational points and the Jacobian class key; nothing in the loop
  uses 4 | #E0 or the GHS construction. The weak family enters only through the instance builder and through the
  cofactor (4) on the target. A cofactor-one random quartic has #C(F_q) = q + 1 - c1 with c1 random, so the base
  size N = ceil(2 sqrt q) is a fixed fraction of the points either way. The tian_n3 band loss is the dense
  O(B^3) elimination, which N4v2 replaced (L1/S linear algebra 0.20-0.22 on the weak family).
prediction:
  verified_fraction: "1.00 (every unit verified; N4v2 had 501/501)"
  exponent_DL1_k12_to_k18: "1.00 [0.93, 1.10]  (N4v2 weak family: DL1 0.938 [0.927, 0.950] full range, 1.017 [1.015, 1.021] high range; disk, quartic_n4v2 README table)"
  DL1_over_rho_neg_k16: "0.03 [0.01, 0.10] at w = 0, 0.05 [0.02, 0.12] at w = 1 (derived: N4v2 DL band-resolved below rho-neg from q = 991, interpolated q* = 555 [389, 752] at w = 0 and 276 [232, 319] at w = 1; DL1/DL = 0.536)"
  rho_reference_price: >
    rho on a quartic Jacobian costs about 5,550 M + 229 I per group addition (tian_n3 REPORT), about 100x the
    cost of an elliptic-curve addition over F_{q^3} (derived: about 3 F_{q^3} multiplications at about 6 M each plus one
    inversion, roughly 50 M). The route the test stands in for would run rho on E(F_{q^3}), not on the Jacobian.
    Report the ratio against BOTH references: rho_J (the internal DLP, valid as measured) and rho_E (modelled with the
    measured cost of an E(F_{q^3}) addition at the same q; the conservative one). Only rho_E is relevant to the
    claim "below rho for the elliptic curve"; expect IC/rho_E about 100x the rho_J ratio at q = 2^16 (derived), i.e. the
    claim is only meaningful at the sizes where rho_E ratios stay below 1: q >= about 2^18-2^20 (derived from N4v2
    0.005 at 2^21 with rho on E0), so the local test establishes the exponent and the solver's non-weak validity,
    not a below-rho row.
discriminating_test:
  local: >
    Rust only. (a) Instance generator: random F(x, y, 1) of degree 4 over F_q, smoothness check (existing
    smoothness certificate of the Tian study), N1, N2 by brute force (q <= 2^18, q^2 <= 2^36 for N2 needs a
    sieve; restrict the counting to k <= 16 and use BSGS/kangaroo in the Hasse-Weil interval for the L-polynomial
    coefficient c3 for k = 17, 18), reject until #J has a prime factor l >= #J/8. (b) Run the N4v2 binary unchanged on
    (C, P, Q = xP). (c) N4v2's own rho-neg and plain rho on the Jacobian with the same counters, plus the E-addition
    price model. Each k <= 16 unit takes seconds to minutes (N4v2 D family 2.8-5.2 s per unit at k = 16 on its pilot
    host); 21 units, one job under 30 minutes on 4 vCPU / 15 GB for k <= 16.
  controls:
    - "C1: N4v2 on its own weak-family instance at the same q reproduces its committed record (replay holds for S/L/D/DL1/DL in the README)."
    - "C2 A/A: each generic unit run twice from a fresh process gives identical records (deterministic)."
    - "C3: relations_bad = holdout_bad = 0 and every kG = Q check passes; a failing unit is charged, not dropped."
    - "C4 positive: plant a decomposition-free target (a random class) and a small multiple of a base point; both recovered."
    - "C5 calibration of rho exactly as N4v2's frozen rule (plain in [1.0, 1.5] steps/sqrt(l), neg in [0.75, 1.15]); excluded cells reported."
falsification: >
  Killed (as a statement that the solver is family-independent) if any of: a unit fails to verify after the charged
  retries; the DL1 exponent over k = 12..18 has its 95% bootstrap interval outside [0.90, 1.15]; DL1 / rho_J at k = 16
  has upper bound above 0.20 at w = 0 (N4v2 weak-family value is about 0.02-0.05); or the DL1 cost per unit at fixed k
  differs from the weak family's at the same q by more than a factor 2 in either direction (which would mean
  the cofactor-4 structure or the builder was doing hidden work). A failure here also closes the n = 3 route's
  downstream half and is the more valuable result.
row_moved: >
  tian_n3_quartic_route item (b) row (generic quartics, band unresolved) and the "Diem arm" row of the treatise
  Jacobian table; gives the first Diem-type scaling on cofactor-one instances to q about 2^18. No leaderboard row; no
  below-rho-for-E claim.
cost_band:
  unit: "M-equivalents (N4v2 convention: lmul and inversions charged per PARAMS.md; mod-l inversions 60 lmul)"
  arms:
    dl1: "N4v2 DL1; unpriced: Pic relation checks, hash lookups, line-pair selection, matrix assembly (list counts for each)"
    dl: "N4v2 DL, registered headline"
    rho_neg_J: "negation rho on the Jacobian, about 3 unpriced operations per step (canonicalisation, DP test, cycle check)"
    rho_plain_J: "plain rho, one unpriced operation per step"
    rho_E_model: "rho with the measured cost of an E(F_{q^3}) addition at the same q; modelled, labelled"
  weights: "w = 0, 0.1, 1 GAE per unpriced native operation; rank two arms only where the order holds at all three"
controls_summary: "C1-C5 above"
risks:
  - "The N4v2 builder may call GHS-specific code (4-point-line recomputation, cofactor-4 targets); if the solver is not instance-agnostic, a small adapter is needed and the test becomes a 1-2 day implementation, not a run."
  - "Computing #J for random quartics beyond q about 2^14 needs a genus-3 point-counting shortcut; BSGS on the c3 interval costs about q^{1.25}, which limits the top size to k about 18."
  - "rho_J overstates IC's advantage by about 100x against the real target (an elliptic curve); the test measures the solver, not an attack."
closest_baseline:
  published: "Diem-Thome 2008 (heuristic O~(q) on plane quartics, experiments at small sizes); Laine-Lauter 2015 time-memory variant (abstract only read)."
  in_repo: "research/tian_n3_quartic_route_20261024 item (b) (generic quartics to q = 8191, dense LA); research/quartic_n4v2_20261020 (weak family only)."
nearest_existing_lead_and_difference: >
  LEAD-TRF-small-genus-cover-general, LEAD-TRF-odd-prime-ghs-cover-floor and LEAD-ICE-relation-graph-rank-large-prime
  (ids only; bodies unread). Difference: this is a family-independence test of an already-built solver at scale
  with a priced band, not a new solver or a new cover.
lean_obligation: "None for the measurement. No mathematical claim is relied on (Diem's O~(q) stays a heuristic)."
implementation_language: "Rust only (existing N4v2 tool); no Python."
next_steps:
  - "1. Frontier tier, 1 h: read the N4v2 builder and solver entry points and state whether (C, P, Q, l) can be passed without the GHS builder; record the adapter size."
  - "2. experiment-director (Sonnet), 2 h: freeze a command card with the generator, k grid, controls C1-C5 and the rho_E price rule; include the resume checkpoint."
  - "3. executor-mechanical (Haiku), 1 h wall: run k = 12..16 locally (21 units), then k = 17, 18 if memory allows; no deviation."
  - "4. experiment-director, 1 h: assess against the falsifiers and write the ratio table for both references and all three weights."
  - "5. If the exponent holds: hand Proposal 3 stage 0 to the Coordinator as unblocked; if not, file the obstruction (what in the solver depends on 4 | #E0)."
```

## Proposal 2

```yaml
id: LEAD-J-dl1-percolation-fb-size
title: Percolation-threshold factor-base size and adaptive stop for the plane-quartic double-large-prime solver (c_D below 2.0)
obstruction_attacked: >
  HEUR-RELATION-RANK (frontier row 2, impact 10) and the large-prime graph assumption HEUR-WD-DT-ASSUMPTION12
  (row 26) are assumed, not measured, in the solver that is the programme's closest-to-rho non-sieve arm. The
  frozen N4v2 choice c_D = 2.0 was picked from {1.5, 2.0, 2.5, 3.0} (disk, PARAMS.md) and 1.75, the step the retry rule
  uses, was never tried as a first attempt. Both linear algebra and pair enumeration scale as c_D^2, so any
  slack in c_D is a straight multiplier on DL1's cost (relations 65%, linear algebra 34% at k = 22, README).
claim: >
  On N4v2's own instances (q = largest prime below 2^k, q = 7 mod 12, k = 16, 18, 20, 22), the probability that a
  single attempt yields enough independent relations crosses 50% at c_50(k) = 1.55 [1.40, 1.75] for all four k,
  and running DL1 at c = 1.75 (single attempt, success on at least 90% of units) lowers the paired total cold M-equivalent
  cost at w = 0 to 0.77 [0.70, 0.85] of the frozen c = 2.0 at each k; an adaptive stop (random pair order, incremental
  union-find with a cycle counter, stop when the cycle rank of the FB-anchored component reaches B(1 + 0.05)) at
  c = 1.75 lowers it to 0.65 [0.55, 0.80]. All logarithms verify.
mechanism: >
  Lines through pairs of base points give edges between the other two rational points; with N = c sqrt(q) base points
  the graph on about q vertices has mean degree d = c^2 / 2 (c = 2: d = 2). Relations are the cycles of the
  recombined graph, and only B + surplus of them are needed, with B = N. Near the percolation point d = 1 + e the
  giant component has excess (cycle rank) about (2/3) e^3 q (Janson-Knuth-Luczak-Pittel; recalled, not
  re-read). Setting (2/3) e^3 q = 1.05 c sqrt(q) gives e = (1.58 c)^{1/3} q^{-1/6} and c_50 = sqrt(2 (1 + e)),
  which is 1.56 at k = 16 and 1.52 at k = 22 (derived: k = 16: q^{-1/6} = 0.171, (1.58 * 1.55)^{1/3} = 1.35, e = 0.23, c = 1.57;
  k = 22: q^{-1/6} = 0.079, e = 0.107, c = 1.49). The pilot's failure at c = 1.5 for k = 16 and 18 (disk) is
  consistent with a threshold just above 1.5; c = 2.0 sits well above it. At c = 1.75, d = 1.53, e = 0.53, the
  excess is about 0.10 q, far above B. The descent from a target to the giant component stays cheap:
  with giant fraction x = 0.2-0.8 each trial succeeds w.p. about 0.5 x^2 and costs a line computation (derived: tens of trials
  at x = 0.2, which is negligible against the 1e8 relation phase).
prediction:
  threshold: "c_50(k) = 1.55 [1.40, 1.75], flat in k = 16..22 (model: 1.57 to 1.49)"
  cost_ratio_c1_75_over_c2_0: "0.77 [0.70, 0.85]  (derived: (1.75/2.0)^2 = 0.766; both phases scale as c^2)"
  cost_ratio_adaptive_c1_75: "0.65 [0.55, 0.80]  (derived: stops at about 85% of the pair work when the excess reaches B)"
  giant_fraction_at_c2_0: "0.80 [0.76, 0.84]  (derived: root of x = 1 - exp(-2x) = 0.797 for mean degree 2)"
  success_at_c1_75_each_k: ">= 0.90 (model e = 0.53 gives cycle rank 0.10 q against B = 3.5 sqrt(q): ratio 100 at k = 22, 25 at k = 16)"
discriminating_test:
  local: >
    Rust, N4v2 binary plus a small instrumentation patch that emits, per unit: vertices, edges, giant size, cycle rank,
    rank of the relation matrix mod l (after SGE) and the surplus. Grid: c in {1.40, 1.50, 1.60, 1.75, 2.00} x k in {16, 18, 20} x 3
    seeds (45 units; each D-family unit is 3-20 s at k <= 20, per PARAMS.md), then k = 22 for c in {1.60, 1.75, 2.00} (9
    units, D family about 75 s each). The success-probability curve vs c, the giant fraction at c = 2.0 and the cost ratio at c = 1.75
    are read off. Under 1 h on 4 vCPU.
  controls:
    - "C1: c = 2.0 reproduces the frozen N4v2 records bit for bit (replay property, README)."
    - "C2: the instrumentation changes no counter (all fields of the old record identical)."
    - "C3: every unit verifies kG = Q; a unit that needs a retry is charged both attempts (N4v2 rule)."
    - "C4 positive for the graph census: a synthetic Erdos-Renyi graph with the same N and q reproduces x = 0.797 at d = 2 within 0.02 (checks the census code, not the curve)."
falsification: >
  Killed if any of: the single-attempt success rate at c = 1.75 is below 0.75 at any of k = 16, 18, 20; the paired cost ratio c = 1.75 / c = 2.0 has upper bound above 0.90
  at w = 0; the measured giant fraction at c = 2.0 is outside [0.70, 0.88] (then the ER picture of the line graph is wrong and the
  threshold law is not the explanation); or the rank of the relation matrix mod l falls below B - 1 at surplus 5% at any unit
  (rank deficit beyond the graph model, HEUR-RELATION-RANK fails in this family). The last two are results about the heuristics, not only about this lever.
row_moved: >
  N4v2 DL1 / DL rows (the programme's ratchet baseline on the weak quartic family; L0, no validation file).
  Predicted improvement 1.3-1.5x at fixed instances, class: engineering (a parameter law plus an adaptive stop; native counters flat or lower). Not
  validated; the claim needs the validation gate and the unchanged rho reference (the frozen records) before any board row.
cost_band:
  unit: "M-equivalents as N4v2 (lmul, ladd charged; mod-l inversions 60 lmul)"
  arms:
    dl1_c2_0: "frozen reference; unpriced: Pic relation checks (excluded from phase times, README deviation 2), hash lookups, pair selection"
    dl1_c1_75: "same code, c = 1.75; the same unpriced list; cycle counter adds union-find operations (count them as native, w-priced)"
    dl1_adaptive: "union-find ops per edge charged at w; stop test every 2^12 pairs"
    rho_neg: "matched negation rho on E0 from the frozen N4v2 records (616 CPU-s per run at q = 4194271); about 3 unpriced operations per step"
    rho_plain: "plain rho, one unpriced operation per step"
  weights: "w = 0, 0.1, 1; the order claim (c = 1.75 below c = 2.0) is made only if it holds at all three (the new native counters are small against the relation work)"
risks:
  - "The line graph is not Erdos-Renyi (edges come from pairs of base points; FB vertices are special); the giant-fraction census is itself the test of that."
  - "The pilot optimum at 2.0 may reflect a failure mode the ER model misses (hold-out violations, tree propagation, the anchoring of components to base points)."
  - "At small k (below about 14) the threshold law gives c above 2 (e grows as q^{-1/6}); the lever is a large-q lever."
  - "A cost reduction of about 1.3-1.5x on a row that is already 0.005x rho changes no ranking; its value is the ratchet and the rank/graph census."
closest_baseline:
  published: "Diem 2006/2011 and Gaudry-Theriault-Thome-Diem: graph of large-prime relations, analysed as a random graph (the percolation threshold is in the same family of arguments); not re-read."
  in_repo: "research/quartic_n4v2_20261020 (PARAMS.md choice of c_D; PROTOCOL.md retry rule c_D += 0.25); research/jv_cover_large_prime_20261018 (large primes on the JV cover: negative, a different graph)."
nearest_existing_lead_and_difference: >
  LEAD-ICE-relation-graph-rank-large-prime and LEAD-ICB-large-prime-variants (ids only). Difference: this
  states a numeric threshold law c_50(k) from the percolation excess, tests it on the N4v2 instances, and prices the saving; if the vault bodies
  already contain that threshold law, this reduces to executing it on N4v2.
lean_obligation: >
  The saving does not need a theorem. If the program wants "relations suffice at cycle rank >= B" as a
  proved statement: Lean obligation `cycle_rank_relations_independent`: for a connected graph with edge labels in a
  vector space over a field, the cycle space has dimension E - V + 1 and the map from cycles to label sums has rank at most
  min(E - V + 1, dim of the label span); this is elementary linear algebra and needs no probabilistic hypothesis.
implementation_language: "Rust only (patch to the existing N4v2 tool)."
next_steps:
  - "1. experiment-director (Sonnet), 1 h: freeze the grid, the instrumentation fields and the falsifiers; check that c = 1.75 is not already on disk as a first attempt."
  - "2. executor-mechanical (Haiku), 2 h wall including k = 22: run the 54 units locally with the resume checkpoint, no changes."
  - "3. experiment-director, 1 h: fit the success curve (logistic in c per k), paired cost ratios, giant fractions; assess against F1-F4."
  - "4. Frontier tier, 2 h: if the threshold law holds, specify the adaptive stop (random pair order, union-find) and the safe fallback to c += 0.25; hand to implementation."
  - "5. Re-run the held-out k = 21, 22 pair at the chosen rule against the frozen rho records; request the validation gate only then."
```

## Proposal 3 (the only mechanism on disk that reaches a non-weak class; high cost, staged)

```yaml
id: LEAD-J-tian-n3-transport-and-reach
title: Odd-l Tian transport for prime-order E over F_{q^3}: measure the reachable class at larger q first, then build the missing isogeny and run the end-to-end route
obstruction_attacked: >
  The invariant that must be matched is the Frobenius-module structure of E[l] under pi_{q^3} (Theorem 4: split with every
  eigenvalue a cube, scalar, or Jordan with l = 1 mod 3), plus l not dividing #E so the isogeny is an isomorphism on the order-N part.
  The programme has the existence theorem (Q <= 13), the existence of a quartic in the isogeny class (26/26 at q <= 17) and the
  downstream solver (N4v2), but no explicit isogeny and no transport map: "No Tian isogeny Res E -> Jac(C) was computed"
  (tian_n3 REPORT). Without the map no prime-order DLP has been moved. This is the atlas cells E2 (non-weak) and E6 for prime order.
claim: >
  (Stage 0, no isogeny) For prime-order E over F_{q^3}, the fraction of curves that admit a Tian kernel at some odd l <= 7 is
  0.45 [0.35, 0.65] at every q in {17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61} (exhaustive over all curves, as the existing census was at q <= 13), flat in q (slope in q within
  +-0.002 per unit q), and rises to at least 0.60 when l <= 13 is allowed.
  (Stage 1) At q = 523, l = 3, the curve of Tian's example, an implementation of the (3,3,3) isogeny in the theta model returns a smooth plane
  quartic C over F_523 with #Jac(C)(F_523) = 143069629 and counts N1 = q + 1, N2 = q^2 + 1, and transports a planted DLP (P, Q = xP) on E to Jac(C),
  where the unchanged N4v2-type solver recovers x.
  (Stage 2) On prime-order E/F_{q^3} at q in {2^12, 2^14, 2^16}, the end-to-end route (transport + DL1) costs at most 1.3 times
  the DL1 solve alone, and is below the matched negation rho on E at q = 2^16 in group-addition units.
mechanism: >
  Tian's kernel Lambda in W[l], W = Res_{F_{q^3}/F_q} E, F_q-rational, maximal isotropic and meeting no conjugate E^{sigma^i}[l], gives
  an F_q-rational (l,l,l)-isogeny to an indecomposable principally polarised 3-fold, which is a Jacobian over
  F-bar_q by Oort-Ueno and over F_q up to a quadratic twist (recalled, not read; the census checked the twist-free Weil
  polynomial). For prime order E the codomain is a plane quartic (hyperelliptic needs #E even, Tian's own remark). Since l does not divide N,
  the isogeny restricts to a group isomorphism E(F_{q^3})[N] to Jac(C)(F_q)[N], so the DLP transports with a one-time cost depending on l and log q only.
  Diem-Thome (O~(q)) and the N4v2 solver then apply. The transport cost is q-independent for fixed l;
  the downstream cost is the measured DL exponent about 0.99 against rho's 1.5 (disk). The existence census shows the class is
  large (24/61, 57/150, 144/323 at q = 7, 11, 13: 0.39, 0.38, 0.45, derived), and Theorem 4 depends only on (q mod l, t mod l, type of phi), i.e.
  on residue data that do not shrink with q; this is the reason for the flatness prediction.
prediction:
  stage0_fraction: "0.45 [0.35, 0.65] for l <= 7; >= 0.60 for l <= 13; flat in q (slope within +-0.002 per unit q)"
  stage0_vs_theorem4: "exact agreement of the measured kernel-existence indicator with Theorem 4's formula on every curve (0 mismatches; the census had 0 at l <= 7)"
  stage1: "quartic recovered at q = 523 with the published count data; transported DLP solved; wall under 10 s for the DL1 solve (derived: N4v2 q* = 555 means roughly a tie with rho at this size, so no below-rho claim at stage 1)"
  stage2_ratio_to_rho_neg_E_q2e16: "0.03 [0.01, 0.10] at w = 0 (derived: N4v2 DL / rho-neg 0.02-0.05 at q = 65479-262111 and 0.005 at 2^21; transport cost one-time)"
  transport_cost_over_dl1: "<= 0.3 at q = 2^16 (to be measured; a model of 1e6-1e8 field multiplications for a dimension-3 theta isogeny of degree 27 plus quartic reconstruction, against DL1 about 1e8-1e9 at 2^16)"
discriminating_test:
  stage0_local: >
    Rust (the existing census tool examples/weil_restriction_isogeny_census.rs is std-only, 8 unit tests): extend Part H from q <= 13 to
    q in {17, ..., 61}, all isomorphism classes of E over F_{q^3} (about 2 q^3 = 4.5e5 curves at q = 61), orders by BSGS over the group, prime-order filter,
    classification at l in {3, 5, 7, 11, 13} by (q mod l, t mod l, phi-type from the l-division polynomial factor pattern over F_{q^3}); output the kernel-existence
    indicator per curve and the union fractions. Minutes to a few hours on 4 vCPU. This stage cannot confirm an attack; it prices how large the reachable set is.
  controls:
    - "C1: the tool reproduces the committed counts at q = 3, 5, 7, 11, 13 (0/8, 0/18, 24/61, 57/150, 144/323)."
    - "C2: for every curve the indicator equals Theorem 4's formula (cube test in F_l, Jordan test l = 1 mod 3)."
    - "C3 negative control: l = 2 reproduces 'kernel iff full rational 2-torsion' on every curve (9, 42, 115, 443, 734 at q = 3..13)."
    - "C4: prime-order filter excludes l | #E (the isogeny is then not an isomorphism on the N-part); count the exclusions."
  stage1: >
    Obstruction named: an explicit (3,3,3)-isogeny out of Res E (a product of three conjugate elliptic curves over F_{q^3}) and the
    reconstruction of C from theta nulls. Plan: build W over the field K over which E[4] and E[3] are rational (degree dividing |GL_2(Z/12)|; at q = 523 this is a
    large but finite tower, polynomial-time), apply the isogeny in the theta model (Lubicz-Robert; Dartois-Maino-Pope-Robert), reconstruct the quartic by
    Weber/Aronhold bitangents (Fiorentino), descend to F_q by Galois invariance, then check the published counts. An external reference implementation may
    be used only to produce an independent certificate (C, Lambda) for comparison, not as the deliverable; the deliverable is native Rust.
  stage2: "Planted DLPs on E at the three sizes with the E-side data generated first (so the transport is exercised on points of E), solved by DL1, verified by kG = Q on E; matched negation rho on E; replay on a second host."
falsification: >
  Stage 0 killed (as 'a constant fraction') if the union fraction at l <= 7 falls below 0.25 at q >= 41 or shows a monotone decline of more than 0.1 over the grid;
  an indicator mismatch with Theorem 4 on any curve falsifies the theorem's reading and must be reported before anything else. Stage 1 killed (as a transport
  route) if no smooth quartic with the published counts is recovered from the Tian kernel at q = 523, or if the recovered Jacobian is the quadratic twist
  and the twist cannot be undone from counts (then the map is not order-preserving), or if the images of E-points cannot be expressed as divisor classes on C
  at polylog cost. Stage 2 killed if transport / DL1 is above 1 at q = 2^16 or the end-to-end ratio at q = 2^16 has upper bound above 0.3.
row_moved: >
  A new row, not an improvement of an existing one: the first measured prime-order (non-weak) class moved to a quartic. If Stage 2 holds it
  would be a verified improvement in reach (a non-weak class below matched rho) on a narrow, undeployed family (prime-order E over a cubic extension F_{q^3} with a Tian kernel);
  until then it is existence data. Leaderboard Table D withheld; the validation gate (replay, validator, matched rho with the 5a band) is required.
cost_band:
  unit: "M-equivalents (F_q multiplications) and group additions on E(F_{q^3}); transport counted as native work at w"
  arms:
    transport: "one-time per instance class; field multiplications in the tower K, theta isogeny, reconstruction, point evaluation (2 points per instance); unpriced: tower-field arithmetic overhead, memory"
    dl1: "N4v2 DL1 (priced as in Proposal 1)"
    rho_neg_E: "matched negation rho on E(F_{q^3}) (about 0.89 sqrt(N) additions, about 3 unpriced operations per step)"
    rho_plain_E: "plain rho"
    generic_reference_BSGS: "none needed; rho is the reference"
  weights: "w = 0, 0.1, 1 on both sides; order claimed only where it holds at all three (N4v2 DL beats rho-neg at all three from q = 991, disk)"
risks:
  - "Stage 1 is a substantial implementation (dimension-3 theta isogeny over a torsion-field tower, quartic reconstruction); a few weeks of expert work is plausible. The cheap stage 0 is useful regardless: it prices how many prime-order curves a working transport would reach."
  - "Quadratic-twist ambiguity: the codomain may be the twist of Jac(C); the order #E decides which, but only if the counts N1..N3 are checked."
  - "Tian's published example is one curve; the paper's text gives no proportion and no existence condition (census report, section 1). The census's 40% is for q <= 13 and l <= 7; stage 0 tests whether it is a small-q artefact."
  - "The reconstruction needs F_q-rational bitangent data; over finite fields the Aronhold set may not be rational, which forces work in an extension and a Galois descent."
  - "Scope: this is a statement about prime-order curves over cubic extension fields. It says nothing about prime fields or about any standard curve (all NIST/SEC curves are over prime or binary fields, not cubic extensions)."
closest_baseline:
  published: "Tian 2023 (arXiv 2012.07173): the construction and one example, no proportion; Diem-Thome 2008 for the quartic; Smith (EUROCRYPT 2008) for the hyperelliptic-to-quartic (2,2,2) case, which needs #E even."
  in_repo: "research/weil_restriction_isogeny_census_20261018 (Theorem 4, existence at q <= 13); research/tian_n3_quartic_route_20261024 (quartic in class, 26/26; generic-quartic Diem arm); research/quartic_n4v2_20261020 (the solver); research/cubic_extension_transfer_20261025 ('a complete native transfer is missing', treatise line 1222)."
nearest_existing_lead_and_difference: >
  LEAD-TRF-weil-restriction-jacobian-class, LEAD-TRF-cover-catalog-jacobian-transfer, LEAD-TRF-odd-prime-ghs-cover-floor,
  LEAD-TRF-small-genus-cover-general (ids only; bodies unread). Difference: the existence theorem is already on disk; this record asks for (i) the reach at larger q with the l-union
  and (ii) the explicit transport and the end-to-end price, which is the step the census report and the Tian study both list as undone.
lean_obligation: >
  Two statements the program would rely on. (L1) `tian_kernel_exists_iff` (Theorem 4): over ZMod l with
  chi(T) = T^2 - tT + q3, the F_l[T]-module F_l[T]/(chi(T^3)) has a 3-dimensional submodule meeting no factor in its
  induced pieces and isotropic for the form sum q^i omega iff the split/cube/Jordan conditions hold; finite linear algebra, provable for all l by case analysis, with a
  decidable instance check for l <= 13. (L2) `isogeny_restricts_to_iso_on_coprime_part`: if psi: A -> B is an isogeny of degree d and gcd(d, N) = 1, then psi induces a group isomorphism of A(F_q)[N] onto B(F_q)[N] (elementary). The
  heuristics (Diem-Thome O~(q), HEUR-RELATION-RANK) stay heuristics; Lean cannot discharge them.
implementation_language: "Rust only (stage 0 extends the existing std-only tool; stage 1 is native theta arithmetic); no Python. Sage, if used for an independent certificate, is a checker, not the deliverable."
next_steps:
  - "1. experiment-director (Sonnet), 2 h: freeze stage 0 (grid q, l set, controls C1-C4, exact Theorem 4 predicate); no new theory."
  - "2. executor-mechanical (Haiku), a few hours wall: run stage 0 locally; report union fractions and any indicator mismatch."
  - "3. Frontier tier, 1-2 days: if the fraction is >= 0.25 at q >= 41, write the stage-1 design: field tower, theta structure level, Aronhold reconstruction, twist test; name the independent certificate source; decide build versus assign."
  - "4. Implementation tier (frontier-assisted), weeks: stage 1 at q = 523 (published example) with the counts N1, N2, #J as acceptance; then stage 2 at 2^12, 2^14, 2^16."
  - "5. Lean tier: L1 and L2 as kernel-checked statements before any claim of 'reach' is quoted outside this lane."
```

## Proposal 4 (extension fields, model first, medium cost)

```yaml
id: LEAD-J-k4-genus2-nagao4
title: E(F_{p^4}) through a genus-2 cover over F_{p^2} and a JV-style Nagao four-point descent to F_p: measure the test on the Jacobian first
obstruction_attacked: >
  The k = 4 programme (E19-E42, V4 line) is solve-dominated (99% of S, treatise) with the validated ladder at 1.5e3x its strongest matched rho at n = 2^42.4 and a fitted crossover near
  2^85.5 [2^79.2, 2^94.3] (prog_openproblems.tex row 6, disk). The Joux-Vitse insight for k = 6 was that a genus-3 cover replaces the summation-polynomial system by a Mumford
  system (six quadrics, Bezout 64, about 9.5e5 multiplications per test, disk); the analogue for k = 4 (q = p^2, n = 2) would replace S_5 by four quadrics in four unknowns (Bezout 16). The
  programme measured the genus-2 route at n = 2 over F_p ("about p/3 Jacobian operations, ratio 0.51-2.83 to rho, unresolved", treatise line 1214) but I find no record
  of a genus-2 cover over F_{p^2} with a Nagao descent to F_p. The census records the existence criterion at n = 2 (kernel iff phi^{-1} has a square root psi with det psi = -q-bar^{-1}),
  and dismissed the route because "index calculus costs O~(q) = rho" without the descent.
claim: >
  On random genus-2 curves y^2 = f(x), deg f = 5, over F_{p^2}, p in {101, 251, 503, 1009, 2039}, a Nagao four-point test (four quadrics in four F_p unknowns after Weil restriction
  of the Mumford condition, factor-base line a F_p + b chosen by the existing line search) has: (i) decomposition rate per test equal to (2B/p)^4 / 24 within [0.85, 1.20]
  (as the JV cover's rate matched its model in 0.92-0.99); (ii) cost per test at most 3e4 F_p multiplications (model 5e3, range 3e2-3e4); (iii) a modelled single-target cost
  S(p) = 12 p C4 + 3 p^2 (multiplication equivalents, relation phase plus sparse linear algebra) that crosses a matched negation rho on a 4|#E elliptic curve over F_{p^4}
  between p = 1e2 and 1.2e4 and tends to 0.09-0.30 of rho from below for p >= 2e4 (the linear algebra and rho both grow as p^2, so the large-p ratio is a constant).
mechanism: >
  Gaudry's decomposition over F_{q^n} needs n-point systems whose size grows with n; Nagao and JV observe that after a cover the same count is a Mumford system.
  Here the Jacobian is 2-dimensional over F_{p^2} (4 over F_p): four points with x in the line, rate 1/24, relations needed about B + 1 with B about p/2 line classes, so
  tests per relation about 24 and 12p tests in all (derived). Linear algebra is sparse (row weight at most 4) on B = p/2 columns: about 2 B * 4 B = 2 p^2
  modular operations. Matched rho on E(F_{p^4}) costs about 0.89 p^2 / sqrt(h) additions (h = 4 for the even-order class), each about 60-90 F_p multiplications (derived: three F_{p^4} multiplications at 9 F_p multiplications plus an inversion); about 34 p^2
  for h = 4. So IC / rho is about 1.8e3 / p while relations dominate (C4 = 5e3: 12 * 5e3 p = 6e4 p against 34 p^2) and levels at about 0.09 when linear algebra dominates (3 p^2 / 34 p^2), p beyond about 2e4. All derived,
  constants unmeasured. The only unknown that decides the lane is C4.
prediction:
  rate_per_test: "(2B/p)^4 / 24 with ratio measured/model in [0.85, 1.20]"
  C4: "5e3 [3e2, 3e4] F_p multiplications per test (JV six-variable test: 9.5e5, Bezout 64; the four-variable system has Bezout 16 and F4 degree about 5)"
  crossover_p_star_model: "2e3 [4e2, 1.2e4] (derived from 12 C4 / 31 with C4 in [3e2, 3e4])"
  large_p_ratio_model: "0.09 [0.09, 0.30] of rho at w = 0 once relation work is below linear algebra (p > 2e4); at w = 0.1 and 1 the unemitted additions in linear algebra and the descent must be priced and may reverse the order, as the n = 3 Diem arm's did"
discriminating_test:
  local: >
    Rust. If the existing Nagao routine in src/cryptanalysis/jv_cover.rs (nagao_decompose) is parametrised by genus, instantiate g = 2, n = 2;
    otherwise a four-unknown copy of the same Weil-restriction builder (1-2 days). Planted DLP in Jac(C)(F_{p^2}) (order about p^4), line search as in
    jv_cover_fbline, relation count B + 1, sparse FGLM/Wiedemann linear stage as in the cover. Report C4 (F4 multiplications per test), rate per test, and
    the modelled S(p) against a matched negation rho priced with the measured c_add of E(F_{p^4}) at the same p. The Jacobian DLP is the stand-in: a cover to a real E needs
    the n = 2 transport (classical gluing, genus-2 Frey-Kani/Scholten type, recalled not read), which is a separate step.
  size: "tests per p are 12 p; at p = 2039 that is about 2.4e4 tests of about 5e3 multiplications: seconds. The whole grid is under an hour on 4 vCPU."
  controls:
    - "C1: the same binary run at g = 3 reproduces the committed JV cover records (bit identical with the frozen options)."
    - "C2 positive: a constructed decomposition (four chosen line points summed into a class) is recovered 20/20."
    - "C3 oracle: brute-force enumeration of decompositions at p = 53 equals the F4 solution set (as the JV Edwards study's meet-in-the-middle oracle)."
    - "C4 A/A: identical records on two runs from fresh processes."
falsification: >
  Killed if C4 exceeds 1e5 multiplications per test at p >= 503 (then p* is above 1e5/31 * 12 = 4e4 and the route is irrelevant at runnable sizes), if the rate is outside [0.7, 1.4] of the model, if the
  four-variable system is positive dimensional on more than 5% of tests (the Weil restriction has more freedom than the count), or if the modelled
  crossover with the measured C4 and E(F_{p^4}) c_add is above p = 1e5. Not killed but reported: the order at w = 0.1 and 1 if linear algebra additions reverse it.
row_moved: >
  None directly; if it holds it produces the first model-with-measured-test for a k = 4 route whose distance to rho is not 1e3x but about 1x, replacing the
  extrapolated 2^85.5 crossover of the V4/D4 line for the covered class. The covered class is only those E/F_{p^4} that have a genus-2 cover over F_{p^2}, determined by the n = 2 kernel criterion
  (positive fraction, not computed here), so the claim is class-restricted, undeployed and not an improvement until an end-to-end run with a transport exists.
cost_band:
  unit: "F_p multiplications, group additions on E(F_{p^4})"
  arms:
    nagao4: "relation phase (F4 counted), sparse linear stage (lmul charged), descent; unpriced: F_p additions in F4 and the linear stage, hash lookups, pair selection (list counts)"
    rho_neg_E4: "matched negation rho on a h = 4 elliptic curve over F_{p^4}, measured c_add, about 3 unpriced operations per step"
    rho_plain_E4: "plain rho"
    v4_d4_line: "the programme's current k = 4 arm, at its measured distance (1.5e3x at 2^42.4, validated, disk) for context only"
  weights: "w = 0, 0.1, 1 GAE per unpriced native operation, both sides; a rank is claimed only where the order holds at all three"
risks:
  - "The cover class and the transport at n = 2 are not built; this record measures only the Jacobian side. It can fail the lane (C4 too large) but cannot by itself establish an attack."
  - "The asymptotic tie (linear algebra p^2 against rho p^2) means the best case is a constant below 1 (about 0.1-0.3), not an exponent; the n = 2 Tian criterion says the cover exists only for a subset of curves."
  - "The treatise's genus-2 n = 2 row over F_p was unresolved (0.51-2.83); the extra descent here is what changes the unit; if it does not, expect the same unresolved band."
  - "A sub-line factor base of size p^0.8 (relation and linear algebra balance at about p^1.6) is an additional lever for p > 2e4 only; it is not part of this record."
closest_baseline:
  published: "Nagao 2007 (Jacobians over F_{q^n} with two large primes) and Joux-Vitse (cover and decomposition, Eurocrypt 2012); Gaudry 2009; for k = 4 the published exponent is q^{3/2} (Gaudry-Diem). A genus-2 cover plus Nagao descent for E(F_{p^4}) was not found in the search."
  in_repo: "src/cryptanalysis/jv_cover.rs and jv_sieve.rs (genus 3); research/weil_restriction_isogeny_census_20261018 (n = 2 kernel criterion); treatise transfer_ext.tex line 1214 (genus-2 route at n = 2 over F_p); research/glv_* E19-E42 (k = 4 Semaev S_5 line)."
nearest_existing_lead_and_difference: >
  LEAD-ICP-gfp5-oa-sdhp-jv and LEAD-ICP-gfp5-odd-char-cover (ids only; they concern k = 5), LEAD-TRF-small-genus-cover-general. Difference: k = 4, genus 2 over the quadratic subfield, with a Weil-restriction descent to the prime field.
lean_obligation: >
  None for the measurement. The class restriction would rely on the n = 2 kernel criterion (`tian_kernel_exists_n2`:
  exists iff phi^{-1} has a square root psi with det psi = -q-bar^{-1}), already stated in the census report as proved;
  a Lean statement of it is the obligation before the class is named in any claim.
implementation_language: "Rust only (existing jv_cover builder, genus parameter); no Python."
next_steps:
  - "1. Frontier tier, 1 h: read jv_cover.rs around nagao_decompose and state whether genus and the number of unknowns are parameters or hard-coded to 3 and 6."
  - "2. experiment-director (Sonnet), 2 h: freeze the grid p in {101, 251, 503, 1009, 2039}, three curves each, controls C1-C4, the c_add price rule for E(F_{p^4})."
  - "3. executor-mechanical (Haiku), 1-2 h wall: run locally; report C4, rate, and the modelled S(p) with intervals."
  - "4. experiment-director, 1 h: assess F1-F4; if C4 <= 3e4, hand the class question (how many E/F_{p^4} have an n = 2 cover) to the Coordinator as a stage-0 census in the style of Proposal 3."
  - "5. Frontier tier: only if the model crosses below p = 1e4, scope the n = 2 gluing transport (classical, genus-2); no work before that."
```

---

## Ranking

1. LEAD-J-quartic-dl1-generic-jacobian (Proposal 1): a one-hour local test that decides whether the only closest-to-rho non-sieve solver is family-independent; its failure or success settles whether Proposal 3 is worth weeks. [pcc:unverified]
2. LEAD-J-dl1-percolation-fb-size (Proposal 2): the cheapest likely measured improvement (about 1.3-1.5x at fixed instances, from a parameter law the pilot data already hint at: c = 1.5 failed at k = 16, 18 and c = 1.75 was never tried), and the first measurement of HEUR-RELATION-RANK on a scaling family.
3. LEAD-J-tian-n3-transport-and-reach (Proposal 3): the only mechanism on disk that could move a non-weak (prime-order) class; stage 0 is cheap and prices the class, stage 1 is the real obstruction (an explicit dimension-3 isogeny) and should not start until Proposals 1 and stage 0 say go.
4. LEAD-J-k4-genus2-nagao4 (Proposal 4): a model plus a one-hour measurement of the four-point test; if the test is cheap it re-opens k = 4 at a distance of about 1x instead of 1e3x for the covered class, but only the Jacobian side is measured.

## Considered and not proposed (with the reason)

- **E3, CM class ideals on P-192 (relation generation).** Priced honestly this is a null by structure, not by measurement. An ideal class of small norm acts as an isogeny to a DIFFERENT curve of the class; a closed walk corresponds to a principal ideal (alpha) and composes to the endomorphism [alpha], which acts on the cyclic group E(F_p) = E[pi - 1] as the scalar (x + y) mod N for alpha = x + y pi (pi = 1 mod the prime above N). It maps P to [lambda]P and Q to [lambda]Q with the same lambda: no information on log_P Q. Non-integer endomorphisms have degree at least |D|/4 (about p) for an order of discriminant about 4p, so none is cheap to evaluate. The class group has about sqrt(|D|) = 2^96 elements and sits in an unrelated group from Z/N. This is the Lemma of conservation of the treatise (isogeny transport) plus the generic recurrence note (conductor_gap_cm_orbits: the CM action satisfies lambda^2 - lambda + 5 = 0, equivalent to a generic recurrence). Kept as an obstruction statement; the Lean obligation, if wanted, is `cm_closed_walk_scalar_action`: for P, Q in a cyclic O-module E[n], phi_alpha acts on both as multiplication by the same element of O/n. Nearest ids: LEAD-STR-p192-class-cm-relations, LEAD-STR-endo-scalar-action-generic, LEAD-ICP-hecke-cm-class-point-rank-completion. A toy run that confirms a null is available (p near 2^20, small class number) but adds nothing beyond the proof. [pcc:unverified]
- **E6 on the weak class: hyperelliptic genus-3 double large primes against the JV sieve.** Disk numbers: the GHS-hyperelliptic ladder is about 0.44x rho at q = 2^22 (prog_openproblems problem 8, derived unit) while the JV sieve is 0.0065x at p = 1511 (q = 2^21.1), on groups of the same size: about 70x apart (derived: 0.44 / 0.0065). Asymptotics agree (q^{4/3} against linear algebra q = p^2): the sieve wins at every size, so a matched run adds a number but no lever.
- **Large primes inside the JV sieve or Nagao arm.** Already negative (jv_cover_large_prime_20261018: at most 0.02-0.56% at p <= 12000; LEADS.md L6, L7: six-point test needs all roots in F_p, so no large prime in the descent). Linear algebra is 2.5e-3 of S there.
- **Multi-line factor bases (a union of m lines with one shared linear algebra).** The test "x_i in the union of m lines" is not a single algebraic system: one needs an F4 call per line assignment, m^6 / 720 times more tests, which cancels the B^{-5}-law gain (the line search already harvests that law, x1.71). Cross-line relations also do not link logs, because two F_p-lines of F_{p^2} meet in at most one point.
- **Isogeny-class selection among weak curves to enlarge the line maximum.** The expected gain from raising the number of (curve, line) candidates from 6.5e4 to 6.5e6 is about +1 sigma in B, i.e. a factor about 1.1-1.15 in S (derived from the B^{-5} law, sigma = sqrt(p)/2); not worth a protocol.
- **Odd-order and cyclic-2-part curves over F_{p^6} reached by base change to F_{p^12}.** Base change makes the 2-torsion rational but doubles the field of the cover: the Nagao linear algebra becomes about p^4 against rho at the order's square root, about p^3 (derived), a loss at every size.
- **Genus-4 Jacobian locus walk at n = 4 (Schottky codimension 1, so about 1/q of the ppav's in the class are Jacobians).** The idea is a random walk in the Frobenius-stable l-isogeny graph of principally polarised 4-folds until a Schottky test (cheap on theta nulls) succeeds, about q steps, giving q^{4/3} with Diem on a plane quintic against rho's q^2 and Gaudry-Diem's q^{3/2}. It depends on the dimension-4 theta isogeny engine that Proposal 3 stage 1 would first have to build (and on genus-4 curve reconstruction), and the treatise records that no congruence predicts which classes reach a Jacobian at q = 3, 5, 7. Revisit only after Proposal 3 stage 1 works at n = 3.
- **LEAD-TRF-miller-cocycle-nonhom-transfer.** Already has an EXO twin (LEAD-EXO-miller-pairing-cocycle); the equidistribution test is a one-hour toy run and belongs to that lead, not to the cover lane.
- **LEAD-TRF-teske-trapdoor-fp6-weak-class and LEAD-TRF-teske-secret-isogeny-trapdoor.** A trapdoor is a construction for a designer, not an attack on a given curve (treatise line 1254); no index-calculus gain follows.
- **LEAD-TRF-pairing-target-field-recost.** Concerns pairing-friendly curves and the finite-field DLP after MOV, not an index-calculus improvement for ECDLP at the programme's measured families; also needs a certified tower-field DLP that is outside this lane.
- **E1 (degree-1 places, g = 2..4) alone.** Not an ECDLP route (the atlas says so); the only same-field route on prime fields is closed by theorem (treatise section on same-field embeddings: a genus-g cover over F_p costs O~(q^{2-2/g}) >= q).
- **B2, B3 (psi/GLS/eigenline folds, exotic coordinates).** Engineering by the programme's own closed form (fold = order of the eigenvalue subgroup mod r, E1-E16) and measured above rho at every size; I found no lever not already in E17-E42.
- **Hilbert early stop or line search on the sieve descent / Nagao arm.** Proposed in lane C (LEAD-C-jv-nagao-hilbert-at-scale, LEAD-C-jv-sieve-hilbert-descent); the Hilbert result of today (counted S x 0.528 at p = 1451, 1601) was not on disk in this session, so no overlap check beyond lane C's text was possible.
