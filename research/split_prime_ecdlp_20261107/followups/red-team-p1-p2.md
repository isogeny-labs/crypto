# Red-team review: P1 (RR–Dickson vs T) and P2 (twist-pure coset), 2026-11-07

Reviewer role: red team (core tier, Opus 5.5). I challenge the evidence here and decide nothing.
The Coordinator and the Validator decide and record. I edited no evidence and reran no
experiment. I made independent checks with scratch scripts outside the repository, listed
under "Checks run". The KB was unavailable, so I cite repository paths.

## Evidence reviewed and its status

| item | evidence | durable? |
|---|---|---|
| P2 | `p2_twist_coset/{PROTOCOL.md,RESULTS.md,runs/run-20261107T214118Z/,analysis/}`. Committed in `2b8fbd7d64` (on origin) and `1f46dcd197` (report PDF, local only). | yes |
| P1 | `p1_rr_dickson/PROTOCOL.md`, `p1tool/`, `cards/p1-run-01.yaml` and the assessment are committed (`5ffb16e932`, `1f46dcd197`). **Untracked:** `runs/run-20261107T214621Z/`, `analysis/p1analyze/`, `analysis/derived/{decision.json,p1_cells.tsv}`, `cards/p1-run-01.{receipt.json,run.stdout,run.stderr}`. No results note exists. | **no** |

**P1 standing: `insufficient_evidence` as durable evidence.** The run tree and the analysis
exist only in the working tree. Two working-tree digests match the director assessment:
`parsed.jsonl` `7f8c55f5…b0cf0` and `jobs.tsv` `6ecc5caf…8911e8`. No digest of
`decision.json` (`b110cc2a…7984`) or `p1analyze/src/main.rs` (`4b379c2b…11bd`) is recorded
anywhere. The P1 findings below are provisional. They still stand once the same bytes are
committed.

## Findings, ordered by severity

Tags: **confirmed**, **plausible** or **rejected**. Class: **wrong**, **unsupported** or
**unpriced**.

### P1-1 (high; confirmed; unsupported): the m5 t3 cell that decides rule 6 has no tower on the RR side

- **The system.** `runs/…/D24_m5_t3/rr_s2.ms` has 14 unknowns: a0–a2, c0, c1, u0–u4 and [pcc:unverified]
  q3_0–q3_3. It has 16 equations:
  - 6 coefficients of the norm identity;
  - 1 target-sign equation;
  - `q3_3 = 1`;
  - 8 coefficients of `U·Q3 = D_8(X) − c`. The constants are 14680055 = −8, 20,
    14680047 = −16 and 12882026 = 2 − c.
- **Why the tower is gone.** Tower levels 1 and 2 are not expanded because W², W⁴ have
  degree < m = 5. This is round7_rr's convention ("levels where W^{2^j} still has degree < m
  are left unexpanded"). Membership is therefore a single divisibility of a degree-8
  polynomial.
- **The T arm keeps its towers.** It has remainder chains on two levels per cherry
  (`t_s2.ms.plant.json`: h0r1, h0r2, h1r1, h1r2).
- **The base is tiny.** It has 8 abscissae, of which 6 (seed 2) or 7 (seed 5) lie on E
  (`emit.json` `base_points`). The planted relation uses 5 of them. This is the regime
  2^t < 2m, where a base this small cannot carry a relation search.
- **Round 7 points the other way.**
  - `research/ic_tree_gluing_20261103/round7_rr/README.md` names **m5 t4** as its deciding
    cell ("Deciding cell: m5 t4 RR at ≥ 6–12 GiB").
  - At m5 t4 it records RR matrices **25–60× wider than T's at equal degree**.
  - P1's m5 t3 shows the same per-degree pattern. At degree 5, RR has 2,878 columns and
    T has 337 (8.5×). At degree 4 the counts are 1,295 and 74.
  - RR wins only because its collapsed system stops at D = 5.
- **Verdict.** "Repository criterion met" follows the letter of round7_rr's criterion ("at
  least one m ≥ 5 cell"), and the frozen P1 protocol chose this cell. It does not show that
  RR beats T at m = 5 in any cell that has a membership tower.
