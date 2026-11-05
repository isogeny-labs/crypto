# Uniform useful S3 yield and conditional collection laws

2026-10-29 · existing PR2381 · [inspected three-page PDF](report.pdf) ·
[all-arm graph](profiles.svg) · [recorded plot data](plot-data.json) ·
[flow diagram](bridge.svg) · [native records](../../native/uniform-20261029/) ·
[raw restore/validation](../../validation/uniform-20261029/README.md) ·
[new Lean module](../../lean-uniform/UniformCollection.lean).

The frozen full workload is now **executed and independently replayed**:
96 arms, 896 separate nonidentity main-source scalar recoveries, 210 canonical
policy comparisons, 536 reachable coefficient spaces and 172 failed recovery
queries. All planned size-two/three bases and transported/inverse controls remain.
The strict primary criterion, **larger mean initial useful rank gain AND lower
mean named matrix degree**, holds in **0/210 original-input and 0/210 reduced-input
comparisons**. This is a finite result on the declared fixtures and iid uniform
nonidentity-with-replacement sampler, not a universal no-advantage theorem.

The exact result does not assert calibrated work, intrinsic regularity, F4/F5
speed, a benchmark/leaderboard promotion, general hardness or GHS. Setup, mapping,
root/sign lifting, matrix work and storage are not priced by a degree alone.

## Exact objects and observed profiles

F43: E `icv1-fp6-t0-74779950`, y²=x³+x, P=(31,18), r11; T
`icv1-fp6-t0-c6a8a47c`, y²=x³+4x, U=(24,26). Actual phi=(41x,11y)
is a base-field isomorphism. No new EC1 alias is assigned to U.

F13: E `icv1-fp4-t0-6bee67e0`, y²=x³+x+4; H
`icv1-fp4-t0-bb514d71`, y²=x³+4x+7; T
`icv1-fp4-t0-097da6b6`, y²=x³+4x+6, the d2 monic twist of E.
The historical degree-two coordinate construction E→H followed by scaling5
H→T gives Phi=(12(x+2/(x−10)),8y(1−2/(x−10)²)). Infinity/kernel(10,0)
map to infinity; P=(0,2)→U=(8,11), r7. The prior actual finite Lean scalar
bridge is retained; no generic isogeny classification is inferred. These exact
identity references are separate from the still-open active registry repair.

Each query uses the original/reduced constrained S3 solver with complete input
witnesses, ordered roots/sign lifts, point checks and coefficient rows modulo r.
The independent Rust implementation shares no C++ field/group arithmetic;
it replays the incremental solver and a separate semantic witness/row-span
check, then reconstructs states breadth-first and solves exact rational
recurrences in descending rank. C++ uses recursive GMP rationals. Query outcome
multiplicity is retained. Base logs are derived from public relation equations;
recovery receives Q alone, never its scalar label. All896 recovered scalars verify.

The table copies literal audited native fractions for size-two policy0:

| Field/arm | Positive-rank probability | Mean initial rank gain | Mean reduced degree | Call recurrence | Reduced degree-request recurrence |
| --- | ---: | ---: | ---: | ---: | ---: |
| F43 main0 | 4/5 | 4/5 | 3 | 35/12 | 35/4 |
| F43 twist0 | 3/5 | 4/5 | 14/5 | 10/3 | 28/3 |
| F13 main0 | 1 | 4/3 | 7/3 | 2 | 14/3 |
| F13 twist0 | 1 | 4/3 | 8/3 | 2 | 16/3 |

Original mean degree is4 in all four rows. F43 main0 uses (4,5),(13,19),
twist0 (15,9),(17,6); F13 main0 (0,2),(7,4), twist0 (6,5),(8,2).
Thus the first F43 twist policy lowers mean per-query reduced degree while
its collection-call and degree-request recurrences increase. These are named
degree-request units, not time or total-cost units. For size-three bases, cubic
membership division leaves original S3 unchanged, exactly. The four panels
retain every arm; identical points overlap. All native Pareto policy sets and
losses remain in index.json. Matched row/rank/probability/call profiles agree;
coordinate-dependent degree profiles are observed rather than assumed equal.

## Failed attempts and full passing contract

The full experiment was planned once; three complete native-contract attempts
were launched, one completed, two failed, zero interrupted/resumed/excluded.
The failed infrastructure gate before the first scientific attempt is separate:
sparse hydration had dropped an unreserved checkpoint-script path, and no
scientific child was released. Reserving the unchanged official tracked path
restored the gate. Every actual experiment/proof launch had a successful gate.

