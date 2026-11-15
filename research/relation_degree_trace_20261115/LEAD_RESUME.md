# Resume: certified-relation leads

## Goal
Chase useful leads after the local native relation-solver component result.
Seek matched full-cost changes; do not infer ECDLP, fastest-solver or
regularity results from a weak native denominator or sparsity alone.

## Lead / hypothesis
Two leads are implemented: the division-free original-quadratic resultant,
and a certified quotient-algebra solver that constructs coordinate maps and
the complete grevlex basis without a reference oracle. Source and assumptions
are in research/relation_degree_trace_20261115/LEADS.md and QUOTIENT_PROTOCOL.md.
Both have producer correctness checks, but no successor timing. Expanded
certificates may offset a quotient-algebra shortcut `[pcc:unverified]`.

## Spec
research/relation_degree_trace_20261115/QPROJECTION_PROTOCOL.md freezes
the current six four-variable cases as exploratory and seeds 5003/7001/9001
on both primes as fresh conditional confirmation. Existing projection-criteria
is the native baseline; quadratic-projection-criteria is the new candidate.
21 paired samples, 7 A/A pairs, fixed bootstrap; no failed-case replacements.
External preprocessing and verification must be paid before a practical claim.

QUOTIENT_SPEED_PROTOCOL.md separately freezes the old best native route versus
quotient on the 18 exploratory 3/4/6-variable cases, in three bounded panels.
Conditional fresh seeds are 11003/13001/17011, independent of the quadratic
lead's holdouts; do not generate them before the stated admission condition.

## State
PR #4228 at a857edd9 contains the completed earlier component study: five
fresh cases with producer median native ratios 23.4–23.9 against a textbook
baseline; one failed A/A control excluded. All supported CAS bases matched,
but mature solver-only diagnostics were much smaller. Full raw archives are
in content-addressed S3, with manifests under the study's artifacts directory.
No signed independent performance verdict exists `[pcc:unverified]`.
Quadratic code and the configurable-baseline harness are committed in 4e187669.
The quotient prototype's final checks pass 35 normal and 36 counter-feature
tests plus strict clippy. Only fixed development fixtures were used; no new
timing or six-variable validation is asserted `[pcc:unverified]`.
The real sparse Git checkout is /workspace/crypto-leads;
older staged source and raw local outputs remain in /workspace/crypto-review.

## Next steps
Inspect quotient-development/{tests,count-tests,clippy,count-clippy}.log and
source hashes before repeating checks. Inspect the PR head and Git status to
confirm whether the quotient source/protocols have been published; publish
only missing work, without editing conflicted evidence.
From the repository root run scripts/checkpoint.sh gate before any experiment.
It must upload to S3 and pass; no token or AWS upload transport is configured.
After credentials are restored, use the applicable frozen speed protocol and
tools/isolated_bench.py reserve to run same-binary native pairs. Quotient-arm
construction is included in its solve/check interval. Do not use the old weak
denominator or combine clock scopes. Stop after the bounded panels/conditional
confirmation and report actual losses or failure to meet tenfold.

## Runs in flight
No research measurement is running. Local compilation/tests may be active;
inspect the tool session and quadratic-development/*.log before restarting.

## Stop and success conditions
No experiments until the S3 checkpoint and local isolation gates pass.
Keep CPU/child, memory and campaign caps from QPROJECTION_PROTOCOL.md. Reject
any changed ideal/basis or missing certificate. Retain every failure/control.
Incremental admission requires a lower paired interval endpoint above 1.05;
tenfold still requires above 10. Do not replace fresh seeds after failures.
The concurrent root-session-log and divergent PCC ledger remain unresolved;
never pick or rewrite either ledger history to merge the PR.
