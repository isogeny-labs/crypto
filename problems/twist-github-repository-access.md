# Twist experiment delivery: GitHub repository access

- **Status:** resolved for canonical repository access and evidence delivery
- **Kind:** access/tool limit; local computation is available
- **Affected work:** [PR #2380](https://github.com/aburan28/crypto/pull/2380)
- **Last updated:** 2026-10-25

## Problem and observed evidence

When resuming the E26 encoding pilot from local commit
`88a27612152e9fdd74c1ed0a17f964442c4c3159`, fetching the existing PR branch
through the previously working Git credential helper returns HTTP 403,
"Write access to repository not granted." The connected GitHub PR read
returns HTTP 404. The precise credential or installation cause is unknown;
this is not a mathematical obstruction or an automatic approval rejection.
No credential values are inspected, retained or placed in reports.

Local source, tests, frozen protocols and bounded computation remain
available. Preserve finished work as local feature-branch commits. Do not
claim a successful push, updated PR or current remote head without checking.

## Exit conditions and next step

- Restore the connected identity's access to `aburan28/crypto`.
- Read the actual PR head and fetch it; reconcile any remote changes without
  force-pushing or rewriting frozen evidence.
- Push the preserved local work to the existing PR branch, update its report
  links and inspect current-head CI and review.

These checks close delivery only. Original scientific, cost, heldout,
historical file-size and review requirements remain separate.

The user was informed through the asynchronous input channel while local
experiments continued. Recheck when connection readiness changes or before
the final delivery attempt; repeated access failures alone supply no new
evidence about the experiments.

## 2026-10-25 recovery

A final explicit `gh auth git-credential` fetch succeeds, and `gh pr view`
reads the open, ready PR at the expected head `88a27612152e9fdd74c1ed0a17f964442c4c3159`.
The canonical returned URL is `https://github.com/isogeny-labs/crypto/pull/2380`.
The old `aburan28/crypto` connector path still returns 404; the canonical
`isogeny-labs/crypto` connector read succeeds as well. The exact earlier
credential failure cause is unknown; the CLI now provides the required route.
The fetch log and subsequent actual push/head/CI state are recorded in
[the E26 checks](../research/twist_routes_1_3_20261021/execution-v2/block-checks-20261025/).
Do not infer that the scientific or historical file-size holds are resolved.

Evidence/index commit `02e233a645e090f374aad0c7260ee7a91e46c4cb` was pushed
without force to the existing branch. The canonical connector confirms that
exact PR head, and the PR title/body update succeeds with draft=false.
The [delivery receipt](../research/twist_routes_1_3_20261021/execution-v2/block-checks-20261025/DELIVERY.json)
records the push and exact-head CI observation: review queued, conflicts with
main, no all-green conclusion. Access and delivery are resolved; CI/review,
merge conflicts, the historical size-policy hold and scientific gaps remain.
