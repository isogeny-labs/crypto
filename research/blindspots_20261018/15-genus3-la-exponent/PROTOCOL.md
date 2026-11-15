# Item 15 protocol: the linear-algebra exponent of genus-3 double-large-prime IC

Frozen 2026-10-18, before any measured card run. Source design: section "15" of
`research/blindspots_20261018/EXPERIMENTS.md` and its "Rules every item
follows". Written by the experiment-director lane. The design is tightened in
places; the only loosening (the size range, Deviation 1) is forced by the
card rules of this lane and is stated, not hidden. Every deviation and every
filled-in gap is listed at the end, with the probes made while building the
instrument (none kept as data).

Class: **improvement of a component against the incumbent component it would
replace** (a linear-algebra solver and filter for the genus-3 IC after the cubic
GHS transfer). The success criterion (CLAUDE.md) applies: a verified, measured
improvement over the incumbent is a success; the gap to the matched rho is
reported as distance remaining, never as a pass/fail gate; a relabelling or
accounting change is not an improvement. Only the counted operations below
count; wall time is descriptive.

## Question

After rounds 1 to 4 the linear-algebra (LA) phase of genus-3 IC-2LP is the
largest excess over the q^{4/3} relation phase (round 3: "next lever: the
linear-algebra exponent, currently about q^1.52 to 1.54 locally"). Can any of
four LA changes, run on the **same relation sets** as the incumbent, lower the
LA exponent, or lower the whole-solve cost by a constant factor of at least
1.2, on held-out curves?

## What already exists, and what is reused (read before designing)

* **The incumbent** is round 4's `r2:lpsgebw2` (`research/ic_improve4_20261015/
  01-genus3-relations/PROTOCOL.md`, "Question"): large primes kept as columns,
  structured Gaussian elimination (SGE), then block Wiedemann with blocking
  (2, 1), at the round-2 kappa schedule `ln kappa_R2(b) = 0.2669 - 0.0941
  (b - 20)` and `cx(b) = 16` for `b <= 17`, `8` above. It is rebuilt from the
  same code in this item's instrument (a copy of `examples/
  ic_improve4_01_genus3_rel.rs`, R1-driven walk). **Gate 0:** every incumbent
  record regenerated here must equal the committed `r2:lpsgebw2@cx` record on
  the same (bits, curve): `k`, `cost`, `rel_cost`, `la_counters`, `steps`
  (committed files `research/ic_improve4_20261015/01-genus3-relations/results/
  run/b{16..20}_c{0..3}_r2.jsonl`, bound by sha256 in the cards that use them;
  curve 3 exists there only for `b >= 18`).
* **Arm (e), oversampling with clique removal, is already measured and is
  reused, not re-run.** Round 4 item 01 candidate C3, arm
  `r2:lpsgebw2o70c@32` (70 % oversampling, clique removal, SGE `cx = 32`, tuned
  on training curves 5 to 7 in its amendment D2), is in the same committed
  `r2` files, on the same instances, targets and walks, so it is exactly paired
  with every arm below. **No arm (e) cell is re-measured**; the reported
  numbers are those records (round 4 README: LA 0.56x at 2^16 to 0.78x at 2^22,
  whole 0.908x to 0.979x, raising the LA exponent by +0.08 over 16..22). Cells
  without a committed record (curve 3 at b = 16, 17) have no arm (e) row.
* **Block Wiedemann (2, 1) against (3, 1) and (4, 1), and the symmetric-blocking
  argument, are already measured or argued.** Round 2 item 06 README: (2, 1)
  LA 0.889x scalar Wiedemann; m = 3 LA 0.934x; m = 4 worse in training. It
  states that symmetric blocking (n = m) needs n (N/m + N/n) = 2N single-vector
  products and cannot fall below scalar Wiedemann's count single-threaded; it
  was "stated, not measured". **Arm (b) here measures it** (blocking (4, 4))
  because the design names it and the generator cost has never been counted.
* **Scalar Lanczos on the normal equations** is the round-1 incumbent
  (`lanczos`, 2N matrix-vector products, measured dominated by scalar
  Wiedemann in round 2 item 01: `lpsgelz` vs `lpsgewd`). **Arm (c)** is its block
  version.
* **The cx per size** was tuned in rounds 2 to 4 (cx 8 to 32); a **merge-weight
  limit** was never a knob. It is new in arm (d).
* Round 4's README notes 2^22 jobs peaked at 3.9 GB; this lane's cap is 4 GB.

## Instrument

`examples/blindspots_15_genus3_la.rs`, a copy of `examples/
ic_improve4_01_genus3_rel.rs` (relation collection, counters, random streams
and every existing arm unchanged; `src/` is not touched; no crate dependency:
std only, a small JSON writer and reader replace `serde_json` so the file builds
with plain `rustc`, because the library does not compile at this branch base).
Added:

1. `--orders <file>`: a committed table `p, ea, eb -> #E(F_{q^3})` replaces
   the PARI/GP `ellcard` call (identical numbers; `gp` is only needed by the
   `orders` subcommand that generated the table). A missing entry aborts.
2. `--no-c1`: skip the uncharged C1 shadow computation (walk, relations and
   every counter of the base arm are unchanged; only wall time falls).
3. Arm grammar `<base>[o<pct>][c][~<mw>][!<seed>][@<cx>]`:
   * `lpsgebw<m>x<n>`: block Wiedemann, `m` coordinate projections, `n` right
     vectors `Y = [b, n-1 random vectors]` (`lpsgebw<m>` is `x1`, unchanged
     code). The matrix generator is the order basis of `[S(x) | -I_m]`
     (iterative mbasis, shifts 0 on the `n` generator rows and 1 on the `m`
     identity rows, the same recurrence as the (m, 1) code generalised to `n`
     columns). The solution is read as `x[kc] = -sum_{i>=1} g_i . S_{i-1}[kc]`
     for the combination `g` of the `n` shortest generator columns whose
     constant term is `e_1` (the `n x n` matrix of their constant terms is
     inverted, counted). Termination: `sigma >= delta_max + ceil(sum delta / m)
     + 20` over the `n` shortest columns (equal to the (m, 1) rule at `n = 1`).
     Counted: one `M_ell` per stored nonzero per matrix-vector product (`n`
     products per Krylov step), every product of the generator, every inversion.
   * `lpsgebl<w>`: block Lanczos of width `w` on the normal equations of the
     square post-SGE system `A x = b`: block conjugate-direction recurrence
     for `B X = C`, `B = A^T A`, `C = [A^T b, w-1 random columns]`; per
     iteration `D = P^T B P`, `alpha = D^{-1} P^T R`, `R <- R - B P alpha`,
     `P <- R + P beta`, `beta = -D^{-1} (B P)^T R`; if `D` is singular (the
     block Krylov space saturates) the direction columns are cut to a maximal
     independent set (a principal sub-block of a symmetric matrix on any
     maximal independent column set is invertible). Stops when the residual of
     the first column is zero. Every product mod ell and every inversion is
     counted; `A` and `A^T` products count one `M_ell` per stored nonzero each.
   * `~<mw>`: a merge-weight limit in SGE: a column of weight greater than
     `mw` is not eliminated (`mw = 0`: no limit, the incumbent).
   * `!<seed>`: another seed for the solver's random choices (projections,
     right vectors); `!0` is the incumbent's.
