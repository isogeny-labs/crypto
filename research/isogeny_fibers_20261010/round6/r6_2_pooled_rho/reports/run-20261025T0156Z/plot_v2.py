"""Tabulate and plot recorded R6.2 v2 values. No result is computed here: the
counts come from the native example; this script only reads them, prints a
table and plots (AGENTS.md "Implementation language: no Python")."""
import json, pathlib, sys
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(__file__).parent
base = here.parent.parent
v2dir = base / "v2"
v1 = json.load(open(base / "paths.json"))
fresh = json.load(open(v2dir / "paths_v2.json"))
replay = json.load(open(v2dir / "paths_v2_replay.json"))
cls = lambda d: [c for c in d["classes"] if "p" in c]

def pairs(d):
    return [(c, q) for c in cls(d) for q in c["pairs"]]

rows = []
for name, d in (("fresh", fresh), ("replay", replay)):
    ps = pairs(d)
    n = len(ps)
    bad = sum(1 for _, q in ps if not q["legacy"].get("route_executable", True))
    shorter = sum(1 for _, q in ps if q["legacy"]["length"] is not None and q["legacy"]["length"] < q["length"])
    twist = sum(1 for _, q in ps if q["ends_on_twist_of_b"])
    rows.append({"set": name, "classes": len(cls(d)), "pairs": n,
                 "corrected_routes_validated": sum(q["route_validated_forward"] for _, q in ps),
                 "corrected_routes_ending_on_twist_of_b": twist,
                 "legacy_routes_not_executable": bad,
                 "legacy_length_shorter_than_corrected": shorter})
json.dump(rows, open(here / "analysis_v2.json", "w"), indent=1)
for r in rows: print(r)

def save(fig, name):
    svg = here / (name + ".svg")
    fig.savefig(svg)
    t = svg.read_text(encoding="utf-8").splitlines()
    svg.write_text("\n".join(l.rstrip() for l in t) + "\n", encoding="utf-8")
    fig.savefig(here / (name + ".png"), dpi=150)

# Fig 1: mean enumerations per pair against component size, v1 frozen vs v2 (paired replay).
fig, ax = plt.subplots(figsize=(6.8, 4.6))
v1c = {(c["p"], c["a"], c["b"]): c for c in cls(v1)}
ax.scatter([c["component_size"] for c in cls(v1)], [c["mean_enumerations"] for c in cls(v1)],
           marker="o", facecolors="none", edgecolors="#888", label="v1 frozen: outgoing edges, routes not all executable (committed paths.json)")
pos = [c for c in cls(replay) if c["mean_total_enumerations"] > 0]
zero = [c for c in cls(replay) if c["mean_total_enumerations"] == 0]
ax.scatter([c["component_size"] for c in pos], [c["mean_total_enumerations"] for c in pos],
           marker="s", color="#2a6fdb", label="v2 replay, same 18 start curves: search + certification + validation")
ax.scatter([c["component_size"] for c in cls(fresh) if c["mean_total_enumerations"] > 0],
           [c["mean_total_enumerations"] for c in cls(fresh) if c["mean_total_enumerations"] > 0],
           marker="^", color="#d9822b", label="v2 fresh classes (v1 protocol, seed 7)")
for c in pos:
    k = (c["p"], c["a"], c["b"])
    if k in v1c:
        o = v1c[k]
        ax.plot([o["component_size"], c["component_size"]], [o["mean_enumerations"], c["mean_total_enumerations"]],
                color="#bbb", lw=.6, zorder=0)
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("component size of the start curve (curves, graph of each version)")
ax.set_ylabel("mean neighbour enumerations per pair (count)")
ttl = "R6.2 paths v2 (run-20261025T0156Z): 10 pairs per class; grey line = same start curve, v1 to v2"
if zero:
    ttl += f"\n{len(zero)} replay class(es) with a single-curve component (0 enumerations) not drawn on the log axis"
ax.set_title(ttl, fontsize=8)
ax.legend(fontsize=6.5, loc="upper left"); ax.grid(alpha=.3, which="both"); fig.tight_layout()
save(fig, "paths_v2_enumerations_vs_component")

# Fig 2: per class, pairs (of 10) whose pre-fix (outgoing-edge) route fails forward validation.
fig, ax = plt.subplots(figsize=(7.2, 4.2))
w = 0.4
for off, (nm, d, col) in enumerate((("v2 fresh classes", fresh, "#d9822b"), ("v2 replay of the v1 classes", replay, "#2a6fdb"))):
    cs = cls(d)
    ax.bar([i + (off - .5) * w for i in range(len(cs))],
           [sum(1 for q in c["pairs"] if not q["legacy"].get("route_executable", True)) for c in cs],
           width=w, color=col, label=nm)
ax.set_xticks(range(len(cls(fresh))))
ax.set_xticklabels([f"{c['pbits']}b #{i+1}" for i, c in enumerate(cls(fresh))], rotation=70, fontsize=6.5)
ax.set_ylabel("pairs of 10 whose pre-fix route is not executable forward")
ax.set_title("R6.2 paths v2: forward validation of the pre-fix search's routes on the corrected graph's pairs", fontsize=8)
ax.legend(fontsize=7); ax.grid(alpha=.3, axis="y"); fig.tight_layout()
save(fig, "paths_v2_legacy_not_executable")
