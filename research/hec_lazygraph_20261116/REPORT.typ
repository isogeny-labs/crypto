#set page(paper:"a4",margin:20mm)
#set text(size:10.5pt)
= Deferred signed LP elimination: exactness and applicability
Genus 2, 3 and 4 · 15 November 2026

== Algebra established and checked
For input matrix M = [L A], eliminating the large-prime columns produces retained rows R with

$"rowspace"(R) = {s : (0,s) in "rowspace"(M)}, quad "rank"(R) = "rank"(M) - "rank"(L).$

A weighted signed graph implements the legal pivots; unbalanced cycles anchor a component, balanced/already anchored cycles emit retained constraints. Emitted constraints can be zero or dependent.

#table(columns:(1fr,1fr),[Gate],[Result],[Arbitrary, unplanted synthetic streams],[36 pass],[Independent prefix rank / containment],[1440 pass],[Exact forward / transpose],[Products and full basis checks pass],[Actual curve reference cases],[12 pass per representation],[Additional supplied targets],[3 source-verified per representation],[Scientific gate exits],[All zero, empty stderr])

== Shared expressions and transpose
Store sparse input rows as leaves and affine combinations as arithmetic-circuit nodes. Apply R in topological order; apply its transpose by reverse accumulation. Costs are linear in actual circuit size plus leaf entries, including dead nodes. The graph and transpose methods are established prior art; see Gaudry et al. 2005 and Kaltofen / Saunders 1991 in SOURCES.md.

A constructed fill-heavy operator has 98,431 expanded nonzeros but a circuit multiplication bound of 3,582 per application. That fixture establishes representational potential, not curve performance.

== What actual curves changed
The first circuit is unfavorable on every curve case tested: cumulative walk-count vectors remain dense in its leaves. A second implementation shares their prefix sums, while keeping the original relation rows and pivots unchanged:

$ t_i = t_(i-1) + delta_i, quad A_i = D_i - t_i - b_i T - a_i C. $

All checks pass for the prefix representation. It improves the first circuit on genus 2 and 3, but worsens genus 4. Against expanded dot products it helps only four of five genus-2 cases, and none of the genus-3/4 cases. The complete table follows.

== Decision and limits
Keep both unfavorable results. Do not promote the synthetic fixture into a tenfold curve claim or start a timing cohort from it. Scalar solving remains explicit; the implicit operator has not been integrated into a Krylov solver. Catalogue/projection, relation production and exact group checks remain. A finite-field normal equation can lose rank, so a rectangular/singular solver needs a separate sound construction.

The mathematical tenfold objective remains open. A more promising next lever must change relation production or the retained algebra, supported by a full-cost case. These gates establish exact elimination and reject unsupported performance extrapolation.

#pagebreak()
= Complete curve representation counts
All original stopping trials, scalar values, row counts, decomposition counts and pivot hashes agree with the eager reference. Every emitted full operator and transpose is checked on basis vectors.

#table(columns:(0.9fr,1.2fr,0.4fr,1fr,1fr,1fr),[Genus / p],[Seed],[LP],[Expanded nnz],[Initial bound],[Prefix bound],
[2 / 101],[20260916],[1],[524],[911],[415],
[2 / 101],[20260916],[2],[459],[904],[464],
[2 / 101],[20260917],[1],[599],[1036],[455],
[2 / 101],[20260917],[2],[565],[960],[461],
[2 / 101],[20260918],[2],[506],[907],[457],
[3 / 61],[20260916],[1],[410],[638],[472],
[3 / 61],[20260916],[2],[413],[752],[490],
[3 / 61],[20260917],[1],[387],[654],[435],
[3 / 61],[20260917],[2],[387],[674],[434],
[3 / 61],[20260918],[2],[390],[753],[484],
[4 / 31],[20260916],[1],[158],[264],[333],
[4 / 31],[20260916],[2],[158],[319],[365],
[4 / 31],[20260917],[1],[152],[277],[380],
[4 / 31],[20260917],[2],[160],[339],[426],
[4 / 31],[20260918],[2],[158],[336],[378])

== What these units mean
Expanded dot products use one multiplication per nonzero; no special handling for coefficients plus/minus one is assumed. Circuit bound is total sparse leaf entries plus twice the number of sum nodes. Additions, allocation, zeroing, graph construction, oracle/search, group checks and explicit scalar solve are not included in the multiplication table. It is a representation diagnostic, not an optimized-baseline or wall-time comparison. Dead nodes are included, no cases dropped.

== Frozen stages and custody
Synthetic gate: ef4473fcf94a, S3 checkpoint 20261116T043823Z.

Actual curves: be5e94cd2808, checkpoint 20261116T043935Z.

Walk prefixes: 83fa8ce7331f, checkpoint 20261116T044149Z.

Each stage ran once after source, lock and binary hashes were committed and checkpoint-gated. Raw output, empty stderr and exit status are retained. Prefix preparation initially failed on a source-generation marker and missing binary target; corrected before its freeze and scientific execution. No timing result is asserted.
