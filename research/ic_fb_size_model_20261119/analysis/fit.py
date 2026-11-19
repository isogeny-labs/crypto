"""Fit GAE(F) = a*F^2 + b*N/(4F) per curve and score M1-M5 (PROTOCOL.md)."""
import json, math, sys, statistics as st
from pathlib import Path
D = Path(__file__).resolve().parent.parent
out = {}
for c in ["p24", "p28"]:
    recs = [json.loads(l) for l in open(D / "sessions" / c / "records.jsonl")]
    recs = [r for r in recs if r.get("status", r.get("verdict", "verified")) and r.get("cost")]
    N = None; rows = {}; rho = []
    for r in recs:
        arm = r["arm"]
        if arm == "rho":
            rho.append(r["cost"]["s"]); continue
        N = int(r["workload"]["curve"]["group_order"])
        F = int(arm[2:])
        d = rows.setdefault(F, {"gae": [], "s": [], "trials": [], "rel": []})
        d["gae"].append(r["cost"]["total_gae"]); d["s"].append(r["cost"]["s"])
        d["trials"].append(r["counters"]["targets_tried"]); d["rel"].append(r["counters"]["relations_found"])
    Fs = sorted(rows)
    g = [st.mean(rows[F]["gae"]) for F in Fs]
    # least squares on relative error: minimise sum ((a x1 + b x2 - g)/g)^2
    x1 = [F * F / gi for F, gi in zip(Fs, g)]; x2 = [N / (4 * F) / gi for F, gi in zip(Fs, g)]
    s11 = sum(v * v for v in x1); s22 = sum(v * v for v in x2); s12 = sum(u * v for u, v in zip(x1, x2))
    t1 = sum(x1); t2 = sum(x2); det = s11 * s22 - s12 * s12
    a = (t1 * s22 - t2 * s12) / det; b = (s11 * t2 - s12 * t1) / det
    table = []
    for F, gi in zip(Fs, g):
        pred = a * F * F + b * N / (4 * F)
        tr = st.mean(rows[F]["trials"]); rel = st.mean(rows[F]["rel"])
        table.append({"F": F, "mean_gae": gi, "mean_S": st.mean(rows[F]["s"]),
                      "fit_gae": pred, "rel_resid": (gi - pred) / gi,
                      "trials": tr, "trials_over_model": tr / (N / (4 * F)),
                      "relations": rel, "relations_over_F": rel / F})
    best = min(table, key=lambda t: t["mean_S"])
    out[c] = {"N": N, "a": a, "b": b, "Fstar_model": N ** (1 / 3) / 2,
              "best_F_w0": best["F"], "best_S_w0": best["mean_S"],
              "rho_S_w0": st.mean(rho), "rows": table}
out["M5_ratio"] = out["p28"]["best_S_w0"] / out["p24"]["best_S_w0"]
json.dump(out, open(D / "analysis" / "fit.json", "w"), indent=1)
for c in ["p24", "p28"]:
    o = out[c]
    print(f"{c}: N={o['N']} a={o['a']:.3f} b={o['b']:.3f} F*model={o['Fstar_model']:.0f} bestF={o['best_F_w0']} S={o['best_S_w0']:.2f} rho={o['rho_S_w0']:.3f}")
    for t in o["rows"]:
        print(f"  F={t['F']:5d} S={t['mean_S']:8.2f} gae={t['mean_gae']:10.0f} fit={t['fit_gae']:10.0f} resid={t['rel_resid']:+.3f} trials/model={t['trials_over_model']:.2f} rel/F={t['relations_over_F']:.2f}")
print("M5 ratio", round(out["M5_ratio"], 3))
