# Item 15, protocol addendum M: a matched Pollard rho in the item's own binary

Frozen 2026-10-22, before any measured run of the cells below. Owner-approved scope: give item 15's
whole-solve distance-to-rho table a rho reference that can face the validation gate
(`docs/VALIDATION_GATE.md`; `scripts/validation_gate.py`, `check_rho_reference`). This addendum adds
cells and a pricing view. It does not change the item's question, arms, decision rule for the LA
arms, or any recorded result.

## Why

PROTOCOL.md ("Metric and analysis", "Distance remaining") takes its rho from round 4 of the
2026-10-14 genus-3 item: the `rho` records in
`research/ic_improve_20261014/04-genus3-2lp/results/run-20261014/`, produced by
`examples/ic_improve_04_genus3_2lp.rs`. That reference fails the gate on three counts:

1. **`same_binary_or_process`**: the rho ran in another binary, not in item 15's instrument.
2. **One walk per curve** at b = 20, curves 0 to 2 only. Rho cost is heavy-tailed (round 4's
   recorded walks run from 0.42 to 3.95 sqrt(ell) steps), and curve 3 has no rho at all.
3. **Rho's own unpriced work is not priced** (AGENTS.md section 5a). The README priced only the IC
   arms' mod-ell products, at `c_ell` F_q multiplications each, and used the rho record unchanged.

## The algorithm (unchanged; now in the item binary)

Instrument: `research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs` (item 15b), a
copy of item 15's pinned `blindspots_15_genus3_la.rs` (sha256 `73cfa489...5462b`, **not modified**;
its cards and pin stay valid).

* **Rho** is `rho_neg` with `canon`. Item 15's file already contained them, byte-identical to round
  4's, but never called them (`#[allow(dead_code)]`). The algorithm is Pollard rho on the classes
  {P, -P} of E0(F_{q^3}). The walk is r-adding with `--rho-r 64` multipliers. `canon` keeps the
  lexicographically smaller of (y2, y1, y0) and its negative. Distinguished points use
  `dpbits = max(2, floor(log2 sqrt(ell)) - 6)`. A walk restarts after `40 << dpbits` steps.
  Fruitless cycles are checked every L = 64 steps, and an escape doubles the cycle's smallest point.
  `max_steps = 40 sqrt(ell) + 1000`. Each answer is checked against the planted k.
* **Dispatch** is ported from round 4's `main` (`ic_improve_04_genus3_2lp.rs`, the `arm == "rho"`
  block). The rho stream is a copy of `base`, the random state just after the IC target k is drawn.
  Target 0 is k, the IC target. Targets 1 onward are fresh draws `randrange(1, ell)` from the same
  stream copy, in round 4's order. Every walk is costed as round 4 costs it:
  `cost = M_q + ceil(log2 q) I_q`, with no M_ell term.
* **The only edits inside `rho_neg` and `canon`** are lines that add to the native counters (see
  Pricing). Drop every line containing `nat(` from those two functions and they equal round 4's
  text exactly; this was checked with `diff`. The arithmetic, branches, random draws and counters
  are those of round 4.
* `--arms ic2lp,rho` runs the IC streams first (as item 15 does), then the rho walks, in one
  process. `--arms rho` runs rho only. Any other `--arms`, including the default, runs IC only, as
  before.
* **Symmetry check.** For every instance the record now stores j(E0) for
  y^2 = x^3 + a x + b over F_{q^3}, with flags `j_in_Fq`, `j_frobenius_fixed`, `j_is_0` and
  `j_is_1728`. F_{q^3} has no subfield other than F_q, so `j` outside F_q means E0 is not a twist
  of a subfield curve, and no Frobenius endomorphism is available to the walk. `j` not 0 or 1728
  means no extra automorphisms. In that case `strongest_rho_symmetry = "negation"` and the gate
  `kind` is `negation`. If a cell instead prints `REVIEW...`, the matched rho is not the strongest
  available for that cell, and the cell cannot be used as a gate row until a stronger walk is built.

## Cells

* **b = 20, curves 0, 1, 2, 3.** These are the four recorded IC cells (cards 06 to 09).
* **One process per curve.** Each process runs the four recorded IC arms with the argv of cards 06
  to 09, then 16 rho walks: `--rho-targets 16`, with target 0 the shared k.