- **Cheapest test.** Run RR-D and T-D at m5 t4 on T24, round 7's deciding cell. Until
  then, scope rule 6 as "single-division membership, base of 8 abscissae".

### P1-2 (high; confirmed; unsupported reading): at m = 3, T is not the lab's strongest algebraic system

- **The stronger arm.** Round 7's full-symmetric arm F is S_{m+1} in e₁..e_m
  (`research/ic_tree_gluing_20261103/README.md`: "m = 3: F is better"; `rows.tsv`).

  | cell | F: D | F: widest | RR: D | RR: widest |
  |---|--:|--:|--:|--:|
  | m3 t4 | 5 | 1,224 | 4 | 2,801 |
  | m3 t5 | 6 | 10,493 | 5 | 31,870 |

- **Wall time** (descriptive): F 0.12 s and 26 s; RR about 0.7 s and about 200 s.
- **Against F, RR is a trade at m = 3.** Its D is lower, but its matrices are 2.3–3.0×
  wider.
- **Rule 5 itself is faithful.** The protocol names only T. The reading "stage-level
  positive at m = 3" should name F as the stronger reference. F cannot be built beyond
  m = 4 (`STATUS.md`), so T is the right reference at m = 5.
- **Caveat.** The F numbers come from another experiment: a Kummer tower, round 7's curve,
  a 6 GiB limit. They cannot be combined into a claim; they are the reason to run an F arm.
- **Cheapest test.** Add F-D at m3 t3, t4 and t5 on T24 and T31, on the same instances and
  at the same 1.5 GiB limit.

### P1-3 (medium; reading sound, decision procedure post-data; unsupported as a frozen-rule outcome): the bound-aware reading behind rule 6 (known issue 1)

**The lower bound is sound (confirmed).**
- **(i) Only finished steps count.** `p1tool parse_log`, pinned pre-data at sha
  `0d9fa76e…`, counts a step only from a complete msolve line that carries the `new` and
  `zero` fields. The step running when the job died is excluded.
- **(ii) A capped log is a prefix of the full run.** D and the widest matrix are maxima over
  the F4 step sequence, and `msolve -t 1` is deterministic.
  - I diffed `K0_m3_t5/t_s1.log` (1.5 GiB, cap) against round 7's
    `logs/run600/m3_t5_k0_T.log` (6 GiB, completed), on deg, sel, pairs, rows and cols.
  - The first 18 F4 steps are identical, even though the two runs use different curves.
  - The capped job died in step 19.
  - The completed run's final widest is 23,720, equal to the capped prefix maximum.
- **(iii) The m5 t3 bound is conservative.** The last, incomplete line of
  `D24_m5_t3/t_s{2,5}.log` shows a degree-8 matrix of 60,855 × 82,927 already formed.
  T's true widest is therefore ≥ 82,927; the parser records 39,898.
- **Caveats.**
  - Exit 139 is a SIGSEGV. No allocation message confirms that the memory cap caused it
    (assessment item 7).
  - D is a quantity specific to msolve's strategy.

**The decision procedure was not frozen (confirmed).**
- **What the frozen text covers.**
  - Capped jobs are "recorded … and never imputed".
  - Rules 2 and 3 compare only **completed** cells.
  - The only cap clause in rule 6 covers "neither m = 5 arm completes".
  - The case where one arm completes and the other caps is not covered.
- **When the reading was written.** The bound-aware decision is at
  `p1analyze/src/main.rs` lines 40–48 and 91–98. It was written after the data:
  `analysis/` dates from 22:24, the receipt from 22:23.
- **What supports the semantics, though not the decision.** p1tool's `D_is_lower_bound`
  flag and round7_rr's "≥" convention.
- **Frozen-faithful verdict:** "5: stage-level positive at m = 3; rule 6 undecided (T-D m5
  t3 capped)". The bound-aware "met" can be reported as a post-data reading, labelled as
  such.
- **The bounds make the m5 t3 inequality robust** to any completion: 5 < 8 ≤ D_T, and
  2,878 < 82,927 ≤ widest_T.
