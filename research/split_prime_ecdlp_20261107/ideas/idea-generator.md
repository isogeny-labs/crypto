# Idea Generator: classical prime-field ECDLP proposals (2026-11-07)

Role: proposals only. No experiment was run, no result is claimed, no hypothesis status changed.
Every numeric statement about the lab is quoted from a file I opened (paths below); every
sentence about a published method is from the web search noted under "Literature".

Sources checked: KB unavailable (research-kb MCP not available in this session). Lead atlas
(`crypto-autoresearcher/knowledge/leads/ecdlp/`) not checked out here, so `--match` was NOT run;
the novelty statements below are limited to what `rg` over `/home/user/crypto` showed.

Files actually opened:
- /home/user/cryptanalysis/AGENTS.md (Name the obstruction; Preserve scope and report evidence)
- /home/user/crypto/AGENTS.md (Tracks of index calculus; Implementation language: no Python; The rule: boundary, table, ratio)
- /home/user/crypto/problems/prime-scalar-exact-splitting.md, tree-glued-pdp-separator-coupling.md, curve25519-family-algebraic-ic-tower.md
- /home/user/crypto/research/ic_tree_gluing_20261103/STATUS.md, round7/README.md, round7_rr/README.md, round7_beyond/DIRECTIONS.md
- /home/user/crypto/research/isogeny_class_ic_yield_20261102/p2_prime_analogues/RESULTS.md
- /home/user/crypto/research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md (lines 196-235)
- /home/user/crypto/research/pkm_dickson_torus_20261015/PROTOCOL.md, README.md
- /home/user/crypto/research/curve25519_frame_search_20261103/README.md
- /home/user/crypto/research/curve25519_leads_wave4_20261104/l11_odd_kummer_design/PROTOCOL.md
- /home/user/crypto/research/generator_effect_ic_20261019/PLAN.md, conductor_gap_rho_large_sample_20261018/README.md
- /home/user/crypto/research/nist_skip_criterion_20261025/strict_elkies_20261029/REPORT.md
- seed report /tmp/claude-0/-home-user/0af71fd6-cb66-5cbf-9292-d617eb53cca4/scratchpad/pdf/report.txt

## Headline, stated plainly

1. The seed (split-prime deficit) gives no handle. Re-deriving the obstruction in the cases I
   could think of (section "Considered and killed") only reconfirmed it. None of the three
   proposals below uses the deficit as a mechanism.
