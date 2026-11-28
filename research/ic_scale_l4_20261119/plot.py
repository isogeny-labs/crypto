"""Graph and summary.json for L4-SCALE (run from this directory)."""
import json, re, math
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
pairs = ["rho-neg-vs-tw2l", "rho-neg-vs-sib8-p256", "tw2l-vs-sib8-p256", "sib8-p256-vs-sib8-p256-aa"]
summ = {b: dict(gae={p: cmp(f"compare-P{b}-{p}.txt") for p in pairs}, bitop={p: cmp(f"bitop-P{b}-{p}.txt") for p in pairs}) for b in (28, 32)}
json.dump(summ, open("summary.json", "w"), indent=1)
T = json.load(open("trend.json"))
fig, ax = plt.subplots(1, 2, figsize=(11, 4.2))
a = ax[0]
col = {"tableB-round3": "#1f77b4", "tableB-ablation": "#2ca02c", "L4-28": "#d62728", "L4-32": "#d62728"}
lab = set()
for p in T["points"]:
    g = "L4 (this study)" if p["group"].startswith("L4") else p["group"]
    a.errorbar(p["x"] + (0.15 if "ablation" in p["group"] else 0), p["r"], yerr=[[p["r"] - p["lo"]], [p["hi"] - p["r"]]], fmt="o", color=col[p["group"]], capsize=3, label=None if g in lab else g)
    lab.add(g)
xs = [19.5 + i * 0.25 for i in range(52)]
for k, st in (("tableB_only(20-24)", "--"), ("all(20-32)", "-")):
    f = T[k]; a.plot(xs, [math.exp(f["intercept"] + f["slope"] * x) for x in xs], st, color="grey", lw=1, label=f"{k}: slope {f['slope']:.3f}/bit")
a.axhline(1, color="k", lw=0.8); a.set_xlabel("log2 r"); a.set_ylabel("sib8-p256 / rho-neg (GAE, w = 0)"); a.legend(fontsize=7); a.set_title("Per-curve ratio vs size [pcc:unverified]")
b = ax[1]
ws = ["w0", "w01", "w1"]; names = ["w = 0", "w = 0.1", "w = 1"]
for i, s in enumerate((28, 32)):
    y = [summ[s]["gae"]["rho-neg-vs-sib8-p256"][w] for w in ws]
    b.bar([j + i * 0.38 for j in range(3)], y, 0.36, label=f"{s} bits")
b.axhline(1, color="k", lw=0.8); b.set_xticks([j + 0.19 for j in range(3)]); b.set_xticklabels(names); b.set_ylabel("sib8-p256 / rho-neg"); b.legend(); b.set_title("Across the weight band (counted GAE)")
plt.tight_layout()
for e in ("svg", "png", "pdf"): plt.savefig(f"scale_l4.{e}", dpi=140)
