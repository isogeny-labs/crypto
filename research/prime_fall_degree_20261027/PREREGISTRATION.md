# Pre-registration: first and last fall degree of prime-field PKM systems

Registered 2026-10-27, before any measured cell ran. Gates (§4) are not
measurements and may run before registration; none had produced a fall
degree when this file was written.

## 1. Question

The binary program (`research/fall_degree_bounds_20261008/`) measured the
Huang–Kosters–Yeo last fall degree `d_F` of Weil-descended Semaev systems over
`F_{2^n}`: 3–4 up to `N = 60`, with the first `d_F ≥ 5` at `m = 3`, `ℓ ≥ 7`.
Nothing comparable exists for prime fields. The PKM tower rounds
(`research/notes/index-calculus/RESEARCH_PKM_TOWER_ORACLE.md`, rounds 1–8)
measured only the F4 solving degree `D`, which grows: `m = 2` reads 4 → 6 to
`N = 20`, `m = 3` reads 5, 6, 6, 7, 7 to `N = 18`, `m = 4` reads 6, 7, 7 to
`N = 16`.

`research/notes/index-calculus/RESEARCH_INDEX_CALCULUS_FACTOR_BASE.md` §3
argues that HKY's machinery "does not port" to `F_p` because `x^p − x` has
degree `p`. That holds for an unstructured factor base. For a PKM tower base it
does not: the tower relations `y_j² = a·y_j·y_{j+1} + b·y_{j+1} + c·y_j + d`
are a Gröbner basis with leading monomials `y_j²` (note Lemma 1). The quotient
`R = F_p[y,u]/T` therefore has a square-free normal form, exactly as the boolean
ring does. Rewriting is degree-non-increasing, so the closure `V_{F,c}`
computed in `R` is HKY's `V_{F ∪ T, c}` (ledger §0, word for word). `R/T` is a
product of copies of `F_p`, so every ideal containing `T` is radical.

So both fall degrees are defined and computable for prime-field tower systems.
This study measures them on the factor bases the repository already generates
(`examples/pkm_tower_pilot.rs`: Kummer, 2-isogeny; Dickson and torus are the
same F4 computation as Kummer, §10.2/§17 of the note, and are not counted as
replications), against the null and naive controls.

## 2. Definitions (one convention: the ledger §0 one)

- `V_{F,c}`: the smallest subspace of `R_{≤c}` containing `F ∩ R_{≤c}` and,
  with every `g` of degree `< c`, every `x·g` for a variable `x`, rewritten to
  normal form. Generators are the normal forms of the pilot's system (the
  `reduced` presentation of note §2.2, which `f4_fp_tower` measures).
- **Last fall degree** `d_F` (inconsistent systems): least `c` with
  `1 ∈ V_{F,c}`. A system whose closure reaches `cmax` without `1` reads
  `d_F > cmax` (censored), and a closure stopped by a limit reads
  `d_F ≥ c` (censored).
- **Operational first fall degree** `ffd`: least `c` with
  `V_{F,c} ∩ R_{≤c−1} ≠ V_{F,c−1}`. The syzygy-based first fall degree
  (Hodges–Petit–Schlather, Caminata–Gorla) is a different number and is not
  measured here.
- `D`: the F4 solving degree (`solving_degree_max`) of `f4_fp_tower` on the same
  system, from the same seeds. Engine choice does not change the systems, since
  the rng is consumed identically.

## 3. Engines

- `src/cryptanalysis/hky_fp_tower.rs` (new): the exact closure, a work queue
  (the binary engine's design), sparse rows over `F_p`, with columns as every
  normal-form monomial of degree `≤ c`. Run as
  `pkm_tower_pilot --engine hky --hky-cmax C`.
- **Capped F4** (`pkm_tower_pilot --engine tower --cap c`). Every row it forms
  is a multiple `u·g` with `deg(u·g) ≤ c` of an element already in
  `V_{F,c}`, so refuting at cap `c` proves `d_F ≤ c`. The converse follows if
  the capped run is a `c`-truncated Gröbner basis. That is the gate in §4 (G3),
  not an assumption. Only after G3 passes may capped F4 extend reach where the
  closure does not fit.

## 4. Gates (before any measured cell; any failure stops the study)

- **G1, unit tests:** `cargo test --lib hky_fp_tower` passes. It covers hand
  cases with known `d_F`, `ffd` and `dim (I ∩ R_{≤c})`.
- **G2, theorem gate `d_F ≤ D`:** on every Kummer/isogeny `m = 2` cell
  `t ≤ 10`, `m = 3` cell `t ≤ 4` and `m = 4` cell `t ≤ 3`, computed with
  `f4_fp_tower`'s `D` on the same systems. Any `d_F > D` is a bug.
- **G3, engine agreement:** on every cell where both run, `1 ∈ V_{F,c}`
  (closure) iff capped F4 refutes at cap `c`, for each `c` from 2 to the
  closure's `d_F`. One disagreement stops the use of capped F4 as a `d_F`
  instrument.
- **G4, planted systems:** on planted (consistent) systems the closure never
  contains `1`, and every generator vanishes at the planted point.

## 5. Cells (prime `P1 = 2013265921 = 15·2^27 + 1`; seed `0x504B4D54`, the pilot's)

Targets: `--planted 0 --random 2`, the recorded rounds' choice (random `x_R`,
inconsistent with overwhelming probability). Engine `hky`, `cmax = 7` unless a
limit stops it. Per-closure limits: `--budget 1800`, `max_cols 2·10⁶`,
`max_entries 6·10⁸`. Censored rows are reported, never dropped.

- **A (N-ladder, `m = 2`):** Kummer, isogeny, null, `t = 1 … 14`; naive
  `t = 1 … 6`.
- **B (N-ladders, `m = 3, 4, 5, 6`):** Kummer and isogeny `t = 1 …` until a
  closure is censored; null-chain at `m = 3`.
- **C (m-ladders, `m = 2 … 10`):** Kummer and isogeny at `t = 2` and `t = 3`.
- **D (reach, conditional on G3):** capped F4 at `c = d_F` of the largest
  closure-measured cell, one size past the closure's reach, per `(kind, m)`.

**Flag, reported and not used to adjust rules:** `λ = |V|^m·2^{m−1}/(m!·p)`
is the expected number of signed decompositions of a random target. A cell
with `|V| < m` is flagged *small base*: the E8 `m = 8`, `n = 6` effect
(refutation over a few points) may make it read low.

## 6. Hypotheses and decision rules (fixed now)

For each `(kind, m)` ladder, take the medians over the 2 targets (the lower
median, as Amendment 12 of the binary program reads it) of `d_F`, `ffd` and
`D` at each completed `N`.

- **H-LFD (bounded last fall degree).** Reads **"`d_F` flat"** if the largest
  and smallest completed `N` have equal median `d_F`, **"`d_F` grows"** if the
  median rises by ≥ 2 across the ladder, and **"mixed"** otherwise.
