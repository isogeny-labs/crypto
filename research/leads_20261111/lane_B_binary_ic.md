# Lane B leads: binary-field index calculus engineering and presentations (2026-11-11)

Author: Idea Generator (proposals only). Nothing in this file was run. No command could be executed in this
session (no shell tool); every number below is quoted from a file on disk, with the path, or is labelled
"derived" (arithmetic on quoted numbers) or "prediction". All values are `[pcc:unverified]` producer
reports unless a path says otherwise. Implementation language for every test below: Rust (existing
`examples/d4_char2_profile`, `ecbench`, `cargo test`); no Python.

## 0. State of the lane (what the leads are measured against)

Rows this lane can move (from `docs/ic/LEADERBOARD.md`):

- Table B random binary, one target, vs matched negation rho: f2m15 13.1x, f2m18 10.6x, f2m21 18x,
  f2m24 23x, f2m27 38.2x (lines 223-227). Phase shares: relations 47-96 %, factor base 4-52 %, LA ~0 %
  (lines 289-293).
- Component section "Frobenius-semilinear root finding, char-2 D4" (lines 169-184): m = 14 IC/rho-neg
  4.27 -> 3.48 (w = 0), 3.88 -> 3.16 (0.1), 2.13 -> 1.74 (1); m = 12 about 40 (w = 0). This is the
  binary row nearest to rho; the red-team report puts m = 15 at 5.2x and the extrapolated crossover at
  m* ~ 17.6 [16.3, 19.2] (`research/gaudry_k4_char2_redteam_20261026/REPORT.md` section 1).
- Progress table (line 23): pair claw (known-log MITM, `ic.known_log_mitm2_v1`) on Koblitz f2m43 at
  r = 2^32.1 is 1.85x matched rho.

Per-trial profile of the k = 4 solver, m = 12, 300 targets, F_q multiplications after `frob_roots`
(`research/ic_d4_char2_lever_20261023/REPORT.md`): total 191,435 = charpoly 87,728 (45.8 %) + echelon
54,809 (28.6 %) + normal forms 21,913 (11.4 %) + roots 22,977 (12.0 %). LA is 0.1-0.4 % of S at
m = 12-15. Trials per relation are about 192 (per-trial success about 0.5 % = 1/(4! * 8): the double count
the treatise records as "fixed for every curve", `conclusion.tex`). So the only free quantity in this
regime is the cost per trial, and S scales with it linearly. With cost exponents 1.07 (IC) and 2.00
(rho) (red-team report section 1), a factor f on per-trial cost moves the extrapolated crossover by
log2(f)/0.93 in m (derived), e.g. f = 0.6 moves m* from 17.6 to about 17.1.

Structure observed in the source (`src/cryptanalysis/gaudry_quartic_bin.rs`):

- The reduced system is 4 Weil components of one F_{q^4} equation in the unknowns
  (S2, S3, s4, P), weights (2, 2, 1, 1), all of weighted degree 4, Hilbert series
  (1+t^2)^2 (1+t+t^2+t^3)^2, quotient dimension 64, Macaulay degree 11 (`FROBENIUS_WEIGHTS`,
  `frobenius_reducible`, line 976-1009). Only 21 of the 105 invariant monomials carry H (E3 in
  `research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md`).
- The eigen step uses chains of one unknown `mvar`, hard-coded to 3 (= P) at the call site (line 2328).
  `charpoly_muls` is the sum over dim + 1 = 65 evaluation points of a dense determinant of the chain
  matrix (lines 2147-2166). 87,728 / 65 = 1,350 per point, which matches a k x k Gauss with k = 16
  (16^3/3 = 1,365; derived). k is not recorded in any report I read; the first step of the first test
  is to print it.

Search status: `research-kb` MCP is not in this session's toolset (not called). The sibling
crypto-autoresearcher lead atlas is vault-encrypted (VLT1); not touched. Existing-lead list read:
`research/leads_20261111/EXISTING_LEADS.txt` (173 ids); sweep report read:
`research/lead_sweep_20261110/SWEEP_REPORT.md`.

