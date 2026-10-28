# Pending launch reservation design

Status: design only, 2026-10-28. Reviewed against source
`009b3342ecbfadcee1e29e88267901cec0823736` in
`/home/ec2-user/crypto-ml-dataset-20261028`. This document does not reserve funds,
create a ledger, launch a Job, grant cloud permissions or establish cloud proof.
This frozen design specifies a bounded extension to `src/ml_dataset/budget.rs`; implementation and
CLI integration follow this contract, with independent software review. Existing USD 200 campaign accounting,
prior obligations, pricing evidence, resource-lifetime requirements and retained
authority state remain unchanged.

## Contract

Reserve one finite ownerless envelope before submitting a billable Job. Retain
that reservation permanently. Once the launcher has reconciled the actual Job
and Pod, bind the reservation atomically to exactly that owner and issue the
existing `crypto.ml-dataset.budget-grant/v1` record. Binding transfers accounting
representation; it neither adds allowance nor releases any capacity.

`launch_id` is the eventual `grant_id`. No placeholder UID, dummy owner or
spendable worker grant exists before binding. A pending reservation cannot
activate a worker. The worker still requires the unchanged grant validation,
exact journal path, retained authority marker and charged remote activation.

An ambiguous submission, failed startup, expired reservation or terminated Job
does not refund the envelope. A replacement owner cannot use the reservation.
This extension supplies allocation mechanics, not Kubernetes attestation,
submission, grant delivery, node lifetime enforcement or an AWS billing cutoff.

## Public schemas and API

All new records derive `Clone, Debug, Serialize, Deserialize, PartialEq, Eq` and
deny unknown fields. All fields below are public. New schema strings are exact.
Existing policy, request, grant, activation and worker journal schemas are
unchanged.

```rust
pub const LAUNCH_REQUEST_SCHEMA: &str =
    "crypto.ml-dataset.budget-launch-request/v1";
pub const LAUNCH_RESERVATION_SCHEMA: &str =
    "crypto.ml-dataset.budget-launch-reservation/v1";
pub const LAUNCH_BINDING_SCHEMA: &str =
    "crypto.ml-dataset.budget-launch-binding/v1";

pub struct LaunchTarget {
    pub cluster_identity: String,
    pub namespace: String,
    pub job_name: String,
}

pub struct LaunchRequest {
    pub schema: String,
    pub launch_id: String, // eventual GrantRequest.grant_id
    pub target: LaunchTarget,
    pub worker_journal_path: String,
    pub binding: RunBinding,
    pub expires_unix_seconds: u64,
    pub resource_envelope_sha256: String,
    pub resources: Resources,    // includes finalization
    pub finalization: Resources,
    #[serde(default)]
    pub credential_role_arn: Option<String>,
}

pub struct LaunchReservation {
    pub schema: String,
    pub campaign_id: String,
    pub policy_sha256: String,
    pub request: LaunchRequest,
}

pub struct LaunchBinding {
    pub schema: String,
    pub launch_id: String,
    pub reservation_sha256: String,
    pub observed_target: LaunchTarget,
    pub owner: OwnerIdentity,
    pub observation_evidence_sha256: String,
}

pub struct LaunchAdmissionReport {
    pub schema: String, // crypto.ml-dataset.budget-launch-admission/v1
    pub admitted: bool,
    pub dry_run: bool,
    pub reasons: Vec<String>,
    pub policy_sha256: String,
    pub prior_and_allocated: Resources,
    pub requested: Resources, // original complete requested envelope
    pub additional_requested: Resources, // zero for an exact existing request
    pub remaining: Option<Resources>,
    pub existing_reservation_sha256: Option<String>,
    pub already_bound: bool,
    pub account_wide_billing_cutoff: bool, // always false
}

impl LaunchReservation {
    pub fn sha256(&self) -> Result<String, String>;
}

impl CampaignLedger {
    pub fn dry_admit_launch(
        path: &Path,
        policy: &CampaignPolicy,
        request: &LaunchRequest,
    ) -> Result<LaunchAdmissionReport, String>;

    pub fn reserve_launch(
        &self,
        request: &LaunchRequest,
    ) -> Result<LaunchReservation, String>;

    pub fn bind_launch(
        &self,
        binding: &LaunchBinding,
    ) -> Result<WorkerGrant, String>;
}
```

