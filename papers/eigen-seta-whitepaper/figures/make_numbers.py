"""Tables and the performance graph of the white paper, laid out from the
committed benchmark records (summaries of recorded values only: medians,
minima, maxima; no result is computed here).

Usage: python figures/make_numbers.py <study_dir> <paper_dir>
  <study_dir> = crypto:research/eigen_seta_whitepaper_20261017
Writes <paper_dir>/numbers_data.tex and perf.{pdf,svg,png} into
<paper_dir>/figures and <study_dir>/figures.
"""
import os, sys, statistics, shutil
import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt

STUDY, PAPER = sys.argv[1], sys.argv[2]
REC = os.path.join(STUDY, "records")
RID = open(os.path.join(REC, "RUN_ID")).read().strip()
SETS = ["L1", "L3", "L5", "Q3", "Q5"]


def rows(name):
    p = os.path.join(REC, RID, f"{name}.tsv")
    if not os.path.exists(p):
        return []
    out = []
    for line in open(p):
        f = line.rstrip("\n").split("\t")
        if len(f) < 2 or f[0] not in ("sizes", "field", "keygen", "op", "pke", "chain"):
            continue
        d = dict(x.split("=", 1) for x in f[2:] if "=" in x)
        d["_kind"] = f[0]
        out.append(d)
    return out


def col(rs, kind, key):
    return [float(r[key]) for r in rs if r["_kind"] == kind and key in r]


def mr(v, fmt="{:.2f}"):
    if not v:
        return "---"
    m = statistics.median(v)
    s = fmt.format(m)
    if len(v) > 1:
        s += " [" + fmt.format(min(v)) + "--" + fmt.format(max(v)) + "]"
    return s + f", $n{{=}}{len(v)}$"


def med(v, fmt="{:.2f}"):
    return fmt.format(statistics.median(v)) if v else "---"


def tfmt(x):
    # seconds -> compact human form for the summary
    if x >= 3600:
        return f"{x/3600:.1f}\\,h"
    if x >= 60:
        return f"{x/60:.0f}\\,min"
    return f"{x:.2f}\\,s" if x < 10 else f"{x:.0f}\\,s"


PARAMS = {  # from eigenx:src/params_generated.rs and the paper (exact)
    "L1": (128, 1341, 21, 1076, 162, "256.8", 144, 277),
    "L3": (192, 1991, 32, 1594, 243, "385.1", 1059, 409),
    "L5": (256, 2630, 42, 2106, 324, "513.5", 380, 536),
    "Q3": (192, 3904, 61, 3125, 485, "768.7", 371, 791),
    "Q5": (256, 5192, 82, 4155, 647, "1025.5", 975, 1049),
}
CAT = {"L1": ("1", "no"), "L3": ("3", "no"), "L5": ("5", "yes ($\\lambda{=}128$)"), "Q3": ("3", "yes"), "Q5": ("5", "yes")}
FLOOR = {"L1": ("128.4", "64.5"), "L3": ("192.6", "96.5"), "L5": ("256.8", "128.5"), "Q3": ("384.4", "192.5"), "Q5": ("512.7", "256.5")}