Candid assessment: the structural levers in this lane (symmetry reduction, Frobenius form, pair
tables, large primes, factor-base shape, SAT/F4 oracles) are all consumed, and the k = 4 trial design
is near a conservation law (yield per trial fixed, cost superlinear in the quotient dimension if
symmetry is broken). I looked for, and could not find, a lever with a high prior. The four leads
below are cheap probes and one scope-gated measurement; two of them have priors of 10 % or lower and
are stated as such.

---

## 1. LEAD-ICB-tableB-binary-known-log-claw (rank 1 by expected measured gain; scope ruling first)

```yaml
id: LEAD-ICB-tableB-binary-known-log-claw
title: Known-log pair claw (ic.known_log_mitm2_v1) on the five Table B random-binary instances
obstruction: >
  Table B's random-binary rows (10.6-38.2x matched rho) use an unknown-log recipe (folded pair
  table, walked targets, m = 2, LA). The programme already has a known-log arm that applies to any
  group (`methods.rs` line 1185, `Applies::Any`) and scored 1.85x on Koblitz f2m43 at r = 2^32.1
  (LEADERBOARD.md line 23). Nobody has put it on the Table B binary instance set, so the board's
  binary rows are not the programme's best binary arm.
claim: >
  On icv1-f2m15-t131-ed7256f0, icv1-f2m18-t393-d2c04152, icv1-f2m21-t2591-de54d72e,
  icv1-f2m24-tm3047-2d0c0e8c and icv1-f2m27-t12543-569dca8b, with the table size chosen near
  T = sqrt(r/2), ic.known_log_mitm2_v1 recovers the logarithm (verified [d]G = Q) at counted cost
  S_IC / S_rho-neg in [1.4, 2.4] at w = 0 on every instance, against the board's 10.6-38.2.
mechanism: >
  Known-log base: no relations, no LA. Cost is T additions to build plus r/(2T) queries (negation
  fold), minimised at 2*sqrt(r/2) = 1.414 sqrt(r), i.e. 1.60x rho's 0.886 sqrt(r) (derived). The
  Koblitz measurement (1.85x) is within 16 % of that floor. Board S for f2m27 is 36.9 sqrt(r),
  26x the floor.
prediction: >
  Ratio to matched rho-neg at w = 0 in [1.4, 2.4], at w = 0.1 and 1 within +-15 % of the w = 0 value
  (table inserts and lookups are unpriced on the IC arm; rho's canonicalisations are unpriced on
  rho: both sides double). Improvement against the board row: x5 (f2m15, 13.1 -> ~2) to x19
  (f2m27, 38.2 -> ~2). The ratio cannot go below ~1.6 (generic floor); a value under 1.4 would indicate
  a counting error and must be treated as a defect.
discriminating_test: >
  Local, under 5 minutes (r <= 2^24.4, T <= 3.5e3 entries). ecbench spec (JSON) with arms
  ic.known_log_mitm2_v1 (params size, base_seed, max_pair_work, max_trials), ic.known_log_mitm2_v1
  again with a different base_seed (A/A), rho.negation (matched), on the five instances, >= 16 targets
  each; `ecbench verify --replay-all` on the session. Board values are read from
  docs/ic/leaderboard.json, not re-run.
falsification: >
  Any instance > 3.0x at w = 0, or any unverified logarithm, or A/A arms differing by more than 10 %,
  or a ratio < 1.4 that survives an audit of the counted additions (then the counter is wrong).
cost_band: >
  IC arm: counted group additions (build T, queries r/(2T)); unpriced: base generation, table inserts,
  table lookups (emit counts; priced at 0, 0.1, 1 GAE each). rho.negation: unpriced canonicalisations
  and partition hashes (counters exist: `canonicalisations_uncharged`), priced at the same 0, 0.1, 1.
  Rank only where the order holds at all three weights (AGENTS.md 5a). Both arms land at ~1.4-1.7 sqrt(r)
  vs 0.886 sqrt(r); no pairing flips are expected, rho stays ahead.
controls: A/A (two base seeds); positive control = planted target recovered on f2m15; negative control
  = the same arm with max_trials below T (must report exhausted, not a log).
risks: >
  (1) Scope: this is a BSGS-type generic arm. The programme labels known-log arms "ic" in ecbench and the
  progress table, but the treatise calls the k-sum/pair-table oracle "bounded by sqrt r; cannot beat
  rho". The Coordinator must rule whether a known-log arm may supersede the unknown-log recipe on a
  Table B row, or only enter the progress table. If not admissible, the result is a reference
  measurement, not an improvement. (2) Memory: the table is T entries; AGENTS.md equal-advice audit
  (conclusion.tex: "an equal-advice audit retired every online win") applies; report table memory.
  (3) Not novel as an algorithm; novelty is the instance set and matched reference only.
closest_baseline: Table B f2m rows (38.2x at f2m27); progress-table pair claw 1.85x at f2m43.
closest_existing_lead: LEAD-ICB-generic-table-oracle (sweep: toy Z_N stand-in, inconclusive because
  the A/A control failed at n = 13, `research/lead_sweep_20261110/LEAD-ICB-generic-table-oracle/RESULT.md`);
  LEAD-RHO-bsgs-family-memory; LEAD-ICP-generic-table-decomposition (prime). Difference: the real
  ecbench method on the real Table B binary instances against matched rho, with the band.
row_moved_if_works: Table B random-binary rows (or the progress table's best binary cell) from
  10.6-38.2x to ~1.5-2.4x; distance to the generic floor 1.6x, to rho the same as the floor.
lean_obligation: none (measurement). The floor 1.414/0.886 is a generic-group counting bound already
  in the treatise (Prop. collisionfloor); no new mathematical claim is relied on.
sources_checked:
  - src/cryptanalysis/ecbench/methods.rs lines 1181-1208 (known_log_mitm2 Applies::Any; known_log_ksum PrimeOnly)
  - docs/ic/LEADERBOARD.md lines 23, 205-227, 273-293
  - research/lead_sweep_20261110/phase2_index.json (LEAD-ICB-generic-table-oracle, line 456)
  - papers/ecdlp-index-calculus-treatise/sections/prog_binary.tex ("Leaderboard rows")
```

