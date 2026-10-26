#!/usr/bin/env python3
"""analysis/summary.json: every PROTOCOL.md prediction, evaluated mechanically
from the committed ecbench outputs (band --json, compare --save), the null
JSONs and, for R4, the residual study's band JSONs. Reads nothing else."""
import glob
import json
import math
import random
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
R = ["R-p16", "R-p18", "R-p36", "R-p37"]
B = ["B-hb19", "B-hb23", "B-hb25", "B-hb27"]
K = ["K-k47", "K-k57", "K-k41", "K-k53", "K-k59", "K-k61"]
EARLIER_R = ROOT.parent / "ic_prime_residual_lp_20261026" / "analysis"


def band(s):
    return json.loads((ROOT / "analysis" / f"band-{s}.json").read_text())


def mean_s(bd):
    return {r["arm"]: dict(zip(bd["weights"], r["mean_s"])) for r in bd["rows"]}


def cmp(s, a, b, w):
    hits = []
    for f in glob.glob(str(ROOT / "sessions" / s / "comparisons" / f"{a}__{b}*.json")):
        d = json.loads(Path(f).read_text())
        if float(d.get("pricing_weight") or 0) == float(w):
            hits.append(d)
    if len(hits) != 1:
        raise SystemExit(f"{s} {a}->{b} w={w}: {len(hits)} saved comparisons")
    o = hits[0]["ops"]
    return {"ratio": o["ratio_b_over_a"], "ci95": o["ci95"], "pairs": o["pairs"]}


def below(c):
    return c["ci95"][1] < 1


def above(c):
    return c["ci95"][0] > 1


def slope(points):
    xs, ys = zip(*points)
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in points) / sum((x - mx) ** 2 for x in xs)


def null_case(path):
    d = json.loads(path.read_text())
    c, f = d["curve"]["group_ops"], d["curve_free"]["group_ops"]
    rng = random.Random(20261107)
    boot = []
    for _ in range(4000):
        mc = sum(rng.choice(c) for _ in c) / len(c)
        mf = sum(rng.choice(f) for _ in f) / len(f)
        boot.append(mf / mc)
    boot.sort()
    ratio = (sum(f) / len(f)) / (sum(c) / len(c))
    return {"case": path.stem, "r": d["r"], "base": d["base"], "runs": d["runs"],
            "verified": [d["curve"]["verified"], d["curve_free"]["verified"]],
            "ratio": ratio, "ci95": [boot[99], boot[3899]],
            "excludes_1": not (boot[99] <= 1 <= boot[3899])}


