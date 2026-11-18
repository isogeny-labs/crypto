#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, sys, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# huge pages change only the paging: every counted field of an arm must agree between hp0 and hp1
CNT = {"ic5b": ("table", "trials", "relations", "p1_trials", "I", "M", "S"), "rhodpb": ("I", "M", "S", "walks", "db")}
same = collections.defaultdict(set)
for r in rows:
    if "error" not in r:
        base = r["key"].rsplit(":hp", 1)[0]
        same[(r["pb"], r["T"], r["idx"], base)].add(tuple(r.get(c) for c in CNT[r["arm"]]))
count_mismatch = sorted(str(k) for k, v in same.items() if len(v) != 1)
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["T"], r["idx"], r["key"])].append(r["ms_arm"])
tm = {k: st.median(v) for k, v in t.items()}
cfgs = sorted({(pb, T, i) for pb, T, i, _ in tm})
keys = sorted({k for *_, k in tm})
fam = {"ic hp0": ["ic:rel:hp0"], "ic hp1": ["ic:rel:hp1"],
       "rho hp0": [k for k in keys if k.startswith("rhodpb") and k.endswith(":hp0")],
       "rho hp1": [k for k in keys if k.startswith("rhodpb") and k.endswith(":hp1")]}
fam["rho"] = fam["rho hp0"] + fam["rho hp1"]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad), count_mismatch=count_mismatch),
           cells={})
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        best = {f: {i: min(tm[(pb, T, i, k)] for k in ks) for i in ii} for f, ks in fam.items()}
        cell = {}
        for num, den in [("ic hp1", "ic hp0"), ("rho hp1", "rho hp0"), ("ic hp1", "rho"), ("ic hp0", "rho hp0")]:
            r = [best[num][i] / best[den][i] for i in ii]
            cell[f"{num}/{den}"] = dict(median=st.median(r), per_instance=r)
        cell["best_rho_key"] = {str(i): min(fam["rho"], key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["ic_hp_lower_4_of_4"] = all(x < 1 for x in cell["ic hp1/ic hp0"]["per_instance"])
        d = cell["ic hp1/rho"]["per_instance"]
        cell["ic_ahead_4_of_4"] = all(x <= 1 for x in d)
        cell["ic_behind_4_of_4"] = all(x > 1 for x in d)
        out["cells"][f"{pb}:{T}"] = cell
c36 = [v for c, v in out["cells"].items() if c.startswith("36:")]
out["gates"] = dict(
    correctness="pass" if not bad and not count_mismatch else "FAIL",
    primary_2p36="lower" if all(c["ic_hp_lower_4_of_4"] for c in c36) else
                 ("not lower" if all(all(x >= 1 for x in c["ic hp1/ic hp0"]["per_instance"]) for c in c36) else "mixed"),
    ic_hp_gain={c: v["ic hp1/ic hp0"]["median"] for c, v in out["cells"].items()},
    rho_hp_gain={c: v["rho hp1/rho hp0"]["median"] for c, v in out["cells"].items()},
    distance_to_rho={c: v["ic hp1/rho"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells=[c for c, v in out["cells"].items() if v["ic_ahead_4_of_4"]],
    ic_behind_cells=[c for c, v in out["cells"].items() if v["ic_behind_4_of_4"]])
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
