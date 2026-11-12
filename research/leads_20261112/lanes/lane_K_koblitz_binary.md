> Agent report, verbatim (2026-11-12). Proposals only; nothing was run; every statement in this file is `[pcc:unverified]`.

# Lane K leads: Koblitz curves and binary fields E(F_{2^n}) (2026-11-12)

Author: Idea Generator (proposals only). Standing of everything below: `[pcc:unverified]`. Nothing was executed:
this session had no shell tool. Every number is either quoted from a file on disk (path given) or marked
"derived" (hand arithmetic on quoted numbers; the executing agent must recompute it before use). No repository
file was edited. Implementation language for every test: Rust or C (existing `ecbench`, `c1_twisted_line_ladder`,
`gaudry_quartic_bin.rs`, native closure); no Python. Note that the deep-chain engine in
`/home/user/ml-cryptanalysis/csd/` is legacy Python: new instance emitters or closures for it must be native.

## Sources checked

- research-kb MCP: NOT AVAILABLE in this session (not called). No `source_id` receipts exist. The sibling lead atlas [pcc:unverified]
  bodies are vault-encrypted; only the id lists were read (`atlas_lead_ids.txt`, `EXISTING_LEADS.txt`). "Nearest
  existing lead" below is by id and by the neighbouring result files on disk, not by atlas body.
- Briefing files: `scratchpad/leads/BRIEFING.md`, `lane_B_binary_ic.md`, `lane_A_falldegree.md`.
- Repository (read): `docs/ecdlp-hardness-corpus/WEAKNESSES.md` section 6 (rows E-BK-028, E-SP-086, FD-013, E-GB-091);
  `docs/ic/LEADERBOARD.md` lines 17-30, 96-152; `research/ecbench_frobenius_orbit_vs_rho_20261014/README.md` and
  `tables/derived-phase-means.jsonl`; `research/ecbench_lazy_orbit_members_20261018/README.md`;
  `research/ecbench_generator_orbit_20261022/README.md` and `tables/{band,table}-a0-n37.md`;
  `research/ecbench_pair_claw_20261003/README.md`; `research/ic_candidate_tournament_20260915/runs/round-0026/REPORT.md`;
  `research/n83_e2e_finalist_results_20261017/RESULT.md`; `research/gaudry_k4_char2_redteam_20261026/REPORT.md`;
  `research/c1_twisted_line_ladder_20261027/README.md`; `research/subgroup_trap_20261107/README.md`;
  `research/lead_sweep_20261110/LEAD-ICB-composite-degree-c2pnb/RESULT.md`;
  `docs/ecdlp-hardness-corpus/lean-round6-20261108.json` (grep only); `src/cryptanalysis/gaudry_cubic.rs` (header);
  `/home/user/ml-cryptanalysis/experiments/ic-bounds-binary-deep/PLAN.md`.
