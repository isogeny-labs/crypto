"""Summary and graph for research/ic_tableb_binary_wwalk_20261119 (reads compare-*.txt, band-*.txt)."""
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
summ = {}
for b in sizes:
    inc = "ksum4" if b <= 18 else "ksum5"
    summ[b] = dict(incumbent=inc,
        ww_over_inc=cmp(f"compare-B{b}-{inc}-vs-ww.txt"), wwtw_over_inc=cmp(f"compare-B{b}-{inc}-vs-ww-tw.txt"),
        wwtw_over_ww=cmp(f"compare-B{b}-ww-vs-ww-tw.txt"), ww_over_rho=cmp(f"compare-B{b}-rho-neg-vs-ww.txt"),
        wwtw_over_rho=cmp(f"compare-B{b}-rho-neg-vs-ww-tw.txt"), wwtw_over_bsgs=cmp(f"compare-B{b}-bsgs-neg-vs-ww-tw.txt"),
        S=band(f"band-B{b}.txt"))
json.dump(summ, open("summary.json", "w"), indent=1)
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
for i, w in enumerate(("S0", "S1")):
    for arm, col in (("incumbent", "#888888"), ("ww", "#f2a541"), ("ww-tw", "#4c78a8"), ("rho-neg", "#222222"), ("bsgs-neg", "#54a24b")):
        ys = [summ[b]["S"][summ[b]["incumbent"] if arm == "incumbent" else arm][w] for b in sizes]
        ax[i].plot(sizes, ys, "o-", color=col, label=("k-sum incumbent" if arm == "incumbent" else arm))
    ax[i].set_yscale("log"); ax[i].set_xticks(sizes); ax[i].set_xlabel("binary Table B curve n (GF(2^n))"); ax[i].grid(alpha=.3, which="both")
    ax[i].set_title(f"Counted S, w = {'0' if w == 'S0' else '1'}", fontsize=10)
ax[0].set_ylabel("S = GAE / sqrt(r) (log)"); ax[0].legend(fontsize=8)
fig.suptitle("Random-binary Table B: C = 2 wwalk (ported) vs the k-sum incumbent; 48 targets x 2 rounds, all verified [pcc:unverified]", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png", "pdf"): fig.savefig(f"binary_wwalk.{ext}", dpi=150)
for b in sizes:
    s = summ[b]; print(b, s["incumbent"], round(s["wwtw_over_inc"]["ratio"], 3), round(s["wwtw_over_rho"]["ratio"], 2), {k: (v["S0"], v["S1"]) for k, v in s["S"].items()})
