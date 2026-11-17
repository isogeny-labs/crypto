# Item 17 protocol: cyclotomic base, counted cost arm up to n = 83

Frozen before any measured run. Source design: section "17" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane, 2026-10-18. Prior result:
`research/cyclotomic_falsifier_20261015/` (yield-neutral at `n = 23`; cost leg
"cyclotomic costlier" by 1.022 and 1.122; `n = 83` "not run").

Class: **component comparison of three factor-base families under one counted
pipeline** (constant level). All three arms use the same group, the same
oracle (the Frobenius-folded pair table) and the same counted unit. Nothing
here is a cross-method row against rho; the one rho number that appears is a
labelled model row (see "Distance to rho"). The section 5a band (native work
priced at 0, 0.1 and 1 GAE per operation) is applied to every arm.

## Question

The cyclotomic base is yield-neutral. Does its algebraic membership test lower
the decomposition oracle's solving degree or cost at the `n = 83` rung?

## What already exists, and what is reused or not

Read before designing: `research/cyclotomic_falsifier_20261015/{PROTOCOL.md,
README.md}`, `research/ic_techniques_20261012/a2_cyclotomic_and_delta/`,
`src/cryptanalysis/{cyclotomic_factor_base.rs,ic_framework/plugins.rs,
ic_boundary.rs}` (the `compact-orbit-scan`, `cyclotomic` and `onb-weight`
plug-ins, `FrobeniusPairTable`, `CountedGroup::mul`), and
`research/blindspots_20261018/16-size-gated-equivariant/PROTOCOL.md` (the
same blockers).

* **Cited, not re-measured.** The `n = 23` cost cells are already measured by
  a committed ecbench session (`cyclotomic_falsifier_20261015/sessions/
  {a1,a0}-n23/`): cyclotomic against compact-orbit-scan at equal columns,
  `S` ratio 1.022 [1.021, 1.023] (`a = 1`, 30 columns) and 1.122 [1.120,
  1.125] (`a = 0`, 17 columns); the yield clause is yield-neutral (15 tests,
  smallest p 0.363). This item does not repeat either. It **uses** their
  stored counters as a hard validation gate for the new instrument (below).
* **Never measured, and the reason of this item.** `n = 83`, and any `n > 62`:
  the plug-ins and `CyclotomicField` are single-word (`n < 64`), and
  `ecbench fb` refuses wide curves.
* **The solver clause was never measured at `n >= 23`.** The `n = 23` solver
  cells timed out at 150 s (`a2_summary_n23*.md`); the only measured solving
  degrees are at `n = 11` (cyclotomic 4 against ONB 3 at `w = 1`).

## The algebraic oracle cannot be posed at n = 83 (BLOCKED, decided from code)

The literal reading of the design ("the oracle's solving degree or cost") is
an algebraic oracle: the descended `m = 2` system with each arm's membership
encoding, solved by `solve_boolean_system`. Four independent reasons it cannot
be measured at `n = 83` (the first three checked by arithmetic in
`--mode degree`, recorded in `raw/degree.jsonl`; the fourth by the build at the
branch base):

1. **Variables.** Monomials are `u64` masks (`koblitz_groebner::MAX_VARS =
   64`). The `m = 2` system has `2n` unknowns for the ONB encoding and `2wn`
   for the cyclotomic unity encoding (`w` roots of unity per summand, each a
   full field unknown with `z^p = 1`): at `n = 83`, 166 (ONB) and 166 / 332
   (cyclotomic `w = 1 / 2`) against 64. At `n = 23` the matched cyclotomic base
   (`w = 2`, 30 columns) already needs 92. `build_membership_system` returns
   `None` in all of these cells.
2. **The membership system alone is dense.** `x^p = 1` with `p = 2n + 1 = 167`
   is the product of `popcount(167) = 5` Frobenius images, each linear in 83
   unknowns: a degree-5 polynomial per coordinate with up to `C(83, 5) =
   2.4 * 10^7` monomials, 83 coordinates, per root of unity.