- **H-FFD (the first fall degree assumption, Petit–Quisquater, transposed to
  PKM systems).** The assumption says `D ≈ ffd`. It reads
  **"FFD assumption fails"** if median `D − ffd` rises by ≥ 2 across the
  ladder, **"consistent"** if it stays within 1 of its value at the smallest
  `N`, and **"mixed"** otherwise. Rows without `D` (past F4's reach) do not
  enter this rule.
- **H-gap (`d_F` versus `D`).** Report the fraction of draws with `d_F < D`
  and the median `D − d_F` per ladder. No pass/fail: it says whether the
  repository's recorded `D` is intrinsic (`d_F = D`) or an artefact of F4's
  strategy (`d_F < D`).
- **H-null (Semaev-specific).** At equal `(m, t)`: **"Semaev-specific"** if
  the null's median `d_F` exceeds the Kummer median at the two largest common
  `N`, **"shape-driven"** if equal there, **"mixed"** otherwise.
- **H-m (fixed `t`, ladder C).** **"flat in m"**, **"grows in m"** (rise ≥ 2)
  or **"mixed"**, on medians, as written. Small-base cells are reported but
  enter the rule.

## 7. Proof obligations (derived, not measured)

- **P1.** Naive control: an explicit lower bound on `d_F` in terms of `2^t`
  (the Yokoyama-type obstruction the factor-base note proves for unstructured
  bases), checked against ladder A's naive row.
- **P2.** An explicit upper bound for `ffd` on Kummer `m = 2`, from the
  reduced `S₃` (degree 2 in `R`), if the data show a constant `ffd`.
- **P3.** Any `d_F` that is constant on a ladder is a conjecture, not a
  theorem, unless proved. HKYY's conditional (binary) has no stated prime-field
  analogue. What the conditional would need over `F_p` is written out in
  RESULTS, not assumed.

## 8. What no outcome can show

Toy primes (`< 2³²`), `t ≤ 14`, `m ≤ 10`. No exponent, no security claim, no
`S`, no leaderboard row. A flat `d_F` here is evidence on an open question, as
in the binary program, not a subexponential algorithm. The memory and cost of a
degree-`d_F` elimination are reported per row, so a flat degree is never
mistaken for a cheap one.

## Amendment 1 (2026-10-28, before any isogeny row and before any random-target row)

The first gate run (G4, planted, Kummer `m = 2`, `t ≤ 6` at `P1`) finished.
Its isogeny part stalled on the builder: `PrimeCurve::point_count` is a
naive `O(p)` loop, and the isogeny tower needs a curve with `2^t | #E`, which
at `P1 ≈ 2·10⁹` takes many counts. Every recorded isogeny cell of the PKM
rounds ran at `P0 = 786433 = 3·2^18 + 1`. Changes:

- **The isogeny arm runs at `P0 = 786433`.** Kummer, null and naive stay at
  `P1`. A Kummer–isogeny difference is therefore also a prime difference.
  The Kummer `D` is the same at `P0` and `P1` on every recorded cell (note
  §10.2), and Kummer at `P0` is added as a control where the isogeny arm runs.
- **The isogeny (and Kummer-at-`P0`) cells run only where
  `λ = |V|^m·2^{m−1}/(m!·p) < 0.1`**, so random targets stay inconsistent
  with high probability: `m = 2, t ≤ 8`; `m = 3, t ≤ 5`; `m = 4, t ≤ 4`;
  `m = 5, t ≤ 3`; `m = 6 … 10, t ≤ 2` (where `λ < 0.1`; computed per cell
  and recorded).
- **G4 runs with `--hky-cmax 5`.** A consistent closure never contains `1`, so
  it runs to `cmax`. The gate needs only that `1` never enters.

No rule of §6 changes.

## Amendment 2 (2026-10-28, before any C, B or null/naive row)

Stage A had produced Kummer `m = 2` rows for `t ≤ 10` at `P1`, and G4 rows.
No other measured row existed. The host is shared, with about 32 GB total and
little free. Changes, all to resources and order:

- Per-closure `max_entries` drops from `6·10⁸` to `3·10⁸` (about 2.4 GB of
  rows). Larger closures read as censored (`d_F ≥ c`), as §2 defines.
- Two chains run at once, with 7 rayon threads each: (1) C, then B;
  (2) A, then the F4 runs `d-A`, `d-B`. A restarts from `t = 1`. Its
  `t ≤ 10` rows are recomputed, and the repeats must agree. Disagreement is a
  reproducibility failure and stops the study.

No hypothesis, rule or cell changes.

## Amendment 3 (2026-10-28, after G1–G4 passed; before any D-cell or t = 1 row)

**Gates.** G1 passed (6 hand cases). G3 passed: 70 capped-F4 runs on
closure-measured systems (`m = 2 … 4`, both primes, `N ≤ 20`), 0
disagreements, `x_R` matched on every one. G4 passed: no planted closure
contains `1`, and every planted point solves its generators. G2 is checked
by `score` as F4 rows arrive.

**Cell D, made concrete.** This is the procedure §5 D allows once G3 has
passed. For a system the closure does not decide, run `f4_fp_tower` uncapped,
which gives `D`, so `d_F ≤ D`. Then run it capped at `D − 1`. If it does not
refute, then `d_F = D` (by the G3-validated equivalence; written `=D (F4)` in
the table). If it refutes, step the cap down. This applies wherever
`f4_fp_tower` runs (at most 4 free unknowns, `m ≤ 6`). The `D` rows for the C
cells are named `DC-*` and are scored like the other `D*` rows.

**Exploratory (not registered; no §6 rule is applied to them).** Cell C at
`t = 1`, Kummer, `m = 2 … 10`, `P1`, closure with `cmax = 12`. Every one of
these is a *small base* cell (`|V| = 2`). They exist because, with
`t ≥ 2`, `m ≥ 7` lies beyond both the closure's limits and the F4 engine's
4-free-variable limit.

## Amendment 4 (2026-10-28, before any C row with m > |V| and before any 1-D row)

**Defect found.** `pkm_tower_pilot` drew the target curve until at least
`m` base abscissae lay on it. With `|V| = 2^t < m` that search never ends.
The first C attempt at `(m, t) = (5, 2)` and its F4 twin looped. They were
stopped and wrote no row. The pilot also forces `t ≥ 2` at `m ≥ 3`, so the
exploratory `t = 1` ladder of Amendment 3 produced rows only at `m = 2`.

**Fix.** For random-target cells (no planted target), the requirement is
`min(max(m, 2), |V|)`. A cell drawable before (`m ≤ |V|`) draws the identical
curve, since the rng is consumed identically. Planted and ladder cells are
unchanged. Cells C at `t = 2, 3` now include `m > |V|` (flagged *small base*,
as §5 already provides). Stage C reruns from `m = 2`, and its repeats must
agree with the rows already written.

**New derived arm (the 1-D specialisation, D6 applied to one summand).**
Evaluating `z_2` at a grid point `ζ^j` is a degree-non-increasing ring map, so
the specialisation lemma gives
`d_F(f) ≥ d_F(q_j)` with `q_j(z) = f(z, ζ^j)`. Here `q_j` is a quadratic in
`x = g·z` on a single Kummer tower, with no root on `μ_K` because `f` has none
on the grid. Two parts:
- (i) For every Kummer `m = 2` closure row, `d_F(q_j)` at 4 grid points.
  This is a check: the row's `d_F` must be at least their maximum.
- (ii) A synthetic ladder `t = 2 … 24` at `P1`: random curve, `g`, `x_2`,
  `x_R`, with `q` checked to have no root on `μ_K`, and 4 draws per `t`.

**Reading for (ii), fixed now:** "1-D grows" if the median rises by `≥ 2`
over the ladder, "1-D flat" if it is constant, "mixed" otherwise. A rise
transfers to `m = 2` as a proved lower bound on the measured draws only.

## Amendment 5 (2026-10-28; stage A had rows for t ≤ 10 only, before any t ≥ 11 row)

- **Operation counts.** Both engines now report `F_p` multiply-adds
  (`muladds`): `f4_fp_tower` in its eliminations, the closure in its
  reductions and products. These are deterministic counts, not timings. Rows
  written before this change lack the field. Stage A is rerun from `t = 1`
  (repeats must agree) so that every A row carries it.
- **A at `t ≥ 11`.** The closure runs to `cmax = 5` only. That gives `ffd`
  and either `d_F ≤ 5` or `d_F ≥ 6`. `d_F` itself comes from the Amendment-3
  procedure: `D` from `f4_fp_tower`, then capped F4 at `D − 1`. The full
  closure at `t = 11` would need about 0.1–0.5 M columns at `c ≥ 6`, which is
  hours per draw on this host.
- **The 1-D arm (Amendment 4 (ii)) read "1-D flat"**: `d_F(q) = 2` on all
  4 draws at every `t = 2 … 24`. DERIVED.md D8 proves it.

## Amendment 6 (2026-10-28, a prediction; before any Kummer `m = 2` row at `t ≥ 11`)

From the `t ≤ 10` medians (3, 4, 4, 4, 4, 5, 5, 5, 5, 6), registered as a
falsifiable conjecture:

**C-step.** For Kummer `m = 2` at `P1`, `d_F = 4 + ⌊(t − 2)/4⌋` for `t ≥ 2`.
- It predicts `d_F = 6` at `t = 11, 12, 13` and `7` at `t = 14`.
- It is refuted by any `t ≥ 11` median that differs.
- `ffd = 4` (theorem D3) is predicted unchanged.

If C-step holds, `d_F` grows linearly in `t`, at one degree per four tower
levels, against `2t + 2` for the no-fall certificate (D4).

## Amendment 7 (2026-10-29, owner-adopted; before any Kummer `m = 2` row at `t >= 12` on the pushed branch; the `t <= 10` rows are being recomputed)

Changes to resources and instruments for stage A at `t >= 11`, a rule for decomposable
targets, and the scorer that implements Amendments 3, 5 and 7. No hypothesis changes, no
§6 rule changes, and the Amendment 6 prediction is unchanged.

**A. Resources.**
- The relaunch runs on Iso Lab EKS (`isolab_k8s_run`), not on the shared 32 GB host of
  Amendment 2. Each job pins `RAYON_NUM_THREADS` and records the instance type.
- For `t >= 12`, each F4 system runs as its own job, with `--targets k`. It takes
  `--budget 86400`, and `--max-dense` in place of `--max-nnz` (§16.2 of the PKM note: `--max-dense`
  bounds the block held in memory). The memory request is 56Gi at `t = 12` and 112Gi at
  `t = 13, 14`. `--max-dense` is the largest multiple of 10⁸ with `4·C <= request − 8 GiB`.
- Closure limits (`max_cols 2·10⁶`, `max_entries 3·10⁸`, `--budget 1800` per closure) are unchanged.

**B. A two-cap bracket for `t >= 12`.** Uncapped F4 at `t = 12, 13, 14` is expected to need
1.7 h, 17 h and 7 days if the recorded 10x-per-level growth holds. That growth is derived in
`research/prime_ic_program_20261029/fall_degree_relaunch/README.md` §2.6. For `t >= 12`, `d_F`
therefore comes from capped F4 at the two degrees that bracket the Amendment 6 prediction
`c* = 4 + ⌊(t − 2)/4⌋`, which is 6, 6, 7 at `t = 12, 13, 14`. This is the procedure §3 and §5 D
allow once G3 has passed.
- Run cap `c* − 1` first. If it refutes, `d_F <= c* − 1`; step the cap down until it does not refute.
  The last non-refuting cap `c` gives `d_F = c + 1`.
- If cap `c* − 1` does not refute, run cap `c*`. If that refutes, `d_F = c*`. If it does not, step the cap
  up until a cap refutes, or until a cap is stopped by a limit (`d_F >= that cap`, censored).
- A capped run stopped by its budget or `--max-dense` is censored: it bounds `d_F` from neither side.
- Uncapped F4 still runs at `t = 11`, and at `t = 12` where it fits the budget, for H-FFD and H-gap.
  Rows without `D` do not enter H-FFD (§6 as written). G2 is checked wherever `D` exists.

**C. Decomposable targets.** For Kummer `m = 2` (every cell of stage A), a draw is certified
inconsistent when the reduced `S₃` has no zero on the grid (D1). The certificate is
`prime_fall_degree_tools ns-degree DIR --t-max 14`, field `grid_zeros`, computed on the
closure row of the same system. A draw with `grid_zeros > 0` is **decomposable**. It is
reported with its relation and excluded from every median and rule. It is not counted as
censored, and it is not redrawn. A cell whose two draws are both decomposable has no median.
For other kinds and `m`, a closure row that never contains `1` while `λ >= 0.01` is reported
as "possibly decomposable", not as `d_F > cmax`.

**D. The scorer.** Before scoring, `prime_fall_degree_tools score` is patched, committed and
reviewed. The patch implements only the following.
1. It reads `CAP-*.jsonl` and the bracket rows `X-*.jsonl` and keys them as the closure rows are keyed.
2. For a system with a closure row censored at `c_last` with no `1`, so `d_F > c_last`:
   - `d_F = D (F4)` if an uncapped `D*` row refuted at `D` and a capped row at `D − 1` did not refute
     (Amendment 3);
   - otherwise the bracket of B;
   - otherwise the closure's one-sided value.
   The source is printed per row (`closure`, `=D (F4)`, `bracket`).
3. The C exclusion.
4. The Amendment 2 repeat check, extended to rows imported from the first run if they are recovered.
5. A line per `t >= 11` giving the C-step reading: **holds at t** (median equals `c*`), **refuted at t**
   (median exact and different, or a bound that excludes `c*`) or **undecided at t** (a bound that
   includes `c*`, or no median).

**E. Disclosures made at registration.**
- *Mac worktree not inspected.* The owner adopted this amendment on 2026-10-29 from a cloud session with no access to the Mac worktree (`RESTART.md`). Any `t >= 11` row later recovered from it predates this amendment. Such rows are listed in RESULTS as pre-amendment rows and are scored under the rules in force before Amendment 7. Any recovered `t >= 12` row is reported separately and never enters an Amendment-7 median.
- *`t = 11` is not a blind test of C-step.* Committed round-2 rows (`research/pkm_tower_round2_20260925/`) already gave `D = 6` at `t = 11`, with cap 5 not refuting, before Amendment 6 was registered (`research/prime_ic_program_20261029/fall_degree_relaunch/README.md`). Only `t = 12, 13, 14` test C-step, and `t = 14` (predicting 7) is the decisive cell.

Source: `research/prime_ic_program_20261029/fall_degree_relaunch/AMENDMENT_7_DRAFT.md` (the rationale notes stay there). Adopted per `research/prime_ic_program_20261029/DECISIONS.md` (D-FD7).

## Amendment 8 (2026-10-29, owner-adopted; before any Kummer `m = 4` row at `t >= 4` in the relaunch)

The `m = 4` extension. The full text, with derivations and estimates, is
`research/prime_ic_program_20261029/fall_degree_relaunch/AMENDMENT_8_DRAFT_m4.md` (§§1-6), adopted
as written, including its optional prediction. The binding parts are restated here.

- **Cells.** Kummer, `m = 4`, chain presentation (2 free unknowns), `P1`, seed `0x504B4D54`,
  `--planted 0 --random 2`, at `t = 4, 5, 6, 7` (`N = 16 ... 28`). The shape-matched null runs at `t = 4, 5`.
- **Instruments.** Closure (`--engine hky`) to `--hky-cmax 8`, run per system (`--targets k`), with
  `max_cols 4·10⁶`, `max_entries = floor((request − 8 GiB) / 8 B)` and `--budget 172800` per closure. Capped F4
  runs only as Amendments 3 and 7 allow, after G3. Order: `t = 4` to `c = 7`, then `t = 5` to `c = 6`, then `t = 5`
  to `c = 7`, then `t = 6` to `c = 6`, then `t = 6` to `c = 7` if the earlier entry counts suggest it fits.
- **Readings** (lower medians over the 2 draws, as in §6):
  - **rise**: `d_F(20) >= 8` with `d_F(16) = 7`;
  - **flat**: `d_F(16) = d_F(20) = 7`;
  - **fall**: `d_F(20) <= 6`;
  - **undecided**: anything else, including a censored `t = 5`, `c = 7` closure.
- **What the readings do.**
  - *Close the lane* (the PKM tower factor base, judged by the HKY last fall degree) if C-step holds at `t = 12-14` and `m = 4` reads *rise*.
  - *Keep it open as a mechanism question only* if `m = 4` reads *flat* or *fall* and C-step is refuted at `t = 12` or `t = 14` with a lower value. Next comes ladder C's growth in `m` at fixed `t`, toward `m = 6 ... 8`.
  - *No change* otherwise.
- **Prediction** (kept): `d_F(16) = 7`, and `t = 5` undecided because the `c = 7` closure will not fit.
- **§6 option, adopted:** widen the closure's arithmetic to 64-bit primes (`u128` products or Montgomery). The
  `m = 4` ladder can then go past `t = 7` at a prime near 2^40. This is engineering. Its cells need their own amendment
  before any row at such a prime exists. Conditioning targets on inconsistency (a decomposability certificate
  for `m = 4`) stays available and is not adopted.
- **Resources.** The decisive `t = 5`, `c = 7` closure is projected at 80-510 GB (derived, low confidence). It runs only
  on a host that fits it: a 256-512 GB host that stays up for days, or a disk-backed echelon. It is not forced into
  the lab's 128 GiB pool. A censored result reads *undecided*, never *flat*.
- No outcome is an `S`, a price or a leaderboard row (§8).

Adopted per `research/prime_ic_program_20261029/DECISIONS.md` (D-FD8).

## Amendment 9 (2026-11-08; resources only; before any relaunch request is submitted, when no relaunch row exists)

The 12-vCPU relaunch on `cpu-spot`. The full text, with its evidence, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_C_cpu12.md`. The binding parts are restated here.

- **Cause.** The largest node the `cpu-spot` pool can create has 16 vCPU. With `kubeReserved` 1 CPU and `systemReserved` 1 CPU it
  has 14000m allocatable, and a 14-vCPU pod needs about 14180m once the daemonsets are added, so it is never scheduled
  (`research/minors_guess_determine_20261107/eks/RELAUNCH_B.md`). The static CPU manager allocates whole cores (`full-pcpus-only`),
  so 12 vCPU is the largest request that fits.
- **Resources.** Every relaunch job requests 12 vCPU. `RAYON_NUM_THREADS` is 12: the launcher sets it from the request
  (`submit-job`), and each job records it in `host.txt` with the instance type (Amendment 7 A). Every job whose time limit exceeds
  90,000 s is pinned to `r6i.4xlarge`, so that it stays within the owner's ceiling of US$40 per job at the Spot price of
  2026-11-08. Jobs of 90,000 s or less are not pinned.
- **Budgets unchanged.** `--budget` stays a wall-clock limit: 1800 s per closure and per F4 system, 900 s in gate 3, 86400 s per
  capped F4 run (Amendment 7) and 172800 s per `m = 4` closure (Amendment 8). The job time limits are unchanged (at most 176,400 s).
  A budget stop is censored as §2 and Amendment 7 B define. With fewer threads, a system close to its budget may be censored where 14
  threads would have finished it. Such a row is reported, never dropped, and reads as a one-sided bound or *undecided*.
- **Instrument note.** The F4 chunk size depends on the rayon thread count, so F4 `muladds` are compared only between rows with the
  same recorded thread count. The closure's batches do not depend on it.
- No hypothesis, rule, cell, seed, memory limit, command or image changes.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (item 1, decisions 4 and 6), with the main session's
decisions of 2026-11-08: keep the wall budgets, and pin `r6i.4xlarge` above 90,000 s.

## Amendment 10 (2026-11-08; stage B high-lambda cell; before `pfd-b` is submitted, when no stage B row exists)

The stage B `m = 6`, `t = 6` rule. The full text, with its derivation, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_A_stageB_m6t6.md` (R2-A). The binding part is restated here.

- **High-lambda cells.** For `m >= 3`, a cell whose `lambda = |V|^m * 2^(m-1) / (m! * p) >= 0.1` at the prime it runs at
  is *high-lambda*. It is run as registered (no flag or command changes), and its rows are written. They are reported in a
  separate table labelled "possibly decomposable (lambda >= 0.1, no certificate)".
- **Excluded from the rules.** A high-lambda cell is excluded from every §6 median and rule (H-LFD, H-FFD, H-gap, H-null,
  H-m) and from the Amendment 6, 7 and 8 readings. It is not counted as censored, not read as "`d_F > cmax`", and not
  redrawn. For H-LFD, "the largest completed `N`" of the `(Kummer, m = 6)` ladder is the largest completed `N` among the
  cells that are not high-lambda (`t <= 5`).
- **Low but non-zero lambda.** Cells with `0.01 <= lambda < 0.1` keep the Amendment 7 C label "possibly decomposable" and
  stay in the rules. `lambda` is computed from `(m, t, p)` and recorded per row; it is not estimated from outcomes.
- **Scope.** At `P1` the rule changes exactly one registered cell: Kummer `m = 6`, `t = 6` (`lambda = 1.517`, derived).
  No hypothesis, instrument, budget, seed, request or command changes. The threshold 0.1 is Amendment 1's.
- **Timing.** Frozen after `pfd-a` (stage A, `m = 2` only, complete) and after `pfd-c` was submitted under Amendment 9.
  Every cell stage C actually runs (`run.sh hky-C`) has `lambda < 0.1` (derived from `|V| = 2^t`): at `P1` every
  `m <= 10`, `t <= 3` cell is below 0.01, and the `P0` cells are limited by Amendment 1's `tmax0` to `m <= 5` at `t = 3`
  (`P0` `t = 3` cells with `m = 9, 10` would exceed 0.1, but they are not run). So this amendment changes no stage C cell.
  No stage B row exists.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (item 1, decision 6: "written as a dated protocol
amendment before its cell runs").

## Amendment 11 (2026-11-09; gate G3's scope; before any stage C G3 row exists and before `pfd-b` is submitted)

The F4 engine's capacity as G3's scope. The full text, with its evidence, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_D_g3_scope.md` (R2-D). The binding parts are restated here.

- **Instrument limit.** `f4_fp_tower` (and `f4_tower_generic`) holds at most `MAX_FREE = 4` free variables; the pilot's tower ring
  has `n_free = m − 2` for every kind, `t`, `p` and non-naive control. So capped F4 runs only at `m <= 6`, as Amendment 3 already
  registers ("at most 4 free unknowns, `m ≤ 6`"). `gate3` selects rows by `N = m·t <= 20` only, so in `pfd-c-20261108` it launched
  capped runs at `t = 2`, `m >= 7`; the pilot panicked (`f4_fp_tower.rs:736`), and `g3` exited 101 with an empty `gates/g3.jsonl`.
- **Scope.** G3 applies as §4 registers it to closure-measured random-target cells (prefix `A-`, `B-`, `C-`, not naive, `N <= 20`)
  with `m <= 6`. A cell with `m >= 7` is labelled **"G3 not applicable (F4 engine capacity)"**. The label depends on `m` only.
- **Use of labelled rows.** Their closure values (`d_F` exact or censored, `ffd`) enter every §6 rule that uses closure values, as
  registered (H-m, H-LFD). No capped-F4 value (`=D (F4)`, the Amendment 7 B bracket, a §5 D cell) exists there or may be made by any
  engine; rows without `D` do not enter H-FFD or H-gap (§6 as written). RESULTS reports them in their own block and discloses that no
  gate checks the closure at 5-8 free variables (G1 cases use 0, 1, 3; G3 reaches 4; G4 runs at `m = 2, 3`).
- **`pfd-c`'s `g3` stage is recorded as "not run (tool defect)", not as a §4 gate failure.** Reasons: G3's domain is "every cell where
  both run"; Amendment 3 registers the F4 capacity as `m ≤ 6`; `gate3` does not skip cells outside it; it panicked before comparing
  anything, so there is no agreement or disagreement evidence. This is the main session's reading of the registered text, made before
  any stage C G3 row exists; the owner may overrule it. The `hky-C` rows stand.
- **Stage C G3 job.** One command job, `pfd-g3c-20261109`, on the frozen image (no new image), cpu 12, memory 56Gi, `seconds` 57600, no
  instance pin. It copies the 19 stage C run files with `m <= 6` from `pfd-c-20261108` (chosen by file name, that is by `(m, t, p)`)
  and runs `prime_fall_degree_tools gate3 --max-n 20 --jobs 2 --budget 900`; pass is exit 0 with ", 0 disagreements". The command is
  frozen verbatim in R2-D §4.4.
- **Order.** The stage C G3 job runs before `pfd-b` and before any `pfd-x` capped run.
- **Stop rule.** Whether a disagreement stops capped F4 only or the whole study, and whether G3 is one gate or one per stage, are not
  decided. If the stage C G3 job reports one or more disagreements (a budget timeout counts as one, as `gate3` defines it), no further
  PKM job is submitted and the question goes to the owner with the rows. If it reports 0 disagreements, the study continues in the
  `R2_READY.md` order.
- **`pfd-b`.** All of its `d-B` and G3 cells are within capacity (`m <= 6`); it runs `g1 hky-B d-B g3` as requested. A budget timeout in
  its G3 at `m = 5, 6`, where no gate run has yet been made, is a risk, not a prediction.
- No hypothesis, §6 rule, cell, seed, budget, memory limit, image or existing command changes.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (item 1, decision 6), with the main session's decisions of
2026-11-09 (rule option 1; `g3` recorded as not run; G3-only job first; the stop rule; the `pfd-b` sentence; the job fields).

## Amendment 12 (2026-11-09; resources only; before the `pfd-b` relaunch is submitted, when no stage B row is admitted)

A price guard replaces Amendment 9's instance pin. The full text, with its evidence, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_E_price_guard.md` (R2-E). The binding parts are restated here.

- **Loss.** `pfd-b-20261108` (pinned `r6i.4xlarge`) was lost to Spot reclamation 16 minutes after its pod started (exit 137,
  "imminent node shutdown", EKS audit log). Nothing from it is admitted. Its one relaunch (owner decision 3) is
  `pfd-b-20261109-r1`.
- **Reason for the change.** Spot placement score, measured 2026-11-09: `r6i.4xlarge` alone 3 (region) and 1 (each zone); eight
  16-vCPU m/r types together 9 (region) and 9, 9, 6 (zones). The pin confines a job of up to 49 h to one Spot pool.
- **No pin.** Jobs with `seconds` above 90,000 are submitted without `instance_type`. Every other field is unchanged.
- **Price guard at start.** Within 30 minutes of the pod's start the node's instance type and zone are read from the EKS audit
  log and the latest Spot price `P` of that type and zone is read. If `(seconds/3600 + 0.25) × (P + 0.0088) > 40`, the job is
  deleted and resubmitted under a new name. That is a cost-guard stop, not a Spot loss; nothing from the deleted job is admitted.
- **Price guard while running.** At every check-in (at most 4 h apart), with `h` the hours since the pod started and `Pmax` the
  highest price read so far: if `h × (Pmax + 0.0088) + (seconds/3600 + 0.25 − h) × (P + 0.0088) > 40`, the job is deleted,
  reported as "stopped by the cost guard", nothing from it is admitted, and whether to run it again goes to the owner.
- Each reading is recorded in the job's receipts and in RESULTS.
- No hypothesis, §6 rule, cell, seed, command, image, memory, cpu, wall budget or job time limit changes. The US$40 per job and
  US$400 total ceilings are unchanged.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (item 1, decisions 3, 4 and 6), with the main session's
decision of 2026-11-09 to enforce the per-job ceiling by observation instead of by the pin.

## Amendment 13 (2026-11-10; the `m = 4` shape-matched null cells; before any `pfd-m4-*` or `pfd-m4n-*` job is submitted, when no `m = 4` row at `t ≥ 4` and no `m = 4` null row exists in the relaunch)

Amendment 8 asks for the shape-matched null at `m = 4`, `t = 4, 5` but gives no command. This amendment fixes it. The full
text, with the source lines and the development check, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_B_m4_null.md` (R2-B). The binding parts are restated here.

- **Independence.** The text was drafted on 2026-11-08 from Amendment 8 and the pilot's source, before any relaunch row
  existed. It uses no recorded row. Since the draft only the resource lines changed (Amendments 9 and 12).
- **Cells.** Four null cells, each paired by `(t, k)` with a Kummer `m = 4` cell of Amendment 8:
  `pfd-m4n-t4-k0-c7`, `pfd-m4n-t4-k1-c7` (`t = 4`, `N = 16`, `--hky-cmax 7`) and `pfd-m4n-t5-k0-c6`, `pfd-m4n-t5-k1-c6`
  (`t = 5`, `N = 20`, `--hky-cmax 6`). `t = 5` at `c = 7` and `t = 6` have no null cell.
- **Command** (the `m4n:` stage of `entry_r2.sh` in the frozen image), with `T`, `K`, `C` from the cell:
  `pkm_tower_pilot --p 2013265921 --ladder-t none --planted 0 --random 2 --engine hky --kinds kummer --m 4 --controls null
  --null-chain --hky-cmax C --hky-max-entries 13958643712 --hky-max-cols 4000000 --budget 172800 --t-min T --max-t-m4 T
  --max-t T --targets K`. The Kummer counterpart differs only in `--controls tower` and no `--null-chain`.
- **Resources.** 12 vCPU, 112Gi, job time limit 176,400 s, no instance pin, price guard of Amendment 12. These equal the
  Kummer counterparts'.
- **Order.** After G3 passed in `pfd-a`, after the Kummer counterpart of the same `(t, k)` has been submitted, and serially.
- **Readings.** No new hypothesis. H-null (§6) is read as written on the `m = 4` ladder at the common completed `N`
  (16 and 20). A cell censored at its `--hky-cmax` gives the one-sided value `d_F > cmax`. When both arms of a pair are
  censored at the same cap the pair reads "mixed" for H-null, never "shape-driven". The null does not enter the Amendment 8
  rise/flat/fall readings; those are Kummer-only. Amendment 10 does not apply (`lambda` at `m = 4`, `t = 4, 5` is far below 0.1).
- **Stated before data.** At `t = 5`, `c = 6` both arms may be censored; the null then adds little at `N = 20`. A null closure
  may use more memory than its counterpart; a limit stop is recorded as censored.
- No hypothesis, §6 rule, Kummer cell, seed, image, wall budget or cost ceiling changes.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (decision 6).

## Amendment 14 (2026-11-10; memory request and `--max-dense` limit of the capped runs at `t = 13, 14` not yet submitted; before any cap-6 row at `t = 13` or `t = 14` exists)

The Amendment 7 B capped jobs at `t = 13` and `t = 14` that have never been submitted request 56Gi instead of 112Gi. The full
text, with its evidence, its disclosure and the independent review, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_F_mem56.md` (R2-F). The binding parts are restated here.

- **Why.** Cell `pfd-x-t13-k0-c6` was launched twice at 112Gi and killed both times by an EC2 Spot interruption (about 10 and
  20 minutes in; no result row). A 112Gi request can only be placed on 128 GiB nodes; since 2026-11-09 20:00Z all four such nodes
  that ran longer than a minute were interrupted within about 50 minutes, and the two 30-minute `t = 12` cap-6 runs on 64 GiB
  nodes were not. Seven nodes: suggestive, not established (`SPOT_LOSSES.md`).
- **Disclosure.** When this was written the main session had seen the `t = 12` rows, the `t = 13` cap-5 rows (both draws: cap 5
  does not refute) and the last line of the lost run's partial log, whose step-26 counters equal the `t = 12` cap-6 log's. No
  cap-6 row at `t = 13` or `t = 14` exists. The rules below do not depend on what that log suggests.
- **Change.** For the covered jobs (today `pfd-x-t13-k1-c6`, `pfd-x-t14-k0-c6`, `pfd-x-t14-k1-c6`, and any later step of those
  brackets): memory request 56Gi; `--max-dense 12800000000`, which is Amendment 7 A's own formula at 56Gi. Nothing else changes:
  image, binaries, stage, `--budget 86400`, 12 vCPU, job time limit, no pin, seeds, draws, caps, bracket order.
- **A run that completes** (`oversize false`, `timed_out false`) is the cell's row, whatever it reads. `--max-dense` only stops a
  run: it is used once, as a stop before the dense block is built (`src/cryptanalysis/f4_fp_tower.rs:1578` at f13a690df0).
- **Re-run rule, fixed before any such row.** A 56Gi run that stops on `--max-dense` (`oversize true`), or is killed by the
  memory limit (container reason `OOMKilled`), is recorded as censored-by-limit and is not the cell's row. The cell is then run
  once at the Amendment 7 A size (112Gi, `--max-dense 27900000000`) under a job name with the suffix `-m112`, and that run's row
  is the cell's row. The trigger is `oversize true` or `OOMKilled` only. Exit 137 is classified by the pod and container reason,
  never by the exit code alone. An `-m112` run that stops on its limit or is `OOMKilled` is censored (Amendment 7 B).
- **A 56Gi run stopped by its budget** (`timed_out true`) is censored as Amendment 7 B says and is not re-run at 112Gi.
- **Spot losses.** Owner decision 3 is unchanged. The `-m112` run is not a Spot relaunch; a Spot-lost job is relaunched at its
  own size; the one relaunch counts per cell, across both sizes. The lost cell `pfd-x-t13-k0-c6` is not relaunched by this
  amendment; if the owner allows another attempt, it runs under it.
- **Cost.** Worst case one extra job per cell, within the US$40 per-job ceiling; the US$400 total is checked before each `-m112`
  submission.
- The two draws of a cell may differ in memory request; each row's request and `host.txt` record it. The `m = 4` requests
  (Amendments 8 and 13) are not touched. No hypothesis, §6 rule, cell, seed or cap changes.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (decisions 3, 4 and 6).

## Amendment 15 (2026-11-14; execution and procedure of stage B: resumable unit jobs; before any stage B row exists)

Stage B's registered invocations are run as short units in numbered, resumable jobs instead of one 48-hour job. The full
text, with its evidence, its disclosure, the runner, its tests and the independent reviews, is
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R2_G_stageB_units.md` (R2-G). The binding parts are restated here. Every
number is `[pcc:unverified]`.

- **Why.** `pfd-b` (`g1 hky-B d-B g3`, time limit 172,800 s) was killed twice by EC2 Spot interruptions, 16 and 31 minutes in
  (`STAGE_B_LOST.md`); no stage B row is admitted. The longest uninterrupted run of this study on the pool since 2026-11-09
  20:00Z is 3.2 hours; a 3.85-hour run was interrupted (`SPOT_LOSSES.md`). Owner decision 8: split stage B into short jobs;
  readings and rules stay as registered.
- **Disclosure.** No stage B row exists or has been seen. When this was written the main session had seen all of stages A and C
  and the `t = 12, 13` capped rows; none enters a rule here.
- **Same systems.** A cell's random stream depends on `(seed, kind, m, t)` only, budgets are per system (F4) or per closure, and
  no state is carried between levels (`examples/pkm_tower_pilot.rs` at f13a690df0). A pilot invocation pinned to one `t`
  computes the systems the registered invocation computes at that `t`, provided the ladder stop is applied between levels.
- **Unit.** One pilot invocation for one `(ladder, t)`: the flags of the ladder's `run.sh` line with
  `--t-min T --max-t-m3 T --max-t-m4 T --max-t T`, and for the `P0` ladders one kind in `--kinds`. Both draws run in the unit.
  25 ladders, 91 units (hky-B 13 and 49; d-B 12 and 42); the table is in R2-G section 3.
- **Rules, fixed before any stage B row.**
  1. Order: levels in increasing `t` within a ladder; ladders in `run.sh`'s order.
  2. Ladder stop: a ladder is closed when a unit's log has the pilot's own line "every run at t = T timed out; stopping". The
     runner makes no judgment of its own.
  3. A unit is done when the pilot exited 0, its files were uploaded and the job's `progress.txt` in S3 has
     `end <unit> exit 0`. A done unit is never run again. A unit found done in two jobs stops stage B (owner).
  4. Resume: unit jobs are numbered and run one at a time; each request is submitted once, and a job that must be repeated
     gets the next number and the earlier one a skip. Two pods under one job name stop stage B (owner). Job `N` is given the result prefix of every earlier job or a
     `skip:<n>:costguard` / `skip:<n>:neverran` for it, each skip with a status receipt. The runner checks that the jobs given
     and the skips (carried forward from earlier jobs) are exactly 1 to `N - 1`, and that each earlier job's runner hash,
     binary hashes and recorded pilot command lines equal its own; otherwise it runs nothing (exit 12). The runner is not
     changed once the first unit job has run.
  5. Interruption: only the unit in flight is lost and it is run again by the next job. "In flight in a job" is a `start`
     line with no `end` and no `abort` line in that job's `progress.txt`; the count does not depend on why the job ended. A
     unit in flight in two earlier jobs is reported lost and goes to the owner (owner decision 3, per unit); the job that
     finds it runs no unit (exit 75). The owner's two answers, and only these: `close:<ladder>:<T>` (the ladder is closed
     there, reported as "lost at t = T") or `allow:<unit>:<K>` with `K >= 3` (lost only when in flight in `K` or more jobs).
     The runner accepts either only for a unit that is lost by the count, and carries both forward.
     The two lost `pfd-b` attempts count toward no unit (owner decision 9, 2026-11-14); each unit starts with a clean count.
  6. Non-zero exit and upload failures, handled mechanically. A unit whose pilot exits non-zero gets one rerun in the next
     job; a second non-zero exit closes its ladder at that level by the runner itself ("tool failure at t = T"). A failed
     upload stops the job; if its `abort` line reaches S3 the unit does not count as in flight. A `P0` isogeny ladder runs
     even if its kummer sibling ended by a tool failure.
  7. Gates: G1 at the start of every job. The `g3` stage is one G3-only job after the last unit: Amendment 11's G3-only
     command with work directory `/work/g3b` and `gate3 /work/g3b pkm_tower_pilot --max-n 20 --jobs 2 --budget 900`; 12 vCPU,
     56Gi, 57,600 s. Its inputs are the row files of the done units of the jobs that count: the last unit job and the jobs
     named in its `prev:` line, never a skipped job; `progress.txt` decides which units are done. Amendment 11's scope and
     stop rule apply.
  8. Stage C repeats: for the units that repeat a stage C cell, `x_r`, `curve` and `tower` of every row must equal stage C's
     admitted row; a difference stops stage B. The outcome comparison is the frozen scorer's, with no exception added.
  9. Cost guard: Amendment 12 applies to every unit job. A job stopped by the cost guard is skipped (`skip:<n>:costguard`),
     never given to a later job, and nothing from it is admitted.
  10. Resources and limits: 12 vCPU, 56Gi, no pin, 172,800 s. US$40 per job and US$400 in total are unchanged. At most 12 unit
      jobs without going back to the owner.
- **Runner.** `research/pkm_falldegree_unblock_20261107/stageb_units_runner.sh`, SHA-256
  `1c2229386976e8d6f22e5f243b269a5d23d588d0e105d6cb530c9a5488157a79`. Each job's command carries the runner text in pieces
  behind a one-line start command (SHA-256 `572ce3fb23e57416732a64a7cc25523c5ff84bd8496a7dd59455e07b1b6bcade`) that
  rebuilds the file and checks the runner's SHA-256 before running it; the launch card checks the start command's hash.
- **Scoring.** For the jobs that count (rule 7), a ladder's unit files are concatenated in increasing `t` into the registered
  file name (for `P0`: kummer units, then isogeny units); the per-invocation summary lines are dropped. No hypothesis, §6
  rule, cell, seed, flag, limit, budget, image or binary changes.
- **What is different.** Each unit is a separate process, so wall seconds near a budget can differ from a long-lived process;
  rows of one stage can come from several jobs and nodes; ladders can be closed by rules 5 and 6, which the registered job
  cannot do because it has no resume. These are stated in R2-G section 5 and get no special treatment in scoring.

Registered per `research/open_leads_execution_20261108/DECISIONS.md` (decisions 3, 4, 6, 8 and 9).

## Amendment 16 (2026-11-14; stage B's execution route: one reserved RunPod CPU pod; before any stage B row exists)

Route only. The text is the R3 draft (`research/pkm_falldegree_unblock_20261107/AMENDMENT_R3_RUNPOD.md`, commit 218b1226d1)
narrowed to stage B, with the two changes listed under "Differences from R3". Every number is `[pcc:unverified]`.

- **Numbering.** Amendment 15 is R2-G (stage B as resumable unit jobs), frozen in PR #4156, which was not merged when this
  was written. This amendment is numbered 16 so that the two do not collide.
- **Owner decision (2026-11-14, ~22:20Z, in session 01TCbLH6osebpnEbutNogBzD).** Asked which cells to run on reserved
  RunPod pods, the owner chose "Stage B m=3–6", described in the question as replacing PR #4156's EKS unit-job route.
  The EKS route had not started: card U1-1 stopped at its pool check (`AccountLimitExceededException`, commit 9ef0ddb37d
  on PR #4156's branch).
- **State before this run (measured).** No stage B row is admitted. The two `pfd-b` attempts were lost to Spot
  interruptions, and their partial rows were never opened (`STAGE_B_LOST.md`). U1-1 never ran. The session that wrote
  this has not seen any stage B row.
- **What runs.** The registered stage B command as one job: `entry_r2.sh g1 hky-B d-B g3`. This is the same stages,
  flags, budgets, limits, seeds and draws as `pfd-b` (Amendments 9–12), `RAYON_NUM_THREADS = 12`.
  - Amendment 15's unit packing exists only to survive Spot interruptions, so it is not used on a reserved pod.
  - The unit and single-job forms compute the same systems with the same budgets, with the ladder stop applied in-process
    (R2-G §2).
- **Host.** One reserved RunPod CPU pod (`interruptible: false`), base image `python:3.12-slim`, at least 56 GiB of memory
  (the registered request). Its vCPU count, memory, `costPerHr` and id are recorded from the API's pod record and from
  `host.txt`.
- **Binaries.** The static musl builds of `f13a690df016d0a457bd40579fbb9d054f486adf`, rebuilt with the recipe of
  `image/build_layer_r2.sh`.
  - Every file must have the SHA-256 recorded in `image/image.json` (`study_binary_sha256`). The runner refuses to start
    on any mismatch.
  - `entry_r2.sh` runs byte-identical. Its `aws s3` calls go to the runner's shim, which copies into the pod's `/results`.
- **Cost guard.** Owner decision 4 is unchanged: US$40 per job and US$400 for the study.
  - A job whose `costPerHr × elapsed` reaches US$40 is stopped and recorded as censored-by-cost.
  - The registered 172,800 s job limit also applies.
- **Disclosure: wall-clock budgets depend on the host.**
  - Every closure and F4 system carries a wall-clock `--budget` (1800 s). A host faster or slower than EKS's
    `r6i.4xlarge` can therefore censor a different set of systems.
  - Degrees and multiply-add counts do not depend on the host; whether a system finishes within its budget does.
  - Each censored row is reported with its host, as §2 defines, and is never dropped.
- **Disclosure: missing safeguards.** No S3 resume checkpoint could be pushed before launch: this container has no
  checkpoint upload token and no AWS credentials. The `bench.record/v1` raw files are hashed into the record, and their
  S3 upload is pending.
- **Unchanged.** No hypothesis, §6 rule, reading, cell, seed, draw, budget, `max_entries`, `max_cols`, binary or stage
  changes. The t = 13/14 and `m = 4` cells are not part of this amendment.

**Differences from R3.**
1. Stage B only.
2. The runner (`runpod/server.py`) gains a chunked upload endpoint. The RunPod HTTPS proxy rejects request bodies of a few
   MB (measured in `research/runpod_kit_20261112/README.md`). The SHA-256 check before `/start` is unchanged.

Registered per the owner's choice above.

## Amendment 17 (2026-11-14; stage B's binaries: identity by reproduction, not by hash; before any stage B row exists)

This amends Amendment 16's binary rule. Every number is `[pcc:unverified]`.

**What happened (measured).**
- A rebuild of `f13a690df0` in this container used the recipe of `image/build_layer_r2.sh`: rustc 1.97.0, musl,
  `-C target-cpu=x86-64`, `--locked`. `pfd_grid_zeros` came out bit-identical. `pkm_tower_pilot`,
  `prime_fall_degree_tools` and `hky_fp_tower_check` did not.
- The three that differ link C code built with this container's `musl-gcc`/gcc 13.3.0 (`ring`, `libsqlite3-sys`,
  `blake3`); `pfd_grid_zeros` does not. The rebuild embeds no build path.
- The frozen image cannot be copied out of ECR here (no AWS credentials). Hashes and toolchain:
  `research/pkm_falldegree_unblock_20261107/runpod/gate17/`.

**The rule, replacing Amendment 16's hash rule for stage B.** A rebuilt binary may stand in for the frozen one only if it
reproduces committed rows of the frozen image exactly. That means every deterministic field: `x_R`, curve, last fall
degree, `ffd`, the fall degrees, closure multiply-adds and column counts, F4 `D` and F4 multiply-adds. Timing is excluded.
The gate runs the registered flags of `run.sh` (rayon 12):

- **Stage A, Kummer `P1`, Kummer and isogeny `P0`, closure and F4, `t ≤ 8`** against
  `results/R9/analysis/pfdA_summary.json`: **48 draws, 432 checks, 0 differences** (measured,
  `gate17/stageA_compare.txt`). The null and naive arms were not run.
- **Stage C, `t = 2`, `m = 3, 4, 5`, both primes, closure** against `results/C9/analysis/pfdC_summary.json`:
  **18 draws, 126 checks, 0 differences** (measured, `gate17/stageC_compare_m3to5.txt`). These are the same systems as
  stage B's `t = 2` points.
- **The `m = 6` cells** were still running on this 4-vCPU host at registration. Their comparison is reported with the
  stage B rows whatever it reads. A wall-budget censoring on this host is a host effect, not a binary difference, and is
  reported as such.
- **G1** passes on the rebuild: `G1: 6 cases pass`.

**Consequences.**
- The runner's SHA-256 gate checks the uploaded files against the gated rebuild's hashes (`gate17/rebuild_binaries.sha256`)
  instead of `image/image.json`.
- `entry_r2.sh` and `run.sh` still match `image/image.json` byte for byte.
- `host.txt` will carry the rebuild's hashes, and RESULTS names them.
- Any stage B row whose deterministic fields later disagree with a frozen-image row of the same system is a reproducibility
  failure (Amendment 2's stop rule).
- No hypothesis, §6 rule, reading, cell, seed, budget or limit changes.

## Amendment 18 (2026-11-15; the `t = 13` draw 0 and `t = 14` capped cells on reserved RunPod pods; before any cap-6 row at `t = 13` draw 0 or at `t = 14` exists)

Route only, for the cells that Amendment 16 left out. Every number is `[pcc:unverified]`.

- **Owner instruction (2026-11-15, ~03:00Z, session 01TCbLH6osebpnEbutNogBzD).** "also launch the t=13/14 cells on
  runpod". This is the non-Spot capacity named in owner decision 10 (PR #4156).
- **Cells and their order.** These are the Amendment 7 B bracket steps, with Amendment 14's limits:
  - `pfd-x-t13-k0-c6-rp`: `cap:13:0:6:12800000000`;
  - `pfd-x-t14-k0-c6-rp` and `pfd-x-t14-k1-c6-rp`: `cap:14:K:6:12800000000`.
  - Each runs `--budget 86400` with the frozen flags of the `cap:` stage.
  - The next step of a bracket follows Amendment 7 B from the row, and is run the same way:
    - cap 7 if cap 6 does not refute at `t = 14`;
    - cap 5 is already measured.
  - Amendment 14's re-run rule is unchanged: `oversize true` or a memory kill gives an `-m112` run with `--max-dense
    27900000000`.
- **Host.** One reserved RunPod CPU pod per cell, at least 56 GiB, `RAYON_NUM_THREADS = 12`. The binaries are those
  admitted by Amendment 17. The runner and client are those of Amendment 16.
  - The registered capped requests named `entry.sh`. The runner installs `entry_r2.sh`, which is `entry.sh` plus the
    `m4n:` stage only.
  - The `cap:` stage text is identical in the two files (checked with `diff` at registration).
- **Cost.** Owner decision 4 applies: US$40 per job and US$400 for the study.
  - Each pod's `costPerHr` is recorded at creation.
  - A job reaching US$40 is stopped and recorded as censored-by-cost.
  - At US$1.04/h the 86,400 s budget costs about US$25, so the cost stop should not bind.
- **Disclosure.**
  - The session registering this has seen the committed rows:
    - `t = 13`: draw 1 `d_F = 6`; draw 0 cap 5 does not refute;
    - `t = 14`: cap 5 does not refute on either draw (`RESULTS_T13_BRACKET.md`, `RESULTS.md`).
  - The bracket is mechanical (Amendment 7 B), so that knowledge leaves no choice open.
  - The Amendment 16 disclosure applies here too: the wall-clock budget makes censoring depend on the host.
- **Unchanged.** No hypothesis, §6 rule, reading, cell, seed, cap, budget or `--max-dense` changes. The C-step reading
  (Amendment 7 D.5) is applied to these rows as registered.

## Amendment 19 (2026-11-15; the `m = 4` cells of Amendments 8 and 13 on reserved RunPod pods; before any `m = 4` row at `t ≥ 4` and before any `m = 4` null row exists)

Route only, plus a study-level spend guard. Every number is `[pcc:unverified]`.

- **Owner instruction (2026-11-15, ~04:20Z, session 01TCbLH6osebpnEbutNogBzD):** "also launch the m=4 cells on runpod".
- **Cells.** Each runs its registered `entry_r2.sh` stage verbatim, with `--budget 172800` per closure:
  - Kummer (Amendment 8): `m4:4:K:7:13958643712:4000000` and `m4:5:K:6:13958643712:4000000`, for `K = 0, 1`;
  - shape-matched nulls (Amendment 13): `m4n:4:K:7:13958643712:4000000` and `m4n:5:K:6:13958643712:4000000`.
- **Host and binaries.** One reserved RunPod CPU pod per cell, 16 vCPU / 128 GB. That is above the registered 112Gi
  request, and `max_entries` is the registered value. `RAYON_NUM_THREADS = 12`. The binaries are those admitted by
  Amendment 17; runner and client are as in Amendment 16.
- **Order.** Amendment 13's order is kept for submission: each null is submitted after its Kummer counterpart of the same
  `(t, K)`. Amendment 13's "serially" existed because the EKS pool admitted one job at a time. Here the eight jobs run
  concurrently on separate pods, and no job's systems or limits depend on another's.
- **Per-job cost.** US$40 per job (owner decision 4). At US$1.04/h that binds at about 38.5 h, before the 49 h job limit.
  A job stopped by it is recorded as censored-by-cost.
- **Study-level spend guard (owner decision 4: US$400 for the study).**
  - The worst case of everything launched under Amendments 16, 18 and this one is US$435, if every job ran to its cost
    stop. That exceeds the ceiling.
  - So actual RunPod spend (`costPerHr ×` elapsed, summed over all pods of the study) is checked at every check-in.
  - When it reaches US$370 (US$30 kept for the earlier EKS jobs, which together cost less than that), the running null
    cells are stopped first, then the `t = 5` Kummer cells. Each stopped job is recorded as censored-by-cost, and the
    owner is told.
- **Disclosure.**
  - No `m = 4` row at `t ≥ 4` and no `m = 4` null row has been seen by this session.
  - The Amendment 16 disclosure applies: the wall-clock budget makes censoring depend on the host.
- **Unchanged.** No hypothesis, §6 rule, reading, cell, seed, cap, budget, `max_entries` or `max_cols` changes. Amendment
  8's readings and Amendment 13's H-null rule apply as registered. The `t = 5, c = 7` and `t = 6` cells stay out (a limit
  change, R3 §3).

## Amendment 20 (2026-11-15; resources only; the study's dollar ceiling)

- **Owner instruction (2026-11-15, ~04:23Z, session 01TCbLH6osebpnEbutNogBzD):** "raise the ceiling to $500".
- **Change.**
  - The study ceiling of owner decision 4 rises from US$400 to US$500. The per-job ceiling stays at US$40.
  - Amendment 19's spend-guard threshold rises from US$370 to US$470, keeping the same US$30 reserve for the earlier
    EKS jobs.
- **Effect (derived).** Every job launched under Amendments 16, 18 and 19 together costs at most US$435 at its per-job
  stop, plus less than US$30 already spent on EKS. That is below US$470, so the guard cannot stop a running job.
- **Recorded.** No row of any of these jobs existed when this was written.
- No hypothesis, rule, cell, limit or budget changes.

## Amendment 21 (2026-11-15; stage B's gate G3: a tool race, recorded and re-run with one worker; before any stage B rule is read)

Procedure only. Every number is `[pcc:unverified]`.

- **What happened (measured).**
  - Stage B's job (`pfd-b-rp`, Amendment 16) ran `g1 hky-B d-B g3`. Every `hky-B` and `d-B` stage ended `exit 0`.
  - `g3` (`prime_fall_degree_tools gate3 ... --max-n 20 --jobs 2 --budget 900`) panicked after 17 min, with exit 101 and
    "capped rows: No such file or directory" at `examples/prime_fall_degree_tools.rs:524`.
  - Before the panic it had written 122 capped comparisons to `gates/g3.jsonl`, all `agree`: 0 disagreements and no
    `G3 DISAGREE` line.
- **Cause (read from the frozen source, f13a690df0, lines 507–525).**
  - `gate3` names each capped run's temporary file by `(p, kind, m, x_r, cap)` only.
  - The `B-m3-null` systems share `p`, `kind`, `m` and `x_r` with their Kummer counterparts. With two workers, one worker
    can delete the file the other is about to read.
  - This is a defect in the tool, not a disagreement.
- **Reading, following Amendment 11's precedent.** Stage B's `g3` stage is recorded as "not completed (tool race)", not as
  a §4 gate failure. Its 122 rows are kept as evidence and are not used as the gate.
- **Re-run.** `prime_fall_degree_tools gate3 <stage B directory> <pilot> --max-n 20 --jobs 1 --budget 900`.
  - It uses the same binaries (Amendment 17), the stage B rows as fetched, and a fresh `gates/` directory.
  - One worker removes the race. The capped runs are independent, so the worker count changes no computation.
  - It runs on this session's host (4 vCPU), which is a host disclosure as in Amendment 16. A budget timeout still counts
    as a disagreement, as `gate3` defines it.
  - If any item disagrees, it goes to the owner with the rows, following Amendment 11's stop rule.
- No hypothesis, §6 rule, cell, seed, budget or limit changes.

## Note on numbering (2026-11-15)

Two sessions froze amendments to this file in parallel. Amendments 15–21 above came from PRs #4168/#4173 (merged first).
Amendments 22–29 below were frozen on branch `aburan28/loving-heisenberg-rbzzbu` as its 15–22, before any row of their jobs, and are renumbered
here by +7 (branch 15→22, 16→23, 17→24, 18→25, 19→26, 20→27, 21→28, 22→29), cross-references included. Files written on that branch
(AMENDMENT_R3/R4, results READMEs, DECISIONS rows) may still cite the branch numbers; `research/pkm_falldegree_unblock_20261107/AMENDMENT_NUMBERING.md` maps them.
Where the two series cover the same cells (stage B, `t = 13` draw 0, `t = 14`, `m = 4`, the stage B G3 race), each series' rows are an
independent run and are compared, not merged.

## Amendment 22 (2026-11-14; execution route only; before any job runs on it): reserved RunPod CPU pods

The owner's instructions of 2026-11-14 were "unblock the fall-degree study and root solver. Run this on runpod", then "do the
isogeny arm at P1 first". The full text, its evidence and three rounds of independent review
(`research/pkm_falldegree_unblock_20261107/REVIEW_R3_R4.md`, SHA-256 `59fd58bc7fb4…`) are in
`research/pkm_falldegree_unblock_20261107/AMENDMENT_R3_RUNPOD.md`, revision 3, SHA-256
`d04aec5fa9f4e6376b26c56b00ddefc0c0989c2672b0785054fd76a4c643bcc3`. That file is binding. The essentials are restated here:

- **What runs.** Registered jobs may run on reserved (`interruptible false`) RunPod CPU pods: 12 vCPU, `cpu5m` or `cpu3m`, image
  pinned by digest. Only Amendment 23's jobs are on this route. The Kummer retries (`pfd-x-t13-k0-c6`, stage B) and the
  `t = 14` and `m = 4` cells need the owner's answer first.
- **Runner** (`runpod/server.py`, SHA-256 `85e8784de26a…`; client `runpod/rp.sh`, SHA-256 `e4eee3a7b47d…`). It refuses a job
  whose files do not match their pins, which has no time limit, whose cost at the limit plus 4 hours idle exceeds the per-job
  ceiling at the pod's live price, whose memory is below the request, which has fewer than 12 CPUs, whose `oom_kill` counter
  is unreadable, or whose results directory is not empty. It kills a job at its registered time limit, with exit `deadline`.
- **Results** live on the pod volume, are fetched with a per-file SHA-256 manifest, and go to S3 by hash. No job starts while S3
  upload is unavailable.
- **Not changed:** hypotheses, §6 rules, cells, seeds, draws, caps, budgets, limits, binaries, stages.
- **Ceilings.** Applying decision 4's ceilings (US$40 per job, US$400 for the study), written for Spot compute, to reserved pods
  needs the owner's confirmation. No job starts on this route before that confirmation, and before the owner confirms the
  account balance once.
- **Losses** are recorded with their reason. Attempts are counted per cell across routes, and the next attempt needs owner
  authority.

## Amendment 23 (2026-11-14; before any isogeny row at `P1` exists): the 2-isogeny arm at `P1`

The full text is `research/pkm_falldegree_unblock_20261107/AMENDMENT_R4_ISOGENY_P1.md`, revision 4, SHA-256
`21f65d61bf1f4cca77772d921293fd3caee2c8e38c0d27463286c2ebe6cfacaa`. That file is binding, including its disclosure (§0)
of everything seen before it was written. The essentials are restated here:

- **Arm.** The 2-isogeny `m = 2` arm runs at `P1 = 2013265921`, `t = 1..13`. The pilot is frozen f13a690df0 plus
  `gaux/aux_order_f13a690df0.patch` (`--aux-order mestre`: exact `#E'` by Mestre on `E'` and its twist, with a private stream,
  so the same seed selects the same tower). Binary `pkm_tower_pilot_r4` has SHA-256 `705b151b6f35…`; `RAYON_NUM_THREADS=12`.
- **Pinned job set.** `iso/PINS.json`, SHA-256 `f02c03dd95a861c51cd30d4dc0ff941590ee3880f18016092d3d33af1778a6da`, and
  `iso/BUNDLE.sha256`, SHA-256 `557ceba4d3cf46b58c7af49477300600d214182eff47713b3cd2b8565eed24ff`. Every argv is in
  `iso/entry_iso.sh`, and every job's bundle must match these pins on the pod.
- **Gates.**
  - G-AUX (16 `P0` draws, field-by-field equality) as the first stage of every job;
  - G1;
  - G2;
  - G3 through `iso/pilot_mestre.sh`;
  - stop on a Mestre panic;
  - an independent naive recount of every selected auxiliary curve after the run.
- **Decomposability.** Every draw is certified by `iso/certify_m2.py` (exact `V × V` count). Decomposable draws are excluded.
- **Readings.** By `iso/score_iso.py` as committed in the bundle:
  - P-ISO-1: transfer of the `P0` medians at `t = 1..8`;
  - P-ISO-2: "flat at 5", "grows", "below 5" or "undecided" at `t = 9..13`, with range aggregation;
  - lower median, certified admission, both draws per cell;
  - any conflict stops R4.
- **Not an improvement claim.** A "flat at 5" reading is a solving-degree description. It does not separate a Semaev-specific
  effect from a tower-ring effect. It is not an improvement over the index-calculus baseline, which also needs the relation
  cost, the auxiliary search and matched rho.
- **Preconditions for the first job:** Amendment 22's (owner ceilings and balance); S3 working, with `705b151b…` in the
  artifact store and a resume checkpoint and `bench.record/v1` written. No `P1` isogeny row existed when this was frozen.

## Amendment 24 (2026-11-15; storage only; before any job of Amendment 23 runs): git in place of S3

Owner decision 12 (`research/open_leads_execution_20261108/DECISIONS.md`), "store in git. do 1-5", given after S3 failed for
this session with `InvalidAccessKeyId`. For the jobs of Amendment 23 it replaces every S3 requirement of Amendments 22 and 23.
Nothing else changes.

- **Results.** Each job's results are fetched from the pod with the pod's own per-file SHA-256 manifest (`/manifest`), checked
  file by file, and committed under `research/pkm_falldegree_unblock_20261107/results/ISO/<job>/`, with the manifest and the
  tarball's SHA-256. Fetches happen at every check-in, so a lost pod loses at most the work since the last fetch.
- **Binary.** `pkm_tower_pilot_r4` (`705b151b…`) is not committed. It is rebuilt bit-for-bit from the committed source and patch
  (Amendment 23; `runpod/REBUILD.md` shows the recipe reproduces the frozen binaries exactly). Its SHA-256 is checked on the pod
  by the runner.
- **Resume note.** `research/pkm_falldegree_unblock_20261107/iso/RESUME_ISO.md`, committed and updated at each check-in.
- **Checkpoint hook.** The hook's S3 push cannot succeed in this session, so launch commands use its recorded opt-out (a pcc
  ledger entry naming decision 12).
