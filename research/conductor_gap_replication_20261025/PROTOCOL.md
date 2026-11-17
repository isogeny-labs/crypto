# Conductor-gap replication: frozen design v1

Date: 2026-10-25. This extends the bounded result in PR #2430. The parent
study's sealed data and claims remain historical evidence. This protocol is
committed before any primary solver run. Classification: controlled
measurement and accounting; a null result is a valid recorded outcome.

## Requirements and distinct panels

| Requested extension | Panel and acceptance evidence |
|---|---|
| Independent isogeny classes | A: D0 = -11, -19, -43, -67 at 32 and 34 field bits: eight distinct (p,t) classes; no pooled scaling fit |
| More volcanoes across the gap | A: three distinct descending N-kernels per class, with the same S1 generator; require disjoint S2 j sets |
| Stronger per-level sampling | A: depth four; target counts 192,64,32,16,8 per curve on levels 0..4, respectively: 192 one-target solves per level per volcano |
| An algorithm using CM structure | B: four fresh prime-order CM pairs, two each for D0=-19,-43; direct CM-map, scalar, and generic-recurrence orbit bases, matched rho |
| The hard transfer regime | B: Frobenius kernel-abscissa field degree at least (N-1)/8; no explicit cross-gap map is assumed or assigned zero cost |

Depth four and the smaller fields are an explicit bounded replication,
separate from the original depth-six, 52-bit class. A does not test the
hard-kernel-field regime: its twist-rational construction deliberately
retains the parent's hypotheses. B has no deep 2-volcano and no transported
cross-gap targets. Neither panel replaces the other or settles the general
KKM/Problem-B question. CM orbit collection is not a prime-field F4/SAT
complexity measurement; that backend remains an implementation gap.

## A: classes, construction, workload

Use volcano-gap construct with depth=4, max-cofactor=256, seed=2026102501,
each D0 in the table, and bits=32,34. Use kernel-seed=701,702,703 to draw
three separate descending lines while holding the S1 generator fixed.
Keep the first three distinct S2 j invariants, using subsequent kernel
seeds through 720 only to replace duplicate or horizontal kernels. Any
failure to obtain three stops that class and remains visible. Each accepted
gap must be prime and greater than p^(1/4); n must be prime and >4sqrt(p).

Export every vertex of the four depth-four 2-volcanoes, with two coordinate
models, using volcano-rho's verified maps. Require counts [1,3,6,12,24]
on each, the parent's side certificates on all models, and no overlap
between the three S2 vertex sets or with S1. Duplicate S1 exports must
agree in their generator. Construction, both point evaluations, dual checks,
and their native costs remain separate from solver costs.

Primary arms: rho.negation on model 0, independently solving every target.
Each level is a separate ecbench spec. target_kind=planted_shared,
target_seed=2026102501; measurement seed=2026102502, rounds=1, warmup=0,
order=shuffle, L0, per-execution watchdog=120 seconds. Same target indices
use the same scalar within one class. Every invocation owns its tables.
No batch distinguished-point work or shared-target speedup is allowed.

Expected primary count: 8 classes x 4 volcanoes x 5 levels x 192 = 30,720.

Controls per class:

- Select the crater and lowest-vertex-id floor curve from each volcano.
  On both coordinate models run 32 targets, with rho.negation (A/A) and
  rho.plain in the same shuffled session, seed=2026102503. Expected 1,024
  records/class. These control observations are not pooled into the primary.
- BSGS negation: two targets on every model-0 curve, seed=2026102502.
  Expected 368 records/class. Exact costs must agree per target across the
  transported instances, including both sides.

Primary unit: cold counted S=GAE/sqrt(n), retaining every unpriced native
counter. Report the 0/0.1/1 GAE sensitivity band alike on each side;
unknown prices remain unknown. Wall/CPU values are descriptive L0 only.

## A: inference frozen before outcomes

Compute S2/S1 separately for each class and level, averaging the three S2
volcano means before dividing by S1. Primary pooled estimate is the
geometric mean of the eight class ratios, with equal class weights, and is
reported separately at each level. The overall secondary estimate weights
the five levels equally. It does not weight by vertex population.

Use 10,000 whole-class bootstrap replicates, native StdRng seed=2026102504;
retain all three S2 volcanoes and the common S1 in each selected class.
Report 90% and 95% percentile intervals, all eight class ratios, and all
three S2/S1 ratios per class. This treats the executed eight-class cohort
as the scope of inference, not a random sample of all ordinary curves.

