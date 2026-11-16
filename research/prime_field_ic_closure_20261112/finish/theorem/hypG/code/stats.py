"""Exact statistics over all exhaustive_G outputs, recomputed from the per-target rows.  [pcc:unverified]"""
import json, glob, math, os, sys
sys.path.insert(0, os.path.dirname(__file__))
from thresholds_hypG import tau
files = sorted(glob.glob(os.path.join(os.path.dirname(__file__), "..", "out", "exh", "G_*.json")))
inst = []; tot_abs = 0
viol = {"line_det": 0, "det_kmm1": 0, "weak_kmm1": 0, "uncert": 0, "RM": 0}
fails = []; tc2_fail = []; maxG_frac = []; maxA = []
for f in files:
    try: D = json.load(open(f))
    except Exception: continue
    s = D["summary"]; rows = D["rows"]; pt = {q["t"]: q for q in D["per_t"]}
    m, k = s["m"], s["k"]; grid = s["grid"]; rho = s["rho"]
    H = {q["t"]: q["H"] for q in D["per_t"]}; H[-1] = 0
    inst.append(os.path.basename(f)); tot_abs += len(rows)
    for r in rows:
        if r["ddet"] is not None:
            sl = r["sline"]; b = k - math.ceil(math.log2(sl)) if sl > 1 else k
            viol["line_det"] += r["ddet"] < b
            viol["det_kmm1"] += r["ddet"] < k - m + 1
            viol["uncert"] += H[r["ddet"]] * r["nV"] < grid
            viol["RM"] += r["ddet"] < m * k - math.log2(r["nV"]) - 1e-9
        if not r["degen"]:
            viol["weak_kmm1"] += r["dweak"] < k - m + 1
    tC2 = math.ceil(tau(m) * m * k) if m >= 3 else 0
    f_ii = [t for t in pt if (not pt[t]["G_ii"]) and pt[t]["G_i"]]
    f_any = [t for t in pt if not pt[t]["G"]]
    name = f"m{m} k{k} p{s['p']} {s['gmode']}{' j0' if s['j0'] else ''} {os.path.basename(f).split('_')[4]}"
    if f_ii: fails.append((name, f_ii, rho, round(s["Ndec_targets"] / s["n"], 3)))
    if m >= 3 and not pt[tC2]["G"]:
        size_ok = pt[tC2]["H"] >= 4 * s["B"] ** (m / 2 - 1)
        tc2_fail.append((name, tC2, "G_i" if not pt[tC2]["G_i"] else "", "G_ii" if not pt[tC2]["G_ii"] else "",
                         round(s["Ndec_targets"] / s["n"], 3), "C1size_at_tC2=" + str(size_ok)))
    maxG_frac.append(s["max_t_original_G"] / rho if s["max_t_original_G"] is not None else None)
    maxA.append((s["max_t_all_targets_good_amended"], k - m))
out = {"instances": len(inst), "abscissas_total": tot_abs, "violating_targets": viol,
       "G_ii_fails_with_G_i_true": fails, "G_fails_at_tC2": tc2_fail, "n_G_fails_at_tC2": len(tc2_fail),
       "n_G_fails_at_tC2_with_C1_size": sum(1 for x in tc2_fail if x[-1].endswith("True")),
       "maxG_over_rho_range": [round(min(x for x in maxG_frac if x is not None), 3), round(max(x for x in maxG_frac if x is not None), 3)],
       "maxG_none": sum(x is None for x in maxG_frac),
       "maxA_ge_k_minus_m_all": all(a >= b for a, b in maxA), "maxA_range": [min(a for a, _ in maxA), max(a for a, _ in maxA)],
       "files": inst}
print(json.dumps(out, indent=1))
