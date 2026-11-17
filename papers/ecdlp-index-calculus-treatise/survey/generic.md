# Generic family digest: the rho side and the IC-vs-rho boundary

Survey date 2026-11-06. Read-only. Sources: `crypto-autoresearcher:knowledge/leads/ecdlp/generic/*.yaml`
(26 LEAD-RHO-* records, last reviewed 2026-10-24 to 2026-11-01), `crypto` origin/main `research/`,
`docs/bounds/FRONTIER.md`, `benchmarks/README.md`, `pcc/`, and the local `cryptanalysis` HEAD
(2026-10-22, older than its origin). Treatise sections checked by grepping all
`papers/ecdlp-index-calculus-treatise/sections/*.tex`.

**Standing note (sabotage rule).** No `pcc/claims/PCC-*` record covers a generic-family result: the
claims on main are IC leaderboard rows and curve25519 autolab rounds. So every number below is an
**unverified assertion** at the pcc level. Where a sealed ecbench bound id (`ECBND1h...`), a replay
audit, or a `VALIDATION.json` exists, it is named. Very few rows in this family have a
VALIDATION.json (e27-walk is one). The treatise must not cite these numbers as validated.

Units: `S` = counted group-addition equivalents (GAE) / sqrt(r). "x floor" = S / sqrt(pi/2A),
A = number of usable automorphisms. Ratios are B/A paired unless stated. Intervals are 95 %.

---

## 1. The generic sqrt(r) lower bound (Shoup/Nechaev)
- Lead: `LEAD-RHO-generic-sqrt-lower-bound`, closed_obstructed (theorem). KR-RHO-13bf67.
- Content: any generic algorithm needs Omega(sqrt r) group operations; every generic lead moves only the
  constant; automorphisms divide by sqrt|Aut| (KR-RHO-0528e7).
- Measured: all generic arms fit alpha <= 0.53 on the frontier (e.g. bsgs.negation 0.529 [0.486, 0.595],
  prime toy, `ECBND1h9d01a5c2ae56`); none below the floor beyond CI noise.
- Treatise: **PARTIAL**. `prog_theory` "Lower bounds" and `summer` mention the generic-group model in
  two lines. Missing: the statement and proof sketch, the scope (generic only; representation-reading
  methods escape it), and the measured exponent table.

## 2. IC-vs-rho boundary arguments (the collision floor)
- `crypto:research/curve25519_rho_parity_bound_20261104/README.md` (2026-11-04), derived bound plus
  Monte Carlo; self-labelled [pcc:unverified].
  - Claim: in the generic group model, an IC solve with F factor-base elements of unknown log needs
    F+1 independent collisions. The k-th collision arrives at E[n_k] = sqrt(r) Gamma(k+1/2)/Gamma(k).
    So IC/rho >= Gamma(F+3/2)/(Gamma(F+1) Gamma(3/2)): 1.500 (F=1), 1.875 (F=2), 2.188 (F=3).
    Monte Carlo (4,000 trials, r = 131,203): 1.517, 1.889, 2.196.
  - It explains the curve25519 autolab "wwalk" (F=2) incumbent converging at 1.82-1.94x rho-neg. The
    earlier 1.43-1.55x readings were single draws of the walk table (autolab correction I1).
  - Parity is possible only at F=0, which is rho or BSGS relabelled. The algebraic track (F4 trace
    replay, t=4 isogeny base, T13) is 1.83e4x behind rho at w=0.1 and scales as r^1.03. The best
    screened algebraic family (m=2 full-coverage base, 256 points) is 265x at w=0.1 and 1.6e3x at w=1.
  - Limits: GGM with uniform walk points; linear algebra uncharged; single target only.
- Related boundary statements:
  - `research/prime_ic_program_20261029` decisions: the generic track is "bounded by sqrt r; cannot
    beat rho" (cited in `distance.tex`).
  - `obstructions.tex` "Oracle-augmented index calculus stays generic".
  - `research/ecbench_e2e_frontier_20261031/README.md` (2026-10-31, uncited): no admitted result on the
    generic prime, random-binary or medium Koblitz ladders beats matched cold one-target rho. Live
    registry: 74 method ids (31 IC, 32 rho/transfer, 7 BSGS, 2 kangaroo, 2 claw) and 11 solver engines.
    A current-binary audit reproduces only 72/144 historical F4/F5/F6 rows. Narrow exception: at
    n=29, one column, the Frobenius-layout pipeline is 0.366 [0.241, 0.523] of rho-frob, but it is
    classified as Frobenius-folded BSGS and rests on rho's fixed set-up at log2 r = 15.4. The GHS-weak
    plane-quartic family is the one real below-rho frontier.
  - `research/rho_parity_20260915/` (uncited): full-cost toy binary IC parity iterations. Five
    engineering iterations; "rho parity is not established: the frozen all-cases gates are not met".
- Treatise: **PARTIAL**. The generic-track label and the oracle obstruction are covered. **ABSENT:**
  the Gamma-function collision floor, which is the cleanest "why IC with a factor base cannot reach
  rho parity generically" argument in the corpus, the e2e frontier census, and rho_parity_20260915.

