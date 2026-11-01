"""Lay out recorded rows (rows.jsonl, rows_m3.jsonl, rows_m3_symmetric.jsonl): D, widest matrix and time per system vs N."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

raw = [json.loads(l) for f in ("rows.jsonl", "rows_m3.jsonl", "rows_m3_symmetric.jsonl") if os.path.exists(f) for l in open(f) if l.strip()]
seen, rows = set(), []
for r in raw:  # the symmetric-only rerun repeats rows_m3's symmetric rows exactly; keep one
    k = (r["m"], r["N"], r["arm"], r["target"])
    if k not in seen:
        seen.add(k); rows.append(r)
style = {"plain-chain": ("#3b6ea8", "o", "plain tower, S3 chain"), "plain-smp1": ("#888", "s", "plain tower, one S_{m+1}"), "symmetric": ("#d08c2c", "^", "symmetric tower")}
fig, axes = plt.subplots(1, 3, figsize=(13, 4.2))
for m, ls in ((2, "-"), (3, "--")):
    for arm, (c, mk, lab) in style.items():
        pts = [r for r in rows if r["m"] == m and r["arm"] == arm]
        if not pts:
            continue
        Ns = sorted({r["N"] for r in pts})
        D = [max(r["D"] for r in pts if r["N"] == n) for n in Ns]
        cols = [max(r["max_cols"] for r in pts if r["N"] == n) for n in Ns]
        ms = [max(r["ms"] for r in pts if r["N"] == n) / 1000 for n in Ns]
        kw = dict(color=c, marker=mk, ls=ls, label=f"m={m}: {lab}")
        axes[0].plot(Ns, D, **kw); axes[1].plot(Ns, cols, **kw); axes[2].plot(Ns, ms, **kw)
        for n, d, cc, t in zip(Ns, D, cols, ms):
            if any(r["timed_out"] for r in pts if r["N"] == n):
                axes[0].annotate("≥ (budget)", (n, d), fontsize=7, color=c, xytext=(3, 3), textcoords="offset points")
                axes[2].annotate("budget stop", (n, t), fontsize=7, color=c, xytext=(3, 3), textcoords="offset points")
axes[0].set_ylabel("F4 solving degree D (max over targets)"); axes[1].set_ylabel("widest matrix (columns)"); axes[2].set_ylabel("time per system (s, max over targets)")
axes[1].set_yscale("log"); axes[2].set_yscale("log")
for a in axes:
    a.set_xlabel("N = m·t (tower unknowns)"); a.grid(alpha=.3)
axes[0].legend(fontsize=7)
fig.suptitle("Symmetric vs plain Kummer PKM tower, f4_fp, p = 786433, 2 random targets per cell (rows.jsonl, rows_m3.jsonl, rows_m3_symmetric.jsonl)", fontsize=9)
fig.tight_layout(); fig.savefig("d_vs_n.svg"); fig.savefig("d_vs_n.png", dpi=150)
