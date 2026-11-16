"""Lay out recorded values of e5c.json (written by the Rust example
generator_effect_e5c). No result is computed here; the only arithmetic is the
sum of recorded histograms over scans and the reference line
scans * 2^24 * 2^-(c+1), the protocol's stated expectation."""
import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = os.path.dirname(os.path.abspath(__file__))
d = json.load(open(os.path.join(here, "e5c.json")))
col = {"P-192": "#4c78a8", "P-224": "#c0392b"}

fig, axes = plt.subplots(1, 2, figsize=(13, 5.2), constrained_layout=True)

# A: leading bits of x([h]P) shared with G.x, pooled over the 17 scans per curve.
ax = axes[0]
for cv in d["curves"]:
    hists = [b["shared_bits_hist"] for b in cv["bases"]]
    top = max(len(h) for h in hists)
    pooled = [sum(h[c] for h in hists if c < len(h)) for c in range(top)]
    total = len(hists) * cv["bases"][0]["h_max"]
    ax.semilogy(range(top), [max(v, 0.5) for v in pooled], "o", ms=4,
                color=col[cv["curve"]],
                label=f'{cv["curve"]}: {len(hists)} scans × 2^24 multiples')
    ax.semilogy(range(top), [total * 2.0 ** -(c + 1) for c in range(top)], "-",
                lw=1, color=col[cv["curve"]], alpha=0.5)
ax.set_xlabel("c = leading bits shared by x([h]P) and x(G)")
ax.set_ylabel("count of h (empty bins drawn at 0.5)")
ax.set_title("A. Shared leading bits with G.x (lines: expectation N·2^-(c+1))")
ax.legend(fontsize=8)
ax.grid(alpha=0.3)

# B: per scan, the most leading bits shared; a match would sit at ell.
ax = axes[1]
for k, cv in enumerate(d["curves"]):
    off = 0.15 * (2 * k - 1)
    xs = [b["abscissa_index"] + (0.3 if b["root"] == "large" else 0) + off for b in cv["bases"]]
    ys = [b["max_shared_bits"] for b in cv["bases"]]
    ax.plot(xs, ys, "o", color=col[cv["curve"]], label=f'{cv["curve"]}: scans vs ±G, 0 matches')
    ax.axhline(cv["ell"], color=col[cv["curve"]], ls="--", lw=1)
    ax.text(15.5, cv["ell"] - 7, f'exact match = {cv["ell"]} bits ({cv["curve"]})',
            ha="right", fontsize=8, color=col[cv["curve"]])
    for c in cv["positive_control"]:
        ax.plot([-0.7 + off], [c["max_shared_bits"]], "*", ms=12, color=col[cv["curve"]],
                label=f'{cv["curve"]} planted [12345]P_first: hit at h = {c["matches"][0]["h"]}'
                if c["target"] == "+T" else None)
ax.set_xlabel("abscissa index of P among the first 16 valid x (0 = P_first; both roots at 0)")
ax.set_ylabel("max over h ≤ 2^24 of shared leading bits")
ax.set_title("B. Best near-miss per scan; stars = positive controls")
ax.set_ylim(0, 240)
ax.legend(fontsize=8, loc="center right")
ax.grid(alpha=0.3)

fig.suptitle("E5c: G vs ±[h]·P for 1 ≤ h ≤ 2^24 on P-192 and P-224 "
             "(run 20261019T2240Z, e5c.json)", fontsize=11)
fig.savefig(os.path.join(here, "e5c.svg"))
fig.savefig(os.path.join(here, "e5c.png"), dpi=150)
