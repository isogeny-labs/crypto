# Saved-work report index

All reports describe analysis of existing data on a shared local host. No
isolation tier, wall/CPU/RSS measurement, fresh legacy run or DLP is admitted.
Counts are exact finite observations, with labels visible and no statistical
interval. The primary flow is one planned, attempted and completed analysis;
zero incomplete, interrupted, resumed or excluded runs. Validation and proof
development do not create additional historical curve samples.

| Record | Report |
|---|---|
| Preparation, synthetic parser controls and remote-byte gate | [preparation](run-preparation/REPORT.md) · [PDF](run-preparation/REPORT.pdf) |
| Saved native analysis001, source b1f58e9d8e08bb3a9e1794166cd69e7556bbffa3 | [analysis001](run-analysis001/REPORT.md) · [PDF](run-analysis001/REPORT.pdf) |
| Independent saved validation002, source 27018452d455be75b27167812cb0d8f11a2876d4 | [validation002](run-validation002/REPORT.md) · [PDF](run-validation002/REPORT.pdf) |
| All six proof/auditor attempt directories, including failures | [proof checks](run-proof/REPORT.md) · [PDF](run-proof/REPORT.pdf) |

The [main report](../REPORT.md) and [PDF](../REPORT.pdf) embed both comparisons
and the four descriptive PDFs. [Graph sources](graph-source.json) bind figure
data to native outputs. The [renderer](render.py) only plots recorded values
and typesets existing prose; it performs no scientific computation.

The ten-count inequality and monotone-budget implication are order statements,
not polynomial identities. The identity.certificate/v1 format in
docs/identity-certificates/README.md certifies polynomial equalities over a
fixed finite field and does not represent this claim. The exact applicable
formal evidence is [WorkLimits.lean](../lean/WorkLimits.lean), its checked
axiom report and [proof map](../proof/PROOF_MAP.json).
