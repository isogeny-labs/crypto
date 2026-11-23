"""EXP-PKM-ODD-001 Amendment 5: descriptive reading of the 24 h extension rows (runs/xb/).

Not part of the frozen decision rule (§5): the frozen verdicts stay in results.json.
Reads the extension rows with the committed m = 3 rows and writes results_extension.json:
- validity of the extension rows (F4 vs exhaustive, oracle vs enumeration, peak RSS);
- the m = 3 degree-2 reference extended to S = 32 with the extension's one draw, and the
  A3 (S = 27) ratio against it (same band rule, reported as descriptive);
- part B m = 3 algebraic/enumeration ratios at S = 32 and 36, next to the committed ones.
"""
import glob
import json
import math
import statistics


def rows(pattern):
    out = []
    for f in sorted(glob.glob(pattern)):
        out += [json.loads(l) for l in open(f) if l.strip()]
    return out


def band(r):
    return "gain" if r <= 0.5 else "loss" if r >= 2 else "no_material_effect"


xa = [r for r in rows("runs/xb/XB-M3-*.jsonl")]
xbr = [r for r in rows("runs/xb/XB-B-*.jsonl")]
rss = [json.load(open(f)) for f in sorted(glob.glob("runs/xb/*.rss.json"))]
main = rows("runs/M3-*.jsonl")
out = {"schema": "pkm-odd-tower-f4.extension/v1",
       "validity": {"part_a": [(r["cell"], r["agree"], r["censored"]) for r in xa],
                    "part_b": [(r["cell"], r["agree"], r["alg_undetermined"]) for r in xbr],
                    "peak_rss_gib": {x["cell"]: round(x["maxrss_kb"] / 2**20, 2) for x in rss},
                    "wall_s": {x["cell"]: round(x["wall_s"]) for x in rss}}}

# m = 3 degree-2 reference: committed tower cells t = 2..4 (median) plus the extension's t = 5 (one draw).
ref = {}
for r in main:
    if r["cell"].startswith("M3-A2-t") and r["control"] == "tower" and not r["censored"]:
        ref.setdefault(r["log2S"], []).append(r["field_ops"])
for r in xa:
    if r["cell"] == "XB-M3-A2-t5-tower" and not r["censored"]:
        ref.setdefault(r["log2S"], []).append(r["field_ops"])
pts = sorted((x, math.log2(statistics.median(v))) for x, v in ref.items())
out["m3_reference_points"] = [(round(2 ** x), round(2 ** y)) for x, y in pts]


def interp(x):
    for (x0, y0), (x1, y1) in zip(pts, pts[1:]):
        if x0 <= x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return None


a3 = [r for r in main if r["cell"] == "M3-A3-k3-tower" and not r["censored"]]
y = interp(a3[0]["log2S"]) if a3 else None
if y is not None:
    rs = [r["field_ops"] / 2 ** y for r in a3]
    out["m3_A3_k3_vs_extended_reference"] = {"S": 27, "r": rs, "bands": [band(v) for v in rs]}
a3n = [r for r in main if r["cell"] == "M3-A3-k3-null" and not r["censored"]]
out["m3_A3_k3_null_field_ops"] = [r["field_ops"] for r in a3n]
out["m3_A23_n4_tower_field_ops"] = [r["field_ops"] for r in xa if r["cell"] == "XB-M3-A23-n4-tower"]

# Part B, m = 3: committed cells plus the extension.
pb = {}
for r in rows("runs/B-M3-*.jsonl") + xbr:
    if r.get("alg_undetermined"):
        continue
    arm = r["cell"].replace("XB-", "").split("-")[2]
    pb.setdefault(f"{r['form']}_{arm}", {}).setdefault(r["S"], []).append(r["ratio_alg_over_enum"])
out["part_b_m3_ratios"] = {k: {s: [round(x, 1) for x in v] for s, v in sorted(d.items())} for k, d in pb.items()}
json.dump(out, open("results_extension.json", "w"), indent=1)
print(json.dumps(out, indent=1))
