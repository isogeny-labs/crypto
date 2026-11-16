#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "scale", "runs.jsonl"))]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["T"], r["idx"], r["key"])].append(r["ms_arm"])
tm = {k: st.median(v) for k, v in t.items()}
cfgs = sorted({(pb, T, i) for pb, T, i, _ in tm})
rho = sorted({k for *_, k in tm if k.startswith("rho")})
ics = ["ic:4:0.7", "ic:4:0.85", "ic:4:1.0"]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad)), cells={})
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        br = {i: min(tm[(pb, T, i, k)] for k in rho) for i in ii}
        cell = dict(instances=ii, best_rho_key={str(i): min(rho, key=lambda k: tm[(pb, T, i, k)]) for i in ii},
                    best_rho_ms={str(i): br[i] for i in ii})
        for k in ics:
            r = [tm[(pb, T, i, k)] / br[i] for i in ii]
            cell[f"{k}/best_rho"] = dict(median=st.median(r), per_instance=r)
        for k in ("ic:4:0.7", "ic:4:1.0"):
            r = [tm[(pb, T, i, "ic:4:0.85")] / tm[(pb, T, i, k)] for i in ii]
            cell[f"ic:4:0.85/{k}"] = dict(median=st.median(r), per_instance=r)
        cell["ic_ahead_4_of_4"] = all(x <= 1 for x in cell["ic:4:0.85/best_rho"]["per_instance"])
        cell["mu_0.85_best_of_3_median"] = min(ics, key=lambda k: st.median(tm[(pb, T, i, k)] / br[i] for i in ii)) == "ic:4:0.85"
        out["cells"][f"{pb}:{T}"] = cell
d = {T: (out["cells"][f"36:{T}"]["ic:4:0.85/best_rho"]["median"], out["cells"][f"32:{T}"]["ic:4:0.85/best_rho"]["median"])
     for T in (1024, 4096, 16384)}
out["gates"] = dict(
    correctness="pass" if not bad else "FAIL",
    primary="gap grows" if all(a > b for a, b in d.values()) else ("gap shrinks" if all(a < b for a, b in d.values()) else "mixed"),
    ic_over_best_rho={c: v["ic:4:0.85/best_rho"]["median"] for c, v in out["cells"].items()},
    mu_0p85_best_cells=sum(v["mu_0.85_best_of_3_median"] for v in out["cells"].values()), n_cells=len(out["cells"]))
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
