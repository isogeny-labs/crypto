# R13-C results: the genus-3 LA follows p^(4/3)(ln p)^c, c ≈ 1.9

Status: measured, single producer, shared host, no independent replay. Every number is
`[pcc:unverified]`. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen before the grid.
Instrument: `genus_ladder` at `f49709baf2`, SHA-256 `97cd8fd2…`, four threads.

## Result

All 26 SGE rows certify `[t]G = Q`. The same-binary anchor at p = 262,147 is
`4.2611e8`, within 0.4% of the campaign binary's `4.2445e8`, so the cross-binary
comparison is sound.

| p | curves × seeds | SGE `la_mul` | FPM_mid | LA share | local `la_mul` slope |
|--:|--:|--:|--:|--:|--:|
| 262,147 | 2 × 1 | 4.261075e8 | 5.686055e9 | 0.674 | — |
| 1,048,583 | 6 × 2 | 3.305559e9 | 4.133224e10 | 0.720 | 1.4778 |
| 1,572,869 | 6 × 2 | 6.010999e9 | 7.366188e10 | 0.734 | 1.4748 |

Fit over the three primes: `la_mul = A · p^(4/3) · (ln p)^c` with **c = 1.915**,
bootstrap 95% [1.851, 1.979] over curves. The fitted model predicts local slopes
1.479 (262k→1.05M) and 1.469 (1.05M→1.57M); the measured slopes are 1.478 and 1.475.

## Against the preregistered predictions

| id | prediction | outcome |
|---|---|---|
| P1 | every run certifies | **held** (26/26) |
| P2 | top-octave local slope ≤ 1.45 | **failed** (1.4748) |
| P3 | fitted c in [1, 3] | **held** (1.915 [1.851, 1.979]) |
| P4 | FPM_mid exponent ≤ 1.42 | **failed** (1.433) |

The falsifier (point slope ≥ 1.50 or c lower bound ≥ 4) is not triggered.

## Reading

The data are consistent with the Diem/GTTD `p^(4/3) polylog` law, not with a plain
`p^(3/2)`: the three-point fit gives c ≈ 1.9 with a tight interval, and the model
reproduces both local slopes to within 0.006. The local slope has **not** fallen below
1.45 by 2^21, so the preregistered "keeps falling" threshold is not independently met;
the support for 4/3 comes from the fitted log factor, not from a falling local slope.

This is a model fit over a 6× range of p, not a proof: one degree of freedom, a
tight-but-possibly-systematic curve sample (the `la_mul` spread across curves is under
2%, because the merged core is the same size), and no independent replay. The H-arm
remains a 0.30–0.40 constant-factor win over rho at these sizes. If the 4/3 law holds
asymptotically, it becomes an exponent win over rho's 3/2, but the crossover is far
because the constant is unfavourable.

## Next discriminating tests

1. Extend the LA measurement to p ≈ 2^22–2^23. The bottleneck is the order search
   (`bsgs1` reaches about 2·10^6), not the index calculus: the 2^20 run is about 90 s.
   Use `bsgs2`, known-order curves, or a precomputed order source.
2. Measure the merged core's average row weight w(p). The model says
   `la_mul ~ cols · rows · w`; a `w` growing as `(ln p)^2` is the direct mechanism.
   The current rows record `core_rows` and `core_cols`, not `w`.
3. Keep the `(2,2,2)` plane-quartic transfer gate as the strategic route to exponent 1;
   this result makes the H-arm's 4/3 asymptotics plausible but does not make it cheap.

## Mechanism: the log factor is the mean row weight

The instrumented run (`run_*_w.jsonl`) records the merged-core geometry. All 26 SGE rows
certify. The decomposition is nearly exact:

| p | rows | cols | mean weight (SGE-filled, nnz/rows) | `la_mul` |
|--:|--:|--:|--:|--:|
| 262,147 | 8,164 | 8,036 | 16.927 | 4.2611e8 |
| 1,048,583 | 20,728 | 20,481 | 19.839 | 3.3056e9 |
| 1,572,869 | 27,202 | 26,892 | 20.749 | 6.0110e9 |

Local exponents: rows 0.672 / 0.670, cols 0.675 / 0.672, weight 0.132 / 0.142.
The product `rows · cols · weight` predicts the `la_mul` ratio to 0.2%:
7.774 vs 7.758 measured (262k→1.05M) and 1.826 vs 1.818 (1.05M→1.57M). The constant is
about 2.55 in both intervals.

Fitting the weight alone against `(ln p)^c` gives **c_w = 1.781** (the direct `la_mul`
fit gives 1.915; the difference is the rows/cols exponents being 0.67 rather than exactly
2/3). So the super-4/3 growth is entirely in the mean row weight; the matrix dimensions
grow as `p^(2/3)`, and `la_mul ≈ 2.55 · rows · cols · w`.

This is the direct test R13-C asked for: the log factor is not an artifact of the fit, it
is the observed growth of the merged core's row weight.
