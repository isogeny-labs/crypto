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
