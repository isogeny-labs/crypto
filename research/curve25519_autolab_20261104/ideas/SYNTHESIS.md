# Curve25519-family ECDLP: cross-program synthesis on the remaining factor of three (Idea Synthesist, 2026-11-04)

Status: **proposals only**. Nothing here is a measurement, a result or a status change. Each number is
either quoted from a committed file (path and claim id given), *derived* (arithmetic in this file on
quoted numbers), or *predicted* (to be frozen by the Coordinator before any run). The autolab outcomes I
quote are `statement` claims with standing `unverified` (AGENTS.md "Sabotage is assumed"); the other
in-repo measurements I cite have no claim id and are unverified assertions.

## 0. Bottom line

1. **Nothing known reaches parity with matched rho-neg on these curves, and nothing in the
   generic-decomposition track can.** The lab's own generic-group argument
   (`research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` §10.1, §10.6) sets the floor
   `κ_total >= sqrt(2K/γ)` for K independent relations. With C unknown seed columns, K = C + 1 and γ = 2
   (`j ∉ {0, 1728}`, `research/curve25519_ic_baseline_20261102/STRUCTURE.md` line 24). Refined for Poisson
   collision counts, the expected number of elements is `Γ(K + 1/2)/Γ(K) · sqrt(r)`: 0.886 (C = 0, which is
   rho), 1.329 (C = 1), 1.662 (C = 2), 1.939 (C = 3). Divided by rho's `sqrt(π/4)`, that is a distance of
   **1.5x, 1.875x and 2.19x at C = 1, 2 and 3**, before any overhead. Parity needs C = 0. C = 0 is
   rho, or baby-step giant-step (BSGS), which `docs/ecbench/README.md` lines 914-926 class as a
   time-memory trade ("never a finding"), not as index calculus.
2. **The measured factor of 3.0 to 3.6 splits exactly into four named factors** (Table 1):
   - unknown columns: 2.19, irreducible at C = 3;
   - table-vs-probe pairing: 1.46 to 1.51, reducible to 1;
   - fixed set-up: 1.04 to 1.84, reducible;
   - rho's own overhead: 0.60 to 0.94, which works in IC's favour at small k.

   The autolab has driven the incumbents to within 4% of their own design floor at k = 28 and 32. What
   is left there is the design, not the tuning.
3. **Best attainable in the generic track** (derived): about 1.95 sqrt(r) at C = 3 and 1.67 sqrt(r) at
   C = 2 with all-pairs harvesting (R1). That is 2.1x and 1.8x rho at k = 32. At k = 24 it is about 1.7x
   and 1.5x, only because matched rho still carries about 0.3 sqrt(r) of overhead there. The ratio rises
   with k toward 2.19x and 1.875x.
4. **Algebraic track.** Under §5a at w = 1, with linear algebra at least N^2 native operations and the
   counting bound on decompositions, an arity-m method reaches matched rho at k = 24 / 32 only if one
   decomposition trial costs at most c*(m, k) native operations (Table 2):
   - m <= 3: excluded outright;
   - m = 4: about 36 / 88;
   - m = 5: about 114 / 520;
   - m = 6: about 180 / 970.

   The lab's measured prime-field Semaev oracles spend about 7·10^5 F4 multiply-adds per target at m = 2
   (`research/curve25519_isogeny_seed_20261103/README.md`, engine table). The gap is at least 10^3 to
   10^4, before the far larger m >= 4 systems.
5. **What would change this** is listed in section 6. In short:
   - an oracle that makes distinct formal combinations collide with probability above γ/r, at a trial cost
     below c*;
   - a sparse solver for the relation system in o(N^2);
   - an automorphism of order above 2 acting on the r-subgroup, which these curves do not have.

   A generic-track S measured below `0.9 · Γ(C+3/2)/Γ(C+1)` is an accounting error to chase, not a result.

## 1. The connection: the gap, decomposed

Table 1 is derived from the holdout S at w = 0 in the round READMEs. The incumbent's design is a
table-probe design. Its floor is `T + (C+1) r/(2T)`, the cost of T box entries at one addition each plus
geometric probes with hit rate 2T/r. The all-pairs floor is `Γ(C+3/2)/Γ(C+1)` = 1.939 at C = 3.

| k | incumbent (C, B, T) [claim] | S inc | design floor | set-up factor | pairing factor (floor / 1.939) | unknowns factor (1.939 / 0.886) | S rho-neg | rho factor (0.886 / S rho) | product = S inc / S rho |
|--:|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 20 | `wsum-b5-n3` (3, 5, 665) [PCC-20261104-1c867a] | 5.384 | 2.925 | 1.84 | 1.51 | 2.19 | 1.487 | 0.60 | 3.62 |
| 24 | `wsum-b8-n3` (3, 8, 2456) [PCC-20261104-b49545] | 3.678 | 2.875 | 1.28 | 1.48 | 2.19 | 1.209 | 0.73 | 3.04 |
| 28 | `wsum-b12-n3` (3, 12, 7812) [PCC-20261104-13dcb4] | 2.949 | 2.832 | 1.04 | 1.46 | 2.19 | 0.981 | 0.90 | 3.01 |
| 32 | `wsum-b20-n3` (3, 20, 34460) [PCC-20261104-997808] | 2.937 | 2.832 | 1.04 | 1.46 | 2.19 | 0.939 | 0.94 | 3.13 |

