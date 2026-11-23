"""Per-operation cost figure from data-fm-opcost.json: ns per native operation on the
boundary ledger's host (median of three frozen runs, bars from min to max), against
the field size, for prime, random binary and Koblitz instances.

    python3 make_fig_fm_opcost.py   # writes fig-fm-opcost.{pdf,svg}
"""
import json
import os

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-fm-opcost.json")))
plt.rcParams.update({"font.size": 8.5, "font.family": "serif", "axes.spines.top": False,
                     "axes.spines.right": False, "axes.grid": True, "grid.alpha": 0.25,
                     "svg.fonttype": "none"})
C = {"ns_per_add": "#0072B2", "ns_per_inversion": "#E69F00", "ns_per_legendre": "#CC79A7",
     "ns_per_as_solve": "#009E73", "ns_per_frobenius": "#56B4E9", "ns_per_canon": "#D55E00",
     "ns_per_lookup": "#999999", "ns_per_word_xor": "#000000"}
LBL = {"ns_per_add": "group addition", "ns_per_inversion": "field inversion",
       "ns_per_legendre": "Legendre symbol", "ns_per_as_solve": "Artin--Schreier solve",
       "ns_per_frobenius": "Frobenius map", "ns_per_canon": "orbit canonicalisation",
       "ns_per_lookup": "table lookup", "ns_per_word_xor": "word XOR"}
M = {"ns_per_add": "o", "ns_per_inversion": "s", "ns_per_legendre": "D", "ns_per_as_solve": "^",
     "ns_per_frobenius": "v", "ns_per_canon": "P", "ns_per_lookup": "x", "ns_per_word_xor": "."}
panels = [("prime", r"prime field: $\log_2 p$", ["ns_per_add", "ns_per_inversion", "ns_per_legendre", "ns_per_lookup", "ns_per_word_xor"]),
          ("char2", r"random binary: $n$ in $\mathbb{F}_{2^n}$", ["ns_per_add", "ns_per_as_solve", "ns_per_frobenius", "ns_per_lookup", "ns_per_word_xor"]),
          ("koblitz", r"Koblitz: $n$ in $\mathbb{F}_{2^n}$", ["ns_per_add", "ns_per_as_solve", "ns_per_frobenius", "ns_per_canon", "ns_per_lookup", "ns_per_word_xor"])]
fig, axes = plt.subplots(1, 3, figsize=(7.2, 3.1), sharey=True)
for ax, (reg, xl, ops) in zip(axes, panels):
    rows = sorted((r for r in D["ledger_host"] if r["regime"] == reg), key=lambda r: r["field_bits"])
    for op in ops:
        xs, ys, lo, hi = [], [], [], []
        for r in rows:
            v = r["ns"].get(op)
            if v is None:
                continue
            xs.append(r["field_bits"]); ys.append(v["median"])
            lo.append(v["median"] - v["min"]); hi.append(v["max"] - v["median"])
        ax.errorbar(xs, ys, yerr=[lo, hi], color=C[op], marker=M[op], ms=3.5, lw=0.9,
                    capsize=1.5, label=LBL[op])
    ax.set_yscale("log")
    ax.set_xlabel(xl)
axes[0].set_ylabel("ns per operation (Xeon 2.1 GHz)")
h, l = [], []
for ax in axes:
    for hh, ll in zip(*ax.get_legend_handles_labels()):
        if ll not in l:
            h.append(hh); l.append(ll)
fig.legend(h, l, fontsize=6.5, ncol=4, frameon=False, loc="upper center")
fig.tight_layout(rect=(0, 0, 1, 0.86))
for ext in ("pdf", "svg"):
    fig.savefig(os.path.join(HERE, f"fig-fm-opcost.{ext}"))
