# Skipped NIST candidates: upstream traits and independent algebra

This is a prospective rerun protocol. The motivating external investigation
already inspected these inputs; this is not a preregistration of unseen data.
The delivery is a set of jobs. Their execution and historical interpretation
remain separate from this planning PR.

## Frozen panel

Use the six entries of `curves.json` verbatim. They are the P-224 generation
run's blocks 777, 1485 and 5736, the P-256 run's block 876, and published
P-224/P-256 controls. `provenance.json` records the timestamps, seeds, root
conventions and source revision. The block indices are one based. These
stream coordinates identify the selected candidates; the legacy `name`
strings in the imported input are fixture keys, not new catalogue names.

The four skipped inputs contain no generator and no precomputed `properties`.
Do not insert an auxiliary generator into the DiSSECT input. Published
controls use their standard generators. PARI may construct an auxiliary
point solely to verify an order or map; record it as such. An original
generator for an unpublished curve remains unknown.

## Jobs and boundaries

`jobs.json` freezes 22 upstream trait jobs and one independent algebra job.
All trait jobs use all six curves and the unmodified default parameter grids
of DiSSECT commit `8edc3030fb91411a55aa6c07d050507d840c5e55` under SageMath 9.8.
The expected total is 438 curve/trait/parameter records. No trait is replaced
with the repository's partial native `curve_traits` implementation. The
user explicitly requested the actual external DiSSECT tool; only that
unchanged tool uses Python. New orchestration is shell, and the independent
mathematics uses native PARI/GP.

Each shard has one CPU, 4 GB RAM, no GPU and a 3,600 second process limit,
followed by at most 10 seconds for termination. PARI individual
factorizations have an additional 120 second limit. Upstream internal
timeouts remain unchanged. No shard is a performance benchmark; elapsed
times or heterogeneous tool runtimes must not be promoted to speed claims.

Each attempt gets a fresh directory. Preserve stdout, stderr, raw JSON,
inputs, exact command, runtime version, source revision and exit status,
including exit 124 (timeout), crashes, partial outputs and internal timeout
messages. Do not overwrite an incomplete attempt or silently retry it as a
success. These first jobs are deliberately not resumable; a failed shard is
rerun in a new directory. A resumable implementation and a merged code pin
are required before promoting the card from `draft` to `ready`.

## Independent algebra job

`algebra.gp` independently proves primality of each field and main order,
counts the curve, its true quadratic twist and the negative-b model with
SEA, and checks the twist-order identity. Factor the twist order and
Frobenius discriminant with proof enabled, check every prime factor and the
products, derive the CM discriminant and Frobenius conductor, and compute
the main embedding degree from a checked factorization of `n-1`.

For every prime degree from 2 through 31, record the roots of the classical
modular polynomial at the input j-invariant. This is a bounded small-degree
neighbor calculation; it is not a complete isogeny-class enumeration.
Construct an explicit 3-isogeny from a rational kernel polynomial. Verify
its normalized rational-map polynomial identity, the codomain order and
transport of a nonzero auxiliary point of the main prime order. Record
kernel, codomain coefficients and the transported point. Constructing this
map establishes no scalar-recovery advantage.

For this panel, `p = 1 (mod 3)`, `t = 1 (mod 3)` and the repeated Frobenius
eigenvalue on 3-torsion is -1. Factoring the 3-division polynomial determines
whether Frobenius is scalar (all four x-coordinates rational) or has a
Jordan block (one rational x-coordinate). The full 3-torsion extension
degree is respectively 2 or 6, with least nontrivial degree 2. The job checks
these fixture hypotheses and the necessary cardinality divisibility.

Compare only pairs with identical characteristic: six pairs in the P-224
field and one in the P-256 field. Record equality of j, equality of traces,
equality of traces up to sign, and whether the product of their negative
ordinary Frobenius discriminants is a square. Distinct j excludes a
geometric isomorphism and hence twisting; distinct traces exclude any
base-field isogeny. Distinct ordinary CM fields exclude a geometric
isogeny. Failure of these exclusions is a lead, not a constructed route.

## Audit and acceptance

`run.sh verify OUTPUT` checks artifact hashes, curve/parameter coverage and
the independent algebra job's completion/invariant flags. It does not
certify every upstream numerical result. Never parse the upstream large
decimal integers through floating-point arithmetic. The native algebra
output deliberately writes large integers as decimal strings.

Before reporting the trait batch as mathematically audited, reconcile the
relevant raw values with the independent job. Retain these known upstream
defects visibly, without patching or rewriting raw output:

1. `torsion_extension.py` assigns `k2 = l` instead of `k2 *= l` in its
   non-scalar repeated-eigenvalue branch. Independently check `least`,
   `full`, `relative` and the point-count recurrence modulo the torsion
   prime. Do not use the affected raw full/relative degrees as invariants.
2. `trace_factorization.py` factors the extension trace but returns the
   base trace in its `trace` field. Compare degree 2 with `t^2 - 2p`.

Keep all generator-dependent nulls. `class_number` supplies bounds, not
an exact class number. A trait call returning is not evidence that its
factorization finished or that all its fields are correct. Count internal
timeouts, unavailable values and discrepancies separately from records.

Completion of execution requires all 23 shard artifacts and all 438 trait
records, with failed or timed-out cases explicitly unresolved. Completion
of the scientific audit additionally requires the independent checks and
a per-field discrepancy table, reviewed against raw values. A null result
is admissible. A proposed historical explanation must distinguish a curve
property from evidence about the original generator's behavior. Neither a
matching feature nor the absence of familiar weaknesses proves why a
candidate was skipped.

## Landing results

Land subsequent immutable outputs under
`research/nist_skipped_curve_dissection_20261021/` in a follow-up PR. Include
a source-linked report, explanatory or quantitative graph, PDF, and a
requirement-to-evidence table under the research-visuals rules. Pack the raw
run tree as a deterministic archive with its hash under AGENTS.md section
13a unless verifiers require loose files. Register ICV1/EC1 identities
before using new curve names in result comparisons. Do not add leaderboard
or benchmark rows for this structural audit. No existing result graph is
changed by this job-definition PR.
