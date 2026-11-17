"""Plot recorded values for Amendment 6 (L2r) beside L2 from runs/*.jsonl only: each
system's peak RSS against the three memory limits (L2r's ulimit -v, the host's memory
cgroup, the 14.5 GB that Amendment 5 registered), and its wall time to the stop.
The limit values are copied from the rows and runs/L2r-A11-m4-t5.dmesg.txt.
No fit or new computation. Rendering aid only (AGENTS.md "no Python")."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
runs = os.path.join(here, "..", "..", "runs")
load = lambda n: [json.loads(l) for l in open(os.path.join(runs, n)) if l.strip()]
l2, l2r = load("L2-A11-m4-t5.jsonl"), load("L2r-A11-m4-t5.jsonl")
CG = 14007508  # memory.limit_in_bytes 14343688192 B, runs/L2r-A11-m4-t5.dmesg.txt
GB = 1e6  # kB per GB (decimal), axis only
C = {"L2": "#c0392b", "L2r": "#2a78d6"}

plt.rcParams.update({"font.size": 8.5, "axes.edgecolor": "#888", "xtick.color": "#444", "ytick.color": "#444"})
fig, (a1, a2) = plt.subplots(1, 2, figsize=(10.5, 3.9))
xs = {("L2", 0): 0, ("L2", 1): 1, ("L2r", 0): 2.4, ("L2r", 1): 3.4}
for name, rows in [("L2", l2), ("L2r", l2r)]:
    for r in rows:
        x = xs[(name, r["target"])]
        stop = "cgroup OOM kill (SIGKILL)" if "signal: 9" in r["failure_reason"] else "ulimit -v malloc failure (SIGABRT)"
        a1.bar(x, r["peak_rss_kb"] / GB, width=0.7, color=C[name], label=f"{name}: {stop}" if r["target"] == 0 else None)
        a1.text(x, r["peak_rss_kb"] / GB + 0.05, f"{r['peak_rss_kb']:,}".replace(",", " "), ha="center", fontsize=6.5)
        a2.bar(x, r["time"], width=0.7, color=C[name])
        a2.text(x, r["time"] + 2, f"{r['time']:.1f}", ha="center", fontsize=7)
for ax in (a1, a2):
    ax.set_xticks(list(xs.values()), [f"{n}\ntarget {t}" for n, t in xs])
    ax.grid(axis="y", color="#e5e5e5", lw=0.6)
for v, lab, ls in [(14500000, "registered in Amendment 5: ulimit -v 14 500 000", ":"),
                   (CG, "host memory cgroup: 14 007 508", "-"),
                   (l2r[0]["ulimit_v_kb"], f"L2r: ulimit -v {l2r[0]['ulimit_v_kb']:,}".replace(",", " "), "--")]:
    a1.axhline(v / GB, color="#555", ls=ls, lw=1, label=lab)
a1.set_ylim(12.0, 16.9)
a1.set_ylabel("peak RSS (GB = 10^6 kB; labels in kB)")
a1.set_title("peak resident memory per system against the limits", fontsize=9)
a1.legend(frameon=True, framealpha=1, edgecolor="none", fontsize=6.3, loc="upper center", ncol=2)
a2.set_ylabel("wall time to the stop (s)")
a2.set_ylim(0, 160)
a2.set_title("wall time per system (descriptive; two containers)", fontsize=9)
fig.suptitle("A11, m = 4, t = 5 (b = 5, 42 variables): L2 (Amendment 5) and L2r (Amendment 6), run-20261019T1740Z", fontsize=9.5)
fig.text(0.5, 0.01, "Rows: runs/L2-A11-m4-t5.jsonl, runs/L2r-A11-m4-t5.jsonl; limits: rows' ulimit_v_kb and the cgroup limit read in L2r's pre-flight "
         "(runs/L2r-A11-m4-t5.dmesg.txt). No system completed; no field_ops exist.", ha="center", fontsize=6.8, color="#555")
fig.tight_layout(rect=(0, 0.04, 1, 0.94))
fig.savefig(os.path.join(here, "l2_l2r_memory.svg"))
fig.savefig(os.path.join(here, "l2_l2r_memory.png"), dpi=150)
