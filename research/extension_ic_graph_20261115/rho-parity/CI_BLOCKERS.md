# CI evidence and publication status for PR4493

GitHub subsequently reports PR4493 merged at2026-11-17T04:24:23Z by account
`aburan28`, merge commit `a17eebb027ab764a07f8a84e0e2bd701bf11d85e`, head
`98ee1d89e068bd59a490f8e722f2eee0d7f66169`. This session invoked no merge
operation. The account identifier does not establish which person or
integration caused the merge. The final head differs from the inspected
`4e4450d7` only in `docs/session-logs/2026-11-17.md`; scientific source, raw
records, report and claims are unchanged.

The failures below predate that merge. No passing required-gate assertion is
made. The branch was deleted before this metadata receipt could be pushed;
this follow-up retains the observed failure and publication chronology.

Observed head: `4e4450d776d6d329df03518c78af16ba8954d351`.
Main through `e7425e39f1b12148f272f94113f7a12cf2cd89c2` is integrated;
GitHub reports the PR mergeable. This is not a green-check assertion.

The following checks stop during required shared artifact downloads, before
validation of their inputs:

- [recompute job 133815847383](https://github.com/isogeny-labs/crypto/actions/runs/44016579721/job/133815847383): `InvalidAccessKeyId` from S3 GetObject, exit2.
- [current job 133815846949](https://github.com/isogeny-labs/crypto/actions/runs/44016579543/job/133815846949): `InvalidAccessKeyId` from S3 GetObject, exit2.
- [names job 133815846597](https://github.com/isogeny-labs/crypto/actions/runs/44016579489/job/133815846597): `InvalidAccessKeyId` from S3 GetObject, exit2.

Literal shared error:

```text
An error occurred (InvalidAccessKeyId) when calling the GetObject operation:
The AWS Access Key Id you provided does not exist in our records.
```

No credential value is reproduced. The affected pulls include existing
`docs/backend-leaderboard`, rootfinding fixtures and `docs/curves`; those
paths and credential settings are unchanged by this PR. No comparison was
retried and no required download or validation gate was skipped. Repair of
CI's external artifact credentials is outside this source change.

The auto-merge connector also selected squash merging, which the repository
disallows. No repository merge setting was changed. The component source,
raw records, PDF and claims retain their frozen bytes; scoped native tests
and Clippy remain the recorded local checks. Full-pipeline rho parity remains
unmeasured. Further paid execution also needs a known remaining budget.
