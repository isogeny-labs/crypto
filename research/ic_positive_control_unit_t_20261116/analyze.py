#!/usr/bin/env python3
"""Gates, tables, figure and PDF for the Cardano-Lucas positive-control port.

Reads run.json (this study) and the 11-12 control's control.json. Writes
analysis.json, tables.md, figure.svg, figure.png and REPORT.pdf next to this
file. Computes nothing the protocol does not name (PROTOCOL.md sections 4-5).
"""
import json
import os
import random
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
RUN = json.load(open(os.path.join(HERE, "run.json")))
OLD = json.load(open(os.path.join(HERE, "..", "ic_positive_control_la_20261116", "run.json")))
ARMS = ["C_coord_square", "C_unit_t", "C_unit_t_fast", "R_rho_negation"]
SHORT = {"C_coord_square": "S", "C_unit_t": "U", "C_unit_t_fast": "V", "R_rho_negation": "R"}
PRICES = [("total_w0", "0"), ("total_w01", "0.1"), ("total_w1", "1")]
B = 2000


def key(r):
    return (r["p"], r["curve_idx"], r["seed"])


rows = RUN["rows"]
by = {}
for r in rows:
    by.setdefault(r["arm"], {})[key(r)] = r
old_by = {}
for r in OLD["rows"]:
    old_by.setdefault(r["arm"], {})[key(r)] = r
ps = sorted({r["p"] for r in rows})

# ── gates ────────────────────────────────────────────────────────────────
gates = {"G1": [], "G2": []}
for arm, fields in [
    ("C_coord_square", ["group_ops", "rel_mul", "la_mul", "trials", "relations", "certified"]),
    ("R_rho_negation", ["group_ops", "native_mul", "certified"]),
]:
    for k, r in by[arm].items():
        o = old_by[arm].get(k)
        if o is None:
            continue  # new prime 131101: nothing to reproduce
        for f in fields:
            if r[f] != o[f]:
                gates["G1"].append(f"{arm} {k} {f}: {r[f]} != {o[f]}")
for r in rows:
    if not r["certified"]:
        gates["G2"].append(f"{r['arm']} {key(r)} not certified")
gate_ok = {g: not v for g, v in gates.items()}


# ── per-p statistics ─────────────────────────────────────────────────────
def mean(v):
    return sum(v) / len(v)


def cell(arm, p):
    return [r for k, r in by[arm].items() if k[0] == p]


def boot_ratio(num_arm, den_arm, p, f_num, f_den=None, seed=20261116):
    """Two-stage bootstrap (curves, then seeds within curve) of mean ratio."""
    f_den = f_den or f_num
    rnd = random.Random(seed)
    curves = sorted({k[1] for k in by[num_arm] if k[0] == p})
    def groups(arm, f):
        g = {}
        for k, r in by[arm].items():
            if k[0] == p:
                g.setdefault(k[1], []).append(r[f])
        return g
    gn, gd = groups(num_arm, f_num), groups(den_arm, f_den)
    est = mean(sum(gn.values(), [])) / mean(sum(gd.values(), []))
    bs = []
    for _ in range(B):
        cs = [rnd.choice(curves) for _ in curves]
        n, d = [], []
        for c in cs:
            n += [rnd.choice(gn[c]) for _ in gn[c]]
            d += [rnd.choice(gd[c]) for _ in gd[c]]
        bs.append(mean(n) / mean(d))
    bs.sort()
    return est, bs[int(0.025 * B)], bs[int(0.975 * B) - 1]


def boot_ratio_pt(num_arm, den_arm, p, seed=20261117):
    """Two-stage bootstrap of mean(rel_mul/trials) ratio."""
    rnd = random.Random(seed)
    def groups(arm):
        g = {}
        for k, r in by[arm].items():
            if k[0] == p:
                g.setdefault(k[1], []).append(r["rel_mul"] / r["trials"])
        return g
    gn, gd = groups(num_arm), groups(den_arm)
    curves = sorted(gn)
    est = mean(sum(gn.values(), [])) / mean(sum(gd.values(), []))
    bs = []
    for _ in range(B):
        cs = [rnd.choice(curves) for _ in curves]
        n, d = [], []
        for c in cs:
            n += [rnd.choice(gn[c]) for _ in gn[c]]
            d += [rnd.choice(gd[c]) for _ in gd[c]]
        bs.append(mean(n) / mean(d))
    bs.sort()
    return est, bs[int(0.025 * B)], bs[int(0.975 * B) - 1]


