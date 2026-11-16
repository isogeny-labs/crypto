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

## Amendment 1: fixture oracle and bounded sparse follow-up, 2026-10-23

Frozen before generating fresh fixtures or running any guessing branch. The
initial structural panel remains governed by the original protocol and binary.
The native follow-up adds a separate exact fixture oracle: invert the binary
linear map y -> y^2+y, solve each S3 quadratic including linear/degenerate
cases, and join the two ends of a three- or four-summand chain. Full-field
quadratic enumeration at n=5,6,7 and the legacy exhaustive chain counter at
n=5 are its correctness controls. This oracle is enumeration for validation;
no IC improvement or end-to-end speedup is inferred from its runtime.

Replay all seven original fixtures independently, including explicit SAT
witnesses evaluated in the Boolean chain. SAT closure controls use degree
three with a 30,000-column cap; refusals remain recorded.

Fresh cells (m,n,ell): (3,25,9), (4,17,6), (4,18,6), (3,34,8), (3,36,8),
(4,38,6). The last has 100 variables and currently exceeds the Boolean
engine's 64-variable layout; fixture counting is supported independently.
For every cell generate 32 draws in each of prefix, geometric, geomtraceu,
random, seed 20261023. Prefix base duplication is retained and identified.
Record every draw; select the first four UNSAT and first three SAT per arm,
without replacing missing classes, for the closure/guessing follow-up.
Commit exact fixture bytes and hashes before closure measurement.

The exact B3 continuations from the HKY study are additionally (6,10,3),
N=58, and (5,15,3), N=60, not replacements by m=3 cells of equal N. Add
(10,10,2), N=100, to increase m at fixed field size. For m>4 the fixture
oracle propagates exact multiplicities along the quadratic chain rather
than enumerating every intermediate tuple. Cross-check it against the
legacy exhaustive oracle at m=5,6 as well before use. Generate the same
32 draws per family. A Boolean layout refusal does not discharge B3.

The declared alternative guess orderings are ascending occurring variable
index (`lowest:k`) and decreasing monomial frequency, ties by variable index
(`frequent:k`). Original balanced ordering is unchanged. Optional sparse
closure is a separate instrument arm capped at 25,000,000 stored nonzeros,
the same 12 GiB address-space and 900 s command limits. A sparse cap or timeout
does not settle a degree or guessing minimum. Begin it on the first UNSAT
draw at each larger cell. Do not retry a capped branch to change its outcome.

## Source update and extraction disclosure, 2026-10-23

Before this campaign's guessing data, the current PR #2429 source was
rechecked. Its `dreg_closure_guess3_20261023/RESULTS.md` already reports
balanced k=3 failing on draws 0,3,5 at branch 0 and passing all eight branches
on draw 6. This independent campaign does not claim priority for that result.
The protocol's full 32-branch panel additionally measures the 21 branches
that source early-stopped; the alternative orderings remain separate arms.

The initial structural panel's closure matrix cap is degree four. Inspection
of extraction telemetry while that panel was running found that the library
solver sets its effective degree to max(requested degree,input degree),
including direct S4 inputs of degree six. Thus its extraction is not a
strict-degree-four comparison. Preserve those records, report this protocol
deviation, and use them for exact correctness and explicitly scoped solver
work only. Do not claim a same-degree or full-query speedup from that panel.
Future rows record input and effective extraction degrees explicitly.

Quartic smoke spec `specs/quartic-smoke.json` uses target seed 202610230001,
order seed 202610230002, 3,600 s per measured child and L0 counted evidence.
The full rungs `specs/quartic-m14.json` through `quartic-m18.json` retain
eight targets and the original 86,400 s child cap; they are pending, not
automatically launched by the local runner. Local smoke runs are serial,
with no other measurement process active.

## Amendment 2: 128-variable masks and the HKY continuation

Frozen before any new fixture or HKY measurement. Add a Rust coefficient-
based S3 chain builder with u128 Boolean masks (at most 127 variables),
verified equation by equation against the library builder for m=3..6.
Carry the source-pinned PR #2214 parallel HKY engine into the instrument,
widening monomial masks to u128 while retaining u64 matrix words. Verify
small fixed points against the library's independent dense closure.
Source: `examples/hky_par.rs` at f0196738eeb033b23719fb79f76d266e3a8d0c16,
SHA-256 4bc7e43aa56fa446a2785c30602d2823f3aed7182a772b7120cfdeae5c30d8b2.

Add a separate bounded u128 sparse engine. Its 25,000,000-entry cap counts
stored pivot and queued-fall monomials; its counter unit is mask comparisons,
not the dense engine's u64 word operations. Compare small sparse fixed points
against the dense engine and include a contradiction using variable 99.
No cross-engine speed ratio is licensed. The limits remain 12 GiB address
space and 900 s per command; use RAYON_NUM_THREADS=4 and HKY_CHUNK=256 for
parallel runs. The first UNSAT random draw in each B3 cell is the local
preflight; complete four-draw large-host panels remain required if these
preflights cannot finish. Dense preflight permits 600,000 columns for
N=58,60; N=100 uses sparse closure. Retain degree-start receipts on failure.

## Amendment 3: internal-cap correction and degree-two/three ablations

Before any scored guessing result, the first eight k=3 calls returned
`engine_cap` immediately. The library's default 40,000-column ceiling was
still active beneath the frozen 140,000-column instrument limit. Preserve
these as unmeasured cap refusals, not degree observations. For subsequent
calls explicitly set F4_F2_MAX_ROWS=50000000 and F4_F2_MAX_COLS=1000000;
the instrument's stricter column, address-space and timeout caps remain.

