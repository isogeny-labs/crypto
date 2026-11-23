#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, sys, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# ic:bi, ic:pf and ic:pf+fp (all mu = 0.85) must make identical tables, trials and counted operations
CNT = ("table", "trials", "relations", "p1_trials", "I", "M", "S")
same = collections.defaultdict(set)
for r in rows:
    if r.get("key") in ("ic:bi", "ic:pf", "ic:pf+fp") and "error" not in r:
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
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        v = {k: {i: tm[(pb, T, i, k)] for i in ii} for k in keys if not k.startswith("rhodpb")}
        v["best rho"] = {i: min(tm[(pb, T, i, k)] for k in rhok) for i in ii}
        cell = {}
        for num, den in [("ic:pf+fp", "ic:bi"), ("ic:pf", "ic:bi"), ("ic:pf+fp", "ic:pf"),
                         ("ic:pf+fp:mu1.0", "ic:pf+fp"), ("ic:pf+fp", "best rho"), ("ic:bi", "best rho"),
                         ("ic:pf+fp:mu1.0", "best rho")]:
            r = [v[num][i] / v[den][i] for i in ii]
            cell[f"{num}/{den}"] = dict(median=st.median(r), per_instance=r)
        cell["best_rho_key"] = {str(i): min(rhok, key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["primary_lower_4_of_4"] = all(x < 1 for x in cell["ic:pf+fp/ic:bi"]["per_instance"])
        cell["ic_ahead_of_rho_4_of_4"] = all(x <= 1 for x in cell["ic:pf+fp/best rho"]["per_instance"])
        cell["ic_behind_rho_4_of_4"] = all(x > 1 for x in cell["ic:pf+fp/best rho"]["per_instance"])
        out["cells"][f"{pb}:{T}"] = cell
cells = out["cells"].values()
out["gates"] = dict(
    correctness="pass" if not bad and not ic_mismatch else "FAIL",
    primary_ic="lower" if all(c["primary_lower_4_of_4"] for c in cells) else
               ("not lower" if all(all(x >= 1 for x in c["ic:pf+fp/ic:bi"]["per_instance"]) for c in cells) else "mixed"),
    distance_to_rho={c: v["ic:pf+fp/best rho"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells=sum(c["ic_ahead_of_rho_4_of_4"] for c in cells),
    ic_behind_cells=sum(c["ic_behind_rho_4_of_4"] for c in cells), n_cells=len(out["cells"]),
    mu1_lower_cells=sum(c["ic:pf+fp:mu1.0/ic:pf+fp"]["median"] < 1 for c in cells))
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
