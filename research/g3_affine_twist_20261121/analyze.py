#!/usr/bin/env python3
"""Analysis of data/control.jsonl and data/main.jsonl per PROTOCOL.md.
Usage: python3 -I analyze.py DIR  -> DIR/summary.json and printed tables."""
import json
import math
import os
import sys


def leg(x, p):
    x %= p
    return 0 if x == 0 else (1 if pow(x, (p - 1) // 2, p) == 1 else -1)


def wilson(x, n, z=1.96):
    if n == 0:
        return [None, None]
    ph = x / n
    d = 1 + z * z / n
    c = (ph + z * z / (2 * n)) / d
    h = z * math.sqrt(ph * (1 - ph) / n + z * z / (4 * n * n)) / d
    return [round(c - h, 4), round(c + h, 4)]


def main(d):
    ctl = [json.loads(l) for l in open(os.path.join(d, "control.jsonl"))]
    out = {"control": {}, "main": {}}
    kappa = {}
    for p in sorted({r["p"] for r in ctl}):
        rows = [r for r in ctl if r["p"] == p]
        st = {"kernels": len(rows), "no_rational_map": 0, "degenerate_system": 0, "smith_inconsistent": 0,
              "G1_fail": 0, "G3_fail": 0, "S_zero": 0, "S_zero_by_nmaps": {}, "agree": 0, "disagree": 0,
              "smith_pass": 0, "smith_fail": 0}
        for r in rows:
            if r.get("smith") == "degenerate_system":
                st["degenerate_system"] += 1
                continue
            if r["n_maps"] == 0:
                st["no_rational_map"] += 1
                continue
            vals = r["vals"]
            if any(v is None for v in vals):
                st["G1_fail"] += 1
                continue
            if vals[0] != vals[1]:
                st["G3_fail"] += 1
            syms = {s for ss in r["smith_symbols"] for s in ss}
            if len(syms) != 1:
                st["smith_inconsistent"] += 1
                continue
            if vals[0] == 0:
                st["S_zero"] += 1
                k = str(r["n_maps"])
                st["S_zero_by_nmaps"][k] = st["S_zero_by_nmaps"].get(k, 0) + 1
                continue
            sm = syms.pop()
            st["smith_pass" if sm == 1 else "smith_fail"] += 1
            mine = leg(r["c"], p) * leg(vals[0], p)
            st["agree" if mine == sm else "disagree"] += 1
        n = st["agree"] + st["disagree"]
        if n >= 50 and st["disagree"] == 0:
            kappa[p] = 1
        elif n >= 50 and st["agree"] == 0:
            kappa[p] = -1
        else:
            kappa[p] = None
        st["kappa_p"] = kappa[p]
        st["compared"] = n
        out["control"][str(p)] = st
    # pattern check
    pat = {}
    for u in (1, -1, 2, -2):
        pat[str(u)] = all(kappa[p] == leg(u, p) for p in kappa)
    out["control_pattern_matches_u"] = pat
    out["P0_kappa_all_plus1"] = all(v == 1 for v in kappa.values())
    mp = os.path.join(d, "main.jsonl")
    if os.path.exists(mp):
        rows = [json.loads(l) for l in open(mp)]
        allc = {"jac": 0, "twist": 0, "degenerate": 0, "uncal": 0, "G1_fail": 0}
        g2 = {"rows": 0, "inv_roots": 0, "inv_zeta8": 0, "inv_sigma": 0, "formula2": 0, "model_t": 0}
        curves = {}
        byN1 = {"N1>p+1": [0, 0], "N1<p+1": [0, 0], "N1=p+1": [0, 0]}
        bins = {}
        perp = {}
        for r in rows:
            p = r["p"]
            e0 = r["evals"][0]
            pp = perp.setdefault(str(p), {"jac": 0, "twist": 0, "degenerate": 0})
            if len(r["evals"]) == 2:
                g2["rows"] += 1
                ok_t = True
                signs = []
                for e in r["evals"]:
                    g2["inv_roots"] += e["inv_roots"]
                    g2["inv_zeta8"] += e["inv_zeta8"]
                    g2["inv_sigma"] += e["inv_sigma"]
                    g2["formula2"] += (e["formula2_val"] == e["val"])
                    signs.append(leg(e["c"], p) * leg(e["val"], p) if e["val"] is not None else None)
                g2["model_t"] += (signs[0] == signs[1])
            if e0["val"] is None:
                allc["G1_fail"] += 1
                continue
            if e0["val"] == 0:
                allc["degenerate"] += 1
                pp["degenerate"] += 1
                curves.setdefault((p, tuple(r["f"])), {})[r["L"]] = 0
                continue
            k = kappa.get(p)
            if k is None:
                allc["uncal"] += 1
                continue
            s = k * leg(e0["c"], p) * leg(e0["val"], p)
            key = "jac" if s == 1 else "twist"
            allc[key] += 1
            pp[key] += 1
            curves.setdefault((p, tuple(r["f"])), {})[r["L"]] = s
            n1 = r["N1"]
            grp = "N1>p+1" if n1 > p + 1 else ("N1<p+1" if n1 < p + 1 else "N1=p+1")
            byN1[grp][0] += (s == 1)
            byN1[grp][1] += 1
            x = n1 / (2 * p + 2)
            b = min(int(x * 10), 9)
            bb = bins.setdefault(b, [0, 0, 0.0])
            bb[0] += (s == 1)
            bb[1] += 1
            bb[2] += x
        n = allc["jac"] + allc["twist"]
        allc["r"] = round(allc["jac"] / n, 4) if n else None
        allc["r_wilson95"] = wilson(allc["jac"], n)
        allc["codomains"] = len(rows)
        for k2, v in perp.items():
            m = v["jac"] + v["twist"]
            v["r"] = round(v["jac"] / m, 4) if m else None
            v["wilson95"] = wilson(v["jac"], m)
        tab = {"++": 0, "+-": 0, "-+": 0, "--": 0, "with_degenerate": 0}
        cov = 0
        ncur = 0
        for key, d2 in curves.items():
            if len(d2) != 2:
                continue
            ncur += 1
            a, b = d2.get(0), d2.get(1)
            if a == 0 or b == 0:
                tab["with_degenerate"] += 1
                if a == 1 or b == 1:
                    cov += 1
                continue
            tab[("+" if a == 1 else "-") + ("+" if b == 1 else "-")] += 1
            if a == 1 or b == 1:
                cov += 1
        out["main"] = {"all": allc, "per_prime": perp, "gates_G2": g2,
                       "two_L_table": tab, "curves": ncur,
                       "coverage_at_least_one_jacobian": [cov, ncur, round(cov / ncur, 4) if ncur else None, wilson(cov, ncur)],
                       "by_N1": {k3: [v[0], v[1], round(v[0] / v[1], 4) if v[1] else None, wilson(v[0], v[1])] for k3, v in byN1.items()},
                       "bins_N1_over_2p2": {str(k4): [v[0], v[1], round(v[0] / v[1], 4), round(v[2] / v[1], 4)] for k4, v in sorted(bins.items())}}
    json.dump(out, open(os.path.join(d, "summary.json"), "w"), indent=1)
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main(sys.argv[1])