- **Amendment 22 §3** ("no job starts while S3 upload is unavailable") reads, for these jobs: no job starts unless the session
  can commit and push to the study branch.

## Amendment 25 (2026-11-15; resources only; before any job of Amendment 23 runs): 16 vCPU, and a job time limit that fits US$40

- **Why** (measured). RunPod refused a 12-vCPU CPU pod ("vcpuCount must be a power of two"). The 16-vCPU reserved pod created
  for `iso-a` (`w3ao0j8mfsiasz`, `cpu5m`/`cpu3m`, 128 GB, EU-RO-1) costs US$1.04/h. Amendment 23 §6's limit of 172,800 s plus
  Amendment 22's 4-hour idle allowance would cost US$54, above the US$40 per-job ceiling (owner decisions 4 and 9).
- **Change.**
  - Amendment 23 jobs run on 16-vCPU pods. `RAYON_NUM_THREADS` stays 12, which is what the instrument depends on.
  - The `iso-a` job time limit is **120,000 s** (cost at the limit plus 4 h idle: US$38.80).
  - The `iso-x` job limit stays 90,000 s, which costs US$27.20 at this price.
- **Unchanged:** every per-system budget (1,800 s; 86,400 s for X), every cell, seed, cap and rule.
- **If the limit is reached.** A job stopped at its time limit has exit `deadline`. Its rows up to the last fetch are kept.
  Systems not reached are reported as not run, not as censored. The projected wall time is 2–6 h (Amendment 23 §6, heuristic).

