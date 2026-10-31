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
[validation helper](validate.sh), [validation receipt](validation.json),
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
bash research/hashed_ecdl_cpu_20261030/target-reuse/validate.sh /tmp/hashed-ecdl-target-reuse-unique /tmp/hashed-ecdl-target-reuse-build
```

The helper refuses existing destinations, checks exact source and patch hashes,
applies only in the snapshot, tests the standalone crate and rechecks that the
original source did not change. Root crypto/CUDA is not built. Test logs are
development validation, not performance data.

The active helper uses Bash, GNU coreutils, jq, Git and Cargo. The original
`validate.sh` and its receipts remain unchanged as historical development
evidence; the Python helper is retired from the reproduction path. The shell
runner retains the exact source/patch/candidate hashes, atomic fresh-directory
refusal, isolated patch application, all three checks and source readback.

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

Shell replacement validation: all32 standalone release tests, locked strict
Clippy, formatting and Bash syntax pass. Negative checks refuse an existing
snapshot and reject wrong source/patch hashes before creating a destination.
The new [software receipt](shell-validation/receipt.json) and exact command logs
are additive; historical Python-era artifacts and the reserved source retain
their original bytes. These are development checks, not timing measurements.

The active reproduction path uses thin shell tooling and the shared
`../validate-snapshot.sh` runner. It reconstructs source, crates and certificate
fixtures from the exact `base_commit` in the immutable descriptor, then checks
the base, patch and candidate digests. This remains reproducible after later
source integration. The current shared source is hash-checked before and after
and is never edited. Original Python helpers and historical receipts remain
unchanged as evidence, outside the active reproduction path. Requires Bash,
GNU coreutils, jq, Git and Cargo. No timing or scientific run is performed.

The [pinned shell reproduction receipt](pinned-shell-validation/receipt.json)
records successful release tests, strict Clippy and formatting after the later
source integration. The current shared source is retained byte-for-byte. The
source descriptors and Python-era receipts keep their original recorded state.
