# NIST factorization bulk archive awaits artifact-store access

- **Status:** `open`
- **Kind:** access/tool limit
- **Affected work:** [R02 evidence](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/README.md), [R05 relation checkpoint](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R05-20261022/README.md), PR #2379
- **Next action owner:** artifact-store operator after connection setup
- **Last updated:** 2026-10-26

## Problem

The completed NIST raw archives require object-store publication, while
the runtime has no configured AWS connection. The October 24 policy now
requires data archives over 64 KiB in `s3://setec-astronomy-artifacts/store`,
using `tools/artifacts/artifacts.py` and real `.artifacts.json` entries.
See [the artifact policy](../docs/artifacts.md). The earlier R02/R05
manifests preserve their historical intended backend; they are not evidence
of publication to either backend. New R06/R07 manifests name the current
required remote and explicitly leave the remote object unset. Completed
R08 adds a [local-only archive](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R08-20261025/archive-manifest.json).
Its compression integrity, streamed restoration of the saved relation
file and byte-identical canonical repack are checked. A fresh full-directory
extraction was not performed because local disk is constrained. Remote
publication remains unresolved.

## Evidence and uncertainty

- **Observed:** current cloud runtime observations list no outbound
  identities, secrets or runtime variables. No credentials or identity
  alias have been inferred or invented.
- **Preserved:** complete raw files, deterministic local archive, byte count
  and SHA-256 in the [R02 manifest](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R02-20261022/archive-manifest.json)
  and [R05 manifest](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R05-20261022/archive-manifest.json),
  plus the [R06 manifest](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R06-20261023/archive-manifest.json)
  and [R07 manifest](../research/nist_skipped_curve_dissection_20261021/factorization/rounds/R07-20261025/archive-manifest.json).
- **Available for review:** results, every individually named prime
  certificate, Lean product proofs, inspected report and compact runnable
  P-256 checkpoint are committed without the bulk archive.
- **Missing capability:** an observed-ready AWS connection authorized to
  use the configured store. No remote upload or `store:` index entry is
  claimed. This access gap does not invalidate the replayable arithmetic,
  and does not make full raw publication complete.

## What would unblock it

- [ ] Select an observed-ready AWS connection authorized for the current
  artifact remote and make the documented artifact transport available.
- [ ] Push the unchanged checksum-bound archives through `tools/artifacts`,
  verify remote readback and SHA-256, then commit the actual standard
  manifests and managed ignore rules. Keep named `store:` entries separate.

Those checks establish durable bulk publication. They do not complete the
remaining factorization or prove a historical seed-selection reason.

The [R06/R08 filtering submission](../backlog/XP-20261026-nist-p256-checkpoint-filter/README.md)
is blocked on publication before another runner can restore its inputs.
The protocol, backlog card and draft expspec are definitions in Git; no
queue task, lab lease or paid host is claimed. Runtime observations on
October 26 still list no configured outbound identities, secrets or runtime
variables. No credential value was inspected or copied into this evidence.

## Next discriminating step

Recheck runtime readiness after connection setup. If ready, use the current
artifact tool for the recorded unchanged archives; do not change the shared backend or
delete remote objects. If access fails, preserve the exact failure and
keep this record open. No credentials are requested in chat or copied into
evidence. Recheck trigger: an existing connection name or completed setup.
