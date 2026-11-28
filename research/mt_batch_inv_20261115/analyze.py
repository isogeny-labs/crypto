#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py -> summary.json.
Time of a key in a configuration = median over its repeats of ms_arm (the whole solve)."""
import json, os, statistics as st, collections
HERE = os.path.dirname(os.path.abspath(__file__))
rows = [json.loads(l) for l in open(os.path.join(HERE, "results", "main", "runs.jsonl"))]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
# IC-bi must make exactly IC-plain's trials and table (same configuration, any repeat)
same = collections.defaultdict(set)
for r in rows:
    if r.get("arm") == "ic5b" and "error" not in r:
        same[(r["pb"], r["T"], r["idx"])].add((r["table"], r["trials"], r["relations"], r["p1_trials"]))
ic_mismatch = sorted(f"{k}" for k, v in same.items() if len(v) != 1)
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["T"], r["idx"], r["key"])].append(r["ms_arm"])
tm = {k: st.median(v) for k, v in t.items()}
cfgs = sorted({(pb, T, i) for pb, T, i, _ in tm})
keys = sorted({k for *_, k in tm})
fam = {"batched rho": [k for k in keys if k.startswith("rhodpb")],
       "unbatched rho": [k for k in keys if k.startswith(("rhodpup", "rho2xksp"))]}
fam["all rho"] = fam["batched rho"] + fam["unbatched rho"]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad), ic_bi_vs_plain_mismatch=ic_mismatch),
           cells={})
for pb in sorted({c[0] for c in cfgs}):
    for T in sorted({c[1] for c in cfgs if c[0] == pb}):
        ii = [i for p, TT, i in cfgs if p == pb and TT == T]
        best = {f: {i: min(tm[(pb, T, i, k)] for k in ks) for i in ii} for f, ks in fam.items()}
        def ratio(num, den): return [num[i] / den[i] for i in ii]
        icb = {i: tm[(pb, T, i, "ic:bi")] for i in ii}; icp = {i: tm[(pb, T, i, "ic:plain")] for i in ii}
        cell = {}
        for name, num, den in [("ic_bi/ic_plain", icb, icp), ("ic_bi/best_batched_rho", icb, best["batched rho"]),
                               ("ic_bi/best_rho", icb, best["all rho"]), ("ic_plain/best_unbatched_rho", icp, best["unbatched rho"]),
                               ("best_batched_rho/best_unbatched_rho", best["batched rho"], best["unbatched rho"])]:
            r = ratio(num, den); cell[name] = dict(median=st.median(r), per_instance=r)
        cell["best_rho_key"] = {str(i): min(fam["all rho"], key=lambda k: tm[(pb, T, i, k)]) for i in ii}
        cell["ic_bi_lower_4_of_4"] = all(x < 1 for x in cell["ic_bi/ic_plain"]["per_instance"])
        cell["ic_ahead_of_rho_4_of_4"] = all(x <= 1 for x in cell["ic_bi/best_rho"]["per_instance"])
        out["cells"][f"{pb}:{T}"] = cell
cells = out["cells"].values()
out["gates"] = dict(
    correctness="pass" if not bad and not ic_mismatch else "FAIL",
    primary_ic="lower" if all(c["ic_bi_lower_4_of_4"] for c in cells) else
               ("not lower" if all(all(x >= 1 for x in c["ic_bi/ic_plain"]["per_instance"]) for c in cells) else "mixed"),
    distance_to_rho={c: v["ic_bi/best_rho"]["median"] for c, v in out["cells"].items()},
    rho_batching_gain={c: v["best_batched_rho/best_unbatched_rho"]["median"] for c, v in out["cells"].items()},
    ic_ahead_cells=sum(c["ic_ahead_of_rho_4_of_4"] for c in cells), n_cells=len(out["cells"]))
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