## 3. The matched rho reference: calibration and finite-size bias
- `research/ecbench_calibration_20261002/README.md` (uncited). Seven generic methods reach their
  analytic constants at r ~ 2^25.4 (prime) and 2^39 (Koblitz). 2,384/2,384 executions verified; 4
  audit receipts, each with 12/12 replays identical. Round 1: two preregistered predictions failed.
  - rho.plain f2m41: 0.792 of theory (band failure).
  - kangaroo.vow fp26: 0.601. It is deterministic per target, so the effective sample was 8 targets
    with mean distance 0.119 r against 0.25 r.
  - Fresh 64-target follow-up: 0.989 and 0.986. Rho's run-to-run coefficient of variation is about 0.5.
- `LEAD-RHO-matched-rho-calibration`, open_next_step. Prime toy rho.negation is 1.453 [1.184, 1.772]
  x floor with alpha 0.398 [0.315, 0.475] (`ECBND1h67e22de83dca`). **No prime medium-tier rho bound
  exists** on FRONTIER.md (still true at 130 bounds). Next step: seal rho.negation, rho.plain and
  bsgs.negation at log2 r 32-48 and fit R(N) = 1 + c/N^beta (H-ECDLP-ff38dc).
- `research/rho_negation_audit_20261020` measured rho.negation at 28-40 bits: 1.075-1.146 x floor.
  So the toy excess shrinks at medium size (descriptive, L0).
- curve25519 notes: measured rho-neg sits at 0.98-1.08 sqrt r against the model's 0.886 sqrt r.
- Treatise: **PARTIAL**. `prog_method` covers the ecbench design and the strong-rho bound. Missing:
  the calibration study, the CV ~0.5 sampling lesson, the toy-tier alpha bias, the missing prime
  medium tier, and its consequence that every prime-field "x rho" gap uses a toy denominator.

## 4. The sealed bounds frontier and challenge protocol
- `docs/bounds/FRONTIER.md`: procedure 2, frontier `ECFR1hac90861c5e8a`, 130 bounds.
  - Koblitz medium tier: ops leader `rho.signed_frobenius_strong` 1.089 [0.927, 1.261], alpha 0.451
    [0.399, 0.503], 144 runs at log2 r 32.1-47.2.
  - Prime toy: no ops leader. Tie set: bsgs.negation 1.100 [0.978, 1.225] (memory 0.502 sqrt r),
    rho.negation 1.406-1.466, rho.plain 1.920 [1.633, 2.221].
  - Koblitz toy: no leader. Ops x floor is 5-10 because of set-up.
  - Oracle (Cheon) domains are listed separately.
- `research/ecbench_frontier_v2_20261023/README.md` (uncited) is a **correction**. The v1 page named
  leaders on point estimates, and its alpha intervals had degenerate ends (kangaroo [0.179, 2.500]
  became [0.401, 0.724]). Under v2, rho.plain is no longer dominated by rho.negation: the order flips
  at 0.34-0.55 GAE per unpriced canonicalisation. The 46 v1 records stay byte-identical.
- `research/ecbench_bounds_challenges_20261006/README.md` (uncited). These were the first paired
  challenge verdicts.
  - Prime toy: bsgs.negation vs rho.negation is a **trade**, 0.8250 [0.725, 0.945] of the ops at
    15.79 [13.38, 19.06]x the table.
  - Koblitz toy: rho.signed_frobenius vs strong **advances at the constant level**, 0.2991
    [0.235, 0.402] of the ops (toy only).
- Treatise: **PARTIAL**. The definition of comparability and the strong-rho incumbent are in
  `prog_method`. Missing: the v1→v2 correction, the frontier tables per domain, and the challenge
  verdicts.

## 5. Pricing band applied to rho arms (§5a on the reference itself)
- `rho_negation_audit` band.json: the order of negation over plain holds at w=0 and 0.1. At w=1 the
  negation S is 1.89-2.03, above plain, so the ranking is unresolved on all four curves. A real
  canonicalisation (one F_p negation plus a compare) is far below 1 GAE, but it was not priced.
- `research/ext_endo_rho_20261023/analysis/*.band.md` (uncited):
  - rho.gls (A=4, F_{p^2}): S at w=0 is 0.554-0.921, against rho-neg's 0.904-1.165. At w=1 the order
    reverses: 2.70-4.20 against 1.74-2.31.
  - rho.frobenius_orbit (A=6, E/F_p on F_{p^3}): w=0 0.587-0.692 against rho-neg 0.783-1.404, and at
    w=1 2.11-2.52.
  - So the strongest reference for extension-field curves depends on the band weight.
- FRONTIER footnote: rho.signed_frobenius dominates rho.plain at charged cost, but the order flips at
  about 0.123 GAE per uncharged operation.
- Treatise: **PARTIAL**. The band is defined in `prog_method` §"The pricing band". **ABSENT:** that
  the band reorders the *rho references among themselves*. This matters for which reference counts as
  "matched".

## 6. Negation map, fruitless cycles and the negation audit
- `LEAD-RHO-negation-map-fruitless-cycles`, closed_known (KR-RHO-fb88e6). Prime toy rho.negation is
  1.453x floor against plain 1.920x (ratio 0.757 vs ideal 0.707). The fruitless 2-cycle rate is 487
  per 1e6 steps, against 1/(2r) = 488.
