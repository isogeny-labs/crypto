# Replayable ECDLP and point-decomposition corpus

Design v1, 2026-10-27 (America/Los_Angeles). Implementation and coverage must be
read from the accompanying status record; this document specifies the intended
corpus, including work that has not yet been implemented. Synthetic inputs only.

## Purpose and acceptance criteria

Build a native `crypto dataset` CLI that produces a large, versioned training
corpus from this repository's actual algorithms. Preserve the inputs, choices,
intermediate algebra, failures, certificates, counters, and source needed to
replay each computation. The consumer is `ml-cryptanalysis`; the generator and
cryptographic verification remain Rust. A successful invocation is not evidence
that every requested method or parameter cell was solved.

The full requested scope is Koblitz, general binary, and prime-field curves,
their twists, decomposition arity **m=2 through 15**, field degrees/bit sizes
**n=23 through 31 and 47 through 61**, and the explicit prime **p=2^61-1**.
Record n, m, coefficient field, subgroup order r, and polynomial variable count
as separate fields. Different generators and keys are mandatory. Requested
methods include factor-base development, symmetry, Frobenius orbits, double
large primes, F4/F5, this repository's F6 implementations, root finding, SAT,
Weil descent, and Petit–Kosters–Messeng (PKM).

Acceptance requires a working CLI, deterministic plans, all planned cells
accounted for, independent solution checks, replay/tamper tests, explicit
coverage, immutable S3 round trips, resumability without duplicate computation,
and a real launched generation workflow. An enormous completed corpus is a
separate acceptance item from a working pilot or a manifest of planned tasks.
No manufactured `solved`, instruction-count, first-fall, or last-fall labels.

## What a model can learn

| Task | Inputs visible at decision time | Supervision and check |
|---|---|---|
| Factor-base proposal | curve, field representation, public candidate basis/point set, budget | verified membership, orbit closure, independent relation yield/rank, full construction cost |
| Symmetry discovery | polynomial/point graph with permutation and coordinate augmentations | explicit action, stabilizers, orbit sizes, equivariance checks and counterexamples |
| PDP routing | target, factor-base definition, polynomial incidence graph | success/censoring and measured cost of each actually attempted backend on the same instance |
| F4/F5 scheduling | current basis, critical pairs/signatures, matrix sparsity and prior events | next admissible pair/reducer/pivot, resulting rank/degree/cost; rejected choices included |
| Root solving | coefficient sequence, field, factorization state | verified factors and roots, multiplicities, completeness certificate where available |
| SAT heuristics | CNF/XOR graph, current trail, clause activity, restart state | decisions, propagation, conflicts, learned clauses, backjumps, proof-checked outcomes |
| Double-large-prime selection | partial-relation graph and actual group identities | cycles with signed elimination witnesses, rank contribution and total graph cost |
| Polynomial consequence proposal | pre-native source equation graph, explicit ring, active variables, fixed bindings and initial public instance | nonzero target polynomial and separately searched generator-multiple witness, admitted by direct identity replay; witness-dependent degree reduction stays restricted |
| Algebraic conjectures | exact systems and certified algebraic invariants | falsifiable formulas, independently checked witnesses/counterexamples, proof obligations |

The [prime-chain ML projection proposal](PRIME-CHAIN-ML-PROJECTION.md) specifies
a concrete lossless equation graph and causal decision-prefix view, including
exact wide integers and independent replay prerequisites. The static prime-chain
graph is now implemented in the native exporter, with current validation status
recorded in [remaining coverage](REMAINING-COVERAGE.md). The new
[prefix certificate implementation](PREFIX-REPLAY-CERTIFICATES.md) adds bounded
semantic replay and observed recursive branch examples; its validation is still
in progress. Intra-F4 decision views and trained models remain open obligations.

