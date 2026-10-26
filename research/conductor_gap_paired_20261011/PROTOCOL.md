> **PROVISIONAL, legacy Python, NOT FOR MERGE: violates the repo's no-Python rule; kept as labelled historical evidence; matched native re-measurement required before any claim.**

# Protocol: paired conductor-gap smoke test over F_{p^2}, gates G0 and G1

Frozen before any code in this directory was run. Directory name carries the
dispatch date (2026-10-11); this protocol was written on 2026-10-26 against
base commit `bbd330a9d` (main of 2026-10-11, about 15 days behind the date of
writing). Result class: **feasibility smoke test and boundary control**. No
ECDLP speedup is claimed or expected, no security statement is made, nothing
here is extrapolated beyond the toy fields below.

## 0. Sources, prior work, and what is new

- Protocol being executed: `docs/experiments/conductor-gap-paired-ecdlp.md` on
  branch `experiment/conductor-paired-cost-20261009` of
  `aburan28/crypto-autoresearcher` (open PR #2041, read-only here). Its gates
  are quoted in section 8.
- Methodology reused: the volcano-level method of
  `research/ecc2k130_conductor_isogenies_20261007/` in the autoresearcher repo
  (`check_levels.gp`, `structure.gp`, its README): `f_pi` from `t^2 - 4q`, the
  l-isogeny volcano of depth `v_l(f_pi)`, a vertex is on the floor iff it has
  exactly one rational l-isogeny. That directory is PARI/GP; PARI is not
  installed here (`apt-get install pari-gp`: "Unable to locate package"), so the
  method is re-implemented, not called.
- **Prior work in this repository (found by `git log`/`git show` on
  `bbd330a9d`).** `research/isogeny_conductor_gap_ic_20261010/` (commits
  `46a845e0d`, `2ab74c68d`, `152a6e653`, `9d1d20c63`, merged with #1647) already
  measures index calculus across a conductor gap with Rust/`ecbench`: 38 prime
  and 8 binary/Koblitz crater-floor pairs, endpoints only (`f_E = 1` against
  `f_E = f_pi`), result flat (yield, `S`, F4 solving degree unchanged within
  run-to-run spread). Its protocol lists **extension fields `F_{p^k}` as "not
  attempted"** and its pairs jump from `End = O_K` to `End = Z[pi]`. The
  frozen protocols I-4 and I-5 in `research/isogeny_class_difficulty_20261008/`
  are the parent hypotheses. This directory does not repeat that round. It adds
  only what that round left open: (i) the extension-field Gaudry-Diem regime
  (Weil-restricted `S_3` over `F_{p^2}`), (ii) intermediate conductors where the
  class has them, (iii) `f_E` certified by two independent methods, (iv) the
  per-attempt raw-receipt, paired-bootstrap, permutation-test accounting of
  autoresearcher PR #2041.
- Rust engines in this repository that were **not** used:
  `src/cryptanalysis/gaudry_cubic.rs` (E over `F_{p^3}`, random prime-order
  curves, so no matched-trace pairs) and `semaev_decomp.rs` (binary `S_4`).
  Reason: the brief caps Rust exploration at about 30 minutes and prefers a
  bounded Python path that produces receipts; neither takes an explicit
  `(a, b)` pair over `F_{p^2}`. Integrating one is part of the next experiment.

## 1. Conflict with the repository's language rule, stated openly

`AGENTS.md` ("Implementation language: no Python") forbids Python for
cryptographic research harnesses and says a working Python harness is not an
acceptable final deliverable. The brief for this task and the coordinator's
follow-up both say explicitly that pure Python is acceptable and preferred for
this smoke test. This directory therefore follows the task instruction and is
labelled a **Python prototype under a task-level exception, not a final
harness**. Consequences, frozen now:

1. Wall/CPU timings are Python-interpreter timings and are labelled "historical
   Python timings". They are a practicality note, never a speed claim.
2. Counted operations (field multiplications in the engines, Groebner reductions)
   are recorded next to every timing, because they are implementation-neutral.
3. Any claim beyond this smoke test needs a native (Rust) replacement of the
   engines and a re-run, per `AGENTS.md`. That is the "next experiment".

## 2. Hypothesis and null

H1 (autoresearcher PR #2041): ordinary curves `E1, E2` over the same `F_q`, same
trace `t`, same `N = #E(F_q)`, with certified `f_E1 != f_E2`, differ
reproducibly in algebraic relation-solving cost.

Null H0: the paired log-cost difference has mean 0 and is indistinguishable from
the isomorphic-model control (same curve, scaled model) and from a
same-conductor control. The expected outcome, by construction of the engines
(the system's shape is identical for every `(a, b)`), is H0; a negative or null
result is a valid result. Notation: `f_pi` is the conductor of `Z[pi]`
(`t^2 - 4q = f_pi^2 D_K`), `f_E` the conductor of `End(E)`, `f_E | f_pi`. They
are never conflated; `f_E` is always certified separately.

## 3. Selection (script `enumerate_pairs.py`)

**Fields.** `q = p^2`, `p` prime in `{19, 23, 29, 31, 37, 41, 43}`.
`F_q = F_p[theta]/(theta^2 - n)`, `n` the least quadratic non-residue mod `p`;
an element `a + b*theta` is stored as the integer `a + p*b`. `F_p` is `b = 0`.
`F_{p^3}` is **not attempted** (reason: the `S_4` three-unknown engine is a
separate implementation; deferred to the next experiment).

**Curves.** Every `F_q`-isomorphism class of `y^2 = x^3 + a x + b` (all `j`, with
both quadratic twists, plus the 6 twists of `j = 0` and the 4 of `j = 1728`:
`2q + 6` classes). Point counts by a vectorised character sum; the total
class count and the per-trace counts are checked against theory (section 4).

**Grouping.** By `(q, t, N, r)`, `r` the largest prime factor of `N`.
A group is **eligible** iff: ordinary (`p` does not divide `t`); `f_pi > 1`; all
prime factors `l` of `f_pi` lie in `{2, 3, 5, 7}` (the supported set of the
volcano code, which needs `2` to generate `(Z/l)^* / +-1`; true for these);
`r >= 4p` and `r^2` does not divide `N` (so the relation rank over `F_r` is not
capped by `r`). Ineligible groups are recorded with the reason, never dropped.
Curves with `j` in `F_p` are **excluded from trials** (flagged
`j_in_Fp`): they carry the extra `p`-Frobenius structure and would confound the
Weil-restriction comparison. They stay in `curves.jsonl`.

**Selection run.** Groups are processed in ascending estimated cost
(`#curves x deg psi_l`). Every curve of a processed group is certified
(section 4). Stop when 6 distinct `(q, t)` groups are fully certified, or after
20 CPU-minutes (whichever first); each curve has a 90 s cap, recorded as
`timeout` on expiry. A **negative selection receipt** is emitted if fewer than
2 groups yield a certified pair; unverified curves are never substituted.

**Pair rule (deterministic).** Within a fully certified group, take the two
trial-eligible curves (`j` not in `F_p`) with distinct certified `f_E` that
minimise `abs(|FB1| - |FB2|)`, ties broken by smaller integer `j`. One pair per
group. Trial pairs: groups ordered by `p` descending, first 4, with distinct
`(q, t)`. Record conductor ratio, the prime factors of the index
`f_E2 / f_E1`, both `j`, twist classes, and an l-isogeny path (BFS in the
l-graph of the differing prime, depth <= 3) when one is found.

**Controls per pair (all recorded in the same files).**
(C-iso) the isomorphic model `(u^4 a, u^6 b)` of the lower-conductor curve with
`u = theta` (so `u^2 = n` lies in `F_p`, the factor base maps bijectively): a
null control, same relations, only timing noise can differ;
(C-same) a same-conductor control, a third curve of the same group with
`f_E` equal to the lower-conductor curve's and a different `j`, when the group
has one. If none exists it is reported as absent, not replaced.

## 4. Certifying `f_E` (two independent methods, both recorded)

For each ordinary curve with `f_pi > 1`:

**Method A, volcano (rational l-isogenies).** For each prime `l | f_pi`, `l` odd:
`psi_l` (division polynomial) is factored over `F_q` (distinct-degree then
Cantor-Zassenhaus); rational kernel polynomials are the degree-`(l-1)/2`
products of irreducible factors that are stable under the doubling map
`x -> x(2P)` (for `l` with `2` generating `(Z/l)^*/+-1` a stable set of that size
is exactly one cyclic subgroup, so this count is exact); `l = 2`: rational roots
of `x^3 + a x + b`. Vertex on the `l`-floor iff exactly **1** rational kernel;
otherwise `l + 1`. Images by Velu (power sums of the kernel roots), each image
checked to have `#E' = N`. Height `h_l` = BFS distance (over `j`) to the nearest
floor vertex; then `v_l(f_E) = v_l(f_pi) - h_l`. Every kernel polynomial, image
and BFS edge is written to `isogeny_edges.jsonl`.

**Method B, group structure (the module identity).** `E(F_q) ~ End(E)/(pi - 1)`
as `End(E)`-modules. With `pi = x + f_pi * omega_K`,
`x = (t - f_pi D_K)/2`, `E(F_q) ~ Z/n1 x Z/n2` has `n1 = gcd(x - 1, f_pi/f_E)`.
`n1` is computed by brute force from the group (all points, exact exponent) and
compared with the prediction for the Method-A conductor, for the curve and its
quadratic twist (`x -> -x`). This is partial: it constrains only the primes `l`
with `l | x -+ 1`; the record states which primes it decided and which it did not.

**Group-level check.** Over all curves of a group, the number of classes must
equal the Kronecker class number `H(t^2 - 4q) = sum over f | f_pi of h(f^2 D_K)`
(Schoof), and the observed distribution of certified `f_E` must equal
`{f : h(f^2 D_K)}`. Class numbers are counted from reduced forms. A mismatch
invalidates the group.

A curve is **certified** iff Method A gives `f_E` and Method B is not
contradicted. A pair is **independently verified** iff both curves are certified,
`f_E1 != f_E2`, the group-level check holds, and for at least one prime `l` of the
index Method B (curve, twist, or the exponent identity) agrees on which curve has
the larger `l`-part or the group-level check separates them; otherwise the pair
is reported as "certified by A only" and does **not** count toward G0.

## 5. Paired trials (script `paired_trials.py`)

**Task.** Decompose random points into two factor-base points over `F_{p^2}`
(Gaudry-Diem, `n = 2`): factor base `F = {P in E(F_q) : x(P) in F_p}`; target
`R = a_j G + b_j H` with `G` of prime order `r` (`G = [h]P0`, `h = N/r`), `H = [d]G`;
find `R = e1 P1 + e2 P2`, `e_i = +-1`, `P_i` in `F`.

**Identical across the two curves of a comparison:** factor-base cardinality `M`
(= the smaller of the two full bases; the base is the first `M` x-classes in
increasing integer order, the larger base is truncated, and the discarded
solutions are counted as `outside_base`), arity (`S_3`, `n = 2` unknowns over
`F_p`), seed, the scalars `d`, `(a_j, b_j)`, number of attempts `T = 24`, engine
settings, monomial orders, and budgets. No per-run wall budget is needed
(a run is 24 attempts); a hard `timeout 900` wraps every script invocation.

**Engines (both on every run).**
1. `exhaustive`: all unordered pairs of the base, `S_3(x_i, x_j, x(R)) = 0` in
   `F_q` with the `x`-pair constants precomputed in setup. Exact baseline yield.
2. `weil_gb`: `S_3(x_1, x_2, X_3)`, `X_3 = x(R)`, symmetrised with
   `e1 = x1 + x2`, `e2 = x1 x2`, Weil-restricted: two quadrics `f0, f1` in
   `F_p[e1, e2]` (one coefficient per `F_p`-coordinate of `F_q`). A Buchberger
   Groebner basis (pure Python; **not F4/F5, not SAT**) in grevlex records the
   observed solving degree (largest S-polynomial degree processed), the largest
   basis degree, the quotient dimension and operation counts; a lex basis
   gives the univariate eliminant whose `F_p`-roots give `(e1, e2)`, then
   `x1, x2` from `z^2 - e1 z + e2`.
Both engines must return the same in-base solution set on every target; any
disagreement is logged and fails the run.

**Verification of every relation (exact).** `P_i` on the curve with `x_i` in
`F_p`; the explicit sign choice with `e1 P1 + e2 P2 = R` holds by the affine
group law; the discrete-log table of `<G>` (`r` entries) confirms
`sum e_i log(h*P_i) == h (a_j + b_j d) (mod r)`.
**Rank.** Rows over `F_r`: columns = distinct `[h]P` classes up to sign, plus the
`G` and `H` columns. "Newly independent" = rank increase of the incremental
echelon form. Duplicate and dependent rows are counted. A solution with `x_i` in
`F_p` but `y_i` outside `F_q`, or a non-zero-dimensional system, is an *invalid*
solution, counted and charged, never a relation.

**Per run (curve, seed, engine) the receipt in `runs.jsonl` holds:** curve id,
seed, engine, order position, `M`, `r`, `h`, per-phase CPU and wall seconds
(`setup`, `target_generation`, `decompose`, `verify`, `linear_algebra`,
`verification_setup`), their total, relations found, invalid, duplicate,
dependent, out-of-base, failed attempts (no solution), rank increments, system
dimensions (variables, equations, quotient dimension), observed maximum
degree, operation counts, and a SHA-256 fingerprint of the relation set.

**Order.** For each comparison and seed the order of the two curves is a coin
flip from `random.Random(seed ^ 0x5eed)`, recorded. Seeds `1..12`. Comparisons:
pair, C-iso, C-same (if present), per trial pair, per engine. One compute
process at a time. Environment (Python, platform, CPU model, git revisions,
exact command) is written to `environment.json`.

## 6. Endpoints and analysis

**Primary:** `y = ln(total CPU seconds / rank increments)` per run, all attempts
and all setup/verification charged; primary engine `weil_gb`. Paired difference
per `(pair, seed)`: `y(larger f_E) - y(smaller f_E)` (for C-iso: model minus
original; for C-same: second minus first by integer `j`). CPU seconds
(`process_time`) are primary because the host is shared; wall seconds are the
sensitivity analysis. Runs with zero rank increments are censored: reported by
count, excluded from the primary mean, with the censored fraction stated.

**Secondary:** relation yield per attempt, rank increments, observed solving
degree, counted operations per independent relation, `exhaustive` engine cost,
failed searches, invalid/duplicate/dependent counts.

**Inference:** per comparison, paired bootstrap (10 000 resamples of seeds) 95%
percentile CI of the mean difference; sign-flip permutation test (20 000
permutations, two-sided) on the seed-level differences; Holm correction across
the scanned pairs, separately per engine; the pooled estimate is a stratified
bootstrap with pairs as fixed strata. Pair effect is also read against the
C-iso control's CI (timing noise floor).

**Decision rules, frozen.** "Reproducible difference" requires: the pair's
primary CI excludes 0, the Holm-adjusted permutation `p < 0.05`, the sign is the
same in at least 11 of 12 seeds, and the pair effect exceeds the C-iso control's
CI half-width and the C-same control's difference. Anything else is reported as
"no reproducible difference detected at this size and power".

## 7. Reproducibility replay (G1)

After the main run, the first 3 seeds of every comparison are re-run in a new
process. G1 requires: identical relation-set fingerprints on every replayed run,
and the replayed primary-endpoint values reported next to the originals
(timing reproducibility is reported, not required to be identical).

## 8. Gates (from PR #2041, reported pass/fail explicitly)

- **G0**: 2 independently verified pairs with different endomorphism
  conductors (distinct `(q, t)` groups, section 4 for "independently verified").
- **G1**: reproducible F4/F5 or SAT runs with raw receipts and exact relation
  verification. **Declared deviation, before the run:** the algebraic engine is
  a pure-Python Buchberger, not F4/F5/SAT. G1 is reported as "met for the engines
  used" or "not met", and in either case "not met as literally worded
  (F4/F5/SAT)".
- **G2**: >= 10 paired seeds per pair and a matched same-conductor control.
  Executed only because the runs are cheap (12 seeds); reported as a count
  criterion at toy scale.
- **G3**: larger replication. **Not attempted.**

## 9. Budgets and stop rules

Total compute under about 90 CPU-minutes, one compute process at a time, every
run under `timeout`. Selection <= 20 CPU-min; trials <= 30 CPU-min (expected
far less); tests and plots the rest. If selection finds no pair, the trial
stage is skipped and the negative receipt is the deliverable. No result is
chosen by seed or by pair after seeing outcomes; pairs follow the rule in
section 3. Raw outputs are never overwritten: a second run goes to a new file.

## 10. Claim hygiene

Public synthetic toy curves over `F_{p^2}`, `p <= 43`. No deployed-curve claim,
no key recovery, no extrapolation to cryptographic sizes, no asymptotic
statement, no "ECDLP bit discount" from a conductor. Curve ids here are local
(`cgp-...`), not registered ICV1 slugs; nothing is added to the curve registry,
scoreboard or ledger (reason: not an `ecbench` session, no `S` column, no
replay certificate). `identity.certificate/v1`: no new algebraic identity is
claimed; `S_3` and the module identity are standard and are checked numerically
by the unit tests.

## Amendments

Each amendment is committed before the run it affects.

### Amendment 1 (2026-10-26, before the selection run)

Context: after freezing section 3, the certification code was developed and
tested on every supported group with `p <= 23` (981 curves, about 30 ms per
curve, 0 failures, and for 55 groups at `p = 11..19` the certified `f_E`
distribution equalled the class-number prediction). Those were code tests, not
the selection run; the same checks are in `tests/`.

1. **Selection scope (replaces "stop when 6 distinct groups are certified").**
   Certification costs far less than the 20 CPU-minute cap, and ordering by
   estimated cost would bias the six groups toward the smallest field. The
   selection run therefore certifies **every eligible group with `t > 0`** for all
   seven fields, subject to the unchanged 20 CPU-minute total and 90 s per-curve
   caps (groups not reached are recorded as `not_reached`). Groups with `t < 0`
   are the quadratic-twist mirrors of `t > 0` groups (same `f_E`, same class
   count); their class counts are checked, but they are not separately certified
   and no trial pair uses them. The pair rule and the trial-pair rule
   (section 3) are unchanged and are applied to the certified groups.
2. **"Independently verified" made operational (a clarification of section 4,
   not a change).** A pair is independently verified iff both curves are
   certified, `f_E1 != f_E2`, the group-level class-number check holds for their
   group, and there is a prime `l` at which Method B is **decisive for both
   curves** (the exact `n1` data of the curve and its twist leave exactly one
   `v_l(f_E)`), Method B's two values differ, and their order agrees with
   Method A. A pair certified by Method A alone is reported but does not count
   toward G0.
3. **Edge file granularity.** `isogeny_edges.jsonl` stores every edge expanded
   during the walks of the trial-pair groups, and for all other certified groups
   the start-vertex edges and the descending path edges, de-duplicated by
   `(l, from j, kernel polynomial)`; counts of all expanded edges are in the
   receipt. Reason: the full edge set would be tens of megabytes.
4. **`curves.jsonl` scope.** Every curve of every group with `f_pi > 1` that was
   processed or skipped (with its reason). Curves with `f_pi = 1`, supersingular
   curves and `t < 0` mirrors are counted per field in the receipt, not listed.
5. **Pair rule applied among independently verified candidates.** The
   deterministic pair rule of section 3 is applied to the independently verified
   candidate pairs of a group (item 2). A group whose only distinct-conductor
   pairs are certified by Method A alone is reported as such and takes no trial
   slot. Fields are processed in descending `p` so that, if the time cap bites,
   the fields the trial-pair rule prefers are reached first.

### Amendment 2 (2026-10-26, after the selection run, before any paired trial)

The selection run is complete (`selection_receipt.json`; 51 independently
verified pairs, 4 trial pairs by the frozen rule, all in `p = 43`). Before any
trial ran:

1. **Permutation test is exact.** With 12 seeds the sign-flip null has only
   `2^12 = 4096` distinct outcomes, so the test enumerates all of them (two-sided,
   statistic `|mean|`) instead of sampling 20 000 random flips. Same null
   distribution, no Monte-Carlo error. `analysis.json` states the method.
2. **How `G` is obtained.** Non-model runs sample `P0` inside the timed setup
   from `random.Random("<seed>:G")` (rejection on `x` until `x^3 + a x + b` is a
   square and `[h]P0 != O`), then `G = [h]P0`, `H = [d]G`, `d` from
   `random.Random("<seed>:S")`, which also yields the attempt scalars
   `(a_j, b_j)`; both curves of a comparison therefore see the same `d` and
   scalars. For the C-iso model run `P0` is the image `(u^2 x, u^3 y)` of the
   original curve's `P0` (regenerated outside the timer), so the model's factor
   base, `G`, `H`, targets and relations are the exact images of the original's.
   The timed difference between sampling and mapping is microseconds.
3. **Receipts are per attempt.** `runs.jsonl` holds, per run, one `run_setup`
   line, 24 `attempt` lines (target `R`, outcome, system size, observed degree,
   per-phase CPU and wall seconds, relations, rank increment) and one
   `run_summary` line. CPU time is `time.process_time`, wall time
   `time.perf_counter`, both read at every phase boundary; the garbage collector
   is disabled during a run. Peak RSS (`ru_maxrss`, process-cumulative) is logged
   after each run. Run total = whole-function CPU, so loop overhead is charged.
4. **Engines are separate run records** with identical random streams. After the
   main run, for every `(curve, seed, M)` the relation-set fingerprints of the
   two engines must be equal (in-base solutions are a property of the curve,
   seed and `M`, not of the engine); a mismatch fails G1.
5. **Unit order.** All `(comparison, seed, engine)` units are shuffled with
   `random.Random(20261026)`; inside a unit the order of the two curves is a coin
   flip from `random.Random(seed ^ 0x5eed ^ (comparison_index * 7919) ^ engine_bit)`
   (section 5 gave only `seed ^ 0x5eed`, which would make every comparison with the
   same seed flip identically; the extra terms make the flips independent), and is
   recorded as `order_position` in every run line.
6. **Shared curves.** The lower-conductor curve of a pair appears in its pair,
   C-iso and C-same comparisons; those comparisons share that curve and are not
   independent. The pair family for Holm correction is the pair comparisons only.

### Amendment 3 (2026-10-26, after the main trials and the replay; exploratory, post hoc)

Observation that motivates it: in `analysis.json` mean relations per run differ
between the two curves of a comparison by amounts as large for the
same-conductor controls (for example P2-same 12.2 against 14.9) as for the pairs
(P2-pair 12.2 against 14.3). Relation yield is a property of the specific point
set, not only of the conductor. This amendment adds one **exploratory** analysis
that is **not** part of any gate or of the frozen decision rules of section 6, and
is labelled exploratory wherever it appears. It was specified before it was run:

- **Exact yield per curve.** For every trial-eligible (`j` not in `F_p`) certified
  curve of every group that has an independently verified pair, with the full
  factor base: `y = #{R in <G>, R != O : R = +-P1 +-P2 for some P1, P2 in F} / (r-1)`,
  the probability that a uniformly random non-identity element of the prime
  subgroup decomposes (no sampling). Normalised yield `z = y / |F|^2`
  (the quantity that removes the leading dependence on the base size).
- **Question.** Within a group (same `q`, `t`, `N`, `r`) do curves with different
  certified `f_E` differ in `z` more than curves with the same `f_E`?
- **Statistic and test.** Within each group with at least two distinct `f_E`:
  `B = sum over groups of sum over f_E classes of n_{g,f} (mean_{g,f} - mean_g)^2`
  (between-conductor sum of squares); null by permuting the `f_E` labels among the
  curves of each group (20 000 permutations, fixed seed); one-sided
  `p = P(B_perm >= B_obs)`. Companion: the mean absolute difference of `z` over
  curve pairs with different `f_E` minus over pairs with the same `f_E`, same null.
  The same two statistics are computed for `y` and, as a calibration, for the
  null label "curve index parity" (a label with no meaning).
- **Reading.** A small `p` would be a lead about a conductor-related (or a
  group-structure-mediated: `E(F_q) ~ Z/n1 x Z/n2` and `n1 = gcd(x - 1, f_pi/f_E)`
  depends on `f_E`) effect on yield, not evidence of a hardness difference, and
  would need the frozen G2 protocol to be followed. A large `p` is a null.

### Amendment 4 (2026-10-26, after all runs; status change, no result changed)

The coordinator withdrew the Python authorisation on which section 1 of this
protocol relied: `AGENTS.md`, "Implementation language: no Python", bans Python
for harnesses, solvers, benchmark drivers, certificate verifiers and result
generation, and the task brief that allowed "pure Python" was wrong. Section 1
above is left as written because it records what was believed when the runs were
made; it no longer describes an exception that exists. Consequences:

1. Everything in this directory produced by `*.py` is **legacy Python
   evidence**, preserved unmodified and labelled as such. Its timings are
   historical Python timings.
2. No Python was started or written after the order. The runs that exist are
   exactly those listed in `README.md`; `summarize.py`, the figures and
   `report.pdf` were not produced and are not planned in Python.
3. No claim of any kind rests on this directory until a matched native
   (Rust) re-measurement exists. `RESULTS_PROVISIONAL.md` records what was
   measured, with that limit.
