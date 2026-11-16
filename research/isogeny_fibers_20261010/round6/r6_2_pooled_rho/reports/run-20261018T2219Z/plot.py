"""Plot recorded R6.2 values (batch.json, paths.json); no computation of results."""
import json, sys, pathlib
import matplotlib; matplotlib.use("Agg")
import matplotlib.pyplot as plt
here = pathlib.Path(__file__).parent; src = here.parent.parent
def save_svg(fig, path):
    fig.savefig(path)
    path = pathlib.Path(path)
    lines = path.read_text(encoding="utf-8").splitlines()
    path.write_text("\n".join(line.rstrip() for line in lines) + "\n", encoding="utf-8")

b = json.load(open(src / "batch.json")); p = json.load(open(src / "paths.json"))
s = b["summary"]; L = [r["L"] for r in s]
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for key, lab, c, m in [("B", "B: transport to E0, pooled", "#2a6fdb", "o"),
                       ("C", "C: L keys on E0, pooled (control)", "#d9822b", "s")]:
    ax.errorbar(L, [r["mean_" + key] for r in s], yerr=[2 * r["se_" + key] for r in s],
                label=lab, color=c, marker=m, capsize=3, lw=1.2)
ax.errorbar(L, [r["L"] * r["mean_single"] for r in s], yerr=[2 * r["L"] * r["se_single"] for r in s],
            label="L x mean(S single)  (independent solves)", color="#888", marker="^", ls="--", capsize=3)
ax.plot(L, [r["ks_reference"] for r in s], color="k", lw=0.8, ls=":", label="Kuhn-Struik reference")
ax.set_xscale("log", base=2); ax.set_yscale("log")
ax.set_xlabel("targets L"); ax.set_ylabel("total cost S = group-op equivalents / sqrt(r)")
ax.set_title(f"R6.2 batch, r = 2^{b['base_curve']['rbits_actual']:.2f}, 20 trials per L, bars = 2 SE", fontsize=9)
ax.legend(fontsize=7.5); ax.grid(alpha=.3, which="both"); fig.tight_layout()
save_svg(fig, here / "batch_cost_vs_L.svg")
fig.savefig(here / "batch_cost_vs_L.png", dpi=150)
fig, ax = plt.subplots(figsize=(6.4, 4.2))
for pb, c in zip([24, 28, 32], ["#2a6fdb", "#d9822b", "#3a9a5b"]):
    cl = [x for x in p["classes"] if x["pbits"] == pb and "p" in x]
    ax.scatter([x["component_size"] for x in cl], [x["mean_enumerations"] for x in cl], color=c, label=f"p ~ 2^{pb}")
ax.set_xscale("log"); ax.set_yscale("log")
ax.set_xlabel("component size (curves)"); ax.set_ylabel("mean neighbour enumerations per pair")
n_components = sum("p" in x for x in p["classes"])
ax.set_title(f"R6.2 paths: bidirectional BFS, {n_components} components, cap {p['cap']}", fontsize=9)
ax.legend(fontsize=8); ax.grid(alpha=.3, which="both"); fig.tight_layout()
save_svg(fig, here / "paths_enumerations_vs_component.svg")
fig.savefig(here / "paths_enumerations_vs_component.png", dpi=150)