4. `selftest`: planted-solution checks of every solver on random sparse
   systems (positive control), a tampered right-hand side (negative control:
   the planted value must NOT be returned), and `lpsgebw2x1` against the
   existing (2, 1) code (same value, same counters).
5. `gate`: record-level comparison against committed round-4 records.
6. `analyze`: the decision rule below, on `raw/` records.

## Arms

All arms run on the **same collection** (one `r2` collection per (b, curve): the
same instance, target, walk, relations and check points); an arm's whole cost
is the collection cost up to its own check plus its own LA counters, booked as
round 4 does. Arms differ only in the LA phase.

| id | arm string | what it is |
|---|---|---|
| a | `lpsgebw2@cx(b)` | incumbent: SGE + block Wiedemann (2, 1) |
| b | `lpsgebw4x4@cx_B(b)` | block Wiedemann with larger, symmetric blocking (4, 4) |
| c | `lpsgebl4@cx_C(b)` | block Lanczos, width 4 |
| c0 | `lpsgebl1@cx_C(b)` | width control: the same recurrence at width 1 |
| d | `lpsgebw2~mw_D(b)@cx_D(b)` | filtering (SGE) with a merge-weight limit and cx tuned per q |
| e | `lpsgebw2o70c@32` | oversampling plus clique removal: **reused** (committed records) |
| n | `lpsgebw2!1@cx(b)` | negative control: the incumbent with other projections |

## Controls

