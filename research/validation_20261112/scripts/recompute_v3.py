#!/usr/bin/env python3
"""Target 1 recompute (validator tool): V3.md table, ratios at 0/0.1/1 GAE,
record metrics, v2 comparison claims, certificate/row consistency.
Usage: recompute_v3.py EXPORT_ROOT OUT.json"""
import json, sys, os
root = sys.argv[1]; D = os.path.join(root, "research/chained_prime_transfer_20261111")
v3 = json.load(open(os.path.join(D, "results_v3.json")))
v2 = json.load(open(os.path.join(D, "results_v2.json")))
rec = json.load(open(os.path.join(root, "benchmarks/records/20261112T0341Z-chained-prime-transfer-v3.json")))
certs = [json.loads(l) for l in open(os.path.join(D, "certs_v3.jsonl")) if l.strip()]
out = {"rows": [], "findings": []}
# V3.md table as committed (validator transcribed from V3.md lines of the v3 table)
claimed = {211: (8.0, 11.4, 42.2), 419: (23.7, 38.3, 169.3), 821: (30.7, 75.2, 476.2), 1619: (40.8, 112.0, 753.5)}
for r in v3["rows"]:
    p = r["p"]; rho = r["rho_src_ops"]; ic = r["ic_jac_group_ops"]; la = r["ic_jac_la_mulmods"]
    ratios = [(ic + w * la) / rho for w in (0, 0.1, 1)]
    ok = all(abs(round(x, 1) - c) < 1e-9 for x, c in zip(ratios, claimed[p]))
    la_eq = la == r["ic_jac_fb"] * r["ic_jac_relations"] if "ic_jac_relations" in r else None
    row = {"p": p, "r": r["r"], "rho_e1": rho, "ic_ops": ic, "la_mulmods": la, "ratio_w0": ratios[0], "ratio_w0p1": ratios[1],
           "ratio_w1": ratios[2], "v3md_ratios": claimed[p], "v3md_ratios_match": ok,
           "la_eq_fb_times_relations": la_eq, "fb": r["ic_jac_fb"], "relations": r.get("ic_jac_relations"),
           "ic_log_eq_x": r["ic_claimed_log"] == r["x"], "ic_recovered_ok": r["ic_recovered_ok"],
           "rho_recovered_ok": r["rho_recovered_ok"], "chain_verified": r["chain_verified"],
           "ic_above_rho_all_w": all(x > 1 for x in ratios)}
    if not ok: out["findings"].append(f"V3.md ratio mismatch at p={p}: {ratios} vs {claimed[p]}")
    out["rows"].append(row)
# v2 comparison
v2r = {r["p"]: r for r in (v2["rows"] if isinstance(v2, dict) else v2)}
cmp = []
for r in v3["rows"]:
    a = v2r.get(r["p"])
    if a is None: cmp.append({"p": r["p"], "v2": None}); continue
    cmp.append({"p": r["p"], "same_x": a.get("x") == r["x"], "same_candidate": a.get("candidate") == r["candidate"],
                "rho_src_same": a.get("rho_src_ops") == r["rho_src_ops"], "rho_isog_same": a.get("rho_isog_ops") == r["rho_isog_ops"],
                "la_same": a.get("ic_jac_la_mulmods") == r["ic_jac_la_mulmods"],
                "v2_rho_jac": a.get("rho_jac_ops"), "v3_rho_jac": r["rho_jac_ops"],
                "v2_ic_ops": a.get("ic_jac_group_ops"), "v3_ic_ops": r["ic_jac_group_ops"],
                "v2_ratio_w0": a.get("ic_jac_group_ops") / a.get("rho_src_ops") if a.get("ic_jac_group_ops") else None})
out["v2_comparison"] = cmp
# record metrics
rows = {r["p"]: r for r in v3["rows"]}
mp = {"rho_e1_ops": "rho_src_ops", "rho_e1prime_ops": "rho_isog_ops", "rho_jc_ops": "rho_jac_ops", "ic_jc_group_ops": "ic_jac_group_ops", "ic_jc_la_mulmods": "ic_jac_la_mulmods"}
mm = []
for m in rec["metrics"]:
    base, p = m["name"].rsplit("_p", 1); p = int(p)
    want = rows[p][mp[base]]
    if m["value"] != want: mm.append((m["name"], m["value"], want))
    if m.get("reference") is not None and m["reference"] != rows[p]["rho_src_ops"]: mm.append((m["name"], "reference", m["reference"]))
out["record_metrics"] = {"n": len(rec["metrics"]), "mismatches": mm}
if mm: out["findings"].append(f"record metric mismatches: {mm}")
# certificates vs rows
cs = {}
for c in certs:
    r = rows[c["p"]]
    arm_ops = {"rho_e1": r["rho_src_ops"], "rho_e1prime": r["rho_isog_ops"], "rho_jc": r["rho_jac_ops"], "ic_jc": r["ic_jac_group_ops"]}[c["arm"]]
    cs.setdefault(c["p"], []).append({"arm": c["arm"], "k_eq_x": c["claimed_k"] == r["x"], "solver_ops_eq_row": c["solver_ops"] == arm_ops})
out["certs_vs_rows"] = cs
bad = [(p, x) for p, l in cs.items() for x in l if not (x["k_eq_x"] and x["solver_ops_eq_row"])]
if bad: out["findings"].append(f"cert/row mismatch {bad}")
out["skips"] = {"tried": v3["candidates_tried"], "by_reason": v3["candidates_skipped_by_reason"]}
for p, n in v3["candidates_tried"].items():
    sk = sum(v3["candidates_skipped_by_reason"].get(p, {}).values())
    if sk + 1 != n: out["findings"].append(f"p={p}: tried {n} != skipped {sk} + 1 row")
json.dump(out, open(sys.argv[2], "w"), indent=1)
for r in out["rows"]: print(r["p"], round(r["ratio_w0"], 3), round(r["ratio_w0p1"], 3), round(r["ratio_w1"], 3), r["v3md_ratios_match"], "LA=fb*rel:", r["la_eq_fb_times_relations"], r["fb"], r["relations"])
for c in cmp: print("v2cmp", c)
print("record metrics", out["record_metrics"]["n"], "mismatches", len(mm))
print("findings:", out["findings"])