data = {s: rows(s) for s in SETS}
one = {s: rows(s + "_1core") for s in ["L1", "L3", "L5"]}
L = []
summ, par, perf, onec, fld, chn = [], [], [], [], [], []
for s in SETS:
    r = data[s]
    sz = next((x for x in r if x["_kind"] == "sizes"), None)
    pk = sz["pk"] if sz else "---"
    sk = sz["sk"] if sz else "---"
    po = sz["pke_overhead"] if sz else "---"
    lam, pb, limbs, E, F, ld, c, fb = PARAMS[s]
    par.append(f"{s} & {lam} & {pb} & {limbs} & {E} & {F} & {ld} & {c} & {fb} & {pk} & {sk} & {po}\\\\")
    kg, kc = col(r, "keygen", "wall_s"), col(r, "keygen", "cpu_s")
    en, de = col(r, "op", "encaps_s"), col(r, "op", "decaps_s")
    pe, pd = col(r, "pke", "encrypt_s"), col(r, "pke", "decrypt_s")
    oks = [x.get("ok") for x in r if x["_kind"] in ("op", "pke")]
    if oks and not all(o == "true" for o in oks):
        print("WARNING: a decapsulation or decryption failed in", s)
    pending = "\\emph{pending}" if not r else None
    summ.append(" & ".join([s, CAT[s][0], CAT[s][1], str(pb), pk, sk,
                            med(kg, "{:.0f}") if kg else "---", med(en, "{:.2f}"), med(de, "{:.2f}"),
                            FLOOR[s][0], FLOOR[s][1]]) + "\\\\")
    jobs = next((x["jobs"] for x in r if x["_kind"] == "keygen"), "?")
    perf.append(f"{s} (jobs={jobs}) & {mr(kg, '{:.1f}')} & {mr(kc, '{:.1f}')} & {mr(en, '{:.3f}')} & {mr(de, '{:.2f}')} & {med(pe, '{:.3f}')} / {med(pd, '{:.2f}')}\\\\" if r else f"{s} & \\multicolumn{{5}}{{l}}{{not measured in this run (\\S\\ref{{sec:method}})}}\\\\")
    fm, f2, fi = col(r, "field", "fp_mul_ns"), col(r, "field", "fp2_mul_ns"), col(r, "field", "fp_inv_ns")
    if fm:
        fld.append(f"{s} & {limbs} & {fm[0]:.0f} & {f2[0]:.0f} & {fi[0]:,.0f}\\\\")
    ce, cw, cs, cm = col(r, "chain", "e"), col(r, "chain", "wall_s"), col(r, "chain", "per_step_us"), col(r, "chain", "fp2_mul_per_step")
    if ce:
        chn.append(f"{s} & {int(statistics.median(ce))} & {med(cw, '{:.2f}')} & {med(cs, '{:.0f}')} & {med(cm, '{:.0f}')}\\\\")
for s in ["L1", "L3", "L5"]:
    r = one[s]
    if r:
        onec.append(f"{s} & {mr(col(r, 'keygen', 'wall_s'), '{:.1f}')} & {mr(col(r, 'op', 'encaps_s'), '{:.3f}')} & {mr(col(r, 'op', 'decaps_s'), '{:.2f}')}\\\\")

allm = [m for s in SETS for m in col(data[s], "chain", "fp2_mul_per_step")]


def nc(name, body):
    L.append(f"\\newcommand{{\\{name}}}{{%\n{body}}}")


nc("SummaryRows", "\n".join(summ))
nc("ParamRows", "\n".join(par))
nc("PerfRows", "\n".join(perf))
nc("OneCoreRows", "\n".join(onec) if onec else "\\multicolumn{4}{l}{---}\\\\")
nc("FieldRows", "\n".join(fld))
nc("ChainRows", "\n".join(chn))
nc("BenchRunId", RID)
de1 = col(data["L1"], "op", "decaps_s"); en1 = col(data["L1"], "op", "encaps_s")
nc("DecLOne", med(de1, "{:.1f}"))
nc("EncLOne", med(en1, "{:.2f}"))
kq3 = col(data["Q3"], "keygen", "wall_s")
nc("KgQThree", tfmt(statistics.median(kq3)) if kq3 else "tens of minutes (not measured here)")
if allm:
    nc("StepMults", f"{min(allm):.0f}--{max(allm):.0f}")
    nc("StepFactor", f"{min(allm)/256:.1f}--{max(allm)/256:.1f}")
open(os.path.join(PAPER, "numbers_data.tex"), "w").write("% Generated by figures/make_numbers.py from the records; do not edit.\n" + "\n".join(L) + "\n")

