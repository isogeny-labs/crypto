import json
S = "research/lit2026_w5_lane_batched_f4_20261011/reports/run-20261012T0008Z/summary.json"
d = json.load(open(S))
out = []
for c in d["cells"]:
    a = c["arms"]
    cpu = lambda arm: (a[arm].get("cpu_ms") or {}).get("median")
    rec = {
        "cell": c["cell"], "engine": c["engine"],
        "orig_word_xors": a["orig"]["word_xors"],
        "gaterref_word_xors": a["gaterref"]["word_xors"],
        "orig_over_gaterref_word_xors": a["orig"]["word_xors"] / a["gaterref"]["word_xors"],
        "cpu_prev_median_ms": cpu("prev"), "cpu_gaterref_median_ms": cpu("gaterref"),
        "cpu_prev_over_gaterref_medians": (cpu("prev") / cpu("gaterref")) if cpu("prev") and cpu("gaterref") else None,
        "stored_ratio_orig_gaterref_word_xors": c["ratios"]["orig/gaterref"]["word_xors"],
        "stored_ratio_prev_gaterref_cpu_median": (c["ratios"]["prev/gaterref"]["cpu_round_ratios"] or {}).get("median"),
    }
    out.append(rec)
for r in out:
    print(json.dumps(r, sort_keys=True))
f6 = [r for r in out if r["engine"] == "f6p"]
print("f6p orig/gaterref word_xors:", [round(r["orig_over_gaterref_word_xors"], 4) for r in f6])
print("f6p cpu prev/gaterref medians:", [round(r["cpu_prev_over_gaterref_medians"], 4) for r in f6])
print("f6p stored cpu prev/gaterref round-median:", [round(r["stored_ratio_prev_gaterref_cpu_median"], 4) for r in f6 if r["stored_ratio_prev_gaterref_cpu_median"]])
import statistics as st
print("--- paired per-round CPU ratios prev/gaterref (study rule: median of 5 per-round ratios)")
for c in d["cells"]:
    if c["engine"] != "f6p": continue
    p = c["arms"]["prev"]["cpu_ms"]["values"]; g = c["arms"]["gaterref"]["cpu_ms"]["values"]
    rr = [x / y for x, y in zip(p, g)]
    print(c["cell"], "per-round", [round(x, 4) for x in rr], "median", round(st.median(rr), 4), "n", len(rr))
