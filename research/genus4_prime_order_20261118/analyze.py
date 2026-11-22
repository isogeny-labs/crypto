#!/usr/bin/env python3
"""Readings, gates and statistics of PROTOCOL.md (genus4_prime_order_20261118).
Written before any IC data of this study existed. Every number it prints is [pcc:unverified]."""
import glob, gzip, json, math, os, random

HERE = os.path.dirname(os.path.abspath(__file__))
NB, SEED = 2000, 20261124
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]
PRIMES = [16411, 32771]


def rows(pattern):
    fs = sorted(glob.glob(pattern))
    keep = [f for f in fs if not (f.endswith(".jsonl") and f + ".gz" in fs)]
    out = []
    for f in keep:
        op = gzip.open if f.endswith(".gz") else open
        out += [json.loads(l) for l in op(f, "rt") if l.strip()]
    return out


def by_key(rs, p):
    d, inst = {}, {}
    for r in rs:
        if r.get("p") != p or r.get("g") != 4:
            continue
        if r.get("kind") == "instance":
            inst[r["curve_seed"]] = r
        elif r.get("kind") == "run" and "ferrari" in r["arm"]:
            k = (r["curve_seed"], r["target_seed"], r["seed"])
            if k in d and {a: b for a, b in d[k].items() if a != "wall_ns"} != {a: b for a, b in r.items() if a != "wall_ns"}:
                raise SystemExit(f"duplicate rows differ {p} {k}")
            d[k] = r
    return d, inst


curves = json.load(open(os.path.join(HERE, "curves.json")))
tune = json.load(open(os.path.join(HERE, "tune.json")))
xstar_is_x = tune["choice"]["cc"] == "2.0" and tune["choice"]["w_max"] == 4 and tune["choice"]["fill_max"] == 1.5

native = lambda r: r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]
cost = lambda r, w: r["group_ops"] + w * native(r)
mean = lambda v: sum(v) / len(v)

data, gates = {}, {"G1": [], "G2": [], "G3": []}
for p in PRIMES:
    X, inst = by_key(rows(os.path.join(HERE, f"b_p{p}_x.jsonl*")), p)
    XS = by_key(rows(os.path.join(HERE, f"b_p{p}_xstar.jsonl*")), p)[0] if not xstar_is_x else X
    chosen = curves[str(p)]["chosen"]
    keys = sorted(k for k in X if k[0] in chosen and k in XS)
    for d, nm in ((X, "x"), (XS, "x*")):
        for k, r in d.items():
            if not (r.get("certified") and r.get("ok")):
                gates["G1"].append(f"{p} {nm} {k}")
    for c in chosen:
        if c not in inst or inst[c]["h"] != 1:
            gates["G2"].append(f"{p} {c}")
    if len(keys) != 60:
        gates["G1"].append(f"{p}: {len(keys)} keys, expected 60")
    data[p] = {"x": X, "x*": XS, "inst": inst, "keys": keys}

Q = [r for r in rows(os.path.join(HERE, "b_p16411_q.jsonl*")) if r.get("kind") == "run"]
qinst = data[16411]["inst"]
q_ratio = mean([r["group_ops"] for r in Q]) / mean([math.sqrt(math.pi * qinst[r["curve_seed"]]["r"] / 2) for r in Q]) if Q else None
if q_ratio is None or not (0.6 <= q_ratio <= 1.6) or not all(r.get("certified") for r in Q):
    gates["G3"].append(f"Q mean / sqrt(pi r/2) = {q_ratio}, runs {len(Q)}")
gate_ok = {g: not v for g, v in gates.items()}


def ref(p, k, neg):
    return math.sqrt(math.pi * data[p]["inst"][k[0]]["r"] / (4 if neg else 2))


def resample(rnd, keys):
    cs = sorted({k[0] for k in keys})
    out = []
    for c in [rnd.choice(cs) for _ in cs]:
        kc = [k for k in keys if k[0] == c]
        tg = sorted({k[:2] for k in kc})
        for t in [rnd.choice(tg) for _ in tg]:
            kt = [k for k in kc if k[:2] == t]
            out += [rnd.choice(kt) for _ in kt]
    return out


def boot(fn, keys, seed=SEED):
    rnd = random.Random(seed)
    raw = [fn(resample(rnd, keys)) for _ in range(NB)]
    bs = sorted(raw)
    return {"ratio": fn(keys), "lo": bs[int(0.025 * NB)], "hi": bs[int(0.975 * NB) - 1], "samples": raw}


out = {"gates": gate_ok, "gate_failures": gates, "tune": tune["choice"], "xstar_is_x": xstar_is_x,
       "q_check": {"runs": len(Q), "mean_over_sqrt_pi_r_2": q_ratio}, "primes": {}, "claims": "[pcc:unverified]"}
