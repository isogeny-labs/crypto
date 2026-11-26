# Common factor-base lookup in cold recovery

2026-11-24 · Local development control · **Unsigned, unverified evidence**.

A shared signed-base lookup supplies one-term relations before any four-leaf
oracle is called. On the fixed F127 fixture, all three methods recover with
four four-leaf oracle calls each. The original-policy control matches all
31 earlier mask, row and consumer records, including five cofactor target-in-base refusals.
This implements a common recovery-policy component, not a new map/isogeny or
an admitted complete-cost gain. [pcc:unverified]

## Fixed policy comparison

Frozen source **6a0c732e8e**. Executable SHA-256:

```text
f643f9fb437e31b58422c54b5ff13f6a0f829da13bbe1778b9de7341c0b6c6e7
```

Both policies use \(E/\mathbb F_{127}:y^2=x^3+x+7\), order 109,
G=(1,3), Q=(11,98), eight unsigned base points, selector depth 4,
seed 781815 and maximum 30 draws. One run per arm/policy; no sampling interval.

| Arm / policy | Draws | Lookup hits | Four-leaf calls | Incomplete calls | Selected rows | Rank |
|:--|--:|--:|--:|--:|--:|--:|
| Direct (O) | 9 | — | 9 | 0 | 9 | 9 |
| Direct (L) | 8 | 4 | 4 | 0 | 8 | 8 |
| Refined (O) | 10 | — | 10 | 0 | 9 | 9 |
| Refined (L) | 7 | 3 | 4 | 0 | 7 | 7 |
| Cofactor (O) | 12 | — | 12 | 5 | 7 | 7 |
| Cofactor (L) | 8 | 4 | 4 | 0 | 8 | 8 |

O: original first-row policy. L: same policy with shared lookup.

All six arms return scalar 69 and pass the fresh final replay locally. Rank
includes the target coordinate, so complete base-plus-target rank is not needed
for its functional to be determined. Values above are recorded counts. [pcc:unverified]

![Recorded four-leaf calls, including failures; the lookup is shared by all arms. Source: evidence/base_lookup_v1/graph_data.csv.](base-lookup-counts.pdf){width=96%}

**Flow:** two suites and six arms planned, attempted and finished; none excluded,
unrun or retried. All child/outer exits are zero, all parent receipts reconcile,
and stderr is empty. Original/lookup outputs contain 37/29 records. The five
original cofactor refusals remain retained at draws 2, 3, 4, 8, 11. No flagged
query is incomplete before recovery. The fixed-prefix unit control covers all
five historical cases even when the new recovery stops earlier. [pcc:unverified]

\newpage

## What changes, and what remains open

![Public membership lookup and the original oracle feed the same consumer.](base-lookup.pdf){width=98%}

For a public residual \(R=[a]G+[b]Q=sP_i\), the submitted row is the single
coefficient \(c_i=s\). The existing consumer replays the group equality and
stores \(s x_i-bd=a\) over the subgroup field. No target scalar is supplied.
Dictionary construction is already in base setup; the new lookup phase and all
consumer work are included in the child intervals and full process receipt.

The recovery policy now admits singletons or the original four-leaf relations.
It does **not** implement the cofactor builder's missing target-in-base four-leaf
chart. Direct/cofactor retain their distinct-x, target-disjoint oracle; Refined
retains the narrower distinct-coarse-image oracle. Misses use those unchanged
methods and still retain failures. An original oracle call still contributes
only its first sorted relation; all additional outputs are currently discarded.

The 27 feature tests and six default tests pass, and the release build succeeds.
New tests replay both signs of every depth-four base point on both toy curves
(using each public point as Q), verify the original Q's five historical
refusals in the fixed mask prefix, preserve misses with an unavailable oracle,
and reject altered policy/count receipts. Local statement claims
PCC-20261124-c54b6a, 1a672f and 98f805 remain unverified. [pcc:unverified]

## Reproduction and remaining requirements

- [Protocol](BASE_LOOKUP.md), [reproduction](BASE_LOOKUP_REPRODUCE.md),
  [original records](evidence/base_lookup_v1/original.jsonl),
  [lookup records](evidence/base_lookup_v1/lookup.jsonl),
  [identity](evidence/base_lookup_v1/source_identity.json),
  [manifest](evidence/BASE_LOOKUP_SHA256SUMS).
- [Frozen source](https://github.com/isogeny-labs/crypto/blob/6a0c732e8e/research/smooth_p_plus_one_20261123/native/src/accounting.rs)
  uses the same production consumer and prior [accounting boundaries](COLD_ACCOUNTING_RESULT.md).
  Raw timings remain instrumentation diagnostics; no timing ratio is reported.
- Each child has 120 seconds; each query retains three seconds and prior solver
  limits. Each suite has 400 seconds plus five-second grace, one worker and
  2 GiB virtual memory. This is a bound, not measured peak memory. OS caches
  are not flushed; parent monitoring can add up to 5 ms. No paid run occurred.
- No new curve/isogeny theory is claimed. The [earlier comparisons](README.md)
  and their complete/failed query records retain their original scope. Study
  figures change here; no admitted scoreboard, timeline or leaderboard row changes.
- Root compilation, the historical benchmark-record gate and canonical export
  schema access remain as recorded in [development checks](DEVELOPMENT_CHECKS.md).
  The [local flow receipt](evidence/base_lookup_v1/development_flow.json) does not
  claim canonical export validation. Registered EKS payload execution remains open.

Next qualify a common policy that consumes **every emitted relation**, charging
all replay, duplicate handling and rank work. The present first-row choice is
not a strongest-reference claim. Keep this lookup control frozen, then resolve
larger full-chart output and registered admitted execution. The original goal
still requires a supported new mechanism and an independently checked full-cost
gain; the generic lookup alone does not establish either.
