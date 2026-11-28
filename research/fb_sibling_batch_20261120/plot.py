"""Lay out the recorded values of data/console.txt (printed by the Rust
example `fb_domain_rootfind`) as one compact figure.  It computes no result:
the plotted numbers, including the ratios used as labels, are read from the
tables the example printed.

    python3 plot.py   # writes sibling_batch.svg and sibling_batch.png (150 dpi)
"""
import pathlib
import re

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
lines = (HERE / "data" / "console.txt").read_text().splitlines()
CONF = re.compile(r"^\s*(\d)\s+(\d+)\s+(\d+)\s+(\d+)\s+\|")
BATCH = re.compile(
    r"batched \| total fmul scan\s+(\d+) frob\s+(\d+) batch\s+(\d+) \| wall scan\s+([\d.]+)s "
    r"frob\s+([\d.]+)s batch\s+([\d.]+)s \| scan/batch\s+([\d.]+)x frob/batch\s+([\d.]+)x"
)
rows, conf = [], None
for line in lines:
    m = CONF.match(line)
    if m:
        conf = tuple(int(g) for g in m.groups()[:3])
        continue
    b = BATCH.search(line)
    if b and conf:
        g = b.groups()
        rows.append(dict(m=conf[0], bits=conf[1], d=conf[2],
                         fs=int(g[0]), ff=int(g[1]), fb=int(g[2]),
                         ws=float(g[3]), wf=float(g[4]), wb=float(g[5]),
                         rs=g[6], rf=g[7]))
assert len(rows) == 11, f"expected 11 configurations, read {len(rows)}"

labels = [f"m{r['m']} {r['bits']}b\n|D|={r['d']}" for r in rows]
x = list(range(len(rows)))
w = 0.27
fig, (a1, a2) = plt.subplots(2, 1, figsize=(7.6, 5.0), sharex=True)
for ax, keys, title, unit in (
    (a1, ("fs", "ff", "fb"), "Total counted multiplications (F4 + root step + batched)", "fmul"),
    (a2, ("ws", "wf", "wb"), "Wall time of all targets (L0)", "s"),
):
    for k, (key, name, col) in enumerate(zip(keys, ("scan", "Frobenius", "batched"),
                                              ("#8a8a8a", "#4a78b5", "#d1782c"))):
        ax.bar([i + (k - 1) * w for i in x], [r[key] for r in rows], w, label=name, color=col)
    ax.set_yscale("log")
    ax.set_ylabel(unit, fontsize=8)
    ax.set_title(title, loc="left", fontsize=9)
    ax.tick_params(labelsize=7)
for i, r in enumerate(rows):
    a2.annotate(f"{r['rs']}×", (i + w, r["wb"]), textcoords="offset points", xytext=(0, 2),
                ha="center", fontsize=6)
a1.legend(fontsize=7, ncol=3, loc="upper left")
a2.set_xticks(x)
a2.set_xticklabels(labels, fontsize=6)
fig.text(0.01, 0.005, "Label: scan ÷ batched wall, as printed. Same solution set in every mode.",
         fontsize=6)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(HERE / "sibling_batch.svg")
fig.savefig(HERE / "sibling_batch.png", dpi=150, pil_kwargs={"optimize": True})
print("wrote sibling_batch.svg, sibling_batch.png")