## Amendment 26 (2026-11-15; before any row of the jobs it adds): isogeny `t = 14`, two Kummer retries, stage B as two jobs

Owner decisions 13–15 (`research/open_leads_execution_20261108/DECISIONS.md`). Route, storage and resources follow
Amendments 22, 24 and 25: 16-vCPU reserved RunPod pods, git storage, the US$40 per-job refusal including 4 hours of idle billing.

**Disclosure.** Decision 14 was given after the isogeny `P1` closure rows `t = 1..9` of job `iso-a` had been fetched and seen,
unscored (`results/ISO/iso-a/fetch-20261115T0423Z/`): `last_fall_degree` 3 at `t = 1`, and 5 at `t = 2..9`, on both draws.
So the `t = 14` cell does not enter P-ISO-2, whose range stays `t = 9..13` as frozen. It gets its own reading below.

1. **Isogeny `t = 14`** (job `iso-14`, limit 90,000 s, US$30.16 at the limit).
   - New stages of `iso/entry_iso.sh` (new bundle pins, hashes below):
     - `A-14`: closure, cmax 5, budget 1,800 s, output `A-m2-iso-p1-t14`;
     - `DA-14`: F4, budget 1,800 s, output `DA-m2-iso-p1-t14`;
     - `cap-14`: capped F4 at `D − 1` (the `run.sh cap-A` rule);
     - `cert` as before;
     - the `x:14:K:C:DENSE` bracket only if `d_F` is still undecided.
   - Every draw is certified (λ = 0.133 ≥ 0.1, so decomposable draws are expected more often than at `t ≤ 13`); decomposable
     draws are excluded.
   - **Reading P-ISO-3 (descriptive, fixed now).** The `t = 14` median under the same rules as §5 of Amendment 23:
     "grows" (exact `> 5`, or censored `≥ 6` with the remaining draws certified), "flat at 5", "below 5" or "undecided". It is
     reported beside P-ISO-2 and never changes P-ISO-2's reading.
