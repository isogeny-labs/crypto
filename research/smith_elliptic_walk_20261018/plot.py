#!/usr/bin/env python3
"""Plots for the W3a Smith-walk study. Plots recorded values only.

Reads data/exhaustive/exh_p*.json, data/exhaustive/graph_p11.json,
data/walks/fit.json and data/walks/walk_*.jsonl; writes figures/*.svg
(and .png at 150 dpi) plus figures/volcano_p11.diagram.json.
Run: python3 plot.py   (from this directory)
"""
import glob
import json
import os
import sys
from collections import defaultdict, deque

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
FIG = os.path.join(HERE, "figures")
os.makedirs(FIG, exist_ok=True)
REPO = os.path.abspath(os.path.join(HERE, "..", ".."))
plt.rcParams.update({"font.size": 9, "svg.fonttype": "path"})

C = {"DH": "#1f77b4", "DE": "#2ca02c", "C2": "#d62728", "C23": "#9467bd", "Ct": "#7f7f7f",
     "A": "#d62728", "B": "#1f77b4"}


def save(fig, name):
    fig.savefig(os.path.join(FIG, name + ".svg"), bbox_inches="tight")
    fig.savefig(os.path.join(FIG, name + ".png"), dpi=150, bbox_inches="tight")
    plt.close(fig)


def load_exh():
    out = {}
    for f in glob.glob(os.path.join(HERE, "data", "exhaustive", "exh_p*.json")):
        d = json.load(open(f))
        out[d["p"]] = d["summary"]
    return dict(sorted(out.items()))


def fig_shares(S):
    ps = list(S)
    fig, ax = plt.subplots(figsize=(6.4, 3.6))
    series = [("DH", "share_h_full", "$D_H$: descended $H$ passing (full_success)"),
              ("DE", "share_e_pass", "$D_E$: weak $E$ passing"),
              ("C2", None, "$C_2$: weak $E$ with a passing $E'$ in its 2-component"),
              ("C23", None, "$C_{23}$: same, 2/3-component"),
              ("Ct", "share_class_t_pass", "$C_t$: same, $K$-isogeny class (same $t$)")]
    for key, field, lab in series:
        if key == "C2":
            y = [S[p]["comp2"]["share_weak_with_pass_in_comp"] for p in ps]
        elif key == "C23":
            y = [S[p]["comp23"]["share_weak_with_pass_in_comp"] for p in ps]
        else:
            y = [S[p][field] for p in ps]
        ax.plot(ps, y, "o-", color=C[key], label=lab, ms=4, lw=1.2)
    ax.axhline(0.1857, color="k", ls=":", lw=0.8)
    ax.text(31.3, 0.1857, "Smith 0.1857 (random $H$)", va="center", fontsize=7)
    ax.axhline(0.5016, color="k", ls="--", lw=0.8)
    ax.text(31.3, 0.525, "ceiling 0.5016 (random $H$)", va="center", fontsize=7)
    ax.axhspan(0.4304, 0.4750, color="orange", alpha=0.15)
    ax.text(31.3, 0.42, "record's class share\nC = 0.43-0.475 (p=7,11)", va="center", fontsize=7)
    ax.set_xlabel("p  (K = F_{p^3}; exhaustive over all 2p^2 reduced weak models)")
    ax.set_ylabel("share")
    ax.set_ylim(0, 1.05)
    ax.set_xticks(ps)
    ax.legend(fontsize=7, loc="lower left")
    ax.set_title("W3a Q1: Smith pass rates on descended H, and closure under elliptic isogenies",
                 fontsize=9)
    save(fig, "q1_shares")


def fig_dist(S):
    ps = list(S)
    bins = ["1", "2", "3", "4", "5+", "none in comp."]
    cols = ["#08519c", "#3182bd", "#6baed6", "#9ecae1", "#c6dbef", "#bdbdbd"]
    fig, axes = plt.subplots(1, 2, figsize=(7.2, 3.4), sharey=True)
    for ax, key, title in [(axes[0], "dist2", "2-isogeny graph"),
                           (axes[1], "dist23", "2- and 3-isogeny graph")]:
        for i, p in enumerate(ps):
            d = S[p][key]
            h = d["hist"] + [0] * 12
            vals = [h[1], h[2], h[3], h[4], sum(h[5:]), d["unreachable"]]
            tot = sum(vals)
            left = 0
            for j, v in enumerate(vals):
                w = v / tot if tot else 0
                ax.barh(i, w, left=left, color=cols[j], edgecolor="white", lw=0.4,
                        label=bins[j] if i == 0 else None)
                left += w
            ax.text(1.01, i, f"n={tot}, mean={d['mean_reachable']:.2f}, max={d['max']}",
                    va="center", fontsize=6.5)
        ax.set_yticks(range(len(ps)))
        ax.set_yticklabels([f"p={p}" for p in ps])
        ax.set_xlim(0, 1)
        ax.set_xlabel("share of failing weak E")
        ax.set_title(title, fontsize=9)
    axes[0].legend(title="steps to nearest passing E", fontsize=6.5, title_fontsize=7,
                   loc="upper center", bbox_to_anchor=(1.1, -0.18), ncol=6)
    fig.suptitle("W3a Q1: isogeny distance from each failing weak E to the nearest passing E "
                 "(mean/max over reachable)", fontsize=9)
    fig.subplots_adjust(wspace=0.75)
    save(fig, "q1_distance")