The [checked consequence contract](../../research/ml_dataset_algebraic_targets_20261030/CONTRACT.md)
adds a concrete algebraic learning target through export policy
`strict-structural-allowlist-v6`. Its eligible input files are `features.jsonl`
for static public problems, `prefix-features.jsonl` for observed branch contexts,
and `algebraic-features.jsonl` for pre-native polynomial contexts. The latter
uses task `prime-polynomial-consequence/v1`: the network sees an exact source
equation graph, ordinary ring with explicit empty quotient rules, original
variable map, fixed bindings and the initial public projection. Coefficients,
modulus and field bindings use decimal strings. All target rows for an identical
context share one feature identity; target choice, event/node IDs, costs and
native outcomes cannot change those input bytes.

Supervision in `restricted/algebraic-records.jsonl` supplies a target polynomial
g and explicit coefficients and monomials witnessing
`g = sum(c_i X^a_i F[j_i])`. A graph or sequence model can learn to propose
consequences, represent multiplier combinations, or prioritize candidates with
observed cancellation. Exact arithmetic must check every generated proposal.
This task supplies neither an optimal consequence policy nor counterfactual
solver costs: the retained targets were selected by the native computation,
then admitted by bounded certificate search and direct replay. Training reports
must preserve that selection bias. Learning a useful consequence also does not
establish a complete solver, a SAT heuristic improvement or an ECDLP recovery.

The implemented extraction is limited to already ideal-equality-certified
recursive prime native nodes, m=3..15 and p<2^32. The direct identity API supports
p<2^62, but wide-field arithmetic support alone does not widen native trace
coverage. Unsuccessful whole-node equality searches provide no individual
consequence examples in this increment. Nonzero verified rows receive feature
occurrences; zero rows and capped suffixes remain restricted, with no public
feature occurrence for those rows. Unknown checks cannot become negative
examples. A separate cumulative operation/row/payload budget preserves admitted
rows and allows the original branch channel to continue. See
[operations](OPERATIONS.md) for the exact accounting scope and limits.

This v6 implementation and its tests are authored; new-source runtime tests
have not yet executed after an initial manifest-macro compilation failure. The [status record](../../research/ml_dataset_algebraic_targets_20261030/STATUS.md)
tracks that gate separately from historical v4/v5 passes. The static and branch
identity rules are preserved, but their byte stability must be checked by the
new actual-CLI fixture. No corpus growth, learned advantage or new theorem is
claimed by source implementation alone.

Start with supervised cost/routing baselines, graph encoders, and sequence models
of solver events. Train branching/reduction policies only after deterministic
replay and action validity checks exist. A policy must still use an exact solver
and verifier. Predictions are proposals; a low predicted degree is not a
certificate, and a model-generated relation is accepted only after group replay.
RL rewards should use verified completion and the cost of all work, including
failed actions, feature extraction, inference and fallback.

A scalar regressor is not the primary scientific target: small-key intervals,
known-log factor bases, and exposed generator transformations can produce easy
but misleading tasks. Keep such populations separately labeled as controls.
Mix natural/unconditioned targets and planted solvable PDPs under a frozen
sampling rule. Store selection/rejection probabilities and failed constructions;
never train only on successes without reporting the resulting selection bias.
The [planted-positive PDP control contract](../../research/ml_dataset_planted_pdp_20261031/CONTRACT.md)
specifies a separate versioned population, restricted witness replay,
denominator accounting, causal feature exclusion, and source-level acceptance
gates. It is a design contract; native runtime support requires a later patch.

## Identity, twists, and sampling

1. Store modulus/irreducible polynomial, field basis, model coefficients, full
   group order and its evidence, subgroup order/factorization/cofactor, generator
   coordinates and order checks, target, and canonical encodings. Use the
   repository's EC1/UID and registered ICV1 names where available. No guessed
   identities or unregistered ICV1 slugs.
2. Twist pairs have independent point counts, subgroup selection, generators,
   targets and solver outcomes. Over odd characteristic use a checked nonsquare
   d and `(a*d^2,b*d^3)`; require the paired-order identity when both counts are
   certified. Over characteristic two use an Artin–Schreier twist with a
   trace-one shift of a. Toggling a by one is a nontrivial twist only for odd n.
   Even n must not silently get the same isomorphism class.
