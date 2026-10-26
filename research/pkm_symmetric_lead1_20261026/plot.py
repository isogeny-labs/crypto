"""Lay out recorded rows: D and last productive degree, widest matrix, time per system vs N.
Sources: this directory (rows.jsonl, rows_n18.jsonl: m = 3 symmetric; rows_m4.jsonl: m = 4) and
../pkm_symmetric_tower_20261026/rows_m3.jsonl (m = 3 plain chain, PR #2632). Nothing is computed here."""
import json, os
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

files = ["rows.jsonl", "rows_n18.jsonl", "rows_m4.jsonl", "../pkm_symmetric_tower_20261026/rows_m3.jsonl"]
raw = [json.loads(l) for f in files if os.path.exists(f) for l in open(f) if l.strip()]
seen, rows = set(), []
for r in raw:
    k = (r["m"], r["N"], r["arm"], r["target"])
    if r["arm"] in ("plain-chain", "symmetric") and k not in seen:
        seen.add(k); rows.append(r)
style = {"plain-chain": ("#3b6ea8", "o", "plain tower, S3 chain"), "symmetric": ("#d08c2c", "^", "symmetric tower")}
fig, axes = plt.subplots(1, 3, figsize=(13, 4.2))
for m, ls in ((3, "-"), (4, "--")):
    for arm, (c, mk, lab) in style.items():
        pts = [r for r in rows if r["m"] == m and r["arm"] == arm]
        if not pts:
            continue
        Ns = sorted({r["N"] for r in pts})
        D = [max(r["D"] for r in pts if r["N"] == n) for n in Ns]
        lp = [max(r["last_productive"] for r in pts if r["N"] == n) for n in Ns]
        cols = [max(r["max_cols"] for r in pts if r["N"] == n) for n in Ns]
        ms = [max(r["ms"] for r in pts if r["N"] == n) / 1000 for n in Ns]
        mk = mk if m == 3 else ("s" if arm == "plain-chain" else "D")
        kw = dict(color=c, marker=mk, ls=ls, ms=7 if m == 4 else 5)
        axes[0].plot(Ns, D, label=f"m={m}: {lab}, D", **kw)
        axes[0].plot(Ns, lp, label=f"m={m}: {lab}, last productive", color=c, marker=mk, ls=":", alpha=.6)
        axes[1].plot(Ns, cols, label=f"m={m}: {lab}", **kw); axes[2].plot(Ns, ms, label=f"m={m}: {lab}", **kw)
        for n, d, t in zip(Ns, D, ms):
            if any(r["timed_out"] for r in pts if r["N"] == n):
                axes[0].annotate("≥ (budget)", (n, d), fontsize=7, color=c, xytext=(3, 3), textcoords="offset points")
axes[0].set_ylabel("degree (max over targets)"); axes[1].set_ylabel("widest matrix (columns)"); axes[2].set_ylabel("time per system (s, max over targets)")
axes[1].set_yscale("log"); axes[2].set_yscale("log")
for a in axes:
    a.set_xlabel("N = m·t (tower unknowns)"); a.grid(alpha=.3)
axes[0].legend(fontsize=6); axes[1].legend(fontsize=6)
fig.suptitle("Symmetric vs plain Kummer PKM tower, f4_fp, p = 786433 (recorded rows; times of m = 4 cells measured under contention)", fontsize=9)
fig.tight_layout(); fig.savefig("d_vs_n.svg"); fig.savefig("d_vs_n.png", dpi=150)