The reservation SHA is the existing budget helper's SHA-256 of compact native
serialization of the typed reservation. It is not the SHA-256 of an arbitrary
pretty-printed input file. CLI output must expose `reservation_sha256` explicitly.
No timestamp or random value is needed in the reservation: committed typed
content and its digest are stable across exact retries.

`LaunchTarget` accepts a deliberately narrow canonical subset: an exact EKS
cluster ARN with partition `aws`, service `eks`, the policy's admitted region,
a 12-digit account ID and a 1–100 character cluster name using ASCII letters,
digits, underscore and hyphen, starting with a letter or digit. Namespace and
Job name must each be a lowercase DNS label of 1–63 characters: letters,
digits or hyphen, with alphanumeric endpoints. Do not normalize aliases, case,
context names or spelling. Unsupported target forms fail closed. Exact equality
of the three strings identifies a target. The request accepts no `generateName`.
These are input restrictions, not a claim that the cluster exists or that a
particular Kubernetes object has been observed.

The launch ID and actual UIDs use the existing bounded identifier validation.
The observation evidence SHA must be a lowercase 64-hex digest. It names the
retained, immutable launcher observation bundle, including the Job response,
Pod controller reference and UID, placement and the externally checked resource
identity/lifetime. The allocator checks the digest's format and exact retry
identity, not that external bundle's truth. No tokens or credentials belong in
these records.

## Minimal persistent representation

Keep SQLite tables, existing event hashes and existing grants as they are. Add
only one defaulted field to the private campaign state:

```rust
struct CampaignState {
    policy: CampaignPolicy,
    grants: BTreeMap<String, WorkerGrant>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    launches: BTreeMap<String, LaunchRecord>,
}

struct LaunchRecord {
    reservation: LaunchReservation,
    binding: Option<LaunchBinding>, // None=pending; Some=permanently bound
}
```

`LaunchRecord` is private, denies unknown fields, and derives the usual serde
traits. The map key equals `reservation.request.launch_id`. A bound record
references the grant under that same key; reconstructing the expected v1 grant
from its request and binding must exactly match the stored grant. Thus one
optional binding captures the two states without separate mutable counters or
two nullable fields that could disagree. The bound owner and observation digest
are retained permanently. No cancellation, release, expiry deletion or rebind
state is introduced.

Current `read_state` validates the SHA of the original stored JSON text before
deserializing. Old state lacks `launches`, so it reads as an empty map without
rewriting stored bytes or historic event hashes. Omitting the empty map also
preserves direct-only serialization. Add `launch_reserved` and `launch_bound`
event kinds to the existing arbitrary-JSON event body; do not rewrite old
events. No new table, migration command or worker journal field is required.

An older binary with `deny_unknown_fields` must reject a nonempty `launches`
state. That is intentional downgrade protection: silently ignoring pending
allocations would renew quota. Direct-only ledgers continue to round-trip.

## Accounting and integrity invariants

For every read used for admission, open, status or mutation, validate the launch
relationships before trusting the accounting result:

1. Every map key equals its launch ID. Reservation schema, campaign and policy
   digest match the retained policy. Structural request validation succeeds;
   retained expired records remain valid historical obligations.
2. Frozen targets are unique across all launch records, including bound and
   expired records. No record is removed to free its ID or target.
3. A pending launch has no grant under its key. A bound launch has one exact
   matching v1 grant and one matching binding: reservation digest, target,
   owner, schema and evidence digest. A dangling or inconsistent pair rejects
   the entire operation; it is never interpreted as an uncharged launch.
4. Total allocation is computed with checked addition:

   `policy.prior + sum(all grants.resources) + sum(pending launches.resources)`.

   A bound launch is counted through its grant only. All six resource
   coordinates must fit the unchanged policy caps. Binding requires the total
   vector before and after the transition to be identical.
5. The existing maximum of 4,096 allocation slots applies to
   `grants.len() + pending_launch_count`, with checked size arithmetic. Bound
   launch metadata does not consume a second slot. The existing 32 MiB state
   encoding limit remains in force.
6. Direct `reserve` rejects every ID occupied by a launch, including pending,
   expired and bound records. It also rejects an owner whose Job UID or Pod UID
   is already bound to a retained launch. First launch binding rejects a Job UID
   or Pod UID already used by any other grant, preserving one owner for this
   launch route. Existing direct-only ownership semantics remain unchanged.

