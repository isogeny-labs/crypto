#!/usr/bin/env python3
"""Layout only: figure and PDF for RESULT.md (numbers copied from koblitz_step.log / profile_n83.txt)."""
import os, re, typst
import matplotlib; matplotlib.use("svg")
import matplotlib.pyplot as plt
plt.rcParams.update({"svg.fonttype": "none", "font.size": 9})
H = os.path.dirname(os.path.abspath(__file__))
fig, ax = plt.subplots(figsize=(6, 3.2))
sizes = [1, 6474]; ax.bar(["level 0\n(crater, f=1)", "level 1\n(conductor 6473)"], sizes, color=["#1f6fb2", "#c4561a"])
for i, s in enumerate(sizes): ax.text(i, s, f"{s}", ha="center", va="bottom")
ax.set_ylabel("curves at level (h of the order)"); ax.set_title("n=83 Koblitz, ℓ=6473: 6474 level-1 j found in F(2^83)")
fig.tight_layout(); fig.savefig(f"{H}/fig_levels.svg")
def esc(s): return re.sub(r"([#$\[\]*_@<>`\\])", r"\\\1", s)
t = '#set page(margin: 1.6cm)\n#set text(size: 9.5pt)\n'
for l in open(f"{H}/RESULT.md").read().splitlines():
    t += (f"= {esc(l[2:])}\n" if l.startswith("# ") else esc(l) + "\n\n" if l else "")
t += '#figure(image("fig_levels.svg", width: 80%), caption: [Level sizes found.])\n'
open(f"{H}/RESULT.typ", "w").write(t); typst.compile(f"{H}/RESULT.typ", output=f"{H}/RESULT.pdf")
