"""Round-4 graphs (Addendum H) from the recorded summary files (layout only:
values are parsed from the tools' own summary lines; nothing is recomputed).

Usage: python plot4.py <results-dir> <out-dir>
"""

import re
import sys
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt

res, out = Path(sys.argv[1]), Path(sys.argv[2])
plt.rcParams.update({"font.size": 8, "svg.fonttype": "none"})


def run(prefix):
    return sorted(d for d in res.iterdir() if d.name.startswith(prefix))[-1]


def save(fig, name):
    fig.tight_layout()
    fig.savefig(out / f"{name}.svg")
    fig.savefig(out / f"{name}.png", dpi=150)
    plt.close(fig)


# H1: trials/p against h
H1 = run("H1-hsweep-run")
txt = (H1 / "hsweep_summary.txt").read_text()
cell = re.compile(r"H1 bits (\d+) h (\d+) (\w+): trials/p ([\d.]+) \(s.e. over \d+ instances ([\d.]+)\)(?: \| model ([\d.]+))?")
alpha = re.search(r"H1 alpha fit J: alpha ([\d.]+) \(bootstrap 95% \[([\d.]+), ([\d.]+)\]", txt)
fig, ax = plt.subplots(figsize=(7.5, 3.6))
style = {"32": ("o", 0.0), "36": ("s", 0.03), "40": ("^", 0.06)}
for m in cell.finditer(txt):
    bits, h, arm, v, se, model = m.groups()
    mk, off = style[bits]
    x = int(h) * (1 + off)
    if arm == "J":
        ax.errorbar(x, float(v), yerr=float(se), fmt=mk, color="#4a6fa5", ms=3, label=f"Arm J, {bits} bits" if h == "4" else None)
        if model:
            ax.plot(x, float(model), mk, mfc="none", color="#4a6fa5", ms=4, alpha=0.6, label=f"model M-H1, {bits} bits" if h == "4" else None)
    elif arm == "R":
        ax.plot(x, float(v), mk, color="#333333", ms=3, label=f"rho, {bits} bits" if h == "4" else None)
hs = [4, 8, 12, 16, 24, 32, 48, 64, 96, 128]
ax.plot(hs, [0.886 / h**0.5 for h in hs], ":", color="#333333", label="rho theory 0.886/√h")
ax.set_xscale("log")
ax.set_yscale("log")
ax.set_xlabel("cofactor h  (#E = h·ℓ)")
ax.set_ylabel("group operations / p")
ax.set_title(f"H1: Arm J trials scale as h^-{alpha.group(1)} (95% [{alpha.group(2)}, {alpha.group(3)}]) vs rho h^-1/2; run {H1.name}", fontsize=7.5)
ax.legend(fontsize=6, ncol=2)
save(fig, "h1_cofactor_law")

# H2: glued instances, steps/p and the pricing band
H2 = run("H2-glue-run")
txt = (H2 / "glue_summary.txt").read_text()
st = re.compile(r"H2 bits (\d+) h (\d+) (\w+): steps/p ([\d.]+) \(s.e. ([\d.]+)")
band = re.compile(r"H2 band bits (\d+) h (\d+): Jglue/rho in \[([\d.]+), ([\d.]+)\]")
fig, axs = plt.subplots(1, 2, figsize=(9, 3.3))
for m in st.finditer(txt):
    bits, h, arm, v, se = m.groups()
    off = {"Jglue": 0.3, "R": -0.3}[arm] + (0.0 if h == "1" else 0.12)
    col = {"Jglue": "#c0392b", "R": "#333333"}[arm]
    mk = "o" if h == "1" else "s"
    axs[0].errorbar(int(bits) + off, float(v), yerr=float(se), fmt=mk, color=col, ms=3,
                    label=f"{'Arm J (glued)' if arm == 'Jglue' else 'rho'}, h = {h}" if bits == "28" else None)
for h, y in [(1, 0.886), (4, 0.443)]:
    axs[0].axhline(y, ls=":", color="#333333", lw=0.7)
axs[0].set_xlabel("log2 #E(F_{p²})")
axs[0].set_ylabel("group operations / p")
axs[0].set_title("H2: prime-order E (h = 1) and h = 4, general gluing")
axs[0].legend(fontsize=6)
for m in band.finditer(txt):
    bits, h, lo, hi = m.groups()
    x = int(bits) + (-0.2 if h == "1" else 0.2)
    axs[1].vlines(x, float(lo), float(hi), color="#c0392b" if h == "1" else "#4a6fa5", lw=3,
                  label=f"h = {h}" if bits == "28" else None)
axs[1].axhline(1, color="k", lw=0.7)
axs[1].set_xlabel("log2 #E(F_{p²})")
axs[1].set_ylabel("Arm J cost / rho cost (band, 12 cells)")
axs[1].set_title("H2 pricing band (Legendre priced at one I)")
axs[1].legend(fontsize=6)
fig.suptitle(f"run {H2.name}", fontsize=7, y=0.02)
save(fig, "h2_glue")

# H4: memory against time
H4 = run("H4-memsweep-run")
txt = (H4 / "memsweep_summary.txt").read_text()
mm = re.compile(r"H4 bits (\d+) theta ([\d.]+): trials/p ([\d.]+) stored/p ([\d.]+)")
fig, ax = plt.subplots(figsize=(6, 3.2))
for bits, mk in [("32", "o-"), ("36", "s-")]:
    pts = sorted((float(t), float(a), float(b)) for bb, t, a, b in mm.findall(txt) if bb == bits)
    ax.plot([p[2] for p in pts], [p[1] for p in pts], mk, label=f"{bits} bits, θ = ⅛ … ½", ms=3)
ax.set_xscale("log")
ax.set_xlabel("stored relations / p (memory)")
ax.set_ylabel("trials / p (time)")
ax.set_title(f"H4: time barely rises as memory falls (rho takes over); run {H4.name}", fontsize=7.5)
ax.legend(fontsize=6.5)
save(fig, "h4_memory_time")

# H7: coverage gain on the ℓ-subgroup
H7 = run("H7-subgroup-run")
txt = (H7 / "checks.txt").read_text()
g7 = re.compile(r"H7 (\w+) n=(\d): ell-subgroup coverage ratio subfield/null mean ([\d.]+) \(s.e. ([\d.]+), (\d+) curves\)")
fig, ax = plt.subplots(figsize=(7.5, 3.0))
rows = g7.findall(txt)
ax.bar(range(len(rows)), [float(r[2]) for r in rows], yerr=[1.96 * float(r[3]) for r in rows], color="#4a6fa5", capsize=2)
ax.axhline(1, color="k", lw=0.7)
ax.set_xticks(range(len(rows)), [f"{r[0]} n={r[1]}" for r in rows], rotation=30, ha="right", fontsize=6.5)
ax.set_ylabel("coverage of ℓ-subgroup: subfield / null")
ax.set_title(f"H7: coset confinement raises relation yield on the target subgroup; run {H7.name}", fontsize=7.5)
save(fig, "h7_subgroup")
print("wrote round-4 graphs")
