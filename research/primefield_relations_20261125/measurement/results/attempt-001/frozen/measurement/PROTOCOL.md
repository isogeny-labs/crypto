# Count M31 shared-selector support and admission work

Protocol version 1. User request: measure the implementation integrated in
[PR #5213](https://github.com/isogeny-labs/crypto/pull/5213).
Measurement tracking: [PR #5217](https://github.com/isogeny-labs/crypto/pull/5217).

## Question and scope

Where does the successful fixed-fixture construction/replay spend arithmetic
work, and what does composed-selector membership cost compared with explicit
lookup in the same certified root set?

The source is the ordinary M31 curve, with `p=2147483647`,
`a=847514846`, `b=2087444724`, subgroup order `r=2147446831`,
`G=(625037715,2100627251)`, and `Q=(1477799919,824532935)`.
The selected row, terminals, and expected outputs are archival acceptance
controls, already disclosed in the parent study. No unknown-target solve or
new relation extraction is performed.

## Paired computations

Arm A evaluates the depth-three Dickson chain and searches the six sorted
terminal values. Arm B binary-searches the sorted explicit support, recovered
by inverse fibers and certified against `gcd(H(D8(X)),X^p-X)`.
Both implement membership of a canonical field element in the same set.
Both reject noncanonical coordinates before field reduction.

The primary ordered query corpus is every abscissa in the archived 4,608-point
public source. A second, explicitly all-positive diagnostic corpus consists
of every certified field root. Do not mix the two hit rates or describe the
positive diagnostic as a collector workload.

Execute two Arm A control passes, followed by five ordered pairs:
`AB, BA, AB, BA, AB`. Thus the fixed schedule has 12 passes (7 A and 5 B).
These are deterministic counter/replay controls, not independent instances
or a wall-time noise estimate. A second full executable invocation must
reproduce all mathematical outputs and counters byte-for-byte.
The six-test correctness suite contains one additional full-fixture invocation;
the prospective total budget is three complete workloads, with two yielding
the measured raw JSON outputs and one serving as a correctness check.

## Charged phases

Retain separate counter deltas for curve/target admission, archived source
construction, original-column preparation, terminal construction, inverse-fiber
root recovery, polynomial support certification, rational lifting, public-label
admission, and archived useful-row replay. Repeated work in the declared
successful path stays charged. Negative mutation tests and repeated membership
controls are outside that successful-path total and are identified separately.

Report complete-path count vectors per arm, or explicitly label totals assembled
from measured common phases plus the measured membership phase. Include setup
in both totals. Do not charge a source once and silently call a prepared-query
counter a cold complete-path result. This remains a construction/replay cost,
not a complete relation-discovery cost.

Counters distinguish native group-add calls, point-scalar calls, field modular
products including squares, field inversion and square-root calls, explicit
selector/polynomial work, scalar-field products/inversion, binary-search
comparisons, membership queries, and admissions. State the exact instrumented
units and excluded allocator/hash-table/compiler work in the output and report.
Group-add calls include identity/doubling cases and are not calibrated GAE.

The original `Arithmetic::Counts::scalar_multiplications` names scalar-field
products, not point scalar-multiplication calls. Supplement it explicitly.
The free polynomial/chain routines and final beta inversion must be counted.

## Output agreement and controls

- Reproduce the frozen terminals, complete roots, curve lifts, historical tag
  identities, `alpha`, nonzero `beta`, and recovered scalar of the parent study.
- Compare A and B on every primary and all-positive query; retain the ordered
  query stream and accepted indices for custody.
- Join lifted support against the full 4,608-column pool. Report full-pool
  coverage separately from coverage by the archived three-column roster.
- Test depth-three operation counts, the native M31 square-root multiplication
  count, canonical-input rejection, and disjoint phase-delta reconciliation.
- Keep wrong-tag/sign/column and zero-beta acceptance gates available through
  the parent study; do not remove or rewrite its frozen records.

## Reproducibility and admission

`seed-manifest.json` fixes the historical SplitMix64 stream, deterministic
hash-table initialization, source ordering, and comparison order. There is no
target-generating stream, tuning panel, resampling, randomized solver, or
randomized execution order. All arms use identical materialized inputs.

Pin Rust 1.100.0, source commit, source/qualifier hashes, executable hash, build
flags, host, one-thread resource limits, and exact command before execution.
The frozen build tree also includes the parent attempt-001 `result.json`, whose
content is included at compile time; retain its original relative path and hash.
Record through the repository benchmark-custody tool; all scientific
calculation and comparison code is Rust. Preserve raw JSON, test outputs,
manifests, and any failure under a unique attempt identifier.

The available host lacks the PSI evidence for the lab's admitted isolated
wall-time path. Counted arithmetic observations are the primary result;
elapsed command time is not a speed comparison. The default cloud execution
workflow was reviewed, but this task authorizes no paid run.

No performance ranking against FLINT/NTL/Magma or rho is made. The paired
reference is explicit membership in the exact same finite set. A comparison
with zero field multiplications on one side has no meaningful multiplication
ratio; retain both count vectors and comparison/storage costs. [pcc:unverified]

## Prior work and non-overlap

Reuse the parent study's PKM/APS sources and pinned native arithmetic. Existing
PKM map-choice/depth/yield work is owned by #5041/#5061; PairProjection,
recursive-join, subfield, and H016 paths remain outside this measurement.
This protocol does not nominate another map-family sweep.

Source policies read: `docs/experiment-seeds.md`, `docs/benchmark-records.md`,
AGENTS sections on native implementation, counted units, and benchmark
admission, and the workspace `isolab-eks` execution skill. The latter requires
published source, a finite budget with known remaining balance, and live
capacity admission before any cloud build or run; none is launched here.
