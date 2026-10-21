# Recovery of omitted session plans

The twelve retained sessions had complete records, specs and host capsules but
their `plan.json` files were omitted from Git. This made the evidence-integrity
job fail before it could check those sessions.

Each plan was reconstructed from the existing records in the original
`PlanDoc` field order: spec-ordered arms, curve/target-ordered workloads, and
sequence-ordered executions with their original algorithm seeds. Every byte
sequence was accepted only when its SHA-256 exactly matched the already
committed `session.json` `plan_sha256`. `PLAN_RECOVERY.json` lists all twelve
matching hashes. Existing session, spec, host and record bytes are unchanged.

The native `ecbench verify --replay 0 --allow-interrupted --exit-code`
integrity audit passes for all twelve sessions (880 records). This checks
retained integrity; it is not a new solver replay or timing measurement.
