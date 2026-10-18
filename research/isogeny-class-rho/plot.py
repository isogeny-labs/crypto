"""Lay out recorded isoclass-rho runs: no computation beyond arranging the
values in walk.json / results.json / trials.csv (AGENTS.md "Implementation
language").

usage: python plot.py RUN_DIR            # RUN_DIR holds walk.json and class/, generators/, shift/ (any subset)
"""
import csv
import json
import math
import sys
from collections import defaultdict
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402
from matplotlib import colors  # noqa: E402

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "tools" / "volcano-diagram"))
from volcano_diagram import render  # noqa: E402

run = Path(sys.argv[1])
W = json.loads((run / "walk.json").read_text())
rid = run.name
verts = W["vertices"]
field = W["field"]
qname = f"p = {field['p']}" if field["kind"] == "prime" else f"F_2^{field['m']}"


def save(fig, name):
    fig.savefig(run / f"{name}.svg", bbox_inches="tight")
    fig.savefig(run / f"{name}.png", dpi=150, bbox_inches="tight")
    plt.close(fig)


def load(mode):
    d = run / mode
    if not (d / "results.json").exists():
        return None, None
    R = json.loads((d / "results.json").read_text())
    rows = list(csv.DictReader(open(d / "trials.csv")))
    return R, rows


R, rows = load("class")
arms = {a["vertex"]: a for a in (R["arms"] if R else []) if a["kind"] == "isogenous"}
expected = R["rho"]["expected_iters_sqrt_pi_n_over_2"] if R else None

# ---------------------------------------------------------------- 1. standard volcano diagram per volcano prime
volcano_ells = [e for e in W["ells"] if e["active"] and e["depth"] > 0]
for e in volcano_ells:
    ell = e["ell"]
    sub = [v for v in verts if str(ell) in v["levels"]]
    ids = {v["id"] for v in sub}
    spec = {
        "schema": "volcano-diagram/v2",
        "ell": ell,
        "depth": e["depth"],
        "title": f"Measured {ell}-isogeny volcano levels of the class walk · {qname} · "
        f"{R['params']['trials'] if R else 0} paired walk seeds per curve",
        "subtitle": f"{rid} · levels in the {ell}-volcano; edges of every walked ℓ drawn",
        "footnote": "vertices reached through other primes sit at their own ℓ-level; grey = walked, not measured",
        "metrics": [
            {"key": "ops", "title": "native rho · mean iterations", "label": "group operations per solve"},
            {"key": "time", "title": "native rho · mean end-to-end time", "label": "ms per solve (transport + solve)"},
        ],
        "vertices": [
            {
                "id": v["id"],
                "label": f"v{v['id']}",
                "level": v["levels"][str(ell)],
                "nbrs": [w for w, l in v["nbrs"] if w in ids],
                "values": {"ops": arms[v["id"]]["summary"]["iters"]["mean"], "time": arms[v["id"]]["e2e_ms_mean"]}
                if v["id"] in arms
                else {},
                "highlight": "root" if v["id"] == 0 else None,
            }
            for v in sub
        ],
    }
    (run / f"volcano-ell{ell}.diagram.json").write_text(json.dumps(spec, indent=1))
    try:
        render(spec, str(run / f"volcano-ell{ell}"))
    except Exception as ex:  # noqa: BLE001
        print("volcano diagram failed for ell", ell, ex)