1. [Attempt01](attempt-01/README.md), frozen `7c463e834d21`: producer completed;
   audit101 correctly rejected the size-three invariant because the old Rust
   division helper used only two base roots. No positive receipt/control pass.
   Original source and exact failure remain unchanged.
2. [Attempt02](attempt-02/README.md), frozen `42b72c3a2feb`: new versioned full-base
   monic division independently verifies the whole96/896/210 record. The strict
   runner exits2 because false-row changes an already empty row list to empty.
   No full control/backend completion is inferred from the positive audit.
3. Attempt03, frozen **`0e88deaae36ac48eee24baaac21ef02bbc979c4d`**: versioned v3
   removes one actual nonempty row and uses the identical equality predicates
   with shorter diagnostics. Producer/audit0, **13 genuinely altered rehashed
   controls reject101 with no receipt**, **22 backend tests pass**, runner0.
   Missing-failed-query is applicable, not null. All97 producer case files match
   both earlier attempts byte for byte; the two positive audit receipts match.

Both corrections were compiled, committed and pushed before their new full
execution. Old versions, all criteria and every altered payload remain. No
polynomial, subgroup, state, witness or source check was weakened. The native
600sec caps were not hit. Exact command/input/exit/source evidence is restored
by the raw archive; build binaries and caches are excluded with binary hashes
retained. The export is a schema-checked post-run `experiment.export/v1` bound
to the committed Markdown protocol, **not a retroactively preregistered JSON
protocol or a canonical protocol-checker approval**.

## Checked theorem and remaining bridge

New [UniformCollection](../../lean-uniform/README.md) proves nine named theorems;
the unmodified repository scoped proof gate passes16 declarations, nine named
plus seven generated helpers, zero failing/extra trusted. Only propext,
Classical.choice and Quot.sound occur. Lean4.34.1/Mathlib
d13f23b723b8a846827a245b89c10fc7d3f11612. The old208 named/269 audited receipts
are historical; no fresh combined gate is claimed. All17 prior IDC1 records
stay unchanged; no new standalone curve-polynomial identity is claimed here.

Given a positive outcome count N, stays<N, strictly descending improving
successors **with multiplicity**, and explicit terminal/first-step equations,
Lean proves recurrence uniqueness, scaling and C=(degreeMass/N)E. In the
one-rank-remaining case E=1/p. For positive main calls, nonnegative lower twist
degree and **no larger twist calls**, it proves strictly lower degree-weighted
demand. The call-count hypothesis is explicit: a larger empty-state mean rank
gain cannot silently supply it.

The binding to actual536 native subspaces and formal iid stopping-time,
absorption/integrability and first-step probability semantics remain open.
The first actual elaboration failure/source are retained separately from the
successful module and gate. A conditional scalar recurrence is not presented
as that missing probability theorem or as a universally formalized native solver.

## Sources, validation and next discriminating step

Checked sources are the [preregistered contract](../../UNIFORM_YIELD_20261025_PROTOCOL.md),
the two pre-rerun corrections, [actual S3 record](../s3-yield-20261024/README.md),
[F13 point/map record](../cm8-transfer-20261025/README.md), pinned Mathlib and the
unmodified proof-gate. Atlas matching at crypto-autoresearcher5998553341fb29d3b9ca1a4c9ac6e58d96ca3ffa
reuses transported-relation-yield, relation-graph rank and Semaev-yield/coset leads;
no invention claim. Existing canonical scoreboard/curve graphs and old frozen
report figures were checked for scope: this exact finite null/conditional result
changes no performance row, model/cover input or prior figure; all remain untouched.

The [validation README](../../validation/uniform-20261029/README.md) records the
fresh root-library outcome and every limit. Native/Lean success does not imply
that suite or exact-head CI is green. Registration still needs reconciliation of
1921 study models with expanded main inputs; strict assertions, both catalog
provenance dependencies, all byte/membership/partition checks and prior claims
stay. Conductor exits127/no reservation or substitute coordinator claimed.

The next discriminating proof is a faithful actual-state/iid expectation bridge;
the next encoding lead is a preregistered higher-arity constrained system with
actual exceptional lifting and subgroup transport, rather than another favorable
two-summand target prefix. The explicit F13 dual remains separately queued.
None of these next steps is running now. Intrinsic regularity, complete calibrated
work, broader geometry, coordination, review and integration remain unresolved.
