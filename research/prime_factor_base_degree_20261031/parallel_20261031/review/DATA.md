# Independent completed-data review, 2026-10-31

This dated addendum follows the source-only reviews in
[MATHEMATICS.md](MATHEMATICS.md) and [INSTRUMENTS.md](INSTRUMENTS.md). Their
statements that experiments had not yet run describe their earlier review
stage. The parent subsequently admitted the bounded local run
`run-20261031T201044Z`. This reviewer did not launch new experiments or CAS
jobs, modify frozen instruments, or retry the original capped cases.

## Evidence checked

The raw run is `/workspace/scratch/secant-parallel-runs/run-20261031T201044Z`.
The code-source commit is `de5a3cfdc6bfd4744a15fa301a1a0b21c872e1b1` and the
recorded runtime checkout is `ff9d54f3ad4030dbe4e8426a02a2c5e5fa806b2c`.
All 18 locked source files and five recorded binaries match the source lock.
The frozen F4 helper source remains
`e201eda231b41820b1733eefd725841036075aadceb9d4cada92926893025842`.

Independent read/hash checks found no discrepancies:

- 85,390 hash comparisons cover the locked source/binaries, audit cells and
  references, nine exported ladder bundles, 14,220 F4 result records and their
  71,100 raw input/token/msolve/stdout/stderr files.
- Every one of the 14,220 saved input records equals its original exported
  bundle entry. All 27 ladder systems/reference/attempt hashes match the
  respective case records. All six arm/presentation populations in every
  ladder cell contain the recorded complete finite-target population.
- Original panel manifests reconcile ten panels, 14,220 requested/attempted
  systems, 14,194 v1 verification successes and 26 extraction caps. There are
  no other failed cases. All four signs remain present per secant target.
- For all 14,194 originally verified cases, independent parsing of preserved
  stderr agrees with the reported first-core pair/row counters and selected
  degrees; input degrees were recomputed from saved input monomials. The
  observed handled-degree definition agrees exactly.

Machine-readable review summaries are [DATA_HASH_REVIEW.json](DATA_HASH_REVIEW.json)
and [BUNDLE_REVIEW.json](BUNDLE_REVIEW.json). These are post-data integrity and
aggregation checks, not new relation searches or preregistered results.

## Exhaustive toy audit and actual F4 degree

The complete audit records 36 cells, 707,472 structured assignments and 10,404
dense assignments, with zero mismatches, false positives, false negatives or
group-replay failures. The 38 cell/reference hashes match. Its independent
group aggregation gives 56 finite ordered relations, eight doubles, 32 finite
unordered pairs and eight ordered inverse pairs for each signed base. The
candidate covers all 18 finite targets; the nongraph control covers 16.
This agrees with the earlier analytical calibration.

All 180 toy F4 systems verified. The actual metric is
`max(maximum_input_degree, maximum_selected_pair_LCM_degree)` in the stock
trace, aggregated by the maximum across all four secant branches. It is not
homogeneous regularity, inherited sugar degree, or a recorded maximum over
every polynomial arising inside every F4 matrix.

| Checked population | Graph degree | Candidate secant | Matched secant controls | Same-base dense |
| --- | ---: | ---: | ---: | ---: |
| F17, all 18 finite targets | 2 | 3 | 4 | 6 |
| F83, all 78 finite targets | 3 / 4 / 5 | 4 / 5 / 6 | 6 / 8 / 10 | 8 / 10 / 12 |
| F101, all 96 finite targets | 3 / 4 / 5 | 4 / 5 / 6 | 6 / 8 / 10 | 8 / 10 / 12 |
| F127, all 138 finite targets | 3 / 4 / 5 | 4 / 5 / 6 | 6 / 8 / 10 | 8 / 10 / 12 |

The F127 degree-5 line uses the separately versioned exact saved-basis
verification below for correctness of the 26 original capped cases. In v1,
six candidate target groups and four target groups for each control remain
unknown because their four-branch extraction did not complete. The original
primary analysis must retain those unknowns. The remaining rows have complete
original verification. The degree table in RESULTS.md states that distinction
correctly.

The supplied homogeneous theorem gives candidate delta=d+1 and dense
delta=2d+1; the actual dense handled degrees are higher. Both measurements
support a reduction for these encodings and these finite populations. They
do not prove a universal solver-degree theorem or novelty over prior work.

## Work, construction and yield interpretation

