# Split-graph bases: complete toy audit and actual F4 comparison

Run `run-20261031T201044Z`, 2026-10-31. The requested three tracks ran:
the complete toy audit, matched stock F4, and the nine-cell degree-3/4/5
construction ladder. The initial tracks ran concurrently; F4 consumed the
larger exports after construction. The frozen primary analysis records
14,220 attempted systems, 14,194 verified systems, and 26 root-extraction
caps. No CAS rerun replaces those caps.

## Observed degree reduction

The split-graph secant presentation lowers the **observed handled F4 degree**
on the checked systems. This metric is exactly the larger of input-generator
degree and the maximum selected-pair LCM degree in the actual stock F4 trace.
It is not sugar degree, the maximum degree of every matrix polynomial, or
the homogeneous invariant of the preceding theorem.

| Field population | Graph degree | Candidate secant | Matched control secant | Same-base dense | Original verification |
| --- | ---: | ---: | ---: | ---: | --- |
| F17, all 18 finite targets | 2 | 3 | 4 | 6 | 180/180 systems |
| F83, all 78 finite targets | 3 / 4 / 5 | 4 / 5 / 6 | 6 / 8 / 10 | 8 / 10 / 12 | Complete |
| F101, all 96 finite targets | 3 / 4 / 5 | 4 / 5 / 6 | 6 / 8 / 10 | 8 / 10 / 12 | Complete |
| F127, all 138 finite targets | 3 / 4 | 4 / 5 | 6 / 8 | 8 / 10 | Complete |
| F127, all 138 finite targets | 5 | 6 on 132 complete target groups; 6 unknown | 10 on 134 complete groups per control; 4 unknown each | 12 on all targets | 2,044/2,070 systems |

Ladder controls are the prospective low-x and hash-x sets of the same size,
with both signs. Values in a secant target group require all four branches.
The F127 degree-5 unknowns are retained, even if a separately versioned
offline check establishes correctness of their saved bases.

The earlier [theorem](../SECANT_GRAPH.md) establishes the specified
homogeneous top-ideal invariant `delta=d+1` versus `2d+1` for the dense
presentation under complete squarefree rational splitting of `h²-f`.
The toy candidate/control homogeneous values are 3/4, versus dense 5.
The measured dense handled degree is higher. These invariants are deliberately
reported separately. Literature-wide novelty remains unconfirmed.

## All-branch work and relation yield

| Toy arm, all 18 finite targets | Reduced pairs | Reduced rows | Peak columns / rows | SAT targets |
| --- | ---: | ---: | --- | --- |
| Candidate secant, four branches | 976 | 2,102 | 16 / 18 | 18/18 |
| Nongraph secant, four branches | 1,186 | 2,522 | 28 / 28 | 16/18 |
| Same candidate base, dense | 344 | 736 | 21 / 17 | 18/18 |
| Same control base, dense | 330 | 706 | 21 / 17 | 16/18 |

The candidate improves degree, matrix width, and pair/row counts against the
matched **secant** control. Against its own **dense** presentation, it lowers
degree but increases branch-summed pairs and reduced rows. The larger
completed panels show the same direction for branch-summed row counts.
These are component proxy counts, not complete field-operation costs.
The stock engine does not expose the complete field-operation total;
cost per usable relation and timing speedup therefore remain null.

Both toy bases have 56 finite ordered relations, eight doubles and 32
unordered pairs, plus eight ordered inverse pairs at the identity. The
candidate covers 18 finite targets and the control 16. Equal-size bases have
the same total all-target ordered yield `K²-K`; geometry changes its
distribution. There is no claim of higher total pair yield or measured
independent-row yield.

## Correctness and construction

The independent exhaustive audit checked **707,472 structured assignments**
and **10,404 dense assignments**, including all four signs and diagonals,
with **zero mismatches, false positives, false negatives or replay failures**.
Identity targets were checked separately through inverse pairs.
All 180 toy F4 systems verified against complete group references.

All nine prospective ladder cells accepted a fully split graph under the
frozen 512-proposal cap. Every rejected curve/graph attempt is retained.

| Curve | Graph attempts for d=3 / 4 / 5 |
| --- | --- |
| `icv1-fp7-t5-de12415d` (F83, order 79) | 1 / 4 / 23 |
| `icv1-fp7-t5-b172aa85` (F101, order 97) | 3 / 10 / 260 |
| `icv1-fp7-tm11-1cabae54` (F127, order 139) | 1 / 2 / 62 |

The toy curve is `icv1-fp5-tm1-a77cc6af`; its candidate is
`FB1hed5b19904cc8` and its nongraph control `FB1h66fbb7ed3fb6`.
Ladder manifests and factor-base dumps carry the canonical ICV1/FB1 formats.
Central registry/EC1 linkage is pending; this report does not promote an
index-calculus leaderboard row.