* **Argv per curve c in {0, 1, 2, 3}** (repository root as working directory):

  ```
  prlimit --as=4294967296 rustc --edition 2021 -C opt-level=3 -C codegen-units=1 \
      -o target/blindspots-15b/c<c>/b15b research/blindspots_20261018/instruments/blindspots_15b_genus3_la_rho.rs
  prlimit --as=4294967296 target/blindspots-15b/c<c>/b15b --bits 20 --curve <c> --kappa 1.3059 --cx 8 \
      --streams r2 --las lpsgebw2@8,lpsgebw2*,lpsgebw4x4@8,lpsgebl4@8 \
      --orders research/blindspots_20261018/15-genus3-la-exponent/orders.csv --no-c1 --no-gp-check \
      --arms ic2lp,rho --rho-targets 16 --rho-r 64 \
      --out research/blindspots_20261018/15-genus3-la-exponent/raw/b20_c<c>_matched.jsonl
  ```

  Then the continuity gates below, then delete the binary. Environment: `RAYON_NUM_THREADS=1`,
  `OMP_NUM_THREADS=1`.
* **Budget.** The estimate is about 4.5 min for the IC arms plus about 16 x 95-130 s for rho, about
  30-40 min per process. The rho tail can make one walk several times longer. A b = 20 rho walk was
  timed at 93.2 s on curve 0, target 0, on the shared 4-vCPU host, with a second job running.
  The run timeout per process is 7200 s. **Deviation:** this breaks item 15's 8-minute card rule
  (PROTOCOL Deviation 1, A5). The rule cannot hold for 16 walks in one process, and the
  same-process layout is the point of this addendum. Total: about 2.3 CPU-h, or about 1 h wall at
  three concurrent single-thread jobs.
* **Not in scope now:** b = 16 to 19 with the same layout (about +1.2 CPU-h). They may be added
  later by a further addendum, never after seeing b = 20 results for the purpose of changing them.

## Continuity controls (gate before any number is used)

1. **IC rows.** Every IC record in `raw/b20_c<c>_matched.jsonl` must equal the recorded
   `raw/b20_c<c>_r2.jsonl` in every counted field. Run
   `b15b gate <arm> raw/b20_c<c>_matched.jsonl raw/b20_c<c>_r2.jsonl` for each of `r2:lpsgebw2@8`,
   `r2:lpsgebw2*`, `r2:lpsgebw4x4@8` and `r2:lpsgebl4@8`. The gate compares k, k_true, correct,
   cost, cost_cl9, rel_cost, la_cost, rel/la/sge/total counters, steps, rows_collected, checks,
   la_cols, la_nnz and matvecs, and each run must print `GATE OK`. Wall times and `vmhwm_kb` are
   descriptive and exempt. A mismatch voids the cell. The recorded rows stay the evidence for the LA
   verdict either way.
2. **Rho target 0 against round 4** (curves 0 to 2). Target 0 must reproduce round 4's record
   exactly: cost and steps on curve 0 are 18866263020 and 232712993, on curve 1 23539588479 and
   290359854, on curve 2 17781900829 and 219336676. This is checked as a control, not as a gate
   condition. The new records stand on their own.
3. **Every rho walk and every IC arm** must return the planted k.
4. **Symmetry:** `strongest_rho_symmetry == "negation"` on every cell.

Smoke results before freezing (binary built outside the repository with the flags above; records
discarded, kept out of `raw/`):

* IC continuity: b = 16 curve 2 and b = 18 curve 3, each with all eight card-01/02 arms. Every
  field except walls equals the recorded `raw/` row (400 of 400 fields per cell), and `gate` prints
  OK on all four b = 20 arm names at b = 18 curve 3. `selftest` passes (120 checks).
* Rho continuity: at b = 16 curve 2, targets 0 and 1 reproduce round 4's two records exactly (cost
  367508185 and 590229926). At b = 20 curve 0, target 0 reproduces 18866263020 exactly, with
  k = 27107377909561436 = the IC k_true.
* Steps over the model sqrt(pi ell / 4): 0.64 and 1.03 (b = 16 c2), 1.01 and 1.61 (b = 18 c3),
  1.20 (b = 20 c0).
* `c_add` = 77, 79 and 81 at b = 16, 18 and 20. This is 60 M_q + 1 I_q at `ceil(log2 q)` I_q, the
  same at all 64 sampled additions. One doubling is 11 more.
* j(E0) lies outside F_q and is neither 0 nor 1728 on the three smoke instances.

## Pricing (AGENTS.md section 5a) and the classification it follows

**Classification (unchanged).** Item 15's frozen PROTOCOL counts `M_q + ceil(log2 q) I_q` as the
arithmetic. It does not count the mod-ell products of SGE and the solver (`CNT[2]`) in that unit; it
prices them in a band ("`c_ell` in {0, 0.1, 9} ... the priced band of AGENTS.md section 5a"). That
is the treatment section 5a gives to unpriced native work. This addendum keeps that classification.
It does not move M_ell into the counted arithmetic, which would be a protocol amendment and an owner
decision. What it corrects is the **unit** of the band.

* **GAE.** One GAE is one generic affine E0(F_{q^3}) addition. It is measured in every process
  (`c_add` in the record) in F_q-mult-eq, the item's unit. Each arm's counted cost converts as
  `counted_fq / c_add`.
