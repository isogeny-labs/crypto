# Round 13 proposals: genus 2–4 Jacobian DLP over F_p (Idea Generator, 2026-11-22)

Status: proposals only. Nothing here is a result. Every number quoted from the repository is
`[pcc:unverified]` (single producer, shared 4-vCPU container, no pcc claim id), and is cited by path.
Numbers computed here from on-disk tables are marked "computed here". No work is assigned. The
Coordinator decides what runs.

## Sources checked

- Read in full: `research/hecdlp_campaign_20261121/FRONTIER.md` (R1–R12) and `CAMPAIGN.md`.
- Lab studies: `research/smith_transfer_cost_20261115/RESULTS.md`, `research/quartic_diem_alg_20261121/RESULTS.md`,
  `research/quartic-diem-sparse_20261115/RESULTS.md`, `research/low-gonality-genus4_20261121/RESULTS.md`,
  `research/plane_quartic_program_20261018/CLOSING.md`, `research/quartic_n4v2_20261020/README.md`,
  `research/smith_even_census_20261115/RESULTS.md`, `research/g2plus_leads_20261116/README.md`,
  `research/g2plus_leads_20261116/L06_odd_ell_isogeny_census/DERIVATION.md` (§4–§7),
  `research/genus_ladder_exponent_g3_20261122/tables.md` (T1, T2),
  `research/genus_ladder_g3_highp_20261121/run_{65537,131101,262147}.jsonl` (`rels_by_lp`, `core_cols`, first rows only),
  `src/cryptanalysis/quartic_ladder.rs` (`jacobian_order`, l. 896–930).
- Session logs: `docs/session-logs/2026-11-20.md` (pinned goal, items 1–4), `docs/session-logs/2026-11-21.md`
  (the five sibling leads, rounds 1 and 2), `docs/ACTIVE.md` (items at l. 20–23).
- Literature (fetched): Smith, *Isogenies and the DLP in Jacobians of genus 3 hyperelliptic curves*,
  arXiv:0806.2995 (Eurocrypt 2008; J. Cryptology 22, 2009): 18.57 % under Hypotheses 1–4, Remark 3 and §9
  on non-tractable kernels; Tian, *Translating the DLP on Jacobians of genus 3 hyperelliptic curves with
  (ℓ,ℓ,ℓ)-isogenies*, arXiv:2007.03172 (J. Cryptology 2021); Ohashi–Onuki–Kudo–Yoshizumi–Nuida,
  arXiv:2401.10500 (theta-null (2,2,2)-isogeny formula (3.8), N_van ∈ {0,1,6,9} classification, Prop. 2.13).
- Literature (from memory, not fetched in this session, so labelled as such): Gaudry–Thomé–Thériault–Diem,
  Math. Comp. 76 (2007), Õ(q^{2−2/g}); Diem, ANTS VII (2006) and Math. Comp. 80 (2011), Õ(q^{2−2/(d−2)}) for plane
  degree d; Diem–Thomé, J. Cryptology 21 (2008), Õ(q) for quartics; Enge–Gaudry (Acta Arith. 2002),
  subexponential only for g/log q → ∞; Petit–Kosters–Messeng (PKC 2016); Lachaud–Ritzenthaler–Zykin (Serre's
  obstruction via χ18); Howe–Leprévost–Poonen (Forum Math. 2000), gluing genus 2 and genus 1.
- **Not available:** the `research-kb` MCP server is not attached to this subagent, so no `kb_status` or
  `search_knowledge` was run. The sibling `crypto-autoresearcher` checkout does not exist at
  `/home/user/crypto-autoresearcher`, so the lead atlas `knowledge/leads/ecdlp/` was not read. This subagent
  has no shell: `tools/lessons.py brief --scope idea-generator` could not be run; a grep of `docs/lessons/`
  finds no entry scoped `idea-generator`. Novelty statements below are therefore limited to the papers and
  paths named, and say "not found here", never "not tried".

## Corrections to the brief's figures

- Smith's route does not reach density 1/2. Smith's figure is about 18.57 % of genus-3 hyperelliptic curves,
  under four hypotheses, two of which carry a factor 1/2 each: a rational trigonal map exists, and the
  isogeny is rational given that map. The lab measured 0.171 [0.111, 0.255] of admitted curves, or 0.121 after
  removing the stop-rule bias (`smith_transfer_cost_20261115/RESULTS.md`, T5). For prime-order Jacobians it is 0
  for pair-type kernels: 0 of 168 in `smith_even_census_20261115` and 0 of 1,110 irreducible septics in
  `smith_near_prime_20261114`.