---

## 2. LEAD-ICB-d4bin-chain-variable-sweep (rank 2; cheapest k = 4 probe; prior about 10 %)

```yaml
id: LEAD-ICB-d4bin-chain-variable-sweep
title: Choose the eigen (chain) unknown and variable order that minimise the chain count k in the char-2 D4 solver
obstruction: >
  The characteristic-polynomial stage is the largest component of the k = 4 binary solve: 87,728 of
  191,435 F_q multiplications per trial at m = 12 (45.8 %). It costs (dim + 1) * (k^3/3) with
  dim = 64 and k the number of monomial chains of the multiplication unknown (derived: 65 * 1,350;
  k = 16 inferred, not recorded). `mvar` is hard-coded to 3 (P) at gaudry_quartic_bin.rs:2328; the
  other weight-1 unknown (s4, index 2) and other variable orders were never tried.
claim: >
  There is an admissible (mvar, variable order) pair in the Frobenius-form system for which the
  chain count k is <= 12, so charpoly_muls per trial falls to <= 0.42 of the current value
  (12^3/16^3).
mechanism: >
  Under degrevlex the chain count of mvar equals the dimension of the slice ring obtained by setting
  mvar = 0 in the leading-term ideal, which depends on the weights of the remaining three unknowns
  and on the order. P and s4 both have weight 1, so by weight symmetry the two choices are likely
  equal (k = 16 both); a different order might not be. The weight-2 unknowns are worse (larger slices).
prediction: >
  Registered before the run: P(k_min <= 12) = 0.10; P(k_min = 16, ratio 1.00 +- 0.02) = 0.80;
  P(ratio > 1.3) = 0.10 for the worse variants. If k = 8 is reached (not expected), charpoly falls to
  about 11k muls and total per trial from 191k to about 115k (0.60), so m = 14 IC/rho-neg would read
  3.48 -> ~2.1 (w = 0), 3.16 -> ~1.9 (0.1), 1.74 -> ~1.05 (w = 1) on top of the existing lever
  (derived; w > 0 columns are lower bounds because the XOR/lookup work is not emitted).
discriminating_test: >
  Step 1 (seconds): print k, chain lengths and the standard-monomial staircase for mvar in {2, 3} and
  for the 2 orders that put the other weight-1 unknown last, from BinSolve.chains on 50 targets at
  m = 8. Step 2 (only if some k < 16, minutes): add a `--mvar`/order option to
  examples/d4_char2_profile and run 300 targets at m = 12 per arm, base arm = current mvar = 3 plus an
  A/A copy; compare charpoly, echelon, NF, roots and total muls per target (paired, same targets).
  Build time on this container is ~10 min (release, fresh); disk is tight (build artefacts ~GB),
  use CARGO_INCREMENTAL=0.
falsification: >
  k = 16 for every admissible choice (ratio 1.00 +- 0.02 on all 300 targets), or any admissible choice
  with k < 16 whose total muls per trial does not fall (other stages grow: NF is 11 %, eigenvectors are
  small). In both cases the stage is closed with the staircase recorded as the obstruction.
cost_band: >
  Counted F_q multiplications, identical solutions required (assert equal decompositions, as the
  frob_roots test does). Unpriced: XOR accumulation and table lookups in the eigen step are not
  emitted by any IC arm (REPORT.md of ic_d4_char2_lever); rho carries canonicalisations_uncharged.
  Report the lever/base ratio at w = 0, 0.1, 1 with the same caveat the existing component row carries;
  do not claim a band robustness result until the counters are emitted.
controls: A/A copy of the base arm; unit test that the roots equal the generic path (as in the
  existing 7 tests); `ecbench verify --replay-all` on any session produced.
risks: >
  Weight symmetry makes the null likely (stated above). Changing mvar changes the shape-replay trace
  (`replay=1`), so replays must be re-recorded. If k = 16 is intrinsic (slice of generic quartics), the
  charpoly stage is at its evaluate-interpolate floor and the next stage to attack is echelon/NF.
closest_baseline: component row "Frobenius-semilinear root finding, char-2 D4" (m = 12: S 44.03 -> 37.62).
closest_existing_lead: LEAD-ICE-boolean-f4-f5-f6-engines and LEAD-ICE-curve-presentation-invariance
  do not touch the eigen step. E18 of research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md states
  "the chain step ... cannot move cost" from a read-only survey, without a run; this lead tests that
  statement with a counter.
row_moved_if_works: char-2 D4 component rows m = 12 and m = 14 (IC/rho-neg), and the k = 4 replication
  plan's IC cost estimate (30 h -> ~20 h wall).
lean_obligation: none (engineering). The solver already guards its own shape assumptions at run time
  ("chain structure" outcomes); no new mathematical claim.
sources_checked:
  - src/cryptanalysis/gaudry_quartic_bin.rs lines 971-1036, 2044-2176, 2318-2345
  - research/ic_d4_char2_lever_20261023/REPORT.md (profile table)
  - research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md (E3, E4, E14, E18)
```

