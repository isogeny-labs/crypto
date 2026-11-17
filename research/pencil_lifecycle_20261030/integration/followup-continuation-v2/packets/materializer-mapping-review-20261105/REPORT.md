# Independent materializer mapping correction review

[pcc:unverified] Exact published candidate ec632eadbadadae6d378162e3334d6dcae9e6068 changes only the RLIMIT_AS assignment to a comment. Source comparison with the published predecessor confirms all other bytes are unchanged. No blocking defect was found in this narrowly scoped source delta; machine status remains not_verified. This review does not release another attempt.

## Native record and causal scope

The previous receipt records Git ls-tree exiting 128 with a pack-mapping Cannot allocate memory diagnostic, zero materialized files and no compiler invocation. Three child groups were reaped and absent according to the native receipt. Recorded cgroup controls were CPU 200000/100000, memory 4294967296, swap 0, pids 512. The proposal correctly treats virtual-address pressure as a possible explanation rather than a measured definitive cause. Pack size alone cannot establish the failing mapping length because Git supports mapping windows. Exact mapping length, RSS demand and the behavior after this correction remain unknown.

## Source trace

Candidate line 10 removes the explicit four-GiB address-space limit. Lines 61–66 require the expected cgroup name and CPU/memory/swap/task bounds before lines 67/75 start external commands. Lines 19–51 retain deadline checks, lease heartbeats and bounded owned-group cleanup. Input pins at 68–70 and final hash checks at 163–168 are unchanged. File/path/vendor/cache limits remain unchanged. These are source observations, not fresh runtime enforcement measurements.

Removing setrlimit does not raise an inherited limit. A future admission should retain the effective inherited address-space limits or expressly preserve that unresolved risk. The proposal directory intentionally lacks a runnable admission/config/helper bundle: it is a source amendment, not an executable release. The existing failed admission cannot be reused. Fresh paths, lease, exact publication binding and finite additional-attempt release remain required. This is an execution-binding dependency, not a newly discovered defect in the one-line correction.

## Method and evidence

Preread plan SHA256 3dd0910d0e3ee5d3821392eeacb68e8af712b47fd24d0cf8e5f1c07e474a2751 was recorded before candidate inspection. Reviewer did not author candidate; prior materializer review and disclosed failure/proposed direction were known, so the review was not blind to motivation. No test, mock, compiler or materializer was invoked. No scientific status is promoted. The used attempt remains consumed; an additional 900 seconds is only proposed, under the unchanged November 6 04:00 UTC deadline.

Primary references: Python resource documentation (https://docs.python.org/3/library/resource.html#resource.RLIMIT_AS) distinguishes virtual-address limits; Git configuration (https://git-scm.com/docs/git-config#Documentation/git-config.txt-corepackedGitLimit) describes mapping windows and simultaneous mapped-pack limits. KB was unavailable per coordinator; exact published code and native receipts were used. Machine-readable results: result.json and joints.csv.
