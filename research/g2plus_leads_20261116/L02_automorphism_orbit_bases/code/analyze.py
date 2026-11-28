import argparse, glob, json, math, os, random, sys
from collections import defaultdict

READINGS = [("I1_k1", 1, lambda g: 1), ("I10_kg2", 10, lambda g: g * g), ("I30_kg2", 30, lambda g: g * g)]
PRIMARY = "I10_kg2"
IC_ARMS = ("A0", "A", "B"); RHO_ARMS = ("Rneg", "Raut")
DEC = {"F5": [12, 13, 14], "F7": [9, 10]}
SKIP = {"arm", "run_id", "wall_s"}

def mean(x): return sum(x) / len(x)
def gmean(x): return math.exp(sum(math.log(v) for v in x) / len(x))

def ic_fpm(r, I, kap):
    return r["rel_mul"] + I * r["rel_inv"] + kap * (r["la_mul"] + I * r["la_inv"])
def rho_fpm(r, I):
    return r["mul"] + I * r["inv"]

def load(rawdir):
    ic, rho = [], []
    for f in sorted(glob.glob(os.path.join(rawdir, "ic_*.jsonl"))):
        ic += [json.loads(l) for l in open(f) if l.strip()]
    for f in sorted(glob.glob(os.path.join(rawdir, "rho_*.jsonl"))):
        rho += [json.loads(l) for l in open(f) if l.strip()]
    gates = {}
    gp = os.path.join(rawdir, "gates_instances.jsonl")
    for l in open(gp):
        d = json.loads(l); gates[d["curve_id"]] = d
    return ic, rho, gates

