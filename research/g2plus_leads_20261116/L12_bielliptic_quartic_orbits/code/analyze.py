"""Tables and mechanical verdict (SPEC.md section 11, PROTOCOL.md section 9)."""
import sys, os, json, argparse, glob, statistics, re


def med(v):
    return statistics.median(v) if v else None


def fmt(x, nd=3):
    if x is None:
        return "NA"
    if isinstance(x, float):
        return ("%." + str(nd) + "g") % x
    return str(x)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--instances", required=True)
    ap.add_argument("--runs", required=True)
    ap.add_argument("--summary", required=True)
    ap.add_argument("--decision", required=True)
    a = ap.parse_args()
    rawdir = os.path.dirname(os.path.abspath(a.summary))
    insts = [json.loads(l) for l in open(a.instances) if l.strip()]
    # ---- load IC runs
    runs = {}
    pat = re.compile(r"^(F1|F0)_p(\d+)_j(\d+)_s(\d+)_(A|B|C|Bp|N)\.json$")
    for fn in sorted(os.listdir(a.runs)):
        m = pat.match(fn)
        if not m:
            continue
        fam, p, j, s, arm = m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4)), m.group(5)
        d = json.load(open(os.path.join(a.runs, fn)))
        d.setdefault("family", fam); d["p"] = p; d["j"] = j; d["s"] = s; d["arm"] = arm
        runs[(fam, p, j, s, arm)] = d
    cols = ["family", "p", "j", "s", "arm", "status", "N_aff", "ncols", "trials", "relations",
            "rows_used", "R", "N", "W", "la_attempts", "SETUP_FPM", "COLLECT_FPM", "LA_FPM",
            "TOTAL", "certified", "sigma_equal", "stream_digest"]
    hdr = ["family", "p", "j", "s", "arm", "status", "N_aff", "ncols", "trials", "relations",
           "rows_used", "R", "N", "W", "la_attempts", "SETUP_FPM", "COLLECT_FPM", "LA_FPM",
           "TOTAL", "certified", "sigma_equal", "stream_digest"]
    with open(a.summary, "w") as f:
        f.write("\t".join(hdr) + "\n")
        for k in sorted(runs):
            d = runs[k]
            f.write("\t".join("" if (d.get(c) is None) else str(d.get(c)) for c in cols) + "\n")

    def ok(k):
        return k in runs and runs[k].get("status") == "ok"

    out_md = []
    P1 = [263, 523, 1031, 2063]
    P0 = [269, 521, 1033, 2069]
    # ---- gates
    gates = {}
    ul = os.path.join(rawdir, "unittest.log")
    g0 = False
    if os.path.exists(ul):
        lines = [l for l in open(ul).read().splitlines() if "OK" in l]
        g0 = bool(lines) and lines[-1].strip().startswith("OK")
    gates["G0"] = g0
    sl = os.path.join(rawdir, "select.log")
    g1 = (len(insts) == 32) and not (os.path.exists(sl) and "INSTRUMENT ERROR" in open(sl).read())
    gates["G1"] = bool(g1)
    notrun = [k for k, d in runs.items() if d.get("status") == "not_run"]
    done = {k: d for k, d in runs.items() if d.get("status") != "not_run"}
    g2tot = sum(d.get("g2_fail_count", 0) or 0 for d in done.values())
    g3tot = sum(d.get("g3_fail_count", 0) or 0 for d in done.values()) + \
        sum(1 for d in done.values() if d.get("status") == "G3_fail")
    gates["G2"] = (g2tot == 0)
    gates["G2_failures"] = g2tot
    gates["G3"] = (g3tot == 0)
    gates["G3_failures"] = g3tot
    t4 = [d.get("tamper_rejected") for k, d in done.items() if k[4] == "A" and k[1] == 263 and k[0] == "F1"]
    gates["G4"] = bool(t4) and all(v is True for v in t4)
    g5 = True
    byinst = {}
    for k, d in done.items():
        if k[4] in ("A", "B", "C", "N"):
            byinst.setdefault((k[0], k[1], k[2], k[3]), set()).add(d.get("stream_digest"))
    for key, st in byinst.items():
        if len(st) != 1:
            g5 = False
    gates["G5"] = g5
    checked = [d for k, d in done.items() if k[4] in ("A", "B", "C", "Bp")]
    g6_bad = [k for k, d in done.items() if k[4] in ("A", "B", "C", "Bp") and not d.get("certified")]
    gates["G6"] = (len(g6_bad) == 0) and len(checked) > 0
    gates["G6_uncertified_runs"] = ["%s_p%d_j%d_s%d_%s" % k for k in g6_bad]
    nrun = [d for k, d in done.items() if k[4] == "N"]
    ncert = sum(1 for d in nrun if d.get("certified"))
    gates["G7"] = (ncert == 0) and len(nrun) > 0
    gates["G7_N_certified"] = ncert
    gates["G7_N_runs"] = len(nrun)
    g8 = False
    rp = os.path.join(a.runs, "repeat_F1_p523_j0_s1_B.json")
    ref = runs.get(("F1", 523, 0, 1, "B"))
    if os.path.exists(rp) and ref:
        r2 = json.load(open(rp))
        r1 = dict(ref); r2 = dict(r2)
        for d in (r1, r2):
            d.pop("wall_s", None)
        r1 = {k: v for k, v in r1.items() if k in r2 or True}
        g8 = (json.dumps(r1, sort_keys=True) == json.dumps(r2, sort_keys=True))
    gates["G8"] = g8
    gates["not_run_count"] = len(notrun)
    gates["not_run_files"] = ["%s_p%d_j%d_s%d_%s" % k for k in sorted(notrun)]

    # ---- paired ratios
    def ratio(fam, p, j, s, num, den, metric):
        kn, kd = (fam, p, j, s, num), (fam, p, j, s, den)
        if ok(kn) and ok(kd) and runs[kd].get(metric):
            return runs[kn][metric] / runs[kd][metric]
        return None
    ratio_defs = [("TOTAL_A/TOTAL_B", "A", "B", "TOTAL"), ("COLLECT_A/COLLECT_B", "A", "B", "COLLECT_FPM"),
                  ("LA_A/LA_B", "A", "B", "LA_FPM"), ("rel_A/rel_B", "A", "B", "relations"),
                  ("ncols_A/ncols_B", "A", "B", "ncols"), ("TOTAL_B/TOTAL_C", "B", "C", "TOTAL"),
                  ("TOTAL_A/TOTAL_C", "A", "C", "TOTAL"), ("TOTAL_B/TOTAL_Bp", "B", "Bp", "TOTAL")]
    stats = {}
    excluded = {}
    for fam, primes in (("F1", P1), ("F0", P0)):
        arms_ok = ("A", "B", "C", "Bp") if fam == "F1" else ("A", "B", "C")
        for p in primes:
            js = sorted({i["j"] for i in insts if i["family"] == fam and i["p"] == p})
            for name, num, den, metric in ratio_defs:
                if fam == "F0" and "Bp" in (num, den):
                    continue
                vals = []
                exc = 0
                for j in js:
                    for s in (1, 2):
                        v = ratio(fam, p, j, s, num, den, metric)
                        if v is None:
                            exc += 1
                        else:
                            vals.append(v)
                stats[(fam, p, name)] = {"median": med(vals), "min": min(vals) if vals else None,
                                         "max": max(vals) if vals else None, "n": len(vals),
                                         "excluded": exc, "values": vals}
    out_md.append("## Gates\n")
    for k, v in gates.items():
        out_md.append("- %s: %s" % (k, v))
    out_md.append("\n## Paired ratios (median [min, max], n, excluded)\n")
    for fam, primes in (("F1", P1), ("F0", P0)):
        names = [n for n, *_ in ratio_defs if not (fam == "F0" and "Bp" in n)]
        out_md.append("### %s\n" % fam)
        out_md.append("| p | " + " | ".join(names) + " |")
        out_md.append("|---|" + "---|" * len(names))
        for p in primes:
            cells = []
            for n in names:
                s_ = stats[(fam, p, n)]
                cells.append("%s [%s, %s] n=%d x=%d" % (fmt(s_["median"]), fmt(s_["min"]), fmt(s_["max"]),
                                                       s_["n"], s_["excluded"]))
            out_md.append("| %d | " % p + " | ".join(cells) + " |")
        out_md.append("")
    # ---- per-run table
    out_md.append("## Per-run counts\n")
    out_md.append("| " + " | ".join(hdr[:-1]) + " |")
    out_md.append("|" + "---|" * (len(hdr) - 1))
    for k in sorted(runs):
        d = runs[k]
        out_md.append("| " + " | ".join(fmt(d.get(c), 6) if not isinstance(d.get(c), str) else d.get(c)
                                         for c in hdr[:-1]) + " |")
    # ---- rho
    rho = {}
    for fn in glob.glob(os.path.join(a.runs, "R_p*_j*_w*.json")):
        d = json.load(open(fn))
        if d.get("status") == "not_run":
            continue
        rho.setdefault((d["p"], d["j"]), []).append(d)
    rho_med = {k: med([x["fpm"] for x in v]) for k, v in rho.items()}
    rho_correct = {k: all(x["correct"] for x in v) for k, v in rho.items()}
    out_md.append("\n## Rho reference (median FPM over walk seeds), gap TOTAL_X / rho median\n")
    out_md.append("| p | j | rho n | rho median FPM | all correct | gap A | gap B | gap C |")
    out_md.append("|---|---|---|---|---|---|---|---|")
    gaps = {}
    for p in P1:
        for i in sorted([i for i in insts if i["family"] == "F1" and i["p"] == p], key=lambda i: i["j"]):
            j = i["j"]
            rm = rho_med.get((p, j))
            cells = []
            for arm in "ABC":
                vs = []
                for s in (1, 2):
                    if ok(("F1", p, j, s, arm)) and rm:
                        vs.append(runs[("F1", p, j, s, arm)]["TOTAL"] / rm)
                gaps[(p, j, arm)] = med(vs)
                cells.append(fmt(med(vs)))
            out_md.append("| %d | %d | %d | %s | %s | %s |" % (p, j, len(rho.get((p, j), [])), fmt(rm),
                                                              rho_correct.get((p, j)), " | ".join(cells)))
    # ---- yields
    yields = {}
    for p in P1:
        sp = tr = 0
        for j in range(4):
            for s in (1, 2):
                k = ("F1", p, j, s, "A")
                if ok(k):
                    sp += runs[k]["split_trials"]; tr += runs[k]["trials"]
        gsp = gtr = 0
        for fn in glob.glob(os.path.join(a.runs, "G_p%d_j*.json" % p)):
            d = json.load(open(fn))
            if "split_trials" in d:
                gsp += d["split_trials"]; gtr += d["trials"]
        yields[p] = {"F1_A_yield": (sp / tr) if tr else None, "G_yield": (gsp / gtr) if gtr else None,
                     "F1_split": sp, "F1_trials": tr, "G_split": gsp, "G_trials": gtr}
    out_md.append("\n## Relation yield per trial (split fraction)\n")
    out_md.append("| p | F1 arm-A split/trials | yield | G split/trials | yield | |diff| |")
    out_md.append("|---|---|---|---|---|---|")
    famctl = True
    for p in P1:
        y = yields[p]
        diff = abs(y["F1_A_yield"] - y["G_yield"]) if (y["F1_A_yield"] is not None and y["G_yield"] is not None) else None
        if diff is None or diff > 0.05:
            famctl = False
        out_md.append("| %d | %d/%d | %s | %d/%d | %s | %s |" % (p, y["F1_split"], y["F1_trials"], fmt(y["F1_A_yield"]),
                                                                 y["G_split"], y["G_trials"], fmt(y["G_yield"]), fmt(diff)))
    # ---- decision quantities
    R_p = {p: stats[("F1", p, "TOTAL_A/TOTAL_B")]["median"] for p in P1}
    vals2063 = stats[("F1", 2063, "TOTAL_A/TOTAL_B")]["values"]
    n_ge = sum(1 for v in vals2063 if v >= 1.5)
    aa = stats[("F1", 2063, "TOTAL_B/TOTAL_Bp")]["values"]
    aa_spread = (max(aa) / min(aa)) if aa else None
    pc_AC = stats[("F0", 2069, "TOTAL_A/TOTAL_C")]["median"]
    pc_AB = stats[("F0", 2069, "TOTAL_A/TOTAL_B")]["median"]
    pc_pass = (pc_AC is not None and pc_AB is not None and pc_AC >= 6 and pc_AB >= 2)
    complete = {p: (stats[("F1", p, "TOTAL_A/TOTAL_B")]["n"] == 8) for p in P1}
    b_fail_a_ok = []
    for (fam, p, j, s, arm), d in runs.items():
        if fam == "F1" and arm == "B" and d.get("status") != "not_run":
            ka = ("F1", p, j, s, "A")
            if ka in runs and runs[ka].get("certified") and not d.get("certified"):
                b_fail_a_ok.append((p, j, s))
    other_gates = ["G0", "G1", "G2", "G4", "G5", "G6", "G7", "G8"]
    invalid = any(not gates[g] for g in other_gates)
    R2063 = R_p[2063]
    if invalid:
        verdict = "invalid"
    elif (not gates["G3"]) or (R2063 is not None and R2063 < 1.5) or b_fail_a_ok:
        verdict = "killed"
    else:
        have = [p for p in P1 if R_p[p] is not None]
        sup = (R2063 is not None and n_ge >= 7 and len(vals2063) == 8
               and all(R_p[p] >= 1.5 for p in have) and pc_pass
               and aa_spread is not None and aa_spread <= 1.5 and len(notrun) == 0)
        verdict = "supported" if sup else "inconclusive"
    out_md.append("\n## Decision quantities\n")
    out_md.append("- R_p (median TOTAL_A/TOTAL_B): " + ", ".join("%d: %s" % (p, fmt(R_p[p])) for p in P1))
    out_md.append("- complete data (8 pairs) per p: %s" % complete)
    out_md.append("- values >= 1.5 at 2063: %d of %d" % (n_ge, len(vals2063)))
    out_md.append("- A/A TOTAL_B/TOTAL_Bp at 2063: min %s max %s spread max/min %s" %
                  (fmt(min(aa) if aa else None), fmt(max(aa) if aa else None), fmt(aa_spread)))
    out_md.append("- positive control at 2069: median TOTAL_A/TOTAL_C %s (>=6), median TOTAL_A/TOTAL_B %s (>=2), pass %s" %
                  (fmt(pc_AC), fmt(pc_AB), pc_pass))
    out_md.append("- arm B uncertified where A certified: %s" % b_fail_a_ok)
    out_md.append("- family control (|yield diff| <= 0.05 at every prime): %s" % famctl)
    out_md.append("- lab-baseline reading, median TOTAL_B/TOTAL_C: " +
                  ", ".join("%d: %s" % (p, fmt(stats[("F1", p, "TOTAL_B/TOTAL_C")]["median"])) for p in P1))
    out_md.append("- VERDICT (mechanical, PROTOCOL.md section 9): %s" % verdict)
    dec = {"gates": gates, "R_p": {str(p): R_p[p] for p in P1},
           "count_ge_1.5_at_2063": n_ge, "n_values_2063": len(vals2063),
           "AA_2063_values": aa, "AA_2063_spread": aa_spread,
           "positive_control_2069": {"median_TOTAL_A_over_C": pc_AC, "median_TOTAL_A_over_B": pc_AB, "pass": pc_pass},
           "arm_B_uncertified_where_A_certified": b_fail_a_ok,
           "family_control_no_effect": famctl, "yields": {str(p): yields[p] for p in P1},
           "ratio_stats": {"%s|%d|%s" % k: {x: y for x, y in v.items()} for k, v in stats.items()},
           "rho_median_fpm": {"%d|%d" % k: v for k, v in rho_med.items()},
           "rho_all_correct": {"%d|%d" % k: v for k, v in rho_correct.items()},
           "gap_TOTAL_over_rho": {"%d|%d|%s" % k: v for k, v in gaps.items()},
           "verdict": verdict}
    with open(a.decision, "w") as f:
        json.dump(dec, f, sort_keys=True, indent=1)
        f.write("\n")
    print("\n".join(out_md))


if __name__ == "__main__":
    main()
