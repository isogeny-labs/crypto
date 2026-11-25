# Larger full-output controls and whole-answer replay

2026-11-24 · Local development evidence · **Unsigned and unverified**.

The separate replay checker matched every reported complete answer to its
declared exhaustive reference set. On F8191 with degree-32 selection, Direct
recovers scalar 4356 after eight calls; Refined stops at rank nine after 30
complete calls, and cofactor exhausts all 30 query budgets. All degree-16 arms
remain unrecovered. The six process receipts and four post-run checks pass,
while five recovery endpoints remain incomplete. [pcc:unverified]

## Frozen larger controls

Both suites use \(E/\mathbb F_{8191}:y^2=x^3+x+5\), n=8053,
G=(0,789), Q=(3197,5073), seed 781815 and 30 maximum draws. The smooth
auxiliary torus has order p+1=8192; the target subgroup order is 8053.
All arms use common signed-base lookup and full-output consumption through
recovery. No singleton or skipped draw occurs in these recorded prefixes.

| Selector degree | Arm | Complete / incomplete calls | Consumed rows | Final rank | Scalar |
|--:|:--|--:|--:|--:|--:|
| 16 | Direct | 30 / 0 | 1 | 1 | none |
| 16 | Refined | 30 / 0 | 0 | 0 | none |
| 16 | Cofactor | 30 / 0 | 1 | 1 | none |
| 32 | Direct | 8 / 0 | 15 | 15 | 4356 |
| 32 | Refined | 30 / 0 | 9 | 9 | none |
| 32 | Cofactor | 0 / 30 | 0 | 0 | none |

The degree-16 base has seven unsigned points; degree 32 has fourteen. All
consumed rows are independent in these runs. Degree-32 Direct returns 19 rows,
consumes 15 and retains four unused rows after recovery; their generation is
already charged. Every cofactor degree-32 failure records `oracle budget
exhausted`. A zero process exit does not mean scalar recovery. [pcc:unverified]

![Final consumer ranks at recovery or the fixed 30-draw endpoint. One target/seed per arm; unequal stopping points are not a cost comparison. Source: evidence/larger_full_output_v1/graph_data.csv.](larger-full-output-counts.pdf){width=96%}

**Flow:** two recovery suites, six arms and four separate audits planned and
completed; one recovered arm, five incomplete recoveries. Zero excluded,
unrun or retried. All six wrapper exits are zero; stderr is empty. The new
recovery files contain 96/74 records and each audit emits one record, for 174
new records total. The historical audit inputs are reused without new solves.
[pcc:unverified]

\newpage

## Whole outputs, including unused suffixes

![Post-run checking compares complete sets and replays recorded decisions; incomplete oracle outputs remain missing.](larger-full-output-audit.pdf){width=98%}

The checker regenerates masks and residuals, checks skipped/singleton decisions,
and compares every completed query's **entire returned set**. Direct/cofactor
use four distinct x-coordinates disjoint from target x; Refined retains four
distinct coarse Dickson images. Every consumed row is replayed through a fresh
production consumer, including its canonical receipt, rank and work counters.

| Recorded input | Complete sets | Incomplete calls | Returned rows | Replayed rows |
|:--|--:|--:|--:|--:|
| F127 full output | 11 | 3 | 49 | 37 |
| F127 full + lookup | 6 | 0 | 29 | 35 |
| F8191 degree 16 | 90 | 0 | 2 | 2 |
| F8191 degree 32 | 38 | 30 | 28 | 24 |

Totals: 145 complete sets, 108 returned rows and 98 consumed rows; seven
consumed rows are historical singleton hits. The 33 incomplete queries are
preserved with no missing output inferred. All four checks pass locally.
Shared fixture/reference/consumer source means this is an oracle cross-check
and separate replay execution, not independent implementation or signed
validation. Audit work is outside the recorded recovery intervals. [pcc:unverified]

All 33 feature tests and six default tests pass; the release build succeeds.
Six corruption controls include deleting an unused suffix row and adjusting
its counts so parent consistency still passes: the full-set checker rejects
that omission. Failed-oracle identity/duplicate skips remain incomplete under
replay. Claims **PCC-20261124-2b5ba8** and **4c549b** retain unsigned standing.
[pcc:unverified]

## Identities, reproduction and next question

Recovery source **77c465a6d1** and executable remain unchanged. Auditor source
**15ff500915**; before-call identity **49517176cd**. SHA-256 identities:

```text
recovery 82de0db7d03119b3b2eeb1091a87775b126357c0be827274866c6f2438b75e88
auditor  4cb6b4682bee77c11c3067be78f680ed6bbae66aefae57a72fa3e30fcc171325
```

[Protocol](LARGER_FULL_OUTPUT.md) · [reproduction](LARGER_FULL_OUTPUT_REPRODUCE.md) ·
[raw records](evidence/larger_full_output_v1/) · [manifest](evidence/LARGER_FULL_OUTPUT_SHA256SUMS).
Limits remain three seconds/query, 120 seconds/child and 400 seconds plus
five-second grace/suite; each audit has 60 seconds plus grace. One worker and
2 GiB virtual-memory bound; no OS cache flush. All setup, collection, consumer
and process boundaries remain [explicit](COLD_ACCOUNTING_RESULT.md).

The results separate low relation yield from solver failure. Next investigate
full-coverage cutoff partitions over feasible fine-image multisets, checking
repeated-image coverage and total work across all branches before any new run.
This is a proposed construction question, not a demonstrated mechanism or gain.
The [prior first-collision study](../prime31_ic_20261115/native/DICKSON_FIRST_COLLISION_S3_20261115.md)
shows why smaller individual systems alone do not settle whole-query cost.

Registered EKS payload/admission, complete full-chart/oriented output and
independent cost evidence remain open. No cost ratio or canonical export
validation is claimed. [Development limitations](DEVELOPMENT_CHECKS.md) and
all earlier failures remain retained; no scoreboard or leaderboard changes.
