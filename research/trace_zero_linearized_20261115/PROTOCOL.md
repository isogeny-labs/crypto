# Symmetric-residual linearization and prepared-admission scale panel

2026-11-15, next independent pre-data round after the terminal686-row panel.
Same authorized local1CPU/2GiB/600second experiment cap, no paid resources or
retries. Earlier protocols/results remain unchanged and source-bound.

## Construction and scope

The existing binary S3 formula is
v²+z*v+z²*u²+b=0 for u=x1+x2 and v=x1*x2. For a fixed residual target z,
u lies in V=span of the actual admitted x codes, v lies in W=span(V*V),
and the displayed equation is binary linear in their coefficient bits.
The native solver constructs those spans, solves the coefficient system,
enumerates its full affine kernel up to the shared model cap, factors
X²+uX+v with the existing field solver, and checks actual supplied point
signs and original group sums. Identity residuals are handled separately.

A rank-deficient space is not silently assumed unique: incomplete coefficient
enumeration or deadline returns UNKNOWN. Refuted requires every residual and
every necessary coefficient solution to be exhausted. This is known symmetric
S3/linearization algebra, not a new criterion or geometric trace-zero chart.
The integral expansion certificate records the explicit even correction;
its evaluated scope is not a formal characteristic-two completeness proof.

## Frozen input and arms

Keep all49 earlier inputs unchanged (discovery/anchor set). Before any new
solver data, add16 new scale cases: n83/131 by d10/12, two distinct-x planted
targets, one uniform full-width scalar and one cancellation per cell. Native
seed rule202611151000+100*n+10*d+kind_index, pinned StdRng. Planted targets
are nonidentity and outside the base, from three distinct x values. Complete
truth preparation is a separate native, CPU0/2GiB/600second process using no
candidate solver; it must finish before the input hash/final protocol freeze.
Report old anchors separately from new heldouts; selection followed the
previous round but preceded the new heldout/candidate results.

Five arms on the same65 cases, two repetitions (650 planned primary rows):

- Original sliced SAT, retaining repeated admission in inner residual calls.
- Prepared sliced SAT: validates immutable base/target once per outer query;
  residuals computed from admitted full-kernel points use a private admitted
  path. All actual model/group replay remains. No public validation bypass.
- Symmetric residual linearization, validating once per query.
- Exact repeated linearized A/A.
- Fresh classical pair-table reference, rebuilt/charged per query; no provided
  truth or cached precomputation. Batch amortization is not measured.

All actual signs are retained. Rotate arms by instance index+replicate. Do
not change samples, thresholds or budgets after results. Query750ms safety
fuse and2000 total SAT conflicts where applicable,500000 allocation cap,
1million domain clauses and1024 total models. Linear coefficient models
share that1024 cap across residuals. Linear columns are recorded separately
from SAT variables; no fictitious SAT allocation/conflicts for linear algebra.

## Pilot, metrics and validity

45 engineering pilot calls: every arm at n5,d0; n7/9,d4; and n83/131 at
d5/8/12. Largest bases are included. Non-n5 targets use separate full-width
uniform scalars from seed202611154000+100*n+d; n5 targets3*base[0]. All rows
must be valid and every n5 arm must Found/replay. Pilot known-answer work and
calls are within the600second combined pilot/freeze/primary cap. If the
engineering estimate is infeasible, preserve the attempt and amend before
primary data rather than downscale silently.

Planning map, fixed before pilot: n5/7/9 use their sole field representative;
n83/131 use the smallest pilot dimension at least as large as the primary
dimension (d4->5,d6->8,d10->12). Sum the per-arm mapped costs for all650 rows
and add total pilot/setup elapsed. Do not multiply every row by the largest
base's cost or use the estimate as timing evidence.

Primary: exact valid decision counts for the complete panel, new scale cases,
and field/dimension/kind strata. Secondary: SAT conflicts, SAT allocations,
linear coefficient columns and actual group additions/doublings across
setup, solving, lifting and final replay. Field construction, Gaussian
elimination, polynomial factorization and SAT work are not priced by that
group unit; no runtime ratio/full-cost/scaling/recovery claim.

Every UNKNOWN remains in the denominator. A failed answer stops the panel
with raw prefix retained; unattempted rows are accounted, never retried.
The native deterministic summary and exact inputs are hash-frozen before
primary rows. Labels are unmasked for this declared descriptive table; no
population p-value or confidence claim. Report repeat spread and A/A
outcome/conflict discrepancies. Native post-run audit rechecks truth and
every saved actual point tuple; independent signed review remains absent.

Sources: binary_semaev.rs existing Vieta/S3 derivation and quadratic solver;
Semaev2004/031, Diem2011 references there; previous trace-zero subspace/product
profiles and terminal-domain study. KB unavailable (wrapper127), scoped
primary/repository fallback. No global novelty assertion.
