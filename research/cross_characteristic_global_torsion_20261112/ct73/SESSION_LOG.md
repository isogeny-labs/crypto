# CT73 scoped session — 2026-11-13 UTC

## 14:49–15:00Z — Prior state, exact controls and publication

### What was done (commands, files, features)

- Inspected crypto root #3877, stacked #4015 at initial head f5475b234331a7562a644aa704de639849488d50, latest dated root log, AGENTS.md, CT69, CT71, CT72 and existing CT72 CI; queried related cryptanalysis/crypto-autoresearcher PRs and code. KB tools and checked-out catalog were absent.
- Consulted Snowden's classical finite-étale prime-to-characteristic torsion specialization notes; wrote a scope-specific theorem correcting the interpretation of CT72 mixed-trace alignment.
- Implemented two independent finite-field/linear-algebra test paths: Python bitwise multiplication, JS carryless polynomial convolution and reduction. Ran python3 -B/-O -B and node/--no-opt locally. All four outputs matched byte-for-byte with SHA256 717eca797dcbe13c8b0bd9719fc916813766b7025c38a5af1fd4cc7252cdfc88.
- Exact CT69 8-by-8 test: 8 correct scalar 2, 8 wrong scalar 1, 48 outside the selected 3-subgroup. Abstract full GL2 tests at ell 3,5,7,11 passed.
- Committed scoped Python, JS, report and bounded CI workflow to the existing stacked PR #4015 feature branch, without editing concurrent native genus-2/genus-3 projects.

### Decisions made (and why)

- CT72's relative Frobenius u is not *intrinsically* required for a DLP-preserving transfer; a common algebraic torsion labeling and same target residue embedding are required. The unimplemented problem is uniform evaluation on arbitrary encoded inputs.
- Keep claims tagged [pcc:unverified] until actual independent signing. No speed, cryptographic-scale, rho or index-calculus superiority assertion. No paid compute.

### Open threads (unfinished, pick up next time)

- Inspect exact-head CI and PR review for CT73; independent PCC scientific signing and parent stack gate are unresolved.
- Construct a genuinely input-uniform, scalar-free torsion lift for both P and Q; only then evaluate complete cold cost against matched rho and admitted index calculus.

### Errors (what went wrong, how it was handled)

- A proposed CT72 scoped-log filename returned 404; CT72's report and the current root session log supplied the prior record. No local arithmetic or test mismatch.
