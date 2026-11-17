# Expanded cutoff loop: narrow support in the fixed cohort

**Verified exact result,30October2026.** The full100--5000integer sweep is
complete on all21original curve rows. Strict within-field separation holds
only at691--718and769--772:32of4901integer cutoffs (0.6529%), or4of645
effective positions. No cutoff from773through5000separates all four paired
fields. Including ramified characters yields zero successful cutoffs.

The [native summary](results/analysis.json), [field coverage](results/field-coverage.tsv),
[all21endpoint rows](results/endpoints.tsv), [independent verification](results/verification.json)
and [completion record](results/completion.json) are committed. The
[five-page PDF](../../../papers/nist-expanded-cutoff-20261030/paper.pdf)
and its [source/figures](../../../papers/nist-expanded-cutoff-20261030/)
document the result. Full raw tables and every control record are delivered
in the local archive identified separately in DELIVERY.json.

## Within-field coverage

| Field bits | Positive-margin cutoffs /4901 | Percent | Margin at691 | Margin at5000 |
| --- | ---: | ---: | ---: | ---: |
| 192 | 1018 | 20.7713 | 1 | -22 |
| 224 | 3957 | 80.7386 | 3 | 19 |
| 239 | 862 | 17.5882 | 4 | -14 |
| 256 | 374 | 7.6311 | 1 | -10 |

Margin means minimum strict count among the named unpublished candidates
minus maximum among paired publications. Every margin must be positive.
These are descriptive within-cohort grid fractions; they are not a statistical
comparison between fields with different publication/control counts.
P384/P521remain in all raw profiles and the endpoint table, but lack an
unpublished same-field control. No numeric margin is invented for them.

P224is the more persistent case in this cohort. Its publication has strict
count299at5000versus318,342,325for its three controls. No random-curve
percentile, security verdict or historical reason follows from that ordering.

## Exact retrospective label enumeration

| Strict cutoff family | Effective positions | Assignments allowing any separator |
| --- | ---: | ---: |
| Earlier2--2000 | 303 | 23/4480 |
| Requested100--5000 | 645 | 29/4480 |
| Supplementary union2--5000 | 669 | 30/4480 |

The requested and earlier families are not nested. Exactly22successful
assignments are shared;7occur only in100--5000and1only in the earlier range.
The supplementary30count is a descriptive set-union calculation from those
two flags, added during native postprocessing; it is not a new prospective
test. The fixed691count remains1/4480. The separate inclusive100--5000family
has30successful hypothetical label assignments; the observed inclusive
labels have no successful cutoff. Do not confuse those two quantities.

These proportions cover only the stated retrospective search families.
They neither authenticate historical exchangeability nor correct for every
previous feature search or produce a probability of deliberate weakening.

## Verification, source and scope

Source/input/commands were committed at93501d9143bf56cfa3f5cd1abb528b1f48daef14
before the real-cohort loop. The producer differs from the prepared source
only in an explicit local-analysis acknowledgement string; its mathematics
is unchanged. The PARI verifier source is byte-identical to the prepared
verifier. It independently reconstructs all five output files, checking14028
odd-prime characters,102921curve/cutoff rows,4901cutoff outcomes and4480label
assignments. Both implementations may share GMP integer primitives.

Ten phase outcomes match their expected native statuses. The two controls,
actual extraction/replay and three corruption-preparation phases exit0.
Each deliberately corrupted character/calibration/summary output is refused
with native1. No corrupt output is promoted to valid evidence.

This is exact local development reanalysis of existing certified results,
under the exception documented before execution in RUNPLAN.md. No new
point count, curve generation, factorization, DLP or timing benchmark ran.
No S3 checkpoint or remote raw-artifact receipt is claimed. The prior denied
paper archive was not exported. All raw and failed presentation/development
records are retained locally.

The result is cutoff-sensitive in the named cohort. It does not explain an
authenticated historical selector or show an ECDLP advantage. The broader
69position draft-compatible census and requested60000accepted random-curve
baseline remain separate unfinished work. Since the statistic uses only p,n,
this loop adds no hidden base-point/b relation test.

The original691graph, its historical303-position family, the G/b study,
factorization evidence and earlier paper remain unchanged as frozen records.
The new full-range graph is supplied here. No IC method, performance metric,
transfer edge, benchmark or leaderboard conclusion changes.

## Development and document notes

The actual mathematical loop passed on its first attempt. The native
descriptive reducer's first compilation failed a misleading-indentation
warning under -Werror; braces corrected it. Later source versions added the
explicit cumulative-family union and portable SVG presentation attributes.
Native scientific tables and plotted values were byte-identical before and
after the SVG-only correction. Earlier sources, logs and the bad rendering
are retained in the local archive.

The first PDF layout check caught one overfull source-hash line. The corrected
build passes make check with resolved citations/references, zero overfull
boxes and all fonts embedded. All five final pages were visually inspected.
No new novel mathematical theorem, historical caller or attack is claimed.
