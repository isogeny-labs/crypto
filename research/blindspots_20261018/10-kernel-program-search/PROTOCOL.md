# Item 10 protocol: program search over the decomposition kernel, fitness = counted oracle cost

Frozen 2026-10-18, before any measured run. Source design: section "10" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places and loosened nowhere; every deviation and every filled-in gap is listed
at the end.

Class: **stage diagnostic** (one oracle stage, the `S_4` pair-and-root kernel,
on toy factor bases over `GF(2^n)`, `n` in {31, 37, 41}). It computes no `S`,
no rho ratio and no GAE, so the section 5a cross-method ranking does not apply
to its own rows. Section 5a is applied the way it is meant for a stage: the
counted unit is field multiplications plus squarings (inversions expanded into
their multiplications and squarings), every unpriced native operation of the
kernel is counted and priced at 0, 0.1 and 1 counted operations, the same for
every arm, and a conclusion is drawn only where it holds at all three weights.
The gap to matched rho is unchanged by this item and is reported as the
unchanged distance remaining; a faster oracle stage is a constant factor on the
relation phase and does not touch the exponent (`semaev_decomp.rs` module doc,
"This does not rescue index calculus").

## Question

Can an automated propose-evaluate loop find a decomposition kernel or
factor-base predicate with lower counted oracle cost than the incumbent at fixed
yield?

## What already exists, and what is reused (read before designing)

* **The incumbent is the repository's `SubspaceOracle::decompose`** in
  `src/cryptanalysis/semaev_decomp.rs`, as `src/cryptanalysis/ic_boundary.rs`
  (`Oracle::SemaevS4`, line 5469) and `ic_oracle_pricing.rs` call it: the factor
  base is an `F_2`-subspace `V` of dimension `l` (all `2^l` abscissae, rational
  or not); the pair loop is over sorted pairs `x1 <= x2` in row order; the
  quartic in `x3` is the general-`b` form; one batch inversion per row; the
  roots in `V` come from `gcd(f, L_V mod f)` with `L_V mod f` by the monic
  Horner route under the quartic Frobenius table; the oracle **returns the first
  raw witness** and the caller `lift_abscissae` accepts it only if all three
  abscissae are factor-base members, otherwise the target yields no relation
  (`ctr.lift_failures`). Its committed cost unit is "pairs"
  (`ic_oracle_pricing.rs` table); `calibrate_s4` converts pairs to nanoseconds.
  **No committed result counts the kernel's field operations per pair**, so the
  counted unit used here is new; the incumbent's own number is the first
  measurement of it (about 141 counted multiplications and squarings per pair in
  the development build, 79 word XORs; reported again by the selftest).
* **Yield conservation (`AR:KN-FIND-007`, established, `AR:DEC-20260724-007`)**
  is cited and used as the prediction to be checked, not re-proved: the mean
  decomposition yield depends only on the base size, so a cleverer set of the
  same size cannot raise it. This item checks it at toy cells (Controls,
  "conservation"), because the fitness below assumes it. It does not apply to
  *which targets* a base reaches; see the trace class below.
* **The trace class of a base** (item 11, `PROTOCOL.md` there, and
  `research/sat_factor_base_review_20260908/MATHEMATICS.md` line 111):
  `delta(P) = Tr(x(P) + a)` is a homomorphism `E(F) -> F_2` with kernel `2E`.
  Every real target `kP` with `P` of odd order lies in `2E` (`delta = 0`), so
  only triples with `delta_1 + delta_2 + delta_3 = 0` can decompose it. A
  subspace base has `delta` balanced about half and half; a base on which
  `delta` is identically 0 wastes no triple; a subspace inside `ker Tr` has
  `delta` identically 1 and **never** decomposes a real target. This is a lever
  the program space contains (the "affine" families), and a trap the fitness
  must not fall into (Controls, `affine-anti`).
