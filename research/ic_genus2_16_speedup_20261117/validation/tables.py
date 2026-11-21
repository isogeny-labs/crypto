#!/usr/bin/env python3
"""Validator's independent re-derivation of the P1/P2 tables and side claims
from the committed JSONL.  Shares no code with P1/analyze.py or
P2/analyze_p2.py; written from the definitions in P1/PROTOCOL.md and the
validation dispatch:

  FPM_1  = cantor_mul + 10*cantor_inv + rel_mul + la_mul
  FPM_g2 = cantor_mul + 10*cantor_inv + rel_mul + g^2 * la_mul
  rhoE   = (R.cantor_mul + 10*R.cantor_inv) / R.steps * sqrt(pi * r / 4)
           (R = the executed negation-map rho run on the same (curve, seed))

Every field is read with [] (lesson L-99ee952350b2): a missing key raises.

usage: python3 tables.py            (paths are fixed relative to this file)
"""
import glob
import json
import math
import os
import re
import sys
from collections import defaultdict

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)
ARMS = {"c": "C_double_lp+pre", "D": "D_deg2_full", "E": "E_deg2_lp", "R": "R_rho_neg", "F": "F_deg2_part"}

findings = []


def note(msg):
    findings.append(msg)
    print("FINDING:", msg)


def load(paths):
    """run records keyed (g, p, curve_seed, seed, arm); duplicates reported."""
    runs = {}
    dup = []
    inst = {}
    for fn in sorted(paths):
        with open(fn) as fh:
            for line in fh:
                if not line.strip():
                    continue
                d = json.loads(line)
                if d["kind"] == "instance":
                    inst[(d["g"], d["p"], d["curve_seed"])] = d
                if d["kind"] != "run":
                    continue
                k = (d["g"], d["p"], d["curve_seed"], d["seed"], d["arm"])
                if k in runs:
                    dup.append((k, fn, runs[k][0], {x: d[x] for x in d if x != "wall_ns"} ==
                                {x: runs[k][1][x] for x in runs[k][1] if x != "wall_ns"}))
                runs[k] = (fn, d)
    return {k: v[1] for k, v in runs.items()}, dup, inst


def fpm(d, k=1):
    return d["cantor_mul"] + 10 * d["cantor_inv"] + d["rel_mul"] + k * d["la_mul"]


def rho_cost(R):
    return R["cantor_mul"] + 10 * R["cantor_inv"]


def rho_expected(R):
    return rho_cost(R) / R["steps"] * math.sqrt(math.pi * R["r"] / 4)


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def pairs_of(runs, need=("c", "D", "E", "R")):
    by = defaultdict(dict)
    for (g, p, cs, sd, arm), d in runs.items():
        by[(g, p, cs, sd)][arm] = d
    out = defaultdict(list)
    incomplete = []
    for key, arms in sorted(by.items()):
        if all(ARMS[a] in arms for a in need):
            out[key[:2]].append((key, arms))
        else:
            incomplete.append((key, sorted(arms)))
    return out, incomplete