* **Gate 0** (above), on every cell that has a committed record (bit-for-bit).
* **Positive control (solvers):** `selftest` recovers planted solutions with every
  solver at several sizes and block shapes; and `lpsgebw2x1` equals `lpsgebw2`
  value and counter for value and counter.
* **Positive control (detector):** arm (e) is a known improvement in LA (0.56x
  to 0.78x); the analysis must report it below 1 on the cells where it exists,
  with the same numbers as the committed round-4 analysis.
* **Negative control:** arm n (identical algorithm, other random projections)
  must show a ratio to (a) with a CI covering 1 and an exponent difference
  covering 0; at `b <= 18` an `aa` collection (independent walk stream) is run
  too and must cover 0.
* **Correctness gates every arm:** each logarithm is checked as `k phi(G) =
  phi(T)` and against the planted `k`; a wrong or missing `k` fails the arm at
  that check (it retries on 10 % more rows, as the incumbent does), and a cell
  enters a fit only if every compared arm is correct there.

## Cells, order of work, budget

Instances: `q` = next prime above `2^b + 1`; seed `20261014*1000 + 10 b +
curve` (round 4's instances). Kappa and cx(b) are the incumbent's.

* **Training** (curves 5, 6, 7; never in a test fit): `b` in {16, 18, 19}; the LA
  grid: `lpsgebw2~mw@cx` for `cx` in {4, 8, 16, 32} and `mw` in {0, 8, 16};
  `lpsgebw4x4@cx` and `lpsgebl4@cx` for `cx` in {8, 16, 32}. Frozen rules,
  applied mechanically:
  * **T-B**: `cx_B(s)` = the `cx` with the lowest geometric-mean `la_cost` over
    the three curves at training size `s` (ties: the smaller `cx`).
  * **T-C**: the same for `cx_C(s)` over `lpsgebl4`; the width control uses it too.
  * **T-D**: `(cx_D(s), mw_D(s))` = the grid point with the lowest geometric-mean
    `la_cost` among the twelve `lpsgebw2~mw@cx` arms.
  * A test size `b` uses the choice of the largest training size `s <= b`
    (b = 16, 17: s = 16; 18: s = 18; 19, 20: s = 19; 20 is a one-bit extrapolation).
* **Test** (held out): curves 0, 1, 2, 3 at `b` = 16, 17, 18, 19, 20; arms a, b,
  c, c0, d, n; plus `aa` at `b <= 18`.
* **Budget:** one thread per job, `prlimit --as=4294967296`, at most 2 CPU-hours
  in all, every card under 8 minutes of wall time. Measured: see Addendum A.

## Metric and analysis

Unit: round 4's `M_q + ceil(log2 q) I_q + c_ell M_ell` with `c_ell = 1`
primary, over the relation phase and the LA phase; `c_ell` in {0, 0.1, 9} are
reported from the raw counters (the priced band of AGENTS.md section 5a: an
order between two arms is stated only where it holds at `c_ell` = 0.1 and 1; at
`c_ell` = 0 the arms tie by construction because their relation phases are
equal). Per arm: LA cost, whole cost, matrix size and nnz given to the solver,
matrix-vector products, generator products. `analyze` (2000 bootstrap
resamples of curves within each size, paired across arms on (b, curve)):

* LA and whole exponents: OLS slope of `ln cost` on `ln q`, every solve a point,
  `b = 16..20`; paired difference to arm a with a 95 % interval; local exponents
  between consecutive sizes.
* Paired ratio arm / a per size (geometric mean over curves) with 95 % CI, for
  LA and for the whole solve.
* Distance remaining: each arm's local exponent at 19 to 20 minus 4/3; the
  whole-solve cost over the matched rho (round 4 of the 2026-10-14 genus-3 item,
  `research/ic_improve_20261014/04-genus3-2lp/results/run-20261014/`, measured
  `rho` record on the same instances, curves 0 to 2; the rho model
  `sqrt(pi ell / 4)` negation-map steps elsewhere), priced at `c_ell` in {0, 0.1, 1}.

## Decision rule (frozen; verdict vocabulary of EXPERIMENTS.md)

For each of b, c, c0, d (and e from its committed records), against a:

* **ADVANCES (exponent)** iff the arm's 95 % LA-exponent interval lies entirely
  below a's interval (design rule) **and** the paired difference's interval is
  below 0 (tightening). **REGRESSES (exponent)** iff the arm's interval lies
  entirely above a's and the paired interval above 0. Else **MATCHES (exponent)**.
* **ADVANCES (constant)** iff the whole-solve paired ratio at `b = 20` is at most
  1/1.2 = 0.8333 **and** its 95 % CI upper end is below 1 (the design's
  "at least 1.2x whole-solve", tightened by the CI). **REGRESSES (constant)**
  iff the ratio's CI lower end is above 1. Else **MATCHES (constant)**.
* **ADVANCES (primitive)** (reported as a third level, not required for the
  design's rule): LA-cost ratio at `b = 20` with CI upper end below 1.
* **TRADES** when levels point in opposite directions (any level ADVANCES while
  another REGRESSES). The overall verdict is the best of the levels that hold
  with no opposing level; a pure primitive gain is stated as ADVANCES
  (primitive), not as an exponent or constant advance.
* **INCONCLUSIVE** if fewer than 3 curves are correct on any fit size for an
  arm; **BLOCKED** if an arm cannot be run (it then says what blocked it).
* Relabelling is not an improvement: an arm whose matrix-vector or generator
  counts changed only because a counter moved (not an operation avoided) is a
  failed gate; the arms are counted by the same `cnt` calls as the incumbent.

## Deviations from the design (every one)

1. **Size range 2^16..2^20, not 2^16..2^22.** The lane's card rules limit
   each card to under 8 minutes and the whole item to 2 CPU-hours. Round 4's
   own jobs took (with two LA arms, host load about 3) 138 s at 2^20, 420 s at
   2^21 and 1030 s at 2^22 and 3.9 GB; a 2^21 cell with six LA arms does not
   fit one card, and 2^22 does not fit the memory cap. This **loosens** the
   design's range. The consequence is stated in the report: no verdict is made
   at 2^21 or 2^22; the committed `r2:lpsgebw2` and `r2:lpsgebw2o70c@32`
   records at 2^21 and 2^22 are cited for context only. Five sizes remain for
   the exponent fit (the rule asks for at least four).
2. **4 curves per size** are kept (curves 0 to 3), as designed; arm (e) rows
   exist only where the committed records do (not curve 3 at b = 16, 17).
3. **Arm (b)** uses `n = 4` random right vectors with one of them `b`; the design
   says "larger blocking (4, 4)" and gives no further detail. The generator
   algorithm is the generalisation of the repository's (m, 1) mbasis, not
   a fast (sub-quadratic) algorithm; the count is quadratic by construction and
   is reported by part (matrix-vector products vs generator products).
4. **Arm (c)** is the block conjugate-direction (block Lanczos) recurrence over
   GF(ell) with breakdown handling, not Montgomery's GF(2) subspace-selection
   algorithm (which needs word-parallel bit vectors). Width 1 is added as a
   control (arm c0): it isolates what the block width buys.
5. **Arm (d)** has two knobs, `cx` and a merge-weight limit `mw`, tuned per
   training size and applied as a step function (see T-D). "Tuned per q" cannot
   mean tuned on the test instance. A progressive merge-weight schedule as in
   NFS filtering is not built; the cap is the single-parameter version.
6. **Arm (e)** is round 4's C3, tuned there (D2), not re-tuned here.
7. **Fewer relation sets than per-arm re-collection would give:** all arms share
   one collection per (b, curve), which is what "same relation sets across arms"
   requires; it also means a fit's noise is across curves only.

## Disclosures: what was seen before this protocol

* Round 2 items 01 and 06 and round 3, 4 item 01 results and READMEs (above).
* The code of `ic_improve4_01_genus3_rel.rs`. No measurement of any new arm.
* Host: PARI/GP was installed with `apt-get` in this session only to generate
  the orders table; msolve, Magma and Sage are not used.

## Addendum A (2026-10-18, after the instrument was built and smoke-tested, before any card run)

**A1. The training stage and rules T-B, T-C, T-D are withdrawn; arm (d) is tuned per instance.**
Every card of this lane is frozen before it runs and may not read another card's output, so
arm strings that depend on training results cannot be frozen. Arm (d) is now an online rule with
no training data: for each check the SGE filter is run for every `(cx, mw)` of the frozen grid
`cx` in {4, 8, 16, 32} x `mw` in {0, 8, 16, 32}, and the candidate with the lowest modelled solver
cost `(matrix-vector products) * nnz + g * N^2` is solved (arm string `lpsgebw2*`). The
matrix-vector count is exact for the solver (`N (1 + n/m)` for block Wiedemann, `2N` for block
Lanczos); `g` is the dense-algebra constant, 6.0 for (2, 1), 12.2 for (4, 4), 21.0 for Lanczos
width 4 and 6.0 for width 1, read off a pilot on **training curve 5 at b = 18** (a non-test cell;
the measured generator/dense products per `N^2` were 6.03, 12.16 to 12.19, 21.01 and 6.00). The SGE
products of all 16 candidates are charged to the arm (they are cached across arms within a check but
every arm is charged every candidate's counters). The rule reads only the instance's own matrix, not
the outcome of the solve, so it is an algorithm and not tuning on the test result. This replaces
Deviation 5's step function; arms (b) and (c) run at the incumbent's `cx(b)`, and the auto-filter
variants of them are reported as diagnostics (below), so a candidate is never judged with a worse
filter than the incumbent without that being visible.

**A2. Arms as run.**

| role | arm string | cells |
|---|---|---|
| a | `lpsgebw2@cx(b)` | all |
| d | `lpsgebw2*` | all |
| b | `lpsgebw4x4@cx(b)` | all |
| c | `lpsgebl4@cx(b)` | all |
| n | `lpsgebw2!1@cx(b)` | b <= 19 |
| bd, cd | `lpsgebw4x4*`, `lpsgebl4*` (diagnostic) | b <= 18 |
| c0 | `lpsgebl1@cx(b)` (width control, diagnostic) | b <= 18 |
| aa | `lpsgebw2@cx(b)` on an independent walk (stream `aa`) | b <= 19 |
| e | `r2:lpsgebw2o70c@32`, reused from the committed round-4 files | where committed |

`cx(b)` = 16 for b <= 17, 8 above; kappa as round 4's `kappa_R2(b)`: 1.9027, 1.7319, 1.5763,
1.4348, 1.3059 for b = 16..20. The diagnostics and the null reach fewer sizes because of the card
time limit; `analyze` gives them per-size ratios and, with fewer than four sizes of three or more
correct curves, no exponent verdict (INCONCLUSIVE by the rule). The decision rule for b, c, d is
unchanged, with exponent on the LA cost, constant on the whole solve at b = 20, primitive on the LA
cost at b = 20. The null arm `n` changes only which coordinates and vectors the solver projects on;
the counted cost may be identical to arm a (it was in the pilots), which is the expected null.

**A3. Cards.** Nine cards, each independent: it builds its own binary with plain `rustc`
(`prlimit --as=4294967296`, std only, because the library does not compile at this branch base),
runs, gates and deletes the binary. Card 1: selftest, b = 16 and 17 (curves 0 to 3); cards 2, 3:
b = 18; cards 4, 5: b = 19; cards 6 to 9: b = 20, one curve each. Gate 0 is run in every card against
the committed round-4 `r2` file of each cell that has one (bound by sha256). The analysis is run
after the cards, not in a card: `analyze raw/analysis.json 16 20 raw/ <committed r2 dir of round 4>
--rho-dir <2026-10-14 item 04 run dir>`; it reads many outputs and so cannot be frozen before they exist.

**A4. Disclosures made before the cards.** The instrument was smoke-tested on every test cell
(b = 16..20, curves 0 to 3) with the same arms as the cards; those records are discarded (kept out
of `raw/`). What they showed, so the verdicts are read as not blind: every arm returned the verified
log on every cell; gate 0 matched the committed records on every cell that has one; arms (b) and (c)
cost more LA than arm (a) at every size (about 1.5x and 1.7x at b = 16 and 17); arm (d) was within
about 2 % of arm (a). The `selftest` positive controls (planted solutions for every solver) and
negative controls (tampered right-hand side) pass, and `lpsgebw2x1` equals the existing (2, 1) code
value for value and counter for counter. PARI/GP was installed with `apt-get` only to produce
`orders.csv` (475 lines: every order computed while building the 33 instances the item and its
pilot use); the measured runs read the table and do not need `gp`.

**A5. Budget (estimated from the smoke wall times, shared host, load about 3).**
b = 16 and 17: about 3 minutes; b = 18: about 3 minutes per card; b = 19: about 5 minutes per card;
b = 20: about 4.5 minutes per card (walk about 75 s, arms a, d, b, c about 190 s). All cards are
under 8 minutes; the measured total is about 40 CPU-minutes, within the 2 CPU-hour budget.
