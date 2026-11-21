#!/usr/bin/env python3
"""Apply PRESET section 6 to finished runs. Usage: analyze_m8.py OUTDIR -> analysis.json + tables.md"""
import json
import math
import os
import sys

sys.path.insert(0, "/home/user/crypto/research/prime_field_ic_closure_20261112/finish/pkm-msolve/scripts")
import breakeven as BE  # noqa: E402


def expo(m, s, w):
    den = m + 1 - w * m * BE.H(s) if s < 0.5 else m + 1 - w * m
    return round(2 / den, 3) if den > 0 else float("inf")


def ols(xs, ys):
    n = len(xs)
    mx, my = sum(xs) / n, sum(ys) / n
    sxx = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sxx


def main():
    out = sys.argv[1]
    idx = [json.loads(l) for l in open(os.path.join(out, "index.jsonl"))]
    res = {}
    for meta in idx:
        f = os.path.join(out, "runs", meta["name"] + ".res.json")
        if os.path.exists(f):
            res[meta["name"]] = json.load(open(f))
    lines = []
    analysis = {"ladders": {}, "armA": []}
    for m in (6, 7, 8, 9, 10):
        a1, a2 = BE.Hinv(1 - 3 / m), BE.Hinv((1 - 3 / m) / 2)
        rungs = {}
        for meta in idx:
            if meta.get("ladder") != m:
                continue
            key = (meta["stage"], meta["k"])
            rungs.setdefault(key, dict(N=meta["N"], nvars=meta["nvars"], depths=meta["depths"], draws={}))
            r = res.get(meta["name"])
            rungs[key]["draws"][meta["idx"]] = None if r is None else dict(
                D=r["D"], stop=r["stop"], W=r["W"], wall=r["wall_s"], rss_kB=r["ru_maxrss_kB"], unit=r.get("unit"))
        rows = []
        for key in sorted(rungs, key=lambda k: rungs[k]["N"]):
            ru = rungs[key]
            d = ru["draws"]
            ran = [v for v in d.values() if v is not None]
            exact = len(ran) == 2 and all(v["stop"] is None for v in ran)
            status = "exact" if exact else ("not_run" if not ran else
                                            "censored" if any(v["stop"] for v in ran) else "single_draw")
            cell = min(v["D"] for v in ran) if exact else None
            lb = None
            if status == "censored":
                lb = min(v["D"] for v in ran if v["D"] is not None) if any(v["D"] for v in ran) else None
            rows.append(dict(stage=key[0], k=key[1], N=ru["N"], nvars=ru["nvars"], status=status, D_cell=cell,
                             D_lb=lb, draws=d))
        ex = [r for r in rows if r["status"] == "exact"]
        out_m = dict(alpha1=round(a1, 4), alpha2=round(a2, 4), rungs=rows)
        if len(ex) >= 3 and ex[-1]["N"] - ex[0]["N"] >= math.ceil(m / 2):
            s = ols([r["N"] for r in ex], [r["D_cell"] for r in ex])
            verdict = "KILL" if s >= a1 else "WEAK-KILL" if s >= a2 else "OPEN-LOW"
        else:
            s, verdict = None, "INCONCLUSIVE"
        out_m["sigma_hat"] = None if s is None else round(s, 4)
        out_m["verdict"] = verdict
        if ex:
            se = (ex[-1]["D_cell"] - ex[0]["D_cell"]) / (ex[-1]["N"] - ex[0]["N"]) if len(ex) > 1 else None
            out_m["sigma_end"] = None if se is None else round(se, 4)
            cens = [r for r in rows if r["status"] == "censored" and r["D_lb"] is not None]
            if cens:
                c = cens[0]
                out_m["sigma_cens_lb"] = round((c["D_lb"] - ex[0]["D_cell"]) / (c["N"] - ex[0]["N"]), 4)
            for nm in ("sigma_hat", "sigma_end", "sigma_cens_lb"):
                v = out_m.get(nm)
                if v is not None:
                    out_m[nm + "_exponent_w1_w2"] = (expo(m, v, 1), expo(m, v, 2))
        analysis["ladders"][m] = out_m
        lines.append(f"\n### m = {m} (alpha*1 = {a1:.4f}, alpha*2 = {a2:.4f}) -> {verdict}, sigma_hat = {out_m['sigma_hat']}, "
                     f"sigma_end = {out_m.get('sigma_end')}, censored-lb slope = {out_m.get('sigma_cens_lb')}\n")
        lines.append("| rung | depths | N | nvars | status | D r0 | D r1 | W r0 | wall r0/r1 s | peak RSS r0 MB |")
        lines.append("|---|---|---|---|---|---|---|---|---|---|")
        for r in rows:
            d0, d1 = r["draws"].get(0), r["draws"].get(1)
            fmt = lambda v: "-" if v is None else (f">={v['D']}" if v["stop"] else str(v["D"]))
            dep = "".join(map(str, rungs[(r["stage"], r["k"])]["depths"]))
            lines.append(f"| {r['stage']} k={r['k']} | {dep} | {r['N']} | {r['nvars']} | {r['status']} | {fmt(d0)} | {fmt(d1)} | "
                         f"{'-' if d0 is None else d0['W']} | {'-' if d0 is None else d0['wall']}/{'-' if d1 is None else d1['wall']} | "
                         f"{'-' if d0 is None else round(d0['rss_kB'] / 1024)} |")
    lines.append("\n### Arm A (p = 40961), uniform t = 1\n")
    lines.append("| system | D | W | unit | stop | wall s |")
    lines.append("|---|---|---|---|---|---|")
    for meta in idx:
        if meta["arm"] != "A":
            continue
        r = res.get(meta["name"])
        analysis["armA"].append(dict(name=meta["name"], res=r))
        if r:
            lines.append(f"| {meta['name']} | {('>=' if r['stop'] else '') + str(r['D'])} | {r['W']} | {r.get('unit')} | {r['stop']} | {r['wall_s']} |")
        else:
            lines.append(f"| {meta['name']} | not run | | | | |")
    v8 = [analysis["ladders"][m]["verdict"] for m in (8, 9, 10)]
    if all(v in ("KILL", "WEAK-KILL") for v in v8) and "KILL" in v8:
        item = "CLOSED-AT-TOY-SCALE"
    elif "OPEN-LOW" in v8:
        item = "OPEN"
    else:
        item = "INCONCLUSIVE"
    rep = {m: ("CONFIRMS" if analysis["ladders"][m]["verdict"] == "KILL" else
               "WEAKENS" if analysis["ladders"][m]["verdict"] in ("WEAK-KILL", "OPEN-LOW") else "INCONCLUSIVE")
           for m in (6, 7)}
    analysis["item_verdict_m_ge_8"] = item
    analysis["repeat_verdict"] = rep
    lines.insert(0, f"Item verdict (m = 8, 9, 10): **{item}**; repeats: m=6 {rep[6]}, m=7 {rep[7]}")
    json.dump(analysis, open(os.path.join(out, "analysis.json"), "w"), indent=1)
    open(os.path.join(out, "tables.md"), "w").write("\n".join(lines) + "\n")
    print("\n".join(lines))


if __name__ == "__main__":
    main()