def main():
    out = {"schema": "ic_planner_exhaustive.summary/v1", "sessions": {}, "predictions": {}}
    for s in R + B + K:
        bd = band(s)
        out["sessions"][s] = {"slug": bd["rows"][0]["curve"], "log2_r": bd["rows"][0]["log2_r"],
                              "verified": {r["arm"]: [r["verified"], r["runs"]] for r in bd["rows"]},
                              "mean_s": mean_s(bd)}
    P = out["predictions"]

    # R1: res4/plan < 1 at w = 0, interval below 1, at every new size.
    r1 = {s: cmp(s, "plan", "res4", 0) for s in R}
    P["R1"] = {"rows": r1, "held": all(below(c) for c in r1.values())}
    # R2: S_1(res4)/S_1(plan) < 1.33 at p36 and p37.
    r2 = {s: cmp(s, "plan", "res4", 1) for s in ["R-p36", "R-p37"]}
    P["R2"] = {"rows": r2, "held": all(c["ratio"] < 1.33 for c in r2.values())}
    # R3 (risky): at p37 the w = 1 point estimate of res4/plan is below 1.
    r3 = cmp("R-p37", "plan", "res4", 1)
    P["R3"] = {"row": r3, "held": r3["ratio"] < 1,
               "w0_and_w1_orders_differ": (cmp("R-p37", "plan", "res4", 0)["ratio"] < 1) != (r3["ratio"] < 1)}
    # R4: slope of ln GAE on ln r over all ten sizes, at w = 0.
    pts = {"res4": [], "plan": []}
    for f in sorted(glob.glob(str(EARLIER_R / "band-p*.json"))) + [str(ROOT / "analysis" / f"band-{s}.json") for s in R]:
        bd = json.loads(Path(f).read_text())
        for r in bd["rows"]:
            if r["arm"] in pts:
                lr = r["log2_r"] * math.log(2)
                pts[r["arm"]].append((lr, math.log(r["mean_s"][0]) + lr / 2))
    sl = {a: slope(p) for a, p in pts.items()}
    P["R4"] = {"points": {a: len(p) for a, p in pts.items()}, "slope": sl,
               "held": 0.45 <= sl["res4"] <= 0.55 and 0.60 <= sl["plan"] <= 0.70}
    # R5: the cheapest residual arm at w = 0 is res2 or res4 at every size.
    r5 = {s: min(["res2", "res4", "res8", "res16", "res32"], key=lambda a: out["sessions"][s]["mean_s"][a][0.0])
          for s in R}
    P["R5"] = {"cheapest": r5, "held": all(v in ("res2", "res4") for v in r5.values())}

    # N1: at most 2 of 16 null intervals exclude 1.
    nulls = [null_case(p) for p in sorted((ROOT / "analysis").glob("null-*.json"))]
    P["N1"] = {"cases": nulls, "n": len(nulls), "excluding_1": sum(c["excludes_1"] for c in nulls),
               "held": len(nulls) == 16 and sum(c["excludes_1"] for c in nulls) <= 2}

    # W1: S_1(m141)/S_1(plan) < 1 with interval below 1 on >= 3 of 4; at w = 0, ratio >= 1 on >= 3 of 4.
    w1 = {s: {"0": cmp(s, "plan", "m141", 0), "1": cmp(s, "plan", "m141", 1)} for s in R}
    P["W1"] = {"rows": w1,
               "w1_below": sum(below(v["1"]) for v in w1.values()),
               "w0_at_least_1": sum(v["0"]["ratio"] >= 1 for v in w1.values())}
    P["W1"]["held"] = P["W1"]["w1_below"] >= 3 and P["W1"]["w0_at_least_1"] >= 3
    # W2: at w = 0 and 1, dm1/plan and dp1/plan each > 1 on >= 3 of 4.
    w2 = {s: {b: {w: cmp(s, "plan", b, w) for w in ("0", "1")} for b in ("dm1", "dp1")} for s in B}
    cnt = {f"{b}@{w}": sum(w2[s][b][w]["ratio"] > 1 for s in B) for b in ("dm1", "dp1") for w in ("0", "1")}
    P["W2"] = {"rows": w2, "counts_above_1": cnt, "held": all(v >= 3 for v in cnt.values())}
    # W3: S_1(pipeline71)/S_1(pipeline) < 1 with interval below 1 on >= 4 of 6; at w = 0 > 1 on >= 4 of 6.
    w3 = {s: {w: cmp(s, "pipeline", "pipeline71", w) for w in ("0", "0.3", "1")} for s in K}
    P["W3"] = {"rows": w3,
               "w1_below": sum(below(v["1"]) for v in w3.values()),
               "w0_above_1": sum(v["0"]["ratio"] > 1 for v in w3.values())}
    P["W3"]["held"] = P["W3"]["w1_below"] >= 4 and P["W3"]["w0_above_1"] >= 4

    # K1: descent/pipeline > 1 with interval above 1 at all six sizes (counted S).
    k1 = {s: cmp(s, "pipeline", "descent", 0) for s in K}
    P["K1"] = {"rows": k1, "held": all(above(c) for c in k1.values())}
    # K2: descent/rho and pipeline/rho each grow with log2 r.
    order = sorted(K, key=lambda s: out["sessions"][s]["log2_r"])
    k2 = {a: [(s, out["sessions"][s]["log2_r"], cmp(s, "rho", a, 0)["ratio"]) for s in order]
          for a in ("pipeline", "descent")}
    P["K2"] = {"rows": k2, "held": all(all(x[2] < y[2] for x, y in zip(v, v[1:])) for v in k2.values()),
               "spearman_like_increasing_pairs": {a: sum(x[2] < y[2] for x, y in zip(v, v[1:])) for a, v in k2.items()}}

    P["all_verified"] = all(v[0] == v[1] for s in out["sessions"].values() for v in s["verified"].values())
    (ROOT / "analysis" / "summary.json").write_text(json.dumps(out, indent=1) + "\n")
    for k, v in P.items():
        print(k, v if isinstance(v, bool) else v.get("held"))


if __name__ == "__main__":
    main()
