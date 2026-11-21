#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, sys, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# every IC option keeps the table size, the trials, the relations and the relation-phase trials
CNT = ("table", "trials", "relations", "p1_trials")
same = collections.defaultdict(set)
for r in rows:
    if r.get("arm") == "ic5b" and "error" not in r:
        same[(r["pb"], r["T"], r["idx"])].add(tuple(r[c] for c in CNT))
ic_mismatch = sorted(str(k) for k, v in same.items() if len(v) != 1)
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["T"], r["idx"], r["key"])].append(r["ms_arm"])
tm = {k: st.median(v) for k, v in t.items()}
cfgs = sorted({(pb, T, i) for pb, T, i, _ in tm})
keys = sorted({k for *_, k in tm})
rhok = [k for k in keys if k.startswith("rhodpb")]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad), ic_count_mismatch=ic_mismatch),
           cells={})
PAIRS = [("ic:rel+cb", "ic:pf"), ("ic:rel", "ic:pf"), ("ic:cb", "ic:pf"), ("ic:rad", "ic:pf"), ("ic:dense", "ic:pf"),
         ("ic:rel+cb", "best rho"), ("ic:pf", "best rho")]
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        v = {k: {i: tm[(pb, T, i, k)] for i in ii} for k in keys if not k.startswith("rhodpb")}
        v["best rho"] = {i: min(tm[(pb, T, i, k)] for k in rhok) for i in ii}
        cell = {}
        for num, den in PAIRS:
            r = [v[num][i] / v[den][i] for i in ii]
            cell[f"{num}/{den}"] = dict(median=st.median(r), per_instance=r)
        cell["best_rho_key"] = {str(i): min(rhok, key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["primary_lower_4_of_4"] = all(x < 1 for x in cell["ic:rel+cb/ic:pf"]["per_instance"])
        d = cell["ic:rel+cb/best rho"]["per_instance"]
        cell["ic_ahead_4_of_4"] = all(x <= 1 for x in d)
        cell["ic_behind_4_of_4"] = all(x > 1 for x in d)
        out["cells"][f"{pb}:{T}"] = cell
cells = out["cells"].values()
out["gates"] = dict(
    correctness="pass" if not bad and not ic_mismatch else "FAIL",
    primary_ic="lower" if all(c["primary_lower_4_of_4"] for c in cells) else
               ("not lower" if all(all(x >= 1 for x in c["ic:rel+cb/ic:pf"]["per_instance"]) for c in cells) else "mixed"),
    distance_to_rho={c: v["ic:rel+cb/best rho"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells=sum(c["ic_ahead_4_of_4"] for c in cells), ic_behind_cells=sum(c["ic_behind_4_of_4"] for c in cells),
    n_cells=len(out["cells"]),
    lever_medians={p: {c: v[f"{p[0]}/{p[1]}"]["median"] for c, v in out["cells"].items()}
                   for p in [("ic:rel", "ic:pf"), ("ic:cb", "ic:pf"), ("ic:rad", "ic:pf"), ("ic:dense", "ic:pf")]})
out["gates"]["lever_medians"] = {f"{a}/{b}": v for (a, b), v in out["gates"]["lever_medians"].items()}
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
