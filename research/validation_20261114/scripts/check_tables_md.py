#!/usr/bin/env python3
"""Check every cell of tables.md (research/full_jacobian_ic_20261113) against the
validator's own recompute (out/recompute_fj.json). Validator V, 2026-11-14.
A printed cell passes when it equals the recomputed value rounded as printed.
Usage: check_tables_md.py TABLES_MD RECOMPUTE_JSON FULL_JACOBIAN_JSON OUT
"""
import json, re, sys

md, rc, fj, out = sys.argv[1:5]
R = {r["p"]: r for r in json.load(open(rc))["rows"]}
F = {r["p"]: r for r in json.load(open(fj))["rows"]}
fails, n = [], 0


def same(printed, value):
    s = printed.strip()
    if "e+" in s or "e-" in s:
        m, e = s.split("e")
        dec = len(m.split(".")[1]) if "." in m else 0
        return abs(float(s) - value) <= 0.5 * 10 ** (int(e) - dec) * (1 + 1e-9)
    dec = len(s.split(".")[1]) if "." in s else 0
    return abs(float(s) - value) <= 0.5 * 10 ** (-dec) * (1 + 1e-9)


sec = ""
fitsec = None
FJ = json.load(open(fj))
for line in open(md):
    if line.startswith("###"):
        sec = line; continue
    c = [x.strip() for x in line.strip().strip("|").split("|")]
    if sec.startswith("### Fitted exponents") and len(c) == 4 and c[3].isdigit():
        key = "fits_stage1" if "stage 1 primes" in sec and "and 2" not in sec else "fits_all"
        fit = FJ[key][c[0]]
        lo, hi = [x.strip() for x in c[2].strip("[]").split(",")]
        for pr, v in ((c[1], fit["slope"]), (lo, fit["ci95"][0]), (hi, fit["ci95"][1])):
            n += 1
            if not same(pr, v): fails.append((key, c[0], pr, v))
        n += 1
        if int(c[3]) != fit["n"]: fails.append((key, c[0], "n"))
        continue
    if not sec.startswith(("### Group", "### Ratio")):
        continue
    if len(c) == 10 and c[0].isdigit() and c[1] in ("A", "B", "A_S", "B_S"):
        p, arm = int(c[0]), c[1]
        a = R[p]["arms"][arm]
        vals = a["ratio_neg_band"] + a["ratio_neg_cal"]
        for i, v in enumerate(vals):
            n += 1
            if not same(c[2 + i], v): fails.append((p, arm, i, c[2 + i], v))
        n += 2
        if c[8] != a["class_neg_band"] or c[9] != a["class_neg_cal"]: fails.append((p, arm, "class", c[8:10]))
    elif len(c) == 12 and c[0].isdigit():
        p = int(c[0]); f = F[p]; r = R[p]
        vals = [f["r"], f["jac_order"], f["rho_E"]["negation"]["total_ops"], r["rho_neg_band"][1],
                f["arms"]["A"]["group_ops"], r["arms"]["A"]["total_band"][1], f["arms"]["B"]["group_ops"],
                r["arms"]["B"]["total_band"][1], f["arms"]["A_S"]["group_ops"], f["arms"]["B_S"]["group_ops"],
                f["arms"]["rho_J_negation"]["group_ops"]]
        for i, v in enumerate(vals):
            n += 1
            if not same(c[1 + i], v): fails.append((p, "row", i, c[1 + i], v))
json.dump({"cells_checked": n, "failures": fails}, open(out, "w"), indent=1)
print("tables.md cells checked", n, "failures", len(fails))
for f in fails[:20]: print(f)
