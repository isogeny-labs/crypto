#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, sys, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# every IC key keeps the table, the trials and the relations; rho's counted fields agree between z0 and z1
CNT_IC = ("table", "trials", "relations", "p1_trials")
CNT_RHO = ("I", "M", "S", "walks", "db")
same = collections.defaultdict(set)
for r in rows:
    if "error" in r:
        continue
    if r["arm"] == "ic5b":
        same[(r["pb"], r["T"], r["idx"], "ic")].add(tuple(r[c] for c in CNT_IC))
    else:
        same[(r["pb"], r["T"], r["idx"], r["key"].rsplit(":z", 1)[0])].add(tuple(r[c] for c in CNT_RHO))
count_mismatch = sorted(str(k) for k, v in same.items() if len(v) != 1)
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["T"], r["idx"], r["key"])].append(r["ms_arm"])
tm = {k: st.median(v) for k, v in t.items()}
cfgs = sorted({(pb, T, i) for pb, T, i, _ in tm})
keys = sorted({k for *_, k in tm})
fam = {"rho z0": [k for k in keys if k.startswith("rhodpb") and k.endswith(":z0")],
       "rho z1": [k for k in keys if k.startswith("rhodpb") and k.endswith(":z1")]}
fam["rho"] = fam["rho z0"] + fam["rho z1"]
for k in ("ic:pf", "ic:ib", "ic:z", "ic:z+ib"):
    fam[k] = [k]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad), count_mismatch=count_mismatch),
           cells={})
PAIRS = [("ic:z+ib", "ic:pf"), ("ic:ib", "ic:pf"), ("ic:z", "ic:pf"), ("rho z1", "rho z0"),
         ("ic:z+ib", "rho"), ("ic:pf", "rho z0")]
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        best = {f: {i: min(tm[(pb, T, i, k)] for k in ks) for i in ii} for f, ks in fam.items()}
        cell = {}
        for num, den in PAIRS:
            r = [best[num][i] / best[den][i] for i in ii]
            cell[f"{num}/{den}"] = dict(median=st.median(r), per_instance=r)
        cell["best_rho_key"] = {str(i): min(fam["rho"], key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["primary_lower_4_of_4"] = all(x < 1 for x in cell["ic:z+ib/ic:pf"]["per_instance"])
        d = cell["ic:z+ib/rho"]["per_instance"]
        cell["ic_ahead_4_of_4"] = all(x <= 1 for x in d)
        cell["ic_behind_4_of_4"] = all(x > 1 for x in d)
        out["cells"][f"{pb}:{T}"] = cell
c36 = [v for c, v in out["cells"].items() if c.startswith("36:")]
out["gates"] = dict(
    correctness="pass" if not bad and not count_mismatch else "FAIL",
    primary_2p36="lower" if all(c["primary_lower_4_of_4"] for c in c36) else
                 ("not lower" if all(all(x >= 1 for x in c["ic:z+ib/ic:pf"]["per_instance"]) for c in c36) else "mixed"),
    ic_gain={c: v["ic:z+ib/ic:pf"]["median"] for c, v in out["cells"].items()},
    ib_gain={c: v["ic:ib/ic:pf"]["median"] for c, v in out["cells"].items()},
    z_gain_ic={c: v["ic:z/ic:pf"]["median"] for c, v in out["cells"].items()},
    z_gain_rho={c: v["rho z1/rho z0"]["median"] for c, v in out["cells"].items()},
    distance_to_rho={c: v["ic:z+ib/rho"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells=[c for c, v in out["cells"].items() if v["ic_ahead_4_of_4"]],
    ic_behind_cells=[c for c, v in out["cells"].items() if v["ic_behind_4_of_4"]])
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
