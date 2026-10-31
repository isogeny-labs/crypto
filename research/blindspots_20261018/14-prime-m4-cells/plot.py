# Lays out values recorded in raw/summary.md (output of `b14 summarize`, which reads raw/*.jsonl). No computation of results.
import re, matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
txt = open("raw/summary.md").read()
def section(h):
    s = txt.split(h, 1)[1]; return s.split("\n## ", 1)[0]
def rows(sec):
    out = []
    for l in sec.splitlines():
        if l.startswith("|") and not l.startswith("|--") and not l.startswith("| bits") and not l.startswith("| source") and not l.startswith("| poly"):
            out.append([c.strip() for c in l.strip("|").split("|")])
    return out
def f(x): return float(x)
ic = rows(section("## Full IC to completion, arm (a)"))
# bits,bx,runs,ok,attempts,hit,GAE,native,S0,S01,S1,rho0,rho01,rho1,r0,r01,r1
design = {16: "8", 20: "15", 24: "29", 28: "63"}
ratios = {k: [] for k in ("0", "0.1", "1")}
for bits, bx in design.items():
    r = [x for x in ic if x[0] == str(bits) and x[1] == bx][0]
    ratios["0"].append(f(r[14])); ratios["0.1"].append(f(r[15])); ratios["1"].append(f(r[16]))
bm = [r for r in rows(section("## Arm (b): F4 on S5 with the membership polynomials")) if r[0] == "16" and r[5] != "0/0"]
bxs = [int(r[1]) for r in bm]; madds = [f(r[7]) for r in bm]
ta = [r for r in rows(section("## Arm (a): T_dec per attempt")) if r[0] == "16"]
fig, ax = plt.subplots(1, 2, figsize=(11, 4.6))
a = ax[0]
a.loglog([2*b for b in bxs], madds, "o-", color="#d62728", label="arm (b) F4 multiply-adds, 16-bit ladder (18 targets/cell)")
a.loglog([2*int(r[2]) for r in ta], [f(r[10]) for r in ta], "s-", color="#1f77b4", label="arm (a) lookups+adds per attempt (GAE at w=0)")
a.loglog([8, 16, 32], [4.8447e7, 8.5478e10, 2.7048e13], "^--", color="#2ca02c", label="arm (c) PKM multiply-adds (|V|=4,8,16; reused, p=2013265921)")
a.set_xlabel("signed base points |F| (arm c plotted at 2|V|)"); a.set_ylabel("native operations per decomposition attempt")
a.set_title("Per-attempt oracle cost vs base size\narm (b) slope 11.03 [11.01, 11.05] in |F|", fontsize=9); a.legend(fontsize=7); a.grid(True, which="both", alpha=.25)
b = ax[1]
x = [16, 20, 24, 28]
for k, c in (("0", "#7f7f7f"), ("0.1", "#ff7f0e"), ("1", "#9467bd")):
    b.plot(x, ratios[k], "o-", color=c, label=f"w = {k} GAE per native op")
b.axhline(1, color="k", lw=.8); b.set_yscale("log"); b.set_xticks(x)
b.set_xlabel("log2 p"); b.set_ylabel("S(arm a full IC) / matched rho S  (>1: IC costs more)")
b.set_title("Arm (a), the only complete m = 4 solve at the design cells\n16 completed runs per cell, 16 rho runs per cell; distance to rho remaining", fontsize=9)
b.legend(fontsize=7); b.grid(True, which="both", alpha=.25)
fig.suptitle("Item 14: first prime-field m = 4 cost cells (host: shared 4-vCPU container, one thread, no PMU isolation)", fontsize=9)
fig.tight_layout(); fig.savefig("graph.svg"); fig.savefig("graph.png", dpi=150)
