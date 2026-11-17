#!/usr/bin/env python3
"""Check that post-hoc budget truncation of arm (a) equals a real conf_budget run (Glucose)."""
import json, random
from pysat.solvers import Solver
from pdp import Field, Curve, Enc
out = []
for n, l, B in ((11, 4, 1024), (11, 5, 128), (7, 3, 4)):
    F = Field(n); E = Curve(F)
    rows = [json.loads(x) for x in open(f"rows_n{n}.jsonl")]
    A = {r["target"]: r for r in rows if r["l"] == l and r["arm"] == "a_glucose"}
    same_result = 0; real_tot = 0; post_tot = 0; rel = 0; real_rel = 0
    for i, r in sorted(A.items()):
        enc = Enc(F, l, r["xR"])
        with Solver(name="glucose4", bootstrap_with=enc.clauses) as s:
            s.conf_budget(B); res = s.solve_limited(); c = s.accum_stats()["conflicts"]
        real_tot += c; post_tot += min(r["conflicts"], B)
        rel += (r["result"] == "SAT" and bool(r["lifted"]) and r["conflicts"] <= B)
        real_rel += (res is True and bool(r["lifted"]))
        # a real run must be a prefix of the unbudgeted run: same answer if it finishes
        same_result += (res is None) or (res == (r["result"] == "SAT") and c == r["conflicts"])
    out.append(dict(n=n, l=l, B=B, targets=len(A), prefix_consistent=same_result,
                    posthoc_exact_cut_conf_per_rel=round(post_tot / rel, 1) if rel else None, posthoc_relations=rel,
                    real_glucose_conf_budget_conf_per_rel=round(real_tot / real_rel, 1) if real_rel else None, real_relations=real_rel,
                    real_over_posthoc_total_conflicts=round(real_tot / post_tot, 2)))
print(json.dumps(out)); json.dump(out, open("budget_check.json", "w"), indent=1)
