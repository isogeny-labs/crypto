# Round25 execution readiness reconciliation

## What was checked

On 2026-11-05, PR2980 remained open at
7c919085c147ad72b2a028528b8a8f93d7f862e6. The exact round25 Kubernetes Job
hashed-ecdl-cpu-round25-20261030 was NotFound; its S3 result listing had no
objects and no continuation token. No interrupted attempt needs restarting.
The frozen protocol, publication receipt and admission record remain unchanged.

The admission condition has recurred through candidate publication, protocol
freeze and this reconciliation: no authoritative shared unreserved-funds
record or operator live-cap readback has been supplied. Delayed AWS Budgets
totals do not resolve outstanding reservations or bound whole-worker idle
exposure. Earlier direct reads failed at role assumption; no broader identity
or privileged workaround was used. The USD2 proposed ceiling remains unreserved,
and the one allowed attempt remains unused.

## CI dependency diagnosis

The live main artifact-autofix workflow invokes
../trusted/.github/scripts/artifact_prefetch.py after checking out
github.event.pull_request.base.sha into trusted. PR2980's API base SHA is
4bbbde1a1ba37e2c992e2fdfcec5989c9610ac76. GitHub's contents API returns404
for that helper at this exact base. Main's workflow gained the helper step in
67ff2abbf3ad20a0a14986c72e5d52c55928bdad on November5. Run42054338707 failed
at prefetch before artifact upload/validation, consistent with that mismatch.

The checked-in feature-branch workflow predates the prefetch step. Changing a
script only in this candidate would not supply a missing file in the trusted
base checkout. Do not substitute PR-controlled code into the trusted path.
Any fix must preserve the trusted workflow/tool provenance and obtain current
head validation; no rerun or workflow security-boundary change was performed.

## Next required evidence

Obtain the existing shared reservation ledger and authorized operator readback
of live NodePool and namespace caps. Reconcile the USD200 daily/USD1000 monthly
ceilings and whole-worker exposure before admitting the USD2 reservation.
Then recheck the frozen input manifest, publication receipt, exact job/results
and capacity; stage and submit the existing single attempt. Preserve failures
and do not rename a retry. The source and protocol are ready; no measured
result or tenfold completion follows from readiness.

## Decisions and errors

The prior goal turn was progress: it published a frozen experiment and exact
publication receipt. This turn narrowed the CI failure to a trusted-base
dependency and reconciled absence of an external attempt. The measurement
remains blocked by external admission evidence; additional unmeasured source
variants would not establish the requested speedup. No cloud spending,
capacity change, evidence rewrite or duplicate job occurred.
