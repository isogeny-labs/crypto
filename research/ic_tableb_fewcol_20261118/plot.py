"""Summary and graph for research/ic_tableb_fewcol_20261118 (reads compare-*.txt and band-*.txt)."""
import json, re
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

def cmp(f):
    s = open(f).readline()
    m = re.search(r"= ([\d.]+) \[([\d.]+), ([\d.]+)\].*w=0: ([\d.]+), w=0.1: ([\d.]+), w=1: ([\d.]+)", s)
    if not m:
        m2 = re.search(r"= ([\d.]+) \[([\d.]+), ([\d.]+)\]", s)
        v = [float(x) for x in m2.groups()]
        return dict(ratio=v[0], lo=v[1], hi=v[2], w0=v[0], w01=v[0], w1=v[0])
    v = [float(x) for x in m.groups()]
    return dict(ratio=v[0], lo=v[1], hi=v[2], w0=v[3], w01=v[4], w1=v[5])

def band(f):
    out = {}
    for line in open(f):
        c = [x.strip() for x in line.split("|")]
        if len(c) > 8 and c[3] and not c[3].startswith(("arm", ":")):
            out[c[3]] = dict(S0=float(c[5]), S01=float(c[6]), S1=float(c[7]))
    return out

sizes = [20, 22, 24]
summ = {}
for b in sizes:
    summ[b] = dict(
        stack=cmp(f"compare-P{b}-ksum5-sib-vs-ksum5-ps.txt"),
        fewcol=cmp(f"compare-P{b}-ksum5-sib-vs-tw2l.txt"),
        aa=cmp(f"compare-P{b}-tw2l-vs-tw2l-aa.txt"),
        tw2l_over_rho=cmp(f"compare-P{b}-rho-neg-vs-tw2l.txt"),
        ps_over_rho=cmp(f"compare-P{b}-rho-neg-vs-ksum5-ps.txt"),
        S=band(f"band-P{b}.txt"),
    )
json.dump(summ, open("summary.json", "w"), indent=1)

fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
arms = [("ksum5-sib", "#888888"), ("ksum5-ps", "#4c78a8"), ("tw2l", "#e45756"), ("rho-neg", "#222222")]
for w, a in (("S0", ax[0]), ("S1", ax[1])):
    for arm, col in arms:
        a.plot(sizes, [summ[b]["S"][arm][w] for b in sizes], "o-", color=col, label=arm)
    a.set_xticks(sizes); a.set_xlabel("Table B curve (field bits)")
    a.set_ylabel(f"S = GAE / sqrt(r), w = {'0' if w == 'S0' else '1'}")
    a.set_ylim(bottom=0); a.grid(alpha=.3)
ax[0].set_title("Counted cost at w = 0"); ax[1].set_title("Counted cost at w = 1 (table work priced)")
ax[0].legend(loc="upper right", fontsize=8)
fig.suptitle("Table B: k-sum incumbent vs prune+sib (STACK) and C = 2 wwalk tw2l (FEWCOL); 48 targets x 2 rounds, all verified [pcc:unverified]", fontsize=9)
fig.tight_layout()
for ext in ("svg", "png", "pdf"):
    fig.savefig(f"tableb_fewcol.{ext}", dpi=150)
print(json.dumps({b: {k: summ[b][k]["ratio"] for k in ("stack", "fewcol", "aa", "tw2l_over_rho")} for b in sizes}))