out = {"gates": gate_ok, "gate_failures": {g: v[:20] for g, v in gates.items()}, "per_p": []}
for p in ps:
    e = {"p": p, "n_runs": {SHORT[a]: len(cell(a, p)) for a in ARMS}}
    for a in ARMS[:3]:
        rs = cell(a, p)
        e[f"rel_mul_per_trial_{SHORT[a]}"] = mean([r["rel_mul"] / r["trials"] for r in rs])
        e[f"la_share_{SHORT[a]}"] = mean([r["la_mul"] / (r["la_mul"] + r["rel_mul"]) for r in rs])
        e[f"group_ops_{SHORT[a]}"] = mean([r["group_ops"] for r in rs])
    e["U_over_S_la_mul"] = boot_ratio("C_unit_t", "C_coord_square", p, "la_mul")
    e["U_over_S_native_all"] = boot_ratio("C_unit_t", "C_coord_square", p, "native_all")
    e["V_over_U_rel_per_trial"] = boot_ratio_pt("C_unit_t_fast", "C_unit_t", p)
    # deviation D1: V ran with the mixed walk, so V vs S is the like-for-like comparison
    e["V_over_S_rel_per_trial"] = boot_ratio_pt("C_unit_t_fast", "C_coord_square", p)
    e["fallbacks"] = {SHORT[a]: sum(r.get("la_fallbacks", 0) for r in cell(a, p)) for a in ["C_coord_square", "C_unit_t", "C_unit_t_fast"]}
    for a in ARMS[:3]:
        for f, w in PRICES:
            e[f"{SHORT[a]}_over_R_w{w}"] = boot_ratio(a, "R_rho_negation", p, f)
    out["per_p"].append(e)

# §5a verdict per arm: below R at all three prices (upper bound < 1) at some p?
verdict = {}
for a in ARMS[:3]:
    s = SHORT[a]
    strict = [e["p"] for e in out["per_p"] if all(e[f"{s}_over_R_w{w}"][2] < 1 for _, w in PRICES)]
    above = [e["p"] for e in out["per_p"] if all(e[f"{s}_over_R_w{w}"][1] > 1 for _, w in PRICES[1:])]
    verdict[s] = {"below_at_all_prices": strict, "above_at_w0.1_and_w1": above}
out["verdict"] = verdict
out["component_rule_unit_t"] = all(e["U_over_S_la_mul"][0] <= 0.85 for e in out["per_p"] if e["p"] >= 4099)
out["component_rule_fallback"] = all(
    e["V_over_U_rel_per_trial"][0] <= 0.9 for e in out["per_p"] if e["p"] in (2053, 8209, 65537)
)
import math as _m
pts = [(_m.log(e["p"]), _m.log(e["V_over_R_w0.1"][0])) for e in out["per_p"] if e["p"] >= 4099]
mx = sum(a for a, _ in pts) / len(pts); my = sum(b for _, b in pts) / len(pts)
out["P5_slope_lnVR_w01"] = sum((a - mx) * (b - my) for a, b in pts) / sum((a - mx) ** 2 for a, _ in pts)
out["V_w01_interval_below_1"] = [e["p"] for e in out["per_p"] if e["V_over_R_w0.1"][2] < 1]
out["claims"] = "[pcc:unverified]"
json.dump(out, open(os.path.join(HERE, "analysis.json"), "w"), indent=1)

# ── tables ───────────────────────────────────────────────────────────────
f3 = lambda t: f"{t[0]:.3f} [{t[1]:.3f}, {t[2]:.3f}]"
f2 = lambda t: f"{t[0]:.2f} [{t[1]:.2f}, {t[2]:.2f}]"
L = []
L.append("# Tables (generated by analyze.py; every number [pcc:unverified])\n")
L.append(f"Gates: {', '.join(f'{g} {"pass" if ok else "FAIL"}' for g, ok in gate_ok.items())}\n")
L.append("## Components, paired on (p, curve, seed)\n")
L.append("| p | runs/arm | U/S LA mul [95%] | U/S incl. additions [95%] | V/U relation mul per trial [95%] | V/S relation mul per trial [95%] (D1) | fallbacks S, U, V |")
L.append("|--:|--:|--|--|--|--|--|")
for e in out["per_p"]:
    L.append(f"| {e['p']} | {e['n_runs']['S']} | {f3(e['U_over_S_la_mul'])} | {f3(e['U_over_S_native_all'])} | "
             f"{f3(e['V_over_U_rel_per_trial'])} | {f3(e['V_over_S_rel_per_trial'])} | {e['fallbacks']['S']}, {e['fallbacks']['U']}, {e['fallbacks']['V']} |")
