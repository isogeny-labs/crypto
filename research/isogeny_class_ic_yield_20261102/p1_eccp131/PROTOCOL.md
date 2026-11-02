# Protocol P1: index-calculus decomposition systems across the ECCp-131 isogeny class

`experiment.protocol/v1`-style design, written 2026-11-02 by the P1 builder **before any
measurement run**. The smoke run (`smoke/`) is an instrument test, not data for the claims
below; it is disclosed in section 11. Part of
`research/isogeny_class_ic_yield_20261102/DESIGN.md`. Freezing (`protocol.json`, canonical hash,
commit) is done by the experiment director / main session after review.

**Status of what this can show.** A boundary / descriptive study. Its preregistered outcome is
a **null**. A null closes a lead; it is not an improvement over the index-calculus baseline
(CLAUDE.md success criterion). `S`, end-to-end cost and speedups are **unset**. Nothing here is
compared to rho, so no matched rho reference or §5a band applies and no row may reach the IC
leaderboard or `benchmarks/README.md`.

## 1. Question

Across the 28 curves of ECCp-131's class that can be built (E0 and the 27 Elkies
l-neighbours, l <= 150), do the algebraic parameters of the decomposition system
(degree of regularity / solving degree, first fall degree, Macaulay matrix size and rank
profile, field-operation count and CPU time of the solver) and the per-trial relation yield
differ from curve to curve beyond what presentation and noise explain?

## 2. Claims and their classes

| id | claim | class | what establishes it |
|:--|:--|:--|:--|
| C1 | The 27 neighbours, their isogenies and the transported planted DLP are correct: kernel degree (l-1)/2, x-map degrees (l, l-1), homomorphism on random points, `#E' = n`, image of the generator has order n, `phi(kP) = k phi(P)` | exact computation | `neighbours.json` checks (all 27 pass), re-derived in `p1_eccp131 make-targets` |
| C2 | Full-width per-trial yield of any fixed-size factor base is the same formula on every curve of the class and is astronomically small; it is **predicted analytically, not observed** (section 3) | heuristic estimate (analytic) | formula, section 3 |
| C3 | For the PKM isogeny-tower factor base and the Semaev systems at m = 2 (t = 3, 4) and m = 3 (t = 2), the refutation/solving degree, first fall degree, rank profile and operation count of the transported-target systems on the 27 neighbours are inside the A/A band of E0 | comparative / statistical | this experiment (sections 5-9) |
| C4 | The instrument can see an algebraic change: adding a known quadric to a planted system lowers D_solve; and a planted decomposition is found with `n_standard_monomials = K` (the exact solution count) on every curve | comparative (positive control) | `ladder` and `planted` rows |
| C5 | Any statement about ECCp-131 itself at full width (p ~ 2^131, a real factor base) | out of scope | "not established" |

Preregistered prediction for C3, stated before data: **H0.** For null (inconsistent) targets the
counted metrics are structure-determined, hence **identical on every arm** (D_solve,
first fall, rank profile, rows, cols, field-multiplication count), with only `cpu_s` varying
inside the A/A band; the only quantity that varies across the class is the per-curve factor-base
size `B_c` (section 3), which is a presentation effect. **Mechanism:** the system's monomial
support and degrees depend on the family (m, t, tower) only; the curve enters through the
coefficients `a, b, x_R` of S3, and a prime-field curve of ~131 bits offers no subfield or
endomorphism to align them. All 28 curves have order n, the same group, so nothing else differs
(`research/isogeny_class_difficulty_20261008`). **H1** (what would falsify H0): some neighbour has
a different D_solve, first fall degree or rank profile than E0 on a target, outside the A/A band
and outside the random-curve population's range, reproduced on the repeat.

## 3. Full-width yield: predicted, not observed

For a factor base of `B` abscissae (`2B` points, both signs) on a group of prime order `n`, an `m`-term
signed decomposition of a uniformly random group element exists with probability

    gamma(B, m) = 2^m * C(B, m) / (n - 1)        (distinct abscissae, up to sign; collisions ignored)

