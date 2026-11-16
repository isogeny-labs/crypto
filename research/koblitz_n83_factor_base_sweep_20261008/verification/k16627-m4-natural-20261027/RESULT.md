# K16627 four-summand card: source gate stopped model launch

The frozen model command exited 1 in 0.64 seconds with
`chained-S3 capacity parameters outside supported domain`. The pinned
`f61b33c1b1bb98efa22177a615270c160859a605` adapter accepts only
`m=5` and `m=6` at its model-capacity entry point, so no K16627 model,
solver or natural query was executed by this card. The raw
[stderr](evidence/stderr.txt) and exact input/source selections in the
[protocol](PROTOCOL.md) retain the failure. Its natural-query arm was
omitted under the frozen conditional rule.

This source gate does not invalidate the later [K16627 m=4 capacity result](../../../n83_k16627_m4_capacity_20261017/RESULT.md):
that separate source built 201,598 SAT variables, 5,915,238 clauses and
3,901 XOR rows under a 4 GiB hard cap. The later
[natural-query result](../../../n83_k16627_m4_natural_query_20261018/RESULT.md)
ended `UNKNOWN_budget` at 1,790 seconds, with no terminal attempt or relation
receipt. The [progress diagnostic](../../../n83_k16627_m4_search_progress_20261018/RESULT.md)
subsequently observed 322,255 decisions and 360 conflicts by its last
complete sample before a 1,800-second cap. Those retained runs are the
actual m=4 evidence; this card adds no yield or runtime estimate.

The next useful step is to repair or select a buildable source revision with
the later m=4 adapter and completed XOR-pass counters, then freeze a paired
natural-query/rank test across the retained bases. A source change requires
a new protocol and exact binary pin. `selected_best_total_runtime` remains
null.
