# Exact wide-field ecbench inputs

[PR #4723](https://github.com/isogeny-labs/crypto/pull/4723), stacked on
[#4721](https://github.com/isogeny-labs/crypto/pull/4721).

`prime_mont4_explicit` adds exact large-prime-field inputs to ecbench through
the repository's existing `Mont<4>` arithmetic. Coordinates use four limbs;
subgroup orders for these generic methods remain u64. The original 202-bit
source and 96-bit pilot therefore fit this representation without changing
their fields or prime subgroup orders. This is a backend component; the
80-bit quotient/map and repeated endpoint measurement criteria remain open.

## Contract and accounting

`p,a,b,group_order,gx,gy` are canonical unsigned decimal strings. The field and
order must fit 256 bits; a and b must be nonzero, avoiding unsupported special
automorphism models. Input coordinates are checked before conversion to
Montgomery form. Exact subgroup, membership, nonsingularity and Hasse checks
are enforced. External certificates are still required for the prime field
and supplied ambient order. Hasse eligibility and `[r]G=O` leave both certificate
obligations open.

Counted group additions/doublings use the existing ecbench ledger. Semantic
field multiplications, squarings and inversions are also tallied; inversion's
internal Fermat exponentiation is represented by the inversion count. No
native price is inferred for an uncalibrated large field.

The enabled methods are existing `rho.negation` and separate
`bsgs.prime_mont4_v1`. The latter uses complete point keys in a deterministic
B-tree. It records inserts, lookups and stored point-key bytes; node overhead,
allocation and exact comparison work remain unpriced. Its final scalar check
belongs to the runner, outside these solve totals. Rho retains its existing
complete candidate check. The 64-bit rho key is a walk hash, not a full point
encoding. Existing word-key BSGS is refused on this backend. This is not a
performance claim or a fully priced comparison.

Public target hashing, special j=0/1728 models and IC/factor-base adapters are
explicitly unavailable here. The new variant does not change existing word
variants, committed method algorithms, registry identities or admission gates.

## Qualification status

Main-library `cargo check` completed. The full release library test build
failed on a pre-existing duplicate test name in `ic_framework/plugins.rs`,
lines 5285 and 5322 (E0428). Its initial missing include subjects and exact
failure output are retained. The duplicate module was not edited.
[pcc:unverified]

The separately pinned PARI source oracle produces 32 shared-law point vectors
on each of the frozen 202-bit source and 96-bit pilot. It checks prime p/r,
source membership and `[r]G=O`, then computes the scalar products. These are
source-only vectors, without a quotient, target isogeny image or timing.
PCC-20261118-224ceb binds this evidence as an unsigned producer attestation.
[pcc:unverified]

The corrected-source focused release target links the actual root ecbench
library and passes all five integration tests. It checks 64 PARI vectors,
BigUint affine arithmetic, tiny BSGS recovery, rho/BSGS dispatch/replays and
canonical guards. The separately frozen qualifier first requires exactly five
listed tests. Root-library clippy passes with warnings denied. The legacy
unit-test failure remains separate. [pcc:unverified]

The unchanged CLI source is compiled in a separate development manifest
against the same real library. It matches 192 exact scalar/coordinate rows
for source, destination and coordinate-control models in two fields. All
192 planned workloads are unregistered. Off-curve generators, tampered rows
and unsupported IC reject. No `run` command was invoked. [pcc:unverified]

The first five-round alternate-order plan produced an actual imbalance
warning and failed the strict empty-stderr check. That input/output remains
unchanged. The separately named successful plan uses six measured rounds
and one warmup, preserving the original minimum. Balanced arm order does
not establish paired endpoint scheduling or algorithm-seed pairing.

The focused lock differs from the parent lock only by its package entry.
The initial offline generation's version drift is preserved in raw evidence;
that version is not used by the active locked build. The official Rust
1.100.0 components are checked against the dated 2026-11-12 stable manifest.
Source and binary pins are separate from Git/source diagnostics.

## Evaluated small-degree controls in the actual fields

| Field | Control degree | Native/PARI map-input checks | Shared nonzero scalars | Harness rows, three models |
|---|---:|---:|---:|---:|
| Frozen 202-bit source field | 2 | 35 | 32 distinct | 96 |
| Frozen 96-bit pilot field | 5 | 35 | 32 distinct | 96 |

The checks total 70 inputs: generators, 64 scalar targets, two kernel inputs
and two infinities. They are input records, not 70 distinct non-kernel points.
The degree-5 case exercises the unchanged complete polynomial evaluator
above word width; the degree-2 case uses the standard closed Vélu map.
Each target satisfies phi(sG)=sH and each H is nonzero of prime order r.
Coordinate controls use (x,y)→(4x,8y) with explicit inverse checks.
[pcc:unverified]

For the actual 202-bit field, the explicit control is
`E: y²=x³−140x+784` → `F: y²=x³−2380x+44688`, with kernel generated by
`(-14,0)` and map `x'=x+448/(x+14)`,
`y'=y*(1−448/(x+14)²)`. Kernel and infinity map to infinity. The cleared
curve-equation identity is zero over the integers. The separate degree-5
equation/map and every point are in `results/five-map.json` and `.pari`.
These are degrees 2 and 5, not the 80-bit target or the pilot's 47-bit degree.
Their conductor-gap fields remain null. [pcc:unverified]

The [transfer assessments](transfer-assessments.json) type both edges over
their prime fields and separate existence/construction, map checks, subgroup
preservation, computational advantage and scaling. They are companion drafts,
not authoritative certificates or promotions. The supplied ambient orders
are inherited by the rational isogenies; no fresh point count was performed.

| Cost term in construction + transfer + solve + recovery + verification | Status |
|---|---|
| Construction, conversion and map evaluation | Unmeasured |
| Source/destination scalar multiplication and ECDLP | Unmeasured in these wide fields |
| Recovery and independent verification | Unmeasured |
| Peak RSS, setup reuse/amortization, uncertainty | Unmeasured; no invented zero or estimate |

PCC-20261118-a0f9ac and -f0f516 bind the two map controls. Final
PCC-20261118-f8faf8 binds corrected-source tests and paired input checks.
All have unsigned standing `unverified` and verdict `not_covered`.

## Requirements retained

| Requested deliverable | Status |
|---|---|
| Prime endomorphism-conductor gap at least 80 bits | Frozen candidate retained; complete target quotient/images and endpoint evidence open |
| Exact fields on both endpoints | Checked on degree-2/5 controls; full 80-bit destination unavailable |
| Dozens of points and several scalars/fields | Two sets of 32 transported scalars; no wide-field endpoint timing |
| Scalar multiplication measurements | Canonical measurement integration still open |
| Paired ECDLP, rho, BSGS and IC | Tiny recovery controls pass; wide-field IC unavailable; no measured wide-field session |
| High fidelity, five rounds, warmup, L2, controls | Original protocol preserved; endpoint scheduling/seeds, strongest wall reference and execution open |
| S3/EKS execution | No target job; live budget/reservations/capacity and tested image still required |
| Largest certified evaluated crossing | Not established by this component |

## Reproduction

From repository root, build without executing tests:

```sh
study=research/prime_conductor_gap80_20261031/wide-harness-20261118
cargo test --manifest-path "$study/Cargo.toml" --locked --offline \
  --release --lib --no-run --message-format=json
```

Freeze the resulting executable and source hashes before running
`qualify-focused-frozen.sh TEST_BINARY FRESH_OUTPUT`. It uses a 300-second
correctness deadline, not a reported runtime. Each replay needs a new output
path. The earlier `qualify-frozen.sh` and full-unit-build manifest are retained
as historical preparation, not as executed tests.

The manifest links the actual library and test file and keeps exact vector
inputs. `cli/Cargo.toml` compiles the original CLI source and its command
modules. `qualify-paired-cli-frozen.sh CLI CHECK_PLAN FRESH_OUTPUT` performs
only plan/codec checks; `qualify-map-negative-frozen.sh` retains the off-curve
controls. CLI `run`, `verify` and `compare` were not called.

The complete new raw tree is packed as deterministic `runs.tar.xz`, with its
checksum alongside it. Extract here to restore `results/`. Individual PCC
proof subjects remain loose. Version-1 source/manifest subjects are retained
under `results/source-v1/`; the corrected version has separate manifests,
outputs and target directory. Do not relabel older binary evidence as the
corrected source. Historical CLI/build manifests remain bound to their own
attempts and archived snapshots.

All requested performance and crossing requirements remain in the original
protocol. No benchmark row or scientific leaderboard is promoted here.

## Sources

KB unavailable: no loaded tool, repository wrapper exit 127. Existing primary
implementation contracts reviewed before work:

- [Montgomery arithmetic](../../../src/ecc/mont_vt.rs), [CountedGroup](../../../src/cryptanalysis/ic_boundary.rs), and [generic BSGS](../../../src/cryptanalysis/ecbench/generic.rs).
- [Workload/target law](../../../src/cryptanalysis/ecbench/workload.rs), [method registry/accounting](../../../src/cryptanalysis/ecbench/methods.rs), and [harness guide](../../../docs/ecbench/README.md).
- [PARI elliptic-curve reference](https://pari.math.u-bordeaux.fr/dochtml/html/Elliptic_curves.html), especially ellmul, ellisoncurve and isprime.
- [General Vélu implementation](../../../isogeny_algos/src/kernel/velu.rs), including its explicit 2-torsion versus ±-pair weights.
- [Rust BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) and [Cargo test target rules](https://doc.rust-lang.org/cargo/commands/cargo-test.html).
- [Frozen target parameters](../parameters.json), [prior transported inputs](../paired-driver-20261118/README.md), and [retained measurement protocol](../MEASUREMENT_PROTOCOL.md).

No new arithmetic formula, complexity advantage or record is claimed.
