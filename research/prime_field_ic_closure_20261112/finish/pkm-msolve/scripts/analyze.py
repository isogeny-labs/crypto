#!/usr/bin/env python3
"""Apply PRESET.txt section 5 to runs/*.res.json and checks.jsonl.  Usage: analyze.py OUTDIR
Writes OUTDIR/analysis.json and prints tables."""
import json
import math
import os
import sys

OUT = sys.argv[1]
BE = json.load(open(os.path.join(OUT, "breakeven.json")))
ALPHA = BE["alpha_star"]
DSTAR = {(c["m"], c["t"]): c for c in BE["cells"]}


def load():
    rd = os.path.join(OUT, "runs")
    sd = os.path.join(OUT, "systems")
    rows = []
    checks = {}
    cf = os.path.join(OUT, "checks.jsonl")
    if os.path.exists(cf):
        for ln in open(cf):
            r = json.loads(ln)
            checks[r["name"]] = r
    for f in sorted(os.listdir(rd)):
        if not f.endswith(".res.json"):
            continue
        r = json.load(open(os.path.join(rd, f)))
        if r.get("placeholder"):
            continue
        meta = json.load(open(os.path.join(sd, r["name"] + ".json")))
        r.update(arm=meta["arm"], p=meta["p"], m=meta["m"], t=meta["t"], N=meta["N"], kind=meta.get("kind"))
        r["check"] = checks.get(r["name"])
        r["finished"] = r["exit"] == 0
        rows.append(r)
    return rows


def defect(r):
    c = r.get("check")
    if not c or not r["finished"]:
        return False
    bad = c.get("check_i_unit_iff_no_zero") is False or c.get("check_ii_planted") is False or (c.get("check_iii_bad") or 0) > 0
    return bad


def cell_value(rs):
    """generous: min over draws; a censored draw gives a lower bound (D >= value)"""
    vals = []
    for r in rs:
        if defect(r):
            continue
        if r["finished"]:
            vals.append((r["D"], False))
        elif r["D"] is not None:
            vals.append((r["D"], True))
    if not vals:
        return None, None
    exact = [v for v, c in vals if not c]
    lbs = [v for v, c in vals if c]
    if not lbs:
        return min(exact), False
    if not exact:
        return min(lbs), True  # min over draws is >= the smallest lower bound
    if min(lbs) >= min(exact):
        return min(exact), False
    return min(lbs), True


