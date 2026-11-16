"""Lay out recorded values of summary2.json (written by the Rust example
generator_effect_dryrun_v2_summary). No result is computed here."""
import json, os
os.environ.setdefault("MPLCONFIGDIR", os.environ.get("TMPDIR", "/tmp"))
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

d = json.load(open("summary2.json"))
cur = d["curves"]
col = {"factor_base": "#6b8e23", "oracle_setup": "#4c78a8", "relations": "#e0a030",
       "linear_algebra": "#c0392b", "verify": "#888888"}
nm = {"factor_base": "base construction", "oracle_setup": "pair-table build",
      "relations": "target draw + probes (relations phase)", "linear_algebra": "linear algebra", "verify": "verify"}
arms = [("rho-neg", "rho"), ("bsgs-neg-B", "bsgs B"), ("bsgs-neg-tbl", "bsgs tbl"),
        ("ic-abscissa", "abscissa"), ("ic-knownlog", "known-log\nprogression"),
        ("ic-knownlog-random", "known-log\nrandom")]
fig, ax = plt.subplots(1, 3, figsize=(17, 5.4), constrained_layout=True)

a = ax[0]
x = 0
ticks, tl = [], []
for ci, c in enumerate(cur):
    for w in (0.0, 1.0):
        for arm, short in arms:
            bw = [b for b in c["arms"][arm]["by_weight"] if b["w"] == w][0]
            if arm.startswith("ic-"):
                bottom = 0
                for ph in col:
                    v = bw["phases_s"][ph]
                    a.bar(x, v, bottom=bottom, color=col[ph],
                          label=nm[ph] if (ci == 0 and w == 0 and arm == "ic-abscissa") else None)
                    bottom += v
            else:
                a.bar(x, bw["s_mean"], color="#999999" if arm == "rho-neg" else "#b07aa1",
                      label=("rho / bsgs total (not split)" if (ci == 0 and w == 0 and arm == "rho-neg") else None))
            lo, hi = bw["s_ci90"]
            a.errorbar(x, bw["s_mean"], yerr=[[bw["s_mean"] - lo], [hi - bw["s_mean"]]], color="black", capsize=2, lw=0.8)
            ticks.append(x); tl.append(f"{short}\nw={w:g} c{ci+1}")
            x += 1
        x += 0.8
    x += 0.8
a.set_xticks(ticks); a.set_xticklabels(tl, fontsize=5.5, rotation=90)
a.set_ylabel("S = GAE / sqrt(r), mean per target (90% bootstrap CI)")
a.set_title("A. Per-phase S, six arms, T = 1")
a.legend(fontsize=6, loc="upper right")

b = ax[1]
pos = 0
for ci, c in enumerate(cur):
    for arm, short, color in (("ic-abscissa", "abscissa", "#4c78a8"), ("ic-knownlog", "progression", "#c0392b"),
                              ("ic-knownlog-random", "random", "#2a9d5c")):
        v = c["yield_block"][arm]
        lo, hi = v["yield_ci90"]
        b.errorbar(pos, v["yield"], yerr=[[v["yield"] - lo], [hi - v["yield"]]], fmt="o", color=color, capsize=4,
                   label=f"{short} measured" if ci == 0 else None)
        if "predicted" in v:
            b.plot(pos, v["predicted"], "x", color="black", ms=9,
                   label="pre-registered |2 sum S|/r" if (ci == 0 and arm == "ic-knownlog") else None)
        pos += 1
    pos += 1.5
b.set_yscale("log"); b.set_xticks([1, 5.5]); b.set_xticklabels(["c1 (B=205)", "c2 (B=240)"])
b.set_ylabel("relations per target tried (90% CI), 3000 targets")
b.set_title("B. Hit rate: pre-registered prediction vs measurement")
b.legend(fontsize=7)

cc = ax[2]
c = cur[0]
mm = c["multi_target_model"]
series = [("abscissa_s_per_key", "abscissa", "#4c78a8", "o"), ("knownlog_s_per_key", "known-log progression", "#c0392b", "s"),
          ("knownlog_random_s_per_key", "known-log random", "#2a9d5c", "^")]
for w, ls in ((0.0, "-"), (1.0, "--")):
    pts = [m for m in mm if m["w"] == w]
    T = [m["T"] for m in pts]
    for key, label, color, mk in series:
        cc.plot(T, [m[key] for m in pts], ls, color=color, marker=mk, label=f"{label} w={w:g}")
    cc.axhline(pts[0]["rho_neg_s"], color="black", ls=":" if w == 0 else "-.", lw=0.9, label=f"rho-neg w={w:g}")
    cc.plot(T, [m["bsgs_neg_B_s_per_key"] for m in pts], ls, color="#b07aa1", marker="d", ms=4, label=f"bsgs B w={w:g}")
    cc.plot(T, [m["bsgs_neg_tbl_s_per_key"] for m in pts], ls, color="#7f4f8f", marker="v", ms=4, label=f"bsgs tbl w={w:g}")
cc.set_xscale("log", base=2); cc.set_yscale("log")
cc.set_xlabel("keys T under one G"); cc.set_ylabel("S per key (derived from measured parts), curve c1")
cc.set_title("C. S per key vs T")
cc.legend(fontsize=6, ncol=2)
fig.suptitle("SCRATCH dry run 2, not data. Generator-effect Amendment 1, two 24-bit a=-3 prime-order curves. "
             "Phase sessions ECBS1h0331ce29276e, ECBS1hd98aace0ae34; yield sessions ECBS1h6bb52a49ea71, ECBS1hf684e20129e7.", fontsize=8)
fig.savefig("dryrun2.svg"); fig.savefig("dryrun2.png", dpi=150)
