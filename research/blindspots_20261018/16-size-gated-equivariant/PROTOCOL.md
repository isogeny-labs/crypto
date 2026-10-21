# Item 16 protocol: size-gated equivariant oracle

Frozen before any measured run. Source design: section "16" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18. Prior result:
`research/ic_improve_20261014/02-equivariant-oracle/` (verdict TRADES).

Class: **component speed-up of the decomposition oracle** (constant level).
It is a within-method comparison: every arm calls the same repository solver
on the same targets, so the section 5a cross-method pricing of unpriced
native work does not rank any arm against rho or another method here. The one
place unpriced native work enters (GF(2^d) products inside the equivariant
pass) is priced at 0, 0.1 and 1 word-XOR equivalents per product in the
counted unit below. No GAE row, no rho gap is produced by this item.

## Question

The equivariant pass loses at `n = 5, 7` and wins 5.1x to 15.5x at `n = 11,
13` (item 02). Does a size-gated dispatcher, which uses the equivariant pass
only when the barrel system has at least `N0` unknowns, advance the oracle?

## What already exists, and what is reused

Read before designing: item 02's `PROTOCOL.md`, `README.md`, `runs/oracle.jsonl`
and `examples/ic_improve_02_equivariant_oracle.rs`.

* **Arms (a) and (b) are item 02's arms I and C.** Item 02 measured them on
  `n = 5, 7, 11, 13`, targets `t = 0..11` (`runs/oracle.jsonl`): I/C = 0.250,
  0.168, 5.10, 15.5; 48 of 48 targets passed the correctness gate; the
  attribution arm F (the incumbent's own degree-3 fall rows, no
  equivariance) gets F/C = 0.93, 0.47, 1.25, 1.34 (so most of the win at
  `n >= 11` is fall-row augmentation).
