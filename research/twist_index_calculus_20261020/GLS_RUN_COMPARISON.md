# Frozen GLS run comparisons

The producer sources and all raw files are retained independently. The
[native verifier](gls_symmetry_verify.rs) passed each complete file and
checked every algebraic census, target scalar and folded witness orbit.

For the v2 A/A comparison, the only fields omitted below are JSON keys
ending in `_ns` (all phase and total wall-time fields). The command
`jq -c 'with_entries(select(.key|endswith("_ns")|not))' FILE | sha256sum`
returns
`49755e845355c2bbe2f28ac3a3917632d570c27667dc4553f7074d7eb9a47c4a`
for both [v2](gls_symmetry_results_v2.ndjson) and
[v2 A/A](gls_symmetry_results_v2_aa.ndjson). Thus all retained fields,
including counter categories, are byte-identical in record order.

For the v1/v2 algebraic comparison, retain full curve, census and negative
control records, and for each solve/rho record retain
`kind,p,b,seed,arm,columns,rank,trials,raw_hits,unique_rows,novel_rows,online_queries,recovered,correct`.
The exact `jq` selector is:

```sh
jq -c 'if .kind=="curve" or .kind=="census" or .kind=="negative_control" then . else {kind,p,b,seed,arm,columns,rank,trials,raw_hits,unique_rows,novel_rows,online_queries,recovered,correct} end' FILE | sha256sum
```

It returns
`32d3744687ae623a42aa260a30aa8b84a6d1d2065a852bc6b8d34ca7631c2efb`
for both [v1](gls_symmetry_results_v1.ndjson) and v2. This comparison
intentionally omits implementation-specific operation counts and wall
times; it is not evidence that v1 and v2 cost the same.
