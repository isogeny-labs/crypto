#!/usr/bin/env python3
"""Plot RECORDED values only (no result is computed here; AGENTS.md "no Python").
Dots: every verified measured record's time.solve_wall_ns (or online.wall_ns) read from
the sessions' records.jsonl.  Bars: median and quartiles read from summary.json, which
analysis/summarise.sh (jq) wrote.  Usage: plot_wall.py STUDY_DIR OUTBASE   (writes OUTBASE.svg/.png)
"""
import json, sys, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

study, outbase = sys.argv[1], sys.argv[2]
S = json.load(open(f"{study}/analysis/summary.json"))
GROUPS = {  # group -> session dirs (same names as run_all.sh)
    "smoke": {"base": ["base-smoke", "base-smoke-rev"], "fix": ["fix-smoke", "fix-smoke-rev"],
              "fix-notrim": ["fix-notrim-smoke"]},
    "claim": {"base": ["base-claim", "base-claim-rev"], "fix": ["fix-claim", "fix-claim-rev"],
              "fix-notrim": ["fix-notrim-claim"]},
}
COL = {"base": "#c0504d", "fix": "#2f6db0", "fix-notrim": "#8a8a8a"}
OFF = {"base": -0.27, "fix": 0.0, "fix-notrim": 0.27}

def values(sessions, arm, metric):
    out = []
    for s in sessions:
        p = f"{study}/sessions/{s}/records.jsonl"
        if not os.path.exists(p):
            continue
        for line in open(p):
            r = json.loads(line)
            if r.get("schema") != "ecbench.record/v1" or r["warmup"] or r["arm"] != arm \
               or r["outcome"]["status"] != "verified":
                continue
            v = r["time"]["solve_wall_ns"] if metric == "solve_wall_ns" else (r.get("online") or {}).get("wall_ns")
            if v is not None:
                out.append(v)
    return out

panels = [("smoke", "solve_wall_ns", "smoke-prime (20-bit prime, 2 targets x 3 rounds x 2 sessions)"),
          ("claim", "solve_wall_ns", "claim-koblitz-r10 (Koblitz a=1 n=17), whole solve"),
          ("claim", "online_wall_ns", "claim-koblitz-r10, online window")]
fig, axes = plt.subplots(len(panels), 1, figsize=(10, 11.5))
for ax, (key, metric, title) in zip(axes, panels):
    gname = {"base": f"{key}-base", "fix": f"{key}-fix", "fix-notrim": f"{key}-notrim"}
    gsum = {g: S.get(gname[g]) for g in gname}
    arms = [a for a in (gsum["base"] or gsum["fix"])["arms"].keys()]
    arms = [a for a in arms if (gsum["base"] or gsum["fix"])["arms"][a][metric]]
    for i, arm in enumerate(arms):
        for g in ("base", "fix", "fix-notrim"):
            if gsum[g] is None or arm not in gsum[g]["arms"] or not gsum[g]["arms"][arm][metric]:
                continue
            st = gsum[g]["arms"][arm][metric]
            ys = values(GROUPS[key][g], arm, metric)
            x = i + OFF[g]
            ax.scatter([x + (j % 7 - 3) * 0.018 for j in range(len(ys))], ys, s=7, color=COL[g], alpha=0.55, linewidths=0)
            ax.plot([x, x], [st["q1"], st["q3"]], color=COL[g], lw=2.2)
            ax.plot([x - 0.1, x + 0.1], [st["median"]] * 2, color="black", lw=1.6)
            ax.text(x, st["max"] * 1.25, f"n={st['n']}", ha="center", fontsize=6, color=COL[g], rotation=90)
    ax.set_yscale("log")
    ax.set_xticks(range(len(arms)))
    ax.set_xticklabels(arms, rotation=25, ha="right", fontsize=8)
    ax.set_ylabel("ns (log scale)")
    ax.set_title(title, fontsize=9.5, loc="left")
    ax.grid(axis="y", alpha=0.3, which="both", lw=0.4)
    ax.axhline(1e6, color="#999", lw=0.6, ls=":")
    if key == "smoke":
        ax.text(len(arms) - 0.5, 1.05e6, "1 ms (grading v3 admits no wall below)", fontsize=6.5, ha="right", color="#666")
h = [plt.Line2D([], [], marker="o", ls="", color=COL[g], label=f"{g}") for g in ("base", "fix", "fix-notrim")]
h.append(plt.Line2D([], [], color="black", lw=1.6, label="median (bar: quartiles)"))
axes[0].legend(handles=h, fontsize=7.5, loc="upper right", ncol=4)
fig.suptitle("Solve wall time per arm, merge-base binary vs PR #2455 binary (descriptive; shared 4-vCPU VM, below L3)\n"
             "dots = recorded verified measured runs; sessions: research/ecbench_allocator_confound_20261023/sessions/*", fontsize=9.5)
fig.tight_layout(rect=(0, 0, 1, 0.965))
fig.savefig(outbase + ".svg")
fig.savefig(outbase + ".png", dpi=150)