samples = {}
arms = ["x"] + ([] if xstar_is_x else ["x*"])
for p in PRIMES:
    D, keys = data[p], data[p]["keys"]
    cell = {"keys": len(keys), "curves": curves[str(p)]["chosen"], "arms": {}}
    for a in arms:
        d = D[a]
        ac = {}
        for nm, neg in (("neg", True), ("plain", False)):
            for n, w in WS:
                b = boot(lambda kk, d=d, w=w, neg=neg: mean([cost(d[k], w) for k in kk]) / mean([ref(p, k, neg) for k in kk]), keys)
                samples[(p, a, nm, n)] = b.pop("samples")
                ac[f"{nm}_w{n}"] = b
        ac["below_neg_all_w"] = all(ac[f"neg_w{n}"]["hi"] < 1 for n, _ in WS)
        ac["mean_cost_w1"] = mean([cost(d[k], 1) for k in keys])
        ac["la_share_w1"] = mean([d[k]["la_mul"] + d[k]["la_add"] for k in keys]) / ac["mean_cost_w1"]
        ac["fb_cols"] = mean([d[k].get("fb_cols", 0) for k in keys])
        ac["per_curve_neg_w1"] = {str(c): mean([cost(d[k], 1) for k in keys if k[0] == c]) / mean([ref(p, k, True) for k in keys if k[0] == c]) for c in cell["curves"]}
        cell["arms"][a] = ac
    if not xstar_is_x:
        b = boot(lambda kk: mean([cost(D["x*"][k], 1) for k in kk]) / mean([cost(D["x"][k], 1) for k in kk]), keys, SEED + 1)
        b.pop("samples")
        cell["xstar_over_x_w1"] = b
    out["primes"][str(p)] = cell

lr = math.log(PRIMES[1] / PRIMES[0])
out["slope"] = {}
for a in arms:
    s = [math.log(b / a0) / lr for a0, b in zip(samples[(PRIMES[0], a, "neg", "1")], samples[(PRIMES[1], a, "neg", "1")])]
    s.sort()
    est = math.log(out["primes"][str(PRIMES[1])]["arms"][a]["neg_w1"]["ratio"] / out["primes"][str(PRIMES[0])]["arms"][a]["neg_w1"]["ratio"]) / lr
    cost_slope = math.log(out["primes"][str(PRIMES[1])]["arms"][a]["mean_cost_w1"] / out["primes"][str(PRIMES[0])]["arms"][a]["mean_cost_w1"]) / lr
    out["slope"][a] = {"ratio_vs_neg_w1": {"slope": est, "lo": s[int(0.025 * NB)], "hi": s[int(0.975 * NB) - 1],
                                           "note": "independent bootstrap per prime (different keys); draws paired by index, not sorted"},
                       "ic_cost_w1_in_p": cost_slope}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items()),
     f"Part A choice: cc = {tune['choice']['cc']}, SGE ({tune['choice']['w_max']}, {tune['choice']['fill_max']})"
     + (" = baseline (x* = x)" if xstar_is_x else ""),
     f"Measured plain rho check (p = 16411, {len(Q)} runs): mean / sqrt(pi r/2) = {q_ratio}", ""]
for p, c in out["primes"].items():
    L += [f"## p = {p}: curves {c['curves']}, {c['keys']} keys", "",
          "| arm | vs ideal negation w=0 | w=0.1 | w=1 | vs plain-rho expectation w=1 | below ideal negation at all w | LA share | fb cols |",
          "|--|--|--|--|--|--|--|--|"]
    for a, v in c["arms"].items():
        L.append(f"| {a} | {f3(v['neg_w0'])} | {f3(v['neg_w0.1'])} | {f3(v['neg_w1'])} | {f3(v['plain_w1'])} | "
                 f"**{v['below_neg_all_w']}** | {v['la_share_w1']:.2f} | {v['fb_cols']:.0f} |")
    for a, v in c["arms"].items():
        L.append("")
        L.append(f"Per curve, {a} vs ideal negation, w = 1: " + ", ".join(f"{k}: {x:.3f}" for k, x in v["per_curve_neg_w1"].items()))
    if "xstar_over_x_w1" in c:
        L += ["", f"x* / x at w = 1: {f3(c['xstar_over_x_w1'])}"]
    L.append("")
for a, s in out["slope"].items():
    v = s["ratio_vs_neg_w1"]
    L.append(f"Slope of ln(arm {a} / ideal negation, w = 1) in ln p: {v['slope']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]; "
             f"slope of ln(IC cost) in ln p: {s['ic_cost_w1_in_p']:.3f}")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt
fig, ax = plt.subplots(figsize=(7.5, 4.4))
for i, a in enumerate(arms):
    for j, (n, _) in enumerate(WS):
        xs = [p * (1 + 0.03 * (i * 3 + j - 2)) for p in PRIMES]
        vs = [out["primes"][str(p)]["arms"][a][f"neg_w{n}"] for p in PRIMES]
        ax.errorbar(xs, [v["ratio"] for v in vs], yerr=[[v["ratio"] - v["lo"] for v in vs], [v["hi"] - v["ratio"] for v in vs]],
                    fmt="o-", capsize=3, label=f"{a}, w = {n}")
ax.axhline(1, color="k", lw=0.8, ls="--")
ax.set(xscale="log", yscale="log", xlabel="p", ylabel="IC / ideal negation rho sqrt(pi r/4)",
       title="Genus 4, prime-order Jacobians  [pcc:unverified]")
ax.legend(fontsize=7)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "figure.svg"))
fig.savefig(os.path.join(HERE, "figure.png"), dpi=90)
print("\n".join(L))