`n = 0x48e1d43f293469e317f7ed728f6b8e6f1` (log2 n = 130.187) is the same for all 28 curves
(isogenous curves have equal order), so `gamma / C(B, m) = 2^m / (n - 1)` is identical across the
class: the predicted ratio of yield per factor-base subset between any two curves is exactly 1.
Observed full-width yield: none, and none is attempted; exact-curve screens found zero relations.

| quantity | value |
|:--|:--|
| `log2 gamma`, m = 2, B = 2^20 | -89.19 |
| `log2 gamma`, m = 3, B = 2^20 | -69.77 |
| `log2 gamma`, m = 2, B = 2^33 | -63.19 |
| `log2 gamma`, m = 3, B = 2^33 | -30.77 |
| `log2 gamma`, m = 2, t = 3 tower (B = 4) / t = 4 (B = 8) | -125.60 / -123.38 |

What does vary across the class is `B_c`, the number of tower abscissae `v` in `V` with `v^3 + a v + b` a
square (|V| = 2^t, fixed across arms): over E0, the 27 neighbours and the iso control it takes
values 1..7 for t = 3 and 2..13 for t = 4 (E0: 6 and 13). It is a binomial lottery of the fixed
abscissa set against each curve, not a hardness difference: a real attack fixes the number of
**points**, which removes it. Every row carries `B_c` and `predicted_yield_log2_per_trial`; the
analysis reports yield per subset (`gamma / C(B_c, m)`), never raw `gamma`, as the cross-curve
comparison. The measured systems are m <= 3 at toy tower sizes; they say nothing about
ECCp-131's real base.

## 4. Instrument

**Factor base (the repository's PKM isogeny form, `src/cryptanalysis/ic_framework/prime_tower.rs`
`prime-isogeny`).** `p - 1 = 2 * 5 * (129-bit prime)`, so the Kummer tower `g*mu_{2^t}` does not
exist for t >= 2; the isogeny form needs no condition on `p`. `p1_eccp131 build-tower` finds an
auxiliary curve `E_aux/F_p` with a rational cyclic subgroup `<Q>` of order 2^5 (10 candidates tried), a
rational point `R'`, chains t Velu 2-isogenies, and sets `V = x(R' + <Q_t>)`, `|V| = 2^t`, `t = 2..5`
(each tower asserted: `|V| = 2^t`, every `v` runs down the chain to the same `c`). Equations per base
element: `u_{j+1} * den_j(u_j) - num_j(u_j) = 0` (j < t, `u_0 = x`, `u_t = c`), quadratic in `u_j`.
The tower depends on `p` and the seed only; it is **the same factor-base family on every arm**.

