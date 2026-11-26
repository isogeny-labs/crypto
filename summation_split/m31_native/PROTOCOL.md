# Native M31 projective comparison protocol

Status: implementation and correctness development; performance execution pending.
Continuation of crypto PR #5087. Date: 2026-11-23 America/Los_Angeles.
No merger, factor-base change, lower-characteristic substitute or new background
experiment is authorized by this file. The owner's current request authorizes
continuing this lane; required execution admission is still applied.

## Frozen mathematical contract

- Coordinate prime: p = 2147483647, with E: y^2 = x^3 - 3x + 21.
- The quadratic extension is solely a geometric witness representation,
  omega^2 = -1. Original coordinates and projected outputs stay over M31.
- Source presentation: branch head 76998f1c0b15255bb26ea60427e35c52b968776e;
  preserve historical Python/C++ evidence without rerunning those research paths.
- Inputs: m = 3 and 4 summands plus one fixed affine target coordinate.
  Each original input is an affine form in t,u, with rank two before section.
- Common diagnostic section: t(t-1) = u(u-1) = 0. Its four points are M31
  points. This is a bounded correctness workload, not a natural factor-base
  scaling experiment. Production factor-base map L is untouched.
- At least 50 deterministic planes per configuration. Fixture family names
  describe only the planted positive offset tuple; other section points can
  have different sectors. The common input stream is frozen before any solve.

## Required arms

1. Monolithic S_(m+1), computed by fixed-degree Sylvester resultants; target
   square and all other constants folded before solving.
2. Preclosed cubic sequential tree, retaining every projective chart.
3. Preclosed cubic balanced tree, with the identical leaves and section.
4. Multiplicity-balanced tree: the third arm plus guarded derivatives.

Both the geometric (z,w,s) presentation and explicit Fp2 coordinates are
supported. Use the geometric presentation for the local quotient engine;
explicit Fp2 requires its own seven-relation local ideal. Do not pretend
that its cross relations are the four-relation geometric ideal.

The final geometric decomposed exporter uses the identical chart-first
variable order as the quotient adapter, then leaf-square variables and t,u.
The manifest records this order explicitly. The shared fixture generator
also owns exceptional leaf permutations, so export, witness qualification
and projected-output checking refer to the identical plane.

The multiplicity guard is computed in the common section algebra from every
original leaf INCLUDING the target. Let e_i = f(x_i)^((p-1)/2). Set

G = product(e_i^2) - product((e_i^2+e_i)/2) - product((e_i^2-e_i)/2).

Append NF(G * partial_t S) and NF(G * partial_u S), differentiating the
unreduced monolithic S first. The guard is zero on either pure sector and
on a zero-ordinate branch. This preserves zero sets; equality of nonradical
ideals is not claimed. Charge the full monolithic construction, derivatives,
Euler powers, reductions, branch handling and additional rows to this arm.

## Native implementations and correctness gates

- `quotient.rs`: exact arithmetic modulo all four relations at every node.
  Local quotient products alone are not a complete F4 algorithm.
- `f4.rs`: bounded batched critical-pair/matrix prototype, with J-pairs
  retained. Completion requires empty pair queue; limits are incomplete,
  never an unsatisfiability result. An independent ordinary Buchberger
  recomputation checks both ideal containments and all critical pairs.
- `systems.rs`: all four matched generators and immutable msolve export.
- `multiplicity.rs`: independent field, point and Sylvester arithmetic;
  complete signed-branch multiset and rational root GCD, including infinity.
- `system_certificate_tests.rs`: all equations evaluated on independently
  constructed Fp2 witnesses, plus monolithic exact-value and mutation checks.
- CLI `check-projection`: exact equality on the complete prescribed section,
  rejecting foreign characteristic, malformed output and leftover auxiliaries.

The 50-per-cell witness tests are NOT 50 completed Groebner extractions.
The required solver gate remains 50 full projected solution sets per
configuration. Signed E(Fp) factor-base relation recovery is a separate
downstream contract; a geometric Semaev zero is not automatically such a row.