3. Do not treat squaring as an endomorphism on every binary model: coefficient
   conjugation may move to another curve. Record the minimal field of definition
   and the actual power of Frobenius preserving the model. Over F_p, p-power
   Frobenius on F_p-rational coordinates is trivial; it supplies no nontrivial
   augmentation. Proper subfields exist exactly at divisor degrees; record them.
4. Pick generators by nonzero multiplication of a certified prime-subgroup
   generator. Retain the multiplier as restricted provenance, not a feature.
   Generate keys over the declared whole subgroup unless an interval control
   explicitly requests otherwise. Sampling must be unbiased, with rejection
   sampling rather than modulo bias where applicable.
5. Group all keys/generators on one curve, both twists, known isomorphic models,
   linked isogenous models, and Frobenius-equivalent tasks in one split component.
   Train/validation/test assignment hashes that component with a fixed split
   seed. Additional holdouts exclude complete field sizes, field polynomials,
   curve families and encodings. Random row splits are inadmissible.

The 24 sizes × 3 families × 2 twist positions × 14 arities give **2,016
parameter cells**, before curve seeds, generators, keys, and backend variants.
For example 16 curve seeds × 8 generators × 64 keys would request **16,515,072
tasks per backend configuration**. This is planning arithmetic, not generated
data or a throughput estimate. Preserve unsupported cells in the coverage map;
do not silently reduce the domain. A large manifest may stream rather than
materialize every cross-product in memory.

## Scientific labels and fall-degree conventions

Maintain separate booleans/statuses for `instance_constructed`,
`known_answer_verified`, `pdp_witness_verified`, `solver_completed`,
`scalar_recovered_without_witness`, and `scalar_replay_verified`. Target creation
by Q=[k]G is not an independently solved ECDLP. A verified PDP alone is not an
ECDLP solve: relation collection, filtering, rank, logarithm linear algebra,
target descent, scalar recovery and verification must also finish.

Every system records its exact ring, field equations or Boolean quotient,
variable names/order, monomial order, homogenization, symmetry substitutions,
auxiliary variables and lifting map. Preserve original and transformed systems.
For chained Semaev equations, field roots alone can be spurious point relations;
lift signs/points and verify the group equation.

Do not identify the first degree drop in one algorithm's trace with the exact
first fall degree. Store that as an observed event, with input/output polynomials
and multiplier/reduction witnesses. An exact first-fall label needs a specified
definition and complete lower-degree exclusion certificates. The maximum degree
visited is an algorithm/ordering-dependent solving-degree observation, not last
fall degree. An exact last-fall label requires sufficient closure/span evidence
and a justification that later falls cannot occur. Otherwise use an observed
event or `unknown`, with the degree budget. A lower bound or bounded-search
interval requires its own valid exclusion evidence; a capped generator-multiple
search alone supplies neither.

For the v6 consequence task, `witnessed_degree_drop` means only that the largest
degree of a nonzero generator-multiple summand in the checked representation
exceeds the nonzero target degree. Zero generators do not contribute to that
maximum, and zero targets have null degree and no drop claim. This
representation-dependent cancellation does not show that the representation is
minimal. The bounded closure in Huang–Kosters–Yang–Yeo permits multiplying
already derived polynomials; checking one identity in original generators does
not establish that closure or exclude other derivations. Consequently exact
first/last fall degrees and minimal derivation degree stay null, and closure
completeness stays false in these records. A searched identity is also distinct
from a trace of native F4 operations.

