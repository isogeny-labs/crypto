# Why do lift sets repel linear structure? Kloosterman moments

2026-10-27. Report: [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ), contains the
derivation). Protocol, frozen before the run: [`PROTOCOL.md`](PROTOCOL.md) (hashes in
[`FREEZE.sha256`](FREEZE.sha256), all three verified after the run). Run:
[`reports/run-20261027T1459Z/`](reports/run-20261027T1459Z/) (`rows.jsonl`: 748 sets,
`summary.json`, `host.txt`, `replication_check.txt`, `stderr.log`, figures).
Parent study: [`../lift_dense_factor_base_20261027/`](../lift_dense_factor_base_20261027/).

**Question.** On E: y² + xy = x³ + ax² + b over F_{2^N}, N prime, the sets of lifting abscissae
contain fewer all-lift 4-dimensional F₂-subspaces than density-paired random sets. Is the cause the
Kloosterman (Sato–Tate) 4th moment of the Walsh spectrum?

**Answer in one paragraph.** Partly. The Walsh spectrum of every curve is a Kloosterman spectrum, so
every curve has the same Walsh power sums, exactly: ΣW⁴ = 2q³ − 2q² − 3q (normalised 4th moment
2 − O(1/q), against 3 for random sets), and ΣW³ = −(−1)^{Tr a}·q(q−2) at odd N. Both are proved in
the report and hold as exact integers on all 120 curves (N = 9, 11, 12, 13). In terms of correlations of ψ = ±1 over
x + y + z = 0 and x + y + z + w = 0, this is C₃ = −ε(q−2) and C₄′ = −(q−2)(q−4). A first-order
expansion of the subspace counts in these two correlations is exact at k = 2 and explains about
70 % of the k = 4 curve deficit (the 4th-moment term, about −105/q; plus a Tr(a)-dependent
3rd-moment term, ±35/q). About 30 % (≈ −48/q) is left unexplained, and it sits only on curves.
The smaller scatter of curves is carried by the 3rd moment, not the 4th: in random sets the 3-point
correlation fluctuates (z_line sd 1.06) and sets the k = 4 scatter (R² 0.92 at N = 11); in curves it is
pinned at ±0.41. The N = 12 "no difference" is a power problem plus a sign flip of the 3-point
term, not subfield cancellation.

## Verdicts (rules frozen in `PROTOCOL.md`; evaluated by the program, `summary.json` → `verdicts`)

| id | rule (short) | verdict | numbers (verbatim from `summary.json`) |
|:--|:--|:--|:--|
| I1, I2 | exact Walsh 3rd/4th-moment identities on every curve | **PASS** | all curves match, 1 distinct ΣW⁴ value per N; N = 9: 267909632, N = 11: 17171474432, N = 13: 1099377385472 |
| P1 | curves have lower normalised Walsh 4th moment than paired nulls | **PASS** | N = 9: curves 2.003902405398264 (sd 4.5e-16) vs nulls 3.017898522141077; N = 11: 2.0009763236153613 vs 2.979949697019886, diff 0.978973373404525 (95 % CI 0.9506 to 1.0065), Cohen d 13.65, Cliff 1, p = 1.0e-5; N = 13: 2.0002441257165615 vs 2.9997707820642376, diff 0.9995266563476761, p = 1.0e-5 |
| P2 | ratio₄ rises with the 4th moment (pooled, p < 0.01 at all N) and within nulls (p < 0.05 at ≥ 2 of 3 N) | **FAIL** (predicted in §0.3c of the protocol) | pooled Spearman N = 9 / 11 / 13: 0.444 / 0.435 / 0.302 (p = 1.0e-5 / 1.0e-5 / 0.0098); within nulls: 0.195 (p = 0.0029) / 0.029 (p = 0.342) / −0.118 (p = 0.794); within curves: undefined, kurtosis sd = 0 |
| P3a | A₂ = E₂ + D_line exactly | **PASS** | integer identity true for every set at N = 9, 11, 12, 13; max abs residual 7.6e-9 |
| P3b | first-order k = 4 prediction within a factor 2 and R² ≥ 0.5 | **PASS** (with a caveat: only 68–89 % of the deficit) | predicted/observed curve deficit 0.885 / 0.705 / 0.683 at N = 9 / 11 / 13; all-set R² 0.614 / 0.9185 / 0.962, slope 0.937 / 1.048 / 1.071; observed deficit ×q: 152.9 / 159.1 / 153.9, predicted ×q: 135.4 / 112.2 / 105.1 |
| positive control | planted rank-one Walsh bias raises the count with the 4th moment | **PASS** | Spearman 0.9444 / 0.9985 / 0.9940 over 20 planted sets (p = 1.0e-5 each); N = 11 η = 0.30: kurtosis 51.76, ratio₄ 9.65 |

