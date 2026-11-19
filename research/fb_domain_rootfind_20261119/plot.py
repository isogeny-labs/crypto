"""Lay out the recorded values of data/console.txt (printed by the Rust
example `fb_domain_rootfind`) as one figure.  It computes no result: every
plotted number is read from the table the example printed, including the
ratios used as bar labels.

    python3 plot.py   # writes root_step.svg and root_step.png (150 dpi)
"""
import pathlib
import re

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = pathlib.Path(__file__).resolve().parent
ROW = re.compile(
    r"^\s*(\d)\s+(\d+)\s+(\d+)\s+(\d+)\s+\|\s+(\d+)\s+(\d+)\s+([\d.]+)x\s+\|"
    r"\s+(\d+)\s+([\d.]+)%\s+\|\s+([\d.]+)s\s+([\d.]+)s\s+([\d.]+)x\s+\|\s+([\d.]+)"
)

rows = []
for line in (HERE / "data" / "console.txt").read_text().splitlines():
    m = ROW.match(line)
    if m:
        g = m.groups()
        rows.append(
            dict(
                m=int(g[0]), bits=int(g[1]), d=int(g[2]),
                scan=int(g[4]), frob=int(g[5]), ratio=g[6],
                wscan=float(g[9]), wfrob=float(g[10]), wratio=g[11],
            )
        )
assert len(rows) == 11, f"expected 11 configuration rows, read {len(rows)}"

labels = [f"m={r['m']}\n{r['bits']}b\n|D|={r['d']}" for r in rows]
x = range(len(rows))
w = 0.38
fig, (a1, a2) = plt.subplots(2, 1, figsize=(12, 8.5), sharex=True)
for ax, (ks, kf, kr, title, unit) in zip(
    (a1, a2),
    (
        ("scan", "frob", "ratio", "Root-step multiplications (counted, deterministic)", "fmul"),
        ("wscan", "wfrob", "wratio", "Wall time of all targets (L0, unisolated)", "seconds"),
    ),
):
    ax.bar([i - w / 2 for i in x], [r[ks] for r in rows], w, label="Scan (control)", color="#7a7a7a")
    ax.bar([i + w / 2 for i in x], [r[kf] for r in rows], w, label="Frobenius + domain index", color="#2f6fb0")
    ax.set_yscale("log")
    ax.set_ylabel(unit)
    ax.set_title(title, loc="left", fontsize=11)
    for i, r in enumerate(rows):
        top = max(r[ks], r[kf])
        ax.annotate(f"{r[kr]}×", (i, top), textcoords="offset points", xytext=(0, 3),
                    ha="center", fontsize=8)
    ax.grid(axis="y", alpha=0.3)
a1.legend(loc="upper left", fontsize=9)
a2.set_xticks(list(x))
a2.set_xticklabels(labels, fontsize=8)
fig.text(0.01, 0.005,
         "Label = scan ÷ Frobenius as printed by the example (>1: Frobenius cheaper). "
         "Every mode visited the identical solution sequence on every target.",
         fontsize=8)
fig.tight_layout(rect=(0, 0.02, 1, 1))
fig.savefig(HERE / "root_step.svg")
fig.savefig(HERE / "root_step.png", dpi=150)
print("wrote root_step.svg, root_step.png")