---

## 3. LEAD-ICB-d4bin-exponent-coset (rank 4; prior about 5 %; one-minute probe)

```yaml
id: LEAD-ICB-d4bin-exponent-coset
title: A second Frobenius/toric substitution in the char-2 D4 system (all 21 carrying monomials in a coset of an index-2 sublattice)
obstruction: >
  The existing Frobenius form removed a factor 8 of quotient dimension (512 -> 64) because every
  monomial carrying H has even exponents in s2, s3, pi (E3; `frobenius_reducible`, "true on every curve
  tried, not proved"). After halving, the 21 carrying monomials sit among the 30 monomials of weighted
  degree <= 4 in (S2, S3, s4, P) (derived: 30 = 15 + 2*6 + 3). Nobody recorded whether they also lie
  in a coset of a further index-2 sublattice.
claim: >
  Over 100 instances at each of m = 8, 10, 12, the F_2-rank of the exponent-difference vectors of the
  carrying monomials in (S2, S3, s4, P) is 3 (not 4), i.e. all carrying monomials have the same parity
  of one fixed linear form, so a further squaring substitution halves the quotient dimension to 32.
mechanism: >
  Pigeonhole (derived): classes of (a, b, c, d) mod 2 over the 30 monomials have sizes 8, 3, 3, 3 (a, b
  even-pair classes), 3, 1, 1, 1, 3, 1, 1, 1, 1; an index-2 coset can contain 21 of them only if the
  form is "a even" or "b even" (23 monomials each); every other form has <= 20 (c even: 20, a + b even: 18).
  So the probe reduces to two checks: is the S2 exponent (or the S3 exponent) even on every carrying
  monomial. If so, S2 -> S2^2 is another bijection of F_q, in characteristic 2 the same rational
  solutions, and the quotient dimension (reduced points) halves.
prediction: >
  P(either form holds on all instances) = 0.05. Expected outcome: both forms fail on at least one
  carrying monomial (the monomials S2 and S3 themselves have odd exponent and are among the 21 in the
  generic count 20-21 terms per Weil equation). If one holds: dimension 32, Macaulay degree about 7-8,
  and per-trial cost falls by a factor 2-4 (derived from echelon ~ D^2 and charpoly ~ D * k^3), moving
  m = 14 IC/rho-neg 3.48 -> 0.9-1.7 at w = 0 (derived; unpriced work not emitted).
discriminating_test: >
  Local, seconds after a build: extend `SymmetrisedBin::parity_counts` (src, line 988) to also print
  the F_2-rank of exponent differences and the counts for the forms {a even, b even}, on 100 generated
  instances (`generate_instance(m, seed)`) for m = 8, 10, 12. No solving is needed.
falsification: >
  Both forms fail on at least one carrying monomial of at least one instance (then the lead is closed,
  with the exponent vectors recorded).
cost_band: >
  No cost claim before the substitution exists. If the probe passes, a second step (a `frobenius=2`
  option) is priced exactly as the existing lever: counted F_q multiplications, equal decompositions,
  A/A, replay-all; XOR/lookup work unpriced and flagged.
controls: the existing `parity_counts` output (0 odd monomials) must be reproduced first; a synthetic
  instance with one planted odd monomial must be rejected by the new check.
risks: >
  Very low prior. Even if the coset holds empirically it is a statement about the symbolic S5 over
  F_2[a, b]; the code keeps the run-time guard (as for frobenius_reducible) but the program should not
  rely on it without the Lean obligation below.
closest_baseline: component row "Frobenius-semilinear root finding" and research/gaudry_quartic_d4_char2_20261015.
closest_existing_lead: LEAD-ICE-curve-presentation-invariance, LEAD-ICP-digit-presentation-solving-degree
  (presentation changes; neither inspects the monomial support of the symmetrised H).
row_moved_if_works: char-2 D4 component rows (m = 12, 14) and the k = 4 crossover estimate.
lean_obligation: >
  If the coset holds on all instances and is to be relied on: a Lean (or kernel-checked polynomial
  identity) statement that the symmetrised Semaev polynomial H over F_2[a, b] has no monomial with odd
  exponent in S2 (resp. S3) in the reduced variables; the same status as the existing, unproved,
  `frobenius_reducible` guard. Until then the claim stays empirical and guarded at run time.
sources_checked:
  - src/cryptanalysis/gaudry_quartic_bin.rs lines 971-1036 (parity_counts, frobenius_reducible, system_frobenius)
  - research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md E3 ("21 of 105 monomials carry H; ... even exponents in s2, s3, pi")
```