* **Nothing measured earlier searches over kernel programs.** The nearest
  committed work is the `ic_candidate_tournament_20260915` autolab (candidate
  factor bases and decomposers as whole pipelines), which varies bases but does
  not search kernel structure with a counted fitness, and the factor-base
  encyclopedia (`docs/ic/factor-base-encyclopedia/README.md`), which closes
  geometry for table oracles by the conservation law above. No cell or arm of
  this item is reused from them; nothing is skipped on their account.
* **The crate does not compile at this branch base** (`cargo build --release
  --lib`: 25 errors in `ecbench` and `gaudry_quartic`, re-run 2026-10-18 at the
  start of this item), so no code that links the crate can run, `ecbench`
  included. The instrument is std-only
  (`examples/blindspots_10_kernel_search.rs`) and each card builds it with
  `rustc`, as items 04, 05, 07, 08, 09, 11 and 12 do. The incumbent is therefore
  a **native replica** of the kernel validated against brute force (Controls),
  not a link against the crate. This is Deviation 1.

## Objects

* `F = GF(2^n)`, `n` in {31, 37, 41}, modulus the repository's sparse
  irreducible (same candidate order as `find_irreducible_sparse`; tails
  `0x9`, `0x53`, `0x9`). Curve `E: y^2 + xy = x^3 + x^2 + 1` (the Koblitz curve
  `a = b = 1`). `#E = 2^n + 1 - V_n` with `V_0 = 2, V_1 = 1, V_k = V_{k-1} -
  2 V_{k-2}` is checked against the group law by `#E . Q = O` on random points
  and equals 2 mod 4 at all three sizes, so `2E = ker delta` exactly.
* A **factor base** is a set `X` of `2^l` abscissae, here `l = 9` in every cell
  (512 abscissae, about 250 rational ones). `x` is **rational** if some
  `P in E(F)` has `x(P) = x` (`Tr(x + a + b/x^2) = 0`, or `x = 0`). The number of
  rational members is `B_r` (the relation matrix has `B_r` columns).
* A **relation** for the target `R` (a point of `2E`, `x_R` its abscissa) is a
  triple of rational members `x1 <= x2 <= x3` of `X` (repeats allowed) with
  `S_4(x1, x2, x3, x_R) = 0`, where `S_4` is evaluated from the resultant form
  `(s^2 p'^2 + s'^2 p^2 + b (s^2 + s'^2))^2 + (s^2 p' + s'^2 p)(p + p')(p p' + b)`,
  `s = x1 + x2, p = x1 x2, s' = x3 + x_R, p' = x3 x_R`. It is equivalent to
  `R = +-P1 +-P2 +-P3` with `P_i in E(F)`; this is verified numerically (S_4
  vanishes on `x(P1 +- P2 +- P3)` for random point triples at `n` = 17, 31, 41).
  The accepted witness of a program on a target is its first relation in its
  scan order.
* **Targets.** Natural targets ("negatives" below, though a few percent of them
  at small cells do decompose): `x(2Q)` for random rational `Q`, uniform over
  `2E`. Planted targets ("positives"): `x(P1 +- P2 +- P3)` with `P_i` random
  rational members of *the program's own base*, signs random, accepted only if the
  sum lies in `2E` and `x_R` is not a base member. Every planted target has the
  planted relation by construction; whether other relations exist is left to the
  scan.
* **Regime.** At `l = 9` and `n >= 31` the expected number of raw `S_4` roots in
  `V^3` for a natural target is below 0.02, so a natural target almost never has
  a relation and a planted target has essentially only its planted one (the
  **sparse regime**). The regime in which relations are common (`l` about
  `n/3`) is out of reach for 256-program searches inside the budget; it is
  covered at toy cells instead (Controls, "conservation").

## The kernel DSL (the search space)

A program is ten integer genes. The canonical form zeroes genes that cannot
change behaviour; a program is valid if its root finder is defined on its base.
`b = 1` throughout.