- Literature (web search, abstract level; full texts NOT read, two fetches of the GGMP paper returned HTTP 403):
  - Galbraith, Granger, Merz, Petit, "On Index Calculus Algorithms for Subfield Curves", SAC 2020,
    [ePrint 2020/1315](https://eprint.iacr.org/2020/1315): Frobenius-invariant factor bases, relations reduced by up
    to 1/n, linear algebra by n^2, "a reduction by a factor of 1/n is the best one could hope for", improved symmetry
    breaking. I could not confirm which bases (lines, subspaces) or which fields they use.
  - Galbraith, Gebregiyorgis, [ePrint 2014/806](https://eprint.iacr.org/2014/806) (binary summation polynomials, SAT vs
    Groebner); Trimoska, Ionica, Dequen, [ePrint 2019/313](https://eprint.iacr.org/2019/313) and WDSat
    [arXiv 2001.11229](https://arxiv.org/abs/2001.11229) (XOR-SAT Weil-descent solver, symmetry breaking).
  - Faugere, Gaudry, Huot, Renault, "Using symmetries in the index calculus for elliptic curves discrete logarithm",
    J. Cryptology 27 (2014), [ePrint 2012/199](https://eprint.iacr.org/2012/199) (found via a search snippet only).
  - Not found in the searches: a source that combines a twisted line theta*F_Q (theta in a proper subfield) with a
    Frobenius fold of the other subfield for Gaudry-type descent on subfield curves. That is a negative search result
    of three queries, not a novelty proof.

## 0. State of the lane, and what the evidence says about where a lever can live

Quoted from disk:

- Best cold matched IC/rho: 1.57x (`LEADERBOARD.md` line 25) and 1.27x (line 29), both on `icv1-f2m29-tm40309-30c52b96`
  at r = 2^15.4. The generator-orbit README says the n29 crossing "rests on rho-frob's fixed 133-GAE set-up" and the
  gap "grows with r: 1.15x at n31, 1.93x at n23, 3.89x at n37" (`ecbench_generator_orbit_20261022/README.md`). At n41
  the orbit arm is 18.16x [15.67, 21.55] (`ecbench_frobenius_orbit_vs_rho_20261014/README.md`).
- The generic claw, which has no relation phase, is 2.195 [1.897, 2.546] x strong rho pooled over six Koblitz curves
  r = 2^32.1 to 2^47.2, "the ratio does not shrink with r" (`ecbench_pair_claw_20261003/README.md`); 2.51x at f2m41, where
  the orbit IC arm is 18x. Its floor is (c + 1/c)/sqrt(n) against rho's sqrt(pi/(4n)).
- Phase shares of the orbit IC arm (`derived-phase-means.jsonl`): n37 relations 13,349 of 19,359 GAE (69 %), n41
  1,373,079 of 1,788,686 (77 %). Linear algebra does not appear. The measured slope 0.165 [0.117, 0.213] per ln r is
  the 2/3 against 1/2 exponent gap of a two-summand pair-table IC.
- Binary D4 (Gaudry, x in F_Q, n = 4m), red-team: corrected IC/rho 5.2 at m = 15 (range 2.4 to 15.0), IC relations
  exponent 1.07, rho 2.00, ratio falls 2^-0.93 per unit m, crossover m* about 17.6 [16.3, 19.2]; LA about 0.4 % of
  GAE at m = 15; "the curves are generic ordinary curves with j = 1/b, and j is not in any proper subfield.
  Aut(E) = {+-1}, so negation is the only automorphism rho can use"
  (`gaudry_k4_char2_redteam_20261026/REPORT.md` lines 7-18, 53-60). Lane B quotes 191,435 F_q mults per trial and about
  192 trials per relation at m = 12.
- Twisted line, subfield curves, n = 4 over F_Q (`c1_twisted_line_ladder_20261027/README.md`): the twisted all-lift
  line u*F_Q has the same yield as the standard line (pooled 1.0238) and the same per-trial cost (0.9606 [0.9279, 0.9913])
  on the tau = 1 curves (a, b in F_16, Tr_{16/2}(b) = 1, hence b not in F_4, j in F_16 \ F_4). The tau = 0, b = 1 curve
  A0 (j in F_2, Koblitz-like) is subfield-trapped (`subgroup_trap_20261107/README.md`: "A0 is subfield-trapped (P1 and P2
  PASS)"). Neither study folds by Frobenius, extracts roots or runs end to end ("Not done").

What follows from this (derived, three consequences that shape the proposals):

1. Two-summand IC cannot have an exponent better than 2/3 in any generic-add-limited setting: relation work is
   r/(B s) adds, table build or LA is B^2/s, so the optimum is r^{2/3}. The Koblitz pair-table rows are therefore
   constants against a floor, and the claw already sits on that floor (2.2x). No fold, table layout or LA trick moves the
   exponent. A lever must replace the "one group addition per trial" cost by an algebraic membership test, i.e. m >= 4
   summands with an F_Q-structured system. The only such route measured in the lab is D4.
2. A table-free m = 2 algebraic oracle (the F1 factorisation of lane A, Phi*_C = P o psi_C, F_2-affine in A for fixed C)
   costs 2^k C-values per target, so total r*c/n against r^{2/3}/n for the table. Flat degree at m = 2 has no economic
   use. (Derived; see "Considered".)
3. A Koblitz curve proper (E/F_2) over F_{2^{dm}} is defined over F_Q = F_{2^m}, so #E(F_{Q^d}) is divisible by
   #E(F_Q), and the prime-order part is at most about Q^{phi(d)} (d = 4: Q^2, d = 3: Q^2, d = 5: Q^4). Rho then costs
   about Q^{phi(d)/2}. The A0 trap in `subgroup_trap_20261107` is this effect. So a Gaudry-type line on a Koblitz curve
   cannot win for d = 3, 4. The usable family is E/F_{2^e} with j(E) not in F_2 and n = e*k, gcd(e, k) = 1, where the
   group order is the full Q^e and a line theta*F_Q with theta in F_{2^e} is stable under the F_{2^e}-Frobenius
   (order k). That family is what the first two proposals use. It is not Koblitz (j not in F_2), it is a weak class
   defined by a subfield of definition, which is why the matched rho must be strengthened with it.

---

## 1. LEAD-KOB-fold-twisted-line-d4 (rank 1)

```yaml
id: LEAD-KOB-fold-twisted-line-d4
title: Frobenius-fold the twisted all-lift line of the binary D4 solver on subfield curves E/F_16 (odd extension degree k)
obstruction: >
  Binary D4 loses to rho by a corrected 5.2x at m = 15 (range 2.4-15), crossover extrapolated at m* ~ 17.6 [16.3, 19.2]
  (gaudry_k4_char2_redteam_20261026/REPORT.md). Its cost is relation-bound (LA 0.4 % of GAE at m = 15) with
  trials-per-relation fixed at about 192 ("the double count ... fixed for every curve", lane B), so the only free
  multiplier is the number of relations required. The programme's Koblitz fold (n-fold, GGMP) is incompatible with a
  Gaudry F_Q-line on a curve over F_2 (subfield trap, group order Q^2), so the two best levers have never been combined.
claim: >
  Let E: y^2 + xy = x^3 + a x^2 + b over F_16 with Tr_{16/2}(a) = 0, b in F_16 \ F_4 (so j = 1/b is in F_16 \ F_4, not in
  F_2 or F_4), k odd, n = 4k, Q = 2^k, theta in F_16 \ F_4, and L = {P in E(F_{2^n}) : x(P) in theta*F_Q}.
  (i) L is closed under the F_16-Frobenius phi (x = theta*u maps to theta*u^16) and phi acts on the x-values of L with
  orbits of size k except for u in proper subfields, so the folded base has about (Q - 1)/k columns.
  (ii) On the r-part, phi acts as a scalar mu with mu^k = 1 (mod r), so every relation found by the unchanged D4 solver
  is a row of a folded matrix with entries +-mu^j; the number of relations to full rank is within [0.8, 1.4] x ((Q-1)/k + 2)
  / (Q - 1 + 2) of the unfolded count, and the solved logarithms match planted ones.
  (iii) At equal per-relation cost the folded IC against the matched rho (class <-1, phi>, size 2k) is
  R0(m) / sqrt(k) with R0 the red-team's generic-curve ratio: about 1.34 at m = k = 15 (range 0.62-3.9) and the crossover
  moves from m* ~ 17.6 to about 15.5 [14.2, 17.1] (derived below). The unfolded twisted-line arm against the same
  matched rho would be R0 * sqrt(k), i.e. the class is harder for IC than a generic curve unless it is folded.
mechanism: >
  F_{2^{4k}} = F_16 (x) F_{2^k} when gcd(4, k) = 1. The line theta*F_Q is an F_Q-line (so Gaudry's Weil descent,
  symmetrised coordinates and the 64-dimensional quotient survive: this is the structural point of obstruction
  E-BK-028, "symmetric coordinates survive only when V^(i) stays l-dimensional") and it is fixed as a set by
  phi = Frob_16, which fixes theta and acts on u in F_Q as u -> u^16, a generator of Gal(F_Q/F_2) because gcd(4, k) = 1.
  The standard line x in F_Q has no such fold on a curve with j not in F_2 (Frob_2 is not an endomorphism), and on a curve
  with j in F_2 it is trapped. Fold: the equivariance constraints l(phi P) = mu l(P) identify the k points of an orbit,
  so each relation carries k times more information per unknown; this is the GGMP mechanism with the Frobenius of the
  base field F_16 acting on the F_{2^k} factor. The solver is unchanged (the ladder measured twisted/standard cost
  0.9606 [0.9279, 0.9913], yield 1.0238).
  Derivation of (iii) (hand arithmetic): relations needed fall by k, per-relation cost is unchanged, canonicalisation
  adds about 4k squarings per relation against about 3.7e7 mults per relation (derived from 192 trials x 191,435 mults,
  lane B), so IC_f = IC_0 / k. Matched rho gains sqrt(2k) instead of sqrt(2): rho_f = rho_neg / sqrt(k). Hence
  IC_f / rho_f = (IC_0 / rho_neg) / sqrt(k) = R0 / sqrt(k). With R0(15) = 5.2 (range 2.4-15.0): 1.34 (0.62-3.9); with
  R0(m) = 5.2 * 2^{-0.93 (m - 15)}: m = 13: 18.9/3.61 = 5.2; m = 17: 1.43/4.12 = 0.35; the root of R0(m)/sqrt(m) = 1 is
  m ~ 15.5; the red-team crossover interval shifts by -log2(sqrt(15))/0.93 = -2.1.
  Side effect: the fold postpones the LA-bound regime, because LA/relations falls by k (LA (Q/k)^2 against relations Q/k).
prediction: >
  Toy (counts, exact): k = 5 (Q = 32): 31 x-values, u^16 = u only for u = 1, so 6 orbits of size 5 plus 1 fixed = 7 columns;
  k = 7 (Q = 128): 126/7 = 18 orbits + 1 = 19 columns; k = 9 (Q = 512): orbit sizes 9 and 3 (u in F_8), (504)/9 = 56 +
  2 + 1 = 59 columns (derived; executing agent recomputes). Relations to full rank, folded/unfolded:
  k = 5: (7 + 2)/(31 + 2) = 0.27; k = 7: 0.17; k = 9: about 0.11-0.12, each within +-25 %.
  End to end (IC counted GAE, w = 0): folded/unfolded IC cost ratio in [1/(1.25 k), 1.4/k] at m = k = 11, 13; IC_f / matched-rho
  1.34 at m = 15 against the red-team's 5.2 for generic curves. Direction and size are exact consequences of the count;
  the uncertain quantity is R0 (red-team range 2.4-15), which this lead does not change.
discriminating_test: >
  Smallest separating run, step 1 (minutes, native, no solver): extend the existing Rust tool `c1_twisted_line_ladder`
  (curves A1, A2: a, b in F_16, tau = 1) with (a) orbit columns under phi, (b) relation collection by its existing
  pair-table decomposition counts at k = 5, 7, 9 for random targets aG + bQ', (c) an F_r elimination with known planted
  logarithms, reporting targets-to-full-rank folded vs unfolded and the recovered log. Null: folded rank growth is not
  faster than unfolded, or recovered logs mismatch (phi not scalar on the r-part, or the base not closed).
  Step 2: one end-to-end cell at m = k = 11 (n = 44, Q = 2048) with `gaudry_quartic_bin` on a twisted line: IC unfolded
  vs folded vs matched rho (class 2k = 22). The k = 4 replication plan prices unfolded m = 15 IC at about 30 h; folded
  it is about 2 h (derived, /15).
falsification: >
  Step 1: ratio folded/unfolded above 0.5 at k = 7 (null), or any recovered logarithm wrong, or fewer than 80 % of
  line x-values in orbits of the predicted size. Step 2: folded IC/unfolded IC above 0.25 at m = 11 at equal trials per
  relation, or a matched-rho arm that gains more than the class size 2k (it would show an extra automorphism and void R0/sqrt(k)).
  Claim (iii) is falsified if the measured IC_f/rho_f at m = 13 is above 15 (the red-team's upper range 15.0, divided by
  sqrt(13), is 4.2; above 15 would mean the fold is not the whole story).
cost_band: >
  Arms: (A) unfolded twisted-line D4 IC; (B) folded D4 IC; (C) rho-neg (generic reference, as in the red team);
  (D) matched rho on class <-1, phi> (size 2k), which requires a `rho.signed_frobenius` variant with base-field exponent 16
  instead of 2 (new arm, to be written). Counted: group additions, solver F_Q mults converted at GAE = mults / fp_muls_per_add
  (159 at m = 15, 161 at m = 17 in the red team). Unpriced natives, each priced at 0, 0.1 and 1 GAE per operation per
  AGENTS.md 5a and ranked only where the order holds at all three: (B) orbit canonicalisation (about 4k squarings per
  relation), LA word operations; (D) canonicalisations (the red team puts rho's unpriced work at about 1.06-1.07 of rho).
  Both folded and unfolded IC are fully priced through the counted solver (99.85 % of IC cost in the red-team accounting).
  The band is expected to leave the order (B) vs (D) unchanged because relation work is 99 % of B; the claim is not
  valid until the band is computed. Prices are host-specific; operation counts are the result.
controls: >
  A/A on rho arms (two seeds). Positive control: planted target recovered through the folded LA at k = 5. Negative
  control 1: the same fold on the standard line of the trapped curve A0 (b = 1) must give no gain (Q^2-sized group,
  relations land in H). Negative control 2: a random line of the same F_2-dimension (not a theta-line) has no phi-stability,
  so the fold must not apply and the rank growth must equal the unfolded one. Negative control 3: k even (not coprime
  to 4) must be rejected by the orbit-size check.
risks: >
  (1) Engineering: `gaudry_quartic_bin.rs` is written for y^2 + xy = x^3 + a x^2 + b with the line x in F_Q; a theta-line
  is the standard line of the isomorphic model with a1 = 1/w, w^2 = theta, so the solver needs a general-a1 path. The
  Frobenius-form reduction (even exponents in s2, s3, pi, "true on every curve tried, not proved") must be re-checked
  on that model; the ladder found the same even-exponent structure for S_5 in characteristic 2, which transfers
  (e_i(theta*u) = theta^i e_i(u)) but is not the lane-B solver. (2) The class is thin: j in F_16 \ F_4 has 12 values (3 Galois
  orbits) and a in 2 classes, so instances are (curve, k) pairs and r must be (near-)prime, to be censused. (3) The
  matched rho must be rho with class 2k; if rho-frob is unavailable for base-field exponent 16, the comparison is
  against a weaker rho and is not a valid distance. (4) R0 inherits the red team's uncertainty (2.4-15) and the
  unit-price direction is undetermined (0.31-3.1). (5) Not Koblitz: this does not touch the Koblitz rows; it is a weak
  class row. (6) Equal-advice audits apply to nothing here (no table), but the structure assumption "all-lift" needs
  Tr(a) = 0 over F_16 and gives Q - 1 x-values only for those curves.
closest_baseline: >
  Binary D4 red-team row (5.2x at m = 15, m* ~ 17.6) and the ladder's unfolded twisted-line cost 0.9606.
nearest_existing_lead_and_difference: >
  LEAD-ICB-frobenius-orbit-factor-base / LEAD-ICB-orbit-linear-algebra (Koblitz, prime n, pair-table, no algebra);
  LEAD-ICB-composite-degree-c2pnb (sweep RESULT: lifted-from-subfield toys are trapped, max lifted ratio 0.0863, E lifted
  from F_Q); LEAD-ICB-frobenius-search-symmetry and LEAD-ICB-nonlinear-frob-stable-bases (bodies unreadable, ids only);
  `c1_twisted_line_ladder_20261027` and `subgroup_trap_20261107` (twisted line yield and cost equal, no fold, no root
  extraction). Difference: here E is not defined over F_Q (j not in F_2), the line is phi-stable, the fold is by the
  Frobenius of the *other* tensor factor, and the matched rho is the strengthened one. GGMP (2020/1315) supplies the fold
  principle; whether it was ever applied to Gaudry-type lines was not found (full text not read).
lean_obligation: >
  Load-bearing mathematics, small: (L1) for gcd(e, k) = 1, theta in F_{2^e}, the set theta*F_{2^k} is stable under
  x -> x^{2^e}, the map u -> u^{2^e} has order k on F_{2^k} and generates Gal(F_{2^k}/F_2), and F_{2^k}(theta) = F_{2^{ek}}
  iff theta generates F_{2^e} over F_2 (statement over Mathlib `GaloisField`); (L2) functoriality: for an endomorphism
  phi of E commuting with the relation, sum P_i = R implies sum phi P_i = phi R, and phi acts on a cyclic prime-order
  subgroup as a scalar of order dividing k; (L3) NOT claimed: r ~ Q^4 is computed per instance by point counting, not
  assumed. If the programme relies on the Koblitz-exclusion statement of section 0 (consequence 3), the obligation is
  #E(F_{q^d}) divisible by #E(F_q) for E/F_q (check `search_theorems`; research-kb unavailable here).
next_steps:
  - "1. (executor-mechanical via director card, 2 h) Census: for k odd in 5..21 and the curves a in {0, ...} Tr_{16/2}(a) = 0, b in F_16 \\ F_4, compute #E(F_{16^k}) from the F_16 point count and the Lucas sequence, factor, keep (curve, k) with r prime and cofactor <= 16; also log phi's scalar mu mod r. Zero solver work."
  - "2. (Sonnet, 0.5 day) Step-1 toy fold test in the Rust ladder tool at k = 5, 7, 9 with planted logs and the three negative controls."
  - "3. (Sonnet, 0.5 day) matched-rho arm with base-field exponent 16 and its A/A in `ecbench` (class size 2k), same targets."
  - "4. (Opus review, Sonnet implement, 2-3 days) general-a1 path in `gaudry_quartic_bin.rs`, re-check of the Frobenius-form guard, then the m = k = 11 end-to-end cell, then 13 and 15 on EKS spot per AGENTS.md 9a."
  - "5. (director) preregister the section-0 formula R0(m)/sqrt(k) and the 5a band before step 4; claim only the folded/unfolded ratio and the distance to matched rho, no ranking until the band holds."
```

---

## 2. LEAD-KOB-flat-degree-crossover-bar (rank 2; cheap, low prior, decides how much today's flat degree is worth)

```yaml
id: LEAD-KOB-flat-degree-crossover-bar
title: Price what "LFD flat at 4" would be worth, and test the one premise that matters (m = 3, 4 square systems at D <= 4)
obstruction: >
  Today's deep-chain interim: LFD 4 on random subspaces dimension 14 in F_{2^22}, 5 on the trace-zero hyperplane, 4 on
  units at n = 2k - 1 (briefing; unverified). Lane A: closure degree 4 for n = 15..33, stall (>= 5) at n >= 35 for random V
  (data-bin-fd-instances.json as quoted in lane A). All of this is m = 2. For m = 2, section 0 consequence 1 shows
  no exponent below 2/3, so flatness there is worth nothing economically; the regime where a flat degree would pay is
  m >= 4 with k just above n/4, where relations are free (yield >= 1 per solve) and only LA (about 2^{n/2}/(4 n^2)
  folded, below rho by about n^{1.5}/4.5) and the per-solve cost remain. WEAKNESSES E-SP-086 ("constant-degree arms
  contradict the published growth of the degree of regularity with n at m = 2") is the live obstruction this attacks.
claim: >
  Under a model with flat F_2 closure degree D, the m = 4 Weil-descent IC with k = n/4 + 2 crosses the matched
  Koblitz-class rho (class size 2n) at n* = 205 (D = 4), 280 (D = 5), 345 (D = 6), and is above rho by a factor about 190
  at n = 163 for D = 4 (derived, +-30 in n*, wider in the deficit). Premise to test: the degree-<= 4 Boolean closure
  of the m = 3 square system (N = 3k variables) and the m = 4 square system (N = 4k) stalls (1 not in W_4) at N = 32-33,
  i.e. flatness does not extend from m = 2 to the regime where it would matter.
mechanism: >
  Model (derived, hand arithmetic, to be recomputed in Rust): r ~ 2^n, base B = 2^k, k = n/4 + 2, columns c = B/(2n),
  yield per solve y = 2^{4k-n}/24 = 10, solves c/y, each costing M(N, <= D)^2.4 / 64 word operations with N = n + 8
  variables, priced at 1 GAE per word operation; LA (c^2 x 5 x 0.015 GAE, the 0.015 coming from the red-team's 0.4 % at
  m = 15, derived); rho-frob = sqrt(pi/(4n)) 2^{n/2}. Crossing when M(n+8, <= D)^2.4 / 64 <= 4.4 sqrt(n) 2^{n/4}.
  Spot values: D = 4, n = 200: lhs 1.3e17, rhs 7e16 (above); n = 220: 3.1e17 against 2.3e18 (below). D = 5: n = 250
  lhs 1.2e22 against 4.5e20 (above); n = 300 1e23 against 2.9e24 (below). D = 6: n = 330 5.2e27 against 5.5e26 (above);
  n = 380 3.9e28 against 3.4e30 (below). At n = 163, D = 4: 1.9e16 against 1.0e14. The same arithmetic gives the
  "LFD ceiling for a crossing at n": about D = 2.3 at n = 163, 3.1 at 233, 3.7 at 283, 4.9 at 409, 6.5 at 571 (derived).
  This is the FPPR claim stated as a bar, and shows that even a flat D = 4 would not put n = 163 in reach.
prediction: >
  P(closure degree <= 4 at the m = 3 square cells k = 10, 11 and the m = 4 square cell k = 8) = 0.15; P(stall at D = 4 in
  at least one of those) = 0.85. Under a stall the D = 4 row is dead and the bar is D >= 5, n* >= 280 at best. The
  prediction is registered before the run; numbers above are conditional on flat D.
discriminating_test: >
  Native degree-<= 4 Boolean closure over GF(2) (M4RI; lane A lead A3's native `lfdclose2`-style tool, not on disk in this
  repository, port a few hundred lines of C) on (a) m = 3 summation system, k = 9, 10, 11 (N = 27, 30, 33; columns
  M(33, <= 4) = 46,938), (b) m = 4, k = 6, 7, 8 (N = 24, 28, 32; M(32, <= 4) = 41,449), random V of dimension k in
  F_{2^n} with n = N, plus the same V with the theta-line structure as control. Unsatisfiable-instance filter by
  enumeration as in lane A. Minutes per instance. Decision: 1 in W_4 on all cells flags the bar's D = 4 row live
  (then a D = 5 closure at N = 33, 1.0e6 columns, is the next cell); a stall anywhere closes it at D = 4.
falsification: >
  The claim (premise) is falsified if every m = 3, 4 square cell above falls at D <= 4 on all instances; the model
  itself is falsified only by measured solver costs, not by this test. If a stall occurs at k = 10 (m = 3) the bar is
  reported as "no flatness where it matters" and the lead closes.
cost_band: >
  Arms: IC (model), rho-frob matched, plus the trivial m = 2 table arm of section 0 as reference (r^{2/3}/n). All native
  work in the model is word operations of F_2 linear algebra and is the dominant, unpriced item: at w = 0 the solve cost
  vanishes and IC "wins" at every n above the LA crossing, at w = 1 the values above hold, so the order does NOT hold
  across the 5a band and no ranking is claimed; the lead delivers the w = 1 bar and the w = 0 bound as the two ends.
  The closure runs cost minutes each locally (N <= 33: matrices under 50k columns).
controls: >
  Positive: reproduce lane A's logged closure at n = 33 (1 in W_4) and the n = 35 stall with the same tool; negative:
  a satisfiable instance must stall; A/A: two instances per cell.
risks: >
  (1) The model prices the solve as dense M^2.4 / 64; real F4 on these sparse systems can be cheaper (then n* falls by
  tens) or the Weil system at m = 4 over F_2 may not have N = n + 8 variables at D = 4 (the S_5 expansion has higher
  degree in c than the m = 2 case; the chain presentation adds 2n variables, ICE chained-semaev results apply). (2) Closure
  degree is a proxy; WEAKNESSES row E-GB-064 says first/last fall degrees are incomparable with the solving degree.
  (3) A stall at D = 4 shows nothing about D = 5. (4) The F_2 closure degree is the wrong proxy for F_Q-line systems (the
  C5 cells of PLAN.md predict LFD 6 and 7 for the scaled subfield at k = 10, 12 while the F_Q solver is flat at
  Macaulay degree 11 in D4); this lead is about unstructured F_2 subspaces only.
closest_baseline: >
  Lane A leads A2/A3 (diagonal closure laws, m = 2); FPPR 2012 (2^{omega t}, first fall degree assumption).
nearest_existing_lead_and_difference: >
  LEAD-ICE-first-fall-degree-assumptions, LEAD-ICE-descent-closure-degree-at-scale (plain-XL, l <= 6, saturating at D = 4,
  null-like), LEAD-ICE-chained-semaev-dreg-vs-semiregular (dreg vs semiregular, chains). Difference: none of them price
  a flat degree into a crossover n or test the square m = 3, 4 regime; this one converts a degree into a bar.
lean_obligation: >
  None for the model (arithmetic) or the measurements (certified closures carry the checkers of PLAN.md). If the
  programme ever relies on "closure degree <= D for all n" the obligation is a kernel statement of the shape of
  `SelectorExtension.lfd_le_depth_add_two_of_two_ne` for m >= 3, which does not exist.
next_steps:
  - "1. (Sonnet, 0.5 day) Port the degree-<= 4 closure to C over M4RI and reproduce n = 33 / n = 35 of lane A."
  - "2. (Sonnet, 0.5 day) Native m = 3, 4 instance emitter (summation polynomial by Weil restriction over GF(2), unsatisfiable filter)."
  - "3. (executor-mechanical, 1 h) Run the six cells and the theta-line control; record 1 in W_4 per instance."
  - "4. (director, 1 h) Recompute the bar table in a small Rust example (M(N, <= D) exact), publish n* with the w = 0 and w = 1 ends."
  - "5. (main session) Decide D = 5 closure at N = 33 only if all D = 4 cells fall."
```

---

## 3. LEAD-KOB-fold-twisted-line-d3 (rank 3; prior about 10 %; needs a new small solver)

```yaml
id: LEAD-KOB-fold-twisted-line-d3
title: Binary Gaudry/Diem n = 3 on E/F_8 subfield curves with the folded twisted line (LA-bound window)
obstruction: >
  For d = 3 the Gaudry exponent is LA-bound (relations Q, LA Q^2 against rho Q^{3/2}), so IC loses by about
  Q^{1/2}; with no large-prime variant that is why the odd-char n = 3 oracle (`src/cryptanalysis/gaudry_cubic.rs`, x in
  F_p, meet-in-the-middle O(log p) per base element) and B1 are refuted. The fold of lead 1 reduces columns by k and LA
  by k^2 while rho gains only sqrt(2k): at k about 13 the LA term alone is below rho.
claim: >
  For E over F_8 with j in F_8 \ F_2, gcd(3, k) = 1, n = 3k, theta in F_8 \ F_2, the folded twisted line (columns
  (Q - 1)/k) gives IC / matched rho (class 2k) <= 1.0 at w = 0 for some k in [11, 17], provided the measured cost of one
  solve times trials per relation is at most about 3.8e4 F_Q mults at k = 13.
mechanism: >
  Relations: S_4(theta u1, theta u2, theta u3, x_R) = 0 is 3 equations over F_Q in 3 unknowns (symmetrised e1, e2, e3),
  yield 1/(3! * 4) per target (by analogy with D4's 1/(4! * 8), derived), so T = 24 trials per relation. Model at k = 13
  (Q = 8192, r ~ Q^3 = 2^39, sqrt r = 7.4e5): columns 630; rho class = sqrt(pi/(4k)) sqrt r = 1.8e5 GAE; LA =
  630^2 * 5 * 0.015 = 3e4 GAE (0.015 GAE per LA op from the red team's 0.4 % at m = 15, derived); relation phase
  630 * T * C3 / 159 with C3 mults per solve and 159 mults per GAE (red team, m = 15; it will differ at n = 39). Parity needs
  T * C3 <= 3.8e4, C3 <= 1.6e3. My scaling guess from D4 (191,435 mults at quotient dimension 64) to a quotient dimension about
  36 is C3 ~ 5.7e4, which gives a ratio about 30 at k = 13, i.e. the expected outcome is NO window. The lead exists to
  measure C3, which is the single unknown.
prediction: >
  P(C3 <= 1.6e3 at k = 9..11) = 0.10. If it holds, ratio at k = 13 about 0.95 and a window k = 12..15 (relation phase falls
  below rho at Q^{1/2} > 24 T C3 / (159 k), LA exceeds rho beyond k about 15-17). If not, ratio about 30 at k = 13
  and the lead closes with C3 recorded.
discriminating_test: >
  Step 0 shared with lead 1 (fold audit at d = 3 on the two Galois classes of j in F_8 \ F_2: orbit count, relations to full
  rank folded vs unfolded at k = 5, 7). Step 1: native 3-unknown solver (symmetrise, Weil-restrict, Groebner or resultant
  to a univariate) in the style of `gaudry_cubic.rs` but char 2 and O(1) per target, counted F_Q mults per trial at
  k = 7, 9, 11, 13 (200 trials each). Decide by C3 against the threshold 1.6e3.
falsification: >
  Measured T * C3 > 3.8e4 at k = 13 (extrapolated from k = 7-11 with the ladder's flat slope), or the fold audit of step 0
  failing as in lead 1.
cost_band: >
  Arms: folded D3 IC, unfolded D3 IC, matched rho (class 2k), generic rho-neg. Counted: solver mults (priced at GAE =
  mults / mults-per-add measured for F_{2^39}, to be measured, not taken from m = 15), LA word ops at 0, 0.1, 1 GAE. LA
  is not negligible here (about 15 % at k = 13 in the model), so unlike lead 1 the 5a order is expected to depend on the LA
  price; no ranking claimed before the band.
controls: A/A; planted target; negative controls as lead 1 (trapped curve with j in F_2, non-theta line, k divisible by 3).
risks: >
  Prior 10 %, stated; a new solver is a day or two of work for a likely negative. LA grows as 2^{2k}/k^2 against rho
  2^{1.5k}/sqrt k, so any window closes by k about 17 without a large-prime variant, which would be a second lead.
  The odd-char analogue is refuted (B1) for the same LA reason.
closest_baseline: B1 (Gaudry-Diem x in F_p, k = 2..5) REF; `gaudry_cubic.rs`.
nearest_existing_lead_and_difference: >
  LEAD-ICP-gfp5-oa-sdhp-jv / odd-char Gaudry leads (prime fields); LEAD-ICB-frobenius-orbit-factor-base (no algebra). Difference:
  characteristic 2, subfield curve, fold, and an explicit measured threshold on C3.
lean_obligation: as lead 1 (L1, L2) with e = 3.
next_steps:
  - "1. (shared with lead 1, step 2) fold audit at d = 3."
  - "2. (Sonnet, 1-2 days) native char-2 S_4 symmetrised solver, counted mults per trial."
  - "3. (executor-mechanical, 1 h) k = 7..13 cost table; director compares with the threshold 1.6e3 before any end-to-end run."
  - "4. (main session) stop here unless C3 is under 3e3; then a large-prime variant is the next lead."
```

---

## Ranking

1. LEAD-KOB-fold-twisted-line-d4: the only mechanism here that changes a measured number in the lab's best algebraic route (D4) by a derived factor k at equal per-relation cost, with an exact toy test and a named matched-rho obligation; it also moves the red-team crossover from m* ~ 17.6 to about 15.5. It is a weak class (j in F_16 \ F_4), not Koblitz.
2. LEAD-KOB-flat-degree-crossover-bar: cheapest test (minutes of native closure), low prior of flatness, and it converts "flat degree" into a priced bar (n* = 205/280/345 for D = 4/5/6, deficit about 190 at n = 163 for D = 4) so the main session knows what the deep-chain data would be worth.
3. LEAD-KOB-fold-twisted-line-d3: prior about 10 %, one unknown (C3 against 1.6e3 mults per solve), likely a clean negative; run only after lead 1's fold audit exists.

## Considered and not proposed (with the reason)

- Claw or known-log table on the Koblitz panel (the Lane B lead 1 transplanted): already measured, 2.195 [1.897, 2.546] pooled, 2.51 at f2m41 (`ecbench_pair_claw_20261003`); floor (c + 1/c)/sqrt(n) against rho's sqrt(pi/(4n)), generic. It is a better "baseline" than the orbit IC at n >= 37 (18x at n41) but is not index calculus and cannot go below 2.26x (floor); no new lead. [pcc:unverified]
- Koblitz E/F_2 over composite n = d m with a Gaudry line (theta-scaled or not): derived group order r <= about Q^{phi(d)} (Q^2 for d = 3, 4), so rho costs about Q and IC needs at least Q/2 relations at positive cost; consistent with the A0 trap (`subgroup_trap_20261107`, A0 PASS) and `lean-round6-20261108.json`. Koblitz curves proper are excluded from lead 1 by this, not by the line.
- Linearised or additive base-defining polynomials making S_3 F_2-linear in parts: for m = 2 the structure is already in lane A F1 (Phi*_C = P o psi_C, affine in A for fixed C, Artin-Schreier trace condition affine in C). Using it table-free costs 2^k C-values per target, total about r c / n against r^{2/3}/n for the pair table (derived): worse by r^{1/3}. For m >= 4 the F_Q-line is the F_2-linear-in-parts structure that already works (D4); a V with dim(V.V) small on a prime-degree field is the refuted quasi-subfield / polynomial family (C7).
- SAT/XOR hybrids: no exponent change (poly factor per solve); the Frobenius symmetry-breaking of GGMP is the nearest published item and LEAD-ICB-frobenius-search-symmetry the nearest id; a priced comparison against LEAD-ICE-sat-xor-pdp-oracle would have to beat a per-solve cost that is already under 1 % of the IC total at m = 15 only on the relation side, so no lead.
- Gain-graph (union-find with gains in <+-lambda>) or character decomposition of the folded relation matrix: the unknown vector is the s = 1 isotypic component, so the folded matrix is the whole content; weight-2 relations are birthday collisions (no better than the claw). LA is absent from the phase tables anyway (relations 69-77 % at n37, n41).
- Frobenius-twist patterns (R = P1 + tau^j P2 + ...), Z[tau] multiplier closure, halving or Verschiebung classes (x(tau-bar P) = x + sqrt(b)/x), tau-adic Gray-code giants: each extra class element costs about one inversion (an addition), so the gain per add is zero; the free ones (tau^i, -1) are already the fold. This is D2/D3 and the cost-per-trial >= 1 add principle.
- tau-adic subset-sum / representation technique (HGJ, Schroeppel-Shamir) on cyclic digit strings: no quotient map on a prime-order group, so no filtering; structured giant sets (tilings) cost a ln r coupon factor against random lists.
- Large primes on the folded D4: the fold already postpones the LA-bound regime by a factor k (side effect of lead 1); a large-prime variant matters only past m about 20 (red team exponents 1.07 vs 2.03) and is D9 ENG.
- D5 (e = 5) and higher: derived per-relation cost grows roughly with quotient dimension to the power 2-3 (64 at d = 4); my rough estimate puts the D5 crossover at m about 22, worse than D4 at m about 15; revisit only if a Hilbert-series count of the symmetrised S_6 system is cheap.
- F_2 closure degree as a cost proxy for F_Q-line systems: PLAN.md cells C5a/C5b (scaled subfield, predicted LFD 6 and 7) already test the mismatch; the Coordinator should preregister that a growing F_2 LFD there says nothing about the D4 solver's constant Macaulay degree. No separate lead.
- Pricing or accounting changes (emitting XOR and lookup counters, rho strong variants): accounting, not improvement (AGENTS.md success criterion).
