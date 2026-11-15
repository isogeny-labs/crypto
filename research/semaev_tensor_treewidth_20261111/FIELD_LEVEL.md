# Field-level elimination: where the structure the bit-level census cannot see lives

Follow-up to the treewidth census in this directory (owner's question, 2026-11-12). The census
showed that every *bit-level* encoding of a point-decomposition relation has primal treewidth at
least 2b, so exact Boolean contraction costs 2^(≥2b) against 2^(b/2) for enumeration, however good
the solver. The lever left is elimination at the *field* level: treat the relation as a polynomial
system over the field and eliminate with Gröbner/F4 or Macaulay linear algebra, which exploits
degree structure a Boolean tensor network cannot represent. This note (a) measures what a
tree-decomposition solver sees when handed the field-level system itself, the Weil-descended binary
Semaev S3 point-decomposition system the lab's F4 and refutation-degree harnesses solve, next to the
refutation degree F4 actually reaches on the same instance; (b) collects the lab's own measured
field-level costs over prime fields (PKM towers) and over F_{2^n} (Semaev S3 with a subspace base),
so the two routes can be set against enumeration on one page. Labels as in the README; every number
`[pcc:unverified]`.

## 1. Why the two fields differ (structural, [derived])

- **Prime fields, small-x base.** The membership constraint `x < 2^l` is not algebraic: the
  polynomial vanishing on `V = {0, …, 2^l − 1}` has degree `2^l`, so a field-level system
  `S3(x1, x2, x_R) = 0, f_V(x1) = 0, f_V(x2) = 0` has degree `2^l` and its Gröbner cost is
  exponential in `l` before any elimination happens; this is why prime-field point decomposition is
  encoded at the bit level (the PDP SAT sweeps, L09) or with an *algebraic* factor base instead
  (Petit–Kosters–Messeng towers, measured here as `research/pkm_dickson_torus_20261015`).
- **Binary fields, subspace base.** `V = span{z^0, …, z^{n'−1}}` is `F_2`-linear, so membership is
  "the high coordinates are zero": free. Weil descent turns `S3(X1, X2, x_R) = 0` into `n`
  equations over `F_2` in `2n'` Boolean variables, each quadratic (the `X^2` terms are linear over
  `F_2` after descent). The field-level system is *small* (2n' variables) and *dense* (every equation
  touches most variables), the opposite shape of the bit-level networks (10^5 variables, arity ≤ 5).

## 2. Measurement: treewidth of the field-level binary system vs its refutation degree

Binary `semaev-descent-treewidth` (`src/bin/semaev_descent_treewidth.rs`): for each `n` it builds
the instance the refutation harness builds (`choose_irreducible(n)`, random `b`, a target `x_R` with
no decomposition where the search is affordable, `n' = n/2`), descends and restricts
(`ffd_harness::weil_descend_s3`, `pc_degree_harness::restrict_to_subspace`), and measures the
primal graph in two encodings with `hashed_ecdl_tensor::treewidth::analyse`:

- **bb**: one factor per equation over its support, D.1's black-box reading;
- **split**: shared AND gadget per quadratic monomial, XOR chains of arity-3 gadgets per equation
  (the L09 subdivision rule), the honest Boolean-network form of the same system.

It then runs `measure_refutation_degree` on the same instance (`d_max` as recorded) and records the
first fall degree, the refutation degree `D*` and the Macaulay matrix sizes per degree. Supported
`n`: 2–16 (the harness's irreducible table) and 22, 28, 30 (`x^n + x + 1`, irreducible by the Ben-Or
test run before the measurement). Rows: `rows/field-level-*.json`; layout `render_field_level.py`
(`FIELD_LEVEL_TABLE.md`, `field_level_vs_n.svg`).

**Rows** (`rows/field-level-8-30.json`, seed 20261112, `d_max = 4`, 3 restarts; full table in
`FIELD_LEVEL_TABLE.md`, graph `field_level_vs_n.svg`) [measured]:

| n | n' | vars 2n' | eqs | cross monomials x1_i·x2_j | bb width (lower = upper) | split vars | split width lower / best upper | split log2 cost | enum one slot n' | D* (first fall) | Macaulay cols at D* |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 8 | 4 | 8 | 8 | 16 | 7 | 110 | 12 / 19 | 22.3 | 4 | 2 | 37 |
| 10 | 5 | 10 | 10 | 25 | 9 | 210 | 13 / 28 | 31.6 | 5 | 2 | 56 |
| 11 | 5 | 10 | 11 | 25 | 9 | 233 | 16 / 31 | 34.6 | 5 | 3 (3) | 176 |
| 12 | 6 | 12 | 12 | 36 | 11 | 297 | 16 / 39 | 41.4 | 6 | 2 | 79 |
| 13 | 6 | 12 | 13 | 36 | 11 | 289 | 17 / 42 | 44.3 | 6 | 4 (3) | 794 |
| 14 | 7 | 14 | 14 | 49 | 13 | 538 | 22 / 52 | 54.2 | 7 | 2 | 106 |
| 15 | 7 | 14 | 15 | 49 | 13 | 554 | 22 / 54 | 56.4 | 7 | 4 (3) | 1,471 |
| 16 | 8 | 16 | 16 | 64 | 15 | 664 | 23 / 74 | 76.4 | 8 | 2 | 137 |
| 22 | 11 | 22 | 22 | 121 | 21 | 1,659 | 32 / 104 | 106.1 | 11 | 2 | 254 |
| 28 | 14 | 28 | 28 | 196 | 27 | 3,103 | 41 / 182 | 184.5 | 14 | 2 | 407 |
| 30 | 15 | 30 | 30 | 225 | 29 | 4,185 | 44 / 159 | 162.4 | 15 | > 4 (2) | - |

Targets: non-decomposable by exhaustive check for `n ≤ 16` (`2n' ≤ 20` bits searched); random for
`n = 22, 28, 30` (the rows with `D* = 2` were refuted, so those targets have no decomposition; the
`n = 30` row's `D*` is above the cap or its random target decomposes: not decided here). The
descended system has exactly `n'²` quadratic monomials, all cross terms `x1_i · x2_j`, and every
equation touches every variable, so in the black-box encoding each equation is a clique on all
`2n'` variables: the width is `2n' − 1` exactly (lower bound = upper bound, `[exact]`).

## 3. The lab's measured field-level costs, for the same page

**Prime fields, algebraic base (PKM Dickson towers; `research/pkm_dickson_torus_20261015`,
`cells.tsv`, kind `dickson`, control `tower`, `p = 786433`) [recorded]:**

| m | N (tower degree, base size 2^N-ish) | F4 solving degree D | matrix width to solution | multiply-adds |
|---|---|---|---|---|
| 2 | 4 | 4 | 16 | 1.4·10³ |
| 2 | 8 | 4 | 152 | 5.0·10⁵ |
| 2 | 12 | 5 | 1,223 | 6.4·10⁷ |
| 2 | 16 | 5 | 4,786 | 5.8·10⁹ |
| 2 | 20 | 6 | 33,426 | 4.8·10¹¹ |
| 3 | 9 | 6 | 1,184 | 1.6·10⁸ |
| 3 | 12 | 6 | 5,018 | 1.0·10¹⁰ |
| 3 | 15 | 7 | 31,378 | 3.1·10¹² |
| 4 | 12 | 7 | 9,451 | 5.2·10¹⁰ |

At `m = 2, N = 20` one decomposition attempt costs 4.8·10¹¹ ≈ 2^38.8 multiply-adds against
2^20 for enumerating one slot [derived]; the study's own reading is that the torus/split Dickson
towers share one F4 computation with Bardet–Faugère–Salvy degree of regularity `N + 1` (its §0,
derived), i.e. the field-level cost is exponential in `N` with a constant above enumeration's.

**Binary fields, subspace base (`ml-cryptanalysis`, F4 on the descended Semaev system)
[recorded]:**

- `docs/STAGE5_SOLVING_DEGREE_20261109.md`: the exact MutantXL solving degree `D_mut` is 2 at
  `(n, m) = (11, 2)`, and constant 7 at `m = 3, k = 5` on every measured row across
  `n ∈ {11, 17, 19, 23, 31}` (128/128, then 16/16 at 19, 11 and 23); `m = 4` at `(11, 4, 2)`: 5–6.
- `docs/session-logs/2026-11-11.md` (l-ladder sizing, `n = 17`, `m = 3`, one target, not scored):
  `l = 3`: 24 s for 8,544 trials; `l = 4`: 240 s for 2,802 trials; `l = 5` did not finish one solve in
  30 min. The relation-yield program's scored result at `n = 23` is in that repository's
  `relation-yield` study (validated there; not re-derived here).

## 4. Reading

- **At the field level, treewidth is the variable count.** The descended system is dense: in the
  black-box encoding its primal graph is the complete graph on the `2n'` variables, width
  `2n' − 1` exactly, at every `n` measured **[exact]**. A tree-decomposition contraction of the
  field-level system is therefore full enumeration of *both* slots, `2^(2n')`: twice the exponent
  of enumerating one slot (`n'`), which is what the naive relation search already does. The honest
  Boolean-network form (arity-3 gadgets) is wider still, lower 12 → 44 and best upper 19 → 182 over
  `n = 8 → 30`, because the shared AND gadgets and XOR chains add variables without removing any
  interaction **[bound, heuristic]**.
- **F4 does not pay that.** On the same instances the refutation harness derives the contradiction
  at Macaulay degree 2 (even `n`) or 3–4 (odd `n`), with 37–1,471 columns **[measured]**: polynomial
  in `n'`, against `2^(2n')` for the best possible tree decomposition. The structure the field-level
  route exploits is *algebraic degree* (the system is quadratic with `n'²` cross terms, and a
  non-decomposable instance is inconsistent at low degree), which a Boolean tensor network cannot
  represent and treewidth cannot see. The `n = 30` row says where that stops being free: `D*` is
  above the cap of 4, and the lab's own measurements put the solving degree at 7 for `m = 3` across
  `n = 11 → 31` (`ml-cryptanalysis`, stage 5), with F4 cost growing accordingly (`l = 4` ten times
  `l = 3` at `n = 17`; `l = 5` not finishing one solve in 30 min).
- **Prime fields.** The small-x base has no low-degree membership polynomial (§1), so there is no
  field-level system to hand to F4 without the degree-`2^l` constraint; the algebraic alternative the
  lab measured, PKM Dickson towers, reaches F4 degree 4–6 at `m = 2` and 6–7 at `m = 3, 4` but costs
  4.8·10¹¹ multiply-adds (≈ 2^38.8) per attempt at `m = 2, N = 20` against `2^20` enumeration (§3):
  the degree is small, the matrices are not. Over prime fields neither route beats enumeration in
  the measured range.
- **Answer to the owner's question, completed.** Encoding the relation as a constraint graph (bit
  level) gives width ≥ 2b; encoding it as a field-level system gives width = number of variables.
  Neither is where the exploitable structure lives: the only structure that beats enumeration at
  any measured size is algebraic degree, exploited by F4 on `F_2`-linear factor bases over binary
  fields, and that is the index-calculus program the lab already runs. The contraction-solver
  framing (hashed or Semaev) does not add a lever to it.

## 5. What this does not do

- It does not run F4 on prime-field small-x systems (the degree-`2^l` constraint makes the question
  moot: see §1) and it does not re-run the PKM or `ml-cryptanalysis` measurements; those are cited
  as recorded.
- The refutation degree is measured with the harness's own `d_max` cap; a row whose `D*` is `null`
  means "above the cap", not "none".
- No pcc claim, no benchmark record, no leaderboard change.
