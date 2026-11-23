"""Lays out recorded values only: the saved ecbench comparisons and the ecbench
tables of the five sessions. Computes no result."""
import json, re, pathlib
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
SESS = ["a0-n13", "a1-n17", "a0-n23", "a1-n29", "a0-n37"]

def table(sess):
    out = {}
    for line in (ROOT / "tables" / f"table-{sess}.md").read_text().splitlines():
        c = [x.strip() for x in line.split("|")]
        if len(c) > 8 and c[1].startswith("ECBS"):
            lo, hi = re.findall(r"[\d.]+", c[8])
            out[c[2]] = (float(c[7]), float(lo), float(hi), float(c[5]) if False else None, c[6])
    return out

def cmp(sess, a, b):
    d = json.loads((ROOT / "sessions" / sess / "comparisons" / f"{a}__{b}.json").read_text())["ops"]
    return d["ratio_b_over_a"], d["ci95"][0], d["ci95"][1], d["pairs"]

rows = []
for s in SESS:
    t = table(s)
    base = next(k for k in t if re.fullmatch(r"ic-lazy-c\d+", k) and k != "ic-lazy-c2")
    n = int(s.split("-n")[1])
    rows.append(dict(s=s, n=n, a=s[1], base=base, c0=int(base.split("c")[-1]), t=t,
                     ratio=cmp(s, "ic-lazy-c2", base)))
fig, ax = plt.subplots(figsize=(9, 4.4))
xs = range(len(rows))
ax.errorbar(xs, [r["ratio"][0] for r in rows],
            yerr=[[r["ratio"][0]-r["ratio"][1] for r in rows], [r["ratio"][2]-r["ratio"][0] for r in rows]],
            fmt="o", color="#0f766e", capsize=4, ms=7)
ax.axhline(1, color="k", lw=0.8, ls="--"); ax.set_yscale("log")
ax.set_xticks(list(xs)); ax.set_xticklabels([f"n={r['n']}, a={r['a']}\nfrozen c={r['c0']}" for r in rows], fontsize=8)
ax.set_ylabel("S(frozen c) / S(c = 2)  (>1: two columns cheaper)")
ax.set_title("Two orbit columns vs the frozen column count, lazy oracle: paired cost ratio\n"
             "ecbench.gae, paired by target, cluster-bootstrap 95 % CI; 96 pairs (n37: 48)", fontsize=10)
ax.grid(alpha=0.3, which="both")
fig.tight_layout(); fig.savefig(HERE/"fig1-ratio.svg"); fig.savefig(HERE/"fig1-ratio.png", dpi=150); plt.close(fig)

fig, ax = plt.subplots(figsize=(9, 4.6))
w = 0.27
for k, (key, lab, col) in enumerate((("base", "IC lazy, frozen c", "#6b7280"), ("c2", "IC lazy, c = 2", "#0f766e"), ("rho", "rho-strong", "#b45309"))):
    ys, lo, hi = [], [], []
    for r in rows:
        arm = {"base": r["base"], "c2": "ic-lazy-c2", "rho": "rho-strong"}[key]
        m, l, h = r["t"][arm][:3]; ys.append(m); lo.append(m-l); hi.append(h-m)
    ax.bar([i + (k-1)*w for i in xs], ys, w, yerr=[lo, hi], capsize=3, color=col, label=lab)
ax.set_yscale("log"); ax.set_xticks(list(xs))
ax.set_xticklabels([f"n={r['n']}, a={r['a']}" for r in rows], fontsize=8)
ax.set_ylabel("mean S (GAE per sqrt r)"); ax.grid(alpha=0.3, axis="y", which="both"); ax.legend(fontsize=8)
ax.set_title("Mean S per arm, fully priced, 95 % interval from ecbench table", fontsize=10)
fig.tight_layout(); fig.savefig(HERE/"fig2-S.svg"); fig.savefig(HERE/"fig2-S.png", dpi=150); plt.close(fig)
print("ok")
