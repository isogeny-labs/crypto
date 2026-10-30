# Reuse generated relation targets: unapplied CPU candidate

The collector currently computes `[u]P+[v]Q` to query the oracle, then recomputes
that same target immediately to validate the returned tuple. This patch reuses
the target already computed by the collector. It still checks coefficient
bounds, tuple length, every point's validity and membership in its factor base,
and the independent affine sum against the exact target passed to the oracle.

The original `check_relation` and `verify` functions are byte-for-byte unchanged.
Both full certificate replays therefore still recompute targets from recorded
coefficients and independently check every relation. A new private collection
helper makes the trust boundary explicit: only the collector may supply its
already computed target. It does not accept a claimed target from an oracle or
certificate. The underlying fixed-base evaluation is deterministic, and target
is a copied immutable point passed to the oracle. See the inspected collection
loop and fixed-base evaluator in `src/cryptanalysis/hashed_ecdl.rs`.

This removes one duplicate fixed-base evaluation per accepted relation during
collection. It does not remove a relation equation, oracle validation, attempt,
trial, elimination, certificate check or serialization. The full end-to-end
clock remains unchanged. No performance gain is claimed before measurement.

## Status and validation

This patch is **unapplied**. T-906 continues to reserve the shared source path;
its task card is ready but still holds write-exclusive ownership. No reserved
source was edited, no reservation was reclaimed, and no cloud run was launched.
This is a separate one-factor candidate from `../optional-profile/`, not a
combined patch. Current executable timings exclude both proposals.

[Source identities](source.json), [patch](target-reuse.patch),
[validation helper](validate.py), [validation receipt](validation.json),
and [unchanged-verifier digest](verifier-check.json) are preserved.

All 32 release tests pass in an isolated standalone CPU snapshot, alongside
strict Clippy and formatting. The new test compares collection checks with the
original checker for every relation in a recovered certificate; rejects a
wrong target, identity/off-curve/noncanonical tuple entries, out-of-range
coefficients and wrong tuple lengths; and confirms full replay rejects a
changed recorded u coefficient. Existing exact archived-certificate checks,
all-target oracle tests and malformed-oracle cases pass unchanged.

Reproduce in a fresh disposable destination:

```sh
python3 research/hashed_ecdl_cpu_20261030/target-reuse/validate.py   /tmp/hashed-ecdl-target-reuse-unique   /tmp/hashed-ecdl-target-reuse-build
```

The helper refuses existing destinations, checks exact source and patch hashes,
applies only in the snapshot, tests the standalone crate and rechecks that the
original source did not change. Root crypto/CUDA is not built. Test logs are
development validation, not performance data.

## Handoff

- Completed: isolated and implemented duplicate target elimination; tested the
  exact patch and checked byte-identical independent verification; published
  the reviewable artifact on follow-up PR #2963. PR #2946 merged into the
  parent feature branch while this artifact was being published; its merged
  source excludes this patch.
- Decision: retain full independent replay and prepare this as a separate
  candidate so any later measurement can isolate its effect.
- Remaining: reconcile shared-source ownership, apply against current source,
  freeze a full five-arm panel with baseline2bd4588fba60 and current92463171363c,
  then evaluate complete output and timing records. The further10x goal remains
  unmet; strict isolation, external comparisons and larger-scale CPU recovery
  remain unresolved.
- Errors: no compilation, test, Clippy or formatting failures in this snapshot.
