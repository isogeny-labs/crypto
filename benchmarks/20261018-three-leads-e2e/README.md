# Fresh end-to-end checks of three ECDLP leads

The [study](../../research/three_leads_e2e_20261018/README.md) is the source of truth. Its [claim table](../../research/three_leads_e2e_20261018/CLAIMS.md) adjudicates every supplied numerical statement. This directory mirrors the main report and SVG/150-dpi PNG figures, including the instruction A/A report.

23,952 primary solves, 84 independent/unfiltered controls and 168 trace-filter controls all verify. Both genus-2 cohorts contain 1,260 targets, one indexed by field size and one by actual subgroup size through 40 bits. The actual-40-bit subgroup arithmetic ratio to negation rho is 1.421–18.362 (default 11.091), with rho cheaper in all 12 price cells. Instrumented retired instructions give 7.487 [6.657,8.372]. These units are separate; L0 wall times are unqualified.

The [deterministic raw archive](../../research/three_leads_e2e_20261018/runs.tar.xz) contains every individual session's PDF, SVG and PNG, the 945-page report bundle, sealed runs, logs, replay receipts and diagnostic failures. Extract it at the study root and open `reports/INDEX.md`. The [evidence guide](../../research/three_leads_e2e_20261018/EVIDENCE.md) gives extraction, checksums and exact replay details. The archive is linked once to avoid a duplicate large artifact.