def main():
    rows = load()
    res = {"cells": [], "verdict": {}}
    by = {}
    for r in rows:
        by.setdefault((r["arm"], r["p"], r["m"], r["t"], r["kind"]), []).append(r)
    print("arm p m t kind | D per draw | W per draw | unit | wall s | rss MB | checks")
    for key in sorted(by, key=lambda k: (k[0], k[1], k[2], k[3], str(k[4]))):
        rs = sorted(by[key], key=lambda r: r["name"])
        Ds = [("%s%s" % (r["D"], "" if r["finished"] else "+")) for r in rs]
        Ws = [r["W"] for r in rs]
        units = [r.get("unit") for r in rs]
        walls = [r["wall_s"] for r in rs]
        rss = [round(r["ru_maxrss_kB"] / 1024) for r in rs]
        chk = ["DEFECT" if defect(r) else ("ok" if r.get("check") and r["finished"] else "-") for r in rs]
        zeros = [(r.get("check") or {}).get("grid_zeros") for r in rs]
        print(key, Ds, Ws, units, walls, rss, chk, "zeros", zeros)
        D, cens = cell_value(rs) if key[4] in ("random", None) else (None, None)
        res["cells"].append(dict(arm=key[0], p=key[1], m=key[2], t=key[3], kind=key[4], D_draws=Ds, W_draws=Ws,
                                 unit=units, wall_s=walls, rss_MB=rss, checks=chk, grid_zeros=zeros,
                                 D_cell=D, D_cell_censored=cens,
                                 Dstar_w1=DSTAR.get((key[2], key[3]), {}).get("Dstar_w1"),
                                 Dstar_w2=DSTAR.get((key[2], key[3]), {}).get("Dstar_w2")))
    # kill rule on arm A random cells
    def cell(arm, m, t, p=None):
        for c in res["cells"]:
            if c["arm"] == arm and c["m"] == m and c["t"] == t and c["kind"] == "random" and (p is None or c["p"] == p):
                return c
        return None
    print()
    for arm, p in (("A", 40961), ("B", 2013265921)):
        for m in (4, 5, 6, 7):
            c2, c3, c1, c4 = cell(arm, m, 2, p), cell(arm, m, 3, p), cell(arm, m, 1, p), cell(arm, m, 4, p)
            a1, a2 = ALPHA[f"m{m}_w1"], ALPHA[f"m{m}_w2"]
            lo, hi, tlo, thi, flag = None, None, None, None, ""
            if c2 and c2["D_cell"] is not None and not c2["D_cell_censored"]:
                lo, tlo = c2["D_cell"], 2
                top = c4 if (c4 and c4["D_cell"] is not None and not c4["D_cell_censored"]) else c3
                if top and top["D_cell"] is not None:
                    hi, thi = top["D_cell"], top["t"]
                    if top["D_cell_censored"]:
                        flag = "top censored (lower bound)"
            # PRESET: at m = 7, if t = 3 gives no usable slope (missing, or censored without a rise),
            # fall back to t = 1 -> 2, flagged.
            t3_unusable = (hi is None) or ("censored" in flag and hi == lo)
            if arm == "A" and m == 7 and t3_unusable and c1 and c2 and c1["D_cell"] is not None \
                    and c2["D_cell"] is not None and not c1["D_cell_censored"]:
                lo, hi, tlo, thi = c1["D_cell"], c2["D_cell"], 1, 2
                flag = "fallback t=1->2" + (" (top censored, lower bound)" if c2["D_cell_censored"] else "")
            verdict = "INCONCLUSIVE"
            sigma = None
            if lo is not None and hi is not None:
                sigma = (hi - lo) / (m * (thi - tlo))
                if "censored" in flag and hi == lo:
                    verdict = "INCONCLUSIVE (censored, no rise)"
                    sigma = None
                elif sigma >= a1:
                    verdict = "KILL"
                elif sigma >= a2:
                    verdict = "WEAK-KILL"
                elif sigma == 0:
                    # flat: SURVIVE needs >= 2 consecutive flat t-steps with t_hi >= 3
                    if thi - tlo >= 2:
                        verdict = "SURVIVE"
                    else:
                        verdict = "FLAT-INCONCLUSIVE"
                else:
                    verdict = "INCONCLUSIVE"
            # least-squares slope over all exact t (secondary)
            pts = [(c["t"], c["D_cell"]) for c in (c1, c2, c3, c4) if c and c["D_cell"] is not None and not c["D_cell_censored"]]
            ls = None
            if len(pts) >= 2:
                xs = [m * t for t, _ in pts]
                ys = [d for _, d in pts]
                mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
                sxx = sum((x - mx) ** 2 for x in xs)
                ls = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx if sxx else None
            # secondary width exponent
            sW = None
            wc = [c for c in (c1, c2, c3, c4) if c and c["W_draws"] and all(w is not None for w in c["W_draws"])]
            fin = [c for c in wc if c["D_cell"] is not None and not c["D_cell_censored"]]
            if len(fin) >= 2:
                a, b = fin[-2], fin[-1]
                if b["t"] - a["t"] == 1:
                    sW = math.log2(min(b["W_draws"]) / min(a["W_draws"]))
            key = f"{arm}_m{m}"
            res["verdict"][key] = dict(t_lo=tlo, t_hi=thi, D_lo=lo, D_hi=hi, sigma=sigma, alpha1=a1, alpha2=a2,
                                       verdict=verdict, flag=flag, ls_slope_all_t=ls, s_W=sW, budget_m_minus_3=m - 3)
            print(key, res["verdict"][key])
    vA = {m: res["verdict"].get(f"A_m{m}", {}).get("verdict") for m in (4, 5, 6, 7)}
    surv = any(v == "SURVIVE" for v in vA.values())
    kk = [vA[6], vA[7]]
    if surv:
        item = "OPEN"
    elif all(v in ("KILL", "WEAK-KILL") for v in kk) and any(v == "KILL" for v in kk):
        item = "CLOSED-AT-TOY-SCALE"
    else:
        item = "INCONCLUSIVE"
    res["item_verdict"] = item
    print("ITEM VERDICT (PKM chain, m >= 6):", item, vA)
    json.dump(res, open(os.path.join(OUT, "analysis.json"), "w"), indent=1)


if __name__ == "__main__":
    main()