The k = 24 incumbent moved after the brief was written: round 20 replaced `wsum-b4-n4` (S 4.201) with
`wsum-b8-n3` (S 3.678). The brief's k = 32 breakdown, 34.5k table additions and 36k trials, fits the model:
34,460 entries, plus an expected 31,158 trials with a standard deviation of about 15.6k per run.

**Why no single lane could see this.**

1. **Autolab** searches (C, B) inside the table-probe design. That design sits at sqrt(2) times the
   all-pairs floor, because it uses only table-probe pairs.
2. **The residual-walk thread** stored every element and reached its all-pairs floor
   (`κ_total/κ_floor` between 1.00 and 1.07, `RESEARCH_RESIDUAL_WALKS.md` §10.6 item 3). It did so with
   B = 256 unknowns, where the floor is 16.
3. **The pipeline's guard** throws away probe-probe collisions by policy (`src/cryptanalysis/ic_boundary.rs`,
   doc comment of `TargetSource::is_guarded`, around line 8717). Such a collision pins d with no
   factor-base part: "measuring rho with extra steps". The guard nevertheless already pays one hash
   insert per probe.
4. **The box's own table-table pairs carry almost no rank** (§1.1).

Small C combined with all-pairs harvesting of seed-carrying rows was in no lane. That combination is R1.

### 1.1 Why the box wastes its own collisions (derived; the same effect as round 15)

Two box entries collide exactly when their coefficient difference is a nonzero vector of the kernel
lattice `Λ = {v : Σ v_c s_c ≡ 0 (mod r)}` with `|v|_∞ <= 2B`. Averaged over seed logs, the expected number
of colliding *pairs* equals the birthday value of about T^2/r: 3.4, 2.9, 1.8 and 2.2 at k = 20 to 32.
That is the D the P2 and P3 preregistrations use (`ideas/PROPOSALS.md`).

The *rank* is a different quantity. It is the number of distinct kernel directions, about
`((4B+1)^C - 1)/(2r)` = **0.035, 0.0086, 0.0018 and 0.0005** at the incumbents. When a direction exists, it
arrives as a burst of order T/4 pairs that all repeat one row. So the box's free rows have rank 0 in
more than 96% of bases at every size, which is why the k = 32 incumbent still needs four probe hits.

Round 15 measured the same thing on `ksum`: "colliding 4-sums share summands and repeat few relations"
[PCC-20261104-836de4]. Shoup's per-pair bound is exact. What fails for small-coefficient tables is
independence across pairs. Birthday-like rank needs the stored set's difference set to have volume much
larger than r.

There is a corollary for BSGS-style tiling. BSGS gets its factor of 2 from stepping a giant stride along
a seed axis so the box translates tile the group. That works only when the kernel lattice is known: C <= 1,
or known logs. For C >= 2 the lattice is random and unknown. The tiled coverage then loses `f^2/2` of the
group at fraction f, which is random sampling to second order. Tiling is therefore not proposed.

## 2. (a) Ideas that would change the exponent: these need structure

- **Literature.**
  - Semaev's summation polynomials (2004).
  - Gaudry (2009) and Diem (2011) on Weil descent, which needs an extension field; a prime field has none.
  - Petit-Kosters-Messeng (PKC 2016), towers of rational or isogeny maps. Its subexponential estimates
    rest on first-fall-degree heuristics, which the lab's L1 note shows are not supported for towers.
  - Huang-Kosters-Yeo (CRYPTO 2015), last fall degree.
  - Amadori-Pintore-Sala (FFA 2018), one relation over a random factor base. This is the C = 0 endpoint
    with algebraic membership testing.
  - Kudo-Yokota-Takahashi-Yasuda (CANS 2018), "acceleration ... and its limitation".
  - Galbraith-Gebregiyorgis (2014): characteristic 2, rho still faster.
  - The Galbraith-Gaudry survey (2016).
  - Hu, ePrint 2024/1923: "no subexponential time method" known over prime fields.

  I found no 2025 result with a verifiable prime-field claim (section 7).
- **The lab's own record.**
  - `research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md`. Theorem 1: a direct presentation
    costs at least c_m·r. Theorem 2: a tower presentation can go below r^{1/2} only if the per-attempt
    exponent satisfies c < (m-1)/2.
  - `research/prime_field_leads_20261031/SYNTHESIS.md`:
    - C3: "empty window at m = 3, one boundary point at m = 4";
    - C4: the small-x base has the additive energy of a random set (0.993 to 1.013);
    - L3: Bernstein-Lange precomputation is 2^168 for Curve25519.
  - `research/ic_arity_sweep_20261027/README.md`: no m in 2..9 crosses rho.
  - `RESEARCH_RESIDUAL_WALKS.md` §10.4-10.5: S_3 and S_4 oracles cost 162x and 566x more than the plain
    table. Meet-in-the-middle triples cost 4.6x more and scale as Θ(n^{2/3}).
  - `problems/curve25519-family-algebraic-ic-tower.md`: the isogeny-tower oracle exists. Its best cost is
    S = 2.5·10^6 at w = 0.1.
