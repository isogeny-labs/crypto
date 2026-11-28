# Errata to RESULTS.md (from the independent validation)

`RESULTS.md` is left byte-equal to the validated commit 0340c08390 (it is hash-pinned). The
validator's report is `validation/validator-report.md`, disposition **validated-with-errata**. These
corrections take precedence over `RESULTS.md`. None of them changes a headline number: 803 / 753 / 44,
r = 0.516 [0.491, 0.541], coverage 0.758 [0.727, 0.786]. Every number is `[pcc:unverified]`.

- **E1 (finding 5).** "S_L = 0 occurred only on single-map kernels" is false. In the control,
  877 kernels have S_L = 0: 151 with one rational trigonal map and 726 with no rational map.
  `analyze.py` dropped the no-map kernels before counting zeros. The validator sampled 44 kernels at
  p = 17, 19 and 29:
  - all 22 zero kernels have only degenerate trigonal candidates over F_{p^2};
  - all 22 nonzero kernels have 2 genuine maps.

  So the zeros coincide with Smith's degenerate cases. The sentence and its table caption are
  wrong; the formula is not.
- **E2 (finding 11).** The leading-coefficient twist factor chi(c) appears on both sides of the
  control and cancels, so the control does not test it. Only the main arm's two-t check tests it,
  and only 34 of those 80 rows can fail. Report it as **34/34 passed**, not 80/80.
- **E3 (finding 13).** "Degenerate implies hyperelliptic" has a gap. Simplicity over F_p does not by
  itself exclude a Weil-restriction product. Such a product needs e1 = e2 = 0, and no row has that,
  so the conclusion holds for this data set.
- **E4 (finding 14).** The positive control took 4.0 min, not 3.5 min.
- **E5 (findings 2 and 15).** Process:
  - `analyze.py` was never frozen. It was last edited during the control run, before any main-arm
    data. It regenerates `summary.json` byte-for-byte.
  - `data/*.jsonl` belong in the artifact store once store credentials work.
  - The study has no PDF yet.
- **Validator corroboration (finding 7).** One constant D fits all 64 formulas to 6.5e-14. It equals
  kappa^9 det(A)^18 with kappa = -(2 pi)^-6. Under the LRZ normalisation that gives
  chi_18 = -2^-28 S_L, which predicts kappa_p = chi_p(-1) in advance. That normalisation was seen only
  through a search summary, so this corroborates and does not prove.