The nine construction cells accepted under the fixed proposal caps. The
attempt counts for d=3/4/5 are 1/4/23 over F83, 3/10/260 over F101, and
1/2/62 over F127. The case-level attempt, reference and system hashes match.
All low-x/hash-x control point sets differ from their candidate in these data;
no same-set control is silently credited as an independent base comparison.

Candidate/control SAT-target coverage varies in both directions. For example,
over F83 at d=3 the candidate covers 54 targets versus 56/58 for the controls;
over F101 at d=4 it covers 82 versus 78/72. Every equal-size signed base still
has the same complete-population ordered total K^2-K. Thus these data do not
establish improved total relation yield or independent-row yield.

The toy all-branch secant candidate uses 976 reduced pairs and 2,102 reduced
rows, versus 1,186/2,522 for its matched secant control. Its own dense
presentation uses only 344 pairs and 736 rows. The larger candidate panels
likewise have more branch-summed reduced rows than their dense presentation,
including F127 d=5: 55,506 versus 18,984. The degree improvement therefore
must not be relabeled as a demonstrated end-to-end speedup. Peak widths and
pair/row counts are useful component proxies; complete field-operation costs,
qualified timing, useful row rank and sparse linear-algebra costs remain
unmeasured.

The frozen analysis separates common-SAT, common-UNSAT and one-sided-SAT
comparisons, with incomplete groups assigned unknown degree. It distinguishes
same-set control encodings from distinct-base comparisons. The post-data
native report retains raw manifests, failed records, null missing metrics and
all four branches. Its descriptive status is explicit. I found no misleading
degree, branch-cost or v1-cap wording in the reviewed RESULTS.md.

## Exact offline saved-basis replay, version 2

I reviewed and approved only the bounded offline replay implemented in
`f4/replay_v2/`. It makes no CAS calls, changes no v1 source or raw records,
and does not raise or bypass the original extraction search budget.

Let I be the canonical input system, G the saved exported basis, and R the
complete independently enumerated signed group reference. The proof is valid:

1. Canonical coefficient reconstruction, exact candidate splitting and exact
   control/dense membership establish R=V_Fp(I), using the reviewed
   secant/group and S3 equivalence.
2. Zero exact normal forms of every input generator modulo G establish
   I contained in the ideal generated by G. Hence V_Fp(G) is contained in
   V_Fp(I)=R.
3. Evaluating every r in R in every g in G establishes the reverse inclusion.

The line-cubic identity covers x=z, x=r, z=r and the triple-equal flex case;
it introduces no divisions that lose these finite-target cases. Nonsingularity
excludes the spurious finite vertical-tangent case, and identity relations
are handled separately. Exact control membership excludes extraneous
abscissae. For dense S3, complete signed lifts and global negation identify
the reference for the selected target ordinate. All these arguments apply
to the actual saved complete inputs, not an arbitrary malformed system.

The replay additionally checks S-pairs, producer references where supplied,
and every available original extracted-root list. It reuses the frozen v1
field/model/group helpers; this is a separate proof route, not an independent
implementation of those primitives. It proves finite-field root-set equality,
not ideal or scheme-multiplicity equality. Enumerating the group reference is
verification work and cannot be counted as successful F4 relation discovery.

The completed companion root is
`/workspace/scratch/secant-parallel-runs/replay-v2-20261031T201044Z`.
Its manifest SHA-256 is
`adca412a6d4adcd970989aa8342d58060db4bb472c9435dbadad0e20a41be8f0`.
I independently checked all 14,220 unique companion records and 71,110
manifest/companion/raw hash links. Every proof succeeds, every available v1
root list agrees with R, and all metrics reparsed from the saved trace agree.
The 26 v1 caps remain recorded as 12 candidate, six low-x and eight hash-x
secant cases, all over F127 at d=5. Companion verification establishes their
saved-basis correctness without turning those operational caps into successful
original extraction runs. Details are in
[REPLAY_V2_REVIEW.json](REPLAY_V2_REVIEW.json).

## Review conclusion

The observed degree reduction on these exact toy populations is supported by
complete reference verification and preserved trace evidence. The component
cost comparison is mixed: the structured candidate improves over the matched
secant controls while its branch-summed work exceeds its own dense baseline.
No literature-wide novelty, P-256 transfer, 17-term improvement, complete
index-calculus speedup or rho parity follows from these experiments.
