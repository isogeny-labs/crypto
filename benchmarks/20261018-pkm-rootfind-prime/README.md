# Root finders against tower F4 and signature F5 on prime curves (2026-10-18)

Copies for browsing. The study directory is the source of truth: [protocol](../../research/pkm_tower_rootfind_20261018/PROTOCOL.md), [amendment](../../research/pkm_tower_rootfind_20261018/AMENDMENT_1.md), [result](../../research/pkm_tower_rootfind_20261018/RESULT.md), [analysis](../../research/pkm_tower_rootfind_20261018/ANALYSIS.json), sessions and replay audits.

- `run-report.pdf`: the run report (Typst), with `s_per_arm`, `fp_multiplications` and `inversions` as SVG and 150 dpi PNG.
- `VALIDATION.json`, `replay-certificate.json`, `validator-report.json`: the validation gate's evidence. The validator (`research/pkm_tower_rootfind_20261018/validate.sh`) recomputes the headline medians from `records.jsonl` with jq, independent of the analyser.
