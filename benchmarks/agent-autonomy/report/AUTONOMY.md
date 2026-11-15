# Agent autonomy report (crypto.autonomy/v1)

Episodes: 3. Generated 2026-11-15T03:39:03Z. All figures are [pcc:unverified] until a pcc claim cites them.

## Unsupervised success

Rate = unsupervised successes / (successes + failures). Pending episodes are counted apart and left out of the rate. Interval: Wilson 95%.

| task class | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| all | 3 | 0 | 0 | 3 | n/a | 0-100% |
| docs-tooling | 3 | 0 | 0 | 3 | n/a | 0-100% |

## Interventions and recovery

- Redirects per episode: mean 0.00, median 0
- Unblocks per episode: mean 0.00, median 0 (reported apart, not counted against)
- Hook messages: 1 (neutral)
- Self-recovery: 2/2 incidents fixed without a human message (100%, 95% CI 34-100%)
- Classifier accuracy on the human-audited sample: no audited messages yet

## Cost

No tokens or USD recorded.

## Weekly series (by week the episode ended, Monday start)

| week | episodes | unsupervised success | fail | pending | rate | 95% CI |
|:--|--:|--:|--:|--:|--:|:--|
| 2026-11-09 | 3 | 0 | 0 | 3 | n/a | 0-100% |

## Episodes

| episode | state | reason |
|:--|:--|:--|
| E-6b87f147cfd6 | pending | revert window not elapsed |
| E-c722c55f5d77 | pending | revert window not elapsed |
| E-eb0c1fa40374 | pending | partial, PR not merged |
