# Algebraic decomposition and characteristic-two quartic IC: pursuit protocol

Frozen before new research data, 2026-10-23. This campaign implements the
owner's request to pursue both selected directions exhaustively. It preserves
three distinct obligations: combine structured factor bases with fall closure;
measure the guessing ladder and complete query cost; and test the characteristic-
two quartic extension-field crossover with complete verified DLPs. Smaller
diagnostics do not discharge larger cells or complete-solve obligations.

## Sources and claims

Read before selecting the instruments:

- `crypto` PR #2214, `research/fall_degree_bounds_20261008/RESULTS.md`:
  last fall degree observations, including satisfiable controls, are distinct
  from a complete decomposition algorithm and from its cost.
- `crypto` PR #2429, `research/dreg_closure_guess_20261022/RESULTS.md`:
  all 16 branches of one four-variable guess set refute at degree four on
  four certified UNSAT inputs at (summands,n,ell)=(3,22,8). Four is an upper
  bound, and the unguessed degree-four system at ell=8 was not measured.
- `cryptanalysis/experiments/pdp-degree-heuristics/{README.md,sym.py,
  factor_base.py}`: product-space bases, symmetric relaxation, mandatory
  splitting, and the corrected XOR sampler for trace-zero progressions.
- `research/gaudry_quartic_d4_char2_20261015/{README.md,PROTOCOL.md}`:
  measured m=14 extension-field row; crossover near 2^68 is an extrapolation.
- Huang--Kosters--Yang--Yeo, arXiv:1505.02532; Kosters--Yeo,
  arXiv:1503.08001. A first fall is not a solving-degree certificate, and
  extension-degree-independent results for zero-dimensional systems do not
  establish bounded degree for summation-polynomial systems.

New claims are exact finite computations or stage diagnostics unless a complete
paired, verified ecbench session and the repository's validation gate support a
pipeline comparison. No missing query, branch, root, split, lift, failed attempt,
linear algebra or setup cost is assigned zero.

## Requirement register and order

| ID | obligation | execution and acceptance |
|---|---|---|
| A1 | independent replay of the selected structural observations | rebuild source-pinned inputs; verify equations and exact small-fixture roots; replay deterministic counts separately from time |
| A2 | native structured-base/symmetric implementation | Rust product spaces for prefix, geometric, trace-zero geometric and random bases; test product-space rank, full S4 identity and splitting against direct enumeration |
| A3 | combine formulations with fall closure | paired targets and bases; chain, eliminated S4 and symmetric e-space; keep relaxed roots and complete point decompositions separate |
| B1 | minimum guesses at ell=8 | all branches for k=0,1,2,3,4, on all four original UNSAT draws; original balanced variable ordering and declared alternatives; capped cells remain unknown |
| B2 | ell=9 and corresponding four-summand cells | fresh fixed-seed inputs, SAT/UNSAT controls, all branches and explicit memory limits; no inference of a minimum from one passing guess set |
| B3 | larger last-fall-degree cells | N=58,60 then approximately 100, larger summand count; controls and complete saturation required; sparse-engine development if dense storage is limiting |
| C1 | replay quartic IC and paired rho | native ecbench, same point and field; plain/weighted/Frobenius IC where feasible, strongest applicable rho, independent replay |
| C2 | push the crossover ladder | base-field m=14,15,16,17,18; eight independent one-target workloads per supported rung; wider arithmetic before unsupported rungs; do not substitute fitted crossover for a measured solve |
| C3 | stack improvements and price the complete method | complete query extraction, base construction, collection, verification, filtering, relation LA, descent and scalar recovery; price native counters at 0,0.1,1 GAE on every arm |
| D1 | external and hardware controls | same-input open CAS baseline, Magma when available, portable CPU reference and actual GPU checks before any GPU claim |
| D2 | durable delivery | source, protocol, raw rows, replay/validator receipts, report/diagram/PDF, requirement statuses, focused PR(s); unexecuted work remains open |

## Initial local structural panel

