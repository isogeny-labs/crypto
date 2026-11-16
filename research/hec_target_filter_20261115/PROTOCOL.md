# Target-preserving weighted filtering gate, 2026-11-15

Previous turn was progress: complete point cold pilot,18recovered/306timed
cells, all six median ratios0.066..0.430, E8passes/E9fails. The fixed-work
free-setup+forest diagnostic max2.190does not justify a10xfilter-only follow-up.
No scientific worker live. Read newest dated log plus point-coldSESSIONfirst.
This gate examines information preservation before any relation-selection
cost case. Class known-method engineering/mathematical boundary, not novelty.

## Literature before approach
KB ready lexical revision9141e386254340a9bdba3036f2bb706a,
last_refresh2026-11-14T21:44:48Zpredates latest work. Queries large-prime /
filtering / rank; HEC / spanning / tree; structured-Gaussian / singleton.
Existing ECDLPtechnique matrix already lists singleton peeling, clique removal,
structured merging and Markowitz. These are prior art, not new methods.
Primary Cavallar *Strategies in Filtering in the Number Field Sieve*,
https://ir.cwi.nl/pub/2078/2078D.pdf, Section2.1/2.2read: pruning and merging
are known, merge-weight optimization needs independent row combinations.
Its F2matrix setting does not justify dropping odd-field weights here.
Avanzi--Theriault publisher abstract read at
https://www.aimsciences.org/article/doi/10.3934/amc.2010.4.189;
full HECpaper not retrieved. It describes joint overharvesting/base-size and
selection, so ordinary harvesting cannot be described as a new breakthrough.
Do not transfer any quantitative benefit or doubleLPanalysis from that abstract.
Bouillaguet merge PDF fetch timed out; no claim its full text was read.

## Exact algebra / proposed gate

Let consistent relation system [B C][z;x]=d, with off-base variableszand corex
(including targetk). The exact projected constraints are

    y^T C x = y^T d for every y with y^T B=0.

Necessity follows by multiplying the system. Sufficiency follows from
im(B)=ker(B^T)^perp over a field. At most M-rank(B)independent projected
constraints exist; graph cycle count alone is not this number over odd fields.
For two nonzero endpoints per edge, each connected component's homogeneous
off-base solution is fixed by one root value until an inconsistent gain cycle
forces it to0: rank=N-1for balanced components, Nfor pinned/unbalanced ones.
One-endpoint rows also pin their component. This is ordinary weighted incidence
linear algebra, not a claim of a new gain-graph theorem.

Explicit same-support counterexample overF5, columnsx,y,kand RHS0:

 A: x+y+k=0; x+y=0 -> k=0.
 B: x+y+k=0; x+2y=0 -> everykpossible.

The unsigned graph/support is identical. Its one cycle emits a target constraint
in A, but pins an off-base variable in B. A support-only claim of identical
target rank is false. Current affine forest already handles these weights.

Safe singleton rule: for j!=target appearing in exactly ONEremaining row,
remove that row and free j. For any assignment of the other variables there
is a unique extension for j because its coefficient is nonzero. Therefore the
set of feasible targetvalues is unchanged, EVENfor inconsistent systems.
Zero-column removal is harmless; zero rows with nonzero RHSmustremain.
Protect target column: removing k=3as a singleton would destroy the answer.
Normalize duplicate terms before occurrence counting so cancellations are exact.

## Frozen native finite checks before code
Implement occurrence-queue peeling preserving original column IDs/RHSand
target; no degree2merge or arbitrary clique deletion. Compare full feasible
targetvalue SETS by exhaustive F5enumeration, dimensions1..3,128seeds each,
0..6random equations, including inconsistent systems. Include forced duplicates,
coefficient cancellation, isolated columns, allzero rows and target-only rows.
For consistent sparse planted systems over5/7/101, dimensions1/3/8/20,
32seeds and all prefixes0..2n+4, compare existing sparse solver's target result
before/after peeling. Check the two same-support counterexamples by both
exhaustive enumeration and solve_fast. Any discrepancy blocks selection use.
Count systems/prefixes/rows removed, no wall-time or extrapolated speed claim.

This is a gate and search/pruning report, not a scientific timing campaign.
No new cloud work, records, defaults, admitted benchmark or old raw edits.
Native Rust scientific execution only. Publish all failures/counterexamples.
Any successor must change useful-relation supply/cost jointly: the old fixed-
work bound cannot be evaded by removing filtering from accounting.

## Session
### What was done
Reviewed current source/logs and primary filtering sections; T-6100checked/
claimed this new directory. Wrote proof/counterexample and gate before code.
### Decisions made
Treat singleton filtering as established and preserve target feasibility;
reject topology-only rank certification in odd characteristic.
### Open threads
Execute exact gate and record actual coverage before any cost experiment.
### Errors
HECfull text not retrieved, mergePDFtimed out. Abstract-only scope explicit.