- **Cheapest test.** Rerun jobs 39 and 40 (T-D m5 t3) at ≥ 6 GiB. If T completes, rule 6 is
  decided under the strict reading.

### P1-4 (medium; confirmed): the P1 evidence is not committed

See the status table above. The relay check is also open (assessment item 9).

### P1-5 (medium; plausible; unsupported by omission): adverse trend at m3 t5

- **P1's own data.** RR has 31,911 columns against T's lower bound of ≥ 23,852, on both
  Dickson primes. The largest matrices hold 1.47·10⁹ cells for RR against ≥ 5.37·10⁸ for T.
  The analyzer reports "Undecided".
- **Round 7's complete run.** The 6 GiB T run on the Kummer analog ends at widest 23,720,
  below RR's 31,870. round7_rr already calls m3 t5 "a trade".
- **Reading.** RR's width advantage holds at t ≤ 4 and very probably reverses at t = 5.
  The growing-size check (AGENTS.md §5) points against RR, and rule 5's "at least one cell"
  does not show it.
- **Cheapest test.** Rerun T-D m3 t5 at 6 GiB; round 7 finished it in about 544 s.

### P1-6 (low; rejected as a bias toward the claim, confirmed as a wording error): the memory limits (known issue 2)

- **Both P1 arms ran at 1.5 GiB** (`provenance.txt` `VLIMIT=1572864`).
- **Round 7 used different limits.** Its T arm ran at 6 GiB (`run.sh` default
  `VLIMIT 6291456`) and round7_rr's RR at 1.5 GiB. The protocol's "Engine and limits
  (round7_rr's)" is therefore accurate for RR only.
- **The cap cannot fake a win.** Under the sound bound reading (P1-3), a cap can turn a
  decided cell into an undecided one. It can never produce a false "RR below T".
- **What it does here.** It hides the m3 t5 trade (P1-5) and leaves rule 6 resting on
  bounds alone.
- **Round 7's T values enter P1 only through the m3 t4 control.** That cell completes at
  either limit.

### P1-7 (low; rejected as a defect): the quotient-dimension fix (known issue 3)

- **The fix is faithful.** Rule 1 fails on "quotient dimension is 0 (planted relation [pcc:unverified]
  lost)". A capped job has no quotient dimension: the parser writes null, which the analyzer
  maps to −1 (line 27). Reading completed jobs only (line 53) is correct.
- **Capped T jobs** have their planted relation verified by the emitter alone (`plant.json`:
  "all equations vanish").
- **Edge case.** A (cell, system) group with one completed job and one capped job would drop
  the completed job's quotient dimension from rule 1. No such group occurs in this run.

### P1-8 (low; confirmed correct): both m5 t3 systems encode the same planted relation

- Both systems use seeds 2 and 5 (`jobs.tsv` rows 37–40), and their `emit.json` instance
  blocks are identical.
- I recomputed the instance independently over F₁₄,₆₈₀,₀₆₃, on y² = x³ + 3,607,823x +
  6,826,282:
  - the five planted points, with their recorded y signs, sum to R exactly;
  - D_8(x_i) = c for all five;
  - the planted coefficients of U equal ∏(X − x_i).
- RR's quotient dimension is 1, so its unique solution is the planted relation.

### P1-9 (low; confirmed): the controls have little power

- **Identical metrics everywhere.** RR's D, widest and max-cells are byte-identical across
  T24, T31 and K0 at t = 3 and 4. T-D is identical on T24 and T31, and both seeds agree.
- **Why.** The Dickson step y ↦ y² − 2 has the same leading forms as Kummer's y ↦ y².
  Rule 3 ("transfer") is structurally expected, so "both primes" and "two targets" are not
  independent replications.
- **Sampler deviation.** The sampler requires base ≥ m + 1 points (`p1tool` main.rs
  line 831). Addendum A3 says "m distinct admissible abscissae". This changes only which
  seeds are used, and D does not depend on the seed.

### P1-10 (low; plausible): only planted, satisfiable instances were measured