L.append("\n## Distance to the matched negation rho (arm total / R total), §5a band\n")
L.append("| p | S w=0.1 | S w=1 | U w=0.1 | U w=1 | V w=0 | V w=0.1 | V w=1 |")
L.append("|--:|--|--|--|--|--|--|--|")
for e in out["per_p"]:
    L.append(f"| {e['p']} | {f2(e['S_over_R_w0.1'])} | {f2(e['S_over_R_w1'])} | {f2(e['U_over_R_w0.1'])} | {f2(e['U_over_R_w1'])} | "
             f"{f2(e['V_over_R_w0'])} | {f2(e['V_over_R_w0.1'])} | {f2(e['V_over_R_w1'])} |")
L.append("\n## Linear-algebra share of native multiplications\n")
L.append("| p | S | U | V |")
L.append("|--:|--:|--:|--:|")
for e in out["per_p"]:
    L.append(f"| {e['p']} | {e['la_share_S']:.2f} | {e['la_share_U']:.2f} | {e['la_share_V']:.2f} |")
L.append(f"\n§5a verdict: {json.dumps(verdict)}\n")
L.append(f"Unit-T rule (U/S LA ≤ 0.85, p ≥ 4099): {out['component_rule_unit_t']}; fallback rule (V/U rel/trial ≤ 0.9 at 2053, 8209, 65537): {out['component_rule_fallback']}; P5 slope ln(V/R, w=0.1) on ln p (p ≥ 4099): {out['P5_slope_lnVR_w01']:.3f}; p with V w=0.1 interval below 1: {out['V_w01_interval_below_1']}\n")
open(os.path.join(HERE, "tables.md"), "w").write("\n".join(L) + "\n")

# ── figure and PDF ───────────────────────────────────────────────────────
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

col = {"S": "#6b6b6b", "U": "#1f77b4", "V": "#d62728"}
fig, ax = plt.subplots(1, 3, figsize=(15, 4.6))
x = [e["p"] for e in out["per_p"]]
for s in "SUV":
    ax[0].plot(x, [e[f"rel_mul_per_trial_{s}"] for e in out["per_p"]], "o-", color=col[s], label=s)
ax[0].set(xscale="log", xlabel="p", ylabel="relation-phase F_p mults per trial", title="(a) cost per relation trial")
ax[0].legend()
for k, (f, w) in enumerate(PRICES[1:], start=1):
    for s in "SUV":
        v = [e[f"{s}_over_R_w{w}"] for e in out["per_p"]]
        ax[k].errorbar(x, [t[0] for t in v], yerr=[[t[0] - t[1] for t in v], [t[2] - t[0] for t in v]],
                       fmt="o-", color=col[s], label=s, capsize=2)
    ax[k].axhline(1, color="k", lw=0.8, ls="--")
    ax[k].set(xscale="log", yscale="log", xlabel="p", ylabel="arm / matched rho",
              title=f"({'bc'[k-1]}) distance to rho at w = {w} GAE/mult")
    ax[k].legend()
fig.suptitle("Genus-3 positive control: unit-T walk (U) and fast fallback roots (V) vs S, matched negation rho R  [pcc:unverified]")
fig.tight_layout()
fig.savefig(os.path.join(HERE, "figure.svg"))
fig.savefig(os.path.join(HERE, "figure.png"), dpi=150)

with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(fig)
    tf = plt.figure(figsize=(11, 8.5))
    txt = open(os.path.join(HERE, "RESULTS.md")).read() if os.path.exists(os.path.join(HERE, "RESULTS.md")) else ""
    body = (txt or "\n".join(L))[:6000]
    tf.text(0.03, 0.97, body, va="top", family="monospace", fontsize=6.2, wrap=True)
    pdf.savefig(tf)
    tf2 = plt.figure(figsize=(11, 8.5))
    tf2.text(0.03, 0.97, "\n".join(L)[:7000], va="top", family="monospace", fontsize=5.6)
    pdf.savefig(tf2)
print("\n".join(L))