Legacy direct grants have no target field, so their target cannot be inferred
or retroactively attested. Target uniqueness is enforced for every known launch
target; identity collisions are checked once the actual owner is available.
The new adapter must use `reserve-launch`, not use direct issuance as a way to
bypass pending allocation. This boundary is explicit rather than inventing
target metadata for historic grants.

Refactor current `GrantRequest::validate` into a shared ownerless envelope check
plus owner validation. The common check covers source/binary/plan/protocol/shard,
exact journal path, resource evidence, checked resources/finalization, compute
floor, optional role and protected activation capacity. Live issuance additionally
checks current expiry. Historical consistency checks use the same structural
rules without rejecting merely because time has passed. Neither path constructs
a fake `GrantRequest` owner to reuse validation.

## Transaction and retry semantics

All mutations retain the existing `Mutex` plus SQLite `BEGIN IMMEDIATE`,
`synchronous=EXTRA`, DELETE journal and checked state/event writes. Independent
allocator handles serialize against the same authoritative database. Successful
commit precedes export of the reservation or grant file. A failed output write
cannot undo a committed allocation.

| Operation and state | Result | Accounting |
| --- | --- | --- |
| Reserve a new, valid launch ID and target | Commit reservation and one `launch_reserved` event | Charge full envelope once |
| Exact reserve retry, pending or bound, including after expiry | Return byte-equivalent typed reservation; no event | Unchanged; evidence recovery only |
| Reserve retry with any changed request field | Reject | Unchanged |
| Different launch ID for retained target | Reject, including after expiry | Unchanged |
| First bind before both deadlines, correct digest/target/new actual owner | Atomically store binding and matching v1 grant, then one `launch_bound` event | Identical total vector |
| Exact bind retry, including after expiry | Return identical stored grant; no event | Unchanged; no new activation balance |
| Bind retry with changed owner, target, reservation SHA or observation evidence SHA | Reject | Unchanged |
| First bind after launch or campaign expiry | Reject | Pending allocation retained |
| Missing reservation, direct-grant ID collision or inconsistent stored linkage | Reject | No fabricated allocation or owner |
| Submission failed, timed out or has unknown outcome | Reconcile original target externally | Allocation retained indefinitely |

For exact retries, compare the complete frozen request or binding before taking
the evidence-recovery path. A changed request cannot acquire the old receipt by
reusing its ID. Exact retry returns are not fresh permission to submit, activate
after expiry, reset an authority marker or recreate a journal. The unchanged
worker validator still rejects expired grants.

Reserve and bind perform their state lookup and all conflict checks inside one
transaction. Two concurrent new reservations cannot each spend the same
headroom. Two first binds with distinct owners or evidence cannot both win; the
loser sees a conflicting permanent binding. Reserve-direct racing reserve-launch
for the same ID has one winner. A rollback or pre-commit crash leaves no partial
binding or orphan grant; a post-commit crash is recovered by exact retry against
the retained authoritative database. Existing authority/path/boot checks and
loss-of-database tombstones apply unchanged.

The launch record deliberately has no speculative `submitted` flag. A database
transaction cannot atomically commit a Kubernetes API operation. The launcher
retains that submission receipt separately and reconciles ambiguous results by
the same cluster/namespace/Job name. Neither an empty response nor an expired
record permits a second Job or a refund.

## Read-only admission and status

`dry_admit_launch` follows current `dry_admit`: read-only/query-only database
access, authoritative path and marker checks, no directories or sidecars
created, no events, grants or network calls. Missing state is hypothetical only
if both ledger and its authority marker are absent. A missing database with an
existing marker rejects. Validate policy and all launch invariants before
reporting capacity.

For a new request, `additional_requested` equals `requested` and `remaining`
subtracts it from the current total. For an exact existing request,
`additional_requested` is all zero and `remaining` is current remaining capacity;
report its reservation SHA. An exact pending request can report admitted while
live. Already bound or expired requests report `admitted=false` with a reason;
their original receipts remain recoverable through exact reserve/bind retries.
A conflicting request reports rejection. A dry report does not reserve capacity
and does not itself authorize Kubernetes submission. This distinct report avoids
changing the existing direct-grant admission schema or double-counting an exact
retry's requested envelope.

