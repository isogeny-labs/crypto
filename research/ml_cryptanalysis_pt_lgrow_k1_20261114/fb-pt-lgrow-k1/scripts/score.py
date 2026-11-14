"""Frozen analysis for PROTOCOL.md (pair-table l*(n) vs fixed l = 7 vs strong rho, K1 n = 17, 19, 23, 29).
S per record: the validated section 5a band function (copied verbatim from ../fb-pt-l7-k1-n23/scripts/score.py,
VAL-20261114-1227fa); bitop/v1 likewise.
L_STAR = {17: 7, 19: 7, 23: 9, 29: 10}, frozen from the sizing; at n = 17 and 19 the l* arm is the l = 7 arm itself.
Solvability rule: any non-verified measured record of ic-pt-l7 or of the l* arm at any n makes the result DOES_NOT_SOLVE.
gap(arm, n) = log2(sum S_arm / sum S_rho) over the 8 targets of n.
Primary: D = slope_l7 - slope_lstar, where slope = OLS of gap over n in {17, 19, 23, 29}. Positive means growing l bends
the gap's growth. Verdict POSITIVE if the 95% bootstrap CI of D lies above 0 at w = 0, 0.1 and 1; NEGATIVE if below 0 at
all three; otherwise NOT_SEPARATED.
Secondary: R_n = log2(S_l7 / S_lstar) at n = 23 and 29, each with the same three-weight verdict rule; every gap; bitop.
Bootstrap: 10,000 resamples; in each, the 8 targets of every n are resampled independently, n in ascending order, one
random.Random(0) stream per unit."""
import json, math, random, sys, pathlib
HERE = pathlib.Path(__file__).resolve().parents[1]
W = (0.0, 0.1, 1.0); B = 10_000; NS = (17, 19, 23, 29); L_STAR = {17: 7, 19: 7, 23: 9, 29: 10}
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
    cells = {}
    for rec in sorted((HERE / "sessions").glob("*/session/records.jsonl")):
        for line in open(rec):
            r = json.loads(line)
            if not r["warmup"]: cells.setdefault(r["workload"]["curve_spec"]["n"], {}).setdefault(r["workload"]["workload_id"], {}).setdefault(r["arm"], []).append(r)
    return cells
def ols(xs, ys):
    mx, my = sum(xs) / len(xs), sum(ys) / len(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)
def stat(sel, unit, w):
    g = {}
    for n in NS:
        ts = sel[n]; tot = lambda a: sum(S(r, unit, w) for x in ts for r in x[a])
        rho = tot("rho-strong"); g[("l7", n)] = math.log2(tot("ic-pt-l7") / rho); g[("ls", n)] = math.log2(tot(f"ic-pt-l{L_STAR[n]}") / rho)
    s7 = ols(NS, [g[("l7", n)] for n in NS]); ss = ols(NS, [g[("ls", n)] for n in NS])
    out = {"slope_l7": s7, "slope_lstar": ss, "D": s7 - ss}
    for n in NS: out[f"gap_l7_n{n}"] = g[("l7", n)]; out[f"gap_lstar_n{n}"] = g[("ls", n)]
    for n in (23, 29): out[f"R_n{n}"] = g[("l7", n)] - g[("ls", n)]
    return out
def ci(v): v = sorted(v); return [v[int(0.025 * len(v))], v[int(0.975 * len(v))]]
def main():
    cells = load(); out = {"targets": {str(n): len(cells.get(n, {})) for n in NS}, "l_star": L_STAR, "solves": {}, "units": {}}
    bad = False
    for n in NS:
        for a in ("rho-strong", "ic-pt-l7", f"ic-pt-l{L_STAR[n]}"):
            st = [r["outcome"]["status"] for x in cells[n].values() for r in x.get(a, [])]
            out["solves"][f"{n}/{a}"] = f"{st.count('verified')}/{len(st)}"
            if a != "rho-strong" and (not st or any(s != "verified" for s in st)): bad = True
    if bad:
        out["verdict"] = "DOES_NOT_SOLVE"
    else:
        for unit, ws in (("band", W), ("bitop", (0.0,))):
            for w in ws:
                key = unit if unit == "bitop" else f"band@{w:g}"; rng = random.Random(0); boots = []
                full = {n: list(cells[n].values()) for n in NS}
                for _ in range(B):
                    boots.append(stat({n: [full[n][rng.randrange(len(full[n]))] for _ in full[n]] for n in NS}, unit, w))
                st = stat(full, unit, w)
                out["units"][key] = {**st, **{k + "_ci": ci([b[k] for b in boots]) for k in st}}
        cis = [out["units"][f"band@{w:g}"]["D_ci"] for w in W]
        out["verdict"] = "POSITIVE" if all(c[0] > 0 for c in cis) else "NEGATIVE" if all(c[1] < 0 for c in cis) else "NOT_SEPARATED"
        for n in (23, 29):
            c = [out["units"][f"band@{w:g}"][f"R_n{n}_ci"] for w in W]
            out[f"verdict_R_n{n}"] = "POSITIVE" if all(x[0] > 0 for x in c) else "NEGATIVE" if all(x[1] < 0 for x in c) else "NOT_SEPARATED"
    text = json.dumps(out, indent=1)
    if "--out" in sys.argv: pathlib.Path(sys.argv[sys.argv.index("--out") + 1]).write_text(text + "\n")
    print(text)
if __name__ == "__main__": main()
