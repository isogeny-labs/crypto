"""Aggregate exhaustive_G outputs into a markdown table + JSON.  [pcc:unverified]"""
import json, glob, math, os, sys
from math import comb
sys.path.insert(0, os.path.dirname(__file__))
from thresholds_hypG import tau
out = []
for f in sorted(glob.glob(os.path.join(os.path.dirname(__file__), "..", "out", "exh", "G_*.json"))):
    try:
        D = json.load(open(f))
    except Exception:
        print("skip", os.path.basename(f), file=sys.stderr); continue
    s = D["summary"]; pt = {q["t"]: q for q in D["per_t"]}
    m, k = s["m"], s["k"]
    tC2 = math.ceil(tau(m) * m * k) if m >= 3 else 0
    B = s["B"]
    sizeC1 = lambda t: pt[t]["H"] >= 4 * B ** (m / 2 - 1)
    # original G at t >= tC2: does some t >= tC2 satisfy G?  failures below the max good t?
    G_ts = [t for t in pt if pt[t]["G"]]
    fails_low = [t for t in pt if not pt[t]["G"] and t <= (s["max_t_original_G"] or -1)]
    tf = k - m   # proven all-good t (amended)
    rec = {"file": os.path.basename(f), "m": m, "k": k, "p": s["p"], "n": s["n"], "g": s["gmode"], "j0": s["j0"],
           "rho": s["rho"], "B": B, "Ndec": s["Ndec_targets"], "Ndec_over_n": round(s["Ndec_targets"] / s["n"], 3),
           "degen_absc": s["n_degenerate_abscissas"], "Delta": s["Delta_values"],
           "floor": s["proven_floor_k_m_1"], "min_dweak": s["min_dweak_nondegen"], "min_ddet": s["min_ddet"],
           "min_ddet_nondeg": s["min_ddet_nondegen"],
           "floors_ok": s["weak_floor_k_m_1_ok"] and s["det_floor_k_m_1_ok"] and s["det_line_floor_ok"] and s["uncertainty_ok"],
           "RM_viol": s["RM_analogue_violations"],
           "tC2": tC2, "G_at_tC2": pt.get(tC2, {}).get("G"), "maxG": s["max_t_original_G"],
           "G_fail_ts_below_maxG": fails_low,
           "nu_at_maxG": pt[s["max_t_original_G"]]["nu"] if s["max_t_original_G"] is not None else None,
           "bound_at_maxG": pt[s["max_t_original_G"]]["bound_ii"] if s["max_t_original_G"] is not None else None,
           "G_ii_fail_ts": [t for t in pt if not pt[t]["G_ii"]],
           "G_ii_fail_ts_with_G_i": [t for t in pt if not pt[t]["G_ii"] and pt[t]["G_i"]],
           "maxA": s["max_t_all_targets_good_amended"],
           "C1_size_at_maxG": sizeC1(s["max_t_original_G"]) if s["max_t_original_G"] is not None else None,
           "sec": s["sec"]}
    out.append(rec)
json.dump(out, open(os.path.join(os.path.dirname(__file__), "..", "out", "exh_summary.json"), "w"), indent=1)
cols = ["m", "k", "p", "n", "g", "j0", "rho", "B", "Ndec_over_n", "degen_absc", "Delta", "floor", "min_dweak", "min_ddet",
        "min_ddet_nondeg", "floors_ok", "RM_viol", "tC2", "G_at_tC2", "maxG", "G_fail_ts_below_maxG", "G_ii_fail_ts", "G_ii_fail_ts_with_G_i", "maxA", "C1_size_at_maxG"]
print("| " + " | ".join(cols) + " |"); print("|" + "---|" * len(cols))
for r in sorted(out, key=lambda r: (r["m"], r["p"], r["k"], r["g"], r["j0"], r["file"])):
    print("| " + " | ".join(str(r[c]) for c in cols) + " |")
print(f"\ninstances: {len(out)}; floors_ok all: {all(r['floors_ok'] for r in out)}; RM violations total: {sum(r['RM_viol'] for r in out)}")
