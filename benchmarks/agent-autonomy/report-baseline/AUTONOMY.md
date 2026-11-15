# Agent autonomy report (crypto.autonomy/v1)

Episodes: 8 ended since 2026-11-01 until 2026-11-14. Generated 2026-11-15T06:44:44.700734Z. All figures are [pcc:unverified] until a pcc claim cites them.

## Unsupervised success

Rate = unsupervised successes / (successes + failures). Pending episodes are counted apart and left out of the rate. Interval: Wilson 95%.

| task class | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| all | 8 | 0 | 4 | 4 | 0% | 0-49% |
| ci-repair | 1 | 0 | 0 | 1 | n/a | 0-100% |
| docs-tooling | 3 | 0 | 2 | 1 | 0% | 0-66% |
| measurement | 2 | 0 | 1 | 1 | 0% | 0-79% |
| other | 1 | 0 | 0 | 1 | n/a | 0-100% |
| research | 1 | 0 | 1 | 0 | 0% | 0-79% |

## Interventions and recovery

- Redirects per episode: mean 0.25, median 0
- Nudges per episode (agent stopped, human restarted it; counted against): mean 0.62, median 0
- Unblocks per episode: mean 0.38, median 0 (reported apart, not counted against)
- Hook messages: 14 (neutral)
- Self-recovery: 0/0 incidents fixed without a human message
- Classifier accuracy on the human-audited sample: no audited messages yet

## Cost

No tokens or USD recorded.

## Weekly series (by week the episode ended, Monday start)

| week | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| 2026-10-26 | 1 | 0 | 0 | 1 | n/a | 0-100% |
| 2026-11-02 | 6 | 0 | 3 | 3 | 0% | 0-56% |
| 2026-11-09 | 1 | 0 | 1 | 0 | 0% | 0-79% |

## Episodes

| episode | state | reason |
|:--|:--|:--|
| E-13aabab301aa | fail | redirect |
| E-142a64a3fb07 | fail | redirect |
| E-1a696f987a18 | pending | revert window not elapsed |
| E-258cd7eeddb2 | pending | revert window not elapsed |
| E-2c4562259141 | fail | nudge |
| E-b54b9a8a1313 | fail | nudge |
| E-e5c3f9c38936 | pending | revert window not elapsed |
| E-fe9c4c6af366 | pending | revert window not elapsed |
