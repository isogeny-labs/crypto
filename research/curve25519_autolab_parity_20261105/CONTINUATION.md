# Curve25519 Autolab parity continuation

User objective: run Autolab until achieving parity with rho for Curve25519.
Status: active; no new scientific trial in this continuation yet.

## Endpoint

The target is the actual RFC 7748 Curve25519 prime-order subgroup, over
`p = 2^255 - 19`, Montgomery coefficient `A = 486662`, cofactor 8 and
`r = 2^252 + 0x14def9dea2f79cd65812631a5cf5d3ed`.
Use public synthetic inputs. The primary comparison is a cold single-target
complete solve, with identical public targets, matched resources and the
strongest applicable rho reference. Charge setup, failures, decomposition,
relation verification, linear algebra, descent and final verification.
Retain the repository's native-operation price band. Miniature measurements,
small-scalar windows, warmed factor-base logs and arithmetic throughput do
not discharge the full-size endpoint. No feasibility or completion date is
asserted here.

## Current source inspection

The authoritative campaign is `research/curve25519_autolab_20261104`,
implemented by `src/bin/ic_autolab.rs`. Its ledger ends at round 35.
Preserve all completed rounds, exclusions, exposures, seed corrections and
the generic/algebraic track separation. The campaign README labels its
reported results unverified pending independent replay and recomputation.
This continuation has inspected the source and retained descriptions, not
replayed those results or promoted their claims.

Two concrete admission limits prevent simply running that loop at 255 bits:

* `src/bin/ic_autolab.rs`, `SizeInfo`: subgroup order `r` is `u64`.
* `src/cryptanalysis/ecbench/workload.rs`, `CurveSpec::PrimeExplicit`:
  all field/group parameters are `u64`; its constructor requires
  `5 <= p < 2^62`.

`src/bin/curve25519_family.rs` provides a big-integer parameter audit and
miniature construction. Its audit is not a full-size IC solve. The existing
wide arithmetic and relation-elimination components must be traced through
an actual measured solver before claiming that widening a JSON field makes
the pipeline support Curve25519.

The reusable actual-curve implementation is already present:
`src/cryptanalysis/curve25519_wide.rs`, `Curve25519Wide`, uses `Fp256`,
BigUint scalars, an explicit Montgomery-to-Weierstrass transformation and
counted group operations. Its bounded `smoke` scan is not a complete IC
pipeline. The remaining shared interfaces are explicit in current source:
`ic_framework/stages.rs::InstanceCtx.r` is `u64`,
`ic_boundary.rs::CountedGroup::mul` accepts `u64`, and
`ecbench/workload.rs::Instance::r` returns `u128`. The existing ecbench
`Instance::Wide` is a binary Koblitz instance, not this prime-field backend.
Reuse `Curve25519Wide`; do not build a second Curve25519 arithmetic engine.

## Next action

Trace and implement the missing full-width path from the Autolab target
manifest through ecbench workload/target construction, solver dispatch,
relation coefficients modulo r, scalar recovery, counted costs and replay.
Start with the existing wide-field and wide-scalar implementations; identify
the narrowest reusable complete pipeline and its unsupported interfaces.
Preserve the existing miniature schema and frozen records. Do not launch a
full-size exhaustive search merely to demonstrate that parsing succeeds.
Use exact RFC parameters and known-answer group/relation tests for adapter
validation before a fresh, bounded comparative protocol is frozen.

After that path is executable, continue source-level candidate development
and fresh-target Autolab comparisons under finite budgets. Passing adapter
tests is infrastructure progress, not rho parity. A completion claim requires
the complete endpoint comparison and independent validation.

## Runtime state

No build, solver or cloud Job was launched by this continuation.
Earlier in this chat, direct Kubernetes readback failed because the worker
could not assume the configured IsoLab launcher role. The MCP overview's
configured caps do not establish live NodePool/quota enforcement. Reconcile
live jobs, authoritative cap readback, reservations and remaining daily and
monthly funds before any cloud submission; publish exact inputs and source
to GitHub and satisfy the checkpoint gate first. Do not repeat completed or
timed-out attempts under renamed registrations.

Conductor reported the new study directory clear. The shared
`docs/session-logs/2026-11-05.md` is held by task T-1590, so this scoped record
serves as the continuation log without writing into that task's reservation.

## Sources consulted

* RFC 7748 section 4.1, read on 2026-11-05:
  https://www.rfc-editor.org/rfc/rfc7748.html#section-4.1
* Current source paths above and the campaign README, ledger and
  `ideas/STRUCTURE-R4.md`.
* `research/curve25519_algebraic_ecbench_20261102/{README,PROTOCOL}.md`
  in the earlier checkout: historical miniature comparison and replay issue.
* `research/curve25519_rho_parity_bound_20261104/README.md`: a model argument,
  not treated here as a general impossibility theorem or a stopping rule.
* Petit, Kosters and Messeng, PKC 2016: primary PDF retrieval returned HTTP
  403. No new mathematical claim relies on the inaccessible paper.

## Session log, 2026-11-05

### What was done

Fetched main, created isolated branch
`codex/curve25519-autolab-parity-20261105`, reconciled the current campaign,
and identified full-size source admission limits. No raw run data changed.

### Decisions made

Keep actual Curve25519 as the endpoint. Continue the existing native Autolab
and ecbench architecture; do not restart its 35 completed rounds or substitute
miniature parity for full-size parity.

### Open threads

Full-width solver integration, finite experiment admission, fresh comparisons,
and independent validation remain. The exact next action is above.

### Errors

The desktop managed-worktree tool could not resolve the remote checkout;
native Git created the isolated worktree successfully. Conductor rejected the
shared daily-log scope; the new study scope passed separately. No permission
or resource limit was broadened.