# ---------------------------------------------------------------- 2. 3D volcano heat map: strata = End-ring conductors
if R:
    measured = [v for v in verts if v["id"] in arms]
    conds = sorted({v["conductor"] for v in verts if v["levels"]}, key=lambda c: (sum(1 for _ in str(c)), c))
    # order strata by the total level (sum of levels over volcano primes): crater on top
    def height(v):
        return sum(v["levels"].values())

    heights = sorted({height(v) for v in verts if v["levels"]})
    hmax = max(heights) if heights else 0
    for metric, key, label in [("ops", "ops", "mean group operations per solve"), ("e2e", "e2e", "mean end-to-end ms (transport + solve)")]:
        fig = plt.figure(figsize=(11, 8))
        ax = fig.add_subplot(111, projection="3d")
        vals = {v["id"]: (arms[v["id"]]["summary"]["iters"]["mean"] if key == "ops" else arms[v["id"]]["e2e_ms_mean"]) for v in measured}
        vmin, vmax = min(vals.values()), max(vals.values())
        norm = colors.Normalize(vmin=vmin, vmax=vmax)
        cmap = matplotlib.colormaps["YlOrRd"]
        pos = {}
        # each stratum (conductor) is a ring at z = −(total level); vertices spread by angle in BFS order
        by_cond = defaultdict(list)
        for v in verts:
            if v["levels"]:
                by_cond[v["conductor"]].append(v)
        strata = sorted(by_cond.keys(), key=lambda c: (height(by_cond[c][0]), c))
        ring_r = {}
        for i, c in enumerate(strata):
            h = height(by_cond[c][0])
            r = 1.0 + 1.2 * h + 0.25 * (i % 3)
            ring_r[c] = r
            n = len(by_cond[c])
            for k, v in enumerate(by_cond[c]):
                ang = 2 * math.pi * k / max(n, 1) + 0.3 * h
                pos[v["id"]] = (r * math.cos(ang), r * math.sin(ang), -h)
            th = [2 * math.pi * k / 200 for k in range(201)]
            ax.plot([r * math.cos(t) for t in th], [r * math.sin(t) for t in th], [-h] * len(th), color="#888", lw=0.6, alpha=0.6)
            ax.text(r * 1.05, 0.15 * (i % 2), -h, f"f={c}: {n} walked, h={by_cond[c][0]['stratum_class_number']:,}", fontsize=6, color="#444")
        for v in verts:
            if v["id"] not in pos:
                continue
            for w, l in v["nbrs"]:
                if w in pos and w > v["id"]:
                    a, b = pos[v["id"]], pos[w]
                    ax.plot([a[0], b[0]], [a[1], b[1]], [a[2], b[2]], color="#bbb", lw=0.5, alpha=0.7)
        xs = [pos[v["id"]][0] for v in verts if v["id"] in pos and v["id"] not in vals]
        ys = [pos[v["id"]][1] for v in verts if v["id"] in pos and v["id"] not in vals]
        zs = [pos[v["id"]][2] for v in verts if v["id"] in pos and v["id"] not in vals]
        ax.scatter(xs, ys, zs, s=6, color="#aaa", depthshade=False)
        mx = [pos[i][0] for i in vals]
        my = [pos[i][1] for i in vals]
        mz = [pos[i][2] for i in vals]
        sc = ax.scatter(mx, my, mz, c=[vals[i] for i in vals], cmap=cmap, norm=norm, s=60, edgecolor="k", linewidth=0.4, depthshade=False)
        if len(vals) <= 40:
            for i in vals:
                ax.text(pos[i][0], pos[i][1], pos[i][2] + 0.08, f"v{i}", fontsize=6)
        ax.set_zlabel("− total volcano level (crater on top)")
        ax.set_xticks([])
        ax.set_yticks([])
        cb = fig.colorbar(sc, ax=ax, shrink=0.6, pad=0.08)
        cb.set_label(label)
        ax.set_title(
            f"3D isogeny-class heat map · {qname} · {label}\n{rid} · rings = endomorphism-ring strata (conductor f of End), "
            f"{len(vals)} measured of {len(pos)} walked curves" + (f" · expected √(πn/2) = {expected:,.0f}" if key == "ops" else ""),
            fontsize=9,
        )
        ax.view_init(elev=28, azim=-55)
        save(fig, f"volcano3d-{metric}")

