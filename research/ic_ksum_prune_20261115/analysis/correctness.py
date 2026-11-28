#!/usr/bin/env python3
"""Per-target correctness check of the -p arms (PROTOCOL.md section 3, 'Correctness').
usage: correctness.py <sessions dir> <out.json>
For every session, round (warm-up included) and target, X-p must equal X on: outcome (status, recovered), counters,
every phase's `native` and `work_native` dicts, `factor_base`, the oracle_setup phase's doubles/scalar_mults, and the
work-model primitives except oracle_setup group_add; and oracle_setup adds(X-p) <= adds(X). Anything else is a mismatch."""
import gzip, json, sys, glob, os
sess, out = sys.argv[1:3]
rep = {"sessions": {}, "pairs_checked": 0, "mismatches": [], "adds_saved": {}}
def key(d): return (d["round"], d["workload"]["target_index"])
def ph(d): return {p["name"]: p for p in d["phases"]}
for sd in sorted(glob.glob(os.path.join(sess, "*-r[0-9]"))):
    name = os.path.basename(sd); recs = {}
    f = os.path.join(sd, "records.jsonl"); fh = open(f) if os.path.exists(f) else gzip.open(f + ".gz", "rt")
    for l in fh:
        d = json.loads(l); recs[(d["arm"],) + key(d)] = d
    n = 0
    for k in (4, 5):
        u, p = f"ksum{k}", f"ksum{k}-p"
        tot_u = tot_p = 0
        for kk in sorted(x for x in recs if x[0] == u):
            a, b = recs[kk], recs.get((p,) + kk[1:])
            if b is None: rep["mismatches"].append([name, u, kk[1:], "missing -p record"]); continue
            n += 1; bad = []
            if (a["outcome"]["status"], a["outcome"]["recovered"]) != (b["outcome"]["status"], b["outcome"]["recovered"]): bad.append("outcome")
            if a["counters"] != b["counters"]: bad.append("counters")
            if a["factor_base"] != b["factor_base"]: bad.append("factor_base")
            pa, pb = ph(a), ph(b)
            if list(pa) != list(pb): bad.append("phase list")
            for nm in pa:
                for fld in ("native", "work_native"):
                    if pa[nm].get(fld) != pb[nm].get(fld): bad.append(f"{nm}.{fld}")
                if nm != "oracle_setup":
                    for fld in ("adds", "doubles", "scalar_mults", "gae"):
                        if pa[nm][fld] != pb[nm][fld]: bad.append(f"{nm}.{fld}")
                else:
                    for fld in ("doubles", "scalar_mults"):
                        if pa[nm][fld] != pb[nm][fld]: bad.append(f"{nm}.{fld}")
                    if pb[nm]["adds"] > pa[nm]["adds"]: bad.append("oracle_setup.adds increased")
            wa = {q["name"]: q["prims"] for q in a["work"]["phases"]}; wb = {q["name"]: q["prims"] for q in b["work"]["phases"]}
            for nm in wa:
                ya, yb = dict(wa[nm]), dict(wb[nm])
                if nm == "oracle_setup": ya.pop("group_add", None); yb.pop("group_add", None)
                if ya != yb: bad.append(f"work.{nm}")
            tot_u += pa["oracle_setup"]["adds"]; tot_p += pb["oracle_setup"]["adds"]
            if bad: rep["mismatches"].append([name, u, list(kk[1:]), bad])
        rep["adds_saved"].setdefault(f"{name}:{u}", [tot_u, tot_p])
    rep["sessions"][name] = n; rep["pairs_checked"] += n
rep["ok"] = not rep["mismatches"]
json.dump(rep, open(out, "w"), indent=1); print(json.dumps({k: rep[k] for k in ("pairs_checked", "ok")}), len(rep["mismatches"]), "mismatches")