# --- graph: every sample, log scale ------------------------------------
fig, ax = plt.subplots(figsize=(7.2, 3.8))
ops = [("keygen", "keygen", "wall_s", "s", "#7a5195"), ("encaps", "op", "encaps_s", "o", "#ef5675"),
       ("decaps", "op", "decaps_s", "^", "#2f6db5"), ("PKE decrypt", "pke", "decrypt_s", "x", "#3a9a5b")]
present = [s for s in SETS if data[s]]
for j, (lab, kind, key, mk, colr) in enumerate(ops):
    for i, s in enumerate(present):
        v = col(data[s], kind, key)
        ax.scatter([i + (j - 1.5) * 0.16] * len(v), v, marker=mk, color=colr, s=14, label=lab if i == 0 else None, zorder=3)
ax.set_xticks(range(len(present)))
ax.set_xticklabels([f"{s}\n(p {PARAMS[s][1]} b)" for s in present])
ax.set_yscale("log"); ax.set_ylabel("wall time [s], log"); ax.grid(True, which="both", axis="y", alpha=0.3)
ax.legend(fontsize=7, ncol=4, loc="upper left")
fig.suptitle(f"run {RID}; 4 vCPU shared VM; every sample shown", fontsize=8)
fig.tight_layout()
for d in (os.path.join(PAPER, "figures"), os.path.join(STUDY, "figures")):
    os.makedirs(d, exist_ok=True)
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(d, f"perf.{ext}"))
    fig.savefig(os.path.join(d, "perf.png"), dpi=150)
print("present:", present, "one-core:", [s for s in one if one[s]])

# --- same-host comparison with liboqs KEMs --------------------------------
PQ = os.path.join(REC, "pqkem_same_host")
reps = sorted(f for f in os.listdir(PQ) if f.startswith("rep") and f.endswith(".tsv")) if os.path.isdir(PQ) else []
pq = {}
for f in reps:
    for line in open(os.path.join(PQ, f)):
        x = line.rstrip("\n").split("\t")
        if len(x) < 3 or x[0] != "kem" or "enabled=false" in line:
            continue
        d = dict(y.split("=", 1) for y in x[2:] if "=" in y)
        pq.setdefault(x[1], []).append(d)


def pqmean(name, key):
    v = [float(d[key]) for d in pq.get(name, [])]
    return sum(v) / len(v) if v else None


def sig2(x):
    from math import floor, log10
    if x <= 0:
        return "0"
    d = 10 ** (floor(log10(x)) - 1)
    return f"{round(x / d) * d:,.0f}"


def fmt_t(us):
    if us is None:
        return "---"
    if us >= 1e6:
        return f"{us/1e6:.2f}\\,s"
    if us >= 1e3:
        return f"{us/1e3:.1f}\\,ms"
    return f"{us:.0f}\\,$\\mu$s"


def fmt_b(b):
    b = int(b)
    return f"{b/1000:.0f}\\,k" if b >= 100000 else f"{b:,}"


cmp_rows = []
groups = [(1, ["ML-KEM-512", "BIKE-L1", "HQC-128", "FrodoKEM-640-AES", "Classic-McEliece-348864"], ["L1"]),
          (3, ["ML-KEM-768", "BIKE-L3", "HQC-192", "FrodoKEM-976-AES", "Classic-McEliece-460896"], ["L3", "Q3"]),
          (5, ["ML-KEM-1024", "BIKE-L5", "HQC-256", "FrodoKEM-1344-AES", "Classic-McEliece-6688128"], ["L5", "Q5"])]
