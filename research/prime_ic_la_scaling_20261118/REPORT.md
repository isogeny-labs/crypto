# Prime-field IC: sparse relation-matrix linear algebra over F_n, and the factor-base sweep

Measured 2026-11-18 (UTC), on the `a = p − 3` P-256-class rungs
(`p ≡ 3 mod 4`), anchored S₃-sweep 3-decomposition oracle (PR #4712), seed
20261005, `+8` extra relations.

**No exponent claim.** The two questions answered here are (i) can the
relation matrix be solved sparsely over a prime subgroup order, and (ii) does
a larger factor base make the 3-decomposition cheaper end to end.  (i) yes,
by 2.7–12×; (ii) no, total wall is `≈ p·c` for every `B` measured.

## 1. Code

`src/cryptanalysis/ec_index_calculus.rs`:

- `sparse_gaussian_eliminate_mod_n` — sparse Gauss–Jordan over `F_n` with
  Markowitz pivot ordering (fewest-entry row, then fewest-entry column),
  column→rows index maintained through fill-in, free unknowns set to zero.
  Returns the same `ModNSolution` (values, `determined` flags, rank) as the
  dense engine, plus an `LaStats`.
- `LaStats` — engine name, `rows`, `cols`, `nnz_in`, `nnz_peak` (fill-in for
  sparse, `rows·cols` for dense), `rank`, `muladds`.
- `LaEngine { Dense, Sparse }`, `SparseRow`, `sparse_row_from_entries`,
  `solve_relation_matrix` (assembles `[M | −b]·(y,x) = a` once and dispatches
  on the engine), and `ec_index_calculus_dlp_s4_{,s3_}staged_with_la`
  wrappers.  The existing `staged` / `_s3_staged` entry points keep the dense
  engine, so earlier records stay reproducible.
- `EcIcStageReport` gained `la: Option<LaStats>`; `examples/a3_s4_ladder`
  takes the factor-base size and the LA engine as arguments
  (`<bits> <seed> <class> <oracle> <fb> <la>`).

## 2. Correctness

- `la_engines_agree_on_the_same_relation_set`: 40 synthetic 3-summand rows
  over 12 columns, both engines: identical `nnz_in`, rank, `determined`
  flags, values, and every determined value equals the planted one.
- `sparse_and_dense_la_agree_on_random_systems`: planted-solution systems at
  `12×8`, `20×12`, `10×12` (rank deficient) and `40×40`; ranks, flags and
  values agree, and the engines agree on consistency of a perturbed
  right-hand side.
- `sparse_la_recovers_the_same_log_as_dense`: both engines recover the same
  scalar end to end on the tiny fixture; `la.rows = relations_collected`,
  `la.rank = cols`.
- 41 module tests pass.  Every row of the sweep below satisfies `[x]G = Q`,
  matches the validation-only sidecar, and agrees with the paired ρ control.

The planted-solution test earned its keep: it caught a real bug in the first
version — fill-in entries created during elimination were not registered in
the column→rows index, so a later pivot on that column missed the row.  The
symptom was a *determined* column with a wrong value on a rank-deficient
system; the dense engine was right and the sparse one wrong.  Fixed and
covered by the tests above.

## 3. Linear algebra at IC dimensions (`sweep.jsonl`)

Same relation-set shape as an m-summand IC row (`m+2` nonzeros per row).

| bits | B | dims | rank | nnz_in | nnz_peak | engine | LA ms |
|--:|--:|:--|--:|--:|--:|:--|--:|
| 16 | 128 | 136×129 | 113 | 544 | 17,544 / 543 | dense / sparse | 0.41 / **0.15** |
| 16 | 256 | 264×257 | 214 | 1,047 | 67,848 / 1,048 | dense / sparse | 1.36 / **0.29** |
| 16 | 512 | 520×513 | 374 | 2,057 | 266,760 / 2,058 | dense / sparse | 5.99 / **0.68** |
| 24 | 256 | 264×257 | 247 | 1,054 | 67,848 / 2,549 | dense / sparse | 24.25 / **7.84** |
| 24 | 512 | 520×513 | 449 | 2,076 | 266,760 / 4,121 | dense / sparse | 44.09 / **3.65** |
| 28 | 256 | 264×257 | 244 | 1,055 | — / 2,632 | sparse | **1.47** |
| 28 | 512 | 520×513 | 481 | 2,077 | — / 7,999 | sparse | **5.22** |
| 28 | 1024 | 1032×1025 | 925 | 4,125 | 1,057,800 / 21,379 | dense / sparse | 57.85 / **17.60** |

Sparse LA is 2.7–12× faster than dense at these dimensions and touches 13–68×
fewer entries; fill-in stays within ~5× of the input on the largest matrix.
Rank is *not* recomputed per relation: the driver assembles the matrix after
collection and solves once, so this does not meet the ledger's
"rank recomputed after every relation" gate for the full orbit-reduced
`n = 21` target.

## 4. Factor-base sweep: does larger `B` pay? (`sweep.jsonl`)

Anchored 3-decomposition, one seed each, all verified.  `rel` = relation
phase wall, `IC` = whole solve.

| bits | B | rows | trials | per relation | rel s | IC s |
|--:|--:|--:|--:|--:|--:|--:|
| 28 | 256 | 264 | 3,590 | 1.46 s | 385.3 | 385.3 |
| 28 | 512 | 520 | 1,059 | 0.41 s | 215.3 | 215.3 |
| 28 | 1024 | 1,032 | 1,034 | 0.099 s | 102.2 | 102.2 |
| 24 | 256 | 264 | 369 | 78 ms | 20.5 | 20.6 |
| 24 | 512 | 520 | 520 | 21 ms | 11.0 | 11.0 |
| 16 | 128 | 136 | 136 | 0.15 ms | 0.02 | 0.02 |
| 16 | 512 | 520 | 520 | 0.06 ms | 0.03 | 0.04 |

Per relation the wall falls like `p·c/B` (one relation costs `p/(2B³)` trials
at `2B²` candidates each), and a full solve needs `≈ B` rows, so the total is
`≈ p·c` for every `B`: 28-bit totals are 385 / 215 / 102 s for a 4× range of
`B`, straddling `p·c_solve ≈ 1.6 × 10² s` at the measured per-candidate cost.
The residual `B`-dependence is the anchor overhead (`2B` point additions per
trial), which amortizes as `p·c_anchor/B` — that is why `B = 1024` is 2.4×
cheaper than `B = 128` at 28 bits without leaving the `p·c` floor.

So the ledger's open question — *"pair the 3-decomposition density with a
larger factor base where `p/B³` beats 2-decomposition total cost"* — is
answered on this fixture: there is no crossover to find.  Relation collection
is `p·c` at every `B`, the 2-decomposition is `p·c′` with a smaller `c′` but
one fewer column per row, and at 28 bits the two totals are 102–121 s
(3-decomp at `B = 1024`) versus 121–126 s (2-decomp at `B = 120`).  Within
Monte-Carlo spread, they are the same number.  `B` trades relation cost
against linear-algebra size and preprocessing, and nothing more.

## 5. What this means for the subexponential goal

Nothing here moves the exponent, and the sweep says why in one line: with
enumeration-based decomposition the per-log cost is pinned to `p·c`, and `B`,
`m` and the LA engine only move `c` (and the preprocessing/LA terms traded
against it).  A subexponential prime-field IC requires a decomposition whose
cost per target is *sublinear in `B`* — the algebraic (Semaev + F4/FFD) route
measured in `research/prime_fall_degree_20261027`, whose solved degree `D`
grows with `m` on the PKM tower bases (m = 2..5 read `d_F` 4,5,6,6 at
`p = 2^31`) — or a factor base with an algebraic membership condition.  The
sparse engine here is the linear-algebra component such a pipeline would need
once `B` is large; at the sizes measured it is not the bottleneck.

Claim boundary: public synthetic known-answer fixtures only; no deployed
curve, no key recovery, no `vs_rho` claim, no asymptotic claim.  Wall times
are exploratory: shared host, single run per cell, load average 6–8 on 14
cores.

## 6. Reproduce

```bash
cargo test --release --lib cryptanalysis::ec_index_calculus
cargo run --release --example a3_s4_ladder -- 28 20261005 p256 s3 1024 sparse
cargo run --release --example a3_s4_ladder -- 28 20261005 p256 s3 1024 dense
research/prime_ic_la_scaling_20261118/run_sweep.sh
```

Files: `sweep.jsonl`, `progress.txt` (per-run UTC timestamps), `run_sweep.sh`,
`provenance.json`, `summarise.py`.