- **The connection.** Splitting S_{m+1} into S_3 factors through auxiliary variables (Semaev 2015,
  arXiv 1504.01175) turns an m-decomposition into a meet-in-the-middle over pair tables. That is the
  generic track again, so it is bounded by section 1. Structure can only enter through a factor base
  whose membership is a cheap polynomial condition, solved faster than enumeration. C4 says the small-x
  base offers no additive structure at the 4-term level.

**Table 2. Algebraic parity budget c*(m, k) at w = 1** (derived; R4).

The model: N columns, N + 1 relations, linear algebra charged N^2, decomposition probability
`p <= min(1, C(2N+m-1, m)/r)`. Each trial costs one group operation plus c native operations. The target is
the matched rho-neg S at w = 1 times sqrt(r): 3,332 GAE (k = 24, round 20) and 43,143 GAE (k = 32,
round 19).

| m | k = 24: best N, c* | k = 32: best N, c* | binding constraint |
|--:|:--|:--|:--|
| 2 | none | none | linear algebra: N about sqrt(r/2) gives N^2 >> sqrt(r) |
| 3 | N about 41, c* < 1 | N about 147, c* < 0.2 | linear algebra plus the counting bound: excluded |
| 4 | N about 42, c* about 36 | N about 161, c* about 88 | both |
| 5 | N about 24, c* about 114 | N about 73, c* about 520 | decomposition; p saturates |
| 6 | N about 17, c* about 180 | N about 43, c* about 970 | decomposition; S_7 has degree 32 per variable |

At w = 0.1 the budgets are roughly 10x larger. The band requires w = 1 as well, and w = 1 binds.

## 3. (b) Ideas that change a generic constant, and whether the rules count them

| idea | effect on the constant | counted as an IC improvement? |
|:--|:--|:--|
| Negation fold | already in both arms (γ = 2) | no relative gain |
| Order-3 or order-4 automorphisms | sqrt(3) or sqrt(2) on IC and rho alike (`RESEARCH_RESIDUAL_WALKS.md` §10.3) | not available: `j ∉ {0, 1728}` |
| Distinguished points, Brent cycle detection, fruitless-cycle handling | rho's overhead only | improves the reference (`rho.*_v2`), never IC |
| Tag tracing (Cheon-Hong-Kim), simultaneous inversion, X ± Y in one inversion (Galbraith-Wang-Zhang, ePrint 2015/605), x-only and batched affine arithmetic | field-level cost per group operation | no: S counts group operations. Round 5 and round 21 record `mitmbatch` and `mitmblocked` at identical S. Rho gets these tricks too. |
| Interleaved BSGS and grumpy giants | GWZ 2015/605: average case below rho "in the fully-optimised context". ecbench calibrates BSGS at 1.5, 4/3 and 1 (`docs/ecbench/README.md` line 909) | no: known-log tables are BSGS ("time-memory trade, never a finding"); the claw pair table is "not an index-calculus candidate" |
| Known-log seeds, or the Amadori-Pintore-Sala one-relation variant (C = 0) | the floor falls to rho's | no: that is BSGS or rho |
| All-pairs harvesting of seed-carrying rows | the pairing factor 1.46 falls to 1 | **R1**: admissible by the letter of the guard (every row has a factor-base part); the owner rules |
| Fewer seeds (C = 2) | sqrt(2(C+1)): 2.83 falls to 2.45 | **R2**: yes, report C |
| Cheaper walk set-up | the set-up factor 1.28 to 1.84 at k = 20 and 24 | **R3**: yes, engineering |
| Multi-target amortisation (L targets share the table and the seeds) | needs C + L rows. The per-target ratio to Kuhn-Struik pooled rho is about sqrt(1 + C/L) (all-pairs; derived): 1.34 at L = 4, 1.09 at L = 16, approaching 1 from above, never below | separate question (AGENTS.md "Primary comparison uses one target"). The matched reference is `rho.multitarget_pooled` (`docs/ecbench/README.md` line 508). Not proposed: it cannot pass rho, and R1's C-ladder already tests the same floor. |
| Precomputation / online clock (Bernstein-Lange: r^{2/3} once, r^{1/3} per target) | an online-only S below rho | no: ecbench S is cold. The note's precomputation for Curve25519 is 2^168 against rho's 2^126 (L3). |

## 4. (c) Accounting and relabelling traps to avoid

1. **The C -> 0 endpoint.** C = 1 is BSGS twice to an unknown base. C = 0 is BSGS or rho. Report C as a
   column in every table, as `ideas/PROPOSALS.md` §0 already asks.
2. **Base-free rows.** In any all-pairs design, a collision whose seed part is zero pins d by itself, and
   that is rho. The guard's rationale (`ic_boundary.rs` around line 8717) is the policy. Count such rows,
   never use them, and void any solve that did.
3. **Rho's fixed overhead at small k.** Matched rho carries 0.60 sqrt(r) of overhead at k = 20 and 0.32
   at k = 24. A low-set-up generic arm can approach or pass it there while its ratio rises with k. A
   versus-rho reading needs all four sizes (AGENTS.md §2, "at growing sizes").
