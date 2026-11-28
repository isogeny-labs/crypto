#!/usr/bin/env python3
"""Readings, gates and statistics of PROTOCOL.md (genus4_followup_20261118).

Written before any data of this study existed. Every number it prints is [pcc:unverified].
"""
import glob, gzip, json, math, os, random

HERE = os.path.dirname(os.path.abspath(__file__))
RES = os.path.join(HERE, "..")
NB, SEED = 2000, 20261123
WS = [("0", 0.0), ("0.1", 0.1), ("1", 1.0)]
P = 16411
W_LBL = "C_double_lp+pre+coord+unitT"
X_LBL = "C_double_lp+pre+coord+unitT+ferrari+sgecoord+sge(4,1.5)"
Q_LBL = "R_rho_plain_nowin"


def rows(pattern):
    out = []
    for f in sorted(glob.glob(pattern)):
        fh = gzip.open(f, "rt") if f.endswith(".gz") else open(f)
        out += [json.loads(l) for l in fh if l.strip()]
    return out


def key(r):
    return (r["p"], r["curve_seed"], r["target_seed"], r["seed"])


def by_arm(rs, label, p=P):
    d = {}
    for r in rs:
        if r.get("kind") == "run" and r.get("g") == 4 and r["arm"] == label and r["p"] == p:
            k = key(r)
            if k in d and {a: b for a, b in d[k].items() if a != "wall_ns"} != {a: b for a, b in r.items() if a != "wall_ns"}:
                raise SystemExit(f"duplicate rows differ {label} {k}")
            d[k] = r
    return d


def uniq(pattern):
    """Plain .jsonl and its .gz copy are the same rows; read one of each pair."""
    fs = sorted(glob.glob(pattern))
    keep = [f for f in fs if not (f.endswith(".jsonl") and f + ".gz" in fs)]
    out = []
    for f in keep:
        out += rows(f)
    return out


sel = json.load(open(os.path.join(HERE, "select.json")))
ccs = sel["cc_star"]
bwx = uniq(os.path.join(HERE, "b_wx.jsonl*"))
bxs = uniq(os.path.join(HERE, "b_xstar.jsonl*")) if ccs != "2.0" else bwx
bq = uniq(os.path.join(HERE, "b_q.jsonl*"))
g3 = uniq(os.path.join(HERE, "b_q_g3.jsonl*"))
lean = rows(os.path.join(RES, "genus4_lean_rho_20261116", "p16411*.jsonl"))
cheap = rows(os.path.join(RES, "genus4_cheap_ic_20261117", "p16411.jsonl.gz"))

ARMS = {"w": by_arm(bwx, W_LBL), "x": by_arm(bwx, X_LBL)}
if ccs != "2.0":
    ARMS["x*"] = by_arm(bxs, X_LBL)
Qold, Qnew = by_arm(lean, Q_LBL), by_arm(bq, Q_LBL)
Q = {**Qold, **Qnew}
inst = {r["curve_seed"]: r for r in bwx if r.get("kind") == "instance" and r["p"] == P}

gates = {"G1": [], "G2": [], "G3": []}
for a, d in ARMS.items():
    for k, r in d.items():
        if not (r.get("certified") and r.get("ok")):
            gates["G1"].append(f"{a} {k}")
for k, r in Q.items():
    if not r.get("certified"):
        gates["G1"].append(f"Q {k}")
F = ["group_ops", "rel_mul", "rel_add", "la_mul", "la_add", "trials", "relations"]
g2n = 0
for a, lbl in [("w", W_LBL), ("x", X_LBL)]:
    for k, o in by_arm(cheap, lbl).items():
        n = ARMS[a].get(k)
        if n is None:
            gates["G2"].append(f"{a} {k} missing")
            continue
        g2n += 1
        for f in F:
            if n.get(f) != o.get(f):
                gates["G2"].append(f"{a} {k} {f}")
g3n = 0
for k, n in by_arm(g3, Q_LBL).items():
    g3n += 1
    o = Qold.get(k)
    if o is None or any(n[f] != o[f] for f in ("steps", "group_ops")):
        gates["G3"].append(f"{k}")
if g3n == 0:
    gates["G3"].append("no re-run row")
gate_ok = {g: not v for g, v in gates.items()}


def native(r):
    return r["rel_mul"] + r["rel_add"] + r["la_mul"] + r["la_add"]


