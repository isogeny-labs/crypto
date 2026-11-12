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

MEASURED_PLACEHOLDER

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

READING_PLACEHOLDER

## 5. What this does not do

- It does not run F4 on prime-field small-x systems (the degree-`2^l` constraint makes the question
  moot: see §1) and it does not re-run the PKM or `ml-cryptanalysis` measurements; those are cited
  as recorded.
- The refutation degree is measured with the harness's own `d_max` cap; a row whose `D*` is `null`
  means "above the cap", not "none".
- No pcc claim, no benchmark record, no leaderboard change.