- `research/rho_negation_audit_20261020/README.md` (uncited; dated 2026-10-20, updated 10-30):
  - Inventory: two reference walks lacked the map, `volcano-rho` and crypto-autoresearcher
    `index_calculus/rho.py`. Both are now on by default; this is a **correction** of earlier rho
    references.
  - ecbench pooled plain/negation is 1.371 [1.257, 1.491] (256 pairs, 4 curves, 28-40 bits; sqrt 2
    lies inside the interval).
  - Wall time (L0) is only 1.140.
  - volcano-rho off/on is about 1.40, at +9-16 % ns per iteration.
- `research/generic_dlp_correctness_20261017` (cited): the `pollard_collab` 2-cycle test never fired.
  Dead trails went 50→1, 106→0 and 197→15.
- `rho_walk_leads` lead 4: a per-epoch representative rule gives 0.83x [0.79, 0.87], worse than no
  negation map.
- ecc-challenges pool, `LEAD-RHO-pool-class-quotient`: the v2 worker has no class quotient. Negation
  walk B3 takes 0.700 [0.673, 0.728] of v2's steps with 0/1,800 trapped trails and ~6 % per-step
  overhead. It sits in open PR #56 as a post-hoc amendment.
- Treatise: **PARTIAL**. The negation map is used throughout as the reference. Missing: the audit
  that found non-negation references, the wall-vs-count gap, and fruitless-cycle handling.

## 7. Automorphism-class walks (Koblitz, j=0, j=1728, GLS, extension Frobenius)
- `LEAD-RHO-automorphism-class-walks`, closed_known.
  - Koblitz rho.signed_frobenius_strong: 1.089x floor.
  - Strong-rho steps/ideal 0.99/0.92/0.94 at n=41/53/61.
  - j=0 crater rho.cubic/rho.negation 0.573 [0.562, 0.585] against theory 0.577 (volcano census).
- `research/prime_rho_automorphism_20261015` (uncited) carries an **erratum (2026-10-24)**: the class
  set-up (16-58 GAE per solve) was left out of the totals. Corrected ratios:
  - rho.cubic/neg 0.647 [0.543, 0.770] (was 0.611);
  - rho.quartic/neg 0.819 [0.722, 0.929] (was 0.791), which now excludes 0.707.
- `research/isogeny-volcano-rho` crater runs:
  - j=0 needs 1.89 ± 0.09x fewer operations (sqrt 3 predicted) and j=1728 1.32 ± 0.06x (sqrt 2
    predicted);
  - in wall time that is 1.33x faster on j=0 but 2.3x slower on j=1728 (no tuned walk there).
- `research/ext_endo_rho_20261023`: see §5.
- cryptanalysis `experiments/prime-j0-rho-coordinate-canon-20261002`: an oriented sixfold walk is the
  default for its j=0 GLV solver; wall ratios exploratory. The atlas cites `docs/sixclass-20261024`:
  1.257x and 1.826x wall at toy size, secp256k1 throughput unmeasured. That file is not on the local
  HEAD.
- `benchmarks/20261030-e27-walk-karatsuba` (has VALIDATION.json): the matched <±psi> walk is
  1.453-1.463x cheaper than negation on E25's GLS curves in F_p multiplications.
- Treatise: **PARTIAL**. The background formula sqrt(pi r/(2A)) and the Koblitz signed-Frobenius
  reference are covered. Missing: the prime j=0/1728 results and their erratum, the GLS/F_{p^3}
  references, and the "ceiling is sqrt|Aut|" closure.

## 8. Equivalence classes beyond Aut(E)
- `LEAD-RHO-non-automorphism-equivalence`, open_unexamined (H-EWALK-29cb90 proposed, never tested).
  Quotient by H < (Z/N)^* with |H| > 6 is cheap only if canonicalisation costs o(log h).
- Treatise: **ABSENT** (deserves a paragraph as an open problem).

## 9. Strong-rho engineering and assurance (the Koblitz reference)
- `research/rho_assurance_20261016` (uncited):
  - Lean 4, 34 theorems, no sorry: least-rotation canonicaliser, Montgomery batch inversion, walk
    algebra over Z_r. The curve-group encoding bridge is open.
  - The TLC model check finds no violation.
  - Q3: 1,196 instructions per step, about 57 field multiplications, against a floor of ~6-7; half was
    canonicalisation.
- Round by round (all outputs identical; native gains "not established"):
  - `strong_rho_canon_20261008`: 1.40x faster per capped walk at m=83, 1.46-1.61x at n=41-61.
  - assurance Q4: 1,196 → 938 instructions per step.
  - `rho_canon2_20261016`: 938 → 781.
  - `rho_step3_20261016`: 781 → 718 (native 1.008x at n53, 1.025x at n61).
  - `benchmarks/20261022-strong-rho-batchinv`: 707.2 → 679.7 at n53.
- `research/ecbench_rho_optimized_20261014`: 0.802155 of complete-solve Callgrind Ir. GAE is exactly
  1.0000; 720/720 replays.