def ic_cost(r, w):
    return r["group_ops"] + w * native(r)


def q_cost(r, q, w):
    K, H, D = r["compares_uncharged"], r["hash_uncharged"], r["coef_adds_uncharged"]
    return r["group_ops"] + w * (K if q == "Q3" else K + H + D)


def mean(v):
    return sum(v) / len(v)


def resample(rnd, keys, curves=False):
    if curves:
        cs = sorted({k[1] for k in keys})
        out = []
        for c in [rnd.choice(cs) for _ in cs]:
            out += resample(rnd, [k for k in keys if k[1] == c])
        return out
    tg = sorted({k[:3] for k in keys})
    by = {t: [k for k in keys if k[:3] == t] for t in tg}
    ks = []
    for t in [rnd.choice(tg) for _ in tg]:
        ks += [rnd.choice(by[t]) for _ in by[t]]
    return ks


def interval(fn, keys, seed=SEED, curves=False):
    rnd = random.Random(seed)
    bs = sorted(fn(resample(rnd, keys, curves)) for _ in range(NB))
    return {"ratio": fn(keys), "lo": bs[int(0.025 * NB)], "hi": bs[int(0.975 * NB) - 1]}


def theo(k, neg):
    r = inst[k[1]]["r"]
    return math.sqrt(math.pi * r / (4 if neg else 2))