* **Those numbers are not reused as this item's data.** The incumbent solver
  changed after item 02: `d72e8b04d` (2026-10-17, "F6-IC gate-first on by
  default") makes the default `solve_boolean_system` do the same splits with
  fewer word XORs (the smoke test below measured -10 % solver word operations
  for I at `n = 11` against item 02's rows, identical roots). Both I and C go
  through that solver, so the ratio moves. Item 02's rows are cited as the
  prior and as the replication reference, and the smoke test checks the new
  instrument against them target by target (same `x_R`, same roots, splits
  equal within 3).
* **The instrument is item 02's, derived and re-built** (see Deviations 3):
  the barrel system, the equivariant degree-3 pass, the fall-row
  augmentation and the incumbent oracle are item 02's code, copied into
  `examples/blindspots_16_size_gated_oracle.rs`.

## The frozen cells are BLOCKED; the measured cells are a declared substitute

The design's cells are `n in {41, 47, 53, 61}`, "production IC". They cannot
be measured on this repository at this host. Four independent reasons, each
checked in code or by the `--mode feas` run recorded in `raw/` (arithmetic
only, no system built):

1. **The incumbent solver cannot represent the system.** Boolean monomials
   are `u64` bitmasks (`koblitz_groebner::MAX_VARS = 64`,
   `pq_groebner_f2::F2BoolMono`). The gauge-fixed barrel system the
   incumbent oracle solves has `4n + 2l` unknowns: 170, 194, 218, 250 at the
   four cells, against 64. The last size that fits is `n = 14` (item 02
   README, "Deviations"); the equal-orbit condition leaves `n = 5, 7, 11, 13`.
2. **The equivariant pass cannot hold the barrel system either.** Its masks
   are 192 bits (3 words); the barrel system has `5n + 2l` = 211, 241, 271,
   311 unknowns. The incumbent's own degree-3 Macaulay matrix at those sizes
   has 0.8 to 2.6 million columns; scaling item 02's measured `n = 23` matrix
   (66,629 x 149,660) gives a dense lower bound of 37 to 377 GB, against the
   4 GB cap. Item 02 already could not run the incumbent step at `n = 29, 31`.
3. **The production pipeline never hands a Frobenius-symmetric system to the
   solver** (item 02 README §1 and §5; the target `x_R` is a constant and
   squaring sends it to `x_R^2`). "Equivariant pass always" and any gate
   around it are dead code at every node of `groebner_decompose_with_geometry`.
   The mechanism that carries most of the win, arm F, was probed on the
   pipeline's own systems (item 02 §5): it regresses there (I/F = 0.59, 0.29,
   0.34 at `(n, l) = (13,4), (17,6), (19,7)`).
4. **The `crypto_lib` library does not compile at the branch base** (25
   errors in `ecbench/workload.rs`, `ecbench/mod.rs`, `gaudry_quartic.rs`;
   items 01, 02, 03, 05, 07, 08, 10 record the same). `cargo build --example`
   cannot build any instrument that uses the crate.

**Verdict at the frozen cells: BLOCKED**, decided here from the code and the
`feas` run, not by a measurement. What would unblock it: (i) monomials wider
than 64 bits through `pq_groebner_f2`, `inherited_f4` and `koblitz_groebner`
(a change to shared `src/` files, outside this item); (ii) a
Frobenius-symmetric formulation inside the production pipeline, or a reason
the barrel gauge system is the production oracle's system at those sizes
(it is not today); (iii) an orbit-blocked incumbent step that fits 4 GB at
`n = 41`.

**The measured substitute** is the same dispatcher question asked on the
only sizes where all three arms exist: the barrel gauge-fixed oracle at
`n in {5, 7, 11, 13}` (barrel unknowns `5n + 6` = 31, 41, 61, 71), `m = 2`,
`l = 3`, the item 02 setting. Its verdict is labelled **barrel-oracle level,
toy sizes** and is never reported as the frozen question's answer or as a
production-oracle result.

## Arms

All arms run in one process per target on the same target, in a per-target
seeded random order (item 02 always ran I first), single thread,
under `prlimit --as=4294967296`. The system construction (barrel build,
gauge fixing, conversion to `F2BoolPoly`) is charged to every arm.

| arm | what runs | charged |
|---|---|---|
| **(a) I** | `solve_boolean_system(gauge-fixed barrel, SolverEngine::default(), split_rule_default())` | build + solve |
| **(b) C** | the equivariant degree-3 pass on the symmetric barrel system, specialisation `s0 = e0`, RREF, reduced rows appended to the generators, same solver | build + full pass incl. assembly + solve |
| **(c) gate** | `if sym_vars >= N0 { C } else { I }`, `sym_vars = 5n + 2l` of the barrel system | the chosen arm's cost; the one integer comparison is charged 0 |
| **F** (control, not in the rule) | the incumbent's own degree-3 step on the gauge-fixed system, fall rows appended, same solver | build + step + solve |
| **I2** (A/A) | I again | as I |

Arm (c) is computed from the measured arms of the same target, not run as a
fourth solve: below `N0` it executes the incumbent's code (cost = I's
measurement, ratio 1 by construction), at or above `N0` it executes C's code
(cost = C's measurement). The A/A arm I2 supplies the noise floor against
which the gated-on sizes are read. This is stated here so the reader knows
that the per-size a/c at a gated-off size is exactly 1 and carries no
information beyond the gate's decision.

## Targets and the tuning of `N0`

* Targets: `x_R` drawn by `StdRng::seed_from_u64(20261014 ^ (n*1000 + t))`
  uniform in `F_{2^n}^*`, item 02's generator.
* **Training:** `t = 0..11` at each of the four sizes (48 targets, item 02's
  own targets, re-run under the current solver). **Held-out:** `t = 12..19`
  at each size: 8 per size, **32 targets**, as the design fixes. Item 02 ran
  `t = 0..11` only, so the held-out targets are new to every arm. The held-out
  rows are never read by the tuning step.
* **Tuning rule (frozen):** candidate thresholds are each observed training
  `sym_vars` value and "never use C". For a threshold `T` the objective is
  `J(T) = sum over training sizes with sym_vars >= T of mean ln(I/C)` (the
  mean-log-cost saving over always-I); `N0 = argmax J`, ties to the smallest
  threshold, "never" wins only if every `J` is `<= 0`. Targets that fail the
  correctness gate are excluded and counted.
* `N0` is a function of the training rows only. The analysis tool applies it
  to the held-out rows; no result of the held-out run can change it.

## Metric and unit

* **Primary:** per-target thread CPU seconds (`CLOCK_THREAD_CPUTIME_ID`),
  each arm's whole cost including assembly. Reason: item 02's finding that
  `F2` word XORs and `F_{2^d}` table products are not commensurable, so no
  single counted number prices both arms without a stated exchange rate.
* **Counted unit (secondary, required to agree):** `F2` word XORs =
  solver `f4_word_ops_thread` + the pass's trivial-block kernel XORs + the
  specialised-row kernel XORs, plus `w x` (GF(2^d) products of the pass) with
  the price `w in {0, 0.1, 1}` XOR-equivalents per product (the section 5a
  band applied to this unit; 0 under-prices the pass, 1 is a floor on a table
  product). The barrel build and orbit enumeration are not in the counted
  unit; CPU time prices them.
* Paired ratio per target `rho = cost(a) / cost(c)`; per size the geometric
  mean with a 95 % percentile bootstrap CI (10,000 resamples of targets
  within size, fixed seeds in the tool); pooled over the 32 held-out targets
  with targets resampled within size. The ratio of summed CPU is reported as
  well (the cost-weighted view).

## Gates (correctness)

* **Oracle gate:** on every target the root sets of I, C and I2 equal a
  brute-force enumeration of `(k1, c1, k2, c2)` with `S3 = 0`, and no arm
  exhausted its budget. F's gate is reported separately (control).
* Any failed target is excluded and counted; a failed gate in the held-out
  run makes the verdict INCONCLUSIVE.

## Decision rule (frozen)

Applied to the held-out rows with `N0` from the training rows:

* **ADVANCES (constant, barrel-oracle level, toy sizes)** iff (i) all gates
  pass, (ii) both training positive controls hold, (iii) the pooled
  `a/c` CI lower bound is `> 1` in CPU time, (iv) **no size regresses**: no
  size has an `a/c` CI upper bound `< 1`, and (v) the counted-unit pooled
  `a/c` CI lower bound is `> 1` at every price `w in {0, 0.1, 1}`. If (i)-(iv)
  hold and (v) fails the verdict is INCONCLUSIVE (the two units disagree).
* **REGRESSES** iff the pooled CI upper bound is `< 1` and every size's
  `a/c` CI upper bound is `< 1`.
* **MATCHES** iff the pooled CI contains 1 and no size's CI excludes 1.
* **TRADES** otherwise (some sizes better, some worse, or the pooled and the
  per-size readings differ).
* The scale caveats are part of the verdict line: toy sizes, the barrel gauge
  oracle, not the production oracle; the gain at the gated-on sizes is mostly
  fall-row augmentation (arm F) and F is known to regress on the pipeline's
  own systems. No exponent statement is made (four sizes, two of them gated
  off).

## Controls

* **A/A (null):** I against I2 per size; the CI is reported and a size whose
  A/A CI excludes 1 is flagged as an order or contention bias (the random
  arm order is the mitigation; item 02 saw up to 4 % at the oracle level).
* **Positive control (training rows):** the instrument must see the known
  loss at the smallest size (`I/C` CI upper bound `< 1` at `n = 5`) and the
  known win at the largest (`I/C` CI lower bound `> 1` at `n = 13`). If
  either is missing the run is INCONCLUSIVE and `N0` is not trusted.
* **Negative control (the wrong gate):** arm (b), "equivariant always", is the
  mis-set gate `N0 = 0`; the analysis tool run with `--force-n0` equal to the
  smallest `sym_vars` must give a pooled or per-size regression (TRADES or
  REGRESSES), and with a threshold above every size must give MATCHES. Both
  were checked on the smoke rows.
* **Replication of item 02:** the instrument reproduces item 02's targets
  (`x_R`), root counts and splits (see Smoke test).

## Budget

Four sizes x (12 training + 8 held-out) targets. Smoke-measured cost per
target: `n = 13` about 21 s wall (I 9.4 s, I2 9.4 s, C 0.9 s, F 1.3 s),
`n = 11` about 5 s, `n = 5, 7` under 0.1 s. Total about 10 CPU minutes
(`n = 13` dominates at 7 minutes). One thread per job, address-space cap
4 GiB per job, peak RSS about 0.1 GB. Far under the 2 CPU-hour item cap and
the 45-minute job cap. The measured runs are split into cards that each
finish in under 8 minutes of wall time.

## Smoke test (not measured data, kept out of every fit)

Run before the cards were frozen, at the smallest draw counts. What was run:

* `--mode feas` on all eight sizes (arithmetic only).
* `--mode oracle`: `n = 5, 7, 11` at `t = 0..3` (training targets) and at
  `t = 12..15` (**held-out targets**), `n = 11` and `n = 13` at `t = 0, 1`, and
  `n = 13` at `t = 12`. Every row: all gates true (I, C, I2 and F).
* The analysis tool on the 24 rows at `n = 5, 7, 11`, including its two
  `--force-n0` negative controls (threshold at the smallest `sym_vars` gives
  TRADES; a threshold above every size gives MATCHES).
* Replication of item 02 on its `t = 0..3` targets at `n = 5, 7, 11`: `x_R` and
  root counts equal on 12 of 12; split counts of I within 2 and of C within 3 of
  item 02's on all 12 (equal on most); solver word XORs of I 10 % lower at `n = 11` than
  item 02's rows (the solver change above).

**Disclosure.** (1) The smoke set included held-out targets `t = 12..15` at
`n = 5, 7, 11` and `t = 12` at `n = 13`; the cards re-run all of them and
nothing was tuned on them, but the author saw their smoke ratios. (2) **Amendment
A1, made after the first smoke analysis and before this file was committed or
any card ran:** the first analysis tool defined the gated arm's cost below `N0`
as the *second* incumbent run I2, so the A/A order noise (a/a2 = 0.984 with CI
[0.970, 0.997] at `n = 7` on four targets) was scored as a regression of the
gate and gave TRADES. A gated arm that is the incumbent's code below `N0` has
the incumbent's cost, so the definition became "cost = I's own measurement"
(ratio 1 by construction), with the A/A ratio reported beside it as the noise
floor. This only affects sizes below `N0`, where the arms are identical code;
it can not move the gated-on sizes or the pooled ratio's direction there. It
is the reason the Arms section says the per-size a/c at a gated-off size is
exactly 1.

## Deviations from the design (every one listed)

1. **Cells: the frozen `n in {41, 47, 53, 61}` production cells are BLOCKED**
   (four reasons above) and replaced by the four feasible barrel sizes
   `n in {5, 7, 11, 13}`. This loosens the design's scale and its object (the
   barrel gauge oracle in place of "production IC"), so the substitute's
   verdict cannot be promoted to the design's question. The frozen cells'
   verdict is BLOCKED, not a measurement.
