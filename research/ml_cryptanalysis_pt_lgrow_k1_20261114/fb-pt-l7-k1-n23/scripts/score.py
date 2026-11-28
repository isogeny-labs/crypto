"""Frozen analysis for PROTOCOL.md (pair-table l = 7 vs strong rho, K1 n = 23). Reads sessions/<job>/session/records.jsonl.
S per record: the section 5a band function of ../fb-ridge-vs-pairtable-k1/scripts/score.py (validated, VAL-20261113-d120d7),
copied verbatim below; bitop/v1 likewise.
Solvability rule: if any measured ic-pt-l7 record is not `verified`, the result is DOES_NOT_SOLVE and no gap is reported.
Primary: gap = log2(sum S_ic-pt-l7 / sum S_rho) at n = 23, bootstrap 95% CI (10,000 resamples over 8 targets, Random(0)).
Secondary: slope of the pt-l7 gap in n, OLS over n = 17, 19 (point estimates from ../fb-ridge-vs-pairtable-k1/results.json)
and n = 23, and the two-point slope 19 -> 23; the slope CI resamples only the n = 23 targets (the n = 17 and 19 points are
held at their point estimates), so it understates the full uncertainty."""
import json, math, random, sys, pathlib
HERE = pathlib.Path(__file__).resolve().parents[1]
W = (0.0, 0.1, 1.0); B = 10_000
def f_of(r): return r["cost"]["total_gae"] / r["cost"]["s"]
def unpriced(r): return sum(r["counters"].get(u, 0) for u in (r["cost"].get("unpriced") or []))
def native(r):
    ph = {p["name"]: p for p in r["phases"]}; g = lambda a, k: (ph.get(a, {}).get("native") or {}).get(k, 0)
    return ((r.get("solver") or {}).get("ops", 0) + g("relations", "trials") + g("relations", "lookups")
            + g("factor_base", "as_solves") + g("factor_base", "frobenius_maps") + g("linear_algebra", "row_ops") + unpriced(r))
def S(r, unit, w):
    f = f_of(r)
    if unit == "bitop": return sum(int(p["bitops"]) for p in r["work"]["phases"]) / f
    if not r["arm"].startswith("ic-"): return (r["cost"]["total_gae"] + w * unpriced(r)) / f
    return (sum(p["adds"] + p["doubles"] for p in r["phases"]) + w * native(r)) / f
def load():
    t = {}
    for rec in sorted((HERE / "sessions").glob("*/session/records.jsonl")):
        for line in open(rec):
            r = json.loads(line)
            if not r["warmup"]: t.setdefault(r["workload"]["workload_id"], {}).setdefault(r["arm"], []).append(r)
    return t
def gap(ts, unit, w):
    return math.log2(sum(S(r, unit, w) for x in ts for r in x["ic-pt-l7"]) / sum(S(r, unit, w) for x in ts for r in x["rho-strong"]))
def ols(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
def ci(v): v = sorted(v); return [v[int(0.025 * len(v))], v[int(0.975 * len(v))]]
def main():
    cells = load(); ts = list(cells.values())
    st = [r["outcome"]["status"] for x in ts for a in ("ic-pt-l7", "rho-strong") for r in x[a]]
    out = {"targets": len(ts), "solves": {a: f"{sum(r['outcome']['status'] == 'verified' for x in ts for r in x[a])}/{sum(len(x[a]) for x in ts)}" for a in ("ic-pt-l7", "rho-strong")}, "units": {}}
    if any(r["outcome"]["status"] != "verified" for x in ts for r in x["ic-pt-l7"]):
        out["verdict"] = "DOES_NOT_SOLVE"
    else:
        prior = json.loads((HERE.parent / "fb-ridge-vs-pairtable-k1" / "results.json").read_text())["units"]
        for unit, ws in (("band", W), ("bitop", (0.0,))):
            for w in ws:
                key = unit if unit == "bitop" else f"band@{w:g}"
                g17, g19 = prior[key]["17"]["gap_ic-pt-l7"], prior[key]["19"]["gap_ic-pt-l7"]
                rng = random.Random(0); boots = []
                for _ in range(B):
                    g = gap([ts[rng.randrange(len(ts))] for _ in ts], unit, w)
                    boots.append((g, ols([17, 19, 23], [g17, g19, g]), (g - g19) / 4))
                g = gap(ts, unit, w)
                out["units"][key] = {"gap_n17": g17, "gap_n19": g19, "gap_n23": g, "gap_n23_ci": ci([b[0] for b in boots]),
                                     "slope_ols_17_19_23": ols([17, 19, 23], [g17, g19, g]), "slope_ols_ci_n23_only": ci([b[1] for b in boots]),
                                     "slope_19_23": (g - g19) / 4, "slope_19_23_ci_n23_only": ci([b[2] for b in boots])}
        out["verdict"] = "MEASURED"
    text = json.dumps(out, indent=1)
    if "--out" in sys.argv: pathlib.Path(sys.argv[sys.argv.index("--out") + 1]).write_text(text + "\n")
    print(text)
if __name__ == "__main__": main()