For a level, equivalent within +/-5% requires its 90% interval inside
[0.95,1.05] and passing correctness/control checks. Five simultaneous
level claims additionally require the 98% interval for every level inside
that band (Bonferroni one-sided alpha=.01). Report failures as inconclusive,
never as equivalence. A detectable difference requires its 99% interval
to exclude one; report any smaller exploratory intervals as secondary.
With only two classes per CM field, family-specific statements remain
descriptive. No exponent fit or cryptographic-size inference is permitted.

Primary controls: all scalars verified, no missing primary attempt, exact
deterministic replays, BSGS exactness. Real positive control: rho.plain /
rho.negation should exceed one with its class-bootstrap 95% interval;
the reference prediction is sqrt(2), without asserting exact finite-size
equality. A/A and coordinate-model comparisons must be shown. Synthetic
rescaling of the observations is only a check of the analyzer, not a new
solver experiment or a measured positive control.

## B: CM-aware collector and recurrence control

For abs(D0)=19,43, scan gap primes from 8191 through 65535 upward. For each,
scan positive odd traces t=1..511 in ascending order. Accept the first
two distinct pairs per CM field with p=(t^2+abs(D0)*N^2)/4 prime, r=p+1-t
prime, r>4sqrt(p), and kernel-abscissa degree >=(N-1)/8. Degree is the
order of t/2 modulo N up to sign. Compute and record it exactly, not from
the size of N. Prime search, point search, and every rejection are setup.

Build the class-number-one crater and three distinct floor curves by
bounded deterministic sampling, seed=2026102510. A nonzero point killed
by prime r>4sqrt(p) proves #E=r; j different from the maximal-order j,
with Frobenius conductor N prime, certifies the floor by the order lattice.
Record these hypotheses and independent witnesses. Limit each floor
search to 64*abs(D0)*N+65536 candidates. Preserve any failed construction.
Make one isomorphic model per curve and transport its generator.

Sixteen independent one-target workloads/model, target_seed=2026102511.
All sides get the same scalar law; the independently chosen generators
are not asserted to be cross-gap images. On every model run:

- rho.negation, matched reference;
- ic.pipeline with cm-scalar-orbit:columns=4,length=32,abs_d=D,mode=scalar;
- the same pipeline with mode=recurrence.

On crater models, additionally run mode=velu. Generic recurrence is
P[k+2]=P[k+1]-[(abs(D0)+1)/4]P[k]; first action is [lambda]P.
The three modes must produce identical base points, coefficients and
columns before comparing their costs. Preserve existing scalar/velu
behavior and add recurrence as a new explicit parameter value.
Oracle=mitm:negation_folded=1, query cap=1,000,000, measurement seed=
2026102512, rounds=1, warmup=0, shuffle, L0, watchdog=120 seconds.
Expected 1,536 all-model reference/scalar/recurrence records plus 128
crater-map records = 1,664. Exact plan counts are verified before running.

Report native costs, query counts, relations, ranks, failures and full cold
S with the common pricing band. A ratio of lower bounds is a diagnostic.
No cross-method win is admitted without all prices and independent replay.
Any failed/censored workload prevents complete-pipeline inference for that
cell; successes are never silently substituted for the complete panel.
The prefix-seed coordinate control changes geometric seeds and is labelled
representation sensitivity, not a fixed-geometric-base arithmetic test.

## Execution, validation, provenance, and stop conditions

Before running: commit this protocol and native source, prepare and freeze
the exact models/specs, source/compiler/binary hashes and manifest. The
experiment-director compiles the mechanical task card; the main session
relays it verbatim to executor-mechanical and obtains an assessment.
Runs and replays are serial, --cpus none --allow-busy, using ecbench's lock.
Do not build/test or start other solvers during execution. Full replay of
every retained deterministic record is required; mismatches stop analysis.

Total execution guard: 90 minutes, 8 GiB of new evidence; preparation guard
15 minutes/class. Stop on wrong answer, failed audit, storage guard or
infrastructure failure, preserving all attempts. Resource exhaustion is
not evidence of a mathematical obstruction. A full panel is complete only
after all planned attempts have statuses; a checkpoint is explicitly partial.
No paid cloud resources or replacement lab credentials are provisioned.

Independent arithmetic replay and metric recomputation, a report with
source-linked graphs and inspected PDF, required tests, and a focused PR
are deliverables. The new branch is stacked on PR #2430; neither its open
state nor a passing CI job promotes the scientific result. No scoreboard
or leaderboard promotion occurs without the repository's validation gate.
Missing algebraic backends, broader families, hard-gap point transport and
cryptographic-scale coverage retain explicit open status.

Conductor preflight was attempted on 2026-10-25: command not found; no shared
lease is claimed. The supplied cloud environment has no lab/cloud runtime
identity. Work uses an isolated checkout and existing connected GitHub access.