def cell_stats(g, rows):
    res = {"pairs": len(rows)}
    per = []
    for key, arms in rows:
        c, D, E, R = (arms[ARMS[a]] for a in "cDER")
        rE = rho_expected(R)
        # consistency: same r in every arm of the pair
        rs = {x["r"] for x in (c, D, E, R)}
        if len(rs) != 1:
            note(f"r differs across arms at {key}: {rs}")
        per.append(dict(key=key, c=fpm(c) / rE, D=fpm(D) / rE, E=fpm(E) / rE,
                        Dg=fpm(D, g * g) / rE, Eg=fpm(E, g * g) / rE, cg=fpm(c, g * g) / rE,
                        cD=fpm(c) / fpm(D), cE=fpm(c) / fpm(E), cbest=fpm(c) / min(fpm(D), fpm(E)),
                        EoverD=fpm(E) / fpm(D), DgoverEg=fpm(D, g * g) / fpm(E, g * g),
                        ex_c=c["exhausted"], ex_D=D["exhausted"], ex_E=E["exhausted"],
                        tpr_c=(c["trials"] / c["relations"]) if c["relations"] else None,
                        tpr_D=D["trials"] / D["relations"],
                        tpr_E=E["trials"] / E["relations"],
                        relEc=(E["relations"] / c["relations"]) if c["relations"] else None,
                        Dla=D["la_mul"] / fpm(D), execrho=rho_cost(R) / rE))
    for k in ("c", "D", "E", "Dg", "Eg", "cg", "cD", "cE", "cbest", "EoverD", "DgoverEg", "tpr_c", "tpr_D",
              "tpr_E", "relEc", "Dla", "execrho"):
        vals = [x[k] for x in per if x[k] is not None]
        if not vals:
            res[k] = res[k + "_min"] = res[k + "_max"] = float("nan")
            continue
        res[k] = gm(vals)
        res[k + "_min"] = min(vals)
        res[k + "_max"] = max(vals)
    res["best"] = "D" if res["D"] < res["E"] else "E"
    res["gain"] = res["c"] / res[res["best"]]
    res["n_ex_c"] = sum(x["ex_c"] for x in per)
    res["n_ex_DE"] = sum(x["ex_D"] or x["ex_E"] for x in per)
    res["per"] = per
    return res


def num(s):
    s = s.replace("*", "").replace(">=", "").replace("~", "").strip()
    m = re.match(r"^([0-9.]+)", s)
    return float(m.group(1)) if m else None


def parse_md_table(path, ncols_min):
    rows = []
    with open(path) as fh:
        for line in fh:
            if re.match(r"^\| (6|7|8|10) \| \d+ \|", line):
                cols = [c.strip() for c in line.strip().strip("|").split("|")]
                if len(cols) >= ncols_min:
                    rows.append(cols)
    return rows


def sigfig_tol(s):
    """Half a unit in the last printed digit of s (rounding tolerance)."""
    s = s.replace("*", "").replace(">=", "").strip()
    m = re.match(r"^([0-9]+)(?:\.([0-9]+))?", s)
    if not m:
        return 0.0
    dec = len(m.group(2)) if m.group(2) else 0
    return 0.5 * 10 ** (-dec)


def compare(label, reported_str, value, lower_bound_expected=None):
    rep = num(reported_str)
    if rep is None:
        print(f"  {label}: reported '{reported_str}' unparsable; recomputed {value:.4g}")
        return True
    rel = abs(value - rep) / rep
    tol = max(0.01 * rep, sigfig_tol(reported_str))
    ok = abs(value - rep) <= tol
    lb = ""
    if lower_bound_expected is not None:
        has = ">=" in reported_str
        if has != lower_bound_expected:
            ok = False
            lb = f" [lower-bound marker {'present' if has else 'absent'}, expected {'present' if lower_bound_expected else 'absent'}]"
    flag = "ok" if ok else "DISCREPANCY"
    print(f"  {label}: reported {reported_str!r} recomputed {value:.4g} (rel diff {rel:.2%}){lb} -> {flag}")
    if not ok:
        note(f"{label}: reported {reported_str} vs recomputed {value:.4g} (rel {rel:.2%}){lb}")
    return ok


def check_certs(name, runs, used_keys):
    n = n_ex = bad = 0
    ex_by_arm = defaultdict(int)
    for k in used_keys:
        d = runs[k]
        n += 1
        ex = d["exhausted"] if d["arm"] != ARMS["R"] else False
        if ex:
            n_ex += 1
            ex_by_arm[d["arm"]] += 1
            continue
        if d["certified"] is not True or d["ok"] is not True:
            bad += 1
            note(f"{name}: run {k} certified={d['certified']} ok={d['ok']} not exhausted")
        if d["arm"] != ARMS["R"] and d["bad_relations"] != 0:
            note(f"{name}: run {k} bad_relations={d['bad_relations']}")
    print(f"[certs] {name}: {n} run records used; {n_ex} exhausted {dict(ex_by_arm)}; "
          f"{bad} non-exhausted runs without certified=true/ok=true")
    return n, n_ex, bad, dict(ex_by_arm)


