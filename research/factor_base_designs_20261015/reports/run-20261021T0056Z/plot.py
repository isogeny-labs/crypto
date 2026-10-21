"""Plot recorded values for Amendment 7 (X1, X2 on a 64 GiB host) beside the earlier
attempts at the same systems, from runs/*.jsonl only: each system's peak RSS against its
wall time to the stop, with each attempt's ulimit -v. No fit or new computation.
Rendering aid only (AGENTS.md "no Python")."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")
load = lambda n: [json.loads(l) for l in open(os.path.join(runs, n)) if l.strip()]
GB = 1e6  # kB per GB (decimal), axis only
cells = [
    ("A0, m = 4, t = 4", [("L1 (15 GB container)", "L1-A0-m4-t4.jsonl", "#c0392b"),
                          ("X1 (64 GiB r7i.2xlarge)", "X1-A0-m4-t4.jsonl", "#2a78d6")]),
    ("A11, m = 4, t = 5", [("L2 (15 GB container)", "L2-A11-m4-t5.jsonl", "#c0392b"),
                           ("L2r (15 GB container)", "L2r-A11-m4-t5.jsonl", "#e08a2c"),
                           ("X2 (64 GiB r7i.2xlarge)", "X2-A11-m4-t5.jsonl", "#2a78d6")]),
]
plt.rcParams.update({"font.size": 8.5, "axes.edgecolor": "#888", "xtick.color": "#444", "ytick.color": "#444"})
fig, axes = plt.subplots(1, 2, figsize=(10.5, 4.1), sharey=True)
for ax, (title, series) in zip(axes, cells):
    limits = set()
    for label, f, col in series:
        rows = load(f)
        for r in rows:
            stop = "cgroup OOM kill" if "signal: 9" in (r.get("failure_reason") or "") else "ulimit -v malloc failure"
            ax.scatter(r["time"], r["peak_rss_kb"] / GB, color=col, s=36, zorder=3,
                       label=f"{label}: {stop}" if r["target"] == 0 else None)
            ax.annotate(f"t{r['target']}  {r['time']:.0f} s", (r["time"], r["peak_rss_kb"] / GB),
                        textcoords="offset points", xytext=(6, -3 - 7 * r["target"]), fontsize=6.3, color="#444")
            limits.add((r["ulimit_v_kb"], col))
    for v, col in sorted(limits):
        ax.axhline(v / GB, color=col, ls="--", lw=0.9, alpha=0.8)
        ax.text(0.02, v / GB + 0.6, f"ulimit -v {v:,}".replace(",", " "),
                fontsize=6.3, color=col, transform=ax.get_yaxis_transform())
    ax.set_xscale("log")
    ax.set_xlim(50, 20000)
    ax.axvline(14400, color="#555", ls=":", lw=1)
    ax.text(14400, 2, " 4 h budget", fontsize=6.5, color="#555", rotation=90, va="bottom")
    ax.set_title(title, fontsize=9)
    ax.set_xlabel("wall time to the stop (s, log; descriptive, hosts differ)")
    ax.grid(color="#e5e5e5", lw=0.6)
    ax.legend(frameon=True, framealpha=1, edgecolor="none", fontsize=6.3, loc="center left")
axes[0].set_ylabel("peak RSS (GB = 10^6 kB)")
axes[0].set_ylim(0, 72)
fig.suptitle("The two open m = 4 cells: every attempt's stop, 15 GB containers against the 64 GiB host (run-20261021T0056Z)", fontsize=9.5)
fig.text(0.5, 0.01, "Rows: runs/{L1,X1}-A0-m4-t4.jsonl, runs/{L2,L2r,X2}-A11-m4-t5.jsonl. Dashed lines: each attempt's ulimit -v. "
         "No system completed; no field_ops exist.", ha="center", fontsize=6.8, color="#555")
fig.tight_layout(rect=(0, 0.04, 1, 0.94))
fig.savefig(os.path.join(here, "m4_memory_hosts.svg"))
fig.savefig(os.path.join(here, "m4_memory_hosts.png"), dpi=150)
