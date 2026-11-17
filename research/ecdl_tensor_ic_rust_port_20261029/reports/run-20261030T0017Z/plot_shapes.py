"""Plot the recorded ecdlscale tables (Go and Rust). Reads the two text files
beside this script; computes nothing beyond parsing them."""
import sys, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = pathlib.Path(sys.argv[1])
def load(name):
    rows = []
    for line in (here / name).read_text().splitlines()[1:]:
        a, b, c = line.split("|")
        bb = int(a.split()[0]); mul, add, pt = map(float, b.split()); md, ratio = c.split()
        rows.append((bb, mul, add, pt, int(md), float(ratio)))
    return rows
go, rs = load("ecdlscale-go.txt"), load("ecdlscale-rust.txt")
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(10, 4))
for idx, label, color in [(1, "mul", "#2a6fdb"), (2, "add", "#d9534f"), (3, "point", "#3c9d5d")]:
    ax1.plot([r[0] for r in go], [r[idx] for r in go], "-", color=color, lw=2, label=f"{label} (Go)")
    ax1.plot([r[0] for r in rs], [r[idx] for r in rs], "x", color="black", ms=8)
ax1.plot([], [], "x", color="black", label="Rust (all series)")
ax1.set_xscale("log", base=2); ax1.set_xlabel("field size b (bits)"); ax1.set_ylabel("edges per bit")
ax1.set_title("Gadget edges / b"); ax1.legend(fontsize=8); ax1.grid(alpha=.3)
ax2.plot([r[0] for r in go], [r[5] for r in go], "-o", color="#7b4fb0", label="Go")
ax2.plot([r[0] for r in rs], [r[5] for r in rs], "x", color="black", ms=8, label="Rust")
ax2.set_xscale("log", base=2); ax2.set_ylim(0, 25); ax2.set_xlabel("field size b (bits)")
ax2.set_ylabel("max degree / b^0.75"); ax2.set_title("Max vertex degree / b^(3/4)"); ax2.legend(fontsize=8); ax2.grid(alpha=.3)
fig.suptitle("ecdlscale structure, Go 1c29f66 vs Rust crates/ecdl-tensor-ic (run-20261030T0017Z; deterministic, n = 1 per b)", fontsize=9)
fig.tight_layout()
fig.savefig(here / "gadget-shapes.svg"); fig.savefig(here / "gadget-shapes.png", dpi=150)