Status retains all existing fields and adds `launches`, `pending_launch_count`,
`bound_launch_count` and `pending_launch_resources`; `prior_and_allocated` and
`remaining` use the complete formula above. Derived expiry is observational
only and must not alter allocation totals. Counts of allocations are not counts
of submitted or successful Jobs. Retain `automatic_refunds=false` and all current
authority-loss and billing-cutoff limitations.

## Proposed CLI integration

Solver agent owns CLI implementation after API agreement. Keep existing `admit`
and `reserve` interfaces unchanged. Add:

```sh
# Pure read-only hypothetical admission; --out and --create-ledger conflict here.
crypto dataset budget reserve-launch --dry-run \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --request launch-request.json

# Explicit commit before any Job creation. --out is required outside dry-run.
# --create-ledger is only for first authoritative initialization, never recovery.
crypto dataset budget reserve-launch \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --request launch-request.json --out launch-reservation.json

# After external reconciliation of the exact target and actual Job/Pod identity.
# This opens existing state only and cannot create a missing allocator.
crypto dataset budget bind-launch \
  --policy policy.json --ledger /durable/campaign.sqlite \
  --binding launch-binding.json --out grant.json
```

Immutable local writes reuse the existing writer and restricted artifact roles.
Reserve output contains the reservation, its native `reservation_sha256`, local
artifact descriptor, campaign status, `cloud_requests: 0`, `launched: false` and
`worker_grant_issued: false`. Bind output contains the v1 grant, grant SHA, local
artifact descriptor, exact accepted binding, status, `cloud_requests: 0` and
`launched: false`. No command implicitly submits a Job or activates a session.
Bound status in exact retry output must remain visible to the launcher.

Do not use `--create-ledger` on a retry. After an uncertain reserve command,
open the existing authoritative ledger and retry exactly. Output errors retain
the committed reservation. Copying the allocator to another path/host, deleting
its marker or reconstructing it from a receipt is not supported recovery.

## Required software tests before integration acceptance

- Read an actual pre-extension state/event fixture with no launches field;
  opening, dry admission and status do not rewrite state or event hashes.
  Direct-only serialization still omits the empty map; v1 worker grant bytes
  remain unchanged for equivalent inputs.
- Reserve pending capacity and show direct and launch admission both include
  it. Bind and show every resource coordinate and remaining allowance are
  identical before/after; exactly one grant is installed.
- Reject every changed request field, ID collision, duplicate target and target
  alias/invalid form. Duplicate targets stay rejected when bound or expired.
- Verify exact reserve and bind retries create no additional events or balance,
  including expired evidence recovery. Reject first expired bind and altered
  owner, evidence, target or reservation digest without state change.
- Race separate SQLite connections on scarce capacity, identical IDs, duplicate
  targets and competing owner bindings. Assert bounded totals and one winner.
- Inject transaction rollback before commit and reopen after a successful
  commit with lost output; neither yields partial linkage, refunds or a second
  allocation. Retained lost-DB authority marker still rejects reinitialization.
- Reject an internally inconsistent pending/grant or bound/missing-grant state
  even when test code recomputes state and event digests. Check all six arithmetic
  overflow/underflow coordinates, slot limits and metadata-size limits.
- Demonstrate dry admission creates no files/events/network calls and reports
  zero additional resources for an exact retained request. CLI tests inspect
  `admitted`, immutable file retry behavior and canonical reservation SHA.

No software test is evidence of cloud owner reconciliation, grant delivery,
worker publication, resource termination, account-wide spend control, CRIU
recovery or a completed scientific dataset cell. Those acceptance steps remain
with the launch adapter and the frozen dataset campaign.

## Primary references and scope

The implementation uses SQLite's existing [IMMEDIATE transaction semantics](https://www.sqlite.org/lang_transaction.html)
for serialized state transitions; it does not claim an atomic transaction across
SQLite and Kubernetes. Grant delivery is a separate proposal based on
[Kubernetes projected volumes](https://kubernetes.io/docs/concepts/storage/projected-volumes/)
and [downward API fields](https://kubernetes.io/docs/concepts/workloads/pods/downward-api/).
These references informed the design on 2026-10-28; none establishes deployed
permissions or successful cloud execution. Existing allocator `open` permits
historical inspection after deadlines, while live admission and worker validation
reject expiry. Exact reservation/grant recovery preserves that distinction.