# ---------------------------------------------------------------- 3. per-curve min / mean / max and every trial
if R:
    order = sorted(R["arms"], key=lambda a: (a["kind"] != "isogenous", a["meta"].get("conductor", 0) if isinstance(a["meta"], dict) else 0, a["arm"]))
    by_arm = defaultdict(list)
    for r in rows:
        by_arm[r["arm"]].append(int(r["iters"]))
    fig, ax = plt.subplots(figsize=(max(9, len(order) * 0.3), 4.8))
    for i, a in enumerate(order):
        s = a["summary"]["iters"]
        col = {"isogenous": "#2a6fdb", "iso-control": "#7a7a7a", "aa-replica": "#2e9e5b", "plant": "#b03a8c", "fold": "#e07b00"}.get(a["kind"], "#000")
        ax.scatter([i] * len(by_arm[a["arm"]]), by_arm[a["arm"]], s=4, color=col, alpha=0.25)
        ax.plot([i, i], [s["min"], s["max"]], color=col, lw=1)
        ax.errorbar([i], [s["mean"]], yerr=[s["ci95_half"]], fmt="o", color=col, ms=4, capsize=2)
    ax.axhline(expected, color="k", ls="--", lw=0.8, label=f"√(πn/2) = {expected:,.0f}")
    ax.set_xticks(range(len(order)))
    ax.set_xticklabels([a["arm"] for a in order], rotation=90, fontsize=6)
    ax.set_ylabel("group operations per solve")
    ax.set_yscale("log")
    ax.set_title(f"Per curve: every solve (dots), min–max (bar), mean ± 95 % CI · {rid} · {R['params']['trials']} seeds per arm")
    ax.legend(fontsize=7)
    save(fig, "per_curve")

    # per stratum / group: means with CI against the pooled mean
    groups = [g for g in R["groups"] if g["group"].startswith(("conductor:", "dist:", "parent_ell:", "field_degree:")) or ":level" in g["group"]]
    fig, ax = plt.subplots(figsize=(max(7, len(groups) * 0.45), 4))
    for i, g in enumerate(groups):
        s = g["summary"]["iters"]
        ax.errorbar([i], [s["mean"]], yerr=[s["ci95_half"]], fmt="o", color="#2a6fdb", ms=4, capsize=2)
        ax.text(i, s["mean"] + s["ci95_half"], f"n={s['n']}", fontsize=5, ha="center", va="bottom")
    ax.axhline(expected, color="k", ls="--", lw=0.8, label="√(πn/2)")
    ax.axhline(R["pooled"]["summary"]["iters"]["mean"], color="#2a6fdb", ls=":", lw=0.8, label="pooled isogenous mean")
    ax.set_xticks(range(len(groups)))
    ax.set_xticklabels([g["group"] for g in groups], rotation=90, fontsize=6)
    ax.set_ylabel("mean group operations ± 95 % CI")
    ax.set_title(f"Pooled by stratum, level, distance, parent ℓ and kernel field degree · {rid}")
    ax.legend(fontsize=7)
    save(fig, "per_group")

    # z-scores of every arm against the rest, with the Bonferroni band
    fig, ax = plt.subplots(figsize=(max(8, len(R["arms"]) * 0.3), 3.6))
    zs = [(a["arm"], a["tests"]["z_welch"] if a["tests"] else 0.0, a["kind"]) for a in order]
    for i, (name, z, kind) in enumerate(zs):
        col = {"isogenous": "#2a6fdb", "iso-control": "#7a7a7a", "aa-replica": "#2e9e5b", "plant": "#b03a8c", "fold": "#e07b00"}.get(kind, "#000")
        ax.bar(i, z, color=col, width=0.7)
    zc = R["tests"]["z_crit_bonferroni"]
    ax.axhline(zc, color="r", ls="--", lw=0.8, label=f"Bonferroni |z| = {zc:.2f} (family {R['tests']['family_size']})")
    ax.axhline(-zc, color="r", ls="--", lw=0.8)
    ax.set_xticks(range(len(zs)))
    ax.set_xticklabels([n for n, _, _ in zs], rotation=90, fontsize=6)
    ax.set_ylabel("Welch z vs pooled others")
    ax.set_title(f"Each arm against the pooled isogenous solves · {rid} (planted and fold arms are expected to leave the band)")
    ax.legend(fontsize=7)
    save(fig, "z_scores")

