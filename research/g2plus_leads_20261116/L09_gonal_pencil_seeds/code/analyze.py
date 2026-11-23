#!/usr/bin/env python3
"""L09 analysis (SPEC section 13; PROTOCOL sections 5-7). Python 3 standard library only.
Usage: python3 code/analyze.py raw/ > raw/summary.json"""
import sys, os, json, glob, math

M64 = 0xFFFFFFFFFFFFFFFF
PS = (251, 1021, 4093, 16381)
BETAS = [(1, 8), (3, 16), (1, 4), (3, 8), (1, 2), (3, 4)]
TAG = "[pcc:unverified]"


class SM64:
    def __init__(self, s):
        self.s = s & M64

    def next(self):
        self.s = (self.s + 0x9E3779B97F4A7C15) & M64
        z = self.s
        z = ((z ^ (z >> 30)) * 0xBF58476D1CE4E5B9) & M64
        z = ((z ^ (z >> 27)) * 0x94D049BB133111EB) & M64
        return z ^ (z >> 31)

    def below(self, n):
        return (self.next() * n) >> 64


def median(xs):
    xs = sorted(xs)
    n = len(xs)
    if n == 0:
        return None
    if n % 2:
        return xs[n // 2]
    return (xs[n // 2 - 1] + xs[n // 2]) / 2


def quantile(xs, q):
    xs = sorted(xs)
    n = len(xs)
    if n == 0:
        return None
    h = (n - 1) * q
    f = int(math.floor(h))
    if f + 1 >= n:
        return xs[f]
    return xs[f] + (h - f) * (xs[f + 1] - xs[f])


def summ(xs):
    return {"n": len(xs), "median": median(xs), "q25": quantile(xs, 0.25), "q75": quantile(xs, 0.75)}


def bootstrap_ci(xs):
    n = len(xs)
    if n == 0:
        return None
    rng = SM64(0x4C09B007)
    meds = []
    for _ in range(10000):
        meds.append(median([xs[rng.below(n)] for _ in range(n)]))
    meds.sort()
    return [meds[249], meds[9749]]


def load(path):
    out = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                out.append(json.loads(line))
    return out


def main(d):
    census = {}
    for p in PS:
        fn = os.path.join(d, f"census_p{p}.jsonl")
        if os.path.exists(fn):
            census[p] = load(fn)
    cells = {}
    for fn in sorted(glob.glob(os.path.join(d, "cell_p*_b*.jsonl"))):
        base = os.path.basename(fn)[:-6]
        _, ps, bs = base.split("_")
        p, b = int(ps[1:]), int(bs[1:])
        runs = {}
        for r in load(fn):
            runs.setdefault(r["arm"], {})[r["seed"]] = r
        cells[(p, b)] = runs
    exacts = {}
    for fn in sorted(glob.glob(os.path.join(d, "exact_p*_b*.jsonl"))):
        base = os.path.basename(fn)[:-6]
        _, ps, bs = base.split("_")
        p, b = int(ps[1:]), int(bs[1:])
        runs = {}
        for r in load(fn):
            runs.setdefault(r["arm"], {})[r["seed"]] = r
        exacts[(p, b)] = runs

    # ---- gates
    gates = {}
    tol = {251: 0.03, 1021: 0.015, 4093: 0.01, 16381: 0.01}
    g3 = {}
    for p, rows in census.items():
        n = len(rows)
        fx = sum(r["split_x"] for r in rows) / (n * p)
        fy = sum(r["split_y"] for r in rows) / (n * p)
        fl = sum(r["line_split"] for r in rows) / max(1, sum(r["line_nondegen"] for r in rows))
        ptsok = all(r["pts_x"] == r["N1"] and r["pts_y"] == r["N1"] for r in rows)
        ok = all(abs(v - 1 / 6) <= tol[p] for v in (fx, fy, fl)) and ptsok and n == 20
        g3[p] = {"seeds": n, "frac_split_xfibre": fx, "frac_split_yfibre": fy, "frac_split_line_residual": fl,
                 "tolerance": tol[p], "pts_sum_equals_N1": ptsok, "pass": ok}
    gates["G3"] = {"per_p": g3, "pass": all(v["pass"] for v in g3.values()) and len(g3) == 4}
    # G4
    g4n, g4bad = 0, 0
    for k, runs in exacts.items():
        for arm in ("A", "B"):
            for s, r in runs.get(arm, {}).items():
                g4n += 1
                if not r["g4_ok"]:
                    g4bad += 1
    gates["G4"] = {"records": g4n, "violations": g4bad, "pass": g4n > 0 and g4bad == 0}
    # G5
    g5 = {"A": [], "B": []}
    diffs = []
    for k, runs in exacts.items():
        ratio = {}
        for arm in ("A", "B"):
            for s, r in runs.get(arm, {}).items():
                if not r["censored"] and r["T_uf"] > 0:
                    ratio.setdefault(arm, {})[s] = r["T_exact"] / r["T_uf"]
        for arm in ("A", "B"):
            g5[arm] += [v - 1 for v in ratio.get(arm, {}).values()]
        for s in set(ratio.get("A", {})) & set(ratio.get("B", {})):
            diffs.append(ratio["B"][s] - ratio["A"][s])
    mA, mB, mD = median(g5["A"]), median(g5["B"]), median(diffs)
    g5ok = (mA is not None and mB is not None and mD is not None and 0 <= mA <= 0.03 and 0 <= mB <= 0.03 and abs(mD) <= 0.01)
    gates["G5"] = {"n_A": len(g5["A"]), "n_B": len(g5["B"]), "n_pairs": len(diffs), "median_A_Texact_over_Tuf_minus_1": mA,
                   "median_B_Texact_over_Tuf_minus_1": mB, "median_paired_B_minus_A_ratio": mD, "pass": g5ok}
    # G6
    g6fn = os.path.join(d, "g6_rerun.jsonl")
    g6 = {"pass": False, "note": "missing"}
    if os.path.exists(g6fn) and (1021, 2) in cells:
        a = [l.rstrip("\n") for l in open(g6fn) if l.strip()]
        b = [l.rstrip("\n") for l in open(os.path.join(d, "cell_p1021_b2.jsonl")) if l.strip()]
        b = [l for l in b if json.loads(l)["seed"] in (0, 1)]
        g6 = {"lines_rerun": len(a), "lines_cell": len(b), "pass": a == b and len(a) > 0}
    gates["G6"] = g6
    # G7
    tot, ok7, viol = 0, 0, []
    for (p, b), runs in sorted(cells.items()):
        for s, ra in runs.get("A", {}).items():
            rb = runs.get("B", {}).get(s)
            if rb is None:
                continue
            tot += 1
            if rb["T"] <= ra["T"] + 64:
                ok7 += 1
            else:
                viol.append({"p": p, "beta_idx": b, "seed": s, "T_A": ra["T"], "T_B": rb["T"]})
    gates["G7"] = {"runs": tot, "within": ok7, "fraction": (ok7 / tot if tot else None), "violations": viol,
                   "pass": tot > 0 and ok7 / tot >= 0.95}
    glob_ok = all(gates[g]["pass"] for g in ("G4", "G5", "G6", "G7"))

    # ---- cells
    out_cells = {}
    for (p, b), runs in sorted(cells.items()):
        A, A2, B, P1, P4 = (runs.get(x, {}) for x in ("A", "A2", "B", "PC1", "PC4"))
        seeds = sorted(set(A) & set(A2) & set(B) & set(P1) & set(P4))
        def rel(x, y):
            return 1 - x / y if y else None
        rB, rAA, r1, r4, fB, fAA, f1, f4, dB1 = [], [], [], [], [], [], [], [], []
        for s in seeds:
            tA = A[s]["T"]
            if tA == 0:
                continue
            rB.append(rel(B[s]["T"], tA)); rAA.append(rel(A2[s]["T"], tA))
            r1.append(rel(P1[s]["T"], tA)); r4.append(rel(P4[s]["T"], tA))
            dB1.append(rB[-1] - r1[-1])
            fa = A[s]["fpm_rel"]
            fB.append(rel(B[s]["fpm_rel"], fa)); fAA.append(rel(A2[s]["fpm_rel"], fa))
            f1.append(rel(P1[s]["fpm_rel"], fa)); f4.append(rel(P4[s]["fpm_rel"], fa))
        N = median([abs(x) for x in rAA]) if rAA else None
        Nf = median([abs(x) for x in fAA]) if fAA else None
        cens = {a: sum(1 for r in runs.get(a, {}).values() if r["censored"]) for a in ("A", "A2", "B", "PC1", "PC4")}
        cost_a2 = median([r["fpm_rel"] + 24 * r["n_cols"] ** 2 for r in A2.values()]) if A2 else None
        c = {"n": len(rB), "N1_median": median([r["N1"] for r in A.values()]), "n_fb_median": median([r["n_fb"] for r in A.values()]),
             "r_B": summ(rB), "r_AA": summ(rAA), "r_PC1": summ(r1), "r_PC4": summ(r4),
             "f_B": summ(fB), "f_AA": summ(fAA), "f_PC1": summ(f1), "f_PC4": summ(f4),
             "N": N, "N_f": Nf, "censored": cens, "median_rB_minus_rPC1": median(dB1),
             "cost_A2_fpm_plus_LA": cost_a2,
             "median_T": {a: median([r["T"] for r in runs.get(a, {}).values()]) for a in ("A", "A2", "B", "PC1", "PC4")},
             "median_fpm_rel": {a: median([r["fpm_rel"] for r in runs.get(a, {}).values()]) for a in ("A", "A2", "B", "PC1", "PC4")},
             "median_fpm_pencil_B": median([r["fpm_pencil"] for r in B.values()]) if B else None,
             "pencil_rows_B_median": median([r["pencil_rows"] for r in B.values()]) if B else None,
             "pencil_lp_B_median": [median([r["pencil_lp%d" % k] for r in B.values()]) for k in range(4)] if B else None,
             "pc_shortfall_PC1_runs": sum(1 for r in P1.values() if any(r["pc_got%d" % k] < r["pc_target%d" % k] for k in range(3))),
             "pc_shortfall_PC4_runs": sum(1 for r in P4.values() if any(r["pc_got%d" % k] < r["pc_target%d" % k] for k in range(3))),
             "_rB": rB, "_fB": fB}
        out_cells[(p, b)] = c

    # ---- beta*
    beta_star = {}
    for p in PS:
        cand = []
        for b in range(6):
            c = out_cells.get((p, b))
            if c is None:
                continue
            if c["censored"]["A2"] <= 2:
                cand.append((c["cost_A2_fpm_plus_LA"], b))
        beta_star[p] = min(cand)[1] if cand else None

    # ---- verdicts
    def verdict(rb, N, pc, thr_ok_pc, cens_ok, gates_ok):
        if N is None or not rb:
            return "INCONCLUSIVE"
        thr = 0.05 + N
        med = median(rb)
        ci = bootstrap_ci(rb)
        if not (gates_ok and cens_ok and pc is not None and pc >= thr):
            base = "INCONCLUSIVE"
        elif med >= thr and ci[0] > N:
            base = "SUPPORTED"
        elif med < thr:
            base = "KILLED"
        else:
            base = "INCONCLUSIVE"
        return base, {"thr": thr, "median": med, "ci95": ci, "N": N, "positive_control_median": pc}

    def vcell(p, b, metric):
        c = out_cells[(p, b)]
        g3ok = gates["G3"]["per_p"].get(p, {}).get("pass", False)
        gates_ok = glob_ok and g3ok
        cens_ok = all(v <= 2 for v in c["censored"].values())
        if metric == "trials":
            res = verdict(c["_rB"], c["N"], c["r_PC4"]["median"], None, cens_ok, gates_ok)
        else:
            res = verdict(c["_fB"], c["N_f"], c["f_PC4"]["median"], None, cens_ok, gates_ok)
        if isinstance(res, str):
            return {"verdict": res, "detail": None, "censor_ok": cens_ok, "gates_ok": gates_ok}
        return {"verdict": res[0], "detail": res[1], "censor_ok": cens_ok, "gates_ok": gates_ok}

    verdicts = {"at_beta_star": {}, "secondary_all_cells": {}}
    for p in PS:
        bs = beta_star[p]
        if bs is None or (p, bs) not in out_cells:
            verdicts["at_beta_star"][str(p)] = {"beta_idx": None, "trials": {"verdict": "INCONCLUSIVE", "reason": "no beta with <=2 of 20 A2 censored"},
                                                  "fpm": {"verdict": "INCONCLUSIVE"}}
        else:
            verdicts["at_beta_star"][str(p)] = {"beta_idx": bs, "trials": vcell(p, bs, "trials"), "fpm": vcell(p, bs, "fpm")}
        for b in range(6):
            if (p, b) in out_cells:
                verdicts["secondary_all_cells"][f"{p}_b{b}"] = {"trials": vcell(p, b, "trials")["verdict"], "fpm": vcell(p, b, "fpm")["verdict"]}
    headline = verdicts["at_beta_star"].get("16381")

    # ---- B3 descriptive
    b3 = {}
    for (p, b), runs in sorted(exacts.items()):
        b3[f"{p}_b{b}"] = {a: {"n": len(runs.get(a, {})), "median_T_exact": median([r["T_exact"] for r in runs.get(a, {}).values()]),
                                "censored": sum(1 for r in runs.get(a, {}).values() if r["censored"])} for a in ("A", "B", "B3")}

    cen = {str(p): {"n_seeds": len(rows), "N1_median": median([r["N1"] for r in rows]), "redraws_median": median([r["redraws"] for r in rows])} for p, rows in census.items()}
    cells_out = {}
    for (p, b), c in out_cells.items():
        cc = {k: v for k, v in c.items() if not k.startswith("_")}
        cells_out[f"{p}_b{b}"] = cc
    res = {"status": TAG, "gates": gates, "cells": cells_out, "beta_star": {str(p): v for p, v in beta_star.items()},
           "verdicts": verdicts, "headline": headline, "b3_exploratory": b3, "census": {"gate_G3_and_counts": g3, "meta": cen}}
    json.dump(res, sys.stdout, indent=1, sort_keys=True)
    print()


if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else "raw")
