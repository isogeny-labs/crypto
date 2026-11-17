"""R7.5 recount for the R6.9 erratum.  Reads the frozen relrate.json and counts the
runs whose measured smooth rate differs from Gaudry's 1/g! by more than 10x.
Computes nothing new beyond ratios of recorded fields; writes recount_v2.json."""
import json
rel = json.load(open("relrate.json"))
print("runs:", len(rel))
print(f"{'q':>3s} {'g':>2s} {'seed':>4s} {'measured':>9s} {'1/g!':>10s} {'meas/(1/g!)':>12s} {'recorded':>9s} {'>10x':>5s} {'<0.1x':>5s}")
over, under, rows = [], [], []
for r in rel:
    ratio = r["measured_rate"] / r["model_heuristic_inv_gfact"]
    assert abs(ratio - r["ratio_measured_over_heuristic"]) < 5e-4 * max(1, ratio), (r["q"], r["genus"], r["seed"])
    hi, lo = ratio > 10, ratio < 0.1
    if hi: over.append((r["q"], r["genus"], r["seed"], ratio))
    if lo: under.append((r["q"], r["genus"], r["seed"], ratio))
    rows.append({"q": r["q"], "g": r["genus"], "seed": r["seed"], "measured": r["measured_rate"], "inv_gfact": r["model_heuristic_inv_gfact"],
                 "ratio": ratio, "recorded_ratio": r["ratio_measured_over_heuristic"], "gt10": hi, "lt_0.1": lo})
    print(f"{r['q']:3d} {r['genus']:2d} {r['seed']:4d} {r['measured_rate']:9.4f} {r['model_heuristic_inv_gfact']:10.3e} {ratio:12.1f} {r['ratio_measured_over_heuristic']:9.1f} {'YES' if hi else '':>5s} {'YES' if lo else '':>5s}")
print(f"\nruns with meas/(1/g!) > 10 : {len(over)} of {len(rel)}")
for q, g, s, x in over: print(f"   q={q} g={g} seed={s}: {x:.1f}x")
print(f"runs with meas/(1/g!) < 0.1: {len(under)} of {len(rel)}")
by_q = {}
for q, g, s, x in over: by_q.setdefault(q, []).append((g, s))
print("by q:", {q: len(v) for q, v in sorted(by_q.items())})
near = [(r["q"], r["genus"], r["seed"], round(x["ratio"], 2)) for r, x in zip(rel, rows) if 5 < x["ratio"] <= 10]
print("runs in (5,10]x (not counted):", near)
ex = [r["ratio_measured_over_exact"] for r in rel]
print(f"meas/exact range: {min(ex):.2f} to {max(ex):.2f}; runs outside [1/4,4]: {sum(1 for v in ex if v>4 or v<0.25)}")
sm = [x for x in rows if x["q"] == 4 and x["g"] == 8]
print("smallest weak instance (q=4,g=8) ratios:", [round(x["ratio"], 1) for x in sm])
def rng(f):
    v = [x["ratio"] for x in rows if f(x)]; return (round(min(v), 1), round(max(v), 1), len(v))
print("range q>=8, g<=4:", rng(lambda x: x["q"] >= 8 and x["g"] <= 4))
print("range q>=16 (all g):", rng(lambda x: x["q"] >= 16))
print("range q=8, g=6:", rng(lambda x: x["q"] == 8 and x["g"] == 6))
json.dump({"runs": len(rel), "gt10": [{"q": q, "g": g, "seed": s, "ratio": x} for q, g, s, x in over], "count_gt10": len(over),
           "count_lt_0.1": len(under), "rows": rows}, open("recount_v2.json", "w"), indent=1)

# neighbouring statements in RESULTS.md section 3, re-derived from the same records
print("\nneighbouring statements")
print("  order checks / failures:", sum(r["order_checks"] for r in rel), sum(r["order_check_failures"] for r in rel), "(text: 136 checks, 0 failures)")
print("  max binomial standard error:", round(max(r["measured_rate_se"] for r in rel), 5), "(text: <= 0.0025)")
print("  walk_steps values:", sorted(set(r["walk_steps"] for r in rel)))
