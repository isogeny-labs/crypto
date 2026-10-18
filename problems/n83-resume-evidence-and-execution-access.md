# N83 resume evidence and execution access

- **Status:** open
- **Kind:** access/tool limit; missing run evidence
- **Affected work:** [primary factor-base sweep](../research/koblitz_n83_factor_base_sweep_20261008/RESULT.md)
- **Next action owner:** N83 sweep maintainer
- **Last updated:** 2026-10-18

## Problem

The resumed primary `a=0` GF(2^83), 81-bit-subgroup study cannot rerun the
complete K2048 high-cap audit or continue its original guarded search
because the original binary and required execution/access capabilities
are unavailable on the current executor.

## Evidence and uncertainty

- **Observed:** the initial PR head was `8f971585692c702d2804668b322e2440177f80d2`,
  without the later run. During recovery upstream advanced to
  `63844b3b2b674d31ef07e8dec8d0f63ffe66bc52` and published its retained receipts.
  All 31 K2048 and 26 K4096 manifest file hashes passed; five K2048 source
  snapshots, worker consistency and independent public-query replay passed.
  Its expected Mac path is
  `/Volumes/SSD990/crypto/artifacts/n83_k2048_natural_highcap_20261018/`.
  That path has not been inspected from this executor.
- **Observed:** HEAD on the exact K2048 S3 key returned 403. No AWS profile
  selector or OIC identity manifest was provided; Docker and Conductor are
  unavailable. These observations do not identify the cause of S3 denial.
- **Recovered:** [native reconstruction](../research/koblitz_n83_factor_base_sweep_20261008/verification/cloud-input-reconstruction-20261018/RESULT.md)
  reproduced the exact archive and passed every-point generic replay and a
  semantic mutation rejection. Lack of archive bytes is therefore resolved.
- **Remaining audit limits:** original static binary bytes and live Docker
  cleanup have not been rechecked on this executor. Retained audit receipts
  report their earlier checks; current recovery does not repeat them.
- **Not established:** natural relation rate, useful rank, complete cold
  runtime, minimum-runtime base, or any mathematical obstruction.

## What would unblock it

- [x] Recover the retained Mac protocol/source/run receipts and check their
  manifest hashes, source snapshots and public-query group replay.
- Original static-binary and live-cleanup rechecks remain unavailable here.
  The retained earlier full audit remains evidence; repeating it is not a
  prerequisite for a new experiment with its own verified worker.
- [ ] Restore the retained supervisor's one-CPU, 4-GiB, zero-swap,
  networkless execution environment and verify it live.
- [ ] Recheck authorized S3 access with HEAD/GET and exact hashes before
  claiming fresh S3 provenance; reconcile the campaign capacity ledger.

These gates permit audited continuation. They do not establish a winner;
natural rank, column logs, target recovery, matched references and full
cold accounting remain separate requirements.

## Next discriminating step

Restore and live-check the guarded execution environment, then freeze the
next retained-base capacity/yield test using a verified worker and exact
public inputs. K4096 model construction remains unexecuted upstream; its
source projection exceeds the earlier one-million-domain-clause cap and
needs a declared admissible cap before launch. The exact K2048 reconstruction
can be reused with explicit reconstructed-input provenance; it is not a fresh
S3 GET. Reconcile the capacity ledger before spending further search time.
If runtime/access checks fail, retain the infrastructure failure without
launching the dependent search. Recheck when executor capabilities change.

## Resolution and history

2026-10-18: exact input reconstruction resolved archive recovery. Later-run
retained receipts were subsequently published upstream and passed hash/source/
query replay. Full original-binary audit, execution access and full-runtime
selection remain unresolved.
