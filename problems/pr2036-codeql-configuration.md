# PR2036 CodeQL configuration and interrupted-runner checks

- **Status:** open
- **Kind:** repository feature/access configuration; interrupted runner/resource limit
- **Affected work:** PR2036 current-head CI acceptance
- **Next action owner:** repository owner for feature configuration; PR author for fresh-head verification
- **Last updated:**2026-10-23

## Problem

PR2036 cannot yet claim green CI because CodeQL uploads report disabled repository
code scanning and the Rust cache-warming runner shut down before its handoff.
These are separate from the N83 SAT/arena implementation and native run validation.

## Evidence and uncertainty

Run39905421370 at c13e33dc: Actions/Python analysis upload errors say code scanning
is not enabled; repository metadata reports private=true. The token does not
expose security_and_analysis settings. The Rust warm-cache job received a runner
shutdown; dependent jobs correctly rejected incomplete analysis. Full source/query
coverage, hash-bound handoff and upload checks remain unchanged. Raw logs are in
the N83 continuation evidence bundle. No mathematical obstruction is established.
Whether settings have since changed or fresh runner capacity resolves shutdown
must be checked on the next head. No paid feature or infrastructure change has
been made implicitly. Two other jobs report ENOSPC before producing step logs;
validation-gate also failed its full-history checkout due disk exhaustion.

## What would unblock it

- [ ] Repository code-scanning feature is available/enabled for this private repo,
  with the existing workflow authorized to upload its complete SARIF results.
- [ ] A current-head full Rust database build, cache-warming handoff and all
  complete query analyses succeed, without coverage or validation exemptions.
- [ ] Fresh current-head jobs run on healthy capacity and all required checks pass.

These establish CI execution/acceptance only, not an N83 relation or speed result.

## Next discriminating step

Push the verified browser/cpuset/blobless-checkout repairs with native continuation
evidence. Inspect the resulting head once scheduled; preserve failures. If feature
errors persist, obtain the owner's configuration decision; do not disable scans
or turn failed validation into success. Honor existing CI budgets and the research
job-submitter role; do not scale or restart unrelated jobs. Stop a given attempt
when its existing workflow deadline is reached. Recheck when new head/settings or
runner capacity changes. N83 worker allowance remains30000seconds.

## Resolution and history

Unresolved. Recording this problem does not finish the CI requirement.
