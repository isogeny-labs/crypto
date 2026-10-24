#!/usr/bin/env python3
"""runs/manifest.json: PLAN 3 order, per-session estimate from scripts/costmodel.json, total, guard and reduction record.
Order: V5 (D, Ua, Ub, N, ctl, Tcr, M), then V6 M, then V6 D, Ua, Ub, S, N, ctl; inside a family by level and chunk."""
import json, os, sys
STUDY = os.environ.get("STUDY", "/home/user/crypto/research/volcano_ecdlp_census_followup_20261023")
cm = json.load(open(f"{STUDY}/scripts/costmodel.json"))
rows = [json.loads(l) for f in ("manifest_std.lines", "manifest_new.lines") for l in open(f"{STUDY}/runs/{f}")]
for r in rows:
    r.setdefault("cheap_arms", r["arms"] - 1); r.setdefault("ic_arms", 1)
    pp = cm["K"] * cm["f"][r["V"]] * (cm["cheap_s"] * r["cheap_arms"] + cm["ic_s"] * r["ic_arms"])
    r["est_s"] = round(15 + r["passes"] * pp, 1)
    r["spec"] = f"specs/{r['name']}.json"; r["out"] = f"sessions/{r['name']}"
fam5 = ["D", "Ua", "Ub", "N", "ctl", "Tcr", "M"]; fam6 = ["D", "Ua", "Ub", "S", "N", "ctl"]
def key(r):
    if r["V"] == "V5": return (0, fam5.index(r["family"]), r["level"], r["chunk"])
    if r["family"] == "M": return (1, 0, r["level"], r["chunk"])
    return (2, fam6.index(r["family"]), r["level"], r["chunk"])
rows.sort(key=key)
for i, r in enumerate(rows): r["seq"] = i
tot = sum(r["est_s"] for r in rows)
def tab(f):
    out = {}
    for r in rows:
        k = f(r); a = out.setdefault(k, {"sessions": 0, "specs": 0, "est_s": 0.0}); a["sessions"] += 1; a["specs"] += 1; a["est_s"] = round(a["est_s"] + r["est_s"], 1)
    return out
m = {"schema": "volcano-census-followup.manifest/v1", "order": "V5 (D, Ua, Ub, N, ctl, Tcr, M) -> V6 M -> V6 (D, Ua, Ub, S, N, ctl); inside a family by level, chunk",
     "cost_model": "scripts/costmodel.json: est_s = 15 + passes * K * f(V) * (cheap_s * cheap_arms + ic_s * ic_arms)",
     "max_session_s_target": cm["session_cap_s"], "max_session_est_s": max(r["est_s"] for r in rows),
     "ctl_subset": os.environ.get("CTL_SUBSET", "0") == "1",
     "total_est_s": round(tot, 1), "total_est_h": round(tot / 3600, 3), "budget_h": 20,
     "by_V_family": {f"{k[0]}-{k[1]}": v for k, v in tab(lambda r: (r["V"], r["family"])).items()},
     "sessions": rows}
json.dump(m, open(f"{STUDY}/runs/manifest.json", "w"), indent=1)
print(f"sessions {len(rows)}  total {tot/3600:.2f} h  longest {m['max_session_est_s']:.0f} s")
for k, v in m["by_V_family"].items(): print(k, v)
