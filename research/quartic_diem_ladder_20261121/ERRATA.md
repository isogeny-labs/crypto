# Errata to RESULTS.md (from the independent validation)

`RESULTS.md` is left byte-equal. The validator's report is `validation/validator-report.md`, disposition
**validated**: 210 of 210 certificates were checked with independent Pic^0 arithmetic, 72 of 72 replayed
rows are identical, and the analysis reproduces byte for byte. These corrections take precedence over
`RESULTS.md`. None of them changes `analysis.json` or a frozen verdict. Every number is `[pcc:unverified]`.

- **E1 (F8): protocol deviation, wrongly disclosed.** RESULTS says the 131101 cell would have used the same
  curves under the frozen 110-minute search cap. It would not: curve 288 was admitted at about 139 minutes
  and curve 228 at about 110.0 minutes, right on the cap. Under the frozen rule the cell has 3 or 4 curves.
  For sfb at w = 0 / 0.1 / 1:

  | cell | w = 0 | w = 0.1 | w = 1 |
  |---|---|---|---|
  | as reported (5 curves) | 0.000192 | 0.775 | 7.75 |
  | without 288 | 0.000179 | 0.746 | 7.46 |
  | without 288 and 228 | 0.000196 | 0.781 | 7.81 |

  The verdict is UNRESOLVED in every case.
- **E2 (F9): "below rho at w = 0.1 at 131101" is a band-point reading, not a ranking.** The frozen band
  rule and AGENTS.md §5a require the order to hold at all three w. Validation does not license a "below
  rho" statement; RESULTS "Next steps" item 1 implies otherwise.
- **E3 (F10): the 0.683 needs its unit.** It is the pooled paired sfb/new ratio in FPM mid, which excludes
  additions. In band units it is:
  - 0.846 [0.747, 0.964] in group operations (w = 0);
  - 0.606 [0.573, 0.639] in all native operations.

  The 85% `la_add` share is specific to sfb at 131101; for new it is 0.55.
- **E4 (F11): labels.**
  - "Ranked at count-calibrated price" is not §5a's timing-based calibrated price.
  - The FPM reading of about 1/1000 excludes additions; with them it is 0.0027.
- **E5 (F7): the rho reference is conservative toward rho.** Executed rho runs average 1.007 × sqrt(pi r/4)
  in steps, but their native work is not emitted. Charging rho 1 native op per step gives sfb at 131101 =
  0.70 (w = 0.1) and 3.87 (w = 1), which is still UNRESOLVED.
- **E6 (F12): minor corrections.**
  - A Pic operation costs 3,990–4,795 field multiplications per curve, not 3,600–4,000.
  - The builds took 19 and 23 min, not 45.
  - The 5-curve intervals are narrow: the 65537 sfb t-interval at w = 0.1 is [0.76, 1.05].
- **E7 (F13): repository policy.** `raw/run.jsonl` (242 KB) belongs in the artifact store once store
  credentials work.