keys_all = sorted(k for k in ARMS["x"] if k in Q and all(k in d for d in ARMS.values()))
keys_new = [k for k in keys_all if (k[2] - k[1]) // 1000003 >= 3]
out = {"cc_star": ccs, "select": sel["grid"], "gates": gate_ok, "gate_failures": {g: v[:20] for g, v in gates.items()},
       "g2_rows_compared": g2n, "g3_rows_compared": g3n, "n_keys": len(keys_all), "n_targets": len({k[:3] for k in keys_all}),
       "q_mean_over_theory": mean([Q[k]["group_ops"] for k in keys_all]) / mean([theo(k, False) for k in keys_all]),
       "arms": {}, "claims": "[pcc:unverified]"}
for a, d in ARMS.items():
    ac = {}
    for q in ("Q3", "Q4"):
        for n, w in WS:
            ac[f"{q}_w{n}"] = interval(lambda kk, q=q, w=w, d=d: mean([ic_cost(d[k], w) for k in kk]) / mean([q_cost(Q[k], q, w) for k in kk]), keys_all)
    ac["Q3_w1_new_only"] = interval(lambda kk, d=d: mean([ic_cost(d[k], 1) for k in kk]) / mean([q_cost(Q[k], "Q3", 1) for k in kk]), keys_new)
    ac["Q3_w1_curve_boot"] = interval(lambda kk, d=d: mean([ic_cost(d[k], 1) for k in kk]) / mean([q_cost(Q[k], "Q3", 1) for k in kk]), keys_all, curves=True)
    for n, w in WS:
        ac[f"vs_plain_theory_w{n}"] = mean([ic_cost(d[k], w) for k in keys_all]) / mean([theo(k, False) for k in keys_all])
        ac[f"vs_neg_ideal_w{n}"] = mean([ic_cost(d[k], w) for k in keys_all]) / mean([theo(k, True) for k in keys_all])
    ac["share_la"] = mean([d[k]["la_mul"] + d[k]["la_add"] for k in keys_all]) / mean([ic_cost(d[k], 1) for k in keys_all])
    ac["share_rel"] = mean([d[k]["rel_mul"] + d[k]["rel_add"] for k in keys_all]) / mean([ic_cost(d[k], 1) for k in keys_all])
    ac["mean_cost_w1"] = mean([ic_cost(d[k], 1) for k in keys_all])
    ac["fb_cols"] = mean([d[k].get("fb_cols", 0) for k in keys_all])
    ac["below_rho_w1"] = ac["Q3_w1"]["hi"] < 1 and ac["Q4_w1"]["hi"] < 1
    out["arms"][a] = ac
if "x*" in ARMS:
    out["xstar_over_x"] = {n: interval(lambda kk, w=w: mean([ic_cost(ARMS["x*"][k], w) for k in kk]) / mean([ic_cost(ARMS["x"][k], w) for k in kk]), keys_all, SEED + 1) for n, w in WS}
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

f3 = lambda v: f"{v['ratio']:.3f} [{v['lo']:.3f}, {v['hi']:.3f}]"
L = ["# Tables (generated by analyze.py; every number [pcc:unverified])", "",
     "Gates: " + ", ".join(f"{g} {'pass' if ok else 'FAIL'}" for g, ok in gate_ok.items())
     + f" (G2 rows compared {g2n}, G3 rows {g3n})", "",
     f"Part A (p = 8209, arm x, 36 keys per cc): cc* = {ccs}", "", "| cc | eligible | mean cost (Q3, w = 1) | / cc 2.0 |", "|--|--|--|--|"]
base = sel["grid"]["2.0"]["mean_cost_q3_w1"]
for cc, v in sel["grid"].items():
    L.append(f"| {cc} | {v['eligible']} | {v['mean_cost_q3_w1']:.4g} | {v['mean_cost_q3_w1'] / base:.3f} |")
L += ["", f"## p = {P}: {out['n_targets']} targets, {out['n_keys']} keys; measured Q mean / sqrt(pi r/2) = {out['q_mean_over_theory']:.3f}", "",
      "| arm | Q3 w=0.1 | Q3 w=1 | Q4 w=1 | Q3 w=1 new targets only | Q3 w=1 curve bootstrap | vs sqrt(pi r/2), w=1 | vs ideal negation sqrt(pi r/4), w=1 | LA share |",
      "|--|--|--|--|--|--|--|--|--|"]
for a, v in out["arms"].items():
    L.append(f"| {a} | {f3(v['Q3_w0.1'])} | {f3(v['Q3_w1'])} | {f3(v['Q4_w1'])} | {f3(v['Q3_w1_new_only'])} | {f3(v['Q3_w1_curve_boot'])}"
             f" | {v['vs_plain_theory_w1']:.3f} | {v['vs_neg_ideal_w1']:.3f} | {v['share_la']:.2f} |")
L.append("")
for a, v in out["arms"].items():
    L.append(f"Below rho at w = 1 (Q3 and Q4 upper < 1), arm {a}: **{v['below_rho_w1']}**")
if "xstar_over_x" in out:
    L += ["", "x* / x cost: " + ", ".join(f"w = {n}: {f3(v)}" for n, v in out["xstar_over_x"].items())]
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["svg.fonttype"] = "none"
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

fig, axs = plt.subplots(1, 2, figsize=(11, 4.4))
ccs_g = [float(c) for c in sel["grid"]]
axs[0].plot(ccs_g, [v["mean_cost_q3_w1"] / base for v in sel["grid"].values()], "o-")
axs[0].axvline(float(ccs), color="r", ls=":", label=f"cc* = {ccs}")
axs[0].set(xlabel="cc (nb = cc p^(3/4))", ylabel="arm x mean cost / cc 2.0", title="Part A: p = 8209 tuning  [pcc:unverified]")
axs[0].legend()
names = list(out["arms"])
for i, a in enumerate(names):
    v = out["arms"][a]
    for j, (lab, key_) in enumerate([("Q3 measured Q", "Q3_w1"), ("Q4 measured Q", "Q4_w1")]):
        e = v[key_]
        axs[1].errorbar([i + 0.15 * (j - 0.5)], [e["ratio"]], yerr=[[e["ratio"] - e["lo"]], [e["hi"] - e["ratio"]]], fmt="o",
                        color=["#1f77b4", "#ff7f0e"][j], capsize=3, label=lab if i == 0 else None)
    axs[1].plot([i + 0.3], [v["vs_neg_ideal_w1"]], "kx", label="vs ideal negation bound" if i == 0 else None)
axs[1].axhline(1, color="k", lw=0.8, ls="--")
axs[1].set_xticks(range(len(names)), names)
axs[1].set(ylabel="IC / rho, w = 1", title=f"p = {P}, {out['n_targets']} targets  [pcc:unverified]")
axs[1].legend(fontsize=7)
fig.tight_layout()
fig.savefig(os.path.join(HERE, "figure.svg"))
fig.savefig(os.path.join(HERE, "figure.png"), dpi=90)
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(fig)
    for src in ["RESULTS.md", "tables.md", "PROTOCOL.md"]:
        pth = os.path.join(HERE, src)
        if os.path.exists(pth):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.97, open(pth).read()[:7000], va="top", family="monospace", fontsize=5.4)
            pdf.savefig(t)
print("\n".join(L))