# ---------------------------------------------------------------- 4. generator sweep
G, grows = load("generators")
if G:
    pv = [p for p in G["per_vertex"] if "vertex" in p]
    fig, axes = plt.subplots(1, len(pv), figsize=(3.2 * len(pv), 4), sharey=True, squeeze=False)
    for ax, p in zip(axes[0], pv):
        v = p["vertex"]
        gs = [a for a in G["arms"] if a["vertex"] == v and a["kind"] == "generator"]
        means = [a["summary"]["iters"]["mean"] for a in gs]
        cis = [a["summary"]["iters"]["ci95_half"] for a in gs]
        ax.errorbar(range(len(gs)), means, yerr=cis, fmt="o", ms=2.5, color="#2a6fdb", capsize=1.5, lw=0.6)
        ax.axhline(G["rho"]["expected_iters_sqrt_pi_n_over_2"], color="k", ls="--", lw=0.8)
        ax.axhline(p["summary"]["mean"], color="#2a6fdb", ls=":", lw=0.8)
        ax.set_title(f"v{v}: {len(gs)} generators\nKW p = {p['kruskal_wallis']['p']:.2f}, var ratio {p['anova']['variance_ratio']:.2f}", fontsize=8)
        ax.set_xlabel("generator index")
    axes[0][0].set_ylabel("mean group operations ± 95 % CI")
    fig.suptitle(f"Generator sweep: P_i = r_i·P, same secret k · {rid} · {G['params']['trials']} seeds per generator", fontsize=9, y=1.04)
    save(fig, "generators")
    # distribution of generator means vs expected spread
    fig, ax = plt.subplots(figsize=(6, 3.6))
    allm = [a["summary"]["iters"]["mean"] for a in G["arms"] if a["kind"] == "generator"]
    ax.hist(allm, bins=30, color="#2a6fdb", alpha=0.7)
    ax.axvline(G["rho"]["expected_iters_sqrt_pi_n_over_2"], color="k", ls="--", lw=0.8, label="√(πn/2)")
    sd = G["tests"]["expected_sd_of_arm_mean"]
    ax.set_title(f"Per-generator means ({len(allm)} generators) · observed sd {G['tests']['observed_sd_of_arm_mean']:,.0f} vs expected {sd:,.0f}", fontsize=8)
    ax.set_xlabel("mean group operations per solve")
    ax.legend(fontsize=7)
    save(fig, "generator_means")

# ---------------------------------------------------------------- 5. shift matrix
Sm, _ = load("shift")
if Sm:
    srcs = sorted({m["source"] for m in Sm["shift_matrix"]})
    tgts = sorted({m["target"] for m in Sm["shift_matrix"]})
    mat = [[float("nan")] * len(tgts) for _ in srcs]
    for m in Sm["shift_matrix"]:
        mat[srcs.index(m["source"])][tgts.index(m["target"])] = m["mean"] / Sm["rho"]["expected_iters_sqrt_pi_n_over_2"]
    fig, ax = plt.subplots(figsize=(max(7, len(tgts) * 0.35), 1.2 + 0.5 * len(srcs)))
    im = ax.imshow(mat, cmap="RdBu_r", vmin=0.7, vmax=1.3, aspect="auto")
    ax.set_xticks(range(len(tgts)))
    ax.set_xticklabels([f"v{t}" for t in tgts], rotation=90, fontsize=6)
    ax.set_yticks(range(len(srcs)))
    src_v = {s["index"]: s["vertex"] for s in W["sources"]}
    ax.set_yticklabels([f"src {s} (v{src_v.get(s, '?')})" for s in srcs], fontsize=7)
    cb = fig.colorbar(im, ax=ax, shrink=0.8)
    cb.set_label("mean ops / √(πn/2)")
    tw = [p for p in Sm["per_vertex"] if "two_way" in p]
    extra = f" · η² target {tw[0]['two_way']['eta2_target']:.3f}, source {tw[0]['two_way']['eta2_source']:.3f}" if tw else ""
    ax.set_title(f"Instance drawn on the source curve, carried to every target · {rid} · {Sm['params']['trials']} seeds per cell{extra}", fontsize=8)
    save(fig, "shift_matrix")

print("plotted", rid)
