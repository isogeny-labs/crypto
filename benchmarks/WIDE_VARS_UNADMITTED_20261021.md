# Retained wide-variable engine benchmark report

This row is preserved verbatim from PR #2335 head
`7e7a94e2bfac28c0f58bb167512a28f6871a3857`. Its report, figures,
raw runs, and original analysis remain unchanged. The benchmark directory
does not yet carry the complete replay certificate, validator report, and
matched-reference evidence required by `VALIDATION.json`, so the row remains
outside the admitted-results table. Its historical wording is not a new
validation receipt or a newly admitted performance claim.

| date | entry | measured | headline (as reported) | host | study |
|:--|:--|:--|:--|:--|:--|
| 2026-10-20 | [`20261020-ic-m4-narrow-engines-wide-vars`](20261020-ic-m4-narrow-engines-wide-vars/) | amendment 2 of the m ≥ 3 study: decomposition cost per relation (wall ns, one core) of the narrow engines (inherited F4, F6-IC, CDCL+XOR SAT on the chained-S₃ system) in a `--features wide-vars` build (`Mask` = `u128`, ≤ 128 unknowns), m = 4, n = 11–31, against the pair-table MITM floor. F4 and F6-IC measured to n = 29 (90 unknowns), n = 31 a censored lower bound (≥ 40.48 log₂ ns); slope 0.644 / 0.645 on n = 11–29 vs floor 0.298 on the same cells (Δ [0.313, 0.372] / [0.316, 0.371]): grows faster than the floor; SAT undetermined. Not an improvement over the baseline | cloud container, 4 vCPU Xeon @ 2.1 GHz, L0 (shared, one grid) | [study](../research/ic_m3_algebraic_vs_mitm_20261014/AMENDMENT_2.md), [report](20261020-ic-m4-narrow-engines-wide-vars/REPORT.pdf) |
