# Generation 6 terminal upper-gate partial-data audit

Recorded: `2026-10-30T03:29:34Z`  
Reviewer: `/root/ci_audit`  
Scope: read-only terminal-state, checkpoint-integrity, record-inventory, and canonical-history compatibility review. No mathematical replay, solver execution, cloud action, consumer change, or scientific qualification was performed.

## Terminal result

The outer recovery controller is terminal `failed` with `original UTC deadline expired`. It remained at zero-based `attempt: 0`, `restores: 0`, and never attempted to create the fresh benchmark job. Generation 6 is the latest committed checkpoint.

The embedded ecbench session is still a checkpointed `running` snapshot with two records, `finished_unix_ms: null`, and `records_sha256: null`. The outer terminal state must not be rewritten as a terminal ecbench session or used to invent a final records digest.

## Generation 6 integrity

- Terminal state: `/home/ec2-user/pencil-upper-checkpoint-harvest-20261030/terminal-state-0328.json`
  - SHA-256 `d3985784784f82244779f0db72bb7e996a7947f571e11fa23701bd856d9580be`
- Checkpoint: `/home/ec2-user/pencil-upper-checkpoint-harvest-20261030/g6/checkpoint.tar`
  - SHA-256 `e7bd3bd69a60c175759950aff34138007c51687872ed0d034bc16968a2027f13`
  - `528,281,600` bytes
  - S3 version `qHEeQ_J4ax0M218b5snGnvuj7tEDxOgL`
- Generation manifest: `/home/ec2-user/pencil-upper-checkpoint-harvest-20261030/g6/generations/g6/manifest.json`
  - SHA-256 `7d8b415e9f38f04e631e61c6194ad2b6713c609ae4cbf8155ea9c182a0362e3d`
- Download receipt SHA-256: `18ea0d1444668d211bf4f382bd4aa87f3d5dfa32795d41306bc71d92badf0818`
- Request SHA-256: `daf54f696c9283075900b099d244dd8666e581c7726c0e502c536219d0a9a30a`
- Instrument SHA-256: `a1c8ca89748a22b8a07ea00fd3983061be0d8197d5cae49fca883dcf5dd50eab`

All seven extracted files match the generation-6 manifest in digest and byte size. All seven are byte-identical to their generation-4 counterparts. No new record appeared between generations 4 and 6.

## Final record inventory

| Seq | Record ID | Arm | Role | Warmup | Exact original-plan slot |
|---:|---|---|---|---:|---:|
| 0 | `ECR1h174e1c3b3a9d` | `rho` | reference | true | yes |
| 1 | `ECR1hba19f769acc3` | `rho_AA` | control | true | yes |

Both records are verified, match the target and planted scalar, and use session `ECBS1h9438bcc3aba8`. The fixed denominator remains:

- `2/18` observed executions
- `2/3` warmups
- `0/15` measured executions
- `0` pencil records
- `0/5` measured pencil/rho pairs
- `16` missing slots
- All 36 algorithm and inclusive cost ratios `null`

The original prepared plan SHA-256 remains `481d795e6208cfb3ca648c463de2d7b232adfb7d28bebc341afa4f58e119dfcf`. The computation plan SHA-256 is `f74ebbc6419891f6f80dd99dff14e988b5e9cc4c5c06d663d8be2277335e29eb`; it differs only by `spec_id` and cannot replace the prepared plan.

The session records artifact-child commit `7d7f5495c5b85ae426efdbcb721b6faf72c1be14`; the controller request and frozen protocol bind instrument source `543a54b4b1c3be7052872af3ffe393a44c69a3c3`. The exact instrument binary is unchanged. A diagnostic must preserve both identities.

## Exact partial history boundary

A future `pencil-execution-history/v1` sidecar may truthfully record both observed rows with:

- `attempts: 1`, derived from outer zero-based `attempt: 0`
- `resumed: 0`
- `checkpoint: null`
- Exact session and sequence identities
- Bound evidence consisting at least of the terminal-state file and generation-6 manifest

The rows occurred before any restore. Later checkpoint generations do not turn these rows into restored records. Do not infer a terminal ecbench session, final records digest, measurement epoch, replay, restore, measured pair, or price ratio.

Canonical packaging remains a nonqualified diagnostic. It must preserve six incomplete-row findings: warmup shortfall, measured-row shortfall, measured-instance shortfalls for all three arms, and no observed pencil run. The canonical CLI must return nonzero.