---

## 4. LEAD-ICB-d4bin-constant-quadratic-elimination (rank 3; prior about 10-15 %; needs implementation)

```yaml
id: LEAD-ICB-d4bin-constant-quadratic-elimination
title: Eliminate the two weight-2 unknowns by linear algebra before the Macaulay step, using the constant quadratic part of the four components
obstruction: >
  Echelon plus normal forms (54,809 + 21,913 = 76,722 of 191,435 muls, 40 %) are a generic weighted
  Macaulay computation at degree 11 on a 387 x 448 matrix (E4). The four components have weighted
  degree 4 with weights (S2, S3) = 2, so their quadratic part in (S2, S3), the coefficients of S2^2,
  S2*S3, S3^2, is a 4 x 3 matrix of numbers (degree 0 in s4, P), not polynomials. That special shape
  is not used by name anywhere in the solver; it is only implicit in the grevlex leading terms.
claim: >
  Row-reducing the 4 x 3 numeric quadratic part gives one combination g with no quadratic term and three
  with leading terms S2^2, S2*S3, S3^2. Solving g for S2 (or S3) as a rational function of the
  remaining unknowns, and reducing the other three, yields a bivariate (s4, P) problem whose counted
  cost (echelon + NF + the 65 k x k determinants, or a bivariate resultant) is <= 0.7 of the current
  Macaulay-11 total per trial at m = 12, with identical rational solutions.
mechanism: >
  Elimination of two of four unknowns by cheap linear algebra replaces a 448-column sparse elimination.
  Rough count (derived, not measured): the resultant route needs about 97 evaluation points times a
  20 x 20 Sylvester determinant, about 262k muls, which is WORSE than the current 165k for
  echelon + charpoly + NF; so the claim is only plausible with a cheaper bivariate step (for instance
  keeping the module structure over F_q[s4, P] with basis {1, S2 or S3} and reusing the chain trick).
prediction: >
  P(<= 0.7) = 0.10; P(0.7-1.3) = 0.35; P(> 1.3) = 0.55, with the resultant route as a counted
  prototype. This is stated so that a null is a registered outcome, not a surprise.
discriminating_test: >
  Step 1 (local, minutes after a build): from `system_frobenius` on 300 targets at m = 12, report
  (a) which monomials are present, (b) the rank of the 4 x 3 quadratic-part matrix (must be 3 on all
  targets), (c) the staircase of the Macaulay-11 echelon: the leading terms in (S2, S3) and the number
  of standard monomials per (S2, S3) pattern. Step 2 (a day, outside the 20-minute local budget):
  counted prototype of the elimination plus a bivariate solve in Rust inside gaudry_quartic_bin.rs,
  gated by a new option, compared paired against the base on 300 targets (equal decompositions).
falsification: >
  Step 1: rank < 3 on any target (then the premise is false for those trials and the lever is
  conditional), or the staircase already shows the {1, S3} x 32-dimension module structure at no
  extra cost (then the Macaulay solve already exploits it and the lead reduces to LEAD-ICB-d4bin-chain-variable-sweep).
  Step 2: total counted muls per trial >= 0.9 of base.
cost_band: >
  Counted F_q muls (new elimination steps are counted by the same mul counter); unpriced work as in
  lead 2 (flagged). Pair with the A/A and equal-decomposition assertion.
controls: A/A; 7 existing unit tests plus a test that decompositions equal the base solver on 12
  solves per m (existing pattern).
risks: >
  The tailored elimination may cost more than the generic route (my own count says so). The premise
  holds only for the Frobenius form with weights (2, 2, 1, 1); the odd-char D4 (E24/E31 in the
  leaderboard) does not have it.
closest_baseline: char-2 D4 component row; research/ic_d4_char2_lever_20261023.
closest_existing_lead: LEAD-ICE-gag-contraction-planner, LEAD-ICE-syzygy-cascade-signatures
  (generic structure of Semaev chains; not this specific constant-quadratic-part shape),
  LEAD-ICB-kic-golden-base-exact-decomp (base choice, not elimination).
row_moved_if_works: char-2 D4 component rows (m = 12, 14).
lean_obligation: none for the engineering claim (solutions are checked by evaluation in the code);
  the "quadratic part is constant" premise is a property of the symbolic H (same status as
  frobenius_reducible; Lean statement only if relied on beyond a run-time guard).
sources_checked:
  - src/cryptanalysis/gaudry_quartic_bin.rs lines 971-1036, 1938-2176
  - research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md E4 (dimension 64, 387 x 448), E14
```

