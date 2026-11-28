"""Summary and graph for research/ic_tableb_binary_sg_20261120 (reads compare-*.txt, band-*.txt)."""
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
def band(f):
    out = {}
    for line in open(f):
        c = [x.strip() for x in line.split("|")]
        if len(c) > 8 and c[3] and not c[3].startswith(("arm", ":")):
            out[c[3]] = dict(S0=float(c[5]), S01=float(c[6]), S1=float(c[7]))
    return out
sizes = [15, 18, 21, 24, 27]
pairs = ["ww-tw-vs-ww-tw-sg", "ww-tw-vs-ww-sg", "rho-neg-vs-ww-tw-sg", "rho-neg-vs-ww-tw", "bsgs-neg-vs-ww-tw-sg"]
summ = {b: dict(cmp={p: cmp(f"compare-B{b}-{p}.txt") for p in pairs}, S=band(f"band-B{b}.txt")) for b in sizes}
json.dump(summ, open("summary.json", "w"), indent=1)
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
for i, w in enumerate(("w0", "w1")):
    for p, lab, col in (("ww-tw-vs-ww-tw-sg", "ww-tw-sg / ww-tw", "#4c78a8"), ("ww-tw-vs-ww-sg", "ww-sg / ww-tw", "#f2a541"),
                        ("rho-neg-vs-ww-tw-sg", "ww-tw-sg / rho-neg", "#222222"), ("rho-neg-vs-ww-tw", "ww-tw / rho-neg", "#888888")):
        ax[i].plot(sizes, [summ[b]["cmp"][p][w] for b in sizes], "o-", color=col, label=lab)
    ax[i].axhline(1, color="#999999", lw=1, ls="--")
    ax[i].set_xticks(sizes); ax[i].set_xlabel("binary Table B curve n"); ax[i].grid(alpha=.3)
    ax[i].set_title(f"Paired counted-S ratio, {'w = 0' if w == 'w0' else 'w = 1'}", fontsize=10)
ax[0].legend(fontsize=8)
fig.suptitle("Binary Table B: stride-generator targets on the wwalk incumbent; 48 targets x 2 rounds, all verified [pcc:unverified]", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(f"binary_sg.{ext}", dpi=150)
print("ok")