3. **The degree is not 4.** The design's "degree-4 membership system" is the
   `n = 11` and `n = 131` value (`popcount(23) = popcount(263) = 4`). At
   `n = 23` and `n = 83` it is `popcount(47) = popcount(167) = 5`, while the
   Hamming-weight-ONB system matched to the same columns has degree `w + 1 =
   4` (`w = 3`). `--mode degree` certifies the attained degree by restricting
   `z` to `d = popcount(p)` free coordinates with random constants elsewhere
   and testing the all-variables monomial (a lower bound on the attained
   degree; the product-of-linear-forms construction gives the upper bound).
4. **The `crypto_lib` library does not compile at the branch base** (25
   errors; items 01, 02, 03, 05, 07, 08, 10, 16 record the same), so no
   instrument using the crate, ecbench included, can be built.

**Verdict for the solver leg: BLOCKED**, decided here, not by a measurement.
What would unblock it: monomials wider than 64 bits in `pq_groebner_f2`,
`inherited_f4` and `koblitz_groebner` (shared `src/` files, outside this
item), plus a hybrid or Frobenius-symmetric formulation, since the dense
degree-5 system above does not fit any budget here even with wide masks.

**The measured substitute** is the part of the oracle's cost that depends on
the base: the counted cost of building the base and the pair table (the
pipeline's `factor_base` and `oracle_setup` phases), which is exactly where the
prior cost leg found its difference, for the three families at the three
rungs, with the structural degree table beside it.

## Arms

All three arms run in the same instrument (`examples/blindspots_17_cyclo_cost.rs`,
std only), on `K_a / F_{2^n}`, `y^2 + xy = x^3 + a x^2 + 1`, with the field
polynomial chosen by the repository's rule (smallest-mask sparse irreducible;
`z^23+z^5+1`, `z^47+z^5+1`, `z^83+z^7+z^4+z^2+1`).

| arm | factor base (columns = projected signed Frobenius orbits) |
|---|---|
| **(b) cos**, the incumbent | `compact-orbit-scan`: raw abscissae `1, 2, 3, ...`, lifted, projected by the cofactor, deduplicated by orbit, first `K` columns |
| **(c) cyc** | `cyclotomic:w=W`: the `gamma`-weight-`W` abscissae `sum_{k in S} gamma^k`, `|S| = W`, `gamma` the least-word primitive `(2n+1)`-th root of unity, sorted by word, same lift / projection / dedup, `columns = K` (or all) |
| **(a) onb** | `onb-weight:w=W`: abscissae of Hamming weight exactly `W` in the type-II optimal normal basis `beta_i = gamma^(2^i) + gamma^(-2^i)`, least `W` that reaches `K` columns; at `n = 47`, where `2n+1 = 95` is composite and no type-II basis exists, the least-word generic normal basis |

