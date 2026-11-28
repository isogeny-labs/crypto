#!/usr/bin/env python3
"""Presentation only: figure.svg/figure.png and REPORT.pdf (vector) from analysis.json and the raw rows."""
import glob, json, os
import matplotlib
matplotlib.use("Agg")
matplotlib.rcParams["pdf.compression"] = 9
import matplotlib.pyplot as plt
from matplotlib.backends.backend_pdf import PdfPages

HERE = os.path.dirname(os.path.abspath(__file__))
a = json.load(open(os.path.join(HERE, "analysis.json")))
PS = ["16411", "32771"]
CCS = ["1.0", "1.4", "2.0", "2.8"]


def wall_by_cc():
    out = {p: {} for p in PS}
    for p in PS:
        for cc in CCS:
            w = []
            for f in glob.glob(os.path.join(HERE, f"ic_pass*_p{p}_cc{cc}.jsonl")):
                for line in open(f):
                    r = json.loads(line)
                    if r.get("kind") == "run" and "(4,1.5)" in r["arm"]:
                        w.append(r["wall_ns"] / 1e9)
            out[p][cc] = sum(w) / len(w)
    return out


def figure():
    f, (a1, a2) = plt.subplots(1, 2, figsize=(10, 4.2))
    # left: measured weights against the §5a band
    xs = range(len(PS))
    for k, (key, c, lab) in enumerate((("w_rel", "#1f77b4", "w_rel (relation op)"), ("w_la", "#d62728", "w_la (mod-r LA op)"))):
        v = [a["primes"][p][key] for p in PS]
        a1.errorbar([x + 0.08 * (k - 0.5) for x in xs], [x["value"] for x in v],
                    yerr=[[x["value"] - x["ci"][0] for x in v], [x["ci"][1] - x["value"] for x in v]],
                    fmt="o", color=c, capsize=3, label=lab)
    for w, ls in ((1, "--"), (0.1, ":")):
        a1.axhline(w, color="k", lw=0.8, ls=ls)
        a1.text(1.25, w * 1.1, f"§5a w = {w}", fontsize=7)
    a1.set(yscale="log", xticks=list(xs), xticklabels=[f"p = {p}" for p in PS], xlim=(-0.5, 1.6),
           ylabel="GAE per counted operation", title="Measured prices vs the §5a band")
    a1.legend(fontsize=7, loc="center left")
    # right: wall time by factor-base constant (post hoc)
    wb = wall_by_cc()
    for p, c in zip(PS, ("#2ca02c", "#9467bd")):
        a2.plot([float(x) for x in CCS], [wb[p][cc] for cc in CCS], "o-", color=c, label=f"p = {p}")
    a2.set(xlabel="factor-base constant cc", ylabel="mean IC wall time (s), SGE (4, 1.5)",
           title="Wall time by cc (post hoc)")
    a2.legend(fontsize=7)
    f.suptitle("Genus-4 IC native-operation prices, timing tier  [pcc:unverified]", fontsize=10)
    f.tight_layout()
    return f


f = figure()
f.savefig(os.path.join(HERE, "figure.svg"))
f.savefig(os.path.join(HERE, "figure.png"), dpi=80)
with PdfPages(os.path.join(HERE, "REPORT.pdf")) as pdf:
    pdf.savefig(f)
    plt.close(f)
    for src in ["RESULTS.md", "tables.md"]:
        lines = open(os.path.join(HERE, src)).read().splitlines()
        for i in range(0, len(lines), 95):
            t = plt.figure(figsize=(11, 8.5))
            t.text(0.03, 0.98, "\n".join(lines[i:i + 95]), va="top", family="monospace", fontsize=5.6)
            pdf.savefig(t)
            plt.close(t)
print(os.path.getsize(os.path.join(HERE, "REPORT.pdf")))
