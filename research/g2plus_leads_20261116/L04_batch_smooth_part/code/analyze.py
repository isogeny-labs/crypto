"""L04 analyze.py: gates + mechanical verdict (SPEC section 8)."""
import argparse, glob, json, os


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True)
    ap.add_argument("--out", required=True)
    ap.add_argument("--verdict", required=True)
    a = ap.parse_args()
    cells, arms = {}, {}
    for fn in sorted(glob.glob(os.path.join(a.raw, "*.jsonl"))):
        for line in open(fn):
            if not line.strip():
                continue
            d = json.loads(line)
            if d["type"] == "cell":
                cells[d["cell"]] = d
            else:
                arms.setdefault(d["cell"], {})[d["arm"]] = d
    gates = {}
    def gate(name, fails):
        gates[name] = {"pass": not fails, "failing": fails}
    gate("G1", [c for c, d in cells.items() if not d["prime_ok"]])
    gate("G8", [c for c, d in cells.items() if not d["G8_ok"]])
    gate("G3", [c for c, A in arms.items() if len({x["decisions_sha256"] for x in A.values()}) != 1])
    f4 = []
    for c, A in arms.items():
        if c.startswith("corr_"):
            for n, x in A.items():
                if x["oracle_ok"] is not True or x["planted_ok"] is not True:
                    f4.append(c + ":" + n)
    gate("G4", f4)
    f5 = []
    for c, A in arms.items():
        for n, x in A.items():
            if not (x["recompose_ok"] and x["lp_ok"] and x["cz_fail"] == 0):
                f5.append(c + ":" + n)
    gate("G5", f5)
    f7 = []
    for c, A in arms.items():
        if "A_pf" in A and "A_pf_aa" in A:
            r = A["A_pf_aa"]["fpm_total_per_cand"] / A["A_pf"]["fpm_total_per_cand"]
            if abs(r - 1) > 0.05:
                f7.append(c)
        else:
            f7.append(c + " (missing)")
    gate("G7", f7)
    try:
        log = open(os.path.join(a.raw, "unittest.log")).read()
        ok = any(l.strip() == "OK" for l in log.splitlines()) and "FAILED" not in log
    except OSError:
        ok = False
    gates["G2"] = {"pass": ok, "failing": [] if ok else ["unittest.log"]}

    per = {}
    for c, A in arms.items():
        if c.startswith("corr_"):
            continue
        row = {}
        for key, lab in (("fpm_total_per_cand", ""), ("fpm_lab_total_per_cand", "_lab")):
            ab = {n: x[key] for n, x in A.items() if n in ("A_pf", "A_nopf")}
            bb = {n: x[key] for n, x in A.items() if n in ("B", "B_pf") or n.startswith("C_m") or n.startswith("Cpf_m")}
            if not ab or not bb:
                continue
            an = min(ab, key=ab.get); bn = min(bb, key=bb.get)
            row["A_best" + lab] = [an, ab[an]]
            row["batch_best" + lab] = [bn, bb[bn]]
            row["R" + lab] = ab[an] / bb[bn]
        g = cells[c]["g"]; N = cells[c]["N"]
        if "B" in A:
            row["B_dec_mul_per_cand_over_pred"] = (A["B"]["dec"]["mul"] / A["B"]["T_cell"]) / (g * (N - g + 1))
            row["B_dec_mul_per_cand"] = A["B"]["dec"]["mul"] / A["B"]["T_cell"]
        if "A_best" in row:
            row["N_star"] = row["A_best"][1] / (2 * g)
        row["setup_fpm"] = cells[c]["setup"]["mul"] + 10 * cells[c]["setup"]["inv"]
        row["g"], row["N"], row["p"] = g, N, cells[c]["p"]
        per[c] = row
    pc = per.get("pc_g3_p65521_N8", {})
    g6 = pc.get("R", 0) >= 2.0
    gates["G6"] = {"pass": g6, "R_pc": pc.get("R")}
    main = [c for c in per if c.startswith("main_g3_")]
    def verdict(key):
        rs = [per[c][key] for c in main]
        if len(rs) != 3:
            return "NO VERDICT (missing main cells)"
        if all(r >= 2.0 for r in rs): return "SUPPORTED"
        if all(r < 2.0 for r in rs): return "KILLED"
        return "INCONCLUSIVE (p-dependent)"
    hard = [k for k in ("G1", "G2", "G3", "G4", "G5", "G7", "G8") if not gates[k]["pass"]]
    if hard:
        v = vl = "NO VERDICT (gate failure)"
    elif not g6:
        v = vl = "INCONCLUSIVE (instrument)"
    else:
        v, vl = verdict("R"), verdict("R_lab")
    out = {"verdict": v, "verdict_lab": vl, "gates": gates, "cells": per, "status": "[pcc:unverified]",
           "secondary": {c: {n: {k: x[k] for k in ("fpm_pre_per_cand", "fpm_dec_per_cand", "fpm_roots_per_cand",
                                                   "fpm_total_per_cand", "fpm_lab_total_per_cand", "accepted", "T_cell")}
                             | {"accept_frac": x["accepted"] / x["T_cell"]} for n, x in A.items()}
                         for c, A in arms.items()}}
    json.dump(out, open(a.out, "w"), indent=1)
    L = [v, "", "All numbers [pcc:unverified].", ""]
    if vl != v:
        L += ["Verdict with fpm_lab differs: " + vl, ""]
    L += ["| gate | result | failing / value |", "|---|---|---|"]
    for k in sorted(gates):
        g_ = gates[k]
        L.append("| %s | %s | %s |" % (k, "pass" if g_["pass"] else "FAIL", g_.get("failing", g_.get("R_pc"))))
    L += ["", "| cell | A_best (arm, fpm/cand) | batch_best (arm, fpm/cand) | R | R_lab |", "|---|---|---|---|---|"]
    for c, r in per.items():
        if "R" in r:
            L.append("| %s | %s %.2f | %s %.2f | %.4f | %.4f |" % (c, r["A_best"][0], r["A_best"][1],
                     r["batch_best"][0], r["batch_best"][1], r["R"], r["R_lab"]))
    open(a.verdict, "w").write("\n".join(L) + "\n")


if __name__ == "__main__":
    main()