2. **Kummer `pfd-x-t13-k0-c6`, attempt 4** (job `pfd-x-t13-k0-c6-rp`, frozen binaries and `entry_r2.sh`).
   - Stages: `cap:12:0:5:12800000000 cap:12:0:6:12800000000 cap:13:0:6:12800000000`.
   - The first two are Amendment 22's positive control (E3.7). They must equal the admitted X1/X3 rows on `inconsistent`,
     `basis_len`, `max_cols` and `muladds`, with the wall time recorded against 1,824 s. Otherwise the third stage's row is not
     admitted.
   - Limit 90,000 s. US$30.16 at the limit, so the control's 30 minutes fit.
3. **Stage B, attempt 3**, packaged as two jobs instead of one, because 172,800 s costs US$54.08.
   - `pfd-b-rp-1`: `g1 hky-B g3`, limit 120,000 s.
   - `pfd-b-rp-2`: `g1 d-B`, limit 120,000 s.
   - The systems, per-system budgets, seeds and rows are those of `run.sh hky-B` and `d-B`, unchanged. The `B-*` and `DB-*`
     row files are the same files the single job would write, and G3 runs over the `hky-B` rows as Amendment 11 says.
   - Each job's first stage is the E3.7 control as in item 2.
   - A job stopped at its limit has exit `deadline`. Its finished invocations are kept; systems not reached are "not run".
   - R2-G stays unfrozen and unused.
