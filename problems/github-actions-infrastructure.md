# GitHub Actions billing, action policy and runner availability

- **Status:** open
- **Kind:** access/tool and infrastructure limits
- **Affected work:** current-main CI and merging all open research PRs in crypto, cryptanalysis and crypto-autoresearcher
- **Next action owner:** organization billing/Actions administrator; code repair by the current session
- **Last updated:** 2026-10-27

## Problem

Required hosted jobs are rejected before starting, selected third-party toolchain actions are prohibited by organization policy, and crypto EKS jobs wait without a runner. Local software repair cannot establish successful hosted CI under these conditions.

## Evidence and uncertainty

- Main crypto-autoresearcher validate run 40361157179 and merge-sweeper run 40520577997 have jobs with no steps and no runner. Their failure annotation says recent account payments failed or the spending limit must be increased.
- Cryptanalysis run 40400741546 reports leanprover/lean-action@v1 disallowed; run 40400739840 reports dtolnay/rust-toolchain@stable disallowed. Their workflow graphs contain no jobs.
- Crypto run 40507630583 has a queued scope job labeled crypto-eks-linux and no assigned runner.
- Runner administration and branch-protection inspection return HTTP 403 to the current integration. No EKS administration credentials are configured in this environment.
- The exact billing account state, full action allowlist and EKS runner health are not visible here. These observations do not establish code correctness or any mathematical result.

## What would unblock it

- [ ] Organization administrator resolves the account payment/spending-limit gate and a hosted job actually starts.
- [ ] Administrator approves the needed actions, or reviewed approved replacements execute the same checks successfully.
- [ ] An authorized runner administrator restores the configured EKS runner pools; queued jobs actually acquire runners.
- [ ] Final PR heads and final main commits finish applicable checks and required reviews successfully, with no bypasses.

Infrastructure restoration is necessary, not sufficient: local integration repairs and individual PR review are still required. See [the repair report](../docs/ci-repair-20261027.md) for delivered changes and exact validation scope.

## Next discriminating step

After each administrator change, inspect one affected run's annotations, job steps and assigned runner, then rerun failed jobs only when the recorded infrastructure cause has actually changed. Once they execute, repair genuine software failures from their logs. Recheck on administrator confirmation or new run activity. Preserve prior failed runs and immutable research evidence.

## Resolution and history

2026-10-27: infrastructure causes confirmed, user informed, local repairs and scoped PR delivery continue. Unresolved; this record does not mark the repository-wide request complete.
