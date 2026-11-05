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