---

## 5. Ranking and what I would not do

1. LEAD-ICB-tableB-binary-known-log-claw: highest chance of a large measured number, minutes, but its
   admissibility as an improvement of a Table B row needs a Coordinator/owner ruling first (generic
   BSGS-type arm; floor 1.6x).
2. LEAD-ICB-d4bin-chain-variable-sweep: one-minute step 1; arithmetic from the profile supports the
   mechanism; prior about 10 %.
3. LEAD-ICB-d4bin-constant-quadratic-elimination: needs a day of implementation; step 1 is a probe.
4. LEAD-ICB-d4bin-exponent-coset: one-minute probe; prior 5 %, but decisive and the payoff is 2-4x.

Considered and dropped with reasons (so no one re-derives them):

- A second coset or union base for k = 4 (patterns (3,1), (2,2) need quotient dimensions 256-384): yield
  grows with the dimension, cost grows faster; consistent with the treatise's "mean yield ... a double
  count" (derived).
- Hybrid guess-and-determine at k = 4: same total work (guessing one summand costs q); the closest
  existing lead is LEAD-ICE-hybrid-guess-determine-l7.
- Krylov/Berlekamp-Massey instead of 65 determinants: needs normal-form rows with fewer than about 41 of
  64 nonzeros to win (derived: 128 matvecs of 16 * d vs 88k); NF rows of a 64-dimensional quotient are
  expected dense. Not proposed.