4. **`m = 4` cells**: on the plan, not started (decision 15). They need the owner to raise the per-job ceiling for them.

The study's running total is checked against US$400 before each pod is created.

**Amendment 26 pins.** The `iso-14` job uses `iso/BUNDLE.sha256` with SHA-256 `ea826e48c1984a9364d59e1a812a777f7af47cb64ad8245f919cea8103d306cf` and `iso/PINS.json` with SHA-256 `f02c03dd95a861c51cd30d4dc0ff941590ee3880f18016092d3d33af1778a6da`
(binaries unchanged from Amendment 23). The Kummer jobs use the frozen binaries, `entry_r2.sh` and `run.sh` of
`image/image.json` (all hashes checked 2026-11-15). The R3 client `runpod/rp.sh start` now sends the same guard fields as
`start-iso`.

## Amendment 27 (2026-11-15; procedure only; before stage B is scored): stage B's G3 rerun with `--jobs 1`

- **What happened** (measured; `results/KUMMER/pfd-b-rp-1/fetch-20261115T1055Z/gates/`).
  - The `g3` stage of `pfd-b-rp-1` panicked 14 s in (exit 101) at `examples/prime_fall_degree_tools.rs:524`
    (f13a690df0): "capped rows: … No such file or directory".
  - Before the panic, 22 G3 runs were written. **All 22 agree** (0 disagreements).
