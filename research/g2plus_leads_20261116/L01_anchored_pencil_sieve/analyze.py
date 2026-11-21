import argparse
import glob
import json
import math
import os
from collections import defaultdict


def load(raw):
    recs = defaultdict(list)
    for path in sorted(glob.glob(os.path.join(raw, "*.jsonl"))):
        base = os.path.basename(path)
        for ln in open(path):
            ln = ln.strip()
            if ln:
                r = json.loads(ln)
                r["_file"] = base
                recs[base.split("_")[0]].append(r)
    return recs


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def stats(xs):
    if not xs:
        return {"gm": None, "min": None, "max": None, "n": 0}
    return {"gm": gm(xs), "min": min(xs), "max": max(xs), "n": len(xs)}


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True)
    ap.add_argument("--out-json", required=True)
    ap.add_argument("--out-md", required=True)
    a = ap.parse_args()
    recs = load(a.raw)
    gates = {}
    # G1
    ut = open(os.path.join(a.raw, "unittest.log")).read().strip().splitlines()
    gates["G1_unittest_ok"] = bool(ut) and ut[-1].strip() == "OK"
    # G3
    g3 = {}
    for r in recs.get("gate", []):
        g3["g%d_p%d" % (r["g"], r["p"])] = bool(r.get("pass"))
    gates["G3_tiny_dlp"] = g3
    gates["G3_all_pass"] = len(g3) == 4 and all(g3.values())
    rel = [r for r in recs.get("rel", [])]
    aa = [r for r in recs.get("aa", [])]
    lp2 = recs.get("lp2", [])
    la = recs.get("la", [])
    allr = rel + aa + lp2
    gates["gate_failure_records"] = sum(1 for r in allr if r.get("status") == "gate_failure")
    gates["v_failed"] = sum(r.get("v1_failed", 0) + r.get("v2_failed", 0) for r in allr)
    gates["G2_ok"] = gates["v_failed"] == 0 and gates["gate_failure_records"] == 0
    naive = [r for r in rel + aa if r["arm"] == "P_naive1" and r["status"] == "ok"]
    gates["G4_relset_equal_all"] = all(r.get("relset_equal") for r in naive) and len(naive) > 0
    gates["G4_n_checked"] = len(naive)
    gates["G5_max_bucket_ok"] = all(r.get("max_bucket", 0) <= r["g"] + 1 for r in allr)
    # G6
    g6 = []
    keyf = lambda r: (r["arm"], r["g"], r["p"], r["curve_index"], r["solver_seed"])
    relidx = {keyf(r): r for r in rel}
    for r in aa:
        o = relidx.get(keyf(r))
        if o is None:
            g6.append({"key": keyf(r), "same": False, "why": "missing"})
            continue
        diff = [k for k in set(r) | set(o) if k not in ("cpu_seconds_diag", "_file") and r.get(k) != o.get(k)]
        g6.append({"key": keyf(r), "same": not diff, "diff": diff})
    gates["G6_aa"] = g6
    gates["G6_ok"] = len(g6) == 6 and all(x["same"] for x in g6)
    # G7
    cells_g7 = {}
    byc = defaultdict(list)
    for r in rel:
        if r["arm"] == "P_full" and r["status"] == "ok":
            byc[(r["g"], r["p"])].append(r)
    for (g, p), rs in byc.items():
        cands = sum(sum(t["cands"] for t in r["per_tuple"]) for r in rs)
        tup = sum(r["tuples"] for r in rs)
        ratio = cands / tup / (p / math.factorial(g + 1))
        cells_g7["g%d_p%d" % (g, p)] = ratio
    gates["G7_ratios"] = cells_g7
    gates["G7_ok"] = len(cells_g7) == 8 and all(0.85 <= v <= 1.15 for v in cells_g7.values())
    # pairs
    P = {(r["g"], r["p"], r["curve_index"], r["solver_seed"]): r for r in rel if r["arm"] == "P_full"}
    B = {(r["g"], r["p"], r["curve_index"], r["solver_seed"]): r for r in rel if r["arm"] == "B_lab"}
    N = {(r["g"], r["p"], r["curve_index"], r["solver_seed"]): r for r in rel if r["arm"] == "P_naive1"}
    pairs = []
    for k in sorted(P):
        p_, b_ = P.get(k), B.get(k)
        if not b_ or p_["status"] != "ok" or b_["status"] != "ok":
            continue
        g = k[0]
        d = {"key": k, "P_fpm_per_rel": p_["fpm_per_rel"], "B_fpm_per_rel": b_["fpm_per_rel"]}
        d["R1"] = b_["fpm_per_rel"] / p_["fpm_per_rel"]
        if g == 2:
            d["R2"] = (b_["strong_fpm"] / b_["relations"]) / p_["fpm_per_rel"]
        d["R3"] = b_["fpm_per_rel"] / ((p_["fpm"] + p_["add"] + p_["fd_add"] + p_["bucket_ops"]
                                        + p_["bucket_scan"]) / p_["relations"])
        for I in (1, 30):
            d["R4_I%d" % I] = (((b_["cantor_mul"] + b_["rel_mul"] + I * (b_["cantor_inv"] + b_["rel_inv"]))
                               / b_["relations"]) / ((p_["mul"] + I * p_["inv"]) / p_["relations"]))
        n_ = N.get(k)
        if n_ and n_["status"] == "ok" and n_["relations"]:
            t0 = p_["per_tuple"][0]
            d["PC"] = (n_["fpm"] / n_["relations"]) / ((t0["mul"] + 10 * t0["inv"]) / t0["cands"])
        pairs.append(d)
    cells = {}
    for g in (2, 3):
        for p in (1031, 4099, 16411, 65537):
            ps = [d for d in pairs if d["key"][0] == g and d["key"][1] == p]
            c = {"n_pairs": len(ps), "complete": len(ps) == 6}
            for name in ("R1", "R2", "R3", "R4_I1", "R4_I30", "PC"):
                xs = [d[name] for d in ps if name in d]
                c[name] = stats(xs)
            c["P_fpm_per_rel"] = stats([d["P_fpm_per_rel"] for d in ps])
            c["B_fpm_per_rel"] = stats([d["B_fpm_per_rel"] for d in ps])
            cells["g%d_p%d" % (g, p)] = c
    gates["PC_ok"] = bool(pairs) and all(d.get("PC", 0) >= 5 for d in pairs) and \
        all("PC" in d for d in pairs)
    # noise floor
    noise = {}
    for arm, src in (("P_full", P), ("B_lab", B)):
        for g in (2, 3):
            for p in (1031, 4099, 16411, 65537):
                xs = [r["fpm_per_rel"] for k, r in src.items() if k[0] == g and k[1] == p
                      and r["status"] == "ok" and r["fpm_per_rel"]]
                if xs:
                    noise["%s_g%d_p%d" % (arm, g, p)] = {"max_over_min": max(xs) / min(xs), "n": len(xs)}
    # budget missing
    missing = [(r["arm"], r["g"], r["p"], r["curve_index"], r["solver_seed"], r["status"])
               for r in rel if r["status"] != "ok"]
    # verdict
    allgates = all([gates["G1_unittest_ok"], gates["G3_all_pass"], gates["G2_ok"],
                    gates["G4_relset_equal_all"], gates["G5_max_bucket_ok"], gates["G6_ok"],
                    gates["G7_ok"], gates["PC_ok"]])
    g2c = [cells["g2_p%d" % p] for p in (1031, 4099, 16411, 65537)]
    complete = all(c["complete"] for c in g2c)
    verdict = {"all_gates_and_pc_pass": allgates, "g2_cells_complete": complete}
    if allgates and complete:
        gms = [c["R1"]["gm"] for c in g2c]
        mins = [c["R1"]["min"] for c in g2c]
        n_below = sum(1 for x in gms if x < 3.0)
        if all(x >= 3.0 for x in gms) and all(x >= 2.0 for x in mins):
            robust = all(c["R2"]["gm"] >= 3.0 and c["R3"]["gm"] >= 3.0 for c in g2c)
            verdict["H1"] = "supported, robust" if robust else "supported against the lab instrument only"
            if not robust:
                verdict["H1_failing_readings"] = [
                    nm for nm in ("R2", "R3") if not all(c[nm]["gm"] >= 3.0 for c in g2c)]
        elif gms[3] < 3.0 or n_below >= 2:
            verdict["H1"] = "killed"
        else:
            verdict["H1"] = "inconclusive"
    else:
        verdict["H1"] = "inconclusive"
    verdict["H1_GM_by_p"] = {p: cells["g2_p%d" % p]["R1"]["gm"] for p in (1031, 4099, 16411, 65537)}
    verdict["H1_MIN_by_p"] = {p: cells["g2_p%d" % p]["R1"]["min"] for p in (1031, 4099, 16411, 65537)}

    def sec(g, name):
        cs = [cells["g%d_p%d" % (g, p)] for p in (1031, 4099, 16411, 65537)]
        if not all(c["complete"] for c in cs):
            return "inconclusive (missing cells)"
        return "GM >= 3.0 at all primes" if all(c[name]["gm"] >= 3.0 for c in cs) else "GM < 3.0 at some prime"
    verdict["H2_g3_R1"] = sec(3, "R1")
    verdict["H3_g2_R2"] = sec(2, "R2")
    # H5
    h5 = {}
    lp = defaultdict(dict)
    for r in lp2:
        lp[(r["g"], r["p"], r["curve_index"])][r["arm"]] = r
    for g in (2, 3):
        for p in (4099, 16411):
            xs = []
            miss = 0
            for c in (1, 2, 3):
                d = lp.get((g, p, c), {})
                pr, br = d.get("P_2lp"), d.get("B_2lp")
                if pr and br and pr["status"] == "ok" and br["status"] == "ok" and \
                        pr.get("fpm_per_cycle") and br.get("fpm_per_cycle"):
                    xs.append(br["fpm_per_cycle"] / pr["fpm_per_cycle"])
                else:
                    miss += 1
            h5["g%d_p%d" % (g, p)] = {"ratios": xs, "gm": gm(xs) if xs else None, "missing": miss}
    verdict["H5_cells"] = h5
    for g in (2, 3):
        vs = [h5["g%d_p%d" % (g, p)] for p in (4099, 16411)]
        if any(v["gm"] is None or v["missing"] for v in vs):
            verdict["H5_g%d" % g] = "inconclusive (missing/non-ok cells)"
        else:
            verdict["H5_g%d" % g] = "GM >= 3.0 at both primes" if all(v["gm"] >= 3.0 for v in vs) else "GM < 3.0 at some prime"
    # H4
    h4 = []
    lai = defaultdict(dict)
    for r in la:
        lai[(r["g"], r["p"], r["curve_index"])][r["arm"]] = r
    for (g, p, c), d in sorted(lai.items()):
        pr, br = d.get("P_full"), d.get("B_lab")
        pe, be = P.get((g, p, c, 1)), B.get((g, p, c, 1))
        row = {"g": g, "p": p, "curve_index": c}
        if pr and br:
            row["P_la_mul"], row["B_la_mul"] = pr["mul"], br["mul"]
            row["P_la_inv"], row["B_la_inv"] = pr["inv"], br["inv"]
            row["P_status"], row["B_status"] = pr["status"], br["status"]
            row["la_ratio"] = pr["mul"] / br["mul"] if br["mul"] else None
            row["P_peak_nnz"], row["B_peak_nnz"] = pr["peak_nnz"], br["peak_nnz"]
            row["P_pivots"], row["B_pivots"] = pr["pivots"], br["pivots"]
            row["P_uncovered"], row["B_uncovered"] = pr["uncovered_cols"], br["uncovered_cols"]
            if pr["status"] == "ok" and br["status"] == "ok" and pe and be:
                row["net"] = (be["fpm_per_rel"] - pe["fpm_per_rel"]) * pe["n_target"] - g * g * (
                    (pr["mul"] + 10 * pr["inv"]) - (br["mul"] + 10 * br["inv"]))
            else:
                row["net"] = None
        h4.append(row)
    # row stats
    rowstats = []
    for k in sorted(P):
        pass
    out = {"gates": gates, "cells": cells, "pairs": pairs, "noise_floor": noise, "missing_runs": missing,
           "verdict": verdict, "H4": h4, "lp2": [
               {k: v for k, v in r.items() if k not in ("f", "_file")} for r in lp2]}
    with open(a.out_json, "w") as fh:
        json.dump(out, fh, indent=1, sort_keys=True, default=str)

    def f3(x):
        return "n/a" if x is None else ("%.3f" % x)

    L = []
    L.append("# L01 analysis summary (generated by analyze.py; [pcc:unverified])\n")
    L.append("## Gates\n")
    for k in ("G1_unittest_ok", "G3_all_pass", "G2_ok", "G4_relset_equal_all", "G5_max_bucket_ok", "G6_ok",
              "G7_ok", "PC_ok"):
        L.append("- %s: %s" % (k, gates[k]))
    L.append("- G2 detail: v1+v2 failures %d, gate_failure records %d" % (gates["v_failed"], gates["gate_failure_records"]))
    L.append("- G7 ratios (cands per tuple / (p/(g+1)!)): " + ", ".join("%s=%.4f" % kv for kv in sorted(cells_g7.items())))
    L.append("- G6 A/A: " + "; ".join("%s same=%s" % (x["key"], x["same"]) for x in g6))
    L.append("\n## Cells: R1 (B_lab / P_full FPM per relation, I=10)\n")
    L.append("| g | p | pairs | P_full FPM/rel GM | B_lab FPM/rel GM | R1 GM | R1 MIN | R1 MAX | R2 GM | R3 GM | R4 I=1 GM | R4 I=30 GM | PC GM | PC MIN |")
    L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for g in (2, 3):
        for p in (1031, 4099, 16411, 65537):
            c = cells["g%d_p%d" % (g, p)]
            L.append("| %d | %d | %d | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s | %s |" % (
                g, p, c["n_pairs"], f3(c["P_fpm_per_rel"]["gm"]), f3(c["B_fpm_per_rel"]["gm"]),
                f3(c["R1"]["gm"]), f3(c["R1"]["min"]), f3(c["R1"]["max"]), f3(c["R2"]["gm"]),
                f3(c["R3"]["gm"]), f3(c["R4_I1"]["gm"]), f3(c["R4_I30"]["gm"]), f3(c["PC"]["gm"]),
                f3(c["PC"]["min"])))
    L.append("\n## Noise floor (max/min FPM per relation over the 6 runs of a cell)\n")
    for k, v in sorted(noise.items()):
        L.append("- %s: %.4f (n=%d)" % (k, v["max_over_min"], v["n"]))
    L.append("\n## 2LP (FPM per graph cycle, seed 1)\n")
    L.append("| g | p | curve | arm | status | relations | cycles | target | fpm | fpm/cycle | rels_by_lp |")
    L.append("|---|---|---|---|---|---|---|---|---|---|---|")
    for r in sorted(lp2, key=lambda r: (r["g"], r["p"], r["curve_index"], r["arm"])):
        L.append("| %d | %d | %d | %s | %s | %s | %s | %s | %s | %s | %s |" % (
            r["g"], r["p"], r["curve_index"], r["arm"], r["status"], r.get("relations"), r.get("cycles"),
            r.get("target_cycles"), r.get("fpm"), f3(r.get("fpm_per_cycle")), r.get("rels_by_lp")))
    L.append("\nH5 cells (B_2lp/P_2lp FPM-per-cycle GM over 3 curves):")
    for k, v in sorted(h5.items()):
        L.append("- %s: GM %s ratios %s missing %d" % (k, f3(v["gm"]), [round(x, 3) for x in v["ratios"]], v["missing"]))
    L.append("\n## Linear algebra (counted sparse elimination, q=2^61-1, seed 1)\n")
    L.append("| g | p | curve | P mul | B mul | P/B mul | P inv | B inv | P peak nnz | B peak nnz | P status | B status | net FPM |")
    L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for r in h4:
        if "P_la_mul" in r:
            L.append("| %d | %d | %d | %d | %d | %s | %d | %d | %d | %d | %s | %s | %s |" % (
                r["g"], r["p"], r["curve_index"], r["P_la_mul"], r["B_la_mul"], f3(r["la_ratio"]),
                r["P_la_inv"], r["B_la_inv"], r["P_peak_nnz"], r["B_peak_nnz"], r["P_status"], r["B_status"],
                "n/a" if r["net"] is None else "%.1f" % r["net"]))
    L.append("\n## Verdict lines (PROTOCOL section 8 thresholds: GM >= 3.0 at all four g=2 primes, MIN >= 2.0 at all four; killed if GM(65537) < 3.0 or GM < 3.0 at two or more primes)\n")
    L.append("- all gates and positive control pass: %s; g=2 cells complete: %s" % (allgates, complete))
    L.append("- H1 rule outcome: %s" % verdict["H1"])
    L.append("- H1 GM by p: %s ; MIN by p: %s" % (verdict["H1_GM_by_p"], verdict["H1_MIN_by_p"]))
    L.append("- H2 (g=3, R1, GM >= 3.0 at all primes): %s" % verdict["H2_g3_R1"])
    L.append("- H3 (g=2, R2, GM >= 3.0 at all primes): %s" % verdict["H3_g2_R2"])
    L.append("- H5 g=2: %s ; g=3: %s" % (verdict["H5_g2"], verdict["H5_g3"]))
    if missing:
        L.append("- non-ok relphase records: %s" % missing)
    with open(a.out_md, "w") as fh:
        fh.write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
