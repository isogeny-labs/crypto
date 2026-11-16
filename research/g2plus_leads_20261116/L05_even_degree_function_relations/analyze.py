import argparse
import glob
import json
import math
import os
from math import factorial

LIMITS = open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "PROTOCOL.md")).read().split("## 11. Limits")[1].strip()
GRID = [(g, p) for g in (2, 3) for p in (1031, 4099, 16411, 65537)]


def gm(xs):
    xs = [x for x in xs if x is not None]
    if not xs:
        return None
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def stat(xs):
    xs = [x for x in xs if x is not None]
    if not xs:
        return {"gm": None, "min": None, "max": None, "n": 0}
    return {"gm": gm(xs), "min": min(xs), "max": max(xs), "n": len(xs)}


def load(path):
    out = []
    with open(path) as f:
        for line in f:
            line = line.strip()
            if line:
                out.append(json.loads(line))
    return out


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--raw", required=True)
    ap.add_argument("--out-json", required=True)
    ap.add_argument("--out-md", required=True)
    a = ap.parse_args()
    raw = a.raw
    res = {}
    # ---- gates
    gates = {}
    gate_curves = []
    failures = []
    for fn in sorted(glob.glob(os.path.join(raw, "gate_*.jsonl"))):
        recs = load(fn)
        summ = [r for r in recs if r["type"] == "gate_summary"]
        gates[os.path.basename(fn)] = bool(summ and summ[-1]["pass"])
        for r in recs:
            if r["type"] == "failure":
                failures.append({"file": os.path.basename(fn), **r})
            if r["type"] == "gate_curve":
                gate_curves.append({k: r[k] for k in ("g", "p", "curve_index", "J_even", "J_odd", "jcheck_ok", "ord_delta")})
    G2 = len(gates) == 5 and all(gates.values())
    runs = {}
    rel_files = sorted(glob.glob(os.path.join(raw, "rel_*.jsonl")))
    for fn in rel_files:
        for r in load(fn):
            if r["type"] == "failure":
                failures.append({"file": os.path.basename(fn), **r})
            elif r["type"] == "run":
                runs[(r["g"], r["p"], r["class"], r["curve_index"], r["solver_seed"], r["arm"])] = r
    aa = {}
    for fn in sorted(glob.glob(os.path.join(raw, "aa_*.jsonl"))):
        for r in load(fn):
            if r["type"] == "failure":
                failures.append({"file": os.path.basename(fn), **r})
            elif r["type"] == "run":
                aa[(r["g"], r["p"], r["class"], r["curve_index"], r["solver_seed"], r["arm"])] = r
    ulog = os.path.join(raw, "unittest.log")
    G1 = False
    if os.path.exists(ulog):
        txt = open(ulog).read().strip().splitlines()
        G1 = bool(txt) and txt[-1].strip() == "OK"
    res["G1_unittest_ok"] = G1
    res["G2_gate_files"] = gates
    res["G2_pass"] = G2
    res["failures"] = failures
    res["gate_curves"] = gate_curves

    def ok(r):
        return r is not None and r.get("status") == "ok"

    # G3 (no failure records; v counters)
    g3 = not failures and all(r["v1_failed"] == 0 and r["vinf_failed"] == 0 and r["v2_failed"] == 0 and
                              r["m1_accepted"] == 0 and r["m2_accepted"] == 0 for r in runs.values())
    mut_checked = sum(r["m1_checked"] + r["m2_checked"] for r in runs.values())
    res["G3_pass"] = g3
    res["G3_mutations_checked_relphase"] = mut_checked
    res["G3_v2_checked_relphase"] = sum(r["v2_checked"] for r in runs.values())
    # G4 holds by construction when no failure record (assertions raise)
    res["G4_pass"] = not any(f.get("stage") == "G4" for f in failures)
    # G5
    g5_bad = []
    g5_n = 0
    for (g, p, cl, c, s, arm), r in runs.items():
        if arm in ("O_T", "E_0_T", "E_1a_T") and cl == "root":
            tw = runs.get((g, p, cl, c, s, arm[:-2]))
            if tw is None:
                g5_bad.append([g, p, c, s, arm, "twin missing"])
                continue
            g5_n += 1
            if tw["keys_sha256"] != r["keys_sha256"]:
                g5_bad.append([g, p, c, s, arm, "digest differs"])
    res["G5"] = {"pairs": g5_n, "bad": g5_bad, "pass": not g5_bad and g5_n > 0}
    # G6
    g6_bad = []
    for k, r in aa.items():
        m = runs.get(k)
        if m is None:
            g6_bad.append([list(k), "rel missing"])
            continue
        d1 = {kk: v for kk, v in r.items() if kk != "cpu_seconds"}
        d2 = {kk: v for kk, v in m.items() if kk != "cpu_seconds"}
        if d1 != d2:
            g6_bad.append([list(k), "fields differ"])
    res["G6"] = {"n": len(aa), "bad": g6_bad, "pass": len(aa) == 16 and not g6_bad}
    # G7
    g7 = {}
    for (g, p) in GRID:
        for arm in ("O", "E_0", "E_1a", "E_low"):
            num = den = 0.0
            nr = 0
            for (gg, pp, cl, c, s, aa_), r in runs.items():
                if gg != g or pp != p or aa_ != arm:
                    continue
                if cl == "oddrand":
                    continue
                if arm == "O" and cl != "root":
                    continue
                num += r["candidates"]
                den += r["pencils"] * p / factorial(r["R"])
                nr += 1
            g7[f"{g},{p},{arm}"] = {"ratio": (num / den) if den else None, "n_runs": nr,
                                    "ok": bool(den) and 0.85 <= num / den <= 1.15}
    res["G7"] = g7
    g7_cell_ok = {(g, p): all(g7[f"{g},{p},{arm}"]["ok"] for arm in ("O", "E_0", "E_1a", "E_low")) for (g, p) in GRID}
    # readings
    def fpm_var(r, kind):
        if r is None or not ok(r) or not r["relations"]:
            return None
        if kind == "I10":
            v = r["mul"] + 10 * r["inv"]
        elif kind == "I1":
            v = r["mul"] + r["inv"]
        elif kind == "I30":
            v = r["mul"] + 30 * r["inv"]
        elif kind == "adds":
            v = r["mul"] + 10 * r["inv"] + r["add"] + r["fd_add"] + r["bucket_ops"]
        return v / r["relations"]

    def ratios(g, p, num_arm, den_arm, kind="I10"):
        out = []
        for c in (1, 2, 3):
            for s in (1, 2):
                n_ = runs.get((g, p, "root", c, s, num_arm))
                d_ = runs.get((g, p, "root", c, s, den_arm))
                x, y = fpm_var(n_, kind), fpm_var(d_, kind)
                if x is not None and y is not None:
                    out.append(x / y)
        return out

    cells = {}
    for (g, p) in GRID:
        cell = {}
        for name, (na, da) in {"R1": ("E_0", "O"), "R2": ("O_T", "E_1a_T"), "R2b": ("O", "E_1a"), "R3": ("E_low", "E_0")}.items():
            cell[name] = stat(ratios(g, p, na, da))
        e0 = [fpm_var(runs.get((g, p, "rootless", c, 1, "E_0")), "I10") for c in (1, 2, 3)]
        o = [fpm_var(runs.get((g, p, "root", c, s, "O")), "I10") for c in (1, 2, 3) for s in (1, 2)]
        e0g, og = gm(e0) if all(x is not None for x in e0) and e0 else None, gm(o) if all(x is not None for x in o) and o else None
        cell["R4"] = {"gm_rootless_E_0": e0g, "gm_root_O": og, "ratio": (e0g / og) if e0g and og else None,
                      "n_rootless": sum(x is not None for x in e0), "n_root": sum(x is not None for x in o)}
        r5 = {}
        for kind in ("adds", "I1", "I30"):
            r5["R1_" + kind] = stat(ratios(g, p, "E_0", "O", kind))
            r5["R2_" + kind] = stat(ratios(g, p, "O_T", "E_1a_T", kind))
        cell["R5"] = r5
        cell["G7_ok"] = g7_cell_ok[(g, p)]
        # complete?
        need = [("root", c, s, arm) for c in (1, 2, 3) for s in (1, 2) for arm in ("O", "E_0", "E_low", "E_1a", "E_g", "O_T", "E_0_T", "E_1a_T")]
        need += [("rootless", c, 1, arm) for c in (1, 2, 3) for arm in ("E_0", "E_1a", "E_g", "E_1a_T")]
        need += [("oddrand", c, 1, arm) for c in (1, 2, 3) for arm in ("O", "O_T")]
        missing = [list(x) for x in need if not ok(runs.get((g, p) + (x[0], x[1], x[2], x[3])))]
        cell["missing_runs"] = missing
        # oddrand descriptive
        orr = [fpm_var(runs.get((g, p, "oddrand", c, 1, "O")), "I10") for c in (1, 2, 3)]
        cell["oddrand_O_gm"] = gm(orr) if all(x is not None for x in orr) else None
        # E_g relation counts
        cell["E_g_relations"] = {f"{cl}_c{c}": (runs[(g, p, cl, c, s, "E_g")]["relations"]) for cl, s in (("root", 1), ("rootless", 1))
                                 for c in (1, 2, 3) if (g, p, cl, c, s, "E_g") in runs}
        cells[f"{g},{p}"] = cell
    res["cells"] = cells
    # row stats per arm
    rowstats = {}
    for arm in ("O", "O_T", "E_0", "E_0_T", "E_low", "E_1a", "E_1a_T", "E_g"):
        for g in (2, 3):
            rs = [r for (gg, pp, cl, c, s, aa_), r in runs.items() if aa_ == arm and gg == g and ok(r)]
            if rs:
                rowstats[f"{arm},g{g}"] = {"n_runs": len(rs),
                                           "mean_of_mean_affine_weight": sum(r["mean_affine_weight"] for r in rs if r["mean_affine_weight"] is not None) / len(rs),
                                           "max_max_col_weight": max(r["max_col_weight"] for r in rs),
                                           "mean_cols_never_hit": sum(r["cols_never_hit"] for r in rs) / len(rs)}
    res["row_stats"] = rowstats
    # spread (A/A noise floor)
    spread = {}
    for (g, p) in GRID:
        for arm in ("O", "E_0", "E_low", "E_1a", "O_T", "E_0_T", "E_1a_T"):
            v = [runs[k]["fpm_per_rel"] for k in runs if k[0] == g and k[1] == p and k[2] == "root" and k[5] == arm and ok(runs[k]) and runs[k]["fpm_per_rel"]]
            if v:
                spread[f"{g},{p},{arm}"] = max(v) / min(v)
    res["spread_max_over_min_fpm_per_rel"] = spread

    # ---- verdicts
    miss_any = any(cells[k]["missing_runs"] for k in cells)
    gates_ok = G1 and G2 and g3 and res["G4_pass"] and res["G5"]["pass"] and res["G6"]["pass"]
    pos_ok = True
    for (g, p) in GRID:
        r3 = cells[f"{g},{p}"]["R3"]["gm"]
        if r3 is None or not (0.8 * (g + 2) <= r3 <= 1.25 * (g + 2)):
            pos_ok = False
    res["positive_control_pass"] = pos_ok
    res["gates_G1_G6_pass"] = gates_ok
    g7_all = all(g7_cell_ok.values())
    res["G7_all_pass"] = g7_all
    r1 = {k: cells[k]["R1"]["gm"] for k in cells}
    big2 = [k for k in cells if k.startswith("2,") and r1[k] is not None and r1[k] > 2.0]
    big3 = [k for k in cells if k.startswith("3,") and r1[k] is not None and r1[k] > 2.0]
    if failures:
        v1 = "failure record present: examine; killed unless traced to an implementation bug"
    elif gates_ok and pos_ok and g7_all and (big2 or len(big3) >= 2):
        v1 = "killed"
    elif gates_ok and pos_ok and g7_all and not miss_any and all(r1[k] is not None and r1[k] <= 2.0 for k in cells):
        v1 = "supported"
    else:
        v1 = "inconclusive"
    res["H1_verdict"] = v1
    res["H1_cells_GM_R1_over_2"] = {"g2": big2, "g3": big3}
    r2 = {k: cells[k]["R2"]["gm"] for k in cells}
    if any(v is not None and v < 1.0 for v in r2.values()):
        v2 = "refuted"
    elif miss_any or any(v is None for v in r2.values()):
        v2 = "incomplete (missing cells)"
    elif all(v >= 1.2 for v in r2.values()):
        v2 = "supported"
    else:
        v2 = "mixed"
    res["H2_verdict"] = v2
    r4 = {k: cells[k]["R4"]["ratio"] for k in cells}
    if any(k.startswith("2,") and r4[k] is not None and r4[k] > 2.0 for k in cells):
        v3 = "refuted"
    elif miss_any or any(v is None for v in r4.values()):
        v3 = "incomplete (missing cells)"
    elif all(v <= 2.0 for v in r4.values()):
        v3 = "supported"
    else:
        v3 = "mixed"
    res["H3_verdict"] = v3
    json.dump(res, open(a.out_json, "w"), indent=1, sort_keys=True)

    def f(x, d=3):
        return "NA" if x is None else f"{x:.{d}f}"

    md = ["# L05 analysis summary (generated by analyze.py; [pcc:unverified])", ""]
    md.append(f"G1 unittest ok: {G1}; G2 gate files pass: {G2}; G3: {g3}; G4: {res['G4_pass']}; G5: {res['G5']['pass']}; "
              f"G6: {res['G6']['pass']}; G7 all cells: {g7_all}; positive control: {pos_ok}")
    md.append("")
    for g in (2, 3):
        md.append(f"## g = {g}")
        md.append("")
        md.append("| p | R1 GM [min,max] | R2 GM [min,max] | R3 GM [min,max] | R4 | G7 ok |")
        md.append("|---|---|---|---|---|---|")
        for (gg, p) in GRID:
            if gg != g:
                continue
            c = cells[f"{g},{p}"]
            md.append(f"| {p} | {f(c['R1']['gm'])} [{f(c['R1']['min'])},{f(c['R1']['max'])}] | "
                      f"{f(c['R2']['gm'])} [{f(c['R2']['min'])},{f(c['R2']['max'])}] | "
                      f"{f(c['R3']['gm'])} [{f(c['R3']['min'])},{f(c['R3']['max'])}] | {f(c['R4']['ratio'])} | {c['G7_ok']} |")
        md.append("")
    md += ["## Verdict lines", "", f"H1: {v1}", f"H2: {v2}", f"H3: {v3}", "", "## Limits (copied from PROTOCOL section 11)", "", LIMITS, ""]
    open(a.out_md, "w").write("\n".join(md))


if __name__ == "__main__":
    main()