2. **"Training sizes" become training targets.** The design tunes `N0` "on
   training sizes", but only four sizes are feasible and all are needed as
   held-out cells, so `N0` is tuned on 12 earlier targets per size and
   evaluated on 8 disjoint targets per size. The tuning step therefore sees
   the sizes it is later tested on; it never sees the tested targets. This
   makes the held-out test weaker than a size-level hold-out, and it is said
   wherever the result is quoted.
3. **Build route.** The library does not compile, so the instrument is a
   single `rustc` build that includes the repository's own solver modules
   unmodified by `#[path]` (`koblitz_groebner`, `inherited_f4`,
   `matrix_f5_f2`, `pq_groebner_f2`, `sparse_macaulay`, `gf2_elim`,
   `fx_hash`, `algebra_cache`, `binary_ecc/f2m`) and links the extern crate
   rlibs of the warm shared `target/release/deps` (`rand`, `rayon`, `serde`,
   `serde_json`, `num_bigint`, `blake3`, `libc`), named and hashed in the
   cards. The incumbent is therefore byte-for-byte the production solver;
   only `find_irreducible` (the smallest-mask irreducible, trial division)
   is a local copy. No file under `src/` is changed.
4. **Arm (c) is derived, not re-run** (stated under Arms): a gated arm that
   is the incumbent's own code below `N0` and C's code above it has the cost
   of the chosen arm; re-running it would duplicate a measured solve.
