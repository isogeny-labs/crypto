# E11: a restricted ("windowed") degree-4 closure for `d_F` at large `N`

Engine: `examples/hky_window.rs`. Raw records: `e11/validate/*.jsonl` (committed
draws), `e11/scaling/*.jsonl` (fresh draws). Exploratory engineering work; not a
registered experiment, no PREREGISTRATION amendment, and RESULTS.md is not changed
by this note.

**Measured vs projected.** Every number in §3 and §4 is measured on this host
(4 cores, 15 GB; runs used 3 rayon threads). Every number in §5 is a
**projection** and is labelled so. Nothing at `N ≥ 58` was run.

## 1. What is computed, and why it is sound

`V_{F,c}` is the HKY degree-`c` closure: the Macaulay rows `m·f` (`deg ≤ c`),
closed under `g ↦ x·g` whenever `deg g < c`, in the Boolean ring. `d_F` is the least
`c` with `1 ∈ V_{F,c}`.

Fix a column set `M = (all monomials of degree ≤ 3) ∪ M4`, `M4` a set of degree-4
monomials. The restricted closure `W_M` keeps only rows supported in `M`:

- a generator `m·f` is admitted iff its support lies in `M`;
- `x·g` is admitted for `g ∈ W_M`, `deg g < 4`, iff its support lies in `M`.

**Soundness.** By induction every element of `W_M` is in `V_{F,4}`: generators are
Macaulay rows of degree ≤ 4, and `V_{F,4}` contains `x·g` for each of its elements
of degree `< 4`. So `1 ∈ W_M` ⇒ `1 ∈ V_{F,4}` ⇒ `d_F ≤ 4`. Together with a full
degree-3 closure that does not contain `1` (computed as `hky_par` does, all
columns), it certifies `d_F = 4` exactly. If `W_M` stops without `1` the result is
**inconclusive (`d_F ≥ 4`)**, never `≥ 5`: a lower bound needs the unrestricted
`V_{F,4}`.

**No cancellation subtlety.** A product `x·t` leaving `M` cannot be cancelled by
another term of `x·g`: two monomials of `g` with the same product differ by `x`,
and then the product is the larger one, which is already in `g`'s support (so in
`M`). The engine therefore rejects a candidate at its first out-of-`M` term, and
that rejection is exact for that candidate.

