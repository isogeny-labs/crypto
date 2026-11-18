# Read-only budget telemetry

`isolab-budget-telemetry.yml` uses the same configured OIDC trigger role as the
existing IsoLab workflow, narrowed by a session policy to identity readback and
`budgets:ViewBudget` on the three named existing budgets. It runs only on
`main` in `isogeny-labs/crypto`, on a GitHub-hosted runner. It neither changes
IAM/budgets nor starts builds, publishes images, launches compute or contacts
the private Kubernetes API. It creates no new identity or permission grant.

After this workflow has been reviewed and merged, the existing GitHub caller can
dispatch it without needing direct AWS credentials:

```sh
gh workflow run isolab-budget-telemetry.yml --repo isogeny-labs/crypto --ref main
```

Record the original dispatch/run ID before any retry. The bounded artifact
contains role-configuration/auth outcomes, an account-number-only identity
receipt, queried budget values, API stderr and workflow/source provenance.
It excludes credentials, role session tokens, subscriber/contact lists and
notifications and cost-filter identities. Authentication or API failure remains a failure and preserves
whatever receipts were obtained. An existing role that lacks budget access
is not broadened by this workflow.

The two daily budget names describe the **same** allowance; never add their
limits together. Monthly and daily telemetry must both be considered alongside
the lane's charged/reserved spending and the existing finite attempt ceiling.
These names and the expected account come from [COST_CONTROLS.md](COST_CONTROLS.md).

Every summary has `launch_admitted=false`. Successful telemetry collection is
not readiness to spend: AWS Budgets cost data is delayed, lane reservations
and live capacity remain unknown, budget filter coverage has not been audited,
and worker/source/input/registry binding and
the construction's resource feasibility are separate obligations. Unknown
spend is not converted to zero or a fabricated remaining balance.

`checked_at` is when the API was read. The budget's `LastUpdatedTime` describes
a budget update; it does **not** establish the age of the reported spend.
The script preserves monetary amounts as reported rather than inferring a
fresh spend timestamp or calculating a launch decision from them.

Primary references checked:

- [AWS CLI DescribeBudget](https://docs.aws.amazon.com/cli/latest/reference/budgets/describe-budget.html),
  including the meaning of `LastUpdatedTime`.
- [AWS Budgets update behavior](https://docs.aws.amazon.com/cost-management/latest/userguide/budgets-managing-costs.html).
- [Budget API/IAM mapping](https://docs.aws.amazon.com/service-authorization/latest/reference/list_budgets.html):
  `DescribeBudget` maps to the read action `budgets:ViewBudget`.
- [AWS credentials action session policies](https://github.com/aws-actions/configure-aws-credentials#session-policies):
  an inline session policy restricts an existing role without changing IAM.

The existing repository variables API returned HTTP 403 in this session. The
workflow checks configured presence server-side and never prints the variable's
value. This document makes no claim that the role is present or budget access
has succeeded until an actual run and its receipts establish that scope.
