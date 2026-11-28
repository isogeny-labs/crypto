# Consume complete oracle output in cold recovery

2026-11-24 · Local development control · **Unsigned, unverified evidence**.

All twelve frozen F127 recovery arms finish with scalar 69 and reconciled
process receipts. With both shared lookup and full-output consumption, Direct
and cofactor-msolve each use one four-leaf oracle call plus two singleton rows,
over three draws. Refined uses four calls and three singleton rows over seven
draws. This qualifies a stronger common collection policy; it does not supply
a new map or an admitted total-cost gain. [pcc:unverified]

## Fixed policy comparison

Frozen source **77c465a6d1**, before-call identity commit **da258b6de6**.
Executable SHA-256:

```text
82de0db7d03119b3b2eeb1091a87775b126357c0be827274866c6f2438b75e88
```

All arms use \(E/\mathbb F_{127}:y^2=x^3+x+7\), n=109, G=(1,3), Q=(11,98),
eight unsigned base points, selector depth 4, seed 781815 and 30 maximum draws.
One run per arm/policy; no sampling interval or timing ratio.

| Policy | Direct calls / rows | Refined calls / rows | Cofactor calls / rows |
|:--|--:|--:|--:|
| First row | 9 / 9 | 10 / 9 | 12 / 7 |
| First row + lookup | 4 / 8 | 4 / 7 | 4 / 8 |
| Full output | 2 / 14 | 7 / 10 | 5 / 13 |
| Full output + lookup | 1 / 13 | 4 / 9 | 1 / 13 |

Calls include incomplete four-leaf queries; rows are all submissions to the
consumer, including dependent and singleton rows. Original cofactor has five
unsupported-chart calls; full-output cofactor without lookup has three before
its earlier recovery. All other cells have zero incomplete calls. [pcc:unverified]

![Recorded four-leaf calls for all four policies; one fixed target and seed per cell. Source: evidence/full_output_v1/graph_data.csv.](full-output-counts.pdf){width=97%}

**Flow:** four suites and twelve arms planned, attempted and completed; zero
excluded, unrun or retried. Every child/outer exit is zero and stderr is empty.
The suites emit 37/29/20/19 records. Both first-row histories match all 54 earlier
attempts. Full-output-plus-lookup Direct/cofactor also have identical three-draw
mask, returned-row and consumer histories, apart from solver-specific metadata.
The first failed unit-test expectation is retained separately. [pcc:unverified]

\newpage

## Every submitted row is charged

![The full oracle output is retained; only recovery stops the successful consumption prefix.](full-output-flow.pdf){width=97%}

For two outputs from one residual, \(c x-bd=a\) and \(c'x-bd=a\) imply
\((c-c')x=0\). Additional rows may therefore add constraints. The consumer
replays every submitted equality and determines its actual rank contribution.
All new-policy recoveries reach rank 9; the following are recorded counts.

| Arm / policy | Returned oracle rows | Consumed | Unconsumed | Dependent |
|:--|--:|--:|--:|--:|
| Direct / F | 17 | 14 | 3 | 5 |
| Refined / F | 11 | 10 | 1 | 1 |
| Cofactor / F | 21 | 13 | 8 | 4 |
| Direct / FL | 11 | 11 | 0 | 4 |
| Refined / FL | 7 | 6 | 1 | 0 |
| Cofactor / FL | 11 | 11 | 0 | 4 |

F: full output. FL: full output with lookup.

Singleton submissions are additional to consumed oracle rows. No duplicate or
rejected row occurs in these six runs. Every unconsumed suffix follows recovery;
its prior construction/extraction cost remains included. [pcc:unverified]

Format 3 retains the entire sorted oracle answer, each consumed row/receipt,
rank transitions and cumulative counts. The parent rejects lost/reordered
prefixes, unexplained early stops and mismatched rank/count/policy metadata.
Formats 1 and 2 retain their original first-row behavior. Refined still uses
the narrower coarse-image oracle; the common policy does not equate chart scope.

Corrected feature tests pass 30/30, default tests pass 6/6, and the release build
succeeds. The initial run had 29 passes and one false assertion that lookup would
not encounter a singleton before recovery. Its exact source/log is retained;
the correction restricts the historical fixed-four-leaf reducer comparison to
the four-leaf-only run. Claims **PCC-20261124-63f792** and **b2543e** remain
unsigned and unverified. [pcc:unverified]

## Reproduction and next requirement

[Protocol](FULL_OUTPUT.md) · [reproduction](FULL_OUTPUT_REPRODUCE.md) ·
[records and identity](evidence/full_output_v1/) ·
[manifest](evidence/FULL_OUTPUT_SHA256SUMS) ·
[development flow](evidence/full_output_v1/development_flow.json).

All setup, lookup, replay, deduplication, rank, linear algebra, final checks and
process overhead retain the [accounting boundaries](COLD_ACCOUNTING_RESULT.md).
Bounds remain three seconds/query, 120 seconds/child and 400 seconds plus
five-second grace/suite, one worker and 2 GiB virtual memory. Parent polling can
add 5 ms; OS caches are not flushed. Raw timers are development diagnostics.

Next qualify larger full-output controls with this strengthened reference under
a separate finite protocol, then the registered workload and admitted cost
comparison. Larger full-chart/oriented completion and independent standing
remain open. [Prior comparisons](README.md) and all failures retain their scope;
no scoreboard, timeline or leaderboard changes. The local flow schema does not
claim canonical experiment-export validation; root-build, historical gate and
export-access limitations remain in [development checks](DEVELOPMENT_CHECKS.md).
