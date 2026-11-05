import csv
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

here = __file__.rsplit("/", 1)[0]
rows = list(csv.DictReader(open(f"{here}/L3_tradeoff_line.csv")))
fig, ax = plt.subplots(figsize=(7.5, 5.0))
styles = {"P-256": "C0", "Curve25519": "C1", "ECCp-131": "C2"}
for curve, color in styles.items():
    pts = [r for r in rows if r["curve"] == curve]
    xs = [float(r["table_exp_t_log2_entries"]) for r in pts]
    ys = [float(r["online_exp_log2_T"]) for r in pts]
    ax.plot(xs, ys, "-", color=color, label=curve)
    ax.plot(xs[0], ys[0], "o", color=color)
    ax.plot(xs[-1], ys[-1], "s", color=color)
ax.set_xlabel("table size, log2 of stored entries (derived from CGK line)")
ax.set_ylabel("online cost, log2 of group operations per target")
ax.set_title("L3: CGK line S*T^2 = l, rho (circle) and Bernstein-Lange (square) endpoints")
ax.legend()
ax.grid(alpha=0.3)
fig.tight_layout()
fig.savefig(f"{here}/L3_tradeoff.svg")