| gene | values | meaning |
|:--|:--|:--|
| `fam` (24) | `sub-low` (span of `z^0..z^8`, the incumbent's base); `sub-rand1..6` (random 9-dim subspaces); `coset2-s1..4` (union of 2 cosets of a random 8-dim subspace, translates `0` and random); `coset4-s1,2` (4 cosets of a 7-dim subspace); `ham`, `ham-shift` (the 512 lowest-weight words in (weight, value) order, plain and XOR a random shift); `geom-s1..4` (the 512 powers `g^0..g^511` of a random `g`; the cyclotomic family, see Deviation 8); `affine-rand-s1,2` (a coset `c + U` of a random 9-dim `U`, random `c`); `affine-aligned-s1,2` (`U` inside `ker Tr`, `Tr(c) = 1`: `delta` identically 0); `affine-anti` (`U` inside `ker Tr`, `Tr(c) = 0`: `delta` identically 1, zero yield) | the base predicate |
| `lift` | 0 / 1 | 0: scan all `2^9` base abscissae; 1: prefilter the pair elements `x1, x2` to the rational members (table built once per base) |
| `stop` | 0 / 1 | 0: stop at the first raw witness and let the caller's lift check decide (the incumbent); 1: continue until a lifted witness |
| `root` | `horner` / `generic` / `eval` | the roots of the quartic in the base: `gcd(f, L mod f)` with `L mod f` by the Horner route under the quartic Frobenius table / by the generic power-of-`t` route / evaluate the quartic at every base element (the only root finder defined on `ham*` and `geom*`) |
| `quartic` | `general` / `b1` / `naive` | the coefficients of `S_4` in `x3`: the incumbent's shared-subexpression form / the same with the two multiplications by `b = 1` dropped / expanded from the resultant as polynomials in `t` |
| `sqtab` | 0 / 1 | use a table of base squares for `s^2 = x1^2 + x2^2` |
| `pairs` | `sorted` / `full` | `x1 <= x2`, or all ordered pairs |
| `abort` | 1, 3/4, 1/2, 1/4, 1/8 | scan only the first `ceil(f m)` rows (values of `x1`) of the list |
| `batch` | row / 8 rows / pair | the monic leading-coefficient inversions: one batched per row / per 8 rows / one per pair |
| `gcd` | `fast` / `generic` | the generic-shape pseudo-remainder gcd written out (with a generic fallback) / the generic routine |

For the coset and affine families the root finder multiplies out
`P(u) = prod_i (u + L_U(c_i))` for `u = L_U(t) mod f`, which vanishes exactly on
the union of cosets: the same Horner route with one extra polynomial step
(`k = 1`: one XOR; `k = 2`: one squaring and one scalar product; `k = 4`: two
squarings, one polynomial product and three scalar products). Setup constants
(the `L_U` coefficients, the symmetric functions of the `L_U(c_i)`, the rational
flags, the squares, the planted-point lists) are built once per base and are
**not** per-target cost; everything that depends on the target, including the
Frobenius inverse roots of `L_U` (`LvHorner::new`, which the incumbent also
rebuilds per target), is counted.

The programs the design names map as follows: *filter order* = `lift` and
`stop`; *early-abort thresholds* = `abort`; *table layout* = `sqtab` and
`batch`; *root-finder choice* = `root` and `gcd`; *base predicate* = `fam`
(subspace, Hamming, cyclotomic and the coset/affine generalisations).

## The unit and the fitness

**Counted cost.** A target's cost is the sum of counters over its scan:
`core = multiplications + squarings` (weight 1; every inversion is an
Itoh-Tsujii chain of counted multiplications and squarings; batch inversion is
Montgomery's), and four unpriced native counters: `lin` (word XORs of the
coefficient arithmetic), `probe` (hash lookups of the lift check, 3 per raw
witness), `trace` (trace tests), `pairs` (pair iterations). The priced cost at
weight `w` is `core + w (lin + probe + trace + pairs)`, `w` in {0, 0.1, 1}. Every
arm emits all five counters for every target (the JSONL carries
`[mul, sqr, lin, probe, trace, pairs]` per target). The replica counts
multiplications unfused, where the incumbent fuses carry-less products under one
reduction (`dot2`); the unit is the same for every arm, absolute counts overstate
the incumbent's real field work, and wall time is descriptive only.

**What is excluded, identically for every arm:** target generation (one group
addition per target, under 0.01% of a scan), and the sign resolution of an
accepted relation (`lift_abscissae` additions; it follows from the lift check
that signs exist).

**Collection cost index.** For program `pi` on its base, let `cbar_N` be the
mean priced cost over the natural targets, `phi` the fraction of planted targets
on which `pi` accepts a relation (retention of positives), `B_r` its number of
rational members and `T_eff` the number of multisets of three rational members
with `delta_1 + delta_2 + delta_3 = 0`. With the natural yield per target
`p_y = kappa T_eff` (`kappa` the same for every base: the yield model, checked
below), the counted cost per accepted relation is `cbar / (p_y phi)` and the
relation-collection cost for `B_r` columns is that times `B_r`, so

> **Q(pi, w) = cbar_N(w) . B_r / (T_eff . phi)**

up to the constant `kappa`, which cancels in every ratio. `Q` is dimensionless
in the sense that only ratios of `Q` are read. The first-order error of
replacing `cbar` by `cbar_N` is `p_y cbar_P / cbar_N` (planted cost over natural
cost, times the natural yield); it is below 0.02 at `n >= 31` and is reported
(`found` among the natural targets is in every JSONL). `Q` is `+inf` when
`phi = 0` or `T_eff = 0` (`affine-anti`) or when the program exceeds the
evaluation cap.

This is "counted oracle cost per relation at equal relation count" with the two
corrections a base-and-kernel search needs: the number of relations needed is
the number of columns `B_r`, and the share of triples that can sum into `2E` is
a property of the base. Both are stated so that no arm is credited for a
relabelling: `Q` equals the incumbent's `Q` for any program that scans the same
pairs with the same operations.

## Arms

* **(a) incumbent**: the program with every gene 0 (`sub-low`, scan all, stop
  at the first raw witness, `horner`, `general`, no square table, sorted pairs,
  all rows, batch per row, fast gcd).
* **(a1) incumbent plus the one-gene patch `lift = 1, stop = 1`**: an added
  reference, not an arm of the rule. It separates "does not scan abscissae that
  have no point" from everything else a search might add.
* **(b) search**: a (1+1)-evolution strategy with an elite archive. Start at
  the incumbent (evaluation 1). Each later proposal draws its parent as the best
  program so far with probability 0.7, else uniformly from the best 8, then
  mutates one gene (probability 0.7) or two genes to a different value, rejects
  any program already evaluated or invalid (up to 200 redraws, then a uniform
  random valid program), and is evaluated. The archive keeps every evaluation.
  **256 evaluations** per cell. Every random draw is a pure function of
  `(cell seed, arm, evaluation index)`, so a card boundary changes nothing.
* **(c) random search (negative control by design)**: 256 distinct programs
  drawn uniformly from the valid canonical genotypes, no mutation, no use of
  results.
* **(p) positive control for the search**: the same engine as (b), started at a
  deliberately handicapped incumbent (`pairs = full`, `quartic = naive`,
  `root = generic`, `batch = pair`: about five times the incumbent's counted
  cost), 256 evaluations, `n = 31` only. It passes if its best training `Q` is
  at most the incumbent's training `Q` and at most half of its own starting `Q`.

**Evaluation.** Training set per cell: 6 natural targets (shared by all
programs) and 32 planted targets per base (seeded per base, so each program is
planted against its own base). Fitness for the search is `Q` at `w = 0.1`. An
evaluation is cut off when one target exceeds 3 times the mean priced-at-1 cost
per target of the arm's start program on the training natural targets (the
incumbent for (b) and (c); the handicapped incumbent for (p), about 15 times the
incumbent); the program is then recorded with `Q = +inf` ("dominated"). Both (b) and (c) use the
same cap. Held-out set per cell (different seeds, never used by a search): **128
natural targets and 128 planted targets per base**, no cap.

