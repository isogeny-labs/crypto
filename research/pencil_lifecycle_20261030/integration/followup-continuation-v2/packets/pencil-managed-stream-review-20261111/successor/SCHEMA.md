# v2 request and output contract

## Request

Schema: `ecbench.pencil-attempt-export-request/v2`.

The request contains `cohort` and `attempts` only. `cohort` supplies pinned
prepared validation, source manifest, binary, protocol, independent coverage
review, seven plan files, and session-custody groups for all seven primary
sessions plus every retained retry session. Each session group supplies exact
final `ecbench.session/v1`, owner-process, and controller-custody bytes. Each
attempt supplies only its route (`session_id`, `sequence`,
`attempt`, optional `retry_of`) and evidence pins:

- native per-execution diagnostic-child context file;
- optional ordinary record;
- zero or more progress snapshots and optional totals;
- optional native `ecbench.diagnostic_attachment/v2`, whose embedded prefix
  manifest is checked for the pencil arm;
- optional relation stream plus its required totals;
- optional resource prefix plus its required publication tail.
- optional immutable per-execution session snapshot, required with resources.

There is no caller `plan_sha256`, workload, method, seed, round, warmup, arm,
role, terminal status, eligibility flag, ineligibility list, cost ledger, or
pair ID. Those are derived from checked bytes and the frozen plans.

A retry may have a distinct native session, but it must name the exact
attempt-zero parent sequence. Its checked native session must bind the same
frozen plan digest as that parent. The existing attempt number is session-wide,
so a supplied retry session must include all 3,240 exact plan sequences under
one nonzero attempt number. It never changes the primary expected set.

## Output

Schema: `ecbench.pencil-attempt-cohort/v2`.

Each attempt row includes all exact scheduled identity fields, original pins,
derived evidence/custody status, explicit retry parent, and raw cost projections
from its checked ordinary record. Each measured triplet includes its exact
plan/session/workload/round/seed identity, three sequences and roles, record
IDs, candidate/reference eligibility, mapper blockers, and either all 36
nine-category cells or `null`.

The exporter never converts missing data to zero. A relation prefix without a
final Terminal is incomplete. Missing record, diagnostic attachment, resource
tail, or required pencil stream is retained with an explicit reason. A retry
never fills a primary slot. Owner/controller closure is reported separately:
its absence prevents no external-authentication claim, but does not erase an
otherwise valid counted-cost observation from the registered mapper.

Progress snapshot and embedded-prefix completeness are separate diagnostic
fields. A structurally checked nonfatal progress-write failure remains visible
but is not itself a finite-cost exclusion; cost admission is still decided by
the checked ordinary records, relation/resource/custody joins, and registered
nine-category mapper.

The exporter reports structurally closed owner/controller bytes separately and
keeps `owner_authentication_verified:false`: exact byte/schema association is
not an external controller signature or authentication mechanism. It also
keeps `scientific_execution_admitted:false`; finite algorithm-cost output and
external execution admission remain distinct fields.

The prepared validation receipt's plan-generation binary is not conflated with
the separately pinned scientific execution binary.

## Exact plan inventory

| file | SHA-256 |
|---|---|
| `plan-16.json` | `35e6311a7105a29c2459543ad0b281ed91e43aa63a99f66226f95a1f3c9cc2ba` |
| `plan-20.json` | `51a99e83c38b4d5d6b19f82337918b2f64f34b4934a5361298e84fc2f8bd432f` |
| `plan-24.json` | `7d34d0ce87c5a167ee1e94fecb08bcac34c9b4339570deb6377effe75efc28c4` |
| `plan-28.json` | `87760bc68b7598250c22976f15e4dbb36b5f2d738647de501222de1a8b27b001` |
| `plan-32.json` | `a9aebbc3120b61380a6936408635ac0f0e60715b4b4918dc826fbab4b578ae96` |
| `plan-36.json` | `6bb162d846bfbfe68b1bf7ba330abbc1e11c403b003215c960e7e71d81be3aa2` |
| `plan-40.json` | `8f2ec73a139f84b660573bd67c54ebfc567626ce7068a135dcb52c61891b22d5` |
