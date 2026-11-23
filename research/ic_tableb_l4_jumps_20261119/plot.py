"""Summary, table and graph for L4-JUMPS (reads compare-*, bitop-*, band-*; run from this directory)."""
import json, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
def cmp(f):
    s = open(f).readline()
    v = [float(x) for x in re.search(r"= ([\d.]+) \[([\d.]+), ([\d.]+)\]", s).groups()]
    d = dict(ratio=v[0], lo=v[1], hi=v[2])
    m = re.search(r"w=0: ([\d.]+), w=0.1: ([\d.]+), w=1: ([\d.]+)\): ([^;]+);", s)
    if m: d.update(w0=float(m[1]), w01=float(m[2]), w1=float(m[3]), band=m[4].strip())
    return d
sizes = [20, 22, 24]
cands = ["tw2l-nj", "tw2l-sg-nj", "sib8-p256-j16", "tw2l-sg-sib"]
# compare --a X --b C prints C/X? check: ratio printed is b/a
summ = {}
for b in sizes:
    summ[b] = dict(vs_inc={c: cmp(f"compare-P{b}-sib8-p256-vs-{c}.txt") for c in cands},
                   vs_rho={c: cmp(f"compare-P{b}-rho-neg-vs-{c}.txt") for c in cands + ["sib8-p256"]},
                   bit_inc={c: cmp(f"bitop-P{b}-sib8-p256-vs-{c}.txt") for c in cands},
                   bit_rho={c: cmp(f"bitop-P{b}-rho-neg-vs-{c}.txt") for c in cands + ["sib8-p256"]})
json.dump(summ, open("summary.json", "w"), indent=1)
col = {"tw2l-nj": "#e45756", "tw2l-sg-nj": "#54a24b", "sib8-p256-j16": "#4c78a8", "tw2l-sg-sib": "#f2a541"}
fig, ax = plt.subplots(1, 3, figsize=(14, 4.2))
for i, (k, t) in enumerate((("w0", "w = 0"), ("w1", "w = 1"))):
    for c in cands:
        ax[i].plot(sizes, [summ[b]["vs_inc"][c][k] for b in sizes], "o-", color=col[c], label=c)
    ax[i].axhline(1, color="#222", ls="--", lw=1); ax[i].set_title(f"arm / sib8-p256, counted S, {t}", fontsize=10)
for c in cands:
    ax[2].plot(sizes, [summ[b]["bit_inc"][c]["ratio"] for b in sizes], "o-", color=col[c], label=c)
ax[2].axhline(1, color="#222", ls="--", lw=1); ax[2].set_title("arm / sib8-p256, bitop/v1", fontsize=10)
for a in ax: a.set_xticks(sizes); a.set_xlabel("Table B curve (field bits)"); a.grid(alpha=.3)
ax[0].legend(fontsize=8)
fig.suptitle("L4-JUMPS: jumps=16 ablation around the sib8-p256 incumbent, 48 targets x 2 rounds [pcc:unverified]", fontsize=9)
fig.tight_layout()
for e in ("svg", "png", "pdf"): fig.savefig(f"jumps.{e}", dpi=150)
print("ok")
