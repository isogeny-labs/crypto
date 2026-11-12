#!/usr/bin/env python3
"""VALIDATOR: compare the producer's summary.json with the validator's recompute (exact), and check the numbers quoted
in RESULTS.md / the producer's headline against the recompute at the quoted precision.
usage: results_check.py <frozen study dir> <recompute.json> <out.json>"""
import json, sys, os

FZ, RC, OUT = sys.argv[1:4]
S = json.load(open(os.path.join(FZ, "results/holdout/summary.json")))
R = json.load(open(RC))
res = dict(summary_fields=0, summary_mismatches=[], max_rel_diff=0.0, quoted=[])
for sc in S["cells"]:
    rc = [c for c in R["cells"] if (c["bits"], c["T"]) == (sc["bits"], sc["T"])][0]
    for name in ("ic_rho_solve", "ic_rho_all", "ic_bsgs_solve", "aa_rho", "fixed_ic_rho_solve"):
        for f in ("median", "lo", "hi", "n"):
            a, b = sc[name][f], (rc[name][f] if f != "n" else len(rc[name]["values"]))
            res["summary_fields"] += 1
            d = abs(a - b) / max(abs(a), abs(b), 1e-300)
            res["max_rel_diff"] = max(res["max_rel_diff"], d)
            if d > 1e-12: res["summary_mismatches"].append([sc["bits"], sc["T"], name, f, a, b])
        res["summary_fields"] += 1
        if sc[name]["wins"] != rc[name]["below1"]: res["summary_mismatches"].append([sc["bits"], sc["T"], name, "wins", sc[name]["wins"], rc[name]["below1"]])
    for f, g in (("verdict_solve", "verdict_solve"), ("verdict_all", "verdict_all")):
        res["summary_fields"] += 1
        m = {"cheaper": "faster", "dearer": "slower", "unranked": "unranked"}
        if m[sc[f]] != rc[g]: res["summary_mismatches"].append([sc["bits"], sc["T"], f, sc[f], rc[g]])
    res["summary_fields"] += 2
    if sc["winners"]["ic"] != rc["winners_ic"]: res["summary_mismatches"].append([sc["bits"], sc["T"], "winners_ic", sc["winners"]["ic"], rc["winners_ic"]])
    if sc["winners"]["rho"] != rc["winners_rho"]: res["summary_mismatches"].append([sc["bits"], sc["T"], "winners_rho", sc["winners"]["rho"], rc["winners_rho"]])
    res["summary_fields"] += 1
    if [sc["fixed_keys"]["ic"][0], sc["fixed_keys"]["rho"][0], float(sc["fixed_keys"]["rho"][1])] != [rc["fixed_keys"]["ic"][0], rc["fixed_keys"]["rho"][0], rc["fixed_keys"]["rho"][1]]:
        res["summary_mismatches"].append([sc["bits"], sc["T"], "fixed_keys", sc["fixed_keys"], rc["fixed_keys"]])
for f in ("aa_median_abs_log_dev", "aa_p95_abs_log_dev"):
    res["summary_fields"] += 1
    if abs(S[f] - R["global_aa"][f]) > 1e-15: res["summary_mismatches"].append([f, S[f], R["global_aa"][f]])

cells = {(c["bits"], c["T"]): c for c in R["cells"]}
O = R["opcost_ranges"]
def rng(vals): return [min(vals), max(vals)]
def q(name, claimed, recomputed, ok, note=""):
    res["quoted"].append(dict(name=name, claimed=claimed, recomputed=recomputed, ok=ok, note=note))
med = lambda b, T, k="ic_rho_solve": cells[(b, T)][k]["median"]
r24 = rng([med(24, T) for T in (1024, 4096, 16384)]); r32 = rng([med(32, T) for T in (1024, 4096, 16384)])
q("ic/rho solve 2^24 range", [0.78, 0.83], r24, round(r24[0], 2) == 0.78 and round(r24[1], 2) == 0.83)
q("2^24 faster in all 3 cells (primary rule)", 3, sum(cells[(24, T)]["verdict_solve"] == "faster" for T in (1024, 4096, 16384)),
  all(cells[(24, T)]["verdict_solve"] == "faster" for T in (1024, 4096, 16384)))
q("ic/rho solve 28/1024", 0.93, med(28, 1024), round(med(28, 1024), 2) == 0.93, "faster under the primary rule: " + cells[(28, 1024)]["verdict_solve"])
q("28/4096 and 28/16384 unranked", 2, sum(cells[(28, T)]["verdict_solve"] == "unranked" for T in (4096, 16384)),
  all(cells[(28, T)]["verdict_solve"] == "unranked" for T in (4096, 16384)))
q("ic/rho solve 2^32 range", [1.11, 1.29], r32, round(r32[0], 2) == 1.11 and round(r32[1], 2) == 1.29)
q("2^32 slower in all 3 cells (primary rule)", 3, sum(cells[(32, T)]["verdict_solve"] == "slower" for T in (1024, 4096, 16384)),
  all(cells[(32, T)]["verdict_solve"] == "slower" for T in (1024, 4096, 16384)))
