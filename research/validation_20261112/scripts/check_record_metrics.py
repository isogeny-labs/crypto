#!/usr/bin/env python3
"""Compare every metric value in the four bench records with the committed raw data
(targets 1-3) or the validator's replay-derived values (target 4). Validator tool.
Usage: check_record_metrics.py REPO_EXPORT_ROOT [small_de_replay.json]"""
import json, sys, os, re, statistics as st, math
root = sys.argv[1]
R = lambda p: json.load(open(os.path.join(root, p)))
res = {}
# target 1
rec = R("benchmarks/records/20261112T0341Z-chained-prime-transfer-v3.json")
rows = {r["p"]: r for r in R("research/chained_prime_transfer_20261111/results_v3.json")["rows"]}
m1 = {"rho_e1_ops": "rho_src_ops", "rho_e1prime_ops": "rho_isog_ops", "rho_jc_ops": "rho_jac_ops", "ic_jc_group_ops": "ic_jac_group_ops", "ic_jc_la_mulmods": "ic_jac_la_mulmods"}
bad = []; n = 0
for m in rec["metrics"]:
    base, p = m["name"].rsplit("_p", 1); p = int(p)
    want = rows[p][m1[base]]; n += 1
    if m["value"] != want: bad.append((m["name"], m["value"], want))
    if m.get("reference") is not None and m["reference"] != rows[p]["rho_src_ops"]: bad.append((m["name"], "reference", m["reference"]))
res["v3"] = (n, bad)
# target 2
rec = R("benchmarks/records/20261112T0421Z-chained-tower-large-p.json")
rows = {r["p"]: r for r in R("research/chained_prime_transfer_20261111/tower_large/tower_large.json")["rows"]}
bad = []; n = 0
for m in rec["metrics"]:
    mm = re.match(r"(ic2|ic3|rho_e)_(group_ops|linalg_mulmods)_p(\d+)$", m["name"])
    if not mm: bad.append((m["name"], "unparsed")); continue
    kind, what, p = mm.group(1), mm.group(2), int(mm.group(3)); r = rows[p]; n += 1
    if kind == "rho_e": want = r["rho_ops"]
    else: want = r["ic"][0 if kind == "ic2" else 1]["ops" if what == "group_ops" else "la_mulmods"]
    if abs(m["value"] - want) > 1e-9 * max(1, abs(want)): bad.append((m["name"], m["value"], want))
    if m.get("reference") is not None and abs(m["reference"] - r["rho_ops"]) > 1e-9 * r["rho_ops"]: bad.append((m["name"], "ref", m["reference"]))
