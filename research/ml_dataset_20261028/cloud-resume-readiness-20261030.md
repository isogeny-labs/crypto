# Cloud continuation readiness

The retained corpus count remains **zero verified cloud dataset cells**. The
read-only IAM recheck in `worker-access-recheck-20261030T0517.json` confirms no
inline policies and only the legacy artifact managed policy on `isolab-job`.
That document embeds the current v1 policy. It does not assess every possible
bucket-policy grant or establish worker access. No permissions or jobs changed.

The prepared `worker-s3-policy-proposed.json` remains an additive proposal:
prefix-limited listing, get and put for the dataset campaign and dated lab
receipts, without deletion or changes to existing permissions. The previously
requested human approval remains absent. Controller source/evidence checkpoints
succeed under a different identity and do not demonstrate worker publication.

The recorded USD200 cumulative ceiling in
`validation-f0863ae/shared-budget-2751.json` is not a daily allowance. Its
"through tomorrow" horizon and existing E12 deadlines have passed. Historical
us-east-1 estimates cannot admit a new us-east-2 worker; preserve unresolved
obligations and reconcile actual resource termination and the authoritative
allocator before any renewal or launch. No new grant or renewed horizon is
inferred from a resume instruction.

The `startup-deployment-proposed/` files are templates. Historical startup
software validation has since passed at f4249f62 (see
`validation-startup-main-20261029/README.md`), but this is not Kubernetes startup
validation. A real deployment still needs a selected tested binary and image,
exact plan/configuration digests, reservation before provisioning, actual Job/Pod
and node identities, finite resource-lifetime evidence, and one conditional
immutable mailbox grant delivery. The mailbox Role/RoleBinding is a separate
permission proposal. Do not reuse readiness003 as current-source evidence.

The existing finite pilot contract retains36 cases, shard0/19,30seconds/case,
2048MiB address-space limit,64MiB trace/case,2CPU/4GiB Job,3600seconds and zero
retries. These historical limits alone do not authorize another submission.
After operational admission, require actual worker conditional PUT/HEAD/fresh
GET/hash/length, retained accounting/termination, and independently budgeted
`dataset budget fetch`, `dataset restore`, `dataset verify` and `dataset status`
on the full publication receipt before expansion. No such successful chain is
asserted here.
