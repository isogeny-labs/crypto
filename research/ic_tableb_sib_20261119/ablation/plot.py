"""Ablation summary and graph (reads compare-*.txt, bitop-*.txt)."""
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
pairs = ["tw2l-vs-tw2l-sib", "tw2l-vs-tw2l-sg", "tw2l-sib-vs-sib8-p256", "rho-neg-vs-tw2l-sib", "rho-neg-vs-sib8-p256"]
summ = {b: dict(gae={p: cmp(f"compare-P{b}-{p}.txt") for p in pairs}, bitop={p: cmp(f"bitop-P{b}-{p}.txt") for p in pairs}) for b in sizes}
json.dump(summ, open("summary.json", "w"), indent=1)
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
lab = {"tw2l-vs-tw2l-sib": ("siblings only (tw2l-sib / tw2l)", "#4c78a8"),
       "tw2l-vs-tw2l-sg": ("stride-generator only (tw2l-sg / tw2l)", "#54a24b"),
       "tw2l-sib-vs-sib8-p256": ("rest of sib8-p256 (sib8-p256 / tw2l-sib)", "#f2a541")}
for i, w in enumerate(("w0", "w1")):
    for p, (l, c) in lab.items():
        ax[i].plot(sizes, [summ[b]["gae"][p][w] for b in sizes], "o-", color=c, label=l)
    ax[i].axhline(1, color="#222222", lw=1, ls="--")
    ax[i].set_title(f"Paired counted-S ratio, {'w = 0' if w == 'w0' else 'w = 1'}", fontsize=10)
    ax[i].set_xticks(sizes); ax[i].set_xlabel("Table B curve (field bits)"); ax[i].grid(alpha=.3); ax[i].set_ylim(0.4, 1.2)
ax[0].legend(fontsize=8)
fig.suptitle("L3-SIB ablation on Table B: which change carries the gain (48 targets x 2 rounds, all verified) [pcc:unverified]", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(f"ablation.{ext}", dpi=150)
print("ok")
