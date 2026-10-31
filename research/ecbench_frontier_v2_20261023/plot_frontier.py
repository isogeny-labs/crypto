"""Plot the recorded frontier values (layout only; every number is read from
records.json, frontier-v1.json and frontier-v2.json, written by extract.sh
from the sealed bound records and the two frontier documents).

Usage: python3 plot_frontier.py   (in this directory)
Writes ops-status.svg and alpha-intervals.svg.
"""
import json

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

recs = json.load(open("records.json"))
old = json.load(open("frontier-v1.json"))
new = json.load(open("frontier-v2.json"))
by_id = {r["bound_id"]: r for r in recs}
by_file = {r["file"]: r for r in recs}


def status(front, bid, unranked=True):
    for d in front["domains"]:
        if bid in d["frontier"]:
            return "frontier"
        if bid in d["dominated"]:
            return "dominated"
        if unranked and bid in d.get("unranked", []):
            return "not ranked"
    return "inadmissible"


DOMAINS = [("prime", "toy"), ("koblitz", "toy"), ("koblitz", "medium")]
COLOR = {"frontier": "#1a9850", "dominated": "#d73027", "not ranked": "#999999",
         "inadmissible": "#000000"}
MARK = {"frontier": "o", "dominated": "x", "not ranked": "s", "inadmissible": "^"}

rows = {dom: [] for dom in DOMAINS}
for r in recs:
    dom = (r["family"], r["tier"])
    if dom not in rows or r["fit_version"] != 2:
        continue
    stem = r["file"][:-3] if r["file"].endswith("-v2") else r["file"]
    v1 = by_file.get(stem)
    rows[dom].append({
        "label": f'{r["method"]}  [{stem}]',
        "ops": r["ops"], "ci": r["ops_ci95"],
        "new": status(new, r["bound_id"]),
        "old": status(old, v1["bound_id"], unranked=False) if v1 else "absent",
        "large": bool(r["tail"] and r["tail"]["large"]),
        "a1": (v1["alpha"], v1["alpha_ci95"]) if v1 else None,
        "a2": (r["alpha"], r["alpha_ci95"]),
        "sizes": len(r["sizes"]),
    })

fig, axes = plt.subplots(3, 1, figsize=(11, 17), gridspec_kw={"height_ratios": [14, 11, 10]})
for ax, dom in zip(axes, DOMAINS):
    rs = sorted([x for x in rows[dom] if x["ops"] is not None], key=lambda x: -x["ops"])
    for i, x in enumerate(rs):
        lo, hi = x["ci"] if x["ci"] else (x["ops"], x["ops"])
        ax.plot([lo, hi], [i, i], color=COLOR[x["new"]], lw=2)
        ax.scatter([x["ops"]], [i], color=COLOR[x["new"]], marker=MARK[x["new"]], s=50, zorder=3)
        tag = f'v1 page: {x["old"]}  ->  v2: {x["new"]}' + ("  (large tail)" if x["large"] else "")
        ax.annotate(tag, (hi, i), xytext=(6, -3), textcoords="offset points", fontsize=7)
    ax.set_yticks(range(len(rs)))
    ax.set_yticklabels([x["label"] for x in rs], fontsize=7)
    ax.set_xscale("log")
    ax.set_xlabel("ops = mean S / sqrt(pi/2A)  (ratio to the generic floor, 95 % two-stage bootstrap)")
    d2 = [d for d in new["domains"] if (d["family"], d["tier"]) == dom][0]
    lead = d2["ops_leader"] or f'none named (tie set of {len(d2["ops_tie"])})'
    ax.set_title(f"{dom[0]} {dom[1]}: fit-v2 bounds, ops leader {lead}", fontsize=10)
    ax.grid(axis="x", alpha=0.3)
    ax.set_xlim(right=ax.get_xlim()[1] * 30)
handles = [plt.Line2D([], [], color=COLOR[k], marker=MARK[k], lw=2, label=k)
           for k in ["frontier", "dominated", "not ranked", "inadmissible"]]
axes[0].legend(handles=handles, title="frontier v2 status", loc="upper right", fontsize=8)
fig.suptitle(f'docs/bounds frontier: {old["frontier_id"]} (v1, main 3a1ea541d) -> '
             f'{new["frontier_id"]} (v2)\nrow = fit-v2 bound; status on the v1 page is that of its '
             "published v1 record", fontsize=10)
fig.tight_layout()
fig.savefig("ops-status.svg")

fig, ax = plt.subplots(figsize=(10, 14))
allrows = [x for dom in DOMAINS for x in rows[dom] if x["a1"]]
allrows.sort(key=lambda x: x["label"])
for i, x in enumerate(allrows):
    (a1, c1), (a2, c2) = x["a1"], x["a2"]
    if c1:
        ax.plot(c1, [i + 0.15] * 2, color="#888888", lw=2)
    if a1 is not None:
        ax.scatter([a1], [i + 0.15], color="#888888", s=14)
    if c2:
        ax.plot(c2, [i - 0.15] * 2, color="#2166ac", lw=2)
    if a2 is not None:
        ax.scatter([a2], [i - 0.15], color="#2166ac", s=14,
                   marker="o" if c2 else "D")
ax.axvline(0.5, color="k", lw=0.8, ls="--")
ax.set_yticks(range(len(allrows)))
ax.set_yticklabels([f'{x["label"]} ({x["sizes"]} sizes)' for x in allrows], fontsize=6)
ax.set_xlabel("fitted exponent alpha of gae = C r^alpha, 95 % interval (dashed: declared 0.5)")
ax.legend(handles=[plt.Line2D([], [], color="#888888", lw=2, label="fit v1 (published record)"),
                   plt.Line2D([], [], color="#2166ac", lw=2, label="fit v2 (twin); diamond = no interval")],
          loc="upper right", fontsize=8)
ax.set_title("alpha intervals before and after the rejecting bootstrap (fit v1 vs fit v2)", fontsize=10)
ax.grid(axis="x", alpha=0.3)
fig.tight_layout()
fig.savefig("alpha-intervals.svg")