The initial panel's degree-four preprocessing is more work than necessary
on small inputs. Before any lower-degree ablation data, register the identical
96-command structural panel at closure degrees two and three as separate
arms. Seeds, exact targets, families, replicates and all other limits are
unchanged. Extraction uses the now-explicit max(closure degree,input degree)
library rule. Preserve every branch, nonsplitting root and incomplete query.
Audit each panel against independent point enumeration. These ablations test
whether cheaper closure can finish the same queries; all end-to-end costs
remain unset until the full pipeline and calibration obligations are met.

## Amendment 4: intermediate guesses and complete-pipeline smoke

Frozen before these arms' data. Add three-variable sets [7,15,24] and
[7,23,24] at the original ell=8 fixtures, every assignment on each of
draws 0,3,5,6. They separate guessing an intermediate from merely reducing
each summand's dimension. [15,23,24] is the image of [7,23,24] under exchange
of the first two summands; certify that exchange on the actual equations
before deriving a degree-verdict equivalence. Never infer equal costs under
a variable permutation.

Integrate the geometric-base and symmetric solver into the ecbench binary
as new versioned plug-ins, retaining every existing method. Complete-DLP
smoke: K_0 over n=7,13,17,23, respectively ell=2,3,3,4; base family geometric,
base seed 17. Two public known-answer targets per field, target seed
202610230101 and interleaving seed 202610230102. Arms: strongest signed-
Frobenius rho, direct S4 hybrid without preclosure, symmetric S4 hybrid with
degree-two preclosure, complete negation-folded pair-table IC on the same
base, repeated strongest rho. Hybrid degree=2 with the explicit input-degree
floor, node_budget=20000, max_solutions=4096, max_cols=30000. Every search
parameter is required by the plug-in. max_trials=100000; 900 s per child;
one round, no warmup, L0 counted evidence. Each field has its own spec.

The complete runner charges setup, actual queries, failed attempts, relation
verification, filtering, relation LA and scalar recovery as the existing
framework does. Signed lifts use CountedGroup, and all solver word operations
are retained in SolverTotals. Symbolic construction and cubic splitting are
not completely counted in native units: S is a lower bound, not a full-price
cost. Keep elapsed phases diagnostic at L0, retain every incompletion, replay
every successful run, and report the common native-work sensitivity band
0,0.1,1 GAE. This smoke does not establish a scaling win, fastest-oracle
claim, or net gain over the best verified IC pipeline. Full calibrated and
external-tool comparisons remain C3/D1 obligations.

Implementation correction before pipeline data: use the generic
`mitm:m=3,negation_folded=1` pair-table control. Inspection showed that the
`pdp3-koblitz` adapter explicitly requires its standard prefix base and
refuses an arbitrary geometric base. Both use the existing negation-folded
pair-table algorithm; no existing oracle or its counting is changed.

## Budget authorization, 2026-10-25

The owner authorized up to USD 100 total additional cloud compute for this
campaign. This is a total ceiling across jobs, replay and supporting cloud
resources, not a per-job allowance. No paid job has been launched. Launcher
credentials and an allocation still need to be verified. Prioritize the
next discriminating cells within that ceiling; keep any unfunded inventory
open and never describe prepared cards as queued.

## Complete-pipeline pricing disclosure, 2026-10-25

The initial pipeline spec's assumption that raw S would leave solver words
unpriced was incorrect for this framework version: the native table uses a
legacy wall-priced solver component. Preserve the raw sessions and this
deviation. Do not interpret raw S or the `lower_bound` flag as a complete
counted cost. Canonical native-work bands reconstruct the recorded parts,
but construction and splitting remain incomplete. No whole-pipeline win,
rho crossover or ledger promotion is licensed by these smoke records.
The n=7 cell exhausted, n=17 was refused by the K_0 constructor, and n=13
completed with exact replays. The n=23 cell was still running at this
disclosure. Missing cells and references remain missing.

## Amendment 5: finite diagnosis of the failed n=7 smoke (2026-10-25)

After the frozen n=7 panel exhausted every arm, independently enumerate its
four seeded factor-base points and every ordered three-point sum including
repetitions, and test prime-subgroup membership. Enumerate all 29 subgroup
elements, their strong-rho canonical forms and the literal source-pinned
four-bit distinguished-point predicate. This is a post-outcome diagnosis,
not a new timing comparison, parameter selection or replacement of the
failed panel. Preserve its complete JSON, exact source hashes and report.

## Amendment 6: correction of the earlier pricing explanation (2026-10-25)

The prior disclosure that completed n=13/n=23 raw S includes solver wall time
was incorrect. Their raw records explicitly say priced_by=pinned; integer
word counts use fixed GAE rates. The n=7 measured solver estimate is removed
by ic_report. The presence of SolverCost.wall_ns does not establish a wall
charge. The earlier protocol text and PDFs remain historical evidence of
that mistake, superseded by this amendment and the corrected reports.

PRICING_AUDIT_PROTOCOL.md and native ic_campaign_pricing_audit independently
rebuild available IC costs from group/native counts and named fixed prices.
The audit also reconstructs the quartic adapter's deterministic virtual
F_q-operation conversions. The ordinary band tool refuses the new quartic
curves because those virtual rows are absent from its global table; preserve
that refusal. A compatibility reconstruction and an additional all-native
solver scenario are labeled separately; neither changes old records or
represents a new algorithmic gain. Uncounted work remains missing, and no
complete-price promotion follows. Experiment parameters and the original
requirement register are unchanged.