4. **Counting colliding pairs instead of rank.** For the box, pairs track T^2/r but rank is near 0 (§1.1).
   A "free rows" headline is relabelling unless S falls.
4a. **Unpriced shifting.** Storing more elements moves cost into table operations that w = 1 prices. A
   gain that holds at w = 0 only is unresolved (rounds 6 and 10).
5. **Online clock.** AGENTS.md "IC measurement rules" allow reporting reusable set-up separately. Seed logs
   and tables are target-independent, so an online-only number must carry its set-up and its L. The
   headline is cold S.
6. **Field-level tricks** move wall time, not S, and the matched rho gets them too.
7. **Winner's curse.** Round 7's train leader regressed on holdout [PCC-20261104-15a1d6]. Keep fresh
   holdouts.
8. **Below the floor means a bug.** A generic-track S below `0.9 · Γ(C+3/2)/Γ(C+1)` is an accounting
   error first (replay, unpriced counters, base-free rows).

## 5. Ranked proposals

Ranking is by the expected reduction in the k = 32 distance, discounted by implementation risk. R2 is the
cheapest and can run first. R4 changes no S; it is the falsification target for the "no parity"
conclusion.

### R1. All-pairs collision walk over C seeds

This is the transfer of the residual-walk all-pairs design onto wsum's small-C basis.

