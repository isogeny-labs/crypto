#!/usr/bin/env python3
"""Frozen analysis of PROTOCOL.md: python3 analyze.py [runs.jsonl] -> summary.json.
Cost of a key on an instance = mean of ms_arm over its 32 seeds (one target each)."""
import json, os, sys, statistics as st, collections, math
HERE = os.path.dirname(os.path.abspath(__file__))
src = sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "main", "runs.jsonl")
rows = [json.loads(l) for l in open(src)]
bad = [r for r in rows if "error" in r or r.get("ok") != r.get("targets")]
t = collections.defaultdict(list)
for r in rows:
    if r not in bad: t[(r["pb"], r["idx"], r["key"])].append(r["ms_arm"])
mean = {k: st.mean(v) for k, v in t.items()}
cnt = {k: len(v) for k, v in t.items()}
keys = sorted({k for _, _, k in mean})
ick = [k for k in keys if k.startswith("ic:")]; rhok = [k for k in keys if k.startswith("rho:st:")]
out = dict(correctness=dict(executions=len(rows), failed_or_unverified=len(bad),
                            short_keys=sorted(f"{k}" for k, c in cnt.items() if c != 32)), cells={})
for pb in sorted({k[0] for k in mean}):
    ii = sorted({k[1] for k in mean if k[0] == pb})
    best_ic = {i: min(ick, key=lambda k: mean[(pb, i, k)]) for i in ii}
    best_rho = {i: min(rhok, key=lambda k: mean[(pb, i, k)]) for i in ii}
    r = [mean[(pb, i, best_ic[i])] / mean[(pb, i, best_rho[i])] for i in ii]
    rs = [mean[(pb, i, "rho:serial")] / mean[(pb, i, best_rho[i])] for i in ii]
    out["cells"][str(pb)] = dict(
        ic_over_rho=dict(median=st.median(r), per_instance=r),
        serial_rho_over_batched_rho=dict(median=st.median(rs), per_instance=rs),
        best_ic_key={str(i): best_ic[i] for i in ii}, best_rho_key={str(i): best_rho[i] for i in ii},
        best_ic_mean_ms={str(i): mean[(pb, i, best_ic[i])] for i in ii},
        best_rho_mean_ms={str(i): mean[(pb, i, best_rho[i])] for i in ii},
        ic_mu_means={k: st.median(mean[(pb, i, k)] for i in ii) for k in ick},
        ic_ahead_4_of_4=all(x <= 1 for x in r), ic_behind_4_of_4=all(x > 1 for x in r))
c = out["cells"]; m = {pb: c[pb]["ic_over_rho"]["median"] for pb in c}
grow = all(m[a] < m[b] for a, b in zip(sorted(m, key=int), sorted(m, key=int)[1:]))
shrink = all(m[a] > m[b] for a, b in zip(sorted(m, key=int), sorted(m, key=int)[1:]))
out["gates"] = dict(correctness="pass" if not bad and not out["correctness"]["short_keys"] else "FAIL",
                    primary="gap grows" if grow else ("gap shrinks" if shrink else "mixed"),
                    ic_over_rho=m, ic_behind_cells=[pb for pb in c if c[pb]["ic_behind_4_of_4"]],
                    ic_ahead_cells=[pb for pb in c if c[pb]["ic_ahead_4_of_4"]])
json.dump(out, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(out["gates"], indent=1))