5. **Tightenings** (allowed): random per-target arm order (item 02 had I
   first); an A/A arm in the same process; the attribution arm F kept as a
   reported control; peak RSS recorded (item 02 did not); the verdict
   requires the counted unit to agree at all three prices; the rule is
   applied by a tool (`examples/blindspots_16_gate.rs`) frozen with the cards.
6. **Held-out count.** The design's "32 held-out targets" is 8 per size at 4
   sizes; the sizes are not the design's.
7. **Section 5a.** No cross-method row exists; the three-price band is applied
   to the one unpriced native cost inside the counted unit (see Class).
8. **Seeds.** Targets use item 02's `StdRng` generator, so the rlib of `rand`
   is linked; the arm-order permutation uses the same `StdRng` with its own
   fixed seed `0x16B11D5F ^ (n*1000 + t)`.

## Reproduce

```sh
# in the repository root, with the shared warm target/release/deps
rustc --edition 2021 -C opt-level=3 -C codegen-units=1 -L dependency=target/release/deps \
  --extern ... -o /tmp/b16 examples/blindspots_16_size_gated_oracle.rs   # exact line: card-01
/tmp/b16 --mode feas --n 5,7,11,13,41,47,53,61
/tmp/b16 --mode oracle --n 5,7,11 --targets 12 --first 0      # training
/tmp/b16 --mode oracle --n 5,7,11 --targets 8  --first 12     # held-out
rustc --edition 2021 -C opt-level=3 -o /tmp/g16 examples/blindspots_16_gate.rs
/tmp/g16 --train raw/train-*.jsonl --held raw/held-*.jsonl     # comma-joined
```

Raw rows: `raw/`. The analysis is run after the cards, on the committed rows, by
the assess stage; it needs no further measurement.