```yaml
id: SYN-R1-allpairs-cwalk
title: "Store every seed-carrying walk element; every collision with a nonzero seed part is a row"
track: generic decomposition; class if it works: engineering (kappa_total/kappa_floor falls from ~1.46 to ~1.0; the floor does not move)
obstruction_attacked: "pairing factor 1.46-1.51 (Table 1): the table-probe design uses only cross pairs and the box's own pairs have rank ~0"
claim: >
  An oracle `cwalk` stores every walk element E_t = [x_t]G + [u_t]Q + sum_c [e_tc]P_c, with one x-keyed entry per
  ±class. The C seeds come from `prime-projected-abscissa:size=C`. Jumps are small-coefficient, all-positive and
  chosen at random over {G, Q, P_1..P_C}: no scalar multiplication, no deterministic step map, so no merging and
  no fruitless cycles. Every key collision whose seed part is nonzero becomes a row over (d, s_1..s_C).
  The oracle solves after Gamma(K+1/2)/Gamma(K) * sqrt(r) stored elements (K = C+1), within 10%:
  kappa_total / Gamma-floor in [0.95, 1.10] at k = 28 and 32. Its S is below the wsum incumbent at
  w = 0, 0.1 and 1 at k = 24 and 32 (at w = 1 for C = 3 only if one table operation is charged per element).
mechanism: >
  Each pair of distinct formal combinations collides with probability 2/r (Shoup; RESIDUAL_WALKS 10.1). The walk's
  coefficient vectors spread over a region whose difference set has volume >> r, so pairs give independent rows.
  The box's pairs do not (1.1). Stored elements cost one addition each, like table entries. Every pair now counts,
  not only table-probe pairs, so P falls from sqrt(2K r) to about sqrt(K r). The residual-walk thread reached this
  floor at 1.00-1.07 with B = 256. Here B is replaced by C = 2..3. Runs identically on a random base.
prediction:
  S_w0:  {k24: {C3: 2.06, C2: 1.78}, k32: {C3: 1.95, C2: 1.67}}         # set-up ~180 GAE + [d]G check; inc 3.678 / 2.937
  S_w01: {k24: {C3: "2.25 [u=1] - 2.45 [u=2]", C2: "1.95 - 2.11"}, k32: {C3: "2.14 - 2.34", C2: "1.84 - 2.00"}}   # inc 3.920 / 3.213
  S_w1:  {k24: {C3: "4.00 - 5.94", C2: "3.44 - 5.10"}, k32: {C3: "3.89 - 5.83", C2: "3.33 - 4.99"}}               # inc 6.100 / 5.697
  ratio_sel_over_inc_w0: {k24: {C3: "0.56 [0.50, 0.65]", C2: "0.48 [0.43, 0.57]"}, k32: {C3: "0.66 [0.62, 0.76]", C2: "0.57 [0.53, 0.66]"}}
  distance_to_rho_u1: {k24: {C3: "1.70x / 1.71x / 1.74x", C2: "1.47x / 1.48x / 1.49x"}, k32: {C3: "2.08x / 2.07x / 2.09x", C2: "1.78x / 1.78x / 1.79x"}}
  u: "unpriced operations per stored element. The incumbent measures ~0.97 at k = 32 ((5.697 - 2.937) sqrt(r) / (T + M)). One entry-API probe-and-insert gives u = 1. A separate lookup and insert gives u = 2, and C = 3 at k = 32 is then unresolved at w = 1."
  note: "At k = 24 part of the gain is set-up (R3's mechanism). The k = 28 and 32 rows isolate the all-pairs effect: set-up is under 4% of S there."
discriminating_test: >
  Stage 0 (exact count, no ecbench): native subcommand `curve25519_family box-kernel <k> <C> <B> <bases>` on the
  k = 20 and 24 miniatures, 1000 bases each. Build WSumTable with keep_collisions, verify the rows with free_rows,
  and report pairs, distinct rows and rank. Recompute the kernel vectors in [-2B, 2B]^C from brute-force seed logs
  (preflight only). Predicted: rank >= 1 in 3.5% (k20, B5) and 0.9% (k24, B8) of bases; pooled pairs within 0.5-1.6x
  of T^2/r, with variance far above Poisson.
  Stage 1 (ecbench smoke): k = 24 and 28, 16 fresh targets x 2 rounds. Arms: rho-neg, inc, cwalk C in {3, 2, 1, 0},
  inc-aa. Then the autolab freeze/holdout rule at k = 24 and 32 for the better of C = 3 and C = 2.
falsification: >
  Any of the following kills R1:
  - The C = 0 positive control has mean S outside [0.85, 1.05] at k = 28: the walk is not birthday-random; stop
    before reading C > 0.
  - kappa_total / Gamma-floor > 1.15 for C = 3 at k = 28 or 32.
  - S(C = 3) >= 0.85 x inc at w = 0 at k = 32.
  - Any solve that used a row with a zero seed part, or any unverified solve.
  - S below 0.9 x Gamma-floor: an accounting error, not a result.
  Stage 0 alone: rank >= 1 in more than 3x the predicted share of bases refutes 1.1, not the walk.
cost_band: >
  Every arm is priced at w = 0, 0.1 and 1 by `ecbench band --ranks`.
  - Counted: one addition per stored element; small-multiple tables of G, Q and the P_c (about 16(C+2) additions);
    jump construction (about 16(C+2)); the internal [d]G = Q check; base scan and projection (3 doublings per seed).
  - Unpriced: one table operation per element; canonicalisation; linear algebra on (C+1) columns.
  - Coefficient updates: keep them as small integers with a checkpoint every 64 steps, so no per-element mod-r work.
    The incumbent's walk already pays 2 mod-r updates per step in bitop/v1. Freeze which counters the band reads
    from the incumbent's record before the run.
  - The rho-neg and inc arms use their own frozen accounting.
controls: [inc-aa (A/A), rho-neg, "cwalk C = 0 (positive control; classed rho, never an IC row)", "C = 1 (classification probe: BSGS-to-unknown-base)", "wrong-logs control in Stage 0 (permuted seed logs: zero kernel vectors must verify)"]
risks:
  - "Classification: under its letter the guard admits seed-carrying rows; under its spirit ('rho with extra steps') the owner may not. Report C, the base-free row count (must be 0) and the rank from seed-carrying rows."
  - "Small jumps over 16 choices: segment repeats could correlate rows. Mitigated by random (not point-determined) jump choice; the C = 0 control detects it."
  - "u = 2 implementations lose the w = 1 order at C = 3, k = 32."
  - "Cannot reach parity: the floor is 1.875x (C = 2) or 2.19x (C = 3) of rho asymptotically."
closest_baseline:
  in_repo: ["RESEARCH_RESIDUAL_WALKS.md 10.1-10.6 (all-pairs, B = 256, kappa_total/floor 1.00-1.07)", "wsum (ic_boundary.rs WSumTable)", "the guard (TargetSource::is_guarded)"]
  published: ["Kuhn-Struik (SAC 2001) multi-collision rho with stored points", "van Oorschot-Wiener parallel collision search", "Amadori-Pintore-Sala (C = 0 endpoint, algebraic membership)"]
  difference: "seed-carrying stored elements on a 2-3 column base; nobody here combined small C with all-pairs rows"
code_needed:
  files:
    - "src/cryptanalysis/ic_boundary.rs: `CollisionWalk` (key -> element index map; checkpointed coefficient vectors; random small-jump table over {G, Q, P_c}; a collision emits Relation::Row{columns: delta e, a: delta x, b: delta u}, with its sign from s1 == s2 or s1 == -s2; rows with zero seed part counted as `base_free_rows`, never emitted)"
    - "src/cryptanalysis/ic_framework/plugins.rs: `CollisionWalkOracle` (params: jumps, coef bound)"
    - "src/cryptanalysis/ecbench/methods.rs: `\"cwalk\"` arm in solve_ic_prime; counters elements_stored, table_ops, base_free_rows, dependent_rows"
    - "src/bin/curve25519_family.rs: `box-kernel` preflight (native Rust)"
    - "research/curve25519_autolab_20261104/families.json: cwalk C = 2, 3 entries; C = 0 as a control family outside the IC track"
  ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=3,scan_cap=48, oracle=cwalk:jumps=16,coef=16"
lean_obligations:
  - "collision_row_sound: in Z/r, X = +/- Y for X, Y with known (x, u, e) decompositions gives the linear row (as P2's)"
  - "rows_determine_d: K = C+1 inhomogeneous rows whose (d, s) block is invertible determine d"
  - "box_collision_iff_kernel_vector (Stage 0; listed in PROPOSALS P3)"
  - "The Poisson/Gamma expectation is a heuristic, not a Lean claim."
