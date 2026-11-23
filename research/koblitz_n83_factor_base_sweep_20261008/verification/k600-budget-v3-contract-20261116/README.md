# K600 cumulative budget controller v3 contract

Status: **implementation gate, no arm search or factor-base ranking**. This
change adds a versioned admission path to `examples/n83_k600_m6_budget.rs`.
The original plan v1 and precharge v2 remain available for their frozen
receipts. A plan v2 must be frozen separately before any constructed or
natural arm is launched. This document does not assign a new runtime
allowance or treat earlier readiness attempts as a measured candidate.

## Why a new contract is required

The source-locked Linux build is revision
`ba620214684e41def8bc6caddf2e782f43920218`; the clean arm checkout is
`a30e67492f821bc6f58ff97c508be9cfe144d7ab`. The already retained Linux
build receipt reports supervisor SHA-256
`e3fcc0da60417838bdbd8dcf33b28d3c35e4be495533890404e32e5f54d8d906`.
The committed preflight lock identifies a separate macOS preflight supervisor
`3ea86decaf897e800aa3f78c5b028c1505b71fe1637cbb858ad717907f0284c0`.
The host-native H2 arm supervisor is a third binary whose hash must be taken
from its completed build. Plan v1 wrongly requires one controller source and
equates the Linux build's supervisor hash with the H2 arm supervisor hash.
The [arm admission protocol](../k600-arm-admission-20261116/PROTOCOL.md)
records the source and setup boundary.

## Plan v2 and cumulative ledger v3

Plan v2 names `controller_source_commit`, `arm_source_commit`,
`build_source_commit`, `allowance_ms`, the arm binary, arm lock, committed
preflight lock, preflight run, and the SHA-256-bound `precharge_receipt`.
The controller checks its *own* clean checkout against the first field and
checks the arm binary and lock against H2. Later arm-stage receipts must name
H2. The allowance must come from a separately frozen authorization and
experiment protocol; the code does not infer one from previous approvals.
`assemble-v3 REQUEST_JSON ARM_LOCK PREFLIGHT_RUN NEW_LEDGER_JSON` is the native
write-once ledger producer. The request uses schema
`n83.k600-m6-cumulative-request/v3`, the three source commit fields,
`allowance_ms`, five artifact `path` entries, and an `intervals` array with
`phase`, `path`, `meter`, and `charged_wall_ms`. It is retained outside the
clean controller source checkout, and its exact bytes are hashed into the
assembled ledger. This avoids putting a source commit ID into that same
commit's tracked bytes. The plan binds the resulting ledger SHA-256.

The `n83.k600-m6-cumulative-precharge/v3` ledger has five required artifacts
(`build`, `host-build`, `stage`, `freeze`, `preflight`) and five required
charged intervals with the same names. Every path is absolute, unique, and
SHA-256-bound. Each interval has `path`, `sha256`, `phase`, `meter`, and a
positive `charged_wall_ms`. Optional `retry` intervals are charged too.
The sum must match `total_charged_wall_ms` and remain below the plan allowance.
An observed wall greater than its charge fails admission. Build and preflight
can use their retained native receipts; host build, stage, and freeze require
native `charge` outer receipts binding child receipt hashes, controller source,
controller binary, successful child exit, no wall cap, and cleanup. The host
build child receipt must identify the exact H2 supervisor digest. A new cold
measurement must include all setup and failed attempts covered by its frozen
protocol, rather than treating a previous readiness artifact as free setup.
The controller checks every listed interval but cannot discover an omitted
attempt. The frozen protocol and independent audit must enumerate the complete
attempt history; an incomplete ledger cannot support a total-cost claim.

The ledger accepts different Linux and macOS supervisor hashes while binding
the common Linux image, exporter and solver hashes and all arm/preflight
source identities. The exact input, staging, and positive-control gates of
the native arm supervisor still apply. This change does not establish rank,
a checked logarithm, total cold runtime, or a winning base.

## Remaining execution gates

1. Complete and audit the H2 host build and read-only S3 stage under their
   separate admission packet; preserve every stopped attempt and its charge.
2. Produce a write-once host-build child receipt, then run the five setup
   phases under native outer interval capture. Freeze a plan v2 with a
   concrete allowance, exact interval inventory, and setup accounting before
   the first measured arm.
3. Run the controller's positive pinned, positive unpinned, and natural
   stages in order. Check every model and scalar and report incomplete or
   censored stages separately. Repeat comparable candidates on the same
   public instances and compare fully charged cold cost with matched rho.

The K600 readiness work and this controller are not a substitute for the
requested full combination sweep. The historical finite grid still includes
many unavailable backend adapters; those cases remain explicit open work.

The [original controller validation](VALIDATION.md) and the
[merged-tree validation](MERGE_VALIDATION.md) record passing example tests and
the failed macOS full-library gate. The merged-tree report is the current
gate status for this PR; neither record is a factor-base performance result.