- Diem's plane-curve bound Õ(q^{2−2/(d−2)}) and GTTD's Õ(q^{2−2/g}) are as stated in the brief.

## Pre-check on disk that removed one candidate before writing

Hypothesis considered: the genus-3 LA's p^{1.5} comes from the large-prime graph drifting toward its
percolation threshold, where path lengths grow polynomially (about n^{1/3}). Disk says no. The ratio of
two-large-prime relations to p at the stopping point is flat: 23308/65537 = 0.356, 46238/131101 = 0.353,
92275/262147 = 0.352 (computed here, `rels_by_lp[2]` of the first row in each `run_*.jsonl` of
`genus_ladder_g3_highp_20261121`). The graph sits at a fixed supercritical mean degree, so it does not drift.
The same flat ratio is the mechanism behind R13-C below.

## Ranked list

Score = (expected exponent change) × (probability it holds) / (cost of the discriminating test in container-days).

| rank | id | one line | Δ exponent | P | cost (days) | score |
|--:|---|---|--:|--:|--:|--:|
| 1 | R13-A | Best quartic arm (Diem 2LP + algebraic decomposition + SGE) on random quartics to q = 2^18: measured exponent ≤ 1.15 | 0.20 | 0.50 | 0.6 | 0.17 |
| 2 | R13-B | Smith route (H → quartic, then R13-A's arm) at p = 2^13…2^18 against the R9/R10 H-arm, end to end in wall time | 0.30 | 0.55 | 1.25 | 0.13 |
| 3 | R13-C | The genus-3 LA's p^{1.5} is p^{4/3}·(ln p)^c with c ≈ 2: its local exponent keeps falling above 2^20 | 0.04 | 0.60 | 0.3 | 0.08 |
| 4 | R13-D | Affine-type (2,2,2) kernels of prime-order H: N_van of the codomain decided from theta-nulls (Thomae + duplication) | 0.40 if the route completes | 0.075 for the route (0.6 for the test's own claim) | 1.0 | 0.03 |

How they depend on each other: R13-A is the upstream gate for R13-B. If the quartic arm stays near
q^{1.3}, the Smith route gains only about 0.1 in exponent over the H-arm's 1.41. R13-D can only matter for
prime-order curves, which Smith's route never reaches. R13-C changes no algorithm; it tests whether the
FRONTIER statement "the LA scales as p^{3/2}, the same as rho" holds asymptotically or is a finite-range
effect.

---

```yaml
id: R13-A
title: Exponent of the lab's best plane-quartic arm on random quartics, q = 2^13 … 2^18
obstruction: >
  Non-hyperelliptic genus 3, density 1 − O(1/q) of genus-3 curves. The best quartic arm is linear-algebra
  bound at q = 8209: 9·la_mul is 58 % of FPM_mid, and its exponent 1.313 [0.943, 1.606] over 2053–8209 does not
  separate from rho (quartic_diem_alg_20261121/RESULTS.md). The sparse generic tool reaches LA ∝ q^1.23 [1.21, 1.24]
  because its row weight after large-prime elimination grows as q^0.29
  (quartic-diem-sparse_20261115/RESULTS.md, `LAops_wied_c2`, `wfb_main_c2`). On the GHS-weak family, N4 v2's DL
  pipeline measured 0.988 [0.986, 0.989] at k = 17–22 (quartic_n4v2_20261020/README.md). Nobody here has
  measured the generic-family arm at q ≥ 2^14.
claim: >
  On random smooth plane quartics over F_q with #Pic^0 = h·r, h ≤ 2, the arm `Q_diem_2lp+alg+sge(8,3)`
  (`genus_ladder run --quartic --quartic-alg --qc 3.0`, R8's Wiedemann) has an exponent in q of at most 1.15
  over q ∈ {8209, 16411, 32771, 65537, 131101, 262147}. The arm is FPM at κ = 3 (the in-situ κ measured in R8).
  The 95 % upper bound is below 1.22. The local la_mul exponent over the top octave is at most 1.20.
density: Non-hyperelliptic genus-3 curves, about 1 − 1/q of genus-3 curves. Restricted to near-prime orders (h ≤ 2) as instance selection only.
mechanism: >
  Diem–Thomé: lines through two factor-base points leave g − 1 = 2 residual points, because the canonical
  series of a quartic is special and not composite. With a factor base of c·√q and two large primes this
  gives Õ(q) (Diem 2006; Diem–Thomé 2008, from memory). The LA is (√q)^2 × row weight. If the weight grows
  polylogarithmically, as at genus 3 (see R13-C's flat E2/p), the exponent tends to 1. The sparse tool's
  q^0.29 weight came without SGE and was priced at w ≥ 0.1 GAE per mod-ℓ operation (a Pic^0 addition is about
  3,523 multiplications, plane_quartic_program_20261018/CLOSING.md W1). That pricing makes the LA almost all of
  S, so the tool's 1.23 is the LA's exponent, not the arm's. Under the lab's FPM with κ ≈ 3, the relation
  phase and the Pic steps dominate, and both are Õ(q).
prediction: >
  Exponent of FPM(κ=3) ∈ [1.00, 1.15], point estimate. Wall-time exponent the same within 0.05.
  Mean merged-core weight grows at most q^0.20. The ratio to the expected quartic negation rho at
  2^18 is at most 0.004. Expected rho = 0.886·√r steps × the measured per-step cost at q ≤ 8209, using the
  cheaper of the two per-step prices given L10's finding that the lab's quartic rho is 1.73× overpriced
  (g2plus_leads_20261116/README.md). Local la_mul exponents fall from octave to octave.
discriminating_test: >
  Existing instrument, no new arithmetic: `genus_ladder run --primes 8209,16411,32771,65537,131101,262147
  --genera 2 --curves 3 --seeds 2 --arms R --quartic --quartic-alg --max-h 2 --qc 3.0` (or the frozen
  equivalent), with measured `Q_rho_neg` only at q ≤ 16411 and the expected-rho reference above that.
  Orders come from `quartic_ladder::jacobian_order` (N1 + BSGS, about 4q Pic^0 steps; at 2^18 about 1e6 steps,
  tens of seconds per curve; about 4 % of curves have h ≤ 2, so about 75 order computations per top prime).
  Protocol frozen before data, with the default-path byte identity checked as in quartic_diem_alg.
falsification: >
  Point exponent ≥ 1.25 with the 95 % lower bound above 1.15, or top-octave la_mul local exponent ≥ 1.25.
  Either reproduces the sparse tool's power-law weight growth inside the best arm and kills the "Õ(q)
  measured" reading for the generic family at these sizes.
cost_band: >
  Arm (a), the IC arm: about 3.3e7 FPM at 8209 (quartic_diem_alg); scaled as q, about 1e9 FPM at 2^18, about 10 s
  per run, 36 runs. Arm (b), the reference, measured quartic negation rho: 25 to 50 × rho on H per p
  (smith_transfer_cost_20261115/RESULTS.md); measured only at 8209 and 16411 (6 runs each, minutes to an hour),
  expected rho above. Instance search: about 1 h at 2^18, less below. Total about 0.6 container-day. §5a band:
  price unpriced native work at 0, 0.1 and 1 GAE for both arms. For IC that is hash lookups of residual points
  (uncounted in quartic_diem_alg, deviation 5) and mod-r operations beyond la_mul. For rho it is canonicalisation,
  distinguished-point test and hash. Rank only where the order holds across the band.
controls: >
  Default arms byte-identical to the bound-r and quartic_diem_alg rows at 8209 (positive identity control).
  A/A seed pairs per instance. Every recovered t certified. `Q_diem_2lp` without alg/SGE as an internal
  reference arm. The qds `light` arm at 2^14–2^16 as an optional cross-tool check of the weight exponent.
risks: >
  r ≈ q^3 ≈ 2^54 at 2^18 needs u128 mod-r products (R8 kernel has them). The 63-bit guard caps q below 2^21.
  BSGS must leave exactly one surviving candidate. Three curves per prime gave under-covering intervals in
  leads 4 and 5 (session log 2026-11-21), so use six if the order search allows. The expected-rho reference is
  a model; state it as the second reading. `plane_quartic_pic::complete` panics are outside the Diem path
  (session log 2026-11-21).
closest_baseline: >
  Diem–Thomé Õ(q), measured by N4 v2 at 0.988 on the GHS-weak family only. quartic_diem_alg (1.313, 2053–8209).
  quartic-diem-sparse lead 1 (LA q^1.23). This proposal is the large-q scaling of the best generic arm. It
  overlaps lead 1's open thread (lightest rows plus block Wiedemann at 2^17–2^19), but tests the unmodified
  best arm, not a selection variant.
not_a_measured_negative: Not a core-shaping, merge-depth, base-constant or light-row variant (R3, R4, C1, C2, R11, R12). It is a scaling measurement of an unchanged arm on a family not yet measured at these sizes.
lean_obligation: None. The claim is empirical and each run is certified by k·G = Q.
```

```yaml
id: R13-B
title: Smith route end to end at p = 2^13 … 2^18 against the current genus-3 H-arm, matched wall time
obstruction: >
  Hyperelliptic genus 3: every core-shaping variant traded relation phase against LA one for one, and the arm
  sits at p^1.414 [1.394, 1.433] wall (R10). Clifford's theorem rules out a Diem-type relation with g − 1 free
  points on a hyperelliptic curve (see "Known impossible" below), so the only exponent route is a transfer.
  Smith's transfer was measured only to p = 8209, where the instance search (orders via N_2) was the blocker
  (smith_transfer_cost_20261115/RESULTS.md, Limits). The quartic order phase that dominated lead 5
  (isogeny-walk-quartic_20261121) is unnecessary here: J_C is F_p-isogenous to J_H, and the per-instance check
  [r]Ψ(G) = 0, Ψ(G) ≠ 0 already pins the subgroup.
claim: >
  For genus-3 hyperelliptic curves that are Smith-positive with h ≤ 8, the C route has a wall-time exponent of at
  most 1.15 over p ∈ {8209, 32771, 131101, 262147}, with a 95 % upper bound of 1.25. The C route is Smith analysis +
  Ψ + Diem 2LP + algebraic decomposition + SGE(8,3) on C, with every step in wall time. On the same instances
  the H-arm (2LP + prefilter + SGE(8,3), R8 Wiedemann) keeps its R10 exponent of about 1.41. The ratio C/H falls
  at least 2× from 8209 to 262147 and is at most 0.35 at 262147.
density: >
  About 0.186 of genus-3 hyperelliptic curves (Smith, heuristic). Measured 0.12–0.17 of admitted curves here.
  Never prime order: cofactor 2, 4, 6 or 8 (smith_transfer_cost T5, smith_even_census).
mechanism: >
  Smith's (2,2,2)-isogeny maps the DLP to a plane-quartic Jacobian in time polynomial in log p (arXiv:0806.2995;
  measured flat at about 1.4e5 multiplications, smith_transfer_cost). Diem's index calculus there is Õ(p), against
  GTTD's Õ(p^{4/3}) on H. The earlier small-p measurement already showed the exponent split: C route
  0.895 [0.810, 0.985] against IC on H 1.270 (FPM_mid, 257–8209). Since then both arms got cheaper (SGE on H;
  alg+SGE on C at 0.54 of the old quartic arm), and the H-arm's wall exponent above 2^16 is 1.41.
prediction: >
  C-route wall exponent ∈ [0.95, 1.15]. H-arm exponent ∈ [1.36, 1.46] on these instances. C/H in [0.15, 0.8] at 8209
  (band, not a gate), at most 0.35 at 262147, falling monotonically in point estimate from 32771 on. C route /
  negation rho on H at most 0.07 at 262147 (R9: H-arm/rho 0.187 at 262147, times 0.35). The transfer, including the
  previously uncounted Smith analysis and Trigonal::new, stays under 5 % of the C route at p ≥ 32771.
discriminating_test: >
  Extend `src/bin/smith_transfer_cost.rs` (implementation gap, small): (i) instance search runs the Smith test
  first and the C4 order method (`OrderMethod::Bsgs1`) only on Smith-positive curves; (ii) the H arm uses SGE(8,3);
  (iii) the C arm calls `quartic_ladder::run_diem_opts` with `Decomp::Algebraic` and SGE(8,3); (iv) wall clock
  around every arm, including Smith analysis. Grid: 3 curves × 2 seeds at the four primes. Negation rho on H on
  the same instances (about 119 s per run at 262147, R9). Protocol frozen first.
falsification: >
  C-route exponent ≥ 1.30 with the 95 % lower bound above 1.15. Or C/H at 262147 ≥ C/H at 32771. Or any
  transferred logarithm ≠ the planted one (correctness failure, which stops the line).
cost_band: >
  H-arm: about 22 s per run at 262147 (R9), 24 runs total. C route: unmeasured above 8209; estimated 5–20 s per run
  at 262147 from about 2.3e7 FPM at 8209 scaled as p, 24 runs. Reference, negation rho on H: 14–119 s per run
  at 2^16–2^18 (R9), 24 runs. Instance search with the Smith test first: about 210 Smith tests (cheap) and about
  38 BSGS orders per top prime, about 1 h. Plumbing about 0.5 day. Total about 1.25 container-days. §5a band at 0,
  0.1 and 1 GAE for unpriced native work in all three arms: Smith analysis factoring, hash lookups of residual
  points, and rho's distinguished-point and hash work.
controls: >
  Default `smith_transfer_cost` path reproduces the 2026-11-15 rows at 257–8209 (identity). Every log certified on
  H. [r]Ψ(G) = 0 and Ψ(T) = [t]Ψ(G) checked per instance. A/A seeds. A non-Smith-positive negative control
  must be rejected by the search with no silent fallback.
risks: >
  The Pic^0 constant (3,523 to about 9,000 multiplications per addition) enters through the walk. Wall time favours
  whichever arm was engineered (R8 sped up the LA shared by both). Smith-positive instances may be rarer at large p
  if a hypothesis fails. Cofactor-2 curves are not the cryptographic prime-order case; the claim must say so.
  The extended search must not change the instances at 8209, which is needed for joining with the old data.
closest_baseline: >
  Smith 2008/2009 (theory: Õ(q) after transfer, about 18.57 %). smith_transfer_cost_20261115 (0.50 of IC on H at 8209,
  FPM_mid). isogeny-walk-quartic_20261121 (order-phase bound at p ≤ 1021). This proposal measures the fully
  charged route 32× higher in p, with the order phase removed by the isogeny, against the lab's current H-arm.
not_a_measured_negative: A transfer to another curve, not a reshaping of the H core. R11 and R12's trade does not apply.
lean_obligation: >
  Lemma `log_transfer_of_hom`: for a group homomorphism ψ : A → B and G of prime order r with gcd(|ker ψ|, r) = 1,
  if Q ∈ ⟨G⟩ and ψ(Q) = t·ψ(G), then Q = t·G. Small and elementary. It is the only mathematical step the
  program relies on beyond per-run certificates. Not checked against crypto-lean here (research-kb unavailable).
```

```yaml
id: R13-C
title: The genus-3 LA's apparent p^{3/2} is p^{4/3}·polylog; its local exponent keeps falling above 2^20
obstruction: >
  FRONTIER R10 reads la_mul ∝ p^{1.496 [1.492, 1.501]}, "the same as rho", and calls this the structural
  reason the arm cannot pull away. If true, only a new LA changes the exponent. If the 1.5 is a finite-range
  log effect, the arm's asymptotic exponent is 4/3 and the planning conclusion changes.
claim: >
  For the genus-3 arm 2LP + prefilter + SGE(8,3) at nb = 2p^{2/3}, la_mul = p^{4/3}·(ln p)^c with c ≈ 2. The local
  la_mul exponent over p ∈ [1048583, ≈2.0e6] is at most 1.475. A pure power law would give at least 1.49.
density: All genus-3 hyperelliptic curves with near-prime order (the existing ladder population).
mechanism: >
  The two-large-prime graph is at a fixed supercritical mean degree: E2/p = 0.356, 0.353, 0.352 at 65537, 131101,
  262147 (computed here). In that regime typical path lengths, and so merged-row weights, grow like log p, not as a
  power. core_cols grows as p^{2/3} times a log factor (local 0.68: 3123, 5003, 8014). The local la_mul exponents
  computed here from genus_ladder_exponent_g3_20261122/tables.md T2 already fall: 1.516, 1.509, 1.487, 1.471 for the
  octaves starting at 65537, 131101, 262147 and 524309. Fitting 4/3 + c/ln p gives c = 2.09, 2.13, 1.97, 1.86. The
  relation phase's local exponents are flat at 1.31, 1.28, 1.33, 1.31.
prediction: >
  Local la_mul exponent between 1048583 and the largest prime the 63-bit guard admits (≈2.0e6) in [1.44, 1.475].
  The model gives 1.474 at c = 2.0 and 1.465 at c = 1.86. E2/p stays in [0.34, 0.37]. Mean merged-core weight
  grows ≤ 10 % per octave. The arm's local wall exponent over the same interval is at most 1.42.
discriminating_test: >
  IC-only runs (counts need no rho, and counts do not need load 1): the R9/R10 binary, 6 curves × 2 seeds at
  p ≈ 1.5e6 and ≈ 2.0e6, plus re-reading R10's 1048583 rows. About 370 s per run at 2e6, under 1 h on 4 threads,
  plus the C4 order search. Analysis: local exponents with a curve-then-seed bootstrap, and E2/p and weight per
  octave from `rels_by_lp` and the SGE fields.
falsification: >
  Local la_mul exponent ≥ 1.49 with the 95 % interval excluding 1.475. Or the sequence of local exponents stops
  falling (new value ≥ 1.481). Or E2/p drifts by more than 10 % (the mechanism fails even if the number fits).
cost_band: >
  IC arm only: 24 runs, about 2.5 CPU-h. Reference: none needed for an LA-count claim. Rho is not compared here. If a
  rho ratio is later reported, rho at 2e6 is about 30–60 min per run (R10 scaling) and must be priced in the §5a
  band like every other arm. Total about 0.3 container-day.
controls: Byte-identical counts on re-run of two R10 instances at 1048583 (A/A). Every log certified. The same arm parameters as R10.
risks: >
  Four existing local exponents mix 6-curve (R9) and 3-curve (R10) primes. The new octave is shorter than 2×
  (the guard caps p near 2^21), so precision is about ±0.02 unless 6 curves are used. A decline can also come
  from SGE saturation rather than log weights, so the weight and E2/p readings must accompany it. This changes
  no algorithm and is not an improvement under the success criterion (relabelling is not an improvement). It
  decides where the next exponent effort should go.
closest_baseline: >
  GTTD's Õ(q^{2−2/g}) analysis (from memory). FRONTIER R2 (la_mul p^1.52, core_cols p^0.69, weight p^0.15) and R10.
  The large-prime cycle "phase transition" literature in factoring (Leyland–Lenstra–Dodson–Muffett–Wagstaff
  MPQS with three large primes, from memory) concerns cycle counts, not row weight.
not_a_measured_negative: A measurement of the unchanged arm at one more octave, plus a mechanism reading. Not a variant.
lean_obligation: None. The random-graph statement is interpretation only, and the program would not rely on it as a theorem.
```

```yaml
id: R13-D
title: Codomain type of the two rational affine-type (2,2,2) kernels of prime-order genus-3 hyperelliptic Jacobians
obstruction: >
  Smith's pair-type route is closed for prime order (0/168, 0/1,110). Every odd-order Jacobian in the
  odd-degree model has Frobenius of cycle type (7,1) on the Weierstrass points and exactly two F_p-rational
  Lagrangians of J[2] of affine type: the extended cyclic Hamming codes from x^3+x+1 and x^3+x^2+1 (L06
  DERIVATION.md §4; census 1583/1583). Whether their codomain is a plane-quartic Jacobian is open. Smith §9
  only "expects" non-hyperelliptic codomains for general kernels, and Remark 3 gives no codomain type. This is
  `docs/ACTIVE.md` item "(2,2,2) non-pair kernels" (PR #4693); this record proposes a specific cheap discriminating method for it.
claim: >
  For a generic genus-3 hyperelliptic curve, the (2,2,2)-isogeny with an affine-type kernel G has a codomain with
  N_van = 0 (a plane-quartic Jacobian over the algebraic closure), on at least 19 of 20 random curves and for
  all 30 affine kernels.
density: >
  If the route completed: every odd-order Jacobian of a degree-7 model, including all prime-order ones. Even-degree
  models of type (3,5) are excluded, because AGL(3,2) has no element of order 15. The F_p-rational fraction
  would further be reduced by Serre's obstruction (Smith Prop. 5: the isogeny is defined over F_q or F_{q^2}).
mechanism: >
  Derived here, to be checked by gate G2: with G as the standard Göpel group, the duplication formula (Ohashi et al.
  (3.8)) gives θ'[a;b]^2 = 2^{−3} Σ_β (−1)^{a·β} θ[0;β] θ[0;β+b]. Per fixed G, the 36 even characteristics split
  28 + 8 between pair-type and affine-type positions of the hyperelliptic vanishing characteristic
  (36·105/135 = 28, 36·30/135 = 8). The 8-element orbit of G's stabiliser is {[0;β]}. So for affine G the single
  vanishing even theta of H is one of the 8 inputs of the formula. Formally this forces no codomain theta to
  vanish: for b = b0 and a·b0 = 0, three paired terms survive; a·b0 = 1 is an odd characteristic. The hyperelliptic
  condition becomes a linear relation among the codomain's θ'[a;0]^2, a codimension-1 condition that need not
  coincide with a theta-null divisor. The prior for N_van = 0 is about 0.6. No general hyperelliptic-to-hyperelliptic
  (2,2,2) construction in genus 3 is known to this author; Mestre's are special RM families (from memory).
prediction: >
  Gates: G1, exactly 1 of 36 even θ^4 vanishes (Thomae) on every curve. G2, for affine G the vanishing
  characteristic is in the duplication input set, and for pair G it is not. G3, the pair-type control gives
  N_van = 0, as in Smith's 10^6-curve experiment where no codomain was hyperelliptic. G4, the Riemann quartic relations
  hold to 1e−10 relative on the chosen sign pattern. Main readout: affine-type N_van = 0 on ≥ 19/20 curves.
discriminating_test: >
  Rust, complex f64, no CAS: 20 random septics with well-separated complex roots. Thomae's formula for θ[η_S]^4
  via Mumford's η-map. Enumerate the 135 Lagrangians with the existing `src/cryptanalysis/smith_kernel_coverage.rs`
  table (105 pair / 30 other). For each G, the symplectic relabelling that makes G standard. The 2^7 relative sign
  patterns of the eight θ[0;β], filtered by G4. Then the 36 codomain θ'^2 and their N_van. Sign-robust conclusion:
  if no surviving sign pattern gives a zero, N_van = 0 whatever the true signs. About one day to implement; the run
  takes seconds.
falsification: >
  For affine G, some codomain θ'^2 vanishes (|·| < 1e−9 relative) on ≥ 19/20 curves for every sign pattern passing
  G4. That means N_van ≥ 1, the codomain is hyperelliptic or a product, and the ℓ = 2 route for prime order closes:
  record it as a problem in `problems/`. If G2 fails, the derivation above is wrong; revise the record before
  reading the main result.
cost_band: >
  This test: about 1 container-day, no rho or IC arm involved, so no §5a pricing applies. If N_van = 0, the next
  steps are not one-day items and are priced separately. (i) Serre twist over F_p for G_h and G_{h*}, about 1/2 each
  (Smith Hypothesis 3), via the χ18-square criterion (Lachaud–Ritzenthaler–Zykin, from memory). (ii) An explicit
  divisor map, theta-coordinate evaluation à la Lubicz–Robert or Tian, multi-week. (iii) Then the full route against
  the H-arm and negation rho with every arm priced at 0, 0.1 and 1 GAE.
controls: Pair-type kernels as positive control (G3). Thomae vanishing count (G1). Repeat at two root configurations (real, complex) to expose numerical artefacts. 30 affine kernels as internal replication (they are S_8-conjugate).
risks: >
  The η-map and the symplectic relabelling are easy to get wrong; G1–G3 exist to catch that. Thomae gives fourth
  powers, so the sign-pattern search can leave several consistent patterns (handled by the sign-robust rule).
  Floating-point near-zeros need two precisions. A positive answer is only geometric. The F_p-rational twist and
  the explicit map remain, so the probability that the whole route completes is about 0.6 × 0.5 × 0.25 ≈ 0.075.
closest_baseline: >
  Smith 2009 §9 and Remark 3 (heuristic: non-hyperelliptic expected). Ohashi et al. 2024 (computes all 135
  neighbours by theta duplication and classifies by N_van; does not classify kernels by pair/affine type).
  Tian 2021 (odd ℓ). L06 DERIVATION.md §6 (lists the codomain question as open).
not_a_measured_negative: No lab study has computed a codomain for a non-pair kernel. It is not an IC variant.
lean_obligation: >
  Finite lemma, decidable over F_2^6: for the standard Göpel group G, an even characteristic m makes (G, m) affine
  type iff m ∈ {[0;β] : β ∈ F_2^3}. Equivalently, the S_8-orbit count 8/28 per G. Provable by `decide` over 135 × 36
  cases; avoid `native_decide` if the proof gate treats it as trusted. The duplication formula itself would be cited,
  not formalised.
```

---

## Known impossible, ruled out, or already measured (so not proposed)

- **Subexponential for fixed genus:** none known. Enge–Gaudry's subexponential algorithm needs g/log q → ∞, and
  the L(1/3) variants are for special large-genus families (from memory). Nothing here claims one.
- **Genus 2:** GTTD gives Õ(q^{2−2/g}) = Õ(q), the same exponent as rho's q^{g/2} = q. The lab's 2LP arm is 3–5×
  above rho to 2^24, consistent with that. Gluing J(H2) × E into a genus-3 quartic Jacobian (Howe–Leprévost–Poonen)
  gives Diem's Õ(q), again equal to rho. Split Jacobians are excluded for prime order: a prime #J forces an
  F_p-simple Jacobian (Tate). There is no exponent route at genus 2, only constants (`docs/ACTIVE.md` item 23 owns those).
- **Diem-type relations on hyperelliptic curves:** proved obstruction. By Clifford's theorem every special linear
  system on a hyperelliptic curve is composite with the g^1_2. So no non-composite family of divisors leaves only
  g − 1 free points, which is what gives Diem's gain on quartics. The plane-quintic model of a genus-3 hyperelliptic
  curve (a quintic with a triple point) gives q^{2−2/(5−2)} = q^{4/3}, equal to GTTD.
- **Smith pair-type route for prime order:** closed (0/168, 0/1,110). Only R13-D's affine kernels and odd ℓ remain.
- **Odd ℓ (Tian 2021):** not proposed here. It is `docs/ACTIVE.md` item 22. L06's census fires on 27.8 / 26.8 / 24.0 %
  of prime-order curves at ℓ = 3, 5, 7, with a voided gate. Kernel points live in F_{p^k} with k up to ℓ^3 − 1. The
  isogeny needs theta-coordinate machinery (multi-week), and Serre's obstruction applies. The cheapest test exceeds one day.
- **Genus-4 hyperelliptic to non-hyperelliptic transfer:** expected impossible by dimension (unverified). The
  Jacobian locus has codimension 1 in A_4, so a generic ppav isogenous to a hyperelliptic Jacobian is not a Jacobian.
  Smith's trick uses A_3 = closure of M_3. Hyperelliptic genus 4 stays at GTTD's q^{3/2}. Non-hyperelliptic genus 4 is
  at Diem's q^{2−2/(g−1)} = q^{4/3}; the trigonal-pencil lead measured relation phase 1.18–1.46 and total S at w = 0.1
  of 1.21–1.49 for p ≤ 2039 (low-gonality-genus4_20261121/RESULTS.md).
- **Extension-field Weil descent and decomposition:** out of scope (prime fields). The lab's extfield lead was
  solver-bound, with the Macaulay solve at about 4.5e5 mulmods per trial.
- **Summation-polynomial or structured factor bases over F_p** (Petit–Kosters–Messeng style): a reduced divisor
  already is a g-point decomposition. A structured factor base of the same size has the same smoothness density, so
  no gain is known.
- **Core-shaping at genus 3** (deeper merge C1, base constant C2, light rows at genus 4/5 R5, relation excess R3,
  bounded core R11, per-prime weight R12): closed in this campaign; the relation phase and the LA trade one for one.
- **Automorphism quotients:** constants only, and only on density-zero families. L02 at genus 3 gave 1.10× [0.91, 1.34];
  the cover-automorphism lead found only {1, ι} on its instances.
- **Critical large-prime-graph drift:** refuted on disk (E2/p flat at 0.352–0.356), see the pre-check above.

## Continuous-learning loop (role record)

- Step 1 could not be run: no shell in this subagent. A grep of `docs/lessons/` finds no `idea-generator` entry.
- Step 3 could not be run (`lessons.py outcome/test/add`). Candidate lesson for the Coordinator to record:
  "Before proposing an LA-exponent lever, compute local exponents per octave from the on-disk per-prime table;
  R10's T2 shows 1.516 → 1.471, which a single power-law fit hides." Evidence:
  `research/genus_ladder_exponent_g3_20261122/tables.md` T2.
