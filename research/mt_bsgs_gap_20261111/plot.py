import json, collections, statistics as st, matplotlib
matplotlib.use("Agg"); import matplotlib.pyplot as plt
rows = json.load(open("translate_sim.json"))
by = collections.defaultdict(list)
for r in rows: by[r["K"]].append(r)
Ks = sorted(by); x = range(len(Ks))
fig, ax = plt.subplots(figsize=(7, 4))
series = [("random offsets", lambda r: r["E_random"]), ("greedy, R = 16", None), ("greedy, R = 256", None), ("tiled (BSGS)", lambda r: r["E_tiled"])]
vals = {"random offsets": [1.0 for K in Ks],
        "greedy, R = 16": [st.mean(r["E_greedy"] / r["E_random"] for r in by[K] if r["R"] == 16) for K in Ks],
        "greedy, R = 256": [st.mean(r["E_greedy"] / r["E_random"] for r in by[K] if r["R"] == 256) for K in Ks],
        "tiled (BSGS)": [st.mean(r["E_tiled"] / r["E_random"] for r in by[K]) for K in Ks]}
cols = ["#888888", "#e08a00", "#c0392b", "#2a9d55"]
for i, (name, c) in enumerate(zip(vals, cols)):
    ax.bar([j + (i - 1.5) * 0.2 for j in x], vals[name], 0.2, label=name, color=c)
ax.set_xticks(list(x)); ax.set_xticklabels([f"K = {K}\n(~{(1<<18)//K} trials)" for K in Ks])
ax.set_ylabel("expected trials per target / random offsets"); ax.axhline(0.5, color="k", lw=0.6, ls=":")
ax.set_title("Choosing trial offsets cannot recover BSGS's factor 2 (n = 2^18, 3 seeds)", fontsize=9)
ax.legend(fontsize=7); fig.tight_layout(); fig.savefig("gap.svg"); fig.savefig("gap.png", dpi=150)
from matplotlib.backends.backend_pdf import PdfPages
with PdfPages("REPORT.pdf") as pdf:
    lines = open("README.md").read().splitlines()
    for k in range(0, len(lines), 95):
        t = plt.figure(figsize=(8.27, 11.69)); t.text(0.04, 0.96, "\n".join(lines[k:k + 95]), fontsize=5.8, family="monospace", va="top"); pdf.savefig(t)
    pdf.savefig(fig)
print("ok")
