#!/usr/bin/env python3
"""Analysis of research/quartic_dl1_cd_sqrt_20261113 by the frozen PROTOCOL.md rules.

usage: analyze.py RESULTS_DIR FROZEN_UNITS_DIR  ->  writes analysis.json and tables.md here.

RESULTS_DIR holds units/<ARM>_<q>_<seed>.jsonl from launch/job.sh; FROZEN_UNITS_DIR holds the frozen
N4 v2 grid units (results/grid-20261025/units.tar.gz of research/quartic_n4v2_20261020, extracted).
"""
import json, math, os, sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
N4V2 = os.path.join(HERE, "..", "quartic_n4v2_20261020", "results")
PHASES = ["fb", "relations", "matrix", "linear_algebra", "log_recovery", "descent", "verify"]
YARMS = ["DL", "DL1", "D"]
XARMS = ["CHAIN", "CRULE", "BOTH"]
W = [0.0, 0.1, 1.0]
TIMING = ("wall", "cpu")


def jl(path):
    out = []
    for line in open(path):
        line = line.strip()
        if line:
            out.append(json.loads(line))
    return out


def frozen_constants():
    c = {}
    for r in jl(os.path.join(N4V2, "summary.json")):
        if r.get("kind") == "ic_size" and r["arm"] == "DL":
            c[r["q"]] = (r["r_inv"], r["r_lmul"], r["gae"])
    return c


def rho_refs():
    """Frozen rho M-eq [w0, w0.1, w1] per (arm, q): summary.json with the frozen calibration-rule
    exclusions; k = 22 from the supplement (summary_supp.json), as the frozen study did."""
    def load(fn):
        recs = jl(os.path.join(N4V2, fn))
        excl = {(r["arm"], r["q"]) for r in recs if r.get("kind") == "rho_excluded"}
        return {(r["arm"], r["q"]): r["meq"] for r in recs if r.get("kind") == "rho_size" and (r["arm"], r["q"]) not in excl}
    main, supp = load("summary.json"), load("summary_supp.json")
    out = {k: v for k, v in main.items() if k[1] != 4194271}
    out.update({k: v for k, v in supp.items() if k[1] == 4194271})
    return out


def counted(rec):
    return {k: v for k, v in rec.items()
            if not k.endswith(TIMING) and k not in ("load1",) and not k.startswith("s.controls") and k != "rec"}


