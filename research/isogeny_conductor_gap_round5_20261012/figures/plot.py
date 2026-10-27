"""Best IC over rho per curve (tables_e7.json, from examples/isogeny_gap_round5_e7_tables.rs): the fiber-closed
table on the raw base against the blind table on the cofactor-cleared base. Lays out recorded/derived values only."""
import json, os
import matplotlib
matplotlib.use("svg")
import matplotlib.pyplot as plt
here = os.path.dirname(os.path.abspath(__file__))
t = json.load(open(os.path.join(here, "../tables_e7.json")))
fig, ax = plt.subplots(figsize=(7.2, 4.4))
for key, h, mk in (("e7a_h4", 4, "o"), ("e7a_h72", 72, "s")):
    cs = t[key]["curves"]
    ax.scatter([c["log2_order"] for c in cs], [c["closed_over_rho"] for c in cs], marker=mk, facecolors="none",
               edgecolors="#c0632b", s=44, label=f"h = {h}: best fiber-closed table, raw base")
    ax.scatter([c["log2_order"] for c in cs], [c["subgroup_over_rho"] for c in cs], marker=mk, color="#3b6ea8", s=44,
               label=f"h = {h}: best blind table, cofactor-cleared base")
ax.axhline(1, color="black", lw=1, ls=":"); ax.text(17.3, 1.12, "rho", fontsize=8)
ax.set_yscale("log", base=2); ax.set_xlabel("log2 #E"); ax.set_ylabel("IC S ÷ rho S, per curve")
ax.set_title("E7a: clearing the cofactor beats the fiber-closed table by about h^(1/3); rho stays ahead", fontsize=9)
ax.grid(True, which="both", alpha=0.3); ax.legend(fontsize=7, loc="upper left", ncol=2)
ax.set_ylim(0.8, 128)
fig.tight_layout(); fig.savefig(os.path.join(here, "e7a_ic_over_rho.svg")); print("ok")
