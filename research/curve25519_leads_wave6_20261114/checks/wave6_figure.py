#!/usr/bin/env python3
"""Two panels: (left) cyclotomic-torus census of Curve25519: largest subgroup order inside the degree-k torus whose prime
factors are all <= B, for B = 2^8, 2^12, 2^16, 2^20 (from checks/cyclotomic_census.out.txt); (right) the out-of-sample
solving-degree cells (sdrule/cells) against the two candidate rules. Unverified."""
import glob, json, os, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
H = os.path.dirname(os.path.abspath(__file__)); W = os.path.join(H, "..")
rows = []
for ln in open(os.path.join(H, "cyclotomic_census.out.txt")):
    m = re.match(r"\s*(\d+) \|", ln)
    if m:
        vals = json.loads(ln.rsplit("|", 1)[1]); rows.append((int(m.group(1)), vals))
fig, (a, b) = plt.subplots(1, 2, figsize=(11.5, 4.6))
cols = ["#9cc3e0", "#5a9bd0", "#1b6ca8", "#0b3d66"]; labels = ["e_max <= 2^8", "<= 2^12", "<= 2^16", "<= 2^20"]
wdt = 0.2
for i in range(4):
    a.bar([r[0] + (i - 1.5) * wdt for r in rows], [float(max(r[1][i], 1)) for r in rows], width=wdt, color=cols[i], label=labels[i])
a.axhline(2.0**64, color="k", ls=":", lw=0.9); a.text(0.6, 2.0**64 * 1.5, "useful |V| for m = 4: 2^64", fontsize=7.5)
a.axhline(2.0**85, color="k", ls="--", lw=0.9); a.text(0.6, 2.0**85 * 1.5, "useful |V| for m = 3: 2^85", fontsize=7.5)
a.set_yscale("log"); a.set_ylim(1, 2.0**95); a.set_xticks([r[0] for r in rows])
a.set_xlabel("torus degree k (subgroups of F_{p^k}^* of order dividing Phi_k(p), p = 2^255 - 19)")
a.set_ylabel("largest |V| with all prime factors <= B"); a.legend(fontsize=7.5, loc="upper right")
a.set_title("Curve25519: no cyclotomic-torus base is both large and low-degree", fontsize=9.5)
a.grid(True, axis="y", alpha=.25)
# right panel
def factors(n):
    out, f = [], 2
    while n > 1:
        while n % f == 0: out.append(f); n //= f
        f += 1
    return out
import math
pts = []
for f in sorted(glob.glob(os.path.join(W, "sdrule", "cells", "*.json"))):
    j = json.load(open(f))
    if j.get("capped") or not j.get("per_target"): continue
    d = j["d_or_t"]; fs = factors(d); emax = max(fs); s = sum(e - 1 for e in fs) - (emax - 1)
    sd = max(r["solving_degree_max"] for r in j["per_target"])
    pts.append((d, sd, emax + 1 + math.ceil(s / 4), (emax + 2, emax + 3)))
x = range(len(pts))
b.plot(x, [p[1] for p in pts], "ko", ms=8, label="recorded SD (k = 22, m = 2)", zorder=4)
b.plot(x, [p[2] for p in pts], "s", color="#c0501a", mfc="none", ms=11, label="W6 chain rule")
for i, p in enumerate(pts):
    b.plot([i, i], list(p[3]), "-", color="#1b6ca8", lw=6, alpha=.45, label="W5 band e_max + 2..3" if i == 0 else None)
b.set_xticks(list(x)); b.set_xticklabels([f"d = {p[0]}\n{'x'.join(map(str, factors(p[0])))}" for p in pts])
b.set_ylabel("solving degree"); b.set_title("Out-of-sample test: each rule misses one cell", fontsize=9.5)
b.legend(fontsize=7.5, loc="upper left"); b.grid(True, alpha=.25)
fig.tight_layout(); fig.savefig(os.path.join(W, "wave6.svg")); fig.savefig(os.path.join(W, "wave6.png"), dpi=150)
print("ok", len(rows), "tori,", len(pts), "sdrule cells")