def volcano(p=11):
    g = json.load(open(os.path.join(HERE, "data", "exhaustive", f"graph_p{p}.json")))
    nodes = {n["id"]: n for n in g["nodes"]}
    adj = defaultdict(set)
    for a, b in g["edges2"]:
        adj[a].add(b)
        adj[b].add(a)
    comps = defaultdict(list)
    for n in g["nodes"]:
        comps[n["comp2"]].append(n["id"])

    def mixed(c):
        w = [nodes[i] for i in comps[c] if nodes[i]["weak"]]
        return any(x["pass"] for x in w) and any(not x["pass"] for x in w)

    best = max((c for c in comps if mixed(c)), key=lambda c: len(comps[c]))
    ids = comps[best]
    # Layout only: level = depth - (graph distance to the nearest degree-1 vertex).
    floor = [i for i in ids if len(adj[i]) <= 1]
    dist = {i: None for i in ids}
    q = deque()
    for i in floor:
        dist[i] = 0
        q.append(i)
    while q:
        u = q.popleft()
        for v in adj[u]:
            if dist[v] is None:
                dist[v] = dist[u] + 1
                q.append(v)
    depth = max(d for d in dist.values() if d is not None) if floor else 0
    lab = {i: f"v{k}" for k, i in enumerate(sorted(ids))}
    verts = []
    for i in sorted(ids):
        n = nodes[i]
        vals = {}
        if n["weak"]:
            vals["pass"] = 1.0 if n["pass"] else 0.0
            vals["jv"] = 1.0 if n["jv"] else 0.0
        verts.append({"id": i, "label": lab[i],
                      "level": (depth - dist[i]) if dist[i] is not None else 0,
                      "nbrs": sorted(adj[i]), "values": vals})
    weak = sum(1 for i in ids if nodes[i]["weak"])
    npass = sum(1 for i in ids if nodes[i]["weak"] and nodes[i]["pass"])
    spec = {"schema": "volcano-diagram/v2", "ell": 2, "depth": depth,
            "title": f"2-isogeny component over F_{{{p}^3}} (largest mixed) · pass(E) on weak curves",
            "subtitle": "W3a Q1 exhaustive run-20261026T0212Z",
            "footnote": (f"{len(ids)} classes, {weak} weak (coloured), {npass} passing; grey = not weak "
                         f"(no (3,5) descent). Levels from graph distance to degree-1 vertices. "
                         f"t = {nodes[ids[0]]['t']}. Labels -> class keys in volcano_p{p}_labels.json"),
            "metrics": [{"key": "pass", "title": "pass(E): some descended H has full_success",
                         "label": "1 = pass, 0 = fail"},
                        {"key": "jv", "title": "JV-weak (cross-ratio norm 1)",
                         "label": "1 = JV-weak"}],
            "vertices": verts}
    json.dump(spec, open(os.path.join(FIG, f"volcano_p{p}.diagram.json"), "w"), indent=1)
    json.dump({lab[i]: {"key": nodes[i]["key"], "a": nodes[i]["a"], "b": nodes[i]["b"],
                        "t": nodes[i]["t"], "weak": nodes[i]["weak"], "pass": nodes[i]["pass"]}
               for i in ids}, open(os.path.join(FIG, f"volcano_p{p}_labels.json"), "w"), indent=1)
    sys.path.insert(0, os.path.join(REPO, "tools", "volcano-diagram"))
    from volcano_diagram import render
    render(spec, os.path.join(FIG, f"volcano_p{p}"))


def load_walks():
    W = defaultdict(list)
    for f in glob.glob(os.path.join(HERE, "data", "walks", "walk_*.jsonl")):
        for line in open(f):
            v = json.loads(line)
            W[(v["arm"], v["p"])].append(v)
    return W


