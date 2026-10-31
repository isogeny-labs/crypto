# Ternary packet label algebra: partial formalization

This project proves the proposed ternary packet's central label action and
structural locality over every additive commutative group. It also proves
uniform, positive slot-map fibers and the fixed-point statements needed over
the label field `ZMod 67`. The complete contraction theorem remains open in
this formalization. No ECDLP algorithm, novelty claim, or complexity improvement
is certified by this project.

The label field is separate from the coefficient ring's characteristic-three
arithmetic. None of the declarations defines or assumes a 3-adic coefficient
ring.

## Definitions and scope

`Pair A = A × A` and `Tag A = Pair A × Pair A`. The rotations are
`rot (r,c) = (-r-c,r)` and `invRot (r,c) = (c,-r-c)`. A digit is the actual
Mathlib type `ZMod 3`. `tagAction d` applies the forward rotation indexed by
`d` to the first pair and by `-d` to the second pair.

`row c` and `col c` select the first pair when `c=1` and the second pair
otherwise. Locality assumes `c≠0`, so the selected component is exactly one
of components 1 and 2. `slotAction` applies the action pointwise to slots
`Fin a → ZMod 3`. In `slot_locality`, the hypothesis `y i = x i + c` is
explicit; the theorem does not silently replace this by an unproved statement
about the 3-adic valuation of an integer slot difference.

`orbitFiberEquiv` gives an explicit translation bijection between fibers over
any two reached classes. `orbit_fiber_uniform`, `orbit_fiber_pos`, and
`slot_count` show these fibers have the same positive cardinality and that the
slot domain has cardinality `3^a`, for every natural `a`, including zero.
This is not yet the classification of every orbit as `3^(number of active
coordinates)` or the complete packet-padding theorem.

`NoThreeTorsion A` means exactly `∀z, z+z+z=0 → z=0`. It is required for the
fixed-point classification, not for order three or locality. Its `ZMod 67`
instance is proved by kernel-checked finite `decide`; the principal algebraic
and structural statements are general theorems rather than finite test
certificates. See [PROOF_MAP.md](PROOF_MAP.md) for every reported declaration
and every missing bridge.

## Pinned dependencies

| Component | Exact pin |
| --- | --- |
| Lean | `leanprover/lean4:v4.35.0`, release commit `eb6cafb7290e8bd5f2937522d8d804f58f02bafc` |
| Shared library and unmodified audit/gate | `aburan28/crypto-lean` commit `148f5dc389b341401e06d76abe7783c4ccf42f2e` |
| Mathlib, inherited from the shared pin | `cfc55fc58714a59dc06e6e79edc6d1d95140d3c2` |
| Remaining transitive packages | Exact revisions in `lake-manifest.json` |

The official Linux binary archive was downloaded from the Lean GitHub release.
Its SHA256 matched the release asset's digest:
`7289597a44543246c9227b8cae501767e095ce17f6d448b95fc4a8dc97268f6d`.
The official installation page, asset metadata, observed versions and checksum
are retained in `receipts/`. Installation was unprivileged; no shell profile,
system installation, or credential setting was changed.

## Reproduce

Run these commands in this project's directory with the pinned Lean/Lake on
`PATH`. This session used
`/workspace/audit/toolchains/lean-4.35.0-linux/bin`.

```sh
MATHLIB_NO_CACHE_ON_UPDATE=1 lake update
lake exe cache get Mathlib.Algebra.Group.Basic Mathlib.Data.ZMod.Basic Mathlib.Tactic.Abel Mathlib.Tactic.FinCases Mathlib.Tactic.NormNum
lake build
lake build +TernaryPacket
lake env lean --run .lake/packages/CryptoLean/tools/AuditAxioms.lean TernaryPacket
lake env leanchecker --fresh --verbose TernaryPacket
```

The unmodified shared gate can also run against the project from its containing
Git repository. It fetches the full Mathlib cache before its build/audit.
`lean4checker` is not installed under that name; the official Lean release
ships the equivalent documented fresh replay tool as `leanchecker`. Fresh
replay checks all imported and newly defined constants in a fresh Lean kernel
environment; it is not an independent external verifier.

## Verification and retained attempts

`receipts/build-06.log` records the final successful `lake build` and all 20
reported `#print axioms` outputs. The explicit module build and the pinned
shared-library audit and unmodified shared proof gate also succeeded. The audit reports 20 theorems, zero
failing and zero trusted native proofs. All transitive axioms are among
`propext`, `Classical.choice`, and `Quot.sound`.

The gate discovered and explicitly checked `TernaryPacket.lean`, reporting
`PASS`, 20 theorems and zero trusted proofs. Module-only replay and the
import-inclusive `leanchecker --fresh` both exited successfully. The fresh
receipt reports `replaying TernaryPacket with --fresh`; exact verification
statuses are recorded in `receipts/VERIFICATION.json`.

Build attempts 1–4 failed. Their exact sources are retained as `.lean.txt`,
and their full logs are retained locally and as deterministic gzip archives
for repository packaging. The initial proofs exposed a `fin_cases` issue
on `ZMod 3`: even a trivial digit trichotomy after enumeration failed, with
universe and decision-instance errors. The corrected proof uses a finite
trichotomy checked by ordinary `decide`, then `rcases`; it does not use
`native_decide` or add an axiom. Separate retained smoke logs document the
failure. Builds 5 and 6 succeeded. The first update's full automatic cache
fetch was intentionally terminated; the subsequent update and targeted cache
fetch succeeded. These are proof/dependency attempts, not scientific timing
measurements or evidence against the mathematical statement.

The packaged failed logs can be checked without retaining megabytes of plain
text in the repository:

```sh
gzip -dk receipts/build-0[1-4].log.gz
sha256sum -c receipts/failed-build-raw-sha256.txt
```

`receipts/COMMANDS.md` records commands and outcomes.