2. The repository's only measured algebraic-track gains to date need structure that prime-order
   curves lack: the Edwards 2-torsion quotient E1Q (`round7/README.md` item 1, "excludes
   prime-order curves such as P-256 and secp256k1") and the Curve25519 frame search
   (`curve25519_frame_search_20261103/README.md`, coverage depends on cofactor-coset pairing).
   The proposals target what is left for prime-order p = 3 (mod 4) NIST-type fields.
3. Honest priors are low. The best one is a transfer experiment (P1), not a new mechanism.

Ranking by information per hour: P1, P2, P3.

---

## P1

```yaml
id: IDEA-20261107-RR-DICKSON-TORUS
title: Riemann-Roch norm-divisibility system with a Dickson-torus membership tower on prime-order curves
obstruction_attacked: >
  problems/tree-glued-pdp-separator-coupling.md ("A node relation with input degree < 4 whose
  tree has D below the cherry tree's"), restricted to the prime-order case where E1Q is
  unavailable. Prime-order curves have no rational 2-torsion, so no Edwards model
  (round7/README.md item 1 Scope). NIST/X9.62 primes P-256, P-384, P-521 have v2(p-1)=1 and
  v2(p+1)>=32, so the Kummer tower (2^t | p-1) does not exist there and only the Dickson torus
  tower (2^t | p+1) does (RESEARCH_PKM_TOWER_ORACLE.md section 2.1 table; pkm_dickson_torus PROTOCOL.md section 3).
claim: >
  On a prime-order E/F_p with p = 3 (mod 4) and 2^t | p+1, the separator-free Riemann-Roch
  norm-divisibility system (A^2 - f C^2 = (X - x_R) U, all inputs degree 2) with membership
  "U | D_{2^t}(X) - c" imposed by the quadratic remainder tower V_{j-1}^2 - 2 = Q_j U + V_j,
  V_t = c, has F4 solving degree D_RR(Dickson) equal to D_RR(Kummer) in every shared cell
  (m=3, t=4,5), and strictly below the cherry tree's D_T(Dickson) at (m=3, t=4).
mechanism: >
  RR carries no partial-sum separators, so the +1.5/summand coupling cost that Round 5 attributes
  to "couplings x node degree" changes. It needs no torsion, only a quotient/remainder tower
  with quadratic steps. The Dickson step y -> y^2 - 2 is quadratic, and
  pkm_dickson_torus README.md Q-A measured that the torus tower gives exactly the split and
  Kummer D in the T-chain system because "its equations are the split Dickson polynomials with a
  different constant c" (same leading forms). The same equivalence is expected to hold for the
  RR tower because the tower is built from the same squaring step. If it does, the only
  measured D gain in the tree-gluing line (D_RR=4 vs D_T=6 at m3 t4, round7/README.md item 2)
  transfers to the field shape that NIST prime-order curves actually have.
prediction:
  point: "D_RR(Dickson, m3, t4) = 4 (same as RR-Kummer); D_T(Dickson, m3, t4) = 6; D_RR(Dickson, m3, t5) = 5"
  interval: "|D_RR(Dickson) - D_RR(Kummer)| <= 1 in every cell (H_same); widest matrix within 1.5x"
  probability_H_same: 0.75
  note: "The round7 RR-Kummer numbers are [pcc:unverified] and the pre-registered prediction in round7_rr/README.md (D_RR = D_T) was wrong in the other direction, so a control re-run of RR-Kummer is part of the test."
discriminating_test:
  what: >
    Native Rust emitter (new example next to pkm_tower_pilot, writing msolve input; msolve 0.6.5
    is the existing native backend; no Python, no Singular needed) builds, for the SAME planted
    prime-order instance and target, four systems: RR-Dickson, T-Dickson (existing chain from
    pkm_tower_pilot --kinds dickson-torus), RR-Kummer and T-Kummer (controls on p0 = 786433,
    p0-1 = 3*2^18, reproducing round7 D values).
  instances: >
    Torus primes from pkm_dickson_torus PROTOCOL.md section 3: T24 = 14,680,063 = 7*2^21-1 and
    T31 = 2,046,820,351 = 61*2^25-1 (both p = 3 mod 4). Curves y^2=x^3+ax+b with PRIME order
    (cofactor 1), checked by point count and replay; x-only target and the exact-sign RR target;
    planted solution evaluated to zero by the generator before msolve is called.
  cells: "m=3, t=3,4,5 on T24 and T31; m=5, t=3 only (m5 t4 does not finish on 15 GB, round7/README.md item 5)"
  engine_limits: "msolve -g 2 -t 1, ulimit -v 1.5 GiB, 1200 s per cell (the round7_rr limits)"
  runtime: "about 1 h wall on one host; 2 targets per cell"
falsification: >
  Killed if any of: (a) D_RR(Dickson) >= D_T(Dickson) in every comparable cell; (b)
  |D_RR(Dickson) - D_RR(Kummer)| >= 2 in any completed cell (then the Kummer RR gain was
  specific to X^d - c, and the transfer fails); (c) the RR-Kummer control does not reproduce
  round7's D_RR=4 at (m3,t4) within +-1 (then the round7 gain is not trustworthy and P1 is moot).
  "Trade, not improvement" if D drops but the widest matrix is larger in every such cell
  (round7_beyond/DIRECTIONS.md rank-1 rule).
cost_band:
  class: "stage diagnostic (AGENTS.md Tracks of index calculus: algebraic track); no pipeline S is claimed"
  arms_priced: >
    RR-Dickson, T-Dickson, RR-Kummer, T-Kummer: per-target F4 cost = msolve wall x measured
    field-mul rate of the same binary, converted to GAE and priced at 0, 0.1, 1 GAE per
    native op. Matched rho.negation reference on the same curve: S ~ 1.3 flat by definition of
    S = ops/sqrt(n). Generic-decomposition incumbent (ecbench 'subtract'/mitm) on the same
    curve. Ordering vs rho reported only where it holds across the 0/0.1/1 band (AGENTS.md 5a).
  expectation: "STATUS.md derives that the dense-Macaulay law crosses rho only beyond roughly 2^500, so every algebraic arm is expected to have S >> 1 at 24-31 bits; the object measured is D and width, not S."
controls:
  positive: "RR-Kummer at p0 reproduces round7 D; planted solution vanishes in every system; ecbench verify --replay on the instance"
  negative: "shape-matched null chain (pkm_tower_pilot --null-chain) for T; for RR, random coefficients on the same support with the planted zero kept"
  nuisance: "two targets per cell; two primes; coset/u seeds recorded; torus condition checked (y^2-4 non-residue for every y in V) as in check_torus"
risks:
  - "N = (2m-1)t + 2m variables is 2-3x the tree's; widest matrix may be larger (trade, not improvement)."
  - "The decisive cells (m>=5) do not fit in 15 GB, so the repo's own improvement criterion cannot be met on this host."
  - "D at m=3 does not predict the m-slope; Round 5 gives +1.5 per summand for T and the RR slope is measured at about +1 per level in t."
  - "Dickson torus tower is degenerate at infinity (pkm_dickson_torus README Q-A-infinity); degree of regularity N+1 by BFS argument may cap any gain."
lean_obligation: >
  Faithfulness of the Dickson membership remainder tower: for monic squarefree U of degree m,
  existence of Q_j, V_j (deg V_j < m) with V_0 = X, V_{j-1}^2 - 2 = Q_j U + V_j, V_t = c is
  equivalent to U | D_{2^t}(X) - c. (Finite-field polynomial algebra; mirrors the Kummer
  statement in prime_one_shot_20261029.)  Rely on this only after it is in Lean or labelled [pcc:unverified].
closest_baseline: >
  Algebraic-track baseline = the cherry tree T (ic_tree_gluing_20261103 STATUS.md), D_T=6 at m3 t4.
  Published: Petit-Kosters-Messeng PKC 2016 (tower/factor-base maps for prime fields), Nagao
  ePrint 2013/548 (Riemann-Roch decomposition, correction history), Semaev 2004/031 (summation
  polynomials). In-repo: round7_rr/README.md (Kummer only), pkm_dickson_torus_20261015 (T chain only).
novelty: >
  Low. This is a combination of two in-repo items whose product I did not find tested
  (rg 'torus|Dickson' in ic_tree_gluing_20261103 shows no RR+Dickson run). Value is applicability
  to the NIST field shape, not a new mechanism.
prior_beats_current_prime_field_IC_baseline:
  stage_level_D_below_tree_at_m3_in_prime_order_torus_cell: 0.55
  repo_improvement_criterion_met (D and width smaller in an m>=5 cell): 0.05
  end_to_end_S_below_generic_incumbent_or_rho: 0.01
```

---

## P2

```yaml
id: IDEA-20261107-TWIST-PURE-COSET
title: Twist-pure coset (or torus element) selection for 2-power factor bases on prime-order curves, with a proved purity lemma and a derived vanishing law
obstruction_attacked: >
  "About half of V are abscissae of E" (RESEARCH_PKM_TOWER_ORACLE.md line 202): half of every
  Kummer/Dickson base is inert, yet it pays full algebraic degree (tower length t). The public
  seed picks the coset ("any g works", line 226). The Curve25519 frame search already showed
  per-(curve,t) frame choice gives 4.71x (t=5) and 3.33x (t=6) fewer F4 mul-adds per relation
  from cofactor-coset pairing (curve25519_frame_search README, P5); the prime-order analogue
  (selection by on-curve purity, no cofactor) was not measured.
lemma_proposed: >
  (Twist-part cancellation.) Let x_1..x_m in F_p, x_R the abscissa of R in E(F_p), n=#E(F_p) odd.
  If S_{m+1}(x_1..x_m, x_R)=0 then there are P_i with x(P_i)=x_i and sum P_i = R' in {R,-R};
  write eta_i = +1 if P_i in E(F_p), -1 if P_i in the twist (pi P_i = -P_i). Then
  S := sum_{eta_i=-1} P_i = O. Proof: apply pi to the relation and subtract: 2S = 0; pi S = -S = S,
  so S in E(F_p)[2] = {O}. Corollary: for m<=3 and R' != O every solution is E-pure (one or two
  twist points would be needed, which cannot cancel; three would force R'=O). Twist columns are
  therefore inert in x-only relation systems, up to a O(1/n_t) per-tuple spurious term at m>=4.
claim: >
  At fixed (m,t) the per-target F4 cost is independent of how many elements of V are E-abscissae,
  while the exact m=2 coverage and the relation yield scale as c^2 and c^m where c is the number
  of E-columns. Selecting, from C public candidate cosets (or torus elements u) of the
  disjointness-valid family, the one with the largest c lowers F4 work per relation by
  G = (c_sel/c_mean)^m. Derived law (Gaussian order statistic of Binomial(2^t,1/2)):
  c_sel/c_mean = 1 + sqrt(2 ln C / 2^t) (capped at 2), so G -> 1 as t grows.
mechanism: >
  F4 sees only the tower equations of V (2^t roots), not which roots are on E. Twist roots add
  columns of the tower, not relations. Choosing the coset whose 2^t values all lie on E is
  unrelated to the group law, so it can only change yield, not degree.
prediction:
  point: >
    t=4, C=10^4: exact binomial order statistic max ~ 15 of 16 (ratio ~1.9; the Gaussian formula
    overshoots to 2); G(m=2) ~ 3.5, G(m=3) ~ 6.6. t=6, C=10^4: c_sel/c_mean ~ 1.33, G(m=3) ~ 2.3.
    t=8, C=10^4: ratio ~ 1.12, G(m=3) ~ 1.4. F4 mul-adds per target change by < 5% between cosets.
  interval: "measured c_sel/c_mean within 10% of the exact binomial order-statistic expectation; G within 20% of (ratio)^m"
discriminating_test:
  what: >
    Native Rust program (existing PKM oracle crate and ecbench; no Python) on prime-order
    curves. For each (p,t,kind in {kummer,dickson-torus}) enumerate C candidate cosets, count
    E-columns c by Legendre symbol of f(x), keep (i) a seed-random coset and (ii) the argmax
    coset. Run the SAME f4_fp_tower system size for both (same m,t) on 4 planted targets each;
    compute the exact m=2 coverage |{R : R = +-P_i +- P_j}| / n by full pair enumeration
    (n <= 2^28 is fine).
  instances: "p about 2^20, 2^24, 2^28, 2^32 with 2^t | p-1 (Kummer, t=4..8) or 2^t | p+1 (torus, t=4..8); prime-order curves"
  extra_control: "restricted-base arm: V intersected with E-columns only, same c, to separate 'twist columns are inert' from 'F4 cost depends on the coset'"
  runtime: "few hours; coverage by enumeration is the cost"
falsification: >
  Killed if (a) F4 cost per target differs between random and selected coset by >10% at equal
  (m,t) (then twist columns are not inert and the lemma-based accounting is wrong), or (b)
  measured coverage ratio deviates from (c_sel/c_mean)^2 by >20% (then mixed E/twist solutions
  or collisions matter), or (c) the lemma fails on exhaustive enumeration of S_3 solutions at
  p <= 10^4 (any E-target solution with a twist point). A confirmed outcome shows that the gain
  exists at toy scale and decays as predicted; it does not establish any scaling benefit.
cost_band:
  arms_priced: >
    random-coset arm, selected-coset arm, restricted-base control, matched rho.negation,
    generic-decomposition incumbent; per-target F4 plus selection cost C*2^t Legendre symbols
    (each ~log2(p) field mults) charged in S (AGENTS.md "Every cost the method incurs belongs in S").
    Reported at 0/0.1/1 GAE; ordering only where it holds across the band.
  honest_expectation: >
    At p about 2^24 the selection cost (about 10^4*16*24 = 4*10^6 mults) exceeds rho's whole run
    (about 1.3*sqrt(2^24) = 5*10^3 ops), so cold end-to-end S gets worse. It pays only amortised
    over T >= T* targets on the same curve; report T*.
controls:
  positive: "Curve25519 miniature frame-search reproduction not needed; the positive control is the planted coset with all 2^t columns on E (constructed), which must reach the maximal ratio exactly"
  negative: "random coset vs a second random coset (A/A): ratio 1.0 +- sampling"
risks:
  - "Reviewers may call this parameter tuning. Precedent: curve25519_frame_search P5 accepted frame choice as a real gain at unchanged per-target F4 cost; the line between 'tuning' and 'improvement' is the cold-S rule above."
  - "Gain provably tends to 1 as t grows: negligible at cryptographic d (about 2^85). No scaling claim."
  - "At d^m about p the O(1/n_t) spurious twist solutions are comparable to real relations for m>=4; the lemma then needs the exact count, not just the corollary."
lean_obligation: >
  The twist-part cancellation lemma, stated abstractly: G abelian group, pi an automorphism,
  P_i with pi P_i = eta_i P_i (eta_i in {+1,-1}), sum P_i = R, pi R = R, and no nonzero x with
  pi x = x and 2x = 0 => sum_{eta_i=-1} P_i = 0. Short; mirror in Lean. The order-statistic law
  is numeric and needs no Lean.
closest_baseline: >
  In-repo: curve25519_frame_search_20261103 (frame selection, cofactor pairing, 4.71x/3.33x);
  RESEARCH_PKM_TOWER_ORACLE.md ("about half of V" line 202; seed-drawn coset line 226);
  curve25519_leads_wave4 l11_odd_kummer_design (selected coset over (p-1)/d cosets for
  coverage). Published: none that I know for this exact selection; Faugere-Gaudry-Huot-Renault
  J. Cryptology 27(4) 2014 is the symmetry precedent, unrelated mechanism.
novelty: "Low. Transfer of an in-repo selection idea to the prime-order case plus an explicit purity lemma and decay law."
prior_beats_current_prime_field_IC_baseline:
  toy_scale_F4_work_per_relation_vs_random_coset_arm: 0.85
  cold_end_to_end_S_with_selection_charged: 0.05
  at_cryptographic_size: 0.0
```

---

## P3

```yaml
id: IDEA-20261107-SEED-NEGATIVE-CONTROL
title: Cross-trace covariate test of the split-prime statistic against IC relation yield and rho cost
obstruction_attacked: >
  The seed has no known mechanism (report.txt Summary: "no known attack or generation mechanism
  explains why split primes would be avoided"). The lab's isogeny-class tests held the Frobenius
  field fixed (p2_prime_analogues/RESULTS.md: ICC ~ 0 within a class). They did not vary the
  Frobenius field (trace t) at fixed p. A coordinate effect of splitting, the only kind of effect
  the obstruction leaves open (round7_beyond DIRECTIONS.md item 3, Theorem 15.1), would appear
  only across traces.
claim: >
  Across prime-order curves over a fixed toy prime p, the normalised IC relation yield per attempt
  (ecbench ic.pipeline, prime-abscissa base of fixed size N, m=2 and 3) and the rho.negation cost
  S per target are statistically independent of E_L = #{odd primes l <= L : (Delta/l)=+1}
  (L=691 and the lab's other cutoffs), of the partial L(1,chi), and of the root counts of the 3-
  and 5-division polynomials, after conditioning on n. Expected size of any trace-dependence of the
  x-only solution count is O(t^2/p^2): the F_p-point count of the S_3 surface is
  (n^2 + n_t^2)/2-ish with n_t - n = 2t, so (n^2+n_t^2) = 2(p+1)^2 + 2t^2.
mechanism: >
  There is none in the generic model (obstruction as given). The test asks whether the seed could
  be a coordinate effect at all. A non-null result would be the first piece of evidence that it
  is, which is why it is worth one cheap run; a null closes the seed for the IC track.
prediction:
  point: "all partial correlations |rho| < 0.03; 95% CI contains 0; no covariate survives Bonferroni (family of about 24 tests, alpha 0.002)"
  probability_null: 0.97
discriminating_test:
  what: >
    ecbench (native, existing), 2000 random prime-order curves y^2=x^3+ax+b per prime, p at
    about 2^24, 2^28, 2^32, point count by existing native SEA/baby-step; covariates computed
    natively from t (Legendre symbols of Delta, partial L(1,chi), root counts of psi_3, psi_5 over F_p);
    outcomes: ecbench rho.negation S per target (1024 targets/curve, A/A arm) and ic.pipeline
    yield per attempt on a fixed-N prime-abscissa base (the p2tool analysis protocol, planted-effect
    power curve as in p2 RESULTS.md).
  positive_control: "planted +2% yield arm; must be detected with power >= 0.9 at the chosen sample"
  negative_control: "comparator leave-one-out and shuffled-trace permutation"
  runtime: "hours on one host at 24-28 bits; 32 bits as time allows"
falsification: >
  The null is falsified (and the seed becomes a mechanism lead) if any covariate shows a
  replicated (fresh seeds) partial effect >= 1% of yield or rho S with FDR < 0.01 at two primes.
  Otherwise the proposal reports the seed as closed for the IC and rho tracks at the tested sizes.
cost_band:
  arms_priced: "rho.negation (reference), ic.pipeline mitm and subtract arms (incumbent), A/A; S = ops/sqrt(n) at 0/0.1/1 GAE per native op"
  note: "this is a covariate test; no new algorithm is priced"
risks:
  - "Almost certainly null; its value is closure, not a gain."
  - "Selection bias in the seed itself (R-hacking on cutoff 691; strict_elkies REPORT.md says 23/4480 relabelings separate) means a positive test must be pre-registered on the covariate list above, not on L=691 alone."
  - "Winner's-curse false positives happened in p2 (c32A m3, not replicated); keep the follow-up rule."
lean_obligation: "none (statistical). The point-count identity for the S_3 surface is a derivation to be checked numerically in the same run (count solutions exhaustively at p <= 10^4)."
closest_baseline: >
  In-repo: p2_prime_analogues RESULTS.md (within-class), conductor_gap_rho_large_sample_20261018
  (rho vs conductor gap), generator_effect_ic_20261019 PLAN.md, nist_skip_criterion_20261025
  strict_elkies REPORT.md (the seed statistic). Published: Jao-Miller-Venkatesan, ANTS VII 2005 /
  J. Number Theory 2009 (random self-reducibility across isogeny classes); nothing I know ties
  the Elkies count to ECDLP cost.
novelty: "None as a mechanism; it is a falsifier of the seed."
prior_beats_current_prime_field_IC_baseline:
  any_non_null_effect: 0.02
  beating_baseline: 0.0
```

---

## Considered and killed (so nobody re-derives them)

| id | idea | kill reason (derived; not experimentally tested here unless marked) |
|---|---|---|
| K1 | Isogeny-pullback tower base on prime-order E (Vélu/Elkies l-isogeny, base = preimage of a Kummer point) | A point-preimage tower needs rational kernel points; on #E=n prime, phi: E(F_p)->E'(F_p) is injective (gcd(deg,n)=1), so a tower's F_p-rational preimage has <=2 points. Set-pullback base equals IC on E' (class-invariance measured null, p2 RESULTS.md; framing in isogeny_class_difficulty_20261008). This is exactly why the isogeny tower works for Curve25519 (cofactor 8 gives rational kernels) and not here. |
| K2 | Prime-order surrogate for E1Q via twist l-torsion (T with pi T = -T, so x(T) in F_p, kernel polynomial splits over F_p) | The translation tau_T is defined over F_{p^2}, not F_p. V is not invariant: P in E(F_p) gives P+T outside E(F_p). Folding V needs rational translation. Only the correspondence survives, which gives no coordinate sign flip. |
| K3 | Mixed E/twist relations as extra relations | Impossible by the purity lemma of P2 (twist part must sum to O). Waste is in columns, not in solutions. |
| K4 | Base in the torus coset of F_{p^2} with trace map to E(F_p) | Trace kills the twist constraint, but the base needs d^m about p^2 (Diem/Weil-restriction count), so d about p^{2/m}: the tower is twice as deep. Strictly worse than the Dickson torus base (x in F_p) that already exists for p = -1 mod 2^k. |
| K5 | Index calculus on the isogeny graph with edge-label unknowns, relations from closed class-group cycles | Labels are k-independent: phi(P)=lambda P', phi(Q)=lambda Q' with the same k on every curve. Cycle relations give lambda products, never k. This is the stated obstruction. |
| K6 | Endomorphism alpha=a+b*pi of smooth norm as an x-line Lattes map | On E(F_p) pi acts as 1, so alpha(P)=(a+b)P; the x-line map R_alpha has degree N(alpha) >= |Delta|/4 about p and equals [a+b] only mod x^p-x. Evaluating the isogeny chain returns what the algebra already gives. |
| K7 | Union of a Kummer and a Dickson base | One of p-1, p+1 has v2=1 for odd p, so both 2-power towers cannot be deep at the same p. |
| K8 | Cross-target parametric Groebner / comparison matrix in x_R | The relation variety over all x_R has |V|^m/m! points, about p/m!; same counting bound as DIRECTIONS.md section 0. |
| K9 | Twist-smoothness or Tate/MOV style leakage | Embedding degree is ord(p mod n), identical across the isogeny class; known dead end. |

## Literature searched (this session, web, standard mode)
- Petit-Kosters-Messeng, PKC 2016 (https://christophe.petit.web.ulb.be/files/16PKC_primeECDLP.pdf): factor bases F = {(x,y): L(x)=0}, maps L_j from auxiliary-curve structure, 2-adic remark for P-224 (p-1 = 2^96 * N'). Tower of 2-power membership.
- Faugere-Gaudry-Huot-Renault, J. Cryptology 27(4):595-635 (2014), ePrint 2012/199; Faugere-Huot-Joux-Renault-Vitse, Eurocrypt 2014 (small-order torsion summation polynomials): the torsion mechanism that P1 explicitly avoids needing.
- Nagao, ePrint 2013/548 (Riemann-Roch decomposition; revisions correct key formulas, so P1 relies on the repo's own faithfulness checks, not on the paper).
- I did not verify the Bernstein-Lange preprocessing results or JMV in this session beyond their citation in repo files.

## Handoff notes for the Coordinator
- Records only; no card, no run, no PR created. If P1 or P2 is taken, an experiment-director card must freeze arms, seeds, kill criteria and the 0/0.1/1 GAE pricing before the run; the run needs `.resume/RESUME.md` and bench.record/v1 per CLAUDE.md.
- P1's decisive cells (m>=5) do not fit in 15 GB (`problems/tree-glued-pdp-separator-coupling.md`); do not read a m=3 result as meeting the round7 improvement rule.
- Suggested problem-file action (not done by me): none new; P1 and P2 should link to the existing tree-glued record.