## The 26 original extraction caps

All stock F4 children completed. The recursive rational-root extractor hit
its 2,000,000-assignment limit on 26 F127 degree-5 systems: 12 candidate
secant, six low-x secant, and eight hash-x secant. The original records remain
`verification_failed` with `root extraction assignment cap`; the primary
analysis treats them as censored/unknown, not correctness errors or successes.

The separately versioned [offline replay](f4/replay_v2/README.md) checks saved
inputs and bases without calling CAS. Its two-inclusion argument compares
the saved basis with exhaustive independent group references. This can
establish finite-field root-set equality for saved outputs; it does not make
the capped extraction algorithm complete or supply a cheaper discovery cost.
The companion completed **14,220/14,220 exact saved-basis checks**, with zero
failed or unattempted checks and zero new CAS calls. This includes all 26
original capped cases. Its manifest SHA-256 is
`adca412a6d4adcd970989aa8342d58060db4bb472c9435dbadad0e20a41be8f0`.
The frozen analysis still has 26 extraction unknowns. The companion is a
complete correctness check of saved finite-field root sets, not a complete
operational discovery run. Versioned metrics remain separate.

## Reproducibility and admission

The [protocol](protocol.json) was frozen before affected data at local code
revision `de5a3cfdc6bfd4744a15fa301a1a0b21c872e1b1`, runtime revision
`ff9d54f3ad4030dbe4e8426a02a2c5e5fa806b2c`.
The canonical protocol hash excluding `freeze` is
`b36100e3394ba1837876a2b52ab5a570548f5b406b207a2108a91003953b9618`.
[source-lock.json](source-lock.json) binds instrument bytes, specifications
and binaries. The public follow-up commit has a different parent/tree; the
standalone source files are published with those exact byte hashes. Local
commit identifiers are provenance, not a claim of identical public trees.

Actual F4 is stock msolve-neogb, revision
`3672113ee2c696a2ac9c637636e0f7214f500ba1`, exact sparse mode 2, seed 315,
one thread, behind the included thin C adapter. This is not the full msolve
CLI. All sign branches, controls, failed construction attempts and extraction
caps are charged or retained. Source/spec freeze and positive controls preceded
the run. Post-data descriptive reporting and offline verification are labelled
as amendments; the original analysis remains unchanged.

Execution used the specific bounded local exception recorded in the protocol:
one CPU and 2 GiB address space per track, at most three concurrent tracks;
audit 120 seconds, ladder 180 seconds outer/120 internal, and combined F4
1,800 seconds with 10 seconds per child. S3 checkpoint admission remained
failed and was not relabelled as a pass. No paid resources were used. Host
clocks/RSS are L0 operational diagnostics; no timing inference is made.

## Requested deliverables and remaining obligations

| Requirement | Status and evidence |
| --- | --- |
| Complete toy correctness, signs, doubles, identity and replay | Verified; [audit evidence](audit/REPORT_EVIDENCE.md), raw audit manifest |
| Matched actual F4 and complete populations | Executed; 14,194/14,220 original verification successes, 26 caps retained |
| Prospective d=3,4,5 construction and rejected work | Verified; all nine cells, attempt logs, exact factorization and FB1 dumps |
| Degree/counter comparison, SAT strata and dense reference | Frozen analysis plus versioned descriptive report; no unpriced-cost substitution |
| Independent mathematical/instrument/data review | [review/](review/); full polynomial bridge remains written, not Lean-formalized |
| Reproducible artifacts, illustrated report and comparison dashboard | This directory, archive/hash and task-specific dashboard |
| Literature-wide novelty | Open; classical group-law/Riemann–Roch antecedents cited |
| Higher-arity relation finding and independent-row solve cost | Not implemented by this two-summand component study |

The next implementation target is sharing elimination across the four sign
branches and replacing coordinate-cube root extraction with exact triangular
factorization. A new prospective comparison should charge that extraction,
all branch work and construction, and test whether reduced degree lowers
complete same-base relation cost.

## Sources and graph disposition

Primary sources checked: [MIT group-law notes](https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf),
[Petit–Kosters–Messeng, PKC 2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
and the [pinned msolve implementation](https://github.com/algebraic-solving/msolve/tree/3672113ee2c696a2ac9c637636e0f7214f500ba1).
See [independent mathematics review](review/MATHEMATICS.md) for assumptions.
This study tests a previously written theorem, whose identity certificate is
`IDC1h621ed1a51b5f3ca8`; it makes no new algebraic-identity claim.

The task-specific comparison dashboard is updated with the exact component
counts and original capped flow. Canonical IC scoreboards and their twins,
the progress timeline and curve-transfer diagrams remain unchanged because
their priced-pipeline/validation contracts have no corresponding result from
this component study. No unvalidated numeric benchmark or IC leaderboard row
is added. The report and graph provide the measured comparison at its scope.
