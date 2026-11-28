# N83 K600 m6 preflight lock and arm source-commit cycle

- **Status:** `open`
- **Kind:** implementation and execution-protocol gap
- **Affected work:** [frozen K600 m6 protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-full-smooth-m6-20261114/PROTOCOL.md), [native preflight](../examples/n83_k600_m6_native_preflight.rs), [native arms](../examples/n83_k600_m6_native_arms.rs), [arm budget controller](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/ARM_BUDGET_CONTROLLER_AMENDMENT.md), [total-runtime selection](n83-mixed-orbit-factor-base-selection.md)
- **Next action owner:** N83 native execution-path implementer
- **Last updated:** 2026-11-15

## Problem

The current K600 six-summand preflight and arm commands cannot pass their
source-commit gates in sequence from one clean checkout: committing the
preflight lock advances HEAD, while both the arm lock and preflight gate
require one identical source commit. This blocks a qualified constructed or
natural K600 arm before any SAT or factor-base measurement begins.

The affected instance is primary `a=0` GF(2^83), retained signed-Frobenius
K600/B99,600 object SHA-256
`3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`,
six summands, public-X hash policy and seed 2026100801. The [frozen
protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-full-smooth-m6-20261114/PROTOCOL.md)
requires a committed source/input lock, zero-trial preflight, then pinned,
unpinned and natural arms under one source identity and full cost accounting.

## Evidence and uncertainty

- **Observed in source:** `native_preflight::prepare` reads clean HEAD `C`
  into `lock.source_commit` and writes a new lock in the source checkout.
  `native_preflight::run` requires that lock to match `git show HEAD:path`,
  so committing it produces a new HEAD `H`. It permits `C` as an ancestor
  and checks the frozen source-file hashes. `native_arms::freeze` records
  its then-current clean HEAD; export and later stages require that exact
  HEAD in the arm lock. `preflight_gate` requires
  `preflight.source_commit == arm_lock.source_commit` and the preflight
  worker config/summary to carry that same value. The arm image inspection
  also requires its source-revision label to equal the arm freeze commit.
- **Derived for this exact workflow:** If freeze runs before the lock
  commit, its arm source is `C` and export after the commit rejects current
  HEAD `H`. If freeze runs after the lock commit, its arm source is `H` and
  the preflight gate rejects the preflight lock's `C`. Since the new tracked
  lock changes the tree, `H != C`. This is a constraint conflict in the
  current implementation, not an algebraic obstruction to index calculus.
- **Missing capability:** A two-revision custody contract, or an equivalent
  immutable lock storage route, that preserves exact source-file and image
  identity while allowing the committed evidence lock to advance HEAD.
- **What this evidence does not establish:** No qualified Linux K600
  preflight or arm was launched to test these gates end to end. This static
  source audit says nothing about natural relation yield or runtime.

## What would unblock it

- [ ] Define and implement separate `source_revision` (the committed
  build/source bytes) and `evidence_revision` (the commit that contains the
  preflight lock), or store the lock in a verified detached evidence ref.
  Reject any change to the frozen source-file hash set, binaries, image ID,
  input object, fixtures or worker config between revisions.
- [ ] Add a native integration test that starts from clean commit `C`,
  prepares and commits a lock to `H`, and verifies that preflight and arm
  admission accept this exact two-revision case while rejecting a changed
  source file, wrong image label, substituted lock and unrelated branch.
- [ ] On the qualified Linux arm64 host, build the source-matched image,
  run the committed zero-trial preflight, and verify its read-only input,
  cgroup, terminal exit and source receipts before any constructed arm.

The first two conditions repair the implementation contract; the third
provides host evidence. They do not establish natural relation yield,
rank, target descent or a minimum cold IC runtime.

## Next discriminating step

- **Action or proof obligation:** Choose the two-revision or detached-ref
  design, patch the native admission path, and run the clean-checkout
  integration test with both acceptance and tamper rejection.
- **Inputs and controls:** Exact K600 object and source hash set above;
  one unchanged-source lock commit as positive control and source, image,
  lock and ancestry mutations as negative controls.
- **Execution limits and stop condition:** Unit/integration tests only
  until the frozen host, input and execution-budget gates pass. Preserve
  all failure receipts; do not launch a search from a merely patched gate.
- **Outcomes and next actions:** A passing test enables a separately
  qualified host preflight. A rejection of the positive control keeps the
  protocol blocked. A host failure is recorded by its actual cause.
- **Recheck trigger:** A versioned native lock-contract patch and test.

## Resolution and history

2026-11-15: Static source audit identified the commit-identity cycle. No
execution attempted under this record. Unresolved.

2026-11-15: The [two-revision source amendment](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/TWO_REVISION_SOURCE_AMENDMENT.md)
adds a strict one-lock `C..H` tree-diff and direct-parent predicate, binds the image to `C`
and arm execution to `H`, and passes a temporary-repository acceptance and
tamper test. The dependent preflight gate test also passes. The complete
Linux image/preflight/freeze chain remains unexecuted, so the problem stays
open for host verification and full budget custody.

2026-11-15: A subsequent audit found four runtime image-label comparisons
still using evidence commit `H`. The [versioned correction](../research/koblitz_n83_factor_base_sweep_20261008/verification/k600-m6-native-arms-20261115/RUNTIME_IMAGE_LABEL_CORRECTION.md)
binds them to build commit `C`; 15/15 release arm example tests passed.
Host execution and complete cold budget custody remain open.
