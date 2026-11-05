# Plots recorded values from ceiling_table.csv only; computes nothing.
import csv, sys
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = sys.argv[1]
rows = list(csv.DictReader(open(f"{d}/ceiling_table.csv")))

def series(m, base, col):
    xs, ys = [], []
    for r in rows:
        if int(r["m"]) == m and r["baseline"] == base:
            xs.append(int(r["n_bits"])); ys.append(float(r[col]))
    return xs, ys

fig, ax = plt.subplots(figsize=(7.2, 4.6), dpi=100)
for m, colour in ((3, "#1f4e79"), (4, "#b5562b")):
    x, y = series(m, "MITM", "mass_log2_inv_f_min")
    ax.plot(x, y, "-", color=colour, lw=2, label=f"m={m}: mass requirement, MITM base (f <= p^-1/{m})")
    x2, y2 = series(m, "MITM", "screen_log2_inv_f_max")
    ax.plot(x2, y2, ":", color=colour, lw=2, label=f"m={m}: screening budget (1/f <= p^(1/2-1/{m}))")
    if m == 3:
        ax.fill_between(x, y2, y, color="#999999", alpha=0.25, lw=0,
                        label="m=3: no f satisfies both")
    xe, ye = series(m, "ENUM", "mass_log2_inv_f_min")
    ax.plot(xe, ye, "--", color=colour, lw=1.2, alpha=0.8, label=f"m={m}: mass requirement, enumeration base (f <= p^-1/2)")
ax.axhline(1.0, color="#666666", lw=0.8)
ax.text(40, 2.0, "constant f = 1/2 (1 bit)", fontsize=8, color="#444444")
ax.set_xlabel("field size log2(p) (bits)")
ax.set_ylabel("required log2(1/f)  (bits)")
ax.set_title("Easy-target fraction needed to move the relation-phase exponent\n(source: ceiling_table.csv, derived by awk from the arithmetic in section 2 of the lead note)", fontsize=9)
ax.grid(alpha=0.3)
ax.legend(fontsize=7, loc="upper left", frameon=False)
fig.tight_layout()
fig.savefig(f"{d}/ceiling_graph.svg")
print("wrote", f"{d}/ceiling_graph.svg")
