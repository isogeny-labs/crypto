## 2026-11-05 — Checked batch reciprocals for initial source windows (T-1692)

### What was done (commands, files, features)

Added an identity-aware reciprocal batch and a checked supplied-inverse value-division constructor. Initial windows use the batch; incremental extra characters retain the scalar path. The native suite passes 125 tests, including 165 batch-kernel cases and 20 frozen recovery agreements. Three accounting tests pass with 185 imported tests filtered; all 20 outputs pass forward point verification. Native/accounting source locks contain 26/36 entries. [pcc:unverified]

### Decisions made (and why)

Use established Montgomery batch inversion and retain all curve, point and reciprocal checks. A nonempty batch has 3(m-1) products and one inverse; checking supplied reciprocals adds m products. Account for new allocation/validation work without a timing claim. Filed unsigned claims PCC-20261105-af2637 and PCC-20261105-8190fe, local verdicts not_covered. Sources checked (KB): unavailable tools/wrapper; used prior accounting/source evidence and ff batch inversion source. [pcc:unverified]

### Open threads (unfinished, pick up next time)

Investigate private validated input objects to reuse source validation without weakening public constructors, then shared polynomial work across shifted points. Coefficient-field enumeration and dense synthesis remain a separate representation task. Original 4,563-case cohort preserved. No cloud or timing panel. PR3493 merged during this round; publish the new component as an incremental PR. Current-head CI/review pending.

### Errors (what went wrong, how it was handled)

No native/generator failure. Native and accounting builds retain 15 and 22 warnings respectively. Raw counter projection includes the test-prefixed first row and asserts one-to-one marker coverage. Published predecessor artifacts remain unchanged.