parity_statement: "Narrows the gap only: 3.1x -> about 2.1x (C = 3) or 1.8x (C = 2) at k = 32. Parity is excluded for C >= 1 by the floor."
```

### R2. Two seeds on the existing box (configuration only)

```yaml
id: SYN-R2-box-C2
title: "wsum with C = 2 seeds: the untried point between the C = 3 incumbents and the C = 1 BSGS endpoint"
track: generic decomposition; class: engineering
obstruction_attacked: "the unknowns term of the table-probe floor sqrt(2(C+1)): 2.83 at C = 3, 2.45 at C = 2"
claim: >
  No autolab spec has used `size=2` (grep of research/curve25519_autolab_20261104 for `size=2,` and `-n2`: none).
  At T* = sqrt(3r/2) the C = 2 box costs 2.449 sqrt(r) plus the incumbent's set-up. That is below the C = 3
  incumbents at every weight at k = 24 and 32.
mechanism: "Same box and probes; one fewer unknown, so 3 rows instead of 4. Box kernel rank (4B+1)^2/(2r) is about 2e-4 at k = 32: no free rows, as at C = 3."
prediction:
  configs: {k24: "B = 29 or 30 (T = 1740 / 1860)", k28: "B = 59", k32: "B = 119 (T = 28560)"}
  S_w0:  {k24: "3.25 [3.0, 3.5]", k32: "2.55 [2.47, 2.70]"}       # inc 3.678 / 2.937
  S_w01: {k24: 3.46, k32: 2.78}                                     # inc 3.920 / 3.213
  S_w1:  {k24: 5.31, k32: 4.88}                                     # inc 6.100 / 5.697 (u = 0.84 / 0.95, from the incumbents)
  ratio_sel_over_inc: {k24: "0.88 / 0.88 / 0.87", k32: "0.87 / 0.87 / 0.86"}
  distance_to_rho: {k24: "2.69x / 2.62x / 2.31x", k32: "2.72x / 2.69x / 2.62x"}
discriminating_test: "One autolab round, exploration slots wsum-b{24,30,36}-n2 at k = 24 and wsum-b{100,119,140}-n2 at k = 32, frozen rule, fresh holdout."
falsification: >
  The best C = 2 arm is not below the incumbent on holdout at w = 0 with band order sel < inc. Or its mean trials per
  run deviate from 3r/(2T) by more than 20% (rows dependent at C = 2). Or any unverified solve.
cost_band: "As every wsum round: inc, the C = 2 arms, rho-neg, inc-aa; counted table and probe additions, unpriced inserts, lookups and guard probes, all at w = 0, 0.1 and 1."
controls: [inc-aa, rho-neg, "wsum C = 1 at the matched T as the BSGS endpoint (reported, not ratcheted)"]
risks: ["Classification: C = 2 is one step from the BSGS endpoint; the owner rules. Report C.", "WSUM_MAX_B = 255 and the 64-bit packing allow B = 140 at C = 2 (8 bits x 2)."]
closest_baseline: "rounds 16-21 (C = 3, 4 only); ideas/PROPOSALS.md P3 ladder (C = 4, 3, 2 proposed, C = 2 never run)"
code_needed: {files: ["research/curve25519_autolab_20261104/families.json (wsum entries with size=2)"], ecbench_oracle_string: "ic.pipeline: factor_base=prime-projected-abscissa:size=2,scan_cap=32, oracle=wsum:coef=119,free=1"}
lean_obligations: ["none new (box_count at C = 2)"]
parity_statement: "Narrows by about 13%: 3.1x -> about 2.7x at k = 32. Its own floor is 2.45/0.886 = 2.76x rho asymptotically."
```

### R3. Walk set-up: settle the P1 contradiction, then cut the jump scalars

```yaml
id: SYN-R3-cheap-walk-setup
title: "The residual is the walk's 16 two-scalar jumps at k = 20 and 24; replace them by additions"
track: generic decomposition; class: engineering
obstruction_attacked: "set-up factor 1.84 (k = 20) and 1.28 (k = 24) in Table 1"
contradiction_to_resolve: >
  Two records disagree.
  - PROPOSALS P1 attributed the residual to walk set-up.
  - Round 12 measured P1's stride stream at 1.08x the walk on ksum4-n12 at k = 24 (7.41 vs 6.86) [PCC-20261104-4afdff].
  Derived here: draw_jumps (ic_boundary.rs, near line 8803) charges 16 x (2 scalar multiplications + 1 addition).
  At ecbench's 1.5·log2(r) per multiplication that is 832 / 1024 / 1216 / 1408 GAE at k = 20 / 24 / 28 / 32.
  Against the measured residual (S - design floor) x sqrt(r) of 890 / 1162 / 678 / 2436 it fits within 4% and 10%
  at k = 20 and 24. It misses k = 28 by -46% (outside my sampling estimate of ±200 GAE) and k = 32 by +73% (inside ±700).
  The restart pool (16 more offsets at 2 multiplications each, `walk_pool_ops`) and the stride's own draws may
  explain both the misfit and round 12. Both records can hold; the test below decides which.
claim: >
  The phase counters show set-up plus walk_pool_ops at >= 80% of the residual at k = 20 and 24. A jump set built
  from one [a]G + [b]Q plus additions (e.g. J_i = J_{i-1} + [c_i]G from a small-multiple table) then lowers S by
  >= 0.55 at k = 24 and >= 1.7 at k = 20, and leaves trials per relation unchanged within 10%.
