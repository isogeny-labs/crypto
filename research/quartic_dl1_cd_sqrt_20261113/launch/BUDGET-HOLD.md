# Launch held: monthly budget exceeded (2026-11-13 17:23Z)

Read-only `aws budgets describe-budgets` (account 104186811093, caller coreuser) before submission:

| budget | limit USD | actual USD |
|:--|--:|--:|
| isolab-daily-cost-100 | 200 | 123.13 |
| spot-experiments-daily-25 | 200 | 37.69 |
| isolab-monthly-account-cost | 1,300 | 1,679.93 |

The monthly remainder is negative, so by `taskq/deploy/eks-karpenter/COST_CONTROLS.md` ("If the
remainder is unknown or exhausted, do not start a new build/job") the Job in `request.json` was
**not submitted**. Already done before the check: the image in `image.json` was published to
`isolab/tasks` (one 62 MB layer; ECR storage only). Nothing else was spent.
The request is ready to submit unchanged once the owner authorises it or the budget allows.