Matched columns: `K` is the number of columns of the full `w = 2` cyclotomic
base (the production spec `cyclotomic:w=2`), or a fixed `K` with `w = 3`
truncated (`cyclotomic:w=3,columns=K`, as the prior study's `a = 0` cell).
Arms (a) and (b) are cut at exactly `K`.

### Two accountings of the same three builders

Both are reported; they differ only in how the base is enumerated.

* **faithful** (production semantics, the convention of the prior cost leg):
  the plug-ins' loops, re-implemented for any `n`: the whole `C(p, w)` set is
  enumerated and sorted first; every lifted point (both roots) is projected;
  new orbits get the order-`r` check, and `cos` also the Frobenius-eigenvalue
  check; `cyclotomic:w=W` without `columns` scans the whole set.
* **tidy** (the same builders written to do the least work): random subset
  order (so consecutive draws are in different Frobenius orbits), one visit per
  Frobenius orbit of raw abscissae, one root, cofactor projection, orbit key,
  dedup, stop at `K`; no sort, no sanity multiplications.

## Unit

Counted operations, as `ic_boundary::GroupOps` counts them: `G` = group
additions + doublings, with `[k]P` by left-to-right double-and-add (`scalar
mults` kept apart). Unpriced native work `N` is counted in field-level
operations by class (subset-sum additions, sort comparisons, lift inversions /
squarings / multiplications / traces / half-trace squarings, orbit-key
squarings, hash operations, Frobenius maps at 2 operations, negations) and
priced at `c in {0, 0.1, 1}` per operation (section 5a):

`S(c) = G + c * N`.

Phases: **FB** (the base), **SETUP** (the Frobenius-folded pair table: `n K
(K+1)` additions, one canonicalisation and hash operation per addition, the
`member_of` Frobenius maps), **S_ready = FB + SETUP**. The relation phase is
not run at `n >= 47`; a labelled model row is derived (below). Wall time is
descriptive.

## Instrument validation (hard gate, before any cell counts)

`--mode validate` rebuilds eight `n = 23` cells and compares the instrument's
`adds`, `doubles`, `scalar_mults`, `cofactor_projected_points`,
`point_lifts_uncharged`, columns and `pair_table_entries` with the counters
stored in the committed ecbench records of
`cyclotomic_falsifier_20261015/sessions/` (also `pair adds = n K (K+1)`):
`a = 1`: `cyclotomic:w=1`, `compact-orbit-scan:columns=1`, `cyclotomic:w=2`,
`compact-orbit-scan:columns=30`; `a = 0`: `cyclotomic:w=2`,
`compact-orbit-scan:columns=17`, `cyclotomic:w=3,columns=3`,
`compact-orbit-scan:columns=3`. All eight must match exactly. If any does not,
the verdict is INCONCLUSIVE.

## Cells (frozen)

`a = 1` and `a = 0` where the curve has a usable prime-order subgroup
(`r > h`; the `a = 0` Koblitz curve over the degree-47 binary field does not: `r = 7,086,097 < h = 19,861,076`).

| cell | n, a | K | cyc | arms | pair table | nulls |
|---|---|---|---|---|---|---|
| C23a1 | 23, 1 | full `w = 2` (30) | `w = 2` | cos, cyc, onb | yes | 200 |
| C23a0 | 23, 0 | full `w = 2` (17) | `w = 2` | cos, cyc, onb | yes | 200 |
| C47a1-74 | 47, 1 | 74 | undefined (95 composite) | cos, onb | yes | 200 |
| C47a1-300 | 47, 1 | 300 | undefined | cos, onb | yes | 100 |
| C83a1-74 | 83, 1 | full `w = 2` (74) | `w = 2` | cos, cyc, onb | yes | 200 |
| C83a0-93 | 83, 0 | full `w = 2` (93) | `w = 2` | cos, cyc, onb | yes | 200 |
| C83a1-300 | 83, 1 | 300 | `w = 3` | cos, cyc, onb | yes | 60 |
| C83a0-300 | 83, 0 | 300 | `w = 3` | cos, cyc, onb | yes | 60 |

The four `n = 83` cells are the **headline** cells. `n = 23` is the
production-validated anchor and the classifier's positive control; `n = 47`
has only arms (a) and (b), reported descriptively (no cyclotomic base exists
there: `p = 2n + 1 = 95 = 5 * 19`). The expected `K` of the full `w = 2` cells
(30, 17, 74, 93) is read off by the instrument (`--kfull 1`); the table gives
the values seen in the pre-flight.

The structural leg (`--mode degree --n 0`) covers `n in {11, 23, 47, 83}`,
both curves where they exist: columns of the `w = 1, 2` cyclotomic bases, the
least ONB weight matched to `K`, the membership degrees (`popcount(2n+1)` for
the unity encoding, certified; `w + 1` for the ONB encoding), the unknown
counts of the `m = 2` systems and the comparison with 64.

## Targets and controls

* **16 targets per cell and arm.** The costs of FB and SETUP are
  deterministic counts, so the targets are not a source of variance for them;
  they are the **control workload of the oracle**: 16 *planted* targets
  `R = P_i + P_j` (random base points) and 16 *random* subgroup points probed
  by stride (`T_{j+1} = T_j + G`) for a fixed number of probes each.
* **Positive control (oracle).** All 16 planted targets must be found and the
  returned decomposition must re-add to `R`, in every cell and arm. A miss
  makes the verdict INCONCLUSIVE.
* **Negative control (oracle).** On random targets the hits must verify, and
  their number is compared with the exact prediction `y = 2 n E / (r - 1)`
  (`E` the number of pair-table entries) times the probes: reported, not gated
  (the probes of one target are correlated).
* **Negative control (population).** `N` generic bases per cell, built by the
  same code from uniformly random raw abscissae in random order (the null of
  the incumbent: any generic Frobenius-stable base of the same fold and column
  count). They give the spread of `S_FB` for a base with no structure.
* **Positive control (classifier).** On the two `n = 23` cells the rule below
  must classify `cyc` against `cos` as **HIGHER** at `c = 0` under faithful
  accounting, as the prior ecbench session found (1.022 and 1.122); the
  instrument's own `S_ready` ratios are printed beside those numbers.
* **Positive control (accounting).** Faithful against tidy: the instrument must
  see the large enumeration cost of the faithful cyclotomic scan (it is the
  mechanism of the prior "costlier" and the point of the two accountings).

## Decision rule (frozen)

For a cell, an arm `X in {cyc, onb}`, an accounting `A`, a price `c`:

* `S_ready(X) = FB_A(X, c) + SETUP(X, c)` (SETUP of the faithful base).
* **Envelope** `[lo, hi]`: `lo = min(S_ready(cos), q_2.5)`, `hi = max(S_ready(cos),
  q_97.5)`, `q` the 2.5 % and 97.5 % quantiles of `S_FB(null draw, A, c) +
  SETUP(cos)` over the null draws. A deterministic count has no sampling
  interval; the envelope is the interval of "what the incumbent (or any
  generic base) costs".