ratio = {}
for cat, names, ours in groups:
    cmp_rows.append(f"\\multicolumn{{6}}{{l}}{{\\emph{{Category {cat}}}}}\\\\")
    for n in names:
        if n not in pq:
            continue
        d0 = pq[n][0]
        star = "$^\\dagger$" if n.startswith("HQC") else ""
        cmp_rows.append(f"{n.replace('Classic-McEliece', 'McEliece')}{star} & {fmt_t(pqmean(n,'keygen_med_us'))} & {fmt_t(pqmean(n,'encaps_med_us'))} & {fmt_t(pqmean(n,'decaps_med_us'))} & {fmt_b(d0['pk_bytes'])} & {fmt_b(d0['ct_bytes'])}\\\\")
    for s in ours:
        r = data[s]
        if not r:
            continue
        kg, en, de = col(r, "keygen", "wall_s"), col(r, "op", "encaps_s"), col(r, "op", "decaps_s")
        sz = next(x for x in r if x["_kind"] == "sizes")
        cmp_rows.append(f"\\textbf{{Eigen-S\\'eta {s}}} & {fmt_t(statistics.median(kg)*1e6)} & {fmt_t(statistics.median(en)*1e6)} & {fmt_t(statistics.median(de)*1e6)} & {fmt_b(sz['pk'])} & {fmt_b(sz['ct'])}\\\\")
        mk = names[0]
        ratio[s] = (statistics.median(en) * 1e6 / pqmean(mk, "encaps_med_us"), statistics.median(de) * 1e6 / pqmean(mk, "decaps_med_us"))
    cmp_rows.append("\\midrule")
if cmp_rows and cmp_rows[-1] == "\\midrule":
    cmp_rows.pop()
extra = []
extra.append("\\newcommand{\\CompareRows}{%\n" + "\n".join(cmp_rows) + "}")
if "L1" in ratio:
    extra.append(f"\\newcommand{{\\RatioEncLOne}}{{{sig2(ratio['L1'][0])}}}")
    extra.append(f"\\newcommand{{\\RatioDecLOne}}{{{sig2(ratio['L1'][1])}}}")
qrat = ", ".join(f"{s} {sig2(ratio[s][1])}" for s in ["L1", "L3", "L5", "Q3", "Q5"] if s in ratio)
extra.append(f"\\newcommand{{\\RatioDecAll}}{{{qrat}}}")
q5 = data["Q5"]
if q5 and col(q5, "keygen", "wall_s"):
    extra.append(f"\\newcommand{{\\KgQFive}}{{{tfmt(statistics.median(col(q5, 'keygen', 'wall_s')))}}}")
with open(os.path.join(PAPER, "numbers_data.tex"), "a") as fh:
    fh.write("\n".join(extra) + "\n")
print("ratios (enc, dec) vs ML-KEM same level:", {k: (round(a), round(b)) for k, (a, b) in ratio.items()})

# --- comparison graph: decapsulation per KEM, by category, log scale ------
fig, axes = plt.subplots(1, 3, figsize=(9.6, 3.6), sharey=True)
for ax, (cat, names, ours) in zip(axes, groups):
    labs, vals, cols = [], [], []
    for n in names:
        if n in pq:
            labs.append(n.replace("Classic-McEliece", "McE").replace("FrodoKEM", "Frodo").replace("-AES", ""))
            vals.append(pqmean(n, "decaps_med_us") / 1e6)
            cols.append("#9aa5b1")
    for s in ours:
        if data[s]:
            labs.append(f"Eigen-Séta {s}")
            vals.append(statistics.median(col(data[s], "op", "decaps_s")))
            cols.append("#2f6db5")
    ax.bar(range(len(vals)), vals, color=cols)
    ax.set_yscale("log")
    ax.set_xticks(range(len(vals)))
    ax.set_xticklabels(labs, rotation=60, ha="right", fontsize=7)
    ax.set_title(f"category {cat}", fontsize=9)
    ax.grid(True, which="both", axis="y", alpha=0.3)
axes[0].set_ylabel("decapsulation [s], log")
fig.suptitle(f"same host; liboqs 0.13.0 (HQC portable) vs Eigen-Séta run {RID}", fontsize=8)
fig.tight_layout()
for d in (os.path.join(PAPER, "figures"), os.path.join(STUDY, "figures")):
    for ext in ("pdf", "svg"):
        fig.savefig(os.path.join(d, f"compare.{ext}"))
    fig.savefig(os.path.join(d, "compare.png"), dpi=150)