- **Cause** (derived from the source and the rows).
  - `gate3` names each worker's temporary output `g3-{p}-{kind}-{m}-{x_r}-{cap}.jsonl`, without the control.
  - Stage B's `B-m3-tower` and `B-m3-null` rows share `(p, kind, m, x_r)` on 10 systems.
  - With `--jobs 2`, a tower run and a null run can write and delete the same file, so one worker finds it missing.
  - This is a tool defect, not a disagreement. Stage A's G3 also mixed tower and null rows and passed; the race is timing-dependent.
- **Change.**
  - Stage B's G3 is rerun once, in full, as `prime_fall_degree_tools gate3 DIR /usr/local/bin/pkm_tower_pilot --max-n 20 --jobs 1 --budget 900`
    (stage `g3-b` of `iso/entry_iso.sh`). With one worker, no two runs exist at once.
  - Binaries: the frozen `prime_fall_degree_tools` (`7e8a6703…`) and the frozen `pkm_tower_pilot` (`f78f3959…`).
  - Rows: the 9 `B-*.jsonl` files of `pfd-b-rp-1` (fetch `20261115T1055Z`), listed with SHA-256 in
    `results/KUMMER/pfd-b-rp-1/G3B_ROWS.sha256`. They are uploaded as `rows.tar` and hash-checked by the runner.
  - Pod: a reserved 16-vCPU pod, under Amendments 22, 24 and 25. Bundle pins: `iso/BUNDLE.sha256` SHA-256 `d832b57a6391eeb0af3967cf797946de13c681c21f8cc829daf44ddf8240741e`.
- **Unchanged:** the systems, caps, `--max-n`, budget, the agree rule, and Amendment 11's stop rule. A disagreement or timeout
  in the rerun stops stage B as Amendment 11 says.
- **Consistency check.** The 22 runs already written must come out the same in the rerun (same system, cap, refutation).

## Amendment 28 (2026-11-15; before any row of the jobs it adds): Kummer `t = 14`, the `m = 4` cells, and the isogeny null

The owner's instruction of 2026-11-15 was "Chase down all leads above": Kummer `t = 14` cap 6, the `m = 4` jobs, and an
isogeny null arm. Route, storage and resources follow Amendments 22, 24 and 25 (reserved 16-vCPU RunPod pods; git; refusal
above the per-job ceiling including 4 h idle). Every Kummer job starts with the E3.7 positive control
(`cap:12:0:5`, `cap:12:0:6`), and its rows are admitted only if X1/X3 are reproduced exactly.

**Disclosure.** Everything in `REPORT_RUNPOD_20261115.md` had been seen when this was written: the isogeny `P1` medians,
Kummer `t = 13` "holds", and stage B. No row of the cells below exists.

1. **Kummer `t = 14`, cap 6** (Amendment 7 B; Amendment 14 at 56Gi; `c*(14) = 7`, the decisive C-step cell of Amendment 7 E).
   - Jobs `pfd-x-t14-k{0,1}-c6-rp`: stages `cap:12:0:5:12800000000 cap:12:0:6:12800000000 cap:14:K:6:12800000000`.
   - Limit 90,000 s, ceiling US$40.
   - If cap 6 does not refute on a draw, that draw's cap 7 (`cap:14:K:7:12800000000`) runs as a new job, with the same rules.
   - Readings: Amendment 7 D.5 as registered.
   - Amendment 14's `-m112` rule applies to these single-heavy-stage jobs (Amendment 22 F-R3-3).
2. **`m = 4`.** Amendment 8's four Kummer cells and Amendment 13's four null cells:
   - `m4:4:K:7:13958643712:4000000` and `m4n:4:K:7:13958643712:4000000`;
   - `m4:5:K:6:13958643712:4000000` and `m4n:5:K:6:13958643712:4000000`;
   - `K = 0, 1`; frozen binaries and `entry_r2.sh`; memory floor 112 GiB (pod 128 GB); limit 176,400 s;
     ceiling US$60 (owner decision 16; US$55.12 at the limit).
   - On RunPod there is no pool limit, so Amendment 13's "serially" becomes "each null is submitted after its Kummer
     counterpart". The order is kept; the jobs may run at the same time.
   - Readings: Amendments 8 and 13 as registered.
   - The 256 GB cells (`t = 5, c = 7` and `t = 6, c = 6`) wait for the `t = 5, c = 6` rows' measured entry counts
     (`CAPACITY_R2.md` §1) and a further amendment.
3. **Isogeny null at `P1`** (`--kinds isogeny --controls null`, `m = 2`).
   - Job `iso-null` (stages `g1 gaux A-null A-null-big DA-null cap-A-null` of `iso/entry_iso.sh`): the stage A isogeny
     cell set at `t = 1..13` with the null control.
   - The null keeps the Vélu tower equations and replaces the `S3` coefficients with random ones (the pilot's `system`;
     kind-independent). The rows are not certified: a null system is not a summation system.
   - Limit 120,000 s, ceiling US$40. Bundle pins: `iso/BUNDLE.sha256` SHA-256 `69611876052a4805b17ed5266a97f06bb2c656fe80236d2129f676161f4fca50`.
   - **Reading P-ISO-N** (`score_iso.py --null`), per `t`, the isogeny tower median (committed `results/ISO/SCORE/score_iso.json`)
     against the null median (lower median, both draws):
     - **"Semaev-specific"** if some `t` has both exact and different, or one exact and the other a bound above it;
     - **"shape-driven"** if no `t` differs and at least 8 `t` have both exact and equal;
     - **"undecided"** otherwise;
     - a conflict stops it.
   - It answers whether P-ISO-2's "grows" belongs to the Semaev system or to the tower ring.

The study total at the limits is checked before each pod is created, against US$400 plus the `m = 4` jobs' ceilings
(owner decision 16).

## Amendment 29 (2026-11-15; route procedure; before the relaunch): balance per batch, frequent fetches

After all 11 Amendment 28 pods were lost (most likely a RunPod balance stop; `results/RUNPOD_LOSSES_20261115.md`):
- **Balance.** No batch of pods starts unless the owner has confirmed the balance covers the batch's spend at its limits (owner
  decision 18 for this batch: US$99.15). This replaces the one-time confirmation of decision 10.
- **Fetches.** While a batch runs, every pod's results are fetched every 15 minutes, checked against the pod's manifest, and
  kept. A loss then costs at most 15 minutes of rows. Fetched rows are committed at each check-in (Amendment 24).
- **Relaunch** (decision 18; Amendment 22 §7): attempt 2 of `iso-null`, `pfd-x-t14-k0-c6` and `pfd-x-t14-k1-c6`, with exactly
  the Amendment 28 stages, limits and pins.

## Amendment 30 (2026-11-15; a launch batch on reserved RunPod pods, with the owner's balance confirmation; before any row of these jobs)

Every number is `[pcc:unverified]`.

- **Owner instruction (2026-11-15, ~19:40Z, session 01TCbLH6osebpnEbutNogBzD):** "Launch". The owner had been asked to confirm
  the RunPod balance for exactly this batch, so this is the per-batch confirmation of decision 18 (Amendment 29).