- Larger or union factor bases to cut trials: LA is 0.4 % at m = 15, so the optimum already sits on the
  relations side; any larger base leaves the F_q-line construction (Diem/Gaudry large-prime variants
  matter only past m ~ 20, derived from exponents 1.07 vs 2.03).
- Pricing/accounting changes (emitting the XOR and lookup counters, rho strong with batched inversion):
  needed for the replication but they are accounting, not improvements.

## 6. Sources checked

Repository (read in full or by range): papers/ecdlp-index-calculus-treatise/sections/{conclusion,
prog_openproblems, distance, prog_binary}.tex, binary_ext.tex (headings only), prog_method.tex lines
206-212; docs/ic/LEADERBOARD.md (lines 1-30, 169-227, 269-293, 452-499, 1119-1178);
research/lead_sweep_20261110/SWEEP_REPORT.md, phase2_index.json lines 454-475;
research/leads_20261111/EXISTING_LEADS.txt; research/gaudry_k4_char2_redteam_20261026/REPORT.md;
research/ic_d4_char2_lever_20261023/REPORT.md; research/gaudry_quartic_d4_char2_20261015/EXPERIMENTS.md;
src/cryptanalysis/gaudry_quartic_bin.rs (ranges above); src/cryptanalysis/ecbench/methods.rs
(lines 1170-1220); AGENTS.md sections "Preserve scope and report evidence", "Name the obstruction"
(start), "Implementation language: no Python", "The rule: boundary, table, ratio" (start), "5a";
docs/session-logs/2026-11-11.md (first 575 lines; no entry bears on lane B).

Literature (web search, titles/abstract level only; no full text read):

- Faugere, Perret, Petit, Renault, "Improving the Complexity of Index Calculus Algorithms in Elliptic
  Curves over Binary Fields", Eurocrypt 2012 (https://christophe.petit.web.ulb.be/files/Eurocrypt2012.pdf):
  O(2^{wt}) with t ~ n/2 for any binary field; closest published method to the descent systems.
- Galbraith, Gebregiyorgis, "Summation polynomial algorithms for elliptic curves in characteristic two",
  Indocrypt 2014 (https://eprint.iacr.org/2014/806): practical issues in characteristic two, prime n.
- Trimoska, Ionica, Dequen, "A SAT-based approach for index calculus on binary elliptic curves",
  Africacrypt 2020 (https://eprint.iacr.org/2019/313): WDSat, symmetry breaking by m!; closest published
  solver comparison (repository status: solver comparison on toy instances only, prog_binary.tex).
- Faugere, Gaudry, Huot, Renault, "Using symmetries in the index calculus for elliptic curves discrete
  logarithm", J. Cryptology 27 (2014) (https://eprint.iacr.org/2012/199): small rational subgroup
  symmetries, quasi-homogeneous structure, FGLM; the k = 4 binary solver is the 2-torsion / weighted
  analogue and none of the four leads is a published method.
- Not fetched: Huang-Kosters-Yeo, Joux-Vitse, Diem (no lead here depends on them); the novelty claims
  above are therefore limited to "not found in the repository's 173 leads, the treatise, or the four
  search results above".
