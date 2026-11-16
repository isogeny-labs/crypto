# Publication closeout — 16 November 2026

[Results PR4358](https://github.com/isogeny-labs/crypto/pull/4358) merged at
03:20:47 UTC as `a79fc35e35c1be023e8c862cb74128b356c4ef30`. The Rust port had
already merged in PR4340. The requested local scale matrix, failures, repair,
reports and proposed hand degree bound are published.

**This was not a green-CI merge.** The final head was an externally added
`ci: refresh required checks` commit, `e1e9ee8a0705468f8327e56c9684813c274f5a97`.
Its tree is identical to the last producer head `a29a4183ba`. Required jobs
completed after the merge with failures:

- Artifact tool tests: one failure and five errors, including missing
  `wire_workflows` helper functions.
- PCC: the job compared head1190 ledger entries with then-current main1195;
  the moving base had advanced. This is not evidence that frozen claim files
  were changed.
- Validation-gate tests: two failures in shared committed-tree/pre-cap report
  checks, including unrelated `ic-aimed-gather` evidence.

The [closeout receipt](scale-20261115T2053Z/closeout-20261116.json) records exact
job URLs, timestamps, failed steps and scope. Size, records, learning, review,
replay and both Rust clippy jobs reported success. Some evidence-audit shards
failed. No full-CI or independent-evidence pass is asserted [pcc:unverified].

The recorded local native tests and arithmetic replay retain their original
scope. Measurements and the hand proof remain **[pcc:unverified]** pending
independent signed validation. Neither merge status nor this metadata closeout
promotes a canonical leaderboard/index row or establishes a timing advantage.

The results-PR follow-up automation is paused. No benchmark was rerun or cloud
experiment submitted. Shared CI repairs are outside this completed publication
follow-up; these failures are recorded for the repository maintainers.
