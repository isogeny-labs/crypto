"""Degree and matrix size per F4 round: first run (-t 1, wall stop) vs A1 run (-t 16, memory stop).
Run from this directory: python3 plot.py  (reads ../../runs and ../../runs-mt)."""
import re, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

SYS = "pkm_kummer_p2013265921_m3_t7_x221026520"
ROW = re.compile(r"^\s*(\d+)\s+(\d+)\s+(\d+)\s+(\d+) x (\d+)\s+[\d.]+%\s+(\d+) new\s+(\d+) zero\s+([\d.]+) \|")

def rounds(path):
    out, t = [], 0.0
    for line in open(path):
        m = ROW.match(line)
        if m:
            d, sel, pairs, r, c, new, zero, real = m.groups()
            t += float(real)
            out.append(dict(deg=int(d), rows=int(r), cols=int(c), new=int(new), t=t))
    return out

one = rounds(f"../../runs/{SYS}.log")
mt = rounds(f"../../runs-mt/{SYS}.log")
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
for rs, lab, mk in ((one, "first run, -t 1 (wall stop)", "o"), (mt, "A1 run, -t 16 (memory stop)", "s")):
    ax[0].step([r["t"] / 3600 for r in rs], [r["deg"] for r in rs], where="post", marker=mk, ms=3, label=lab)
    ax[1].plot(range(1, len(rs) + 1), [r["cols"] / 1e3 for r in rs], marker=mk, ms=3, label=lab)
ax[0].axhline(8, ls=":", c="grey"); ax[0].text(0.02, 8.08, "degree 8 selected (398,409 pairs), then exit 139 at 104 GB", fontsize=8)
ax[0].set_xlabel("cumulative F4 round time, real (h)"); ax[0].set_ylabel("round degree"); ax[0].set_ylim(3.5, 8.6)
ax[1].set_xlabel("F4 round"); ax[1].set_ylabel("matrix columns (thousands)"); ax[1].set_yscale("log")
for a in ax: a.grid(alpha=.3)
ax[0].legend(loc="lower right", fontsize=8)
fig.suptitle(f"{SYS} (N = 21): msolve 0.6.5 F4 rounds", fontsize=10)
fig.tight_layout()
fig.savefig("msolve-mt-degree.svg"); fig.savefig("msolve-mt-degree.png", dpi=150)
print(len(one), "rounds (-t 1);", len(mt), "rounds (-t 16); last -t 16 deg-7 round cols", [r["cols"] for r in mt if r["deg"] == 7][-1])