- `research/r12_rho_ns_per_step_20261026` (uncited): the unbatched tuned rho is 5.48/9.34/9.56x slower
  per step than strong rho at n=41/53/61. This explains the audit's 599 against 240 ns/step
  discrepancy (L0, descriptive).
- Rule-v8 effect (`distance.tex`): batching rho's canonicalisation raised the IC gap 1.30-1.58x.
- Treatise: **PARTIAL**. `prog_method` has one paragraph (1,196 → 679.7) and `distance.tex` notes v8.
  Missing: the assurance results (Lean/TLC), the per-round table, and R12.

## 10. Native cost of a rho step: GAE vs native, canonicalisation pricing, harness overhead
- `LEAD-RHO-native-step-cost`, open_next_step: price the Koblitz canonicaliser in field
  multiplications (EXP-CERTBIN-b4d29c). `fp64_branchfree_20261016`: 98.9 → 68.2 ns/step. No qualified
  Tier-0 host, so native wall timings are not admitted.
- `benchmarks/20261018-k-conversion` (cited in `prog_frontier`): K = 4.48 rho steps per Macaulay
  column touch.
- `benchmarks/20261016-hwcost-rho-model`: an analytic time model, counted ops x cycles per op.
- `benchmarks/20261022-ecbench-harness-overhead`: the registry was parsed per lookup. The timed window
  falls to 0.03-0.07x at 20 bits and 0.64-0.81x at 32 bits for every rho/BSGS/kangaroo arm.
  `ecbench_child_walk_ir_20261101`, `ecbench_walk_trace_ir_20261101` and `ecbench_walk_round3_20261102`
  (uncited) are further Ir rounds; GAE unchanged.
- Treatise: **PARTIAL** (K conversion only). Missing: why GAE understates native rho cost, and how
  harness overhead contaminated small-size wall ratios.

## 11. Walk-design leads on prime curves
- `research/rho_walk_leads_20261014` (uncited; Modal CPU, N ~ 2^32-2^40):
  - Lead 1, best of P±R: 1.129 [1.081, 1.184] fewer steps under DPs (predicted sqrt(k^2/(2k-1)) =
    1.155; positive control 1.151). Best of four: 1.463 [1.401, 1.527]. Net per-step cost 1.215x at
    1024 walks per inversion (net ~0.95), and 1.03-1.12x predicted gain at 8-64.
  - Lead 2, lazy DP test: 0.998 [0.995, 1.001]. The table index on lazy x gives 0.733.
  - Lead 3: a shared index sequence gives 0.073x (no collisions possible; proof). The pair table
    R_i + R_j gives 1.027 [0.982, 1.073].
  - Lead 4: see §6.
- `LEAD-RHO-best-of-candidates-indegree`, open_next_step: build into the gpu/nist kernel at W=16-32.
- Treatise: **PARTIAL**. Best-of P±R is in `prog_frontier` ("The reference moved too"). Missing:
  leads 2-4, the in-degree prediction, and the batch-size dependence.

## 12. Iteration-function mixing (Teske r-adding walks)
- `LEAD-RHO-r-adding-walk-mixing`, closed_known. Plain rho over 3,200 solves on 26-bit volcanoes
  averages 659,263 ± 12,422 iterations against sqrt(pi n/2) = 654,547 (1.007x).
- isoclass-n45: the planted 4-adding walk gives 1.291, against Teske's 1.137.
- HEC rho: 8-bit branch coefficients made the reference slow at large N (§25).
- Treatise: **PARTIAL** (one Teske mention in `prog_theory`).

## 13. Cheaper step primitives (tag tracing, point halving)
- `LEAD-RHO-cheaper-step-primitives`, open_unexamined (KR-RHO-38be82, KR-RHO-6654ba). Never measured.
- Treatise: **ABSENT** (one paragraph in open problems).

## 14. BSGS family, the pair claw, and memory
- `LEAD-RHO-bsgs-family-memory`, closed_known (cost_model). bsgs.negation is the prime-toy ops co-leader
  at 1.100x floor, but needs 0.50 sqrt(r) entries, which is 2^127 at 256 bits.
- `research/ecbench_pair_claw_20261003` (uncited): the cryptanalysis#175 pair claw is randomised BSGS
  on signed-Frobenius classes. It costs 2.20x [1.90, 2.55] strong rho at its best shape and 32.6x
  [25.3, 41.0] at the PR's n=83 shape. 960 runs, 12/12 replays. Cost (c+1/c)/sqrt n >= 2.26x the rho
  floor.
- Engineering:
  - lit2026 W11 (cited in `prog_prime`): paired BSGS 16.75 → 3.71 M-equivalents per visited point.
  - generic_dlp_memory_parallelism (cited): BSGS 1.30-1.98x wall.
  - `benchmarks/20261016-sea-point-count`: BSGS point counting 1.99-3.24x.
- Treatise: **PARTIAL** (engineering lines only). **ABSENT:** the claw result and the
  memory-vs-ops trade (the challenge verdict "trade").