**The per-row rule is not the exact closure, and that matters.** The admissibility
of `x·g` depends on the representative `g`, not just on the subspace. Testing only
the stored echelon rows (as `hky_par` does) computes a subspace of the exact
restricted closure, and on 33 of 90 committed `d_F = 4` draws that stalls before
`1` (W(1), §3). The fix is the **exact window pass** (`…x` policies): for a window
`U` with `C(U, 4) ⊆ M4`, compute `L_U = W_M^{<4} ∩ (P_{≤2} ⊕ span C(U,3))`, the low
part whose cubic terms all lie in `U` (an intersection with a coordinate
subspace: echelonise the low rows with the non-local cubic columns ordered
highest; the rows with local pivots span `L_U`), and add `x·g` for `g ∈ L_U`,
`x ∈ U`. All those products lie in `M`, so the pass is sound. Products of rows
already known to be done (local basis rows, which the round loop multiplied by
every variable, and earlier passes' rows) are skipped by keeping, per window, an
echelon basis of the done part and multiplying only a complement. The pass runs
whenever the round loop reaches a fixpoint, until neither produces anything.
(`…v` policies, a per-variable version that is exact for any `M4`, are implemented
but were not validated or used.)

Columns: degree ≤ 3 by the combinatorial number system (as `hky_par`, but on
`u128` masks, so `N ≤ 128`; beyond 64 variables build with `--features
wide-vars`, which `build_decomposition_system` needs; that build type-checks), the
`M4` block in increasing-mask (= colex) order, so the `full` policy reproduces
`hky_par`'s column order exactly. Rows are dense bit rows truncated at their pivot
word, as in `hky_par`; §5 reports their measured fill-in.

## 2. Policies (`M4`)

Blocks as `build_decomposition_system` lays them out: `m` summand blocks of `ℓ`
bits, then `m − 2` intermediate blocks of `n` bits. Links: `{X0, X1, E0}`,
`{E_{i−1}, X_{i+1}, E_i}`, last `{E_{m−3}, X_{m−1}}` (the target is a constant).

| policy | `M4` |
|:--|:--|
| `w1`, `w1x` | degree-4 monomials inside the blocks of one link (`x` = with exact window passes) |
| `w2`, `w2x` | inside two consecutive links (for `m = 3` this is every monomial) |
| `wpx` | inside two blocks of one link, exact passes over those pairs |
| `supp` | the degree-4 monomials of the degree-4 generators `x·f` |
| `full` | every degree-4 monomial (dense; validation arm) |

`w1` and `w2` per-row recall is read off the `w1x`/`w2x` runs: a run that certified
with `exact_passes = 0` is exactly a per-row certification, because the passes
only start once the per-row rounds stall.

## 3. Validation against the dense engine (committed draws)

Inputs: every committed unsatisfiable draw with a dense reference in `e4/`, `e6/`
(A, B), `e7/`, `e8/` (`run_validate.sh`; e4 and e6 A through their ladder input
files, joined on cell and draw). Binary sha256 in `sha256_hky_window`, built at the
commit in `commit_base`. `fall_degree_e11_tools` `compare` and `derived` produce
`validation_summary.{txt,json}` and `derived.txt`.

- **112 draws** joined (90 with dense `d_F = 4`, 22 with `d_F = 3`), `m = 3…8`,
  `N = 11…54`.
- **(i) Degree-2/3 full-closure dims equal `hky_par`'s `per_degree` dims on all 112
  draws**, and the 22 `d_F = 3` draws are refuted at degree 3 by both. 0 mismatches.
- **(ii) Soundness: 0 violations.** Every certification is on a draw with dense
  `d_F ≤ 4`.
- **(iii) Recall over the 90 dense `d_F = 4` draws:**

| policy | certified / 90 | recall |
|:--|--:|--:|
| `w1x` (W(1) + exact window passes) | 90 | **1.000** |
| `w2x` | 90 | 1.000 |
| `w2` per-row | 84 | 0.933 |
| `w1` per-row | 57 | 0.633 |
| `supp` | 33 | 0.367 |
| `wpx` (block pairs) | 6 | 0.067 |

Per cell (`w1x`; dense degree-4 seconds from the committed records, 4 threads,
where they store it; window seconds at 3 threads).

**The seconds columns are uncontrolled diagnostics, not a speed result.** The two
engines ran at different thread counts, on different occasions, in fixed order, with
no A/A noise floor and no interleaved rounds (AGENTS.md, "Measure the noise floor,
then interleave"). No speedup is claimed from them. What this section establishes
is about columns, recall and soundness, which are counts. A speed comparison would
need a matched, isolated, interleaved rerun.

| `m:n:ℓ` | `N` | `d_F = 4` draws certified | `w1x` cols | dense cols | cols ratio | `w1x` s (median) | dense deg-4 s (median) | `w1x` basis MB |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 3:13:5 | 28 | 4/4 | 14,883 | 24,158 | 0.62 | 3.6 | — | 13.9 |
| 3:15:5 | 30 | 4/4 | 20,656 | 31,931 | 0.65 | 9.2 | — | 26.7 |
| 3:18:6 | 36 | 4/4 | 42,778 | 66,712 | 0.64 | 54.0 | — | 114.5 |
| 4:9:3 | 30 | 4/4 | 12,119 | 31,931 | 0.38 | 2.2 | — | 9.2 |
| 4:13:4 | 42 | 4/4 | 46,724 | 124,314 | 0.38 | 47.4 | 314.9 | 135.9 |
| 5:10:2 | 40 | 3/3 | 26,197 | 102,091 | 0.26 | 2.0 | — | 30.6 |
| 5:11:3 | 48 | 4/4 | 46,164 | 213,053 | 0.22 | 35.0 | 533.3 | 132.4 |
| 5:13:3 | 54 | 3/3 | 77,343 | 342,541 | 0.23 | 187.9 | 810.6 | 370.0 |
| 6:8:2 | 44 | 4/4 | 23,840 | 149,986 | 0.16 | 2.0 | 117.2 | 29.8 |
| 6:9:2 | 48 | 4/4 | 33,549 | 213,053 | 0.16 | 3.0 | 99.7 | 59.3 |
| 7:8:2 | 54 | 3/3 | 38,885 | 342,541 | 0.11 | 4.0 | 260.2 | 83.1 |

(All 27 cells are in `validation_summary.txt`; the smaller ones are all 100% too.)
At `N = 54` the `w1x` basis is 0.37 GB against the dense engine's measured 12.6 GB
peak. The recorded wall times are 4–65× lower for `w1x`, but they were taken at 3
threads against 4, uncontrolled (see above). This is not a speed claim.

## 4. Scaling on fresh draws (balanced diagonal, measured)

`newcells` draws with `hky_par`'s rule (same `cell_seed`, `ladder_sample` stream,
seed 20261012, 4 kept unsatisfiable draws per cell). Solutions counted with
`chained_s3_solution_count_fast` at `m = 3` and `chained_solution_count` at `m = 4`
(same counts as `hky_par`'s counters; the general-`m` fast counter was not used).
The dense engine `hky_par` (rebuilt at this commit, sha256 `a0181d2e…`, 3
threads, one process per draw) was then run on the same draws for reference.
`run_scaling.sh`, `fall_degree_e11_tools` `scaling-table` → `scaling_table.json`.

| cell | `N` | dense `d_F` (per draw) | `w1x` certified | `w1x` cols | dense cols | `w1x` s, median (max) | dense deg-4 s, median | `w1x` peak RSS | dense peak RSS | `w1x` basis |
|:--|--:|:--|:--|--:|--:|--:|--:|--:|--:|--:|
| 4:12:3 | 36 | 4 4 4 4 | 4/4 | 28,792 | 66,712 | 26.2 (28.2) | 62.6 | 0.28 GB | 0.84 GB | 52 MB |
| **3:21:7** | 42 | **≥5 ≥5 ≥5 ≥5** | 0/4 (correctly inconclusive) | 79,234 | 124,314 | 307 (316), to fixpoint | 424 | 1.22 GB | 1.70 GB | 384 MB |
| 4:16:4 | 48 | 4 (draw 0; see below) | 4/4 | 89,209 | 213,053 | 462 (556) | 1,691 (draw 0) | 1.58 GB | 4.57 GB | 497 MB |

- Peak RSS: window = `VmHWM` of the one process that ran the cell's four draws;
  dense = `ru_maxrss` of each per-draw process, maximum shown. At these sizes the
  window peak is dominated by fixed buffers (the 1 GiB candidate chunk), not the
  basis.
- **4:16:4 is certified `d_F = 4` on all four draws by the window engine alone**
  (full degree-3 closure without `1`, plus `1 ∈ W_M`); the dense reference was
  needed only for timing. The dense run was stopped by the 2 h background limit
  during draw 1, so it covers draw 0 only (`scaling.log`). On that draw the recorded
  window wall time is 3.7× lower (an uncontrolled diagnostic, not a speed claim), and
  its peak RSS is 2.9× lower.
- **3:21:7: `d_F ≥ 5` on all four draws (dense engine).** The degree-4 dense
  closure reaches its fixpoint at 116,485–118,185 of 124,314 columns without `1`.
  The draws are unsatisfiable by two counters (`chained_s3_solution_count_fast`
  and the root-walking `chained_solution_count`, both 0; `recount_3-21-7.jsonl`),
  so this is not a satisfiable-draw artefact. It is the first cell in this study
  with `d_F > 4`; earlier m = 3 cells up to (18, 6), `N = 36`, were all `d_F ≤ 4`.
  The window engine stops without `1` (71–72k of 79k columns), as soundness
  requires. **Not a registered result** (no amendment covers this cell; four
  draws, one cell; reported here for the main session to decide on).
- The dense reference here is `hky_par` rebuilt at this commit (sha256
  `a0181d2e…`), not the E7 binary (`24cf10d6…`); the window engine reproduced its
  degree-2/3 dims on all 12 draws.
- Binaries: 4:12:3 and 3:21:7 windows ran the validation binary
  (`sha256_hky_window`, `5bed2674…`); 4:16:4 ran the final binary
  (`sha256_hky_window_final`, `ba5510e2…`, which adds the `count` subcommand and
  the unvalidated `…v` policy). The final binary reproduces the validation
  binary's `w1x` results draw for draw: certified, dim, rounds, exact passes and products, and degree-2/3 dims agree on all 112 committed draws (`diff_rerun.txt`: SAME 112 DIFF 0). The committed source is that binary's source after a `rustfmt` pass (formatting only).

## 5. Projection to `N ≈ 100` (PROJECTION, not measured)

`fall_degree_e11_tools` `project` → `projection.json`. Columns are exact counts (`fall_degree_e11_tools` `columns`).
Basis bytes use `cols²/16` (truncated dense rows); the measured `basis_bytes` lies
between 0.51× and 1.04× of that on all 98 certified local runs (below 1 when `1`
enters before the space fills). Peak RSS adds the exact-pass state: a `done`
basis per window of at most `deg3_cols²/16` (1.4–1.7 GB here), transient copies of
the cubic rows during a pass (at most ≈ 3 × `deg3_cols²/16`), and the 1 GiB
candidate chunk, so roughly basis + 7–13 GB. Time is a log-log least-squares fit, per `m`, of the median
`w1x` seconds (3 threads) of the certified local cells against columns,
extrapolated 10–50× beyond the largest measured column count: **an order of
magnitude, not an estimate with an error bar.** Thread scaling beyond 3 threads
was not measured.

| cell | `N` | dense cols | dense basis | `w1x` cols | `w1x` basis | est. `w1x` peak | time fit exponent (cells) | projected `w1x` time, 3 threads |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| 5:25:5 | 100 | 4,087,976 | 1,045 GB | 890,676 (22%) | 50 GB | ≈ 60 GB | 3.3 (4, `m = 5`, ≤ 77k cols) | ~130 h |
| 4:32:8 | 96 | 3,469,497 | 752 GB | 1,390,377 (40%) | 121 GB | ≈ 130 GB | 2.6 (4, `m = 4`, ≤ 89k cols) | ~130 h |
| 3:48:16 | 96 | 3,469,497 | 752 GB | 2,169,913 (63%) | 294 GB | ≈ 300 GB | 3.0 (4, `m = 3`, ≤ 43k cols) | ~2,000 h |

All three also need the full degree-3 closure (147,537–166,751 columns, ≤ 1.7 GB
dense), which is cheap.

**What the projection assumes.** That `d_F = 4` still holds at these cells. If it
does not, `w1x` spends the same resources and returns "inconclusive". 3:21:7 shows
this is a live risk on the `m = 3` diagonal already at `N = 42`.

**Fill-in.** The truncated basis rows are 1–10% ones (`derived.txt`, `density`):
about 7–9% at `m = 3`, 4–7% at `m = 4, 5`, 1–2% at `m = 6, 7`. Sparse rows with
32-bit column indices cost `32 × density` of the dense bytes, so sparse storage
would *raise* memory for `m ≤ 5` (×1.3–3) and only pays at `m ≥ 6`. For the N ≈ 100
targets, dense truncated rows over the restricted columns are the right
representation.

## 6. Verdict

**The restriction idea works as an upper-bound certifier.** W(1) with exact window
passes certified every one of the 90 committed dense `d_F = 4` draws and every
fresh `d_F = 4` draw (8/8), with no soundness violation in 124 draws, at 11–64% of
the dense columns. The per-row rule alone does not (63% recall): the exact pass
is what makes it work.

**Feasibility at `N ≈ 100` (projection).**
- **5:25:5 (`N = 100`): feasible on one large-memory host.** About 0.9M columns,
  a 50 GB basis, ≈ 60 GB peak: a 128 GB instance (e.g. `r7i.4xlarge`, or
  `r7i.8xlarge` for 32 vCPU). Projected ~130 h at 3 threads; a 32–64 thread host
  would shorten that by an unmeasured factor. This cell is the best candidate.
- **4:32:8 (`N = 96`): feasible but heavier.** 1.4M columns, 121 GB basis, ≈ 130
  GB peak: a 256 GB instance (`r7i.8xlarge`). Projected ~130 h at 3 threads.
- **3:48:16 (`N = 96`): not worth running with this engine.** W(1) keeps 63% of
  the columns (a chain of two links has only two windows), 294 GB basis, a 384–512
  GB host, ~2,000 h projected at 3 threads. And the same diagonal already has
  `d_F ≥ 5` at 3:21:7, so the likely outcome is "inconclusive" after the full
  cost.

Each of these first needs the full degree-3 closure (≤ 1.7 GB), and a run must be
built with `--features wide-vars`. The engine is single-host; nothing here is
distributed.

**What this engine cannot do.** It never proves `d_F ≥ 5`. Deciding a cell
where `d_F ≥ 5` (as 3:21:7 now appears to be) needs the unrestricted `V_{F,4}`:
dense 0.75–1.05 TB at `N ≈ 100`. The `x2iedn.32xlarge` / `x1e.32xlarge` class
(≈ 4 TB) could in principle hold it. Time at that size was not measured or
projected.

**Next methods, most promising first.**
1. **Run 5:25:5 with `w1x` on a 128–256 GB host** (gate: rerun `run_validate.sh`
   with the wide-vars build first). Per the projection it is the cheapest way to
   a `d_F = 4` certificate at `N = 100`. It answers only if `d_F = 4` there.
2. **Settle 3:21:7 before scaling further.** Whether `d_F = 5` or more needs a
   degree-5 closure. The dense one is ≈ 975k columns, ≈ 60 GB. A degree-5 W(1)
   closure is ≈ 0.55M columns (an extension of this engine to `c = 5`, not built).
   Also needed: more draws and neighbouring cells (3:24:8, `N = 48`; 4:20:5 is
   `N = 60`, outside the local tier). If `d_F` steps to 5 on the `m = 3` diagonal,
   that matters more than any `N = 100` run.
3. **Lower-bound certificates without the dense closure.** `1 ∉ V_{F,4}` is
   witnessed by a linear functional on degree-≤ 4 polynomials that vanishes on
   `V_{F,4}` and takes 1 at `1` (a degree-4 "pseudo-solution" in the sense of
   polynomial-calculus degree lower bounds, e.g. Razborov's R-operators). If such
   a witness can be built from the chain structure and checked locally, `≥ 5`
   becomes provable at `N ≈ 100`. This is research, not engineering.
4. **Block-local elimination of the intermediate blocks** (the method suggested
   in the brief) is not needed for upper bounds, since `w1x` already has full
   recall. It could reduce the `m = 3` cost, where the large shared block `E0`
   keeps 63% of the columns, but nothing here measures that.

## Files

- `../../../examples/hky_window.rs`: the engine (`ladder`, `newcells`, `count`).
- `run_validate.sh`, `validate/*.jsonl`, `validate.log`: §3 runs (policies
  `w1x,w2x,supp,wpx`); `validate_final/`, `validate_final.log`: the final
  binary's `w1x` rerun; `fall_degree_e11_tools` `diff-rerun` compares the two.
- `fall_degree_e11_tools` `compare` → `validation_summary.{txt,json}`; `derived` → `derived.txt`.
- `run_scaling.sh`, `scaling/*.jsonl`, `scaling.log`, `fall_degree_e11_tools` `maxrss`: §4;
  `scaling-table` → `scaling_table.json`; `scaling/recount_3-21-7.jsonl`.
- `fall_degree_e11_tools` `columns` (exact column counts), `project` → `projection.json` (§5).
- `plot.py` → `window_vs_dense.svg`: degree-4 seconds against columns, `w1x`
  (dots) against dense (squares), with the projected N ≈ 100 column counts marked.
  No PDF was produced (no TeX or SVG converter on this host).
- `commit_base`, `sha256_hky_window` (validation binary), `sha256_hky_window_final`.


The helpers that produced these outputs were Python when the study was run. They are now the
native `examples/fall_degree_e11_tools.rs`, which reproduces every committed output byte for
byte (`NATIVE_TOOLS.md`).