## Measurement fields and comparisons

Compare native quotient F4 against the original applicable Rust F4 and
msolve 0.11.0 exact sparse F4 (`-l 2 -t 1`), on identical inputs and a matched
term order. msolve's two-block elimination mode (`-e`) uses different order
semantics from a single graded order; record that as a distinct projection
comparison, not an unexplained engine difference. Singular `std` is an
independent algebra checker, not a separately claimed F4 implementation.

For each case retain: source/input/binary hashes, full command, arity and
mode, term and variable order, original output set, recovered output set,
completion/censor reason, maximum raw pre-normal-form row degree, maximum
actual normalized matrix-column degree, rows/columns/nonzeros, pairs visited,
complete preparation/solve/extraction time, per-process peak RSS and counters.
Separate generator degree, pair-LCM degree, actual matrix degree and any
mathematical solving-degree claim. msolve's printed current-pair degree is
not automatically accepted as scanned maximum matrix degree.

Use one process per case for peak RSS, fixed thread count, interleaved arms,
an A/A noise panel, at least five timing rounds after correctness, and
complete generation, guard and root-extraction accounting. Retain every
timeout/resource limit. Do not rank censored cases as completed solves.
Leave unmeasured fields absent, never zero. No asymptotic or full ECDLP
inference follows from this fixed-size four-point section.

## Execution admission and finite proposed budget

Current available algebra executables: Sage 10.10, Singular 4.4.1,
msolve 0.11.0, Rust/Cargo 1.100.0, installed by workspace PR #109.
Current missing verified access: shared Conductor/research-KB, authenticated
EKS submission and live capacity/lease view, checkpoint/S3 custody. Do not
infer an empty queue or launch permission from a local process snapshot.

Proposed admission envelope, not an executed or reserved job: one 4-vCPU,
16-GiB x86 worker, one solver thread, 30 seconds and 4 GiB per case, maximum
2 hours total and $5 total including build/idle/failures; zero automatic
retries. All four arms over both arities remain required even if some time
out. A timeout is censored. EKS owner ceilings still apply. If this envelope
cannot satisfy the workload, retain the unmet requirement and revise the
protocol before spending; do not silently shrink the field or case count.

Before execution: recover a shared lease, verify capacity and remaining
budget, publish exact source/config/launch command, pass checkpoint gate,
obtain image digest and native isolation/A-A admission, then submit once
through the supported IsoLab route. Record submission identity and reconcile
ambiguous status before any retry. Shared connection work already belongs
to workspace PR #109; do not start a competing local control plane.

Local builds and unit/static checks remain development under AGENTS section
9a. The delivered CLI only prepares inputs or checks supplied outputs; it
does not launch solvers or remote infrastructure.

## Literature and prior-work receipt

KB unavailable: no loaded MCP endpoint, served catalog or authenticated
wrapper. Used exact PR #5087 sources and the concurrent PR #5120 local code
as fallback. PR #5120 owns selector input normalization and the ordinary
three-summand signed solver; this lane owns the projective chart quotient.

- Semaev, 2004: https://eprint.iacr.org/2004/031 (summation-polynomial construction).
- Diem, 2011, Proposition 3.2(c), Lemma 3.5:
  https://doi.org/10.1112/S0010437X10005075 (projective fiber and fixed-resultant semantics).
- Kosters and Yeo: https://arxiv.org/abs/1503.08001 (first-fall pitfalls;
  their characteristic-two morphism is not an M31 theorem).
- Caminata and Gorla: https://arxiv.org/abs/2112.05579 (solving/last-fall
  distinction; first fall is not a substitute for actual matrix-degree data).
- msolve source and input contract:
  https://github.com/algebraic-solving/msolve (unique polynomial terms and exact F4).

No novelty-priority claim, signed PCC verdict, performance result or Lean
theorem is established by this protocol. Keep PR #5087 draft and unmerged.