res["tower_large"] = (n, bad)
# target 3
rec = R("benchmarks/records/20261112T1550Z-cover-pushforward.json")
prim = {r["p"]: r for r in R("research/cover_pushforward_20261112/pushforward.json")["rows"]}
sup = {r["p"]: r for r in R("research/cover_pushforward_20261112/pushforward_15seeds.json")["rows"]}
bad = []; n = 0
for m in rec["metrics"]:
    nm = m["name"]
    mm = re.match(r"(.*)_p(\d+)$", nm)
    if mm:
        base, p = mm.group(1), int(mm.group(2))
        if base == "ic_jc_group_ops": want = st.median(s["ic_jc"]["group_ops"] for s in prim[p]["seeds"]); ref = st.median(s["rho"]["ops"] for s in prim[p]["seeds"])
        elif base == "ic_e1_two_term_group_ops": want = st.median(s["ic_e1_two_term"]["run"]["group_ops"] for s in prim[p]["seeds"]); ref = st.median(s["rho"]["ops"] for s in prim[p]["seeds"])
        elif base == "ic_e1_fibre_first_collision_group_ops": want = st.median(s["ic_e1_fibre"]["first_collision"]["ops"] for s in sup[p]["seeds"]); ref = st.median(s["rho"]["ops"] for s in sup[p]["seeds"])
        elif base == "birthday_e1_first_collision_group_ops": want = st.median(s["birthday_e1"]["ops"] for s in sup[p]["seeds"]); ref = st.median(s["rho"]["ops"] for s in sup[p]["seeds"])
        elif base == "b_over_e1": want = round(prim[p]["b"]["b_size"] / prim[p]["e1_order"], 4); ref = None
        else: bad.append((nm, "unknown")); continue
        n += 1
        if m["value"] != want: bad.append((nm, m["value"], want))
        if ref is not None and m.get("reference") != ref: bad.append((nm, "ref", m.get("reference"), ref))
    else:
        doc, tag = (prim, "primary") if nm.endswith("_primary") else (sup, "15seeds")
        key = nm[len("pushed_"):-len("_" + tag)]
        keymap = {"relations": "relations", "e1_valid": "e1_valid", "e1_invalid": "e1_invalid", "e2_valid": "e2_valid", "e2_invalid": "e2_invalid",
                  "e2_rows_carrying_target": "e2_rows_carrying_target", "fibre_type_e1": "fibre_type_e1", "fibre_type_e2": "fibre_type_e2", "push_mismatch": "push_mismatch"}
        k2 = keymap.get(key)
        if k2 is None: bad.append((nm, "unknown")); continue
        keys = ["e1_push_mismatch", "e2_push_mismatch"] if k2 == "push_mismatch" else [k2]
        want = sum(s["pushed"][kk] for r in doc.values() for s in r["seeds"] for kk in keys); n += 1
        if m["value"] != want: bad.append((nm, m["value"], want))
res["cover_pushforward"] = (n, bad)
# target 4 (against the validator replay of small_de.json, if given)
if len(sys.argv) > 2:
    rec = R("benchmarks/records/20261112T1753Z-small-de-endo-fb.json")
    raw = json.load(open(sys.argv[2]))
    nat = lambda d: d["muls"] + d["invs"] + d["tests"] + d["la_ops"]
    cells = {(c["bits"], c["m"], c["d"]): c for c in raw["cells"]}
    bad = []; n = 0
    for m in rec["metrics"]:
        mm = re.match(r"(ic_single_instance_w0p1|ic_relation_phase_w0p1|columns|targets_certified|decomposition_probability)_b(\d+)_m(\d)_d(m?\d+)_a(\d)$", m["name"])
        if not mm: bad.append((m["name"], "unparsed")); continue
        kind, b, mmm, d, a = mm.group(1), int(mm.group(2)), int(mm.group(3)), mm.group(4), "A" + mm.group(5)
        d = -int(d[1:]) if d.startswith("m") else int(d)
        c = cells[(b, mmm, d)]; arm = next(x for x in c["arms"] if x["arm"] == a)
        w = 0.1
        rel = sum(r["gae"] + arm["build_gae"] + w * (nat(r["native"]) + nat(arm["build_native"])) for r in arm["reps"]) / len(arm["reps"])
        tgt = sum(t["gae"] + w * nat(t["native"]) for t in arm["targets"]) / len(arm["targets"])
        best = min(sum(x["gae"] + w * x["native"] for x in rk["runs"]) / len(rk["runs"]) for rk in c["rho"])
        want = {"ic_single_instance_w0p1": rel + tgt, "ic_relation_phase_w0p1": rel, "columns": arm["columns"],
                "targets_certified": sum(1 for t in arm["targets"] if t["certified"]),
                "decomposition_probability": sum(r["decomp_prob"] for r in arm["reps"]) / len(arm["reps"])}[kind]
        n += 1
        if abs(m["value"] - want) > 1e-9 * max(1, abs(want)): bad.append((m["name"], m["value"], want))
        if m.get("reference") is not None and abs(m["reference"] - best) > 1e-9 * best: bad.append((m["name"], "ref", m["reference"], best))
    res["small_de"] = (n, bad)
for k, (n, bad) in res.items():
    print(f"{k}: {n} record metrics compared, {len(bad)} mismatches"); [print("   ", b) for b in bad[:10]]