## 15. Store-all schedules: grumpy giants, compiled tiered walk
- `LEAD-RHO-grumpy-store-all-schedules`, open_next_step. Bernstein-Lange two grumpy giants and a baby:
  0.71875 against 0.6753 targets solved at 1.5 sqrt l. EXP-GRUMPY-cd7b82 Stage 1 O-SUPPORT-M12 at
  l = 101, 257; EXP-GRUMPY-eb8675 (2^10-2^20 certified ladder) has not been run.
- `research/tiered_rho_compiled_walk_20260921` (uncited): compiled schedule 19,027,631/65,537 against
  19,114,086/65,537 unguided, 0.45 % below at 2^16. Theoretical; no wall claim.
- Treatise: **ABSENT**.

## 16. Kangaroo / lambda
- `LEAD-RHO-kangaroo-interval`, closed_dominated. kangaroo.vow is 2.203 [1.803, 2.527]x floor on the
  prime toy (vs rho.negation 1.453), and 15.558 [13.742, 17.597] on Koblitz medium (vs 1.188).
  Calibration (§3) found the deterministic-target sampling artefact (0.601 → 0.986). Revisit only for
  short-interval or low-entropy secrets.
- Treatise: **ABSENT** (named once in the band paragraph).

## 17. Distinguished points, multi-target batch rho (Kuhn-Struik), Bernstein-Lange precomputation
- `LEAD-RHO-dp-multitarget-precomputation`, closed_known; these are the incumbent controls.
- Equal-advice audit, cited: BL rho is 53-317x faster online (n=41 with exact advice: 317 [137, 739]);
  at 1/1024 of the bytes still 7.7-32x.
- `20261016-n83-precomputed-rho-control`, cited: IC/precomputed-rho online median 3.74x (time-matched)
  to 214.73x. The 28 Oct reproduction is cited in `distance.tex`.
- `research/ic_rho_koblitz_20260923` (uncited): batch rho at k=32 costs 0.210/0.239/0.190 of k=1 per
  target. Each interval covers Kuhn-Struik's 0.199. Pricing:
  - a canonical step is 2.74-2.92 units, Bailey's step 1.38-1.41, a stored pair 4.5-6.1;
  - the thread's "1.17x" figure becomes 6.51x against batch rho, 6.38x priced, and 43.7x for one
    target alone.
  - This is a **correction** (accounting class).
- `research/ic_preprocessing_rho_equal_advice_20261011` (uncited): a derived calculation with the
  generic line S·T^2 ≳ N (Corrigan-Gibbs-Kogan), not measured.
- `problems/nist-best2-dp-bias-and-gpu-validation.md`: the P-256 BEST2 DP deficit is not attributed;
  0.947 is derived.
- Treatise: **PARTIAL**. The equal-advice retirement of online wins is covered (abstract, intro,
  `prog_koblitz`, `prog_frontier`, `audit`). Missing: the Kuhn-Struik batch reference and the
  1.17x → 6.51x correction, the T^2 S tradeoff law, and a stated theory of the BL l^{1/3} table.

## 18. Advice-table variants (reselected, grown, basin-instrumented, isogeny-transported)
- `LEAD-RHO-advice-table-variants`, open_next_step.
  - A T/2 reselected table falls short of static by -0.064 [-0.080, -0.049] at 2^24 (contradicted,
    EV-ECDLP-60e266).
  - S1_v2 met at 3 of 4 cells at T_sel 0.65-0.75T (toy, preliminary).
  - The basin instrument failed its comparability gate.
  - Isogeny transport of a table is exact, costs <= 1.3 % of a solve, and is never faster than solving
    at the source.
- Treatise: **ABSENT**.

## 19. Cheon's DLP with auxiliary inputs
Six sub-lines. It is the largest absent block in this family.
- **Acquisition obstruction**, `LEAD-RHO-cheon-aux-acquisition`, closed_obstructed (theorem). From
  (P, [x]P) a generic algorithm spans only [a+bx]P. Computing [x^2]P is square-DH, which is
  Omega(sqrt r). DEC-20260802-204 is the acquisition gate. So Cheon never touches ordinary ECDLP
  generically.
- **Deployed divisor census**, `LEAD-RHO-cheon-deployed-applicability`, open_blocked.
  RUN-AUXIN-fd1edc gives E(r): P-256/secp256k1 ~0.254 at d ~ 2^128 (r-1), P-224 0.394 (r+1),
  P-521 bound 0.452. These need degree-d supply. The amendment was refused (DEC-20260923-a4c3d5).
- **Powers-of-tau census**, `research/srs_auxiliary_census_20260927` (uncited). It is a reproduction
  of ethresear.ch #6692, not a discovery. Generic floors at constant 1 (not measurements):

  | setup | bits lost vs rho |
  |:--|--:|
  | EIP-4844 mainnet (q = 4,095, d = 3,648) | 5.74 |
  | Zcash Sapling | 10.82 |
  | Filecoin | 13.82 |
  | Aztec Ignition | 13.12 |
  | Perpetual Powers of Tau (d = 536,259,126) | 14.32 |

  Its largest-admissible d adds about +0.49 bits over the published d for three setups.
  The practical p-1 algorithm ran at about 20x its floor at 24-48 bits
  (`torsion_auxiliary_inputs_20260918`, cited in `prog_prime`).