ra = rng([c["ic_rho_all"]["median"] for c in R["cells"]])
q("incl. verify range", [0.82, 1.27], ra, round(ra[0], 2) == 0.82 and round(ra[1], 2) == 1.27)
rf = rng([c["fixed_ic_rho_solve"]["median"] for c in R["cells"]])
q("fixed-key control range", [0.75, 1.28], rf, round(rf[0], 2) == 0.75 and round(rf[1], 2) == 1.28)
fv = {f"{c['bits']}/{c['T']}": c["verdict_fixed"] for c in R["cells"]}
pv = {f"{c['bits']}/{c['T']}": c["verdict_solve"] for c in R["cells"]}
q("fixed-key control 'the same pattern' (decisions equal to the primary rule)", "same pattern", fv, fv == pv,
  "decisions differ in " + ", ".join(k for k in pv if pv[k] != fv[k]))
q("A/A rep0/rep1 interval contains 1 in all 9 cells", 9, sum(c["verdict_aa_contains_1"] for c in R["cells"]), all(c["verdict_aa_contains_1"] for c in R["cells"]))
q("A/A widest 24/1024", [0.863, 0.667, 1.145], [cells[(24, 1024)]["aa_rho"][k] for k in ("median", "lo", "hi")],
  [round(cells[(24, 1024)]["aa_rho"][k], 3) for k in ("median", "lo", "hi")] == [0.863, 0.667, 1.145])
q("A/A median |log| 0.028, p95 0.28", [0.028, 0.28], [R["global_aa"]["aa_median_abs_log_dev"], R["global_aa"]["aa_p95_abs_log_dev"]],
  round(R["global_aa"]["aa_median_abs_log_dev"], 3) == 0.028 and round(R["global_aa"]["aa_p95_abs_log_dev"], 2) == 0.28)
rb = rng([c["ic_bsgs_solve"]["median"] for c in R["cells"]])
q("ic/bsgs range", [1.33, 1.59], rb, round(rb[0], 2) == 1.33 and round(rb[1], 2) == 1.59)
w = R["winners"]
q("rhodpu (kappa 0) fastest rho", 29, w["rho"].get("rhodpu k=0.0"), w["rho"].get("rhodpu k=0.0") == 29)
q("rho2xks fastest rho (RESULTS: 7)", 7, w["rho"].get("rho2xks k=0.5", 0) + w["rho"].get("rho2xks k=0.25", 0),
  w["rho"].get("rho2xks k=0.5", 0) + w["rho"].get("rho2xks k=0.25", 0) == 7, "rhodpu kappa 0.25 wins 1 (32 b, T = 16384), not mentioned")
q("ic5 20, ic5w 16", [20, 16], [w["ic"].get("ic5"), w["ic"].get("ic5w")], [w["ic"].get("ic5"), w["ic"].get("ic5w")] == [20, 16])
q("GAE 214-294 ns", [214, 294], O["gae_ns"], round(O["gae_ns"][0]) == 214 and round(O["gae_ns"][1]) == 294)
q("I/M 23-31", [23, 31], O["I_over_M"], round(O["I_over_M"][0]) == 23 and round(O["I_over_M"][1]) == 31)
q("native op 0.005-0.009 GAE (canon_hash_ns / padd_ns)", [0.005, 0.009], O["native_gae"],
  round(O["native_gae"][0], 3) == 0.005 and round(O["native_gae"][1], 3) == 0.009,
  "numbers match, but canon_hash_ns times canonicalisation + one hash of x, which the harness counts as 2 native operations (N += 2 per point)")
q("std lookup 2^10-2^16 0.04-0.15", [0.04, 0.15], O["std_lookup_2p10_2p16"], round(O["std_lookup_2p10_2p16"][0], 2) == 0.04 and round(O["std_lookup_2p10_2p16"][1], 2) == 0.15)
q("std lookup 2^18-2^20 0.12-0.42", [0.12, 0.42], O["std_lookup_2p18_2p20"], round(O["std_lookup_2p18_2p20"][0], 2) == 0.12 and round(O["std_lookup_2p18_2p20"][1], 2) == 0.42)
q("std lookup 2^22-2^24 0.27-0.85", [0.27, 0.85], O["std_lookup_2p22_2p24"], round(O["std_lookup_2p22_2p24"][0], 2) == 0.27 and round(O["std_lookup_2p22_2p24"][1], 2) == 0.85)
q("fast hasher 2-5x cheaper", [2, 5], O["std_over_fast"], round(O["std_over_fast"][0]) >= 2 and round(O["std_over_fast"][1]) <= 5)
q("1836 executions, 0 errors, every target verified", [1836, 0], [R["checks"]["rows_jsonl"], R["checks"]["ok_ne_targets"] + R["checks"]["errors"]],
  R["checks"]["rows_jsonl"] == 1836 and R["checks"]["ok_ne_targets"] == 0 and R["checks"]["errors"] == 0)
res["quoted_failures"] = [x["name"] for x in res["quoted"] if not x["ok"]]
json.dump(res, open(OUT, "w"), indent=1)
print("summary fields", res["summary_fields"], "mismatches", len(res["summary_mismatches"]), "max rel diff", res["max_rel_diff"])
print(res["summary_mismatches"][:5])
for x in res["quoted"]: print("OK " if x["ok"] else "BAD", x["name"], x["claimed"], x["recomputed"], x["note"])
