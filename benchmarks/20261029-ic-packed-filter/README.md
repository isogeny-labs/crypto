# 2026-10-29: membership filter on the packed pair table (IC component speed-up)

Copies for browsing. The study directory
[`research/ic_packed_filter_20261029/`](../../research/ic_packed_filter_20261029/README.md)
is the source of truth: raw sessions, analysis, cards, validation and the figure source.

- [`ic-packed-filter-report.pdf`](ic-packed-filter-report.pdf): the study report (from `research/ic_packed_filter_20261029/REPORT.typ`).
- [`summary.svg`](summary.svg), [`summary.png`](summary.png) (150 dpi): the summary figure (wall statistic per size; counted distance to matched rho at w = 0, 0.1, 1).
- [`VALIDATION.json`](VALIDATION.json): the validation-gate file of this row. It pins the independent replay certificate and validator report of TASK-20261029-654f73 and the matched `rho-neg` reference with its §5a band, for the two P32 curves only.

**Headline (32 bits, validated with caveats, single session, not replicated).** The measure is whole-solve wall per charged GAE at N_inc, filtered against the unfiltered packed table (`mitm-prime-blocked:m=2,filter=1` against `mitm-prime-blocked:m=2`).
- **Corrected ratio:** 0.9601 [0.9569, 0.9633], at identical charged GAE.
- **Beside it:**
  - `bitop/v1` +0.301% (build-side only; the probe test and the build's pass over the empty slots are not counted);
  - S′ `dgbf / dgb` 1.0 / 1.0505 / 1.3586 at w = 0 / 0.1 / 1 (`dgbf` costlier in the counted unit at w > 0; `table_insert_attempts` unpriced on every IC arm; these figures decide nothing).
- **Distance remaining** to matched rho: `dgbf / rho-neg` 31.47 / 31.50 / 31.63. Rho is cheaper at every weight.
- **Other sizes:** 24 bits read a regression (1.0032 [1.0006, 1.0058]); 28 bits is undecidable (infrastructure).
- **Host:** c8a.2xlarge (AMD EPYC 9R45) on EKS, L2, two curves, 288 pairs.