**Decomposition systems (Semaev, short Weierstrass `y^2 = x^3 + a x + b`).**
`S3(x1,x2,x3) = (x1-x2)^2 x3^2 - 2[(x1+x2)(x1 x2 + a) + 2b] x3 + (x1 x2 - a)^2 - 4b(x1+x2)`.
* m = 2: tower(x1), tower(x2), `S3(x1, x2, x_R)`: 2t unknowns, 2t + 1 equations.
* m = 3: tower(x1..x3), `S3(x1, x2, w)`, `S3(w, x3, x_R)`: 3t + 1 unknowns, 3t + 2 equations (chained S3, as the repository's PKM pilot).
Configurations: **m2t3** (6 unknowns), **m3t2** (7), **m2t4** (8). m3t3 (10 unknowns) is outside the
budget (see section 10). The correctness of S3 is checked in the run: for planted instances the
point sum is computed with elliptic-curve arithmetic and the planted assignment is asserted to
zero every equation.

**Solver (disclosure, per the coordinator's note).** No honest 131-bit F_p Groebner/Dreg formulation exists in the repository: `f4_fp` stops at p < 2^32 (it panics above; corrected 2026-11-02, this text said 2^63), `pkm_tower_pilot` and `prime_tower` at p < 2^63, and the prime `ic.pipeline` has no F4 stage (pair-table MITM at m = 2|3). The formulation below is the repository's own PKM system (tower + S3) written down at 131 bits with a **new** engine; it is not repo code and not what the repo's prime IC does per attempt. Measuring the repo's `ecbench run` method `ic.pipeline` (prime_explicit, prime-abscissa, `mitm:m=2|3`) on the 28 curves is a different instrument, **not done here** (open item; the p2 builder's `p2tool` pattern applies). The repository's `f4_fp` is limited to `p < 2^32`, so it cannot run at `p ~ 2^131`. The
engine here (`rust/src/engine.rs`, `fp.rs`) is an incremental **Macaulay-matrix (XL-style) sparse
echelon** over a 3x64-bit Montgomery `F_p`: for degree D = 2, 3, ... it adds the rows `mu * f_i` with
`deg(mu f_i) = D`, reduces each against the echelon rows held (degrevlex, leading column = largest
monomial; with the tower leading terms `u_j^2`), keeps the new pivots, and records per degree: rows
(cumulative), columns `C(N+D, D)`, rank, new pivots, new pivots with leading degree < D (a degree
fall), zero reductions, non-zeros held, field multiplications, row operations, CPU time, and the
**staircase** of leading monomials (number of standard monomials `n_std`, variables with a pure power).
It is **not** F4 (no critical pairs, no symbolic preprocessing selection) and **not** a
Groebner basis engine; its D is the Macaulay degree at which the criteria below hold. On the 147 mirror systems of
`CROSSCHECK.md` (primes < 2^32, where `f4_fp` runs) it was strictly above F4's top step degree every time (an observation on
these shapes; no theorem, degree falls make the two incomparable). No claim about F4 time is made. Cross-checked against `f4_fp`
(`CROSSCHECK.md`, `crosscheck/`), with the limits listed there and in section 12.

**Metrics (all defined per attempt; counted metrics first, the clock last).**

| metric | definition |
|:--|:--|
| `D_solve` | first D at which `n_std = K` (K = exact number of solutions over the closure, enumerated from V) and every variable has a pure power as a leading monomial; for `K = 0` it is the **refutation degree**: first D with the constant 1 as a leading monomial |
| `first_fall_degree_obs` | first D at which a new pivot has leading degree < D |
| `D_semiregular_ref` | first non-positive coefficient of `prod(1 - z^{d_i}) / (1 - z)^N` (reference only) |
| `final_rows`, `final_cols`, `final_rank`, `rank_profile_by_leading_degree` | Macaulay matrix size and rank at D_solve |
| `field_mults`, `row_ops` | counted `F_p` multiplications (all, including the reduction and the pivot inversions) and pivot applications |
| `cpu_s` | thread CPU time (`CLOCK_THREAD_CPUTIME_ID`) of the engine call; `wall_s` also recorded |
| `K_exact_solutions` | exact solution count (m = 2: `S3` on `V x V`; m = 3: common roots of the two quadratics in `w` over `V^3`) |
| `B_c`, `predicted_yield_log2_per_trial` | section 3 |

An attempt that reaches the degree / column / time cap is recorded with its status
(`cap_degree`, `cap_cols`, `cap_time`) and **kept** as a censored row.

## 5. Arms (same instrument, budgets, seeds policy and stopping rule on every arm)

| arm | n | what | role |
|:--|--:|:--|:--|
| `E0` | 1 | ECCp-131, `icv1-fp131-t29529377007934231835-9681b699` | reference |
| `n{l}_{0,1}` | 27 | the 27 Elkies neighbours (l = 5, 7, 13, 17, 29, 37, 61, 73, 83, 89, 107, 131, 149: two each; l = 67: one), ICV1-style slugs in `neighbours.json` | treatment |
| `iso` | 1 | isomorphic model of E0, `(a u^4, b u^6)` | model control (expected = E0 exactly) |
| `AA_repeat` | 1 | the E0 instance again, a separate run slot | A/A: determinism and clock noise floor |
| `AA_perm` | 1 | the E0 instance with a SHA-256-seeded permutation of the variable order | A/A: re-encoding noise floor (it moves the leading terms and the operation count) |
| `rnd0..rnd26` | 27 | random `(a, b)` over the same `p` (the closest valid null: same field, same system shape; the order need not be prime because only `a, b, x_R` enter) | negative control population |
| `E0` + `ladder` | | planted E0 instance plus one random quadric vanishing at the planted point | positive control (instrument sensitivity) |

Optional, off by default: `--special N` adds `a = 0` (j = 0) and `b = 0` (j = 1728) arms as an
exploratory coefficient-structure probe. A single build-time probe (2 targets) gave counts identical
to E0; they are **not** a positive control.

Target kinds: `null` (transported target, no decomposition exists at this width: the system is
inconsistent, `K = 0`), `planted` (per-curve decomposition built from two/three factor-base
points, `K >= 1`), `ladder` (planted plus extra quadric; E0 and `AA_repeat` only). Null targets are
**paired across arms**: target `R_tau = k_tau P0` on E0, its image under the verified isogeny (the
u-scaling for `iso`, identity for E0 and the A/A arms) is the abscissa used on each neighbour; target 0
is the planted-DLP point `Q = kP`. Random-curve arms draw their own random point per target (not
paired). Planted instances are per-curve (factor-base points differ per curve), so they are not paired
across curves; `AA_repeat` and `AA_perm` use E0's planted instance.

## 6. Seeds, order, ledger

* Master seed 20261102. Every random choice is `SHA-256("{seed}|{label}")`: targets
  (`p1_eccp131 make-targets`: `k_tau = SHA256(seed|target-k|tau) mod (n-2) + 1`), planted points and signs,
  random curves, the A/A permutation, ladder coefficients, schedule order. Nothing is chosen
  after a look.
* **Order:** for every slot `(rep, config, kind, tau)` the arms are shuffled by a derived seed
  (interleaved), so no arm runs systematically first. Slots run in a fixed order; arms interleave
  within each slot.

| factor | treatment |
|:--|:--|
| instrument (commit, `rustc -Vv`, binary SHA-256, threads) | fixed; binary hash in the run header |
| host (CPU, governor, co-tenants) | measured (header `cpu`, `nproc`; isolab tier recorded by the executor); clock metrics are claimed only against the same-run A/A band |
| order / warm-up | randomized within slot; reps >= 2 for timing configs |
| inputs and seeds | derived, listed above |
| memory layout | fresh engine state per attempt; threads share the process |
| spot preemption | counts are deterministic and survive a restart; a preempted row loses its clock (`wall_valid` false by the executor); the job is idempotent per attempt, restart re-runs the file from scratch |
| analyst | arm labels masked in the analysis (`--mask`: neighbour and random-curve names replaced by hashes; the key is written next to the output, not printed; E0 and the A/A arms stay labelled); key opened only after the analysis output is frozen |

## 7. Sample sizes

| config | arms | null targets | planted targets | reps | attempts (about) |
|:--|--:|--:|--:|--:|--:|
| m2t3 | 58 | 32 | 8 | 3 | 5,600 + planted 1,400 + ladder 48 |
| m3t2 | 58 | 32 | 8 (arms with >= 3 factor-base abscissae; others logged as not formed) | 3 | 5,600 + planted ~700 + ladder 48 |
| m2t4 | 39 (E0, 2 A/A, iso, 27, 8 random) | 8 | 2 | 1 | 312 + 78 |

Counts are deterministic: repeating them is a determinism check, not new information; the reps buy
the clock. Resolution for the counted metrics: a single target with a different `D_solve` is visible
(1 of 32 pairs), and `identical_count_profile_fraction` is reported per arm. For the clock the
paired log-ratio over 32 targets has a standard error of about CV_paired / sqrt(32); the smoke run's
A/A ratios were within 0.88-1.15 on two targets, so the full-run band will be set by the A/A arms
themselves, not by a pre-set number. No permutation test is used, so the `4m/alpha` rule does not
apply; decisions use Bonferroni-level bootstrap intervals (section 8).

## 8. Analysis (script `rust/src/analyze.rs`, subcommand `analyze`; path and SHA-256 are recorded when frozen)

1. **Paired ratios vs E0.** For each arm and each metric (`cpu_s`, `field_mults`, `final_rows`,
   `final_cols`, `final_rank`, `D_solve`, `first_fall_degree_obs`, `n_standard_monomials_final`),
   the geometric mean of the per-target ratio `arm / E0` with a percentile bootstrap over
   targets. Level 0.9995 with 40,000 resamples (Bonferroni over 27 neighbours x 4 primary
   metrics = 108 tests at family alpha 0.05).
2. **Exact agreement.** `identical_count_profile_fraction` per arm: the share of targets whose
   `(D_solve, final_rank, final_rows, final_cols, first_fall, rank profile, status)` equal E0's.
3. **Curve variance share (ICC).** Crossed two-way random-effects ANOVA (curves x targets), the
   share `sigma_c^2 / (sigma_c^2 + sigma_t^2 + sigma_e^2)` (negative components clipped to 0),
   computed for three sets: E0 + 27 neighbours; the A/A set (E0, `AA_repeat`, `AA_perm`);
   the 27 random curves. Bootstrap CI by target resampling. Undefined (`all_values_equal`) for a
   metric with zero variance, reported as such, not as 0.
4. **Predicted yield.** `gamma / C(B_c, m)` per curve versus the formula's constant; `B_c` reported.
5. **Positive control.** `ladder` vs `planted` on E0: `D_solve` must drop by at least 1 on every
   planted target, and every planted row must have `n_std = K`, `planted_verified = true`.

**Decision rule.** The class effect is *detected* for a metric iff some neighbour's paired interval
excludes 1 **and** lies outside the interval of `AA_perm` (the re-encoding floor) **and** outside the
range of the random-curve arms **and** the same neighbour repeats on the `rep` replicates. A
difference in an exact count (`D_solve`, rank profile) on any target of any neighbour, absent from
the A/A arms, is reported as an anomaly with its target and arm, even when the intervals
include 1. Otherwise: **null** (H0 stands). A null at this resolution says nothing about full width
(C5).

## 9. Stopping rule

Fixed `N` (section 7). No interim looks, no change of `N`, metric, null or threshold after data;
any change is a new labelled protocol version (an amendment saying whether it preceded the data it
affects), both reported. A config where more than 5% of null attempts end `cap_time` is declared
unresolved for that config (reported, not dropped). The run ends when the schedule is exhausted.

## 10. Cost and sizing (smoke measurements, single thread, this 4-core x86 sandbox)

CPU time per attempt (thread CPU): m2t3 null 0.38 s, planted 0.17 s, ladder 0.03 s; m3t2 null
1.74 s, planted 1.76 s, ladder 0.31 s; m2t4 null 67 s, planted 24 s (probe, E0 only);
m3t3 planted E0: see `smoke/m3t3_probe.txt` (outside the budget).

| tier | CPU-hours (estimate) | wall on 4 workers |
|:--|--:|--:|
| tier 1: m2t3 (2,370 s) + m3t2 (10,350 s), 58 arms, reps 3 | 3.5 | about 0.9 h |
| tier 2: m2t4, 39 arms, 8+2 targets (20,900 s null + 1,870 s planted) | 6.3 | about 1.6 h |
| analysis | negligible | seconds |
| neighbour build (once, done) | 0.05 (181 s wall, PARI) | n/a |

Memory is small: the m2t4 null attempt holds about 20 MB of echelon rows plus the monomial index (126k columns); four workers fit easily in 15 GB. The wall column assumes four free cores; `available_parallelism` in this sandbox reports 3, so the director should check the pod's CPU quota.

## 11. Disclosure of pre-freeze observations

The builder looked at the following before writing this protocol, all at 2 targets or fewer: (a) the
smoke run (`smoke/`, 2 neighbours `n5_0`, `n149_1`, 2 targets, A/A and random arms): every counted
metric (D_solve, rank, rows, cols, first fall, field multiplications) was **identical on every
arm for null targets**, the variable-order arm changed only the operation count (ratio 1.165 /
0.976) and the clock; (b) the special-coefficient probe (identical counts); (c) E0 / `n5_0` m2t3
probes. H0 above was therefore stated knowing that the counted metrics look structure-determined
at this size; it remains a prediction about the other 26 neighbours, 32 targets and the larger
configurations.

## 12. What this does not establish; open items

* No claim at full width: the systems are toy towers (|V| <= 16), m <= 3; D_solve at toy size
  does not extrapolate.
* The engine is not F4. Its correctness rests on `selftest`, the per-row planted-point check, `n_std = K` on every solved
  planted row, the independent enumeration of K, and the follow-up cross-check (`CROSSCHECK.md`, 2026-11-02): on 252 mirror
  systems at primes 2^14..2^61-1 no disagreement with an independent dense Macaulay elimination (688 degree profiles), the exact
  solution enumeration, or `f4_fp` (solution sets, quotient dimension, inconsistency; 147 systems, p < 2^32 only: `f4_fp` panics
  at p >= 2^32). Not done: F4 above 2^32 (`f4_tower_generic` not wired), a Magma/msolve comparison (AGENTS.md §15).
* m3t3 (10 unknowns), m = 4 and larger towers are out of budget.
* Time is a secondary metric; every neighbour's isogeny cost (phi evaluation, build time per
  neighbour in `neighbours.json`, 0.00-9.2 s) is a **transport cost**, reported separately and not
  priced into any yield or solve cost.
* The 27 neighbour slugs are not yet in `docs/curves/registry.json` (AGENTS.md §11: register
  before citing; `python3 scripts/build_curve_registry.py`).
* The three construction steps are Rust subcommands of the `rust/` crate (`build-neighbours`, `build-tower`, `make-targets`)
  that drive libpari 2.17 through dlopen (`rust/src/pari.rs`, the mechanism of `p2_prime_analogues/p2tool/src/pari.rs`; the GP
  source is `rust/gp/construct.gp`); no Rust SEA / modular-polynomial isogeny code exists here. They build and verify inputs
  only. They replaced `build_neighbours.py`, `build_tower.py`, `make_targets.py` (deleted); the committed `neighbours.json`,
  `tower.json`, `targets.json` were produced by those scripts and the Rust port reproduces them (table in `CROSSCHECK.md`, section A:
  `targets.json` byte-identical; `neighbours.json` and `tower.json` identical to the byte except the wall-clock timing fields).
  Measurement, analysis and all result tables are Rust, per AGENTS.md "no Python".

## 13. Files and commands

| file | role |
|:--|:--|
| `p1_eccp131 build-neighbours` | the 27 neighbours, checks, planted DLP, per-neighbour build times -> `neighbours.json`, `build_neighbours.log` |
| `p1_eccp131 build-tower` | PKM isogeny towers t = 2..5 -> `tower.json` |
| `p1_eccp131 make-targets` | 32 seeded targets transported to 31 arms -> `targets.json` |
| `rust/` | `p1_eccp131 selftest \| run \| analyze \| build-neighbours \| build-tower \| make-targets \| jsoncmp`; GP source `rust/gp/construct.gp` |
| `crosscheck/` | `p1_crosscheck`: the engine against the repository's `f4_fp` (see `CROSSCHECK.md`) |
| `run_full.sh` | `tier1`, `tier2`, `analyze` orchestration |
| `smoke/` | smoke rows, log, analysis |

Rebuild inputs (needs the libpari 2.17 shared object, e.g. the `cypari2.libs/libpari-*.so.2.17.2` of any venv that has cypari2;
about 3 min for the first, seconds for the rest; `BIN` is the built binary, see below):

    export P1_LIBPARI=/path/to/libpari-....so.2.17.2        # or pass --libpari
    $BIN build-neighbours --out neighbours.json
    $BIN build-tower --out tower.json --tmax 5
    $BIN make-targets --nb neighbours.json --ntargets 32 --out targets.json
    $BIN jsoncmp OLD.json NEW.json --skip timing_s,wall_s,build_total_s_sum   # canonical comparison

`build-tower` note: PARI's `ellgroup(E,,1)` picks its generator from the PARI random state, so the tower is reproduced only when
no other randomised PARI call (`ellcard`, `isprime`) runs before it; `build-tower` therefore sets only `p` before the search.

Build and test the instrument:

    cd rust && cargo build --release --locked && cargo test --release && ./target/release/p1_eccp131 selftest

Full run (tier 1 and 2 as frozen in `run_full.sh`; route and host chosen by the director,
isolab-eks default, 4 vCPU):

    ./run_full.sh tier1
    ./run_full.sh tier2
    ./run_full.sh analyze      # masked, 99.95% intervals, 40,000 resamples

Outputs: `runs/tier{1,2}_rows.jsonl` (one JSON row per attempt plus a header with seed, input
hashes, binary hash, host), `runs/tier{1,2}_analysis.json`, `.txt`, and the mask key
(`*.key.json`, opened after the analysis is frozen). Export to the unified
`experiment.export/v1` is the director's step.
