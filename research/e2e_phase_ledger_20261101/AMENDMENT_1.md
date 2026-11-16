# Amendment 1: failed first attempt and diagnostic remediation

Filed 2026-11-01 after failed workflow runs `41343651715` and `41343654905`,
and before any replacement measurement. The original protocol, specs, seeds,
source revision `32303a6d51d7a43ce887cd9da8f6d82afea4793c`, and binary SHA-256
`f3aab23417ece64cb3373e5c66d2c3be343e18c0e89347f626e0aea18ab1b774`
remain unchanged.

## Observed failure evidence

The fixed diagnostic run `41356000954` classified the original shards:

- target 0: CodeBuild status `FAILED`; its `BUILD` phase was
  `TIMED_OUT`/`BUILD_TIMED_OUT`. No launcher request, status, objects, or result
  receipt was readable, so the remote pod outcome is unknown.
- target 1: launcher state `failed`, remote launcher exit 23, retained Job UID
  `d5c4f94f-7171-46a0-a14f-e5cb7e213dca`, and retained Pod UID
  `9a1f74a1-2ae0-4dd3-ab1e-51fa771eca69`. Kubernetes reported pod exit 41,
  the native audit gate. The diagnostic role could not read the selected pod
  receipts, so the audit's exact failed predicate is not yet classified.

Neither shard is admissible. There are no reported backend rankings, phase
timings, F4/F5/F6/root-finding comparisons, or pooled partial results from this
attempt. The derived exporter remains undispatched.

## Prospective remediation

Before any replacement measurement:

1. increase the VPC launcher CodeBuild terminal limit from 70 to 100 minutes;
2. make the portable client wait the requested EKS duration plus one hour for
   provisioning, verification, and terminal receipt recovery;
3. grant the main-branch GitHub diagnostic role read access only to hash
   manifests plus identity, audit, L2, control, and owner-check receipts;
4. keep raw records, stdout, stderr, and all other pod objects outside that
   role's read boundary;
5. rerun the fixed diagnosis and identify the exact target-1 audit predicate.

The launcher update requires explicit deployment from an authorized Iso Lab
deployment host. Merging code alone does not change the live launcher or IAM
role.

## Replacement-run rule

This amendment does **not** authorize a replacement experiment yet. A second
pre-data amendment must record the target-1 root cause and any payload or
protocol correction before launch. If a replacement is then admitted, both
shards receive fresh workflow, Job, and Pod identities; no record from the
failed attempt is reused. The factor bases, targets, target seeds, measurement
seeds, arm order, warmup/round counts, child 90-second cap, full-rank solver,
L2 gate, replay-all audit, and A/A control remain frozen.

The replacement attempt has a cumulative incremental AWS ceiling of USD 10,
including failed launchers and workers. No launch occurs if actual accumulated
cost or remaining allowance is unknown. The original protocol's single
infrastructure-only retry limit remains in force.