def band(rows, g):
    """Validator's 5a-style sensitivity in FPM units: unpriced counted additions /
    hashes / compares priced at w FPM per op, w in {0, 0.1, 1}, on every arm.
    IC: U = rel_add + la_add.  rho: U = compares_uncharged + hash_uncharged +
    coef_adds_uncharged (+ native in the 'wide' variant), scaled to expected steps."""
    out = {}
    for w in (0.0, 0.1, 1.0):
        for wide in (False, True):
            vals = defaultdict(list)
            for key, arms in rows:
                c, D, E, R = (arms[ARMS[a]] for a in "cDER")
                U_R = R["compares_uncharged"] + R["hash_uncharged"] + R["coef_adds_uncharged"] + (R["native"] if wide else 0)
                rE = (rho_cost(R) + w * U_R) / R["steps"] * math.sqrt(math.pi * R["r"] / 4)
                for a, d in (("c", c), ("D", D), ("E", E)):
                    vals[a].append((fpm(d) + w * (d["rel_add"] + d["la_add"])) / rE)
                vals["cbest"].append((fpm(c) + w * (c["rel_add"] + c["la_add"])) /
                                     min(fpm(D) + w * (D["rel_add"] + D["la_add"]), fpm(E) + w * (E["rel_add"] + E["la_add"])))
            out[(w, wide)] = {a: gm(v) for a, v in vals.items()}
    return out


