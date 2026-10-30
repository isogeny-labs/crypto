import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp") + "/mpl")
import numpy as np
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
T = json.load(open("data/profile_table.json"))
fig, ax = plt.subplots(1, 3, figsize=(14, 4.2))
cells = sorted({t["cell"] for t in T if t["degree"] == 4})
col = {c: p for c, p in zip(cells, ["#2a6fbb", "#d9822b", "#3a9a5b", "#8a3ab9"])}
# (a) rank fractions at degree 4
for i, c in enumerate(cells):
    for t in [t for t in T if t["cell"] == c and t["degree"] == 4]:
        ax[0].scatter(i + (0.12 if t["sat"] else -0.12), t["rank"] / t["columns"], color="#2a6fbb" if t["sat"] else "#c0392b", s=30)
        ax[0].scatter(i, t["xl_rank"] / t["columns"], color="#777", marker="_", s=120)
ax[0].set_xticks(range(len(cells))); ax[0].set_xticklabels(cells, fontsize=8)
ax[0].set_ylabel("rank / columns, degree 4")
ax[0].set_title("closure rank (blue sat, red unsat = refuted) and XL rank (grey bar)", fontsize=8)
# (b) subspace-column share per block
seen = set()
for t in [t for t in T if t["degree"] == 4 and t["sat"]]:
    b = {int(k): v for k, v in t["blocks"].items()}
    ax[1].plot([2, 3, 4], [100 * b[d]["sub_cols"] / b[d]["cols"] for d in (2, 3, 4)], marker="o", color=col[t["cell"]], label=None if t["cell"] in seen else t["cell"])
    seen.add(t["cell"])
ax[1].set_xticks([2, 3, 4]); ax[1].set_xlabel("degree block"); ax[1].set_ylabel("% of columns that are subspace-only monomials")
ax[1].legend(fontsize=7); ax[1].set_title("products-of-subspace-coordinates share of columns", fontsize=8)
# (c) cost: Krylov vs dense base vs closure at degree 4 (unsat draws)
rows = [t for t in T if t["degree"] == 4 and not t["sat"]]
x = np.arange(len(rows)); w = 0.27
kr = [2 * (t["xl_rank"] / 64) * t["input_nnz"] for t in rows]
ax[2].bar(x - w, kr, w, label="Krylov 2(r/64)nnz (plain XL)", color="#3a9a5b")
ax[2].bar(x, [t["base_ops"] for t in rows], w, label="dense Macaulay reduction", color="#2a6fbb")
ax[2].bar(x + w, [t["total_ops"] for t in rows], w, label="closure total (refutes)", color="#c0392b")
ax[2].set_yscale("log"); ax[2].set_xticks(x); ax[2].set_xticklabels([f'{t["cell"][1:]}\nd{t["draw"]}' for t in rows], fontsize=6)
ax[2].set_ylabel("word operations"); ax[2].legend(fontsize=6); ax[2].set_title("degree 4, unsatisfiable draws: Krylov = dense base << closure", fontsize=8)
plt.tight_layout(); plt.savefig("matrix_profile.png", dpi=160); plt.savefig("matrix_profile.svg"); print("ok")