Theoretical motivation includes [Huang–Kosters–Yang–Yeo,
On the last fall degree of zero-dimensional Weil descent systems](https://arxiv.org/abs/1505.02532)
and [On Generalized First Fall Degree Assumptions](https://eprint.iacr.org/2015/358).
The zero-dimensionality and field-equation hypotheses matter; finite experiments
cannot prove a universal first-fall assumption. A fully checked counterexample
can refute a precisely stated universal conjecture. Neural predictions require
independent algebraic checks or a proof, with Lean formalization for new theorems.

## Artifact graph and schema

Each immutable shard binds a frozen plan/protocol digest, source commit, binary
SHA-256, toolchain, feature flags, hardware/OS, sampling rule and seed derivation.
Use decimal/hex strings for large integers; never floating-point curve values.
Canonical JSON/JSONL is the interchange/control format; compact binary, sparse
matrix blocks, DIMACS/XOR, and compressed traces carry large objects. A Parquet
training view can be derived later without replacing the canonical artifacts.

`plan -> curve+twist -> generator -> target -> factor base -> encoding -> solver
events -> root/lift/relation -> relation graph/matrix -> recovery -> replay` is
an explicit content-addressed DAG. Each edge names input digests. Every artifact
has role, relative path, digest, size, encoding version, producer and completion
status. Preserve at least:

- curve/order evidence, full factor-base point sets and membership rules,
  ordered bases, orbit representatives/action maps, signs and stabilizers;
- original polynomials, Weil-descent matrices/bases, coefficient and monomial
  tables, symmetry maps, variable maps, and every generated CNF/XOR formula;
- F4 matrices and pivots, F5 signatures/criteria, F6-specific states,
  reductions, zero rows, syzygy/reduction provenance and extraction witnesses;
- root-factorization states and roots; SAT decisions, trail/reason edges,
  conflicts, learned/deleted clauses, restarts, models and proof files;
- attempted relations including rejects/duplicates, large-prime graph edges
  and cycle elimination, filtered matrices/ranks/nullspaces and recovered logs;
- exact argv, stdout/stderr, exit/signal/OOM/timeout, counters, checkpoints,
  verification and remote upload receipts.

An instrumented backend must report which events it actually exports. An
aggregate-statistics adapter cannot advertise full intermediate-artifact
coverage. Trace exhaustion is a recorded stop/failure, not silent dropping.
Chunk and upload incrementally. Include a lossless replay tier plus a derived
sampled training tier; the latter never replaces the former.

The manifest has independent coverage fields for curve construction, polynomial
encoding, backend execution, event tracing, PDP verification, end-to-end ECDLP,
and cost measurement. Status vocabulary includes `verified`, `partial`,
`unsupported`, `budget_exhausted`, `timeout`, `oom`, `invalid`, and `not_attempted`.
SAT timeout is `unknown`, never UNSAT. An UNSAT label must say whether it is
solver-reported or independently proof-checked.

## Measurement contract

The current native F4 row-reduction multiplication counter is process-wide by
default (`src/cryptanalysis/f4_fp.rs::FIELD_OPS`); each report takes a difference
over its call. Concurrent calls can include one another's work. Preserve counter
mode and process concurrency in restricted measurement provenance, and require
one active solve per process or independently verified attribution before using
these deltas as per-instance cost labels. Nested/inclusive counters cannot be
summed into exclusive phase totals. These are counted row-reduction operations,
not complete field-operation totals or retired CPU instructions.

Use a typed counter vector: group additions/doublings, field add/mul/square/inv,
polynomial operations, matrix rows/columns/nonzeros/rank/XOR-word operations,
SAT decisions/propagations/conflicts, root tests, relation attempts/accepts,
large-prime graph operations, bytes read/written, peak memory and trace bytes.
Missing counts are null with reasons; never derive an instruction count by
renaming a field-op or XOR counter.

Retired hardware instructions require an admitted hardware counter with event
name, scope, privilege mode, enabled/running time, multiplexing and thread/process
coverage. Valgrind/Callgrind Ir is a distinct simulated instruction metric with
its own tool/version/ISA. Instrumented and uninstrumented runs are separate arms.
Cost labels exclude neither failed attempts nor verification. Keep setup, target
construction, target-dependent solve, independent replay, tracing and upload
separate, with explicitly stated intervals. Cold and reused-setup costs both
remain available, with the exact reuse count and amortization formula.

Throughput generation can run on shared Spot hosts. Its wall clocks are
diagnostic; they do not establish comparative performance. Any performance
comparison uses matched inputs, reference, A/A and controls under the repository
isolation rules. Do not rank unlike native counters or claim faster-than-rho
from PDP-only data. A solver-cost policy is evaluated including model inference.

## Durable CLI and execution lifecycle

Intended commands: `crypto dataset plan`, `generate`, `verify`, `status`,
`publish`, and a campaign/shard interface. `--help` must expose the actual
implemented surface and reject unknown or invalid parameters.

Plans freeze family/size/m/seed axes, twist policy, population, solvers,
factor-base policy, per-cell work/degree/time/memory/trace caps, split scheme,
and output budgets. A worker takes a deterministic disjoint shard. Acquire an
exclusive local lease before execution; an interrupted lease requires explicit
stale-owner reconciliation. Every cell writes an attempt record before work,
then durable artifacts and an atomic completion receipt. Resume verifies all
existing digests, configuration/source identity and receipts before skipping
completed work. Failed attempts remain, and retries have new attempt IDs.

Publish with the existing `crypto_lib::store` content-addressed S3 backend.
Upload complete chunks first, manifests second, a completion receipt last. Keep
per-worker indexes to avoid concurrent index updates; consolidate verified
indexes into Git. Remote existence is not enough: perform a download-and-hash
round trip on each new shard before treating it as durable. No remote deletion,
name repointing or overwrite. `store/index.json` and small checkpoint/status
records go to Git; raw multi-gigabyte traces go to S3. Checkpoint after every
completed bounded cell/chunk and at a fixed maximum interval during long calls.
Report any backend that cannot checkpoint mid-call as such.

Use the repository's current EKS Spot lab runner for cloud workers, with an
immutable source snapshot, digest-pinned image, finite TTL and resource limits.
Application manifests support completed-cell recovery; the ordinary throughput
Job does not provide CRIU recovery or timed-comparison isolation. Launch a bounded correctness shard
first, verify it, then expand a frozen plan in successive finite waves. The
full requested cross-product stays on the campaign even when a wave covers
only part of it. No unbounded spending or claim that queued tasks are running.
Git checkpoints occur after the design, tested implementation, each verified
generation wave, and reports; S3 chunk receipts occur during generation.

## Validation and staged delivery

1. Commit this design before implementing the generator. Map exact native APIs
   and implement a capability registry with honest limits.
2. Implement CLI, deterministic synthetic instances, persisted artifacts,
   independent scalar/PDP checks, and plan/shard/receipt validation. Add meaningful
   wrong-scalar, wrong-twist, corrupt-artifact, overlap, replay and interrupted
   publication tests.
3. Integrate actual solver adapters and record the exact algebra/counters
   available. Instrument missing event streams in separate reviewed increments.
   Keep every unsupported parameter/method visible until implemented.
4. Freeze and commit a bounded pilot protocol before taking data. Run the
   protocol, verify every artifact, round-trip S3, commit receipts and generate
   a source-linked report/diagram/PDF. Do not infer FFD/LFD or speed claims.
5. Launch bounded durable generation waves, check per-cell coverage and failure
   distributions, add native consumer/export integration and independent replay.
6. Train only after split/leakage checks pass. Compare to random/constant,
   classical heuristics, feature ablations, label-permutation nulls and a planted
   positive control. Freeze a held-out benchmark before tuning policies.

The source scan must distinguish the repository's F6 geometric closure from
its degree-filtered algebra engines; neither name alone implies a canonical
general-purpose Gröbner algorithm. Likewise a PKM audit score is not a complete
PKM descent/solve. Those distinctions are part of the capability registry and
must survive into training labels and user-facing reports.