- **P-224-shape scaling**, `research/cheon_p224_shape_20261022/REPORT.md` (uncited). **Retraction
  (2026-10-23):** the H3 gate failed (5 of 84 rho runs had S outside [0.3, 4]), so every Cheon/rho
  ratio and crossing is withdrawn. Cheon-only scaling stands:
  - fitted exponents 0.447 (n-1) and 0.413 (n+1);
  - formula costs 2^97.50 and 2^88.36 against rho 2^111.83;
  - extrapolations (fit A) 2^101.71 and 2^95.59 at 224 bits (model, not measured).
- **Cheon as ecbench methods**, `research/cheon_ecbench_20261022/REPORT.md` (uncited; benchmarks row
  `20261022-cheon-ecbench`).
  - Methods: cheon.pminus1_comb, cheon.pminus1_naive and cheon.pplus1_comb, with the static-DH oracle
    charged as a separate resource. Bounds live in "oracle domains" on FRONTIER.md.
  - Data: 320 runs at 32-56 bits; 320/320 replays.
  - Pooled ratio to rho.negation over 36-56 bits at w=0: 0.552 [0.455, 0.651] (n-1 comb), 0.279
    [0.216, 0.419] (n+1 comb), 7.66 (naive). The order holds across the 12-cell w x q band.
  - **No exponent separation:** Cheon 0.450 [0.406, 0.473] and 0.428 [0.389, 0.460] overlap rho's
    0.496 [0.431, 0.611] and 0.513 [0.386, 0.636].
- **Medium-tier re-run**, `research/cheon_ecbench_medium_20261106/PROTOCOL.md` (2026-11-06,
  protocol only, no results). The original sessions mixed tiers, so procedure 2 cannot rank them.
- **Toy reproduction**, `LEAD-RHO-cheon-toy-reproduction`, open_next_step. EXP-AUXIN-fef6ac on
  F_10007 gave O-NO-SPLIT. EXP-AUXIN-25a29a (56-bit r, d = 65536) is approved but has no runs.
