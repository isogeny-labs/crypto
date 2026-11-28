"""Summary and graph for research/ic_tableb_sib_20261119 (reads compare-*, bitop-*, band-*)."""
import json, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

def cmp(f):
    s = open(f).readline()
    v = [float(x) for x in re.search(r"= ([\d.]+) \[([\d.]+), ([\d.]+)\]", s).groups()]
    d = dict(ratio=v[0], lo=v[1], hi=v[2])
    m = re.search(r"w=0: ([\d.]+), w=0.1: ([\d.]+), w=1: ([\d.]+)\): ([^;]+);", s)
    if m:
        d.update(w0=float(m[1]), w01=float(m[2]), w1=float(m[3]), band=m[4].strip())
    e = re.search(r"end-to-end wall ([\d.]+)", s)
    if e:
        d["e2e_wall"] = float(e[1])
    return d

def band(f):
    out = {}
    for line in open(f):
        c = [x.strip() for x in line.split("|")]
        if len(c) > 8 and c[3] and not c[3].startswith(("arm", ":")):
            out[c[3]] = dict(S0=float(c[5]), S01=float(c[6]), S1=float(c[7]))
    return out

sizes = [20, 22, 24]
pairs = ["tw2l-vs-sib8-p256", "tw2l-vs-sib8-p512", "tw2l-vs-tw2l-aa", "rho-neg-vs-sib8-p256", "rho-neg-vs-sib8-p512", "rho-neg-vs-tw2l"]
summ = {b: dict(gae={p: cmp(f"compare-P{b}-{p}.txt") for p in pairs},
                bitop={p: cmp(f"bitop-P{b}-{p}.txt") for p in pairs},
                S=band(f"band-P{b}.txt")) for b in sizes}
json.dump(summ, open("summary.json", "w"), indent=1)

fig, ax = plt.subplots(1, 3, figsize=(14, 4.2))
arms = [("tw2l", "#e45756"), ("sib8-p512", "#f2a541"), ("sib8-p256", "#4c78a8"), ("rho-neg", "#222222")]
for i, (w, t) in enumerate((("S0", "w = 0"), ("S1", "w = 1 (table work and second points priced)"))):
    for arm, col in arms:
        ax[i].plot(sizes, [summ[b]["S"][arm][w] for b in sizes], "o-", color=col, label=arm)
    ax[i].set_title(f"Counted S, {t}", fontsize=10); ax[i].set_ylim(bottom=0)
for arm, col in arms[1:3]:
    ax[2].errorbar(sizes, [summ[b]["bitop"][f"rho-neg-vs-{arm}"]["ratio"] for b in sizes],
                   yerr=[[summ[b]["bitop"][f"rho-neg-vs-{arm}"]["ratio"] - summ[b]["bitop"][f"rho-neg-vs-{arm}"]["lo"] for b in sizes],
                         [summ[b]["bitop"][f"rho-neg-vs-{arm}"]["hi"] - summ[b]["bitop"][f"rho-neg-vs-{arm}"]["ratio"] for b in sizes]],
                   fmt="o-", color=col, capsize=3, label=arm)
ax[2].axhline(1, color="#222222", lw=1, ls="--", label="rho-neg")
ax[2].set_title("bitop/v1: arm / rho-neg [95%]", fontsize=10)
for a in ax:
    a.set_xticks(sizes); a.set_xlabel("Table B curve (field bits)"); a.grid(alpha=.3)
ax[0].set_ylabel("S = GAE / sqrt(r)"); ax[0].legend(fontsize=8); ax[2].legend(fontsize=8)
fig.suptitle("Table B round 3: sibling-pool wwalk vs the round-2 incumbent tw2l; 48 targets x 2 rounds, all verified [pcc:unverified]", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(f"tableb_sib.{ext}", dpi=150)
print("ok")
