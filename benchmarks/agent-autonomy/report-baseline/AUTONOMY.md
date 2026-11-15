# Agent autonomy report (crypto.autonomy/v1)

Episodes: 3 ended since 2026-11-01 until 2026-11-14. Generated 2026-11-15T04:22:32.877457Z. All figures are [pcc:unverified] until a pcc claim cites them.

## Unsupervised success

Rate = unsupervised successes / (successes + failures). Pending episodes are counted apart and left out of the rate. Interval: Wilson 95%.

| task class | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| all | 3 | 0 | 1 | 2 | 0% | 0-79% |
| ci-repair | 1 | 0 | 0 | 1 | n/a | 0-100% |
| docs-tooling | 1 | 0 | 1 | 0 | 0% | 0-79% |
| other | 1 | 0 | 0 | 1 | n/a | 0-100% |

## Interventions and recovery

- Redirects per episode: mean 0.33, median 0
- Unblocks per episode: mean 0.00, median 0 (reported apart, not counted against)
- Hook messages: 1 (neutral)
- Self-recovery: 0/0 incidents fixed without a human message
- Classifier accuracy on the human-audited sample: no audited messages yet

## Cost

No tokens or USD recorded.

## Weekly series (by week the episode ended, Monday start)

| week | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| 2026-11-02 | 3 | 0 | 1 | 2 | 0% | 0-79% |

## Episodes

| episode | state | reason |
|:--|:--|:--|
| E-13aabab301aa | fail | redirect |
| E-e5c3f9c38936 | pending | revert window not elapsed |
| E-fe9c4c6af366 | pending | revert window not elapsed |