* **Native work, every arm, emitted as counters** (`native_names` in the record):
  * Rho: `rho_partition_index`, `rho_canonical_compare`, `rho_dp_test`, `rho_dp_insert`,
    `rho_dp_lookup`, `rho_cycle_compare`.
  * IC: `ic_walk_partition_index`, `ic_relation_hash` (the seen-set), `ic_factor_base_lookup`,
    `ic_lp_graph_hash` (substitution maps, pending lists, large-prime column index, BFS
    adjacency), `ic_row_entry` (sparse entries built or substituted, additions and sign changes
    mod ell), `ic_singleton_filter_entry`, `ic_sge_bookkeeping` (heap pushes and pops, fill scans,
    column-list scans and edits, merge and compaction entry steps).
  * Also native, per the PROTOCOL's classification: `m_ell` = `CNT[2]` (mod-ell products) and
    `i_ell` = `CNT[3]` (mod-ell inversions). The original unit never priced `CNT[3]` at all.
  * The IC graph is a substitution map, not a union-find, so there is no union-find counter.
  * One native op is one table, heap or index operation, or one sparse entry visited, that does no
    counted product. The index loads inside a counted product (one per matrix nonzero in a
    matrix-vector product) belong to that product and are not counted again.
  * Clique removal is not instrumented. It is used only by arm e, which is not run here.
  * The counters sit beside `CNT` and never touch it. Continuity control 1 is the proof that no
    counted cost moved.
* **The view.** For every arm, rho included: `S(w) = counted_fq / c_add + w * native_ops`, with
  w in {0, 0.1, 1} GAE per native op. Here `native_ops` = sum of the NAT counters + `m_ell` +
  `i_ell` (rho has no mod-ell work). The binary writes this as `gae.S` per record, and in
  `matched_rho` as each IC arm's S(w) over the mean S(w) of that process's correct rho walks.
* **The old `c_ell` view is descriptive only.** `descriptive_c_ell_ratio_fq` is
  `(counted_fq + c_ell * m_ell) / mean rho cost`, c_ell in {0, 0.1, 1}, in F_q-mult units. It is
  reported apart and never used to rank.

## Decision rule (frozen)

* **Per curve and per w.** For each IC arm a, d, b, c: `ratio_c(w) = S_IC(w) / mean over the 16
  walks of S_rho(w)`. Only correct walks count; a cell with fewer than 12 correct walks is not
  used.
* **Aggregate.** Geometric mean over the usable curves, with a 95 % bootstrap interval: 2000
  resamples, walks resampled within each curve, then curves resampled.
* **Rank only where the order holds across the band.** "IC below rho" is stated for an arm only if
  the aggregate ratio is below 1 at all of w = 0, 0.1 and 1, with each interval's upper end below 1.
  "IC above rho" needs the mirror condition. Otherwise the arm is reported **unresolved**: give
  the w at which the order changes, and use no wording that suggests a win for either side.
  `order_holds_across_band` in the gate descriptor is true only in the first two cases.
* The rho model `sqrt(pi ell / 4) * cost_per_step` is shown beside the measured mean as a
  secondary column and is never used to rank.
* The gap to rho is reported as distance remaining (CLAUDE.md success criterion), not as a
  pass/fail gate. The LA verdicts of item 15 (arm d ADVANCES (primitive); b, c REGRESS) do not
  depend on rho and are not re-decided here.

**What to expect, stated before the run.** The README says "the IC whole cost is below the single
measured rho run ... at every price in the band". That used `c_ell` pricing (M_ell at 0, 0.1, 1
F_q-mult-eq), left rho unpriced, and compared against one walk per curve. It **may not survive
section 5a.** At about 81 F_q-mult-eq per GAE, M_ell at 0.1 GAE costs about 8 F_q-mult-eq per
product, not 0.1. The README's own slope (arm d: 0.457 + 0.262 x c_ell) then puts IC above rho at
w = 0.1 and 1 GAE before rho's native work (about 4 native ops per step) is priced. In the smoke
records above (two walks, not data), every arm read IC below rho at w = 0 and above it at w = 0.1
and 1, which is unresolved. If the measured cells say the same, the README's distance table must
be cited as a lower-bound comparison (section 5a, "a ratio produced before this rule is relabelled
as a lower-bound comparison"). Its frozen numbers are not rewritten.

## What is unchanged

* Item 15's PROTOCOL, cards 01 to 09, `raw/` records, `analysis.json`, LA verdicts and the pinned
  instrument `blindspots_15_genus3_la.rs`.
* The instances, seeds, kappa, cx, arms and random streams of the IC arms. Every counted operation
  and every existing output field of the instrument; the new fields are added beside them.
* The classification of M_ell as unpriced native work, and round 4's rho algorithm and cost unit.
