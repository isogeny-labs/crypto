#!/usr/bin/env python3
"""Sensitivity (NOT a registered or repository reading): U_all = U_pre + every
work_native operation counter (all keys except 'counted:*' markers) on the
IC arms, at w. Rho/BSGS records carry no work_native, so they are unchanged.
Reports pooled ksum*/inc-sel ratios and per-curve order rho < ksum5 < inc-sel."""
import json, math, sys
from collections import defaultdict
from pathlib import Path
SESS = Path(sys.argv[1])
def Upre(r):
    t = 0
    for name in r["cost"]["unpriced"]:
        base = name[:-10] if name.endswith("_uncharged") else name
        if name in r.get("counters", {}): t += r["counters"][name]; continue
        t += sum((ph.get("native") or {}).get(name, (ph.get("native") or {}).get(base, 0)) for ph in r["phases"])
    return t
def Wall(r):
    return sum(v for ph in r["phases"] for k, v in (ph.get("work_native") or {}).items() if not k.startswith("counted:"))
recs = defaultdict(dict); percurve = defaultdict(list)
for b in (20, 24, 28, 32):
    for line in open(SESS / f"holdout-P{b}" / "records.jsonl"):
        r = json.loads(line)
        if r["warmup"] or r["outcome"]["status"] != "verified": continue
        x = (r["cost"]["total_gae"], Upre(r) + Wall(r), math.sqrt(r["workload"]["curve"]["r"]))
        recs[(b, r["arm"])][(r["workload"]["workload_id"], r["round"])] = x
        percurve[(r["workload"]["curve"]["slug"], r["arm"])].append(x)
S = lambda x, w: (x[0] + w * x[1]) / x[2]
out = {}
for b in (20, 24, 28, 32):
    for B in ("ksum5-sel", "ksum4-sel"):
        A = "inc-sel"; ks = sorted(set(recs[(b, A)]) & set(recs[(b, B)]))
        out[f"P{b}|{B}/inc-sel"] = {str(w): sum(S(recs[(b, B)][k], w) for k in ks) / sum(S(recs[(b, A)][k], w) for k in ks) for w in (0, 0.1, 1)}
    ks = sorted(set(recs[(b, "rho-neg")]) & set(recs[(b, "ksum5-sel")]))
    out[f"P{b}|ksum5-sel/rho-neg"] = {str(w): sum(S(recs[(b, "ksum5-sel")][k], w) for k in ks) / sum(S(recs[(b, "rho-neg")][k], w) for k in ks) for w in (0, 0.1, 1)}
order = {}
for slug in sorted({k[0] for k in percurve}):
    m = lambda arm, w: sum(S(x, w) for x in percurve[(slug, arm)]) / len(percurve[(slug, arm)])
    order[slug] = all(m("rho-neg", w) < m("ksum5-sel", w) < m("inc-sel", w) for w in (0, 0.1, 1))
out["order_rho_lt_ksum5_lt_inc_all_curves_all_weights"] = all(order.values())
print(json.dumps(out, indent=1))
json.dump(out, open(Path(sys.argv[2]), "w"), indent=1)