## Effect sizes (curves against density-paired nulls, orig + fresh-seed populations)

| N | ratio₄ curves (n) | ratio₄ paired nulls (n) | difference, 95 % bootstrap CI | Cohen d | p (one-sided perm.) |
|:--|:--|:--|:--|:--|:--|
| 9 | 0.7013411861689629 (50) | 1.0190200234075077 (50) | 0.31767883723854484 [0.2301, 0.4052] | 1.40 | 1.0e-5 |
| 11 | 0.9223235921902894 (50) | 1.0163656592964478 (50) | 0.09404206710615848 [0.0801, 0.1081] | 2.62 | 1.0e-5 |
| 13 | 0.9812078211503522 (10) | 0.9942165716819922 (10) | 0.013008750531639968 [0.0054, 0.0205] | 1.43 | 0.0030 |
| 12 (composite) | 0.9884823152412485 (10) | 0.9982719980627464 (10) | 0.009789682821497836 [−0.0025, 0.0214] | 0.69 | 0.0715 |

Instrument checks: the parent's 104 `orig` ratios are reproduced with max |diff| = 0.0
(`replication_check.txt`); over all 200 / 200 / 50 nulls the mean ratio₄ is 1.0053737 / 1.0017403 /
1.0003116 at N = 9 / 11 / 13 (E_k unbiased).

## What was found

1. **Fewer additive quadruples: yes, exactly.** C₄′ = −(q−2)(q−4) for every curve, standardised
   z_plane = −4.60 / −9.23 / −18.47 at N = 9 / 11 / 13 (sd across curves ≤ 0.013), against
   N(0, 1) for nulls (sd 1.01 / 0.99 / 0.99). The deficit grows like 0.2·√q standard deviations
   but shrinks like 1/q in relative terms.
2. **Size of the effect.** Observed curve deficit ≈ 155/q at every N (152.9, 159.1, 153.9). First-order
   prediction → −(105 + 35ε)/q, ε = (−1)^{Tr a}. At N = 11, Tr(a) = 0: observed 0.0939, predicted
   0.0722; Tr(a) = 1: observed 0.0614, predicted 0.0373. The observed Tr(a) split (0.0325) matches the
   predicted one (0.0349); the common offset (≈ 47/q) is the unexplained third.
3. **Nulls obey the first-order formula, curves sit below it.** Over nulls the fit of observed on
   predicted has slope 0.947 and R² 0.924 (N = 11), slope 0.986 and R² 0.983 (N = 13); curves lie
   ≈ 48/q below the diagonal at odd N (figure 3) and on it at N = 12.
4. **Scatter.** sd of ratio₄: curves 0.0182 vs paired nulls 0.0474 at N = 11 (ratio 0.38); z_line sd
   0.413 vs 1.061 (ratio 0.39). Inside nulls, R² of ratio₄ on z_line is 0.915 (N = 11) and 0.976
   (N = 13), on z_plane 0.0016 and 0.0056. So scatter = rigidity of the 3rd moment, not the 4th.
5. **Composite N = 12.** C₃ = +(q+2) flips the line term's sign, so the first-order deficit is
   only 0.0165 (observed 0.0115, 10 curves vs 10 paired nulls, null sd 0.02): no contradiction with
   the parent's "no difference". Curves hold 85–131 all-lift F₄-subspaces of F₄-dimension 2 (nulls: mean 3.22, max 9;
   about 3), but that is at most 0.026 % of A₄; dropping them changes ratio₄ from 0.9875974815932574 to
   0.9873476161452932 (Koblitz curve). Subfield cancellation is not supported.

## Not established

- The remaining ≈ 30 % of the odd-N curve deficit (≈ 48/q, same sign at both Tr(a), constant in q·deficit).
  It needs higher-order correlations (rank-4 classes, size ≥ 5). Candidate: the 5- and 6-point
  correlations, whose Walsh sums are also curve-independent; not computed.
- Anything at N > 13 or k ≥ 5; N = 13 has only 10 curves. No ECDLP cost was measured: no
  leaderboard, benchmark or rho row.
- P2 as written does not hold; the 4th moment alone does not drive the scatter.

## Layout and reproduction

```sh
cargo build --release
./target/release/lift-repulsion-mechanism run reports/run-<UTC>/ --threads 3   # ~18 min, 748 sets
python3 plot.py reports/run-<UTC>/                                              # layout only
python3 -c "import typst; typst.compile('REPORT.typ', output='REPORT.pdf')"
```

`explore <N>` prints exact Walsh power sums of six curves (the check that suggested I1/I2). Disk
footprint of the crate with `target/`: about 1 MB. Executed locally (exact counting at N ≤ 13), not on
isolab-eks; no timing claim is made. The CLAUDE.md session log and Conductor task were not touched
because this work was confined to this directory.