prediction:
  S_w0:  {k20: "3.31 [3.0, 3.8]", k24: "3.04 [2.9, 3.3]", k32: "2.88 (null expected)"}   # inc 5.384 / 3.678 / 2.937
  S_w01: {k24: 3.28, k32: 3.16}                                # scalars are counted group ops; unpriced unchanged
  S_w1:  {k24: 5.46, k32: 5.64}
  ratio_sel_over_inc: {k20: "0.62", k24: "0.83 / 0.84 / 0.90", k32: "0.98 / 0.98 / 0.99"}
  distance_to_rho: {k24: "2.51x / 2.49x / 2.37x", k32: "3.07x / 3.06x / 3.03x"}
discriminating_test: >
  Stage 0 (exact, no run): `tools/artifacts/artifacts.py pull` the committed session records and read the setup
  phase GAE, walk_pool_ops, walk_restarts, stride_* and scalar-multiplication counts per solve. Sessions:
  - round 12, stride arm and incumbent;
  - round 20 holdout ECBS1hbaf914946ee3 (k = 24);
  - round 16 holdout ECBS1h458afdb84a20 (k = 20);
  - round 19 holdout ECBS1hf3547017358d (k = 32).
  Stage 1: panel {inc, inc + `jumps=cheap`, rho-neg, inc-aa} at k = 20 and 24, frozen autolab rule; k = 32 as a
  no-effect check.
falsification: >
  Stage 0: set-up plus pool below 50% of the residual at k = 20 and 24 (hypothesis dead; residual stays open).
  Stage 1: paired sel/inc interval contains 1 at k = 24, or point estimate > 0.92; or trials per relation change by
  more than 10% (cheap jumps degrade probe independence); or any unverified solve.
cost_band: "inc and inc + cheap differ only in counted set-up additions; unpriced counters identical by construction; rho-neg and inc-aa priced alike at w = 0, 0.1, 1."
controls: [inc-aa, rho-neg, "k = 32 no-effect arm"]
risks: ["Jumps on one line R0 + Z·J0 are fine for a table-probe design (probes need only distinctness and known (a, b)) but must not be reused for a rho-type walk.", "Rho can adopt the same set-up: the matched reference may then improve too. The IC gain counts regardless."]
closest_baseline: "ideas/PROPOSALS.md P1 (stride variant, falsified in round 12); docs/ecbench/README.md lines 885-887 (frozen rho reference 58x theory at n = 13 from 32 full-size jump scalars)"
code_needed: {files: ["src/cryptanalysis/ic_boundary.rs: a `jumps=cheap` option for draw_jumps and draw_pool_offset", "src/cryptanalysis/ecbench/methods.rs: parameter plumbing"]}
lean_obligations: ["none (no mathematical claim; every solve is checked by ecbench)"]
parity_statement: "Narrows only at small k (k = 24: 3.0x -> 2.5x); nothing at k = 32. Composes with R1 and R2."
```

### R4. Algebraic parity budget: the falsification target for "no parity"

```yaml
id: SYN-R4-algebraic-budget
title: "Native-cost budget c*(m, k) an algebraic decomposition must meet to reach matched rho at w = 1"
track: algebraic; class: a bound card (no S change)
claim: >
  The model: N columns, arity m, linear algebra >= N^2 native operations, decomposition probability
  <= min(1, C(2N+m-1, m)/r), one group operation plus c native operations per trial. Under §5a such a method
  reaches the matched rho-neg S at w = 1 (2.301 at k = 24, 1.862 at k = 32) only if c <= c*(m, k):
  m <= 3 never; m = 4: about 36 / 88; m = 5: about 114 / 520; m = 6: about 180 / 970 (Table 2).
mechanism: "Exact counting plus the best known sparse-solver scaling (Wiedemann/Lanczos ~ N x nnz). The LA term caps N at about sqrt(X): 45 at k = 24 and 161 at k = 32."
prediction: "Every algebraic oracle in the repository measures at least 10^3 x c* per trial at its arity. Measured anchor: pkm-iso m = 2 at about 7.0e5 F4 multiply-adds per target (isogeny-seed engine table)."
discriminating_test: >
  Exact, no new run: compute native operations per decomposition trial from committed counters:
  - ECBS1h405f20219036 (pkm-iso, m = 2);
  - research/curve25519_algebraic_ladder_20261102 (m = 2, 3);
  - round 3's rootfind-pairs:m=3;
  - RESEARCH_RESIDUAL_WALKS 10.4-10.5 (S_3, S_4).
  Compare each with c*. Only if a candidate lands within 10x of c* does it get a k = 20 smoke.
