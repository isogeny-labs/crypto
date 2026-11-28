# Round25: numerator scratch comparison

## What was done
Prepared all three original fixtures, seeds111/222/333,16-trial/1,000,000-work caps, six arms and 180 executions (162 measured,18 warmups). u/x are original baseline/A-A; v/w are published candidate c47886258a0e2b3d00a0734f059aaaa3373ca322 at batch64/batch1; y/z are the retained suffix-only executable at 4939261c9fcc48bf86f17ac36fa458f52a155b2e. Restoration commit6ca7537e243b has identical lib.rs; using the custody-verified executable avoids a new parent build. Candidate archived from Git and built offline/locked standalone; root/CUDA build is excluded.

## Decisions made
Runner, native analyzer, fixtures, ordering and full-certificate checks are unchanged from round24. Parent/candidate differ only in numerator scheduling and scratch storage; complete timing charges reverse-pass y loads and branches. Both full replays, negative scalar control, all preparation/discarded trials and serialization remain mandatory. L0 timings guide selection and cannot establish tenfold acceptance. All missing cells and failures are retained; no extra repeats after outcomes.

Source/reference basis: residual-scratch/README.md, batch-endpoints/README.md, residual-filter-revert/README.md, prior exact analyzer and protocols. The source delta and materializations bind the candidate. No mathematical novelty claim.

## Budget and admission
One attempt, zero retries,2vCPUs/4GiB,300seconds,concurrency1. Proposed reservation USD2 all-in, including worker provisioning/idle time, failed attempts, storage, transfer and any build charges. No cloud build is planned. The ceiling is not an estimate and is not yet an admitted reservation. Shared daily USD200 and monthly USD1000 ceilings apply; no capacity increase or provider/pool fallback is authorized.

Before submission, verify current unreserved funds, whole-worker maximum exposure and live NodePool/namespace limits; account for other lanes' reservations. Stop if unknown. Publish exact frozen protocol, input manifest and launch argv to GitHub and preserve publication receipt before any S3 staging or job. Bind an absolute expiry at admission; preserve this job's one attempt across failures. No job, upload or study timing rows exist for round25.

## Open threads
Complete freeze and native synthetic-panel tests; publish the exact package. Admission remains separate from protocol/software validity. No request to waive cost controls is needed: preparation can proceed while funds and live caps are unresolved. The current PR artifact-migration workflow also lacks its trusted prefetch script; that unrelated failure is recorded in PR2980.

## Errors
The first formatting check requested wrapping the extended exporter allowlist; formatting was corrected. No study execution was attempted.

## Frozen validation and handoff
Protocol df2d4e9d7415845dfb13d3ca6fe41489b1d43e0e8ec0a18578900a77206fd058
was frozen before data at 2026-11-05T15:19:01Z against draft commit
0fd49ee8b45933d5be5a8dd8f169da22e6fb7dfa. Native validation passes. All five
executable identities and all twelve staged-input manifest entries verify.
The native suite passes before and after freeze:14 passed,1 ignored; two
profile-only tests return without work, leaving12 applicable synthetic checks.
Strict native Clippy, formatting, shell syntax and exporter build pass.
The candidate's47 library tests are software validation, not timing samples.

The remaining admission gates are captured in admission.json. Direct live
NodePool and namespace reads failed at role assumption; the exact errors are
preserved. No access escalation, capacity change, S3 staging or cloud job.
The current user was asked for the existing shared reservation ledger/operator
receipt location while preparation continued. Freeze and software readiness
are complete; publication readback remains to be attached after push.

Exact publication readback passed for all14 listed protocol/source-manifest,
materialization, command, runner, analyzer and admission files at
ed24073bf650c2db6f761051d5bd2668ef5d47b6. The operator runbook's
check-experiment-git verified local/index/committed byte equality and the exact
GitHub commit; publication-receipt.json retains hashes and byte counts.
The current rustc -Vv matches rustc.txt. This completes publication readiness,
not spending or cluster admission. No input file changed after this readback.