* **Class:** `LOWER` iff `S_ready(X) < lo` **and** `S_ready(X) / S_ready(cos) <=
  0.99`; `HIGHER` iff `S_ready(X) > hi` **and** the ratio `>= 1.01`; else
  `WITHIN` (a 1 % materiality threshold: below it a constant-level effect is
  not reported as an advance or a regression).
* **Accounting label:** `ADVANCES` if the class is `LOWER` at all three prices,
  `REGRESSES` if `HIGHER` at all three, `MATCHES` if `WITHIN` at all three,
  `TRADES` otherwise (the order does not hold across the band).
* **Cell verdict:** `ADVANCES` iff both accountings are `ADVANCES`;
  `REGRESSES` iff faithful is `REGRESSES` (the tidy label is appended as a
  qualifier); `MATCHES` iff faithful is `MATCHES` and tidy is not `ADVANCES`;
  `TRADES` otherwise.
* **Pooled verdict (arm (c) against (b))** over the four headline cells:
  **ADVANCES (constant)** iff all four are `ADVANCES`; `REGRESSES` iff all four
  are `REGRESSES`; `MATCHES` iff all four are `MATCHES`; `TRADES` otherwise.
  INCONCLUSIVE if the validation gate, the planted-target gate or the
  classifier positive control fails.
* **A2** is recorded **closed** unless the pooled verdict is ADVANCES.
* **Solving degree (structural, no rule class):** reported as lower, equal or
  higher by comparing `popcount(2n+1)` with the matched ONB `w + 1` at each
  cell; the algebraic solve itself is BLOCKED (above).
* Arm (a) is classified and pooled the same way and reported, not decisive.

The mechanical application of this rule is `--mode rule --files ...` of the
instrument (a Rust tool, run by the assess stage on the committed `raw/*.jsonl`).

### Distance to rho (reported, not a gate)

Per cell and price, `--mode rule` prints a **model row, not run**:
`S_model = S_ready + (K + 1) / y * (1 + y + c n)` in `ops / sqrt(r)` (relations
needed `K + 1`, one stride addition and `n` native operations per probe, one
addition per hit; linear algebra ignored), against ecbench's theory
`S_rho-frob = sqrt(pi/2) / sqrt(2n)` (0.185 at `n = 23`, 0.097 at `n = 83`).
The reference is a floor (the measured rho-frob `S` was 2.37 times its theory
at `n = 23`) and its own native work is unpriced, so the ratio is an upper
bound on the distance remaining. It is reported as distance remaining, never
pass or fail. Whole-solve `S` at `n >= 47` is a model because the yield at
`K <= 300` is `10^-4` to `10^-17` per probe.