Every P1 decision uses planted systems (quotient dimension ≥ 1). A relation search mostly [pcc:unverified]
solves systems with no solution. D on those is unmeasured, here and in round 7.

### P2-1 (high; confirmed; unpriced): the gain ignores factor-base size

- **What rule 4's G measures.** G = (c_sel/c_rand)^m is the relation yield of one F4 solve.
- **What an index calculus needs.** About c_E relations: by rule 1 twist columns never
  appear, so the matrix has c_E unknowns up to sign.
- **The stage cost.** Filling the base costs c_E·W/coverage ∝ c_E^{1−m}·W. At fixed t the
  relation-stage ratio is therefore (c_sel/c_rand)^{m−1}.
- **Corrected ranges** (t = 4, curve 0):

  | m | reported G | relation-stage ratio |
  |---|---|---|
  | 2 | 2.25–25 | 1.5–5.0 |
  | 3 | 3.4–125 | 2.25–25 |

  The headline "2.25 to 125" becomes "1.5 to 25".
- **Linear algebra** grows by (c_sel/c_rand)² or more.
- **Rule 4 still passes** its 1.2 threshold (minimum 1.5).
- **Matched comparator not run.** At equal base size the comparator is a random coset at
  t + 1, with c_E ≈ 2^t. M3's W grows about 42× from t = 4 to t = 6 at m = 2.
- **Cheapest test.** Recompute c_E·W/coverage per arm from the committed TSVs, which needs
  no run. Add a random-coset m2 t5 arm, which takes seconds.

### P2-2 (medium; confirmed mis-specification, plausible anomaly; unsupported headline): gain against one random coset (dispatch question)

**Mis-specification (confirmed).**
- The precedent that P2 cites, `research/curve25519_frame_search_20261103/README.md`
  (Predictions, P3), already concluded: "P3 was mis-specified: it compares against one
  random draw … The gain over the mean frame is the curve-independent statistic."
- Rule 4 repeats that design: its random arm is candidate 0.
- The figures 25 and 125 come from c_E = 3 draws.
- The fair statistic is (c_max/c_mean)^{m−1} with the P2-1 correction, or
  (c_max/c_mean)^m without it. At t = 4 these are about 1.9 and 3.5 (m = 2), and 3.5 and
  6.6 (m = 3).

**Anomaly (plausible, unexplained).** [pcc:unverified]
- On Kummer 1,049,089 at t = 4, the idx0/idx1 draws are (3, 10), (4, 3) and (3, 4): five
  of six are ≤ 4 of 16.
- Under Binomial(16, ½), five or more of six below that level has probability about
  5×10⁻⁷, or about 6×10⁻⁶ after allowing for the 12 (prime, t) blocks.
- **The seeds follow the frozen rule.** I verified them against
  SHA-256("split_prime_ecdlp_20261107/P2/cosets/1049089/<i>/4").
- **The coset lists reproduce.** I regenerated them independently (c = 70432, 430236, …).
- **The first draw is unbiased.** Over 3,000 other seeds, mean c_E is 8.00 and
  P(c_E ≤ 4) is 0.033–0.042.
- **No mechanism found.** The seed-mixing constants (`0xC05E7`, `t<<40`) were fixed in the
  instrument after the seed strings were frozen.

**Rule 4 does not depend on it.** Against idx1 the minimum gain is still 2.25, and against
c_mean it is about 3.5.

### P2-3 (medium; plausible; unsupported): F4 cost measured only on planted targets

- `p2coset emit` plants a sum of m E-points (`m3.jsonl` `planted_verified`; quotient
  dimensions 2 and 6).
- At the M2/M3 coverages (10⁻⁵ to 5·10⁻³), more than 99 % of relation-search attempts have
  no solution.
- The per-attempt W, and rule 2's coset independence, are therefore untested for the
  dominant case.
- **Cheapest test.** Run 4 uniform targets without a planted solution per arm at m2 t4 and
  m3 t4 on curve 0.

### P2-4 (low–medium; confirmed): rules with little falsification power, a known lever, no class