def main():
    os.chdir(ROOT)
    # ------------------------------------------------------------ P1 table
    p1_paths = glob.glob("P1/main/*.jsonl") + glob.glob("P1/supp/*.jsonl")
    p1, dup1, inst1 = load(p1_paths)
    print(f"P1: {len(p1)} run records from {len(p1_paths)} files; duplicates: {len(dup1)}")
    for d in dup1:
        print("  duplicate", d)
        if not d[3]:
            note(f"P1 duplicate key with different content: {d}")
    cells1, inc1 = pairs_of(p1)
    for k in inc1:
        print("  incomplete pair (not in table):", k)
    stats1 = {gp: cell_stats(gp[0], rows) for gp, rows in cells1.items()}
    print("\n| g | p | pairs | c/rhoE | D/rhoE | E/rhoE | D/rhoE(g2) | E/rhoE(g2) | best | gain(GM c/best) | GM c/min(D,E) per pair | c exh | D/E exh | exec-rho/rhoE |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
    for gp in sorted(stats1):
        s = stats1[gp]
        lb = ">=" if s["n_ex_c"] else ""
        print(f"| {gp[0]} | {gp[1]} | {s['pairs']} | {lb}{s['c']:.4g} | {s['D']:.4g} | {s['E']:.4g} | {s['Dg']:.4g} | "
              f"{s['Eg']:.4g} | {s['best']} | {lb}{s['gain']:.4g} | {lb}{s['cbest']:.4g} | {s['n_ex_c']} | {s['n_ex_DE']} | {s['execrho']:.3g} |")

    print("\nCompare with P1/RESULTS.md table:")
    for cols in parse_md_table("P1/RESULTS.md", 8):
        g, p = int(cols[0]), int(cols[1])
        s = stats1.get((g, p))
        if s is None:
            note(f"P1 RESULTS row g={g} p={p} has no complete pairs in data")
            continue
        lbx = s["n_ex_c"] > 0
        compare(f"g={g} p={p} pairs", cols[2], s["pairs"])
        compare(f"g={g} p={p} c/rho", cols[3], s["c"], lbx)
        compare(f"g={g} p={p} D/rho", cols[4], s["D"])
        compare(f"g={g} p={p} E/rho", cols[5], s["E"])
        compare(f"g={g} p={p} D/rho(g2)", cols[6], s["Dg"])
        compare(f"g={g} p={p} gain", cols[7], s["gain"], lbx)
        arm = re.search(r"\((D|E)\)", cols[7])
        if arm and arm.group(1) != s["best"]:
            note(f"g={g} p={p}: best arm reported {arm.group(1)}, recomputed {s['best']}")
        bold = "D" if cols[4].startswith("**") else ("E" if cols[5].startswith("**") else None)
        if bold and bold != s["best"]:
            note(f"g={g} p={p}: bold arm {bold}, recomputed best {s['best']}")

    # ------------------------------------------------------- P1 side claims
    print("\nP1 side claims (recomputed):")
    g678 = [(gp, x) for gp, s in stats1.items() if gp[0] in (6, 7, 8) for x in s["per"]]
    mn = min(g678, key=lambda t: t[1]["cE"])
    print(f"  P-a: min per-pair c/E over g in 6,7,8 = {mn[1]['cE']:.4g} at {mn[1]['key']} (reported 1.08 at g=6 p=211); "
          f"pairs with c/E<1: {sum(1 for _, x in g678 if x['cE'] < 1)}")
    for gp in ((6, 31), (6, 53), (6, 101), (6, 211)):
        s = stats1[gp]
        print(f"  P-a: g=6 p={gp[1]} per-pair c/D range {s['cD_min']:.3g}-{s['cD_max']:.3g}; pairs c/D<1: "
              f"{sum(1 for x in s['per'] if x['cD'] < 1)}/{s['pairs']}")
    for gp in sorted(stats1):
        s = stats1[gp]
        if s["cD_min"] < 1 and gp[0] != 6:
            print(f"  P-a: non-g6 cell with a c/D<1 pair: {gp} min {s['cD_min']:.3g}")
    s10 = stats1[(10, 11)]
    print(f"  P-a: g=10 p=11 GM E/D = {s10['EoverD']:.4g} (reported 50x); min pair c/E = {s10['cE_min']:.4g} (reported 0.88)")
    g6 = [(p, stats1[(6, p)]["gain"]) for p in (31, 53, 101, 211)]
    xs = [math.log(p) for p, _ in g6]
    ys = [math.log(v) for _, v in g6]
    mx, my = sum(xs) / 4, sum(ys) / 4
    slope = sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
    print(f"  P-b: g=6 gains {[round(v, 3) for _, v in g6]}; LSQ log-log slope = {slope:.4g} (reported -1.05)")
    pstar6 = 211 * stats1[(6, 211)]["gain"] ** (1 / -slope)
    print(f"  P-b: crossover at LSQ slope from p=211: p* = {pstar6:.4g}; at slope -1: {211 * stats1[(6, 211)]['gain']:.4g} (reported ~270)")
    for p in (53, 101, 211):
        s = stats1[(6, p)]
        print(f"  P-c: g=6 p={p} GM D_g2/E_g2 = {s['DgoverEg']:.4g} (min pair {s['DgoverEg_min']:.3g}) (reported 6x at 53, 117x at 211)")
    for gp in sorted(stats1):
        if gp[0] >= 8:
            s = stats1[gp]
            print(f"  P-d: {gp} GM E/D (FPM_1) = {s['EoverD']:.4g} (min pair {s['EoverD_min']:.3g}) (reported range 2.3-51x)")
    for gp in sorted(stats1):
        s = stats1[gp]
        if gp[0] == 6:
            print(f"  mech: {gp} trials/relation GM c={s['tpr_c']:.4g} [{s['tpr_c_min']:.4g}-{s['tpr_c_max']:.4g}] "
                  f"D={s['tpr_D']:.4g} E={s['tpr_E']:.4g}; rels E/c={s['relEc']:.3g}; D LA share={s['Dla']:.3g}")
    for gp in sorted(stats1):
        s = stats1[gp]
        if gp[0] != 6:
            print(f"  mech: {gp} trials/relation GM c={s['tpr_c']:.4g} D={s['tpr_D']:.4g} E={s['tpr_E']:.4g}; "
                  f"rels E/c={s['relEc']:.3g}; D LA share={s['Dla']:.3g}")

    # ---------------------------------------------------- P2 crossover
    p2_paths = glob.glob("P2/crossover/*.jsonl") + glob.glob("P2/crossover_supp/*.jsonl")
    p2, dup2, inst2 = load(p2_paths)
    print(f"\nP2 crossover: {len(p2)} run records; duplicates: {len(dup2)}")
    cells2, inc2 = pairs_of(p2)
    for k in inc2:
        print("  incomplete pair:", k)
    stats2 = {gp: cell_stats(gp[0], rows) for gp, rows in cells2.items()}
    for gp in sorted(stats2):
        s = stats2[gp]
        lb = ">=" if s["n_ex_c"] else ""
        print(f"  g={gp[0]} p={gp[1]} pairs={s['pairs']} c/rhoE={lb}{s['c']:.4g} D={s['D']:.4g} E={s['E']:.4g} "
              f"best={s['best']} gain={lb}{s['gain']:.4g} c-exhausted={s['n_ex_c']} D/E-exhausted={s['n_ex_DE']}")
    print("Compare with P2/RESULTS_crossover.md table:")
    for cols in parse_md_table("P2/RESULTS_crossover.md", 8):
        g, p = int(cols[0]), int(cols[1])
        if (g, p) not in stats2:
            continue
        s = stats2[(g, p)]
        lbx = s["n_ex_c"] > 0
        compare(f"P2 g={g} p={p} pairs", cols[2], s["pairs"])
        compare(f"P2 g={g} p={p} c/rho", cols[3], s["c"], lbx)
        compare(f"P2 g={g} p={p} D/rho", cols[4], s["D"])
        compare(f"P2 g={g} p={p} E/rho", cols[5], s["E"])
        compare(f"P2 g={g} p={p} gain", cols[6], s["gain"], lbx)
    allst = dict(stats1)
    allst.update(stats2)
    print("Decay slopes (log-log, adjacent p) from recomputed gains:")
    for g, ps in ((6, (31, 53, 101, 211)), (7, (31, 53, 101)), (8, (31, 53))):
        gains = [allst[(g, p)]["gain"] for p in ps]
        sl = [math.log(gains[i + 1] / gains[i]) / math.log(ps[i + 1] / ps[i]) for i in range(len(ps) - 1)]
        print(f"  g={g} p={ps} gains={[round(x, 3) for x in gains]} slopes={[round(x, 3) for x in sl]}")
        pmax = ps[-1]
        print(f"     p* at slope -1 from p={pmax}: {pmax * gains[-1]:.4g}")

    # ------------------------------------------------------- P2 lever F
    print("\nP2 lever F (pilot):")
    nctl = nbad = 0
    Frows = defaultdict(list)
    for dname in sorted(glob.glob("P2/pilot_phi*")):
        phi = dname.split("phi")[-1]
        pf, dupf, _ = load(glob.glob(os.path.join(dname, "*.jsonl")))
        for (g, p, cs, sd, arm), d in sorted(pf.items()):
            if arm == ARMS["E"]:
                ref = p1.get((g, p, cs, sd, arm))
                if ref is None:
                    note(f"F pilot E control {g},{p},{cs},{sd} phi={phi} has no P1 record")
                    continue
                nctl += 1
                diff = [x for x in (set(d) | set(ref)) - {"wall_ns"} if d.get(x, "<missing>") != ref.get(x, "<missing>")]
                if diff:
                    nbad += 1
                    note(f"F pilot E control mismatch {g},{p},{cs},{sd} phi={phi}: {diff}")
                continue
            if arm != ARMS["F"]:
                continue
            c, D, E, R = (p1[(g, p, cs, sd, ARMS[a])] for a in "cDER")
            rE = rho_expected(R)
            Frows[(g, p, phi)].append(dict(F=fpm(d) / rE, cF=fpm(c) / fpm(d), bF=min(fpm(D), fpm(E)) / fpm(d),
                                           cols=d["fb_cols"], cert=d["certified"] and d["ok"] and not d["exhausted"],
                                           exc=c["exhausted"]))
    print(f"  E control: {nctl} reruns, {nbad} mismatches (all fields except wall_ns)")
    frep = {}
    for cols in parse_md_table("P2/RESULTS_F.md", 7):
        g, p = int(cols[0]), int(cols[1])
        frep[(g, p)] = cols
    for g, p in sorted({(k[0], k[1]) for k in Frows}):
        phis = sorted({k[2] for k in Frows if k[:2] == (g, p)}, key=float)
        cols = frep.get((g, p))
        rep = {name: (cols[i].split("/") if cols else None) for name, i in (("cols", 3), ("F", 4), ("cF", 5), ("bF", 6))}
        for j, phi in enumerate(phis):
            v = Frows[(g, p, phi)]
            lb = any(x["exc"] for x in v)
            vals = dict(cols=v[0]["cols"], F=gm([x["F"] for x in v]), cF=gm([x["cF"] for x in v]), bF=gm([x["bF"] for x in v]))
            print(f"  g={g} p={p} phi={phi} pairs={len(v)} cols={vals['cols']} F/rhoE={vals['F']:.4g} "
                  f"c/F={'>=' if lb else ''}{vals['cF']:.4g} min(D,E)/F={vals['bF']:.4g} certified={all(x['cert'] for x in v)}")
            if len({x['cols'] for x in v}) != 1:
                note(f"F g={g} p={p} phi={phi}: fb_cols differs across seeds")
            if cols:
                for name in ("cols", "F", "cF", "bF"):
                    compare(f"F g={g} p={p} phi={phi} {name}", rep[name][j].strip(), vals[name],
                            lb if name == "cF" else None)

    # ------------------------------------------------------- certificates
    print()
    used1 = [k for gp, rows in cells1.items() for key, arms in rows for k in
             [key + (ARMS[a],) for a in "cDER"]]
    used2 = [k for gp, rows in cells2.items() for key, arms in rows for k in
             [key + (ARMS[a],) for a in "cDER"]]
    check_certs("P1 table", p1, used1)
    check_certs("P2 crossover table", p2, used2)
    allF = {}
    for dname in sorted(glob.glob("P2/pilot_phi*")):
        pf, _, _ = load(glob.glob(os.path.join(dname, "*.jsonl")))
        for k, d in pf.items():
            allF[(dname,) + k] = d
    fruns = {k: d for k, d in allF.items()}
    check_certs("P2 F pilot (F and E reruns)", fruns, list(fruns))
    # all run records, even those not in tables
    allp = glob.glob("P1/main/*.jsonl") + glob.glob("P1/supp/*.jsonl") + glob.glob("P2/crossover*/*.jsonl")
    for fn in sorted(allp):
        with open(fn) as fh:
            for line in fh:
                d = json.loads(line)
                if d["kind"] == "run" and d["arm"] not in ARMS.values():
                    note(f"unexpected arm {d['arm']} in {fn}")

    # ---------------------------------------------- band sensitivity
    print("\nValidator 5a-style sensitivity (FPM units; w FPM per unpriced add/hash/compare on every arm):")
    print("| g | p | w | rho U variant | c/rhoE | D/rhoE | E/rhoE | GM c/min(D,E) |")
    print("|---|---|---|---|---|---|---|---|")
    flips = []
    for gp in sorted(allst):
        rows = (cells1.get(gp) or cells2.get(gp))
        b = band(rows, gp[0])
        for (w, wide), v in sorted(b.items()):
            print(f"| {gp[0]} | {gp[1]} | {w} | {'+native' if wide else 'uncharged only'} | {v['c']:.4g} | {v['D']:.4g} | {v['E']:.4g} | {v['cbest']:.4g} |")
        for a in ("c", "D", "E"):
            sides = {b[k][a] < 1 for k in b}
            if len(sides) > 1:
                flips.append((gp, a, "vs rho"))
        if len({b[k]["cbest"] > 1 for k in b}) > 1:
            flips.append((gp, "best-vs-c", ""))
    print("Order flips across the band (arm vs rho at 1; best(D,E) vs c at 1):", flips or "none")

    print(f"\n{len(findings)} findings")
    for f in findings:
        print(" -", f)


if __name__ == "__main__":
    main()