def pct(v, q):
    v = sorted(v); i = q * (len(v) - 1)
    lo = int(math.floor(i)); hi = int(math.ceil(i))
    return v[lo] + (v[hi] - v[lo]) * (i - lo)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True); ap.add_argument("--out", required=True); ap.add_argument("--tsv", required=True)
    ap.add_argument("--reps", type=int, default=2000)
    a = ap.parse_args()
    ic, rho, gates = load(a.raw)
    out = {"gates": {}, "cells": {}, "pooled": {}, "verdicts": {}}
    # G1/G2
    failing = sorted(c for c, g in gates.items() if not g["pass"])
    out["gates"]["G1_G2_failing_curves"] = failing
    ic = [r for r in ic if r["curve_id"] not in failing]
    rho = [r for r in rho if r["curve_id"] not in failing]
    # G3
    g3 = {}; void = set()
    cnt = defaultdict(lambda: [0, 0])
    for r in ic + rho:
        k = (r["family"], r["bits"], r["arm"])
        cnt[k][0] += 1
        if not (r["certified"] and r["status"] == "ok"): cnt[k][1] += 1
    for k, (n, bad) in sorted(cnt.items()):
        g3["%s|%s|%s" % k] = {"runs": n, "uncertified": bad}
        if bad / n > 0.05: void.add((k[0], k[1]))
    out["gates"]["G3"] = g3
    out["gates"]["G3_void_cells"] = sorted("%s|%s" % c for c in void)
    ic_ok = [r for r in ic if r["certified"] and r["status"] == "ok"]
    rho_ok = [r for r in rho if r["certified"] and r["status"] == "ok"]
    # index
    ICI = defaultdict(dict)   # (fam,bits) -> (curve,seed) -> arm -> row
    for r in ic_ok: ICI[(r["family"], r["bits"])].setdefault((r["curve_id"], r["seed"]), {})[r["arm"]] = r
    RHI = defaultdict(dict)
    for r in rho_ok: RHI[(r["family"], r["bits"])].setdefault((r["curve_id"], r["seed"]), {})[r["arm"]] = r
    # G4, G5, G6, G7
    g4 = []; g5 = []; g6 = []; g7 = {}
    for (fam, bits), d in sorted(ICI.items()):
        if fam in ("F5", "F7"):
            ratios = []
            for (cid, s), arms in sorted(d.items()):
                if all(x in arms for x in IC_ARMS):
                    ds = [arms[x]["digests"] for x in IC_ARMS]; n = min(len(x) for x in ds)
                    if not all(x[:n] == ds[0][:n] for x in ds): g4.append((cid, s))
                    q = arms["A"]["q"]
                    if not (arms["B"]["ncols"] * q + arms["A"]["delta0"] == arms["A"]["ncols"] and arms["A0"]["ncols"] == 2 * arms["A"]["ncols"]):
                        g5.append((cid, s))
                    ratios.append(arms["A0"]["relations_collected"] / arms["A"]["relations_collected"])
            if ratios:
                g7["%s|%d" % (fam, bits)] = {"mean_pos_ratio": mean(ratios), "n": len(ratios), "pass": 1.8 <= mean(ratios) <= 2.2}
        else:
            for (cid, s), arms in sorted(d.items()):
                if "A" in arms and "B" in arms:
                    for k in arms["A"]:
                        if k not in SKIP and arms["A"][k] != arms["B"][k]: g6.append(("ic", cid, s, k))
    for (fam, bits), d in sorted(RHI.items()):
        if fam in ("C5", "C7"):
            for (cid, s), arms in sorted(d.items()):
                if "Rneg" in arms and "Raut" in arms:
                    for k in arms["Rneg"]:
                        if k not in SKIP and arms["Rneg"][k] != arms["Raut"][k]: g6.append(("rho", cid, s, k))
    out["gates"].update({"G4_failures": g4, "G4_pass": not g4, "G5_failures": g5, "G5_pass": not g5,
                         "G6_failures": g6, "G6_pass": not g6, "G7": g7,
                         "G7_pass": all(v["pass"] for v in g7.values())})
    # cell data
    cells = sorted(set(ICI) | set(RHI))
    cells = [c for c in cells if c not in void]
    cellinfo = {}
    for cell in cells:
        fam, bits = cell
        icd = defaultdict(lambda: defaultdict(list)); rhd = defaultdict(lambda: defaultdict(list))
        meta = {}
        for (cid, s), arms in sorted(ICI[cell].items()):
            for arm, r in arms.items():
                icd[arm][cid].append(r); meta[cid] = r
        for (cid, s), arms in sorted(RHI[cell].items()):
            for arm, r in arms.items():
                rhd[arm][cid].append(r); meta.setdefault(cid, r)
        arms_need_ic = ("A", "B"); curves = sorted(set.intersection(*[set(icd[x]) for x in arms_need_ic] + [set(rhd[x]) for x in RHO_ARMS]))
        cellinfo[cell] = (icd, rhd, curves, meta)

    def arm_value(rows, arm, I, kap):
        if arm in RHO_ARMS: return [rho_fpm(r, I) for r in rows]
        return [ic_fpm(r, I, kap) for r in rows]

    def precompute(cell):
        icd, rhd, curves, meta = cellinfo[cell]
        g = meta[curves[0]]["g"]
        P = {}
        for name, I, kf in READINGS:
            kap = kf(g)
            P[name] = {arm: [arm_value((icd if arm in IC_ARMS else rhd)[arm][c], arm, I, kap) for c in curves]
                       for arm in ("A", "B", "Rneg", "Raut")}
        return P

    pre = {cell: precompute(cell) for cell in cells}

    def estimate(Pr, draws=None):
        means = {}
        for arm, per_curve in Pr.items():
            if draws is None:
                means[arm] = mean([mean(v) for v in per_curve])
            else:
                cd, sd = draws
                vals = []
                for pos, ci in enumerate(cd):
                    v = per_curve[ci]; idx = sd[arm][pos]
                    vals.append(mean([v[i] for i in idx]))
                means[arm] = mean(vals)
        D1 = means["A"] / means["B"]; rg = means["Rneg"] / means["Raut"]
        return means, D1, rg, D1 / rg

    # point estimates
    pt = {cell: {name: estimate(pre[cell][name]) for name, _, _ in READINGS} for cell in cells}
    # bootstrap
    brng = random.Random(20261116)
    boots = {cell: {name: {"D1": [], "N": []} for name, _, _ in READINGS} for cell in cells}
    pooled_b = defaultdict(lambda: {name: {"N": [], "D1": [], "rg": []} for name, _, _ in READINGS})
    for rep in range(a.reps):
        rep_res = {}
        for cell in cells:
            n = len(pre[cell][PRIMARY]["A"])
            cd = [brng.randrange(n) for _ in range(n)]
            sd = {}
            for arm in ("A", "B", "Rneg", "Raut"):
                lens = [len(pre[cell][PRIMARY][arm][ci]) for ci in cd]
                sd[arm] = [[brng.randrange(L) for _ in range(L)] for L in lens]
            for name, _, _ in READINGS:
                m, D1, rg, N = estimate(pre[cell][name], (cd, sd))
                boots[cell][name]["D1"].append(D1); boots[cell][name]["N"].append(N)
                rep_res[(cell, name)] = (D1, rg, N)
        for fam, bl in DEC.items():
            cl = [(fam, b) for b in bl if (fam, b) in pre]
            if not cl: continue
            for name, _, _ in READINGS:
                pooled_b[fam][name]["N"].append(gmean([rep_res[(c, name)][2] for c in cl]))
                pooled_b[fam][name]["D1"].append(gmean([rep_res[(c, name)][0] for c in cl]))
                pooled_b[fam][name]["rg"].append(gmean([rep_res[(c, name)][1] for c in cl]))
    # assemble
    tsv = ["family\tbits\treading\tn_curves\tIC_A\tIC_B\tR_neg\tR_aut\tD1\tD1_lo\tD1_hi\trho_gain\tN\tN_lo\tN_hi\trel_ratio\trel_pred\tpos_ratio\tgap_A\tgap_B"]
    def f6(x): return "%.6g" % x
    for cell in cells:
        fam, bits = cell
        icd, rhd, curves, meta = cellinfo[cell]
        ce = {"n_curves": len(curves), "curves": curves, "readings": {}}
        relA = mean([mean([r["relations_collected"] for r in icd["A"][c]]) for c in curves])
        relB = mean([mean([r["relations_collected"] for r in icd["B"][c]]) for c in curves])
        pred = mean([(meta[c]["nA"] + 10) / (meta[c]["nB"] + 10) for c in curves])
        pos = None
        if "A0" in icd:
            prs = []
            for (cid, s), arms in ICI[cell].items():
                if cid in curves and "A0" in arms and "A" in arms:
                    prs.append(arms["A0"]["relations_collected"] / arms["A"]["relations_collected"])
            pos = mean(prs) if prs else None
        la = {}
        for arm in IC_ARMS:
            if arm in icd:
                rs = [r for c in curves for r in icd[arm][c]]
                la[arm] = {"LA_model_2dim_nnz": mean([2 * r["dim"] * r["nnz"] for r in rs]),
                           "dim": mean([r["dim"] for r in rs]), "nnz": mean([r["nnz"] for r in rs]),
                           "max_fill": max(r["max_fill"] for r in rs), "trials": mean([r["trials"] for r in rs]),
                           "relations": mean([r["relations_collected"] for r in rs]),
                           "grp_ops": mean([r["grp_ops"] for r in rs]),
                           "rel_mul": mean([r["rel_mul"] for r in rs]), "rel_inv": mean([r["rel_inv"] for r in rs]),
                           "la_mul": mean([r["la_mul"] for r in rs]), "la_inv": mean([r["la_inv"] for r in rs]),
                           "la_attempts": mean([r["la_attempts"] for r in rs])}
        rh = {}
        for arm in RHO_ARMS:
            rs = [r for c in curves for r in rhd[arm][c]]
            rh[arm] = {"steps": mean([r["steps"] for r in rs]), "mul": mean([r["mul"] for r in rs]),
                       "inv": mean([r["inv"] for r in rs]), "abandon_frac": sum(r["abandoned_steps"] for r in rs) / max(1, sum(r["steps"] for r in rs)),
                       "lookahead_extra": mean([r["lookahead_extra"] for r in rs]), "walks_started": mean([r["walks_started"] for r in rs]),
                       "dps_stored": mean([r["dps_stored"] for r in rs]), "rmul_bookkeeping": mean([r["rmul_bookkeeping"] for r in rs]),
                       "n_runs": len(rs)}
        q = meta[curves[0]]["q"]
        ce["rel_ratio"] = relA / relB; ce["rel_pred"] = pred; ce["pos_ratio"] = pos
        ce["rel_ratio_within_5pct_of_pred"] = abs(relA / relB / pred - 1) <= 0.05
        ce["la"] = la; ce["rho"] = rh
        for name, I, kf in READINGS:
            means, D1, rg, N = pt[cell][name]
            b = boots[cell][name]
            lo1, hi1 = pct(b["D1"], .025), pct(b["D1"], .975); loN, hiN = pct(b["N"], .025), pct(b["N"], .975)
            ce["readings"][name] = {"IC_A": means["A"], "IC_B": means["B"], "R_neg": means["Rneg"], "R_aut": means["Raut"],
                                    "D1": D1, "D1_lo": lo1, "D1_hi": hi1, "rho_gain": rg, "N": N, "N_lo": loN, "N_hi": hiN,
                                    "gap_A": means["A"] / means["Raut"], "gap_B": means["B"] / means["Raut"]}
            if name == PRIMARY and fam in ("F5", "F7"):
                ce["rho_gain_in_band_[0.75sqrtq,1.15sqrtq]"] = 0.75 * math.sqrt(q) <= rg <= 1.15 * math.sqrt(q)
            tsv.append("\t".join([fam, str(bits), name, str(len(curves)), f6(means["A"]), f6(means["B"]), f6(means["Rneg"]), f6(means["Raut"]),
                                  f6(D1), f6(lo1), f6(hi1), f6(rg), f6(N), f6(loN), f6(hiN), f6(relA / relB), f6(pred),
                                  "NA" if pos is None else f6(pos), f6(means["A"] / means["Raut"]), f6(means["B"] / means["Raut"])]))
        out["cells"]["%s|%d" % cell] = ce
    # pooled and verdicts
    for fam, bl in DEC.items():
        cl = [(fam, b) for b in bl if (fam, b) in pre]
        if not cl: continue
        missing = [b for b in bl if (fam, b) not in pre]
        pooled = {"cells": [c[1] for c in cl], "missing_or_void_cells": missing}
        vs = {}
        g7ok = all(g7.get("%s|%d" % c, {}).get("pass", False) for c in cl)
        for name, I, kf in READINGS:
            N = gmean([pt[c][name][3] for c in cl]); D1 = gmean([pt[c][name][1] for c in cl]); rg = gmean([pt[c][name][2] for c in cl])
            pb = pooled_b[fam][name]
            lo, hi = pct(pb["N"], .025), pct(pb["N"], .975)
            lo1, hi1 = pct(pb["D1"], .025), pct(pb["D1"], .975)
            pooled[name] = {"N": N, "N_lo": lo, "N_hi": hi, "D1": D1, "D1_lo": lo1, "D1_hi": hi1, "rho_gain": rg}
            if not g7ok: v = "INCONCLUSIVE (instrument, G7 failed)"
            elif lo >= 2.0: v = "SUPPORTED"
            elif hi < 2.0: v = "KILLED"
            else: v = "INCONCLUSIVE"
            vs[name] = v
            tsv.append("\t".join([fam, "pooled", name, str(len(cl)), "NA", "NA", "NA", "NA", f6(D1), f6(lo1), f6(hi1), f6(rg), f6(N), f6(lo), f6(hi), "NA", "NA", "NA", "NA", "NA"]))
        pooled["verdict_by_reading"] = vs
        pooled["band_dependent"] = len(set(vs.values())) > 1
        out["pooled"][fam] = pooled
    json.dump(out, open(a.out, "w"), indent=1, sort_keys=True)
    open(a.tsv, "w").write("\n".join(tsv) + "\n")
    print("wrote", a.out, a.tsv)

if __name__ == "__main__":
    main()