falsification: "Any oracle with measured c <= c*(m, k) and decomposition rate >= half the counting bound on the k = 20/24 miniatures. That would reopen the algebraic track for a full band run."
cost_band: "No run. The c* derivation charges every phase at w = 1, the binding weight; c* at w = 0.1 is about 10x larger and is reported beside it."
controls: ["recompute c* with LA = m·N (reading only) as the optimistic bound, to show how much the conclusion rests on the solver assumption"]
risks: ["LA >= N^2 is a known-method assumption, not a theorem: an o(N^2) solver for these systems would loosen c* sharply.", "Counting bound assumes distinct m-sums; harmless here (C(2N+m-1, m) used)."]
closest_baseline: "RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md Theorem 2 (c < (m-1)/2); prime_field_leads C3 (empty window m = 3, boundary m = 4)"
code_needed: {files: ["none for the card; a native `curve25519_family c-star` printer if the Coordinator wants it tabulated per k"]}
lean_obligations: ["msum_count_le: the number of x-classes of signed m-multiset sums of N points is <= C(2N+m-1, m) (finite combinatorics). The LA bound is not a Lean claim."]
parity_statement: "States what parity would require. Nothing known meets it."
```

## 6. What would change the conclusion

Each item has a checkable criterion.

1. **A generic-track S below 0.9 x Γ(C+3/2)/Γ(C+1) at two sizes, replayed on a second host.** Read it as an
   accounting error until base-free rows, unpriced counters and replay are cleared. If it survives, the
   generic floor of `RESEARCH_RESIDUAL_WALKS.md` §10.1 is wrong, and that is the bigger finding.
2. **An oracle meeting Table 2 on the k = 20/24 miniatures**, with a decomposition rate at least half the
   counting bound. This is the only route to parity within §5a. It is also the concrete form of §10.6
   item 3: distinct combinations colliding with probability above γ/r.
3. **A sparse solver for the relation system in o(N^2) native operations.** It removes the cap on N and
   moves c* by orders of magnitude.
4. **A group action of order above 2 on the r-subgroup, computable in O(1) group operations.** It lowers the
   floor for IC and rho alike, so it does not change parity. It is excluded here by `j ∉ {0, 1728}` and by
   `STRUCTURE.md` §2(c).

The obstruction to record (I may not write `problems/`): "the generic-decomposition IC floor is
Γ(C+3/2)/Γ(C+1) sqrt(r), which is 1.5x to 2.19x matched rho for C = 1..3". It is a derived generic-group
bound with a Poisson heuristic, not a proved theorem. Exit criteria are items 1 to 3 above. Suggested
record: `problems/curve25519-family-generic-ic-floor.md`, linked from the autolab README.

## 7. Evidence, literature and limits

**Evidence read (paths):**
- `research/curve25519_autolab_20261104/README.md`
- `round-016`, `-017`, `-018`, `-019`, `-020`, `-021/README.md`
- `ideas/PROPOSALS.md`, `families.json`
- `research/notes/index-calculus/RESEARCH_RESIDUAL_WALKS.md` §10
- `research/notes/ecdlp-general/RESEARCH_ALGEBRAIC_IC_LOWER_BOUND.md` §0
- `research/notes/ecdlp-general/RESEARCH_PRIME_FIELD_LEADS_20261031.md` (C3, C4, L1, L3)
- `research/prime_field_leads_20261031/SYNTHESIS.md`
- `research/ic_arity_sweep_20261027/README.md`
- `research/curve25519_isogeny_seed_20261103/README.md`
- `research/curve25519_ic_baseline_20261102/STRUCTURE.md`
- `problems/curve25519-family-algebraic-ic-tower.md`
- `docs/ecbench/README.md` (lines 12, 198-204, 502-508, 880-926)
- AGENTS.md: success criterion, tracks, §1-6 and §5a, "IC measurement rules"
- Source: `src/cryptanalysis/ic_boundary.rs` (WSumTable around 7406-7720; draw_jumps around 8803; the guard around 8717-8729; walk set-up around 9205-9300)

**Literature** (abstract or search summary only unless stated):
- Semaev 2004 ([ePrint 2004/031](https://ia.cr/2004/031)) and 2015 ([arXiv 1504.01175](https://arxiv.org/pdf/1504.01175))
- Galbraith-Gaudry ([ePrint 2015/1022](https://eprint.iacr.org/2015/1022.pdf))
- Galbraith-Gebregiyorgis ([ePrint 2014/806](https://eprint.iacr.org/2014/806))
- Petit-Kosters-Messeng ([PKC 2016](https://link.springer.com/chapter/10.1007/978-3-662-49387-8_1))
- Amadori-Pintore-Sala ([ePrint 2017/609](https://eprint.iacr.org/2017/609))
- Kudo-Yokota-Takahashi-Yasuda, CANS 2018 ([dblp](https://dblp.uni-trier.de/rec/conf/cans/KudoYTY18))
- Hu 2024 ([ePrint 2024/1923](https://eprint.iacr.org/2024/1923))
- Galbraith-Wang-Zhang ([ePrint 2015/605](https://eprint.iacr.org/2015/605)). The PDF returned 403, so their
  numerical constants are not quoted.
- Mueller ([Caen 2018 slides](https://nitaj.users.lmno.cnrs.fr/Caen2018/Caen2018Slides/Caen2018Mueller.pdf))

A 2025 search surfaced only unverifiable items: a Jnanabha 2025 paper and an Experimental Mathematics 2025
bibliography entry. Neither is relied on.

**Not done:**
- No runs and no ecdb query: I have no execution tool.
- The lead atlas (`crypto-autoresearcher/knowledge/leads/ecdlp/`) is not on disk here, so `--match` was not run.
- Session records in the artifact store were not read, so R3's Stage 0 is unchecked.
- The c* and floor numbers are derivations, not measurements.
- No code, result, status or ledger file was edited.