Native instrument: `examples/ic_algebraic_followup.rs` and its support module.
Fixed seed 20261023. Fields n=13,19,23,31, Koblitz a=0,b=1, exact polynomial
and prime subgroup recorded in each input. Dimensions ell=2,3,4; families
prefix, geometric, geomtraceu, random. Four independently generated public
known-answer subgroup targets per field, shared across bases. Seeds 0 and 1
are base replicates. Prefix is deterministic and is not counted as two
independent bases. Formulations: chained S3, eliminated S4, symmetric e-space.

Initial local caps: degree <=4, 30,000 Macaulay columns, 12 GiB per process,
900 seconds per command, one measurement process at a time. A support or
resource refusal is retained as a row. Correctness controls use n=5 and 7,
ell=1 and 2, enumerate every Boolean root and every unordered x-multiset,
and verify the symmetric correspondence and splitting exactly. A/A replay
uses the same source, input and options; all deterministic fields must agree.
No significance test or population-wide degree law is claimed from this
panel. Effect sizes are per matched target, with completion counts stated.

The symmetric equations are a relaxation: an e-solution counts as a
decomposition only when its monic cubic factors completely over the exact V,
including repeated roots, and its lifted signed points sum to the target.
The full root count, rejected roots, split tests, and point checks are recorded
when extraction runs; a refutation-only row leaves all complete-query costs null.

## Guessing experiment

Input is the byte-identical fixture from PR #2429; its source SHA and digest
are retained in `inputs/manifest.json`. Draw order 0,3,5,6. Balanced ordering
of guessed variables is [7,15,23,24,6,14,22,25]; k is its prefix. Additional
predeclared orderings are lowest-occurring and most-frequent (ties by index),
and each is a separate arm. Every assignment of the selected variables is
run; per-branch observations are written immediately and never overwritten.
SAT fixtures 1,2,4 are retained as controls, not UNSAT observations.

Degree is four. Column cap starts at 140,000 for k=3 and 130,000 for k>=4;
k=0,1,2 need a larger host if their actual occurring-variable count exceeds
the cap. Per-process memory is 12 GiB locally and command timeout is 900 s.
A saturated non-refuting branch disproves that guess set at the chosen degree;
an incomplete or capped branch is inconclusive. A passing set proves only an
upper bound. A minimum requires every smaller set in the declared family to
be measured, and is never a minimum over all possible variable subsets.

Historical preflight: the prior 124,314-column k=4 branches took 124--331 s
and peaked at 10.6 GB. The k=3 matrix has 136,698 columns, so memory may be
the limiting factor. The complete local four-draw k=3 panel has 32 branches;
its historical runtime envelope is 1--3 CPU hours. Preflight and any amendment
are retained separately; no timeout is retried as an attempt to change a result.

## Quartic study and infrastructure gates

Use `ecbench` and the existing `bin-t-line` / `gaudry-d4-bin` plugins. Initial
smoke uses base-field m=6,8,10,12, two independent targets per rung, one round,
no warmup, same-point negation rho and a repeated rho control. It earns only
counted evidence at L0 when this shared container cannot isolate. The full
panel uses eight targets and at least four sizes; wall-time claims require
the repository's isolation receipt and A/A gate.

Before larger rungs, audit actual scalar and coefficient widths. Unsupported
rungs get a typed refusal; never truncate field elements, group orders, scalars
or counters. The known implementation currently asserts base-field m<=15;
its extension-group order and subgroup scalars are u64. Widening those paths
and verifying them are C2 implementation work, not already-supported runs.

Remote experiments use the repository's ephemeral runner with explicit
instance type and TTL. No usable remote credential or paid allocation has been
verified in this session. Prepare exact jobs and resource estimates before
requesting missing access or budget. Do not describe jobs as queued until the
runner returns a job identifier. Local work continues independently.

## Evidence and closeout

All runs record source/input/binary/protocol hashes, exact argv, host,
limits, status, deterministic counters and diagnostic time separately.
Preserve every failed build, timeout, OOM and mismatch. No result is admitted
to the performance leaderboard without VALIDATION.json and all required
independent certificates. A bounded negative stays in this study.

Conductor is not installed in this workspace; no task reservation is claimed.
Review `docs/index-calculus-scoreboard.html`, the leaderboard, progress data
and browser when results land. Add a study panel for new structural findings;
retain end-to-end fields as null until their obligations are met.