def main():
    res, frozen_dir = sys.argv[1], sys.argv[2]
    C = frozen_constants()
    units = {}  # (arm, q, seed) -> {"ic": [records], "unit": rec}
    for fn in sorted(os.listdir(os.path.join(res, "units"))):
        if not fn.endswith(".jsonl"):
            continue
        arm, q, seed = fn[:-6].split("_")
        recs = jl(os.path.join(res, "units", fn))
        units[(arm, int(q), int(seed))] = {"ic": [r for r in recs if r.get("rec") == "ic"],
                                         "unit": next((r for r in recs if r.get("rec") == "unit"), None)}

    # unit cost per IC arm Y: every attempt record of Y summed (failed attempts charged)
    def cost(u, y):
        rs = [r for r in u["ic"] if r["arm"] == y]
        q = rs[0]["q"]
        ri, rl, gae = C[q]
        meq = unp = 0.0
        for r in rs:
            for p in PHASES:
                meq += r.get(f"{p}.mul", 0) + ri * r.get(f"{p}.inv", 0) + rl * r.get(f"{p}.lmul", 0)
                unp += r.get(f"{p}.hash", 0) + r.get(f"{p}.cmp", 0)
        return [meq + w * gae * unp for w in W], len(rs), all(r["ok"] for r in rs[-1:]), rs

    gates = {"H0": {"compared": 0, "mismatches": []}, "H1": {"pairs": 0, "violations": []}, "H2": {"units": 0, "failures": []},
             "attempts_gt1": []}
    # H2 and attempts
    for (arm, q, s), u in units.items():
        gates["H2"]["units"] += 1
        if u["unit"] is None or u["unit"]["rc"] != 0:
            gates["H2"]["failures"].append(f"{arm}_{q}_{s}: rc {u['unit'] and u['unit']['rc']}")
        for y in YARMS:
            rs = [r for r in u["ic"] if r["arm"] == y]
            if not rs or not rs[-1]["ok"] or rs[-1]["k_found"] != rs[-1]["k"]:
                gates["H2"]["failures"].append(f"{arm}_{q}_{s}/{y}: not verified")
            if len(rs) > 1:
                gates["attempts_gt1"].append(f"{arm}_{q}_{s}/{y}: {len(rs)} attempts")
    # H0: BASE == frozen grid unit on counted fields
    for (arm, q, s), u in units.items():
        if arm != "BASE":
            continue
        fp = os.path.join(frozen_dir, f"ic_{q}_{s}_SD.jsonl")
        if not os.path.exists(fp):
            gates["H0"]["mismatches"].append(f"{q}_{s}: frozen unit missing")
            continue
        fr = {r["arm"]: r for r in jl(fp) if r.get("rec") == "ic" and r["arm"] in YARMS}
        for y in YARMS:
            a = counted([r for r in u["ic"] if r["arm"] == y][-1])
            b = counted(fr[y])
            keys = [k for k in a if k in b]
            gates["H0"]["compared"] += 1
            diff = [k for k in keys if a[k] != b[k]]
            if diff:
                gates["H0"]["mismatches"].append(f"{q}_{s}/{y}: {diff[:6]}")
    # H1: chain changes only relations.mul (BASE vs CHAIN, CRULE vs BOTH)
    for (a_arm, b_arm) in (("BASE", "CHAIN"), ("CRULE", "BOTH")):
        for (arm, q, s), u in units.items():
            if arm != a_arm or (b_arm, q, s) not in units:
                continue
            v = units[(b_arm, q, s)]
            for y in YARMS:
                ra = [r for r in u["ic"] if r["arm"] == y]
                rb = [r for r in v["ic"] if r["arm"] == y]
                gates["H1"]["pairs"] += 1
                if len(ra) != len(rb):
                    gates["H1"]["violations"].append(f"{a_arm}/{b_arm} {q}_{s}/{y}: attempts differ")
                    continue
                for x, z in zip(ra, rb):
                    cx, cz = counted(x), counted(z)
                    diff = [k for k in cx if k in cz and cx[k] != cz[k] and k != "relations.mul"]
                    if diff:
                        gates["H1"]["violations"].append(f"{a_arm}/{b_arm} {q}_{s}/{y}: {diff[:6]}")
                    if not z["relations.mul"] < x["relations.mul"]:
                        gates["H1"]["violations"].append(f"{a_arm}/{b_arm} {q}_{s}/{y}: relations.mul not lower")
    cc = [json.loads(l) for l in open(os.path.join(res, "chaincheck.jsonl")) if l.strip().startswith("{")]
    gates["H1"]["chaincheck"] = cc
    gates["H1"]["chaincheck_mismatches"] = sum(r["mismatches"] for r in cc)

    # ratios
    qs = sorted({q for (_, q, _) in units})
    ratios = {}
    for x in XARMS:
        for y in YARMS:
            for q in qs:
                seeds = sorted(s for (a, qq, s) in units if a == "BASE" and qq == q)
                per_w = []
                for wi in range(3):
                    rr = []
                    for s in seeds:
                        if (x, q, s) not in units:
                            continue
                        rr.append(cost(units[(x, q, s)], y)[0][wi] / cost(units[("BASE", q, s)], y)[0][wi])
                    per_w.append(rr)
                if not per_w[0]:
                    continue
                ratios[f"{x}/{y}/{q}"] = {
                    "x": x, "y": y, "q": q, "k": q.bit_length(), "seeds": len(per_w[0]),
                    "geomean": [math.exp(sum(map(math.log, r)) / len(r)) for r in per_w],
                    "min": [min(r) for r in per_w], "max": [max(r) for r in per_w],
                    "improvement": all(max(r) < 1 for r in per_w)}
    # headline
    big = [262111, 524287, 1048507, 2097091, 4194271]
    head = [ratios[f"BOTH/DL/{q}"]["improvement"] for q in big if f"BOTH/DL/{q}" in ratios]
    verdict = "improvement" if head and all(head) and len(head) == 5 else ("partial" if any(head) else "none")

    # mean cost per arm and distance to rho
    R = rho_refs()
    dist = {}
    for arm in ["BASE"] + XARMS:
        for y in YARMS:
            for q in qs:
                cs = [cost(u, y)[0] for (a, qq, s), u in units.items() if a == arm and qq == q]
                if not cs:
                    continue
                mean = [sum(c[i] for c in cs) / len(cs) for i in range(3)]
                d = {"meq_mean": mean}
                for ra in ("plain", "neg"):
                    if (ra, q) in R:
                        d[f"over_rho_{ra}"] = [mean[i] / R[(ra, q)][i] for i in range(3)]
                dist[f"{arm}/{y}/{q}"] = d

    out = {"schema": "quartic_dl1_cd_sqrt.analysis/v1", "constants": "frozen N4 v2 grid ic_size (r_inv, r_lmul, gae)",
           "weights": W, "gates": gates, "ratios": ratios, "headline": {"rule": "BOTH/BASE for DL below 1 at every seed and w, k = 18..22",
           "per_size": dict(zip([str(q) for q in big], head)), "verdict": verdict}, "cost_and_rho": dist}
    json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

    L = ["# Tables (generated by analyze.py)", "",
         f"Gates: H0 {gates['H0']['compared']} BASE records compared, {len(gates['H0']['mismatches'])} mismatches; "
         f"H1 {gates['H1']['pairs']} pairs, {len(gates['H1']['violations'])} violations, chaincheck mismatches {gates['H1']['chaincheck_mismatches']}; "
         f"H2 {gates['H2']['units']} units, {len(gates['H2']['failures'])} failures; units with a retried IC arm: {len(gates['attempts_gt1'])}.", "",
         f"Headline (BOTH / BASE, DL, k = 18..22): **{verdict}**.", "",
         "| arm / BASE | IC arm | k | q | seeds | geo-mean w=0 | min–max w=0 | geo-mean w=0.1 | geo-mean w=1 | below 1 at every seed and w |",
         "|:--|:--|--:|--:|--:|--:|--:|--:|--:|:--|"]
    for key in sorted(ratios, key=lambda k: (XARMS.index(ratios[k]["x"]), YARMS.index(ratios[k]["y"]), ratios[k]["q"])):
        r = ratios[key]
        L.append(f"| {r['x']} | {r['y']} | {r['k']} | {r['q']} | {r['seeds']} | {r['geomean'][0]:.3f} | {r['min'][0]:.3f}–{r['max'][0]:.3f} | "
                 f"{r['geomean'][1]:.3f} | {r['geomean'][2]:.3f} | {'yes' if r['improvement'] else 'no'} |")
    L += ["", "Distance to the frozen matched rho (mean cold M-eq of the arm / rho M-eq; w = 0, 0.1, 1):", "",
          "| arm | IC arm | k | IC / rho-plain | IC / rho-neg |", "|:--|:--|--:|:--|:--|"]
    for arm in ["BASE", "BOTH"]:
        for q in qs:
            d = dist.get(f"{arm}/DL/{q}")
            if not d:
                continue
            f3 = lambda v: " / ".join(f"{x:.4f}" for x in v) if v else "excluded (frozen calibration rule)"
            L.append(f"| {arm} | DL | {q.bit_length()} | {f3(d.get('over_rho_plain'))} | {f3(d.get('over_rho_neg'))} |")
    open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")
    print("\n".join(L[:6]))


if __name__ == "__main__":
    main()