def fig_walks():
    fp = os.path.join(HERE, "data", "walks", "fit.json")
    if not os.path.exists(fp):
        return
    F = json.load(open(fp))
    rows = F["rows"]
    W = load_walks()
    fig, axes = plt.subplots(1, 3, figsize=(11.5, 3.8))
    ax = axes[0]
    for arm, dx in [("A", 0.97), ("B", 1.03)]:
        for (a, p), ws in W.items():
            if a != arm:
                continue
            ok = [w["res"]["nodes_tested"] for w in ws if w["res"]["passed"]]
            cen = [w["res"]["nodes_tested"] for w in ws if not w["res"]["passed"]]
            mk = "o" if p % 4 == 3 else "D"
            ax.scatter([p * dx] * len(ok), ok, s=7, marker=mk, color=C[arm], alpha=0.45, lw=0)
            ax.scatter([p * dx] * len(cen), cen, s=12, marker="x", color=C[arm], alpha=0.6, lw=0.8)
        R = sorted([r for r in rows if r["arm"] == arm and r["successes"] > 0], key=lambda r: r["p"])
        ax.plot([r["p"] for r in R], [r["mean_nodes"] for r in R], "-", color=C[arm], lw=1.2,
                label=f"arm {arm}: frozen estimator (all nodes / successes)")
    xs = sorted({r["p"] for r in rows})
    ax.plot(xs, [0.4 * x for x in xs], "k:", lw=0.9, label="0.4 p (slope-1 guide)")
    ax.set_xscale("log")
    ax.set_yscale("log")
    ax.set_title("tested nodes per walk (dots: success, o p=3 mod 4, diamond p=1 mod 4;\n"
                 "x: censored, at the count where it stopped)", fontsize=8)
    ax.set_ylabel("tested nodes")
    ax = axes[1]
    for arm in "AB":
        R = sorted([r for r in rows if r["arm"] == arm and r["successes"] > 0], key=lambda r: r["p"])
        ax.loglog([r["p"] for r in R], [r["mean_cost"] for r in R], "o-", color=C[arm], ms=4,
                  label=f"arm {arm}: frozen (censored walks included)")
        ax.loglog([r["p"] for r in R], [r["mean_cost_excl_trapped"] for r in R], "x--",
                  color=C[arm], ms=4, alpha=0.7, label=f"arm {arm}: trapped walks left out")
    ax.set_title("cost per success, F_p multiplications\n(Smith calls derived from wall time)",
                 fontsize=8)
    ax = axes[2]
    causes = [("censored_isolated", "isolated start", "#636363"),
              ("censored_trapped", "trap rule", "#e6550d"),
              ("censored_steps", "step cap", "#fdae6b"),
              ("censored_nodes", "node cap", "#756bb1"),
              ("censored_wall", "wall cap", "#bcbddc")]
    labels = []
    for k, arm in enumerate("AB"):
        R = sorted([r for r in rows if r["arm"] == arm], key=lambda r: r["p"])
        for i, r in enumerate(R):
            x = i + (k - 0.5) * 0.38
            bottom = 0
            for key, lab, col in causes:
                v = r[key] / r["walks"]
                ax.bar(x, v, 0.36, bottom=bottom, color=col, edgecolor="k", lw=0.4,
                       hatch="///" if arm == "B" else None,
                       label=lab if (k == 0 and i == 0) else None)
                bottom += v
        labels = [f"{r['p']} ({'1' if r['p'] % 4 == 1 else '3'} mod 4)" for r in R]
    ax.set_xticks(range(len(labels)))
    ax.set_xticklabels(labels, fontsize=6, rotation=45, ha="right")
    ax.set_ylim(0, 1)
    ax.set_ylabel("share of walks censored")
    ax.set_title("censoring by cause (plain: arm A, hatched: arm B)", fontsize=8)
    for a in axes[:2]:
        a.set_xlabel("p (first prime above 2^k, k = 10..16, 18, 20)")
    axes[0].legend(fontsize=6, loc="lower right")
    axes[1].legend(fontsize=6, loc="lower left")
    axes[2].legend(fontsize=6, loc="upper left", bbox_to_anchor=(1.01, 1.0))
    fits = {(f["arm"], f["variant"], f["metric"]): f for f in F["fits"]}
    a1 = fits[("A", "frozen", "cost")]
    a2 = fits[("A", "post_hoc_p3mod4_successes_only", "nodes")]
    fig.suptitle(
        f"W3a Q2 sampled walks, run-20261026T0244Z. Arm A cost exponent (frozen fit) "
        f"{a1['exponent']:.2f} [{a1['ci95'][0]:.2f}, {a1['ci95'][1]:.2f}] -> '{a1['verdict']}'; "
        f"post hoc, successful walks at p = 3 mod 4: nodes exponent {a2['exponent']:.2f} "
        f"[{a2['ci95'][0]:.2f}, {a2['ci95'][1]:.2f}]", fontsize=8.5)
    fig.subplots_adjust(wspace=0.3, top=0.80)
    save(fig, "q2_walks")


if __name__ == "__main__":
    S = load_exh()
    fig_shares(S)
    fig_dist(S)
    volcano(11)
    fig_walks()
    print("ok")
