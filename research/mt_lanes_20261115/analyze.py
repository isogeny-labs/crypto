#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, sys, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# the IC batch size changes only the grouping of inversions: table, trials, relations and p1_trials must agree
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
fam = {"IC (all B)": [k for k in keys if k.startswith("ic:")], "IC B32": ["ic:B32"],
       "rho (all lanes)": [k for k in keys if k.startswith("rhodpb")],
       "rho L32": [k for k in keys if k.startswith("rhodpb:L32:")]}
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad), ic_count_mismatch=ic_mismatch),
           cells={})
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        best = {f: {i: min(tm[(pb, T, i, k)] for k in ks) for i in ii} for f, ks in fam.items()}
        cell = {}
        for num, den in [("IC (all B)", "rho (all lanes)"), ("IC B32", "rho L32"),
                         ("rho (all lanes)", "rho L32"), ("IC (all B)", "IC B32")]:
            r = [best[num][i] / best[den][i] for i in ii]
            cell[f"{num}/{den}"] = dict(median=st.median(r), per_instance=r)
        cell["best_ic_key"] = {str(i): min(fam["IC (all B)"], key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["best_rho_key"] = {str(i): min(fam["rho (all lanes)"], key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        r = cell["IC (all B)/rho (all lanes)"]["per_instance"]
        cell["ic_ahead_4_of_4"] = all(x <= 1 for x in r)
        cell["ic_behind_4_of_4"] = all(x > 1 for x in r)
        out["cells"][f"{pb}:{T}"] = cell
cells = out["cells"].values()
out["gates"] = dict(
    correctness="pass" if not bad and not ic_mismatch else "FAIL",
    distance_tuned={c: v["IC (all B)/rho (all lanes)"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells_tuned=sum(c["ic_ahead_4_of_4"] for c in cells),
    ic_behind_cells_tuned=sum(c["ic_behind_4_of_4"] for c in cells), n_cells=len(out["cells"]),
    rho_lane_gain={c: v["rho (all lanes)/rho L32"]["median"] for c, v in out["cells"].items()},
    ic_batch_gain={c: v["IC (all B)/IC B32"]["median"] for c, v in out["cells"].items()})
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