- Treatise: **PARTIAL, effectively ABSENT**. There is one sentence in `prog_prime` ("beats rho from 24
  bits up, but only for the DLP with auxiliary inputs").

## 20. Low-Hamming-weight DLP
- `LEAD-RHO-lhw-polymem-collision-walks`, open_next_step. EXP-LHW-b49bc6 Stage 1 expired its claim;
  this is not a negative result.
- `LEAD-RHO-lhw-modular-filter-price`, open_next_step. The residue-guess multiplier G_M is unproved.
- `LEAD-RHO-lhw-preprocessing-frontier`, open_unexamined.
- Literature: Esser-May 2^0.65n with polynomial memory.
- Treatise: **ABSENT** (out of IC scope; at most an open-problems paragraph).

## 21. ECC2K-130 GPU rho campaign
- Throughput: 15.884 B updates/s per RTX PRO 6000 (sigma-table, paired 1.0239 against 15.514; A/A
  0.099 %; 300/300 replay). ALU busy 91 %; speed of light 27.4 B/s; goal 26 B/s. G7 RTX PRO 4500
  12.50 B/s. The FPGA VU47P estimate is 5-12 B/s, which is derived, not measured.
- About 229x the 2010 GTX 295 (`benchmarks/20261014-paper-figures`).
- Iteration function:
  - SIGMA_SIGN=2 gives 0.958-0.967 of the iterations at n=41 and a rate of 0.9965, for 0.961-0.970
    time per solve; not rolled out.
  - The v3 table walk is projected at 0.81-0.86x and is unmeasured.
  - Best-of-two sigma emulated at 0.836x, but +38 % work.
- `research/ecc2k130_rho_audit_20260925` (uncited): 2^28.41 steps per completed walk at the HW(x) <= 32
  cutoff, with duplicate-point, seed-reset and fruitless-cycle guards.
- `research/ecc2k_cycle_escape_20260926` (uncited): cycle escape v3 correctness receipts.
- Treatise: **PARTIAL by design**. The treatise defers to `paper/ecc2k130/`. `prog_koblitz` and
  `prog_frontier` mention signed sigma 0.961-0.970 and the 6000 card-years. A short summary chapter
  section with a cross-reference would suffice.

## 22. GPU and SIMD rho on prime curves; distributed pools
- `LEAD-RHO-prime-gpu-rho`, open_blocked (no admitted GPU arm; PCIe telemetry contract missing).
  - `benchmarks/20261013-nist-gpu-rho-ladder`: P-256 1.075 → 1.632 B it/s and P-384 0.210 → 0.548 B
    it/s on one RTX PRO 6000 (W=32, by-reference inversion; descriptive). The levers were PTX carry
    chains (1.19x/1.38x) and W.
  - Seeding: 0.66 against 11.53 ms per seed, 17.4x (`20261017-gpu-nist-jac-seed`).
  - `research/nist_gpu_arithmetic_20260926` and `p256_rho_round1_20260926` (uncited) are the protocol
    and the pilot.
- ECCp-131 CPU: 1.23 M steps/s with a 32-bit Montgomery batched walk; 15.06 M with an AVX-512
  eight-lane walk (cited in `prog_prime`).
- Batched affine plus Montgomery trick, `LEAD-RHO-batched-inversion-shared-pm`, closed_known. P-256
  plain step 179 ns at 1024 walks per inversion; proved in Lean.
- ecc-challenges pool: see §6.
- Treatise: **PARTIAL**. ECCp-131 throughput is covered. GPU NIST and the pool are **ABSENT**.

## 23. Rho invariance across isogeny classes and volcano levels
- `research/isogeny-volcano-rho`, benchmarks row `20261016-isogeny-volcano-rho` (uncited). One
  40-bit instance carried by Vélu 3-isogenies to every level of a depth-3 volcano. Pooled iterations:
  L0 700,483 ± 47,849, L1 681,766, L2 637,158, floor 659,589, all 659,263 ± 12,422 against 654,547.
  There is no level effect. Generator and secret controls: |z| < 0.8, KS p > 0.6.
- `research/isogeny-class-rho` and `-n45` (uncited).
  - 46-bit class P2: 4,400 solves on 38 curves; pooled 1.0097 against 1.016; Kruskal-Wallis p 0.38;
    0 flagged curves.
  - The planted negation fold reads 0.732 and the planted 4-adding walk 1.291 (positive controls).
  - The generators family was flagged once (KW p 1.2e-4) and **did not replicate** (KW p 0.97 on
    re-run seed).
- `benchmarks/20261017-isogeny-volcano-cost`: rho floor/crater 1.022 [0.983, 1.062]; BSGS identical.
  The v2 study found rho 1.004.
- Correction already in the treatise: the volcano census "IC 22-25 % cheaper on the crater" claim
  was withdrawn because of the crater rho reference (`audit.tex`, erratum A3).
- Overlaps the structure family (`LEAD-STR-isogeny-class-generic-invariance`).
- Treatise: **ABSENT** as a positive statement. `obstructions` gives transport conservation for IC,
  not the measured rho invariance that is its generic twin.

## 24. Hyperelliptic-Jacobian rho references
- `benchmarks/20261018-hec-rho-negation`: fold/plain 0.67-0.89 at N <= 6e4 but a median 0.968 at
  N >= 1e5 (R1 <= 0.80 **falsified**).
- `20261018-hec-rho-hash-mixing`: H1 and H2 both falsified.
- `20261019-hec-rho-branch-width`: full-width coefficients give an S_walk median of 1.390 at large N
  (8-bit/full 1.418 on the largest row), so the old reference was slow.
- Treatise: **COVERED (brief)** in `prog_method` "Generic baselines". 1-2 more paragraphs would help,
  since genus-2/3 transfer rows are ranked against it.

## 25. ecbench statistics, censoring and independence
- `research/ecbench_statistics_v2_20261105` (uncited; PR #2442): a linearised t(K-1) interval and
  censoring of timeouts and exhausted runs.
  - Censoring is the big effect. ic-sub6-mitm/rho-strong goes from 6.981 to 648.2 [419.6, 1001]
    (lower bound); rho-frob/bsgs-neg goes from 3.512 to 212.0.
  - At K=2 the new interval is 13.1x wider.
  - Coverage: 0.957/0.948/0.947 at K = 2/4/8.
- `research/ecbench_independence_20261009` (uncited): a same-machine replay changed the environment
  class id. This led to the hardware-class gate (correction).
- `research/ecbench_accounting_v2_20261023`: preregistered accounting v2 (Typst only; not read in
  detail).
- Treatise: **PARTIAL**. The statistics paragraph in `prog_method` predates v2. The censoring
  correction changes how earlier "x rho" figures should be read.

## 26. Generic relabellings measured as IC arms (known-log two-sum, k-sum tables, curve25519 m=3)
- `research/ecbench_known_log_targets_20261021` (uncited): the target-blind known-log two-sum beats
  the abscissa pipeline at 16/20/24 bits, but **negation rho is cheaper at every size**.
- `research/curve25519_m3_generic_20261102` (uncited): m=3 generic decompositions are 7.9-17x rho-neg
  at k = 20-28 (train). The mitm3 oracle is worse than the incumbent everywhere.
- Already in the treatise: the prime k-sum table, the generic track and `rootfind-subtract`
  ("bounded by the square-root frontier").
- Treatise: **PARTIAL**. Curve25519-family work is entirely absent; §2's parity floor is the theory
  for it.

## 27. Generic DLP correctness (Pohlig-Hellman, collab)
- Pohlig-Hellman total_steps 284,364 → 10,682 (26.62x); factorisation bounded.
- Treatise: **COVERED** (`prog_method` "Generic baselines").

## 28. Frobenius-orbit IC against the best rho (exponent gap r^{1/6})
- `research/ecbench_frobenius_orbit_vs_rho_20261014`: slope 0.165 [0.117, 0.213] per ln r against the
  model's 1/6; 2,816/2,816 verified.
- Treatise: **COVERED** (`prog_frontier`, `distance`).

---

## Corrections and retractions to flag in the treatise
1. Cheon P-224 shape: all Cheon/rho ratios withdrawn (H3 gate failure, 2026-10-23).
2. prime_rho_automorphism erratum: class set-up was omitted. Cubic 0.611 → 0.647; quartic
   0.791 → 0.819, which no longer contains sqrt(1/2).
3. Frontier v1 → v2: ops leaders named on point estimates were withdrawn; degenerate alpha intervals
   were replaced.
4. Negation audit: volcano-rho and the autoresearcher rho.py references lacked the negation map. Any
   earlier IC comparison against them was against a sqrt 2-weaker rho.
5. ic_rho_koblitz: the 1.17x figure is 6.51x against batch rho (Kuhn-Struik), and 43.7x for one
   target.
6. ecbench statistics v2: censored pairs had understated ratios by up to 92.9x (lower bounds).
7. Calibration round 1: two prediction failures were attributed to sample size; the follow-up passed.
8. curve25519 wwalk 1.43-1.55x readings were single table draws; the true figure is about 1.8-1.9x,
   matching the F=2 floor.
9. ecbench_independence: same-machine replay masqueraded as independent (hardware-class gate added).
10. HEC rho: the 8-bit branch-coefficient reference was slow at large N; its gains were falsified.

## Coverage table

| # | line of work | coverage | est. pages it deserves |
|--:|:--|:--|--:|
| 1 | Generic sqrt(r) lower bound | PARTIAL | 2 |
| 2 | IC-vs-rho collision floor (Gamma bound), e2e frontier census, rho_parity iterations | PARTIAL (floor ABSENT) | 4 |
| 3 | Matched-rho calibration, finite-size bias, missing prime medium tier | PARTIAL | 3 |
| 4 | Sealed bounds frontier v1→v2, challenge verdicts | PARTIAL | 3 |
| 5 | Pricing band reorders the rho references | PARTIAL (ABSENT for rho-vs-rho) | 1.5 |
| 6 | Negation map, fruitless cycles, negation audit, pool walk | PARTIAL | 3 |
| 7 | Automorphism-class walks (Koblitz, j=0/1728, GLS, F_{p^3}) | PARTIAL | 3 |
| 8 | Equivalence classes beyond Aut(E) | ABSENT | 0.5 |
| 9 | Strong-rho engineering and Lean/TLC assurance | PARTIAL | 3 |
| 10 | Native step cost, GAE vs native, harness overhead | PARTIAL | 2 |
| 11 | Walk-design leads (best-of, lazy DP, correlated walks) | PARTIAL | 2 |
| 12 | r-adding walk mixing | PARTIAL | 1 |
| 13 | Tag tracing / point halving (unexamined) | ABSENT | 0.5 |
| 14 | BSGS family, pair claw, memory trade | PARTIAL (claw ABSENT) | 2 |
| 15 | Grumpy giants, compiled tiered walk | ABSENT | 1.5 |
| 16 | Kangaroo / lambda | ABSENT | 1 |
| 17 | DP, Kuhn-Struik batch rho, BL precomputation | PARTIAL | 3 |
| 18 | Advice-table variants | ABSENT | 1 |
| 19 | Cheon auxiliary inputs (6 sub-lines) | ABSENT (1 sentence) | 7 |
| 20 | Low-Hamming-weight DLP | ABSENT | 1 |
| 21 | ECC2K-130 GPU rho campaign | PARTIAL (deferred to separate paper) | 2 |
| 22 | GPU/SIMD prime rho, pools | PARTIAL | 2 |
| 23 | Rho invariance across isogeny classes/volcanoes | ABSENT | 2.5 |
| 24 | Hyperelliptic-Jacobian rho references | COVERED (brief) | 1 |
| 25 | ecbench statistics v2, censoring, independence | PARTIAL | 2 |
| 26 | Generic relabellings as IC arms (known-log, curve25519 m=3) | PARTIAL | 1.5 |
| 27 | Generic DLP correctness fixes | COVERED | 0.5 |
| 28 | Frobenius-orbit IC vs rho exponent gap | COVERED | 0 (already) |
| | **total** | 3 COVERED, 17 PARTIAL, 8 ABSENT | **~57** |

## Suggested structure (new Part, about 55-60 pp)
- **Ch. A: The generic floor and the IC-vs-rho boundary** (~8 pp): §1, §2 (Gamma collision floor with
  Monte Carlo figure), §26, §8, §13.
- **Ch. B: The matched rho reference** (~14 pp): §3, §4, §5, §25, §9, §10. Calibration, frontier,
  band-on-references, statistics and engineering of the incumbent.
- **Ch. C: Walk design** (~11 pp): §6, §7, §11, §12, §23, §24. Negation, automorphisms, iteration
  functions, invariance across isogeny classes, Jacobians.
- **Ch. D: Memory, advice and many targets** (~9 pp): §14, §15, §16, §17, §18. BSGS/claw/grumpy,
  kangaroo, DP/Kuhn-Struik/BL, and the advice variants.
- **Ch. E: Auxiliary inputs: Cheon** (~7 pp): §19.
- **Ch. F: Rho at scale** (~5 pp): §21, §22, with a cross-reference to `paper/ecc2k130/`.
- **Appendix: LHW DLP and open generic leads** (~2 pp): §20 and the open_next_step list.