- **Jobs.** Each is the registered stage, run verbatim by `entry_r2.sh` on the binaries admitted by Amendment 17.

  | job | what | stage | pod | per-job cap |
  |:--|:--|:--|:--|:--|
  | `pfd-x-t14-k0-c7-rp2`, `pfd-x-t14-k1-c7-rp2` | `m = 2`, `t = 14`, cap 7 (Amendment 7 B) | `cap:14:K:7:12800000000` | 16 vCPU / 128 GB | US$40 |
  | `pfd-m4-t5-k0-c7-rp`, `pfd-m4-t5-k1-c7-rp` | `m = 4`, `t = 5`, `c = 7` | `m4:5:K:7:30064771072:4000000` | 32 vCPU / 256 GB | US$100 (owner-approved, 2026-11-15) |
  | `pfd-m4-t6-k0-c6-rp`, `pfd-m4-t6-k1-c6-rp` | `m = 4`, `t = 6`, `c = 6` | `m4:6:K:6:13958643712:4000000` | 16 vCPU / 128 GB | US$40 |

  - **Cap 7.** This is attempt 2. Attempt 1 (pods `mn2he0hsnc4uqo`, `aarqk8kswd3zry`) was lost with no rows in the
    2026-11-15 balance stop.
    - Cap 6 on both draws completed without refuting (`results/RPX/*-m112-rp`), so cap 7 is the Amendment 7 B next step.
    - Amendment 14's order holds: 56Gi first; `oversize` or a memory kill gives an `-m112` run with `--max-dense
      27900000000` on a 256 GB pod.
  - **`m = 4`, `t = 5`, `c = 7`.** This changes a limit, as R3 §3 required. `max_entries = 30064771072` is CAPACITY_R2 §1's
    formula for a 232Gi request on a 256 GB pod.
    - Sizing basis (measured): `t = 4`, `c = 7` reached 1.33·10⁹ entries at 91,276 columns. The `t = 5`, `c = 6` closure
      is complete at 129,040 columns and 182,700 entries.
    - Estimate (derived, extrapolated): 5·10⁹ to 2·10¹⁰ entries, about 40–160 GB.
    - A closure stopped by the limit reads *undecided*, never *flat* (Amendment 8).
  - **`m = 4`, `t = 6`, `c = 6`.** Registered limits. The `c = 6` levels at `t = 4, 5` were small (128k and 183k entries).
- **Fetch cadence.** Results are fetched every 15 minutes while a job runs (Amendment 29), so a balance stop loses at most
  15 minutes of rows.
- **Cost.** Worst case for the batch, if every job ran to its cap: US$80 + US$200 + US$80. The study ceiling stays US$500
  (Amendment 20), checked against actual spend at each check-in.
- **Coordination.** These are the same cells as Amendment 28 items 1 and 2. They run here, once. Session
  01TCbLH9vZ4sepWADHKijoFX is told before launch, so that it does not launch them too.
- No hypothesis, §6 rule, reading, seed or budget changes. The Amendment 8 readings apply to the `t = 5`, `c = 7` rows.

## Amendment 31 (2026-11-15; `t = 14` by two routes: the registered `-m112` capped F4, and the exact closure; before any row of these jobs)

Every number is `[pcc:unverified]`.

- **Owner instruction (2026-11-15, ~23:10Z, session 01TCbLH6osebpnEbutNogBzD).** Offered three choices for deciding `t = 14`, the
  owner answered "1-2" ("err 1-2"):
  1. the exact closure route;
  2. the registered `-m112` capped-F4 re-run at up to US$60 per job.
  This is also the per-batch balance confirmation (decision 18, Amendment 29).
- **State.** At `t = 14`, cap 5 and cap 6 complete without refuting on both draws, so `d_F ≥ 7`. Cap 7 at 56Gi stopped
  `oversize` at degree 6 on both draws (Amendment 30). This is not a reading.

### Route F (registered; Amendment 14's `-m112` step)

- **Jobs.** `pfd-x-t14-k{0,1}-c7-m112-rp`, stage `cap:14:K:7:27900000000` (entry `entry_r2.sh`, unchanged).
- **Host.** 32 vCPU / 256 GB reserved pods.
- **Limits.** Job limit 90,000 s, per-job cap US$60 (owner-approved).
- **Reading (Amendment 7 B).**
  - Refutes: `d_F = 7`.
  - Completes without refuting: `d_F ≥ 8`.
  - `oversize` or a memory kill at 112Gi: censored, `d_F ≥ 7` stands.

### Route C (new instrument use: the exact closure, the study's primary engine)

- **Why.** The capped F4 holds a dense block that outgrew 112Gi at `t = 14`. At degree 7 with `N = 28` the closure has about
  1.6·10⁶ normal-form columns (derived, `Σ_{k≤7} C(28, k)`), within `max_cols 4·10⁶`. Amendment 5 had restricted `t ≥ 11` to
  `cmax 5` only for the shared 32 GB host.
- **The instrument.** `runpod/entry_m2c.sh` (committed with this amendment; sha256 in `runpod/m2c.sha256`) runs one closure
  cell.
  - Flags: those of `run.sh`'s `A-m2-tower-big` line, with `--hky-cmax 7`, `--targets K`, and the limits below.
  - After the closure: `pfd_grid_zeros`.
  - The runner gains an `m2c` entry that installs only the gated pilot, `pfd_grid_zeros` and this script, each
    SHA-256-checked.
- **Local smoke test (measured).** `t = 6`, draw 0 reproduces stage A: `x_R = 464833983`, `d_F = 5`.
- **Validation gate, fixed now.** The `t = 14` closure value is admitted only if the closure reproduces `d_F = 6` on all four
  draws of `t = 12` and `t = 13`. Those values are known from the capped-F4 brackets (`RESULTS_T12_BRACKET.md`,
  `RESULTS_T13_BRACKET.md`, Amendment 18).
  - Any disagreement: no route-C value is admitted, and the owner is told.
  - A validation cell censored by a limit: the gate is undecided for that cell, and the `t = 14` value is reported but not
    admitted.
- **Jobs.**
  - Validation: `pfd-m2c-t{12,13}-k{0,1}-rp`, stage `m2c:T:K:7:13958643712:4000000:86400`, on 16 vCPU / 128 GB pods. Job limit
    14,400 s, cap US$40 (about US$8 at the limit).
  - Decisive: `pfd-m2c-t14-k{0,1}-rp`, stage `m2c:14:K:7:30064771072:4000000:86400`, on 32 vCPU / 256 GB pods. Job limit
    86,400 s, cap US$60.
- **Reading.** `1` enters the closure at `c = 7`: `d_F = 7`. Complete to `c = 7` without `1`: `d_F ≥ 8`. Censored by a limit:
  `d_F ≥` the last complete degree plus one. `grid_zeros > 0`: decomposable, excluded (Amendment 7 C).

### Both routes

- **What the result decides.** Both routes measure the same quantity on the same two systems. If both give a value, they
  must agree; a disagreement is a reproducibility failure (Amendment 2). With `m = 4` reading "rise" (Amendment 30):
  - `d_F(14) = 7` on both draws means C-step holds at `t = 12–14`, so Amendment 8 closes the PKM-tower lane;
  - `d_F(14) ≥ 8` means C-step is refuted (upward): "no change" under Amendment 8.
- **Spend.** Batch worst case: US$120 (F) + US$120 (C decisive) + 4 × US$8 (C validation). With about US$95 spent, that stays
  within the US$500 ceiling (Amendment 20).
- **Fetch cadence.** Results are fetched every 15 minutes (Amendment 29).
- No hypothesis, §6 rule, seed or draw changes.

## Amendment 32 (2026-11-18; route procedure and spend record; no job launched)

Every number is `[pcc:unverified]`.

- **Owner instruction (2026-11-18, session 01TCbLH6osebpnEbutNogBzD).** After the Amendment 31 batch was found lost, the owner
  said "Try again", then "Use run pod api v2".
- **Route change.** REST v1 (`rest.runpod.io/v1`) returned 502. `runpod/rp.sh` now uses REST v2 (`api.runpod.io/v2`): body
  `{name, cloud: SECURE, cpu: {id, vcpuCount}, image, disk: 20, ports, env, entrypoint, cmd}`, price field `cost`. The flavor is
  `CPU_ID` (default `cpu5m`, 8 GB per vCPU). Same image digest, same `server.py`, same guards. A 2-vCPU smoke pod reached the
  runner's `/status` and was deleted.
- **The Amendment 31 batch: what it cost (measured, `GET /v2/billing/pods?podId=…`, 2026-11-14 to 2026-11-19).**
  - Route F `t14 k0`, `k1`: US$63.40 each. Route C decisive `t14 k0`: US$63.40; `k1`: US$53.66.
  - Validation `t12/t13 k0/k1`: US$31.74 each.
  - Total: **US$370.84**. Every pod was billed for about 30.5 h, then disappeared. No row was fetched.
- **Why it overran.** The runner's guard bounds the *job*, not the *pod*. A pod stays billed after its job ends until the client
  deletes it, and the session that would delete it was down. The validation pods ran 30.5 h for a 4 h job.
- **Ceiling.** About US$95 before Amendment 31 plus US$370.84 is about US$466 of the US$500 ceiling (Amendment 20). The
  Amendment 31 batch at v2 prices (worst about US$220) does not fit. **No job is launched under this amendment.** A relaunch
  needs the owner to raise the ceiling or to choose a subset, and a pod-lifetime fix: the client deletes each pod when its job
  ends, and no batch starts unless a check-in is scheduled within the shortest job limit.

## Amendment 33 (2026-11-18; relaunch of the Amendment 31 batch, attempt 2, on REST v2; before any row of these jobs)

Every number is `[pcc:unverified]`.

- **Owner decision (2026-11-18, session 01TCbLH6osebpnEbutNogBzD).** The owner said "resume". Asked which relaunch to run, with
  the overrun stated, the owner chose **"Full batch"**: both routes of Amendment 31, all 8 jobs. The study ceiling rises from
  US$500 to **about US$690** (Amendment 20 amended). This answer is also the per-batch balance confirmation (decision 18,
  Amendment 29).
- **Jobs, stages, hosts, limits, readings and the validation gate.** Exactly as in Amendment 31, with the job names suffixed
  `-v2`. The client is `rp.sh` on REST v2 (Amendment 32), flavor `cpu5m` (8 GB per vCPU; US$2.08/h at 32 vCPU, US$1.04/h at
  16 vCPU).
- **Pod lifetime (new; the Amendment 32 overrun).** A pod is billed until it is deleted, so:
  - `rp.sh reap ID OUTDIR DEADLINE` fetches the results, checks them against the pod's manifest, and deletes the pod once its
    job is done, or once the deadline passes, whatever the job's state. Deadline: launch + job limit + 30 min.
  - The session runs the reaper every 15 minutes. Scheduled check-ins, which survive a session restart, also run it hourly and
    at each deadline.
- **Spend.** Job guards: 2 × US$51.65 (F) + 2 × US$49.92 (C decisive) + 4 × US$4.16 (C validation) = US$219.8 worst case,
  plus at most 30 min of reap lag per pod. With about US$466 already spent, that is about US$690 worst case.
- No hypothesis, §6 rule, seed or draw changes.

## Amendment 34 (2026-11-19; the `t = 13` validation cells with a longer job limit; before any row of the reruns)

Every number is `[pcc:unverified]`.

- **State.**
  - Validation `t = 12`, both draws: the closure gives `d_F = 6`, uncensored, `grid_zeros = 0` (2.3 h each).
  - Validation `t = 13`, draw 0: stopped by the 14,400 s job limit with no row (the pilot writes on completion only). Censored,
    so its gate cell is undecided (Amendment 31). Draw 1 runs to the same limit at 01:28Z.
  - Route F `t = 14`, draw 0: aborted with an allocation failure at degree 6 on the 256 GB pod. Censored: `d_F ≥ 7` stands.
- **Owner instruction (2026-11-19, ~00:40Z).** The `t = 13` reruns were offered with a 12 h limit at about US$11–13 each. The
  owner said "resume".
- **Change.** Jobs `pfd-m2c-t13-k{0,1}-rp-v3`: stage `m2c:13:K:7:13958643712:4000000:86400`, unchanged. Same 16 vCPU / 128 GB
  pods, same binaries and hashes. Job limit 43,200 s, cap US$40; reaped at the limit + 30 min (Amendment 33).
  - Draw 1 is rerun only if its current job is also censored.
- **Spend.** At most 2 × US$13.5, within the US$690 ceiling (Amendment 33).
- Gate, readings, seeds and draws are unchanged.