## Budget

One thread (the instrument is single-threaded), `prlimit --as=4294967296` on
every run, the shared `target/` only (the card builds its binary into
`target/blindspots-17/card-NN/` and deletes it). Estimated CPU from the
pre-flight: about 25 minutes in total (largest job: one `K = 300`, `n = 83`
pair table, 75 s), against the 2 CPU-hour allowance; every card is under 8
minutes of wall time on the pre-flight with room for a 1.5 times slower host.
Seven cards, split by cell and arm.

## Deviations from the frozen design (none loosens it)

1. **Cells.** `n = 47` has no cyclotomic base and (for `a = 0`) no usable
   subgroup; the cell is run with arms (a) and (b), and with `a = 1` only.
   `K` at `n = 83` is the full `w = 2` cyclotomic base (74, 93) and the
   truncated `w = 3` base (300); `n = 23` is the anchor, not a rung measured
   for the first time. Two extra `K = 300` cells were added at `n = 47, 83`.
2. **"Counted oracle cost" is read as `S_ready` (FB + SETUP)** because the
   oracle's own phase (pair table and probe) is arm-independent by
   construction (`n K (K+1)` additions in every arm), so the only base-dependent
   cost is the base. The algebraic oracle is BLOCKED. This is a narrowing of
   what can be measured, stated as such, and the decision rule is stricter than
   the design's (an envelope plus a 1 % threshold plus the whole band, in both
   accountings, instead of "interval excluding 1").
3. **The "degree-4 membership system" is degree 5** at `n = 23` and `n = 83`
   (certified), and lower than the matched ONB system's degree only at `n = 11`.
4. **"16 targets"** are the control workload (planted and random), not a
   variance source for deterministic counts.
5. **Added:** tidy accounting, the null population, the materiality threshold,
   the model rows. **Not done:** F4 or WDSat on any system (BLOCKED), the
   `n = 23` yield and cost cells (cited).
6. **Instrument.** The design says Rust: it is, std only. It re-implements the
   plug-ins instead of calling them (the library does not compile), and is
   validated against the plug-ins' stored counters at eight cells before any
   other cell counts.

## Pre-flight disclosure

Before this protocol was committed the instrument was run on every cell with
small draw counts (nulls 20, probes 50 to 300) and on `n = 83` / `K = 300`
alone for timing, on scratch output not committed. Because the base and pair
table costs are deterministic counts, **the pre-flight already contained the
numbers the cards will reproduce**: the cards are a replication of counts plus
the larger null populations and probe counts, not an independent draw. What it
showed, recorded here without changing any rule:

* Validation 8 of 8 exact. `K_full` of the `w = 2` cyclotomic base: 74
  (`n = 83, a = 1`), 93 (`a = 0`), 30 and 17 at `n = 23`.
* `n = 83, a = 1, K = 74`, faithful: `G_FB` = 496,984 (cyc), 17,466 (cos),
  12,224 (onb); tidy: 2,960 for all three. Pair table 460,650 additions and
  454,286 entries in every arm. Planted targets found 16 of 16.
* An earlier draft of the tidy accounting enumerated subsets in lexicographic
  order; it visited many orbit duplicates (`{0, b}` and `{0, 2b}` are one
  Frobenius orbit) and was replaced by random subset order before any card was
  written. The envelope, the 1 % threshold and the headline set were written
  after seeing these counts; they are neutral about the direction of the
  result and are stated here so a reader can discount them.
* The generic-base null of `S_G` is nearly degenerate (the number of
  projections per column is fixed), so the envelope is narrow and the 1 %
  threshold carries the "within noise" reading.

## What does not count

Changing the cells, the arms, the accountings, the prices, the null size, the
class thresholds or the headline set after a card has run. Any change is a
dated amendment below, and both versions are reported.

## Amendments

(none)
