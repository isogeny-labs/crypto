#!/usr/bin/env python3
"""Check the exact arm emits the identical relation stream, rank trajectory and scalar."""
import json, sys
b = json.load(open(sys.argv[1])); e = json.load(open(sys.argv[2]))
norm = lambda rows: [(r["label"], [tuple(t) for t in r["terms"]]) for r in rows]
checks = {
 "relation_rows_identical": norm(b["relation_rows"]) == norm(e["relation_rows"]),
 "pair_entries": (b["pair_entries"], e["pair_entries"]),
 "distinct_pair_sums": (b["distinct_pair_sums"], e["distinct_pair_sums"]),
 "probes": (b["probe_additions"], e["probe_points"]),
 "classes": (b["classes_by_base_terms"], e["classes_by_base_terms"]),
 "ranks": ((b["lp_rank"], b["projected_rows"], b["factor_base_target_rank"]),
           (e["lp_rank"], e["projected_rows"], e["factor_base_target_rank"])),
 "per_query_rank": [ (q["lp_rank"],q["projected_rows"],q["factor_base_target_rank"]) for q in b["queries"]] ==
                   [ (q["lp_rank"],q["projected_rows"],q["factor_base_target_rank"]) for q in e["queries"]],
 "scalar": (b["recovered_scalar"], e["recovered_scalar"]),
}
ok = checks["relation_rows_identical"] and checks["per_query_rank"] and all(
    x == y for k,(x,y) in ((k,v) for k,v in checks.items() if isinstance(v, tuple)))
checks["PASS"] = ok
print(json.dumps(checks, indent=1)); sys.exit(0 if ok else 1)