**The four rules were close to certain to pass.**
- Rule 1 restates a lemma that is already Lean-checked.
- Rule 2 follows from the tower's leading forms not depending on c. RESULTS says so, and
  D and the widest matrix are identical in every group.
- Rule 3 at m = 2 is an identity (Addendum A6).
- Rule 4's statistic is ≥ 1 by construction (argmax). It fails only when idx0 is within
  1.095× of the maximum.

**The Kummer half is a known lever.**
- It is the scaling sub-case (x ↦ x/g) of the Möbius-frame selection measured in
  `curve25519_frame_search_20261103`. P5 there: F4 mul-adds per relation fall 4.71× at t = 5.
- P2 extends that lever to prime-order curves and the Dickson torus.

**No class given.** RESULTS does not classify the change as AGENTS.md §3 requires.

### P2-5 (low; confirmed wrong, immaterial): analyzer errors

- **T\* uses the wrong coverage.** `p2analyze/src/main.rs` computes T* for the m = 3 rows
  from m = 2 coverage (the cov tuple field `.1` is `m2_coverage`). With the measured m = 3
  coverage, T* at kummer-1049089 m3 is about 1.8·10⁻⁷ rather than 7.7·10⁻⁸; it remains
  ≪ 1.
- **T\* is in the wrong unit.** It is expressed in relations; the protocol defines T* in
  targets.
- **The m = 3 G is a formula, not a measurement.** The measured m = 3 coverage ratios
  (`m2.jsonl` `m3_coverage`) on Kummer 1,049,089 are:

  | curve | formula | measured | deviation |
  |---|--:|--:|--:|
  | 0 | 125 | 102.9 | −17.7 % |
  | 2 | 151.7 | 124.0 | −18.3 % |

  The analyzer never reads `m3_coverage`, so "coverage follows c_E^m" is checked at m = 2
  only.
- **Selection cost omits coset construction.** The 2^t multiplications per coset and the
  f(x) evaluations are left out; the omission is small.

### P2-6 (low; confirmed OK): scope

The note is scoped correctly: a toy-scale stage diagnostic, with no S and no rho
comparison, the validation gate not run, and the claim marked `[pcc:unverified]`. The
decay in t matches prediction: c_max/c_mean → 1 at the base sizes a cryptographic-size
attack would need.

## Cheapest falsification of each claimed outcome

| claimed outcome | cheapest test | what would change the outcome |
|---|---|---|
| **P2 rule 4** (toy-scale gain) | 4 targets without a planted solution per arm at m2 t4 and m3 t4, curve 0 | W differs by more than 10 % between the selected and random cosets, which kills under rule 2. The factor-base correction (P2-1) gives at least 1.5 ≥ 1.2, so the outcome survives it. |
| **P1 rule 5** (m3 t3, t4, both Dickson primes) | replay the 8 jobs (seconds) and add F-D arms | The values are exact counts that do not depend on the seed, so a replay would only confirm them. The F arm would turn the reading into a trade against F. |
| **P1 rule 6** (m5 t3) | (a) T-D m5 t3 at ≥ 6 GiB | Decides rule 6 under the strict reading. The prefix bounds (D ≥ 8, widest ≥ 82,927) make a reversal implausible. |
| | (b) RR-D vs T-D at m5 t4 on T24 | If RR is not below T on both D and width, rule 6 does not extend past the cell where the tower collapsed. |

## Checks run (scratch only, nothing written to the repository except this note)

- **Capped-log prefix.** I diffed the step lines of the P1 capped log (K0 m3 t5) against
  round 7's completed 6 GiB log.
- **P2 coset draws.** I reimplemented p2coset's SplitMix64 coset draw. It reproduces the
  first coset values exactly, and I used it to test the first-draw distribution over 3,000
  seeds.
- **P2 seeds.** I checked the coset seeds against the frozen SHA-256 rule.
- **P1 m5 t3 plant.** I recomputed the point sum, D_8 membership and U from the plant
  JSON.
- **Hashes.** I checked the bound instrument hashes: p2coset `963c2f08…`, p1tool
  `0d9fa76e…`, P1 PROTOCOL `cc04efc9…`. I checked the P1 working-tree digests against the
  assessment.