**Selection of the finalists.** The finalist of an arm is the evaluated
program with the lowest finite training `Q` (ties: lowest index). Nothing else
chooses it.

## Controls

* **Brute-force equality (instrument correctness).** At `n = 17, l = 6`, for 400
  random valid programs and 10 targets each (6 natural, 4 planted, from the
  program's own base), (i) every accepted witness satisfies `S_4 = 0` with all
  three abscissae rational base members; (ii) for every exact program
  (`stop = 1`, `abort = 1`) the decision equals brute-force existence of a
  rational triple with `S_4 = 0`. Zero disagreements required.
* **Handicap positive control (instrument sensitivity).** At `n = 31, l = 9`:
  `pairs = full` costs 2.00 times the pairs (required 1.95 to 2.05); the lift
  prefilter costs the squared rational fraction of the pairs (required 0.20 to
  0.30); `abort = 1/8` costs `1 - (7/8)^2 = 0.234` of the pairs (required 0.22 to
  0.25); `root = generic` costs more than 1.3 times the incumbent's counted
  operations; `quartic = b1` costs fewer. The search positive control is arm (p).
* **Null.** The incumbent evaluated on two disjoint halves of the held-out
  natural targets: the ratio of their `Q` has an interval covering 1 at all three
  weights (reported in `summary`).
* **Negative controls.** Arm (c). `affine-anti` is a family that decomposes no
  real target: its `Q` must be `+inf`, never the cheapest. A family whose
  rational points all have `delta = 0` (`affine-aligned`) must show the doubled
  yield the model predicts (below).
* **Yield conservation and the yield model (cells with common relations).** At
  `(n, l) = (17, 6)` (all 24 families, 4000 natural targets) and `(19, 7)` (the
  families that use a root finder other than `eval`, 3000 natural targets); both
  have `2^n - 1` prime, so no family degenerates (`(13, 5)` is too dense, its
  rates saturate). Every family runs its exact program (`lift = 1, stop = 1,
  quartic = b1`; `eval` for `ham*`, `geom*`) and reports the found rate, `T_eff`
  and the Poisson-corrected yield per feasible triple `kappa = -ln(1 - rate) /
  T_eff`. KN-FIND-007 and the yield model predict `kappa` constant across
  families; the check passes for a family if its `kappa` is within 25% of the
  median over the `sub-*` families. The finalists and the incumbent are run on
  the same targets, and their counted cost per relation found times `B_r` (`W`)
  is reported at the three weights; `W` at these cells is a natural-yield
  measurement with no planting and carries no verdict weight. A finalist whose
  family violates the yield model cannot support a verdict (INCONCLUSIVE).
  (Development at `(19, 7)` gave `kappa` between 2.74e-5 and 3.06e-5 over all
  22 non-trap families, including the `affine-aligned` ones whose found rate is
  about twice that of a balanced base, and found the same number of relations
  for the incumbent as for the exact lifted program: a raw witness with a
  non-rational abscissa essentially needs a conjugate pair or `x_R` in the base.
  These numbers are not used for the rule.)
* **Ablation.** For each finalist, per differing gene: the finalist with that gene
  reverted to the incumbent value, and the incumbent with only that gene changed,
  on 48 natural and 48 planted held-out targets. It attributes the gain; it
  carries no verdict.

## Cells, budget and the run plan

* Cells: `n` in {31, 37, 41}, `l = 9`, arms (a), (a1), (b), (c) in each, arm (p)
  at `n = 31`. Seeds: a fixed base per cell, `0xB10_5EED_0000 + n`; training,
  held-out, family and search streams are separate pure functions of it. The
  development runs that calibrated the instrument used another seed base and are
  discarded; none of their numbers is a result.
* Per search: 256 evaluations, about 0.5 to 3 seconds each in development
  (random programs are slower, the capped ones are cut at the first target).
  Searches run in chunks of 64 evaluations per command, resumed from a state
  directory (`--state`) that holds the append-only archive; a chunk boundary
  changes nothing (pure functions of the evaluation index).
* Planned measured CPU, one thread, one process at a time, address space capped
  at 4 GB (`prlimit --as`; the instrument uses a few MB): searches 25 to 35
  minutes in total (seven searches; (b) about 3, (c) about 5 and (p) about 4
  minutes each when alone, up to twice that on the loaded shared host), held-out
  evaluation about 5 minutes, ablations about 5, conservation cells about 3,
  summaries seconds. About 45 minutes alone and under 1.5 hours at the loaded
  rates seen in development, inside the 2 CPU-hour item limit; no single command
  exceeds 10 minutes.
* Cards are `kind: frozen_command`, `experiment_id: BLINDSPOT-10`, none longer
  than 9 minutes (the timeouts of a card's commands sum to at most 540 seconds).
  The first card builds the binary and runs the selftest; every later card
  rebuilds its own private copy from the bound source (about 4 seconds) and
  removes it, as items 09 and 11 do, so that no card depends on a file another
  card produced except the search state directory. All outputs are under
  `research/blindspots_20261018/10-kernel-program-search/raw/`.

## Decision rule (frozen)

For each cell and each weight `w` in {0, 0.1, 1}, from the held-out data:
`Q(B)/Q(A)` and `Q(B)/Q(C)`, where `B` is arm (b)'s finalist, `C` arm (c)'s
finalist and `A` the incumbent, each with a 95% interval from 2000 bootstrap
resamples (the 128 natural targets resampled jointly for both programs, the 128
planted targets resampled independently per program).

A ratio is *better* at a weight if the upper end of its interval is below 1,
*worse* if the lower end is above 1, otherwise it *straddles* 1.

* **ADVANCES (constant)** for the cell: `Q(B)/Q(A)` is better at all three
  weights **and** `Q(B)/Q(C)` is better at all three weights. (The design says
  "beats (c)'s best"; the interval requirement on that comparison is a
  tightening, Deviation 5.)
* **MATCHES** for the cell: `Q(B)/Q(A)` is better at all three weights but
  `Q(B)/Q(C)` is not better at all three: the search improves on the incumbent
  by a measured amount, but random search of equal budget finds an equally good
  program. The improvement over (a) is then a verified measured improvement
  *of the program space* and is reported as such; it is not credited to the search.
* **REGRESSES**: `Q(B)/Q(A)` worse at all three weights.
* **TRADES**: better at some weights and worse at others.
* **INCONCLUSIVE**: otherwise, or when the yield-model check fails for the
  finalist's family, or a control fails.
* The **overall** verdict is the common per-cell verdict if the three cells
  agree, and INCONCLUSIVE (with the three per-cell verdicts listed) if they do
  not. The level is *constant* throughout: no program in this space changes the
  `Theta(2^n)` relation-collection cost.

Report with every verdict: `Q(B)/Q(A1)` (what the search adds beyond the
one-gene lift patch), the gene-level ablation, the training-to-held-out
shrinkage of the finalists' `Q`, the per-arm running-best curves, the dense-cell
cost per relation, the unpriced counters of all arms, and the unchanged distance
to matched rho (this is a stage diagnostic; no `S` is computed).

## Deviations from the design, and filled-in gaps

1. **Replica, not ecbench.** "Evaluated by ecbench counted cost" is realised as
   the counted field operations of a native std-only replica of the incumbent
   kernel and of every program, because the crate (and with it `ecbench` and the
   real `SubspaceOracle`) does not build at this branch base. The replica is
   validated against brute force on every program class it runs (Controls); it
   is not validated against the crate's kernel bit for bit, and no GAE
   conversion is attempted.
2. **`l = 9` in every cell** (the design fixes `n`, not `l`). Inside the budget
   this is the sparse regime (about 0.02 raw roots per natural target at `n =
   31`). The measured yield and the cost per relation on natural targets are
   therefore taken at the toy cells above, where the model is checked, and
   carried to the three cells by the yield model. This is a limitation of the
   design's cell choice at this budget and is stated in every verdict.
3. **Held-out set of 128 natural + 128 planted targets** per cell, where the
   design gives 64 held-out targets per `n`. Larger, never smaller.
4. **The metric.** The design's "counted oracle cost per relation at equal
   relation count" is implemented as `Q` above (cost per relation times the
   number of columns, with the trace-feasible triple count), because relations
   are essentially absent from natural targets in the sparse regime and because
   bases of different composition must be compared at equal collection work.
   The planted targets supply the positives; their relation is the planted one.
5. **Rule tightened:** the comparison with (c) must exclude 1 at all three
   weights, per cell, and the overall verdict needs all three cells to agree.
6. **Added reference and control arms:** (a1) and (p); neither enters the rule.
7. **Frobenius-invariant subspaces are not in the space.** On `K_n` they allow
   orbit folding, but `x^37 - 1` has no divisor of degree near 9 except 1 and 36,
   `x^41 - 1` none except 1, 20 and 21, and `x^31 - 1` has dimensions 5, 6, 10, 11
   only, none of which is 9.
8. **"Cyclotomic" is the geometric progression `{g^i : 0 <= i < 512}`**, not a
   coset of a multiplicative subgroup: `2^31 - 1` is prime and so has no subgroup
   of order near 512, `2^37 - 1 = 223 . 616318177`, `2^41 - 1 = 13367 .
   164511353`. Its only defined root finder is `eval`.
9. **The coset and affine families are an addition** to the design's three
   families (subspace, Hamming, cyclotomic); they are the generalisation of
   "subspace" under which the Horner route still applies and under which the
   trace class of the base is a free parameter.
10. **Counting unfused operations** (see "The unit"). The unit is the same for
    every arm.
11. **Development calibration:** before freezing, the engine was run end to end
    at `n = 31, l = 9` on seeds other than the frozen ones to size the cells,
    the evaluation cap and the cards; the families `affine-*` and the `T_eff`
    normalisation were added after that run because the conservation cell showed
    rate / triple varying with the trace class. Those runs are discarded.

## Outputs

`raw/` holds the build and selftest logs, the append-only search archives
(`state/n<N>-<arm>.jsonl`), `state/finalists-n<N>.json`, the held-out and ablation
JSONL (one line per program with per-target counters), the conservation JSONL,
the per-card receipts and `summary.json`. `README.md` (verdict by the rule above,
written after the cards run), the graph (`Q` ratios with intervals per cell and
the running-best curves) and `REPORT.pdf` follow the `research-visuals` skill;
the speed result also goes to `benchmarks/20261018-blindspots-kernel-search/`
at integration.

## Repair pass R2 (2026-10-18; execution plan only, nothing above changes)

The first run was stopped inside the card sequence. Receipts exist for card 01
(selftest, completed) and card 04 (`n=31`, arm `b`, evaluations 0..191, completed);
they are kept and not repeated. Card 02 and card 03 have no receipt: their
commands were cut off by the executor's 2-minute shell timeout and left a partial
`raw/state/n31-p.jsonl` (151 evaluations) that no receipt covers. It is not used.
Cards 05..18 never ran. The first run proved no instrument bug, so the instrument
(`examples/blindspots_10_kernel_search.rs`, sha256 `f69bbce4...`) is unchanged, and
the question, arms, cells, metric and decision rule are unchanged.

What the repair changes is the card layout, nothing else:

* Every output of a revision card sits under `raw-r2/` (receipts
  `raw-r2/receipt-card-NN-r2.json`); nothing under `raw/` is touched.
* A search is deterministic given its archive (`resume_at` recomputes the same
  programs; the smoke test showed a card boundary changes nothing apart from
  `wall_ms`), so re-running card 02 and 03 work from index 0 reproduces the same
  program sequence. The `b`, `n=31` archive continues from card 04's committed
  archive: card 05-r2 binds it by sha256, copies it to `raw-r2/state/n31-b.jsonl`
  and appends evaluations 192..255 there.
* Chains of 256 evaluations are cut into two pieces of 128 (about 4 to 4.5 minutes
  each, measured at 1.8 to 2.1 s per evaluation with two lanes running); the sum of
  the commands' timeouts of every card is at most 475 s, so no card can exceed
  eight minutes. The first piece writes `raw-r2/state-1/`, the second binds that
  file, copies it to `raw-r2/state/` and continues there; `raw-r2/state/` holds
  the final archives that finalists, held-out, ablation and conservation read.
* Cards are written in stages, each only once the files it binds exist and are
  committed (`make_r2_cards.py STAGE` writes and lints a stage):

| stage | cards | work |
|:--|:--|:--|
| 1 | 02, 05, 06, 07, 08, 10, 12 | `p` n31 [0,128); `b` n31 [192,256) from card 04's archive; `b` n37, n41 [0,128); `c` n31, n37, n41 [0,128) |
| 2 | 03, 19, 20, 09, 11, 13 | the second 128 of `p` n31, `b` n37, `b` n41, `c` n31, n37, n41 |
| 3 | 14, 15, 16, 17, 18, 21 | finalists and held-out n31, n37 (14); held-out n41 and conservation 17/6 (15); ablation n31, n37, n41 (16, 17, 18); conservation 19/7 (21) |
| 4 | 22 | summarize |

| first-run card | revised as |
|:--|:--|
| 01, 04 | completed, kept |
| 02 | 02-r2 (and 03-r2 for the second half of the `p` chain; first-run 03 held the same work) |
| 05, 06, 07 | 05-r2, 06-r2, 07-r2, 19-r2, 20-r2 (the `b` chains, re-cut) |
| 08, 09, 10, 11, 12, 13 | 08-r2, 09-r2, 10-r2, 11-r2, 12-r2, 13-r2 |
| 14, 15 | 14-r2, 15-r2 |
| 16, 17, 18 | 16-r2, 17-r2, 18-r2, 21-r2, 22-r2 (ablation one cell per card; summarize last) |

The finalists, held-out and conservation files are the same objects as in the
first design; `summarize` reads `raw-r2/` instead of `raw/` (`--dir`, `--state`).
Smoke test before freezing (not a result): every arm (`b`, `c`, `p`) at each of
n = 31, 37, 41 for 24 evaluations, the resume from card 04's 192-line archive, and
full-size `finalists`, `heldout`, `ablate`, `conserve` (17/6, 19/7) and `summarize`
on those archives ran to completion with exit 0.

### R2 stage 2 (2026-10-18; execution plan only, nothing above changes)

Stage 1 (cards 02, 05, 06, 07, 08, 10, 12-r2) completed and its archives are committed. Stage 2
(cards 03, 19, 20, 09, 11, 13-r2) is written now, each binding its stage-1 archive
(`raw-r2/state-1/n<N>-<arm>.jsonl`) by sha256. One change from the stage-1 layout, to the card
text only: the copy into `raw-r2/state/` is `cp -t raw-r2/state <bound file>` instead of
`cp <bound file> <destination file>`, so that no argv names a file the card creates and the
after-the-fact lint of `check-relay` passes (card 05-r2 failed that lint, not the run).
Instrument unchanged (sha256 `f69bbce4...`, no bug proved by the first run). Resume smoke
before freezing: each of the six chains resumed from its committed 128-line archive for one
evaluation (index 128) in a scratch directory, exit 0. Stages 3 and 4 are frozen only after
stage 2 has run and its archives are committed (`make_r2_cards.py 3`, then `4`).
