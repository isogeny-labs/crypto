# Exact allocation audit: upper tail 0.292916567

30 October 2026. Independent subset-enumeration and rank-sum programs agree
on the complete finite allocation distribution. For the fixed comparison of
42 additional candidates and eight publications, 102 within-field pairs give
55 higher additional scores, seven ties and 40 lower scores. The observed
tie-weighted fraction is 117/204 = 0.573529412. The inclusive upper tail is

    5869345 / 20037600 = 0.292916567.

This agrees with the external report's 200000-draw estimate near 0.29. It does
not meet that report's nominal 0.05 replication threshold under the allocation
model. No real curve was sampled, and this fraction is not a percentile among
random curves, a probability of manipulation or a security score.

## Exact arithmetic and source checks

All 69 seed positions and 82 branch endpoints remain in the bound input. The
pre-existing primary comparison selects the 42 root/width-category additions
and eight publications in the 192/224/239/256-bit fields. The named eleven
omissions, seven P-384 positions and P-521 singleton are outside this particular
comparison; no new subset or cutoff is chosen after the audit.

| Field bits | Published | Additional | Pairs | Additional higher | Ties | Additional lower |
| --- | --- | --- | --- | --- | --- | --- |
| 192 | 3 | 21 | 63 | 29 | 6 | 28 |
| 224 | 1 | 8 | 8 | 7 | 1 | 0 |
| 239 | 3 | 9 | 27 | 17 | 0 | 10 |
| 256 | 1 | 4 | 4 | 2 | 0 | 2 |
| Total | 8 | 42 | 102 | 55 | 7 | 40 |

The producer enumerates all field subsets and convolves their exact integer
histograms. The independent verifier uses doubled midranks and a rank-sum
dynamic program. Together they account for all 20037600 joint label assignments,
including tied values as distinct labelled positions. Entire field/joint
histograms, not just the tail, are committed in [results](results/).

All 60 seed-identified source observations match; eight source publication
rows lack seed identifiers and are checked as field-score multisets. This
does not claim individual curve-identity replay for those eight. The source
study's original checkpoint/protocol deviations and unretained 48-bit validation
rows remain disclosed. This audit does not repair their historical admission
or execute any source sampling/null/Monte Carlo program. All source data and
labels were visible; this is retrospective verification, not a new holdout.

The finite tail has a p-value interpretation only under within-field label
exchangeability. It neither proves absence of selection nor corrects for all
previously explored features. The 102 pairs share curves and are not 102
independent curve draws; the conditional label calculation keeps that structure.

## What this means for the historical question and G/b

The [earlier broader census](../broader_census_reanalysis_20261030/RESULTS.md)
still has no strict separator anywhere100–5000. A conjunction applying root
and order-width screens before the narrow splitting rule still fits the
recorded labels. Its historical use and motive remain unverified. No original
decision/visitation log is available, and neither weakness hypothesis is proved.

[BASE-CHANGE.md](BASE-CHANGE.md) gives the algebraic meaning of a possible
public scalar recipe G=[s]Q. For nonzero s modulo the prime order n, multiplying
a target by s inverse converts its DLP to base Q, and multiplying by s converts
it back. That supplies no destination solver. Coefficient b is in Fp; scalar
interpretation modulo n is a separate operation. The coordinate identities
y²=b at x=0 and y²=b−2 at x=1 follow from the curve equation and are not scalar
logs. The old finite G/b measurements and conditional Lean proofs are unchanged.
No new scalar search, DLP or machine-checked theorem is claimed.

## Validation, provenance and limits

Source freeze fc7bac88dc217d01c59ff750d973d2796ebba1b4 precedes allocation
arithmetic. All five files reconstruct byte for byte, and all ten phases match
expectations: seven exit0, three deliberately altered field/frequency/summary
copies correctly refused with native exit1. Shared compiler and TSV I/O are
disclosed; no statistic/null arithmetic is shared. Sources, native binaries,
arguments, outputs and wrong copies are retained in the local raw archive.

The two-page anonymous report and standalone build pass make check. All pages
are inspected, fonts embedded, references resolved and no overfull boxes remain;
extracted text and both page rasters match. Native summary code alone produces
the presentation values. [REPRODUCE.md](REPRODUCE.md), [EXECUTION.json](EXECUTION.json)
and [DELIVERY.json](DELIVERY.json) bind the full record.

This is exact local analysis of existing verified results under AGENTS9a's
development exception, explicitly CHECKPOINT_SKIP. No S3 checkpoint/upload or
gate on the real-curve experiment is claimed passed. The previously denied
older paper payload remains local and excluded. No new curve, character,
point count, factorization, random draw, DLP, timing or paid job was introduced.

The requested 60000 accepted real-curve baseline and publication percentiles
remain unrun. [RUNTIME-RECHECK.md](RUNTIME-RECHECK.md) records restored native
public HTTPS/Git access with the network grant, absent prior temporary AWS
setup, and the still-unqualified PARI worker route. No new AWS authentication
verdict is inferred. No finite trait catalog exhausts all mathematical functions.

## Source count-typo amendment

The external post-run AMENDMENT.md is retained as an additional documentary
record at sources/AMENDMENT.md (upstream Git blob26831e84e4d787993aa624fd9d27be4b22337a77,
revision3ff8c32c15f91b355d18e207d6044a1d2363499f). It corrects a prose count:
42additional positions in the four primary fields,48including six P384positions.
The original explicit pair formula already gives102. Our own frozen plan and
inputs already specify42and102; no cohort, code, output or verdict changes.
This documentary addition was retrieved after the native audit and is bound
separately in RESULTS-SHA256SUMS, outside the already closed raw snapshot.
The original source/protocol and its procedural history remain unchanged.
