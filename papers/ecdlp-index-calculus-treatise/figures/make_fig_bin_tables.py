#!/usr/bin/env python3
"""Lay out the data-bin-*.json files as LaTeX tables (tab-bin-*.tex).
Formatting only, except where a column is marked derived: codim W_D = columns - dim W_D,
min/max/median over committed runs, and arm/rho ratios of band means."""
import json, os, re, statistics

H = os.path.dirname(os.path.abspath(__file__))


def J(s):
    return json.load(open(os.path.join(H, f"data-bin-{s}.json")))


def num(x):
    return f"{x:,}".replace(",", "{,}")


def ci(v, fmt="{:.3f}"):
    return f"{fmt.format(v[0])} [{fmt.format(v[1])}, {fmt.format(v[2])}]"


def write(name, lines):
    open(os.path.join(H, f"tab-bin-{name}.tex"), "w").write("\n".join(lines) + "\n")


# 1. per-instance fall degree (two tabulars: P and S; R, U and T) ------------
fd = J("fd-instances")
VZ = {("poly", "curve"): ("P", "curve"), ("rand", "curve"): ("R", "curve"), ("rand", "rand"): ("U", "unif."),
      ("tr0", "curve"): ("T", "curve"), ("poly", "rand"): ("S", "unif.")}
HEAD = [r"\begin{tabular}{@{}lccrrrrrrcrll@{}}", r"\toprule",
        r"inst. & $V$ & $z$ & $n$ & $k$ & $D$ & $\dim W_D$ & columns & codim & $1\in W_D$ & s & $d_{\tilde F}$ & cert. \\",
        r"\midrule"]
for part, fams in (("a", "PS"), ("b", "RUT")):
    out = list(HEAD)
    prev = None
    for r in sorted((r for r in fd["m2"] if r["name"][0] in fams),
                    key=lambda r: ("PRUTS".index(r["name"][0]), r["n"], r["name"])):
        V, z = VZ[(r["basis"], r["zmode"])]
        if prev and V != prev:
            out.append(r"\addlinespace")
        prev = V
        dim = r["W_dims_top"][-1]
        dF = str(r["d_ref_exact"]) if r["d_ref_exact"] is not None else f"$\\ge{r['d_ref_lower_bound']}$"
        cert = r["certificate"]["cert_sha256"][:8] if r["certificate"] else ""
        out.append(" & ".join([r["name"], V, z, str(r["n"]), str(r["k"]), str(r["D_top"]), num(dim),
                               num(r["ncols_top"]), "--" if r["W_one_top"] else num(r["ncols_top"] - dim),
                               "yes" if r["W_one_top"] else "no",
                               f"{r['seconds_top']:.1f}", dF, f"\\texttt{{{cert}}}" if cert else "--"]) + r" \\")
    out += [r"\bottomrule", r"\end{tabular}"]
    write(f"fd-instances-{part}", out)

# 2. certificates ---------------------------------------------------------------
meta = {r["name"]: (r["n"], r["k"], VZ[(r["basis"], r["zmode"])][0], 2) for r in fd["m2"]}
meta |= {r["name"]: (r["n"], r["k"], "R" if r["basis"] == "rand" else "P", 3) for r in fd["chain_t3"]}
meta |= {r["name"]: (r["n"], r["k"], "Q", 2) for r in fd["sat"]}
nsol = {r["name"]: r["boolean_solutions"] for r in fd["sat"]}
out = [r"\begin{tabular}{@{}lrrcrlrrrrl@{}}", r"\toprule",
       r"inst. & $m$ & $n$ & $V$ & sol. & $\dim W_d$, $d=0..4$ & columns & codim $\ge$ & closure s & check s & SHA-256 \\",
       r"\midrule"]
for name, c in sorted(fd["certificates"].items(), key=lambda kv: (meta[kv[0]][3], meta[kv[0]][0], kv[0])):
    n, k, V, m = meta[name]
    out.append(" & ".join([name, str(m), str(n), V, str(nsol.get(name, 0)),
                           ", ".join(num(x) for x in c["W4_dims"]), num(c["ncols"]), num(c["codim_W4_ge"]),
                           c["closure_s"].replace("~", "$\\approx$"), str(c["check_s"]),
                           f"\\texttt{{{c['cert_sha256'][:8]}}}"]) + r" \\")
out += [r"\bottomrule", r"\end{tabular}"]
write("fd-certs", out)

# 3. large primes, every arm -------------------------------------------------
lp = J("lp")
G = {"n53": "n53-all", "n61a": "n61-s1", "n61b": "n61-s2"}
LAB = {"n53": r"$n=53$, 16 workloads", "n61a": r"$n=61$, session 1, 4 workloads",
       "n61b": r"$n=61$, session 2, 3 workloads"}


def armkey(a):
    m = re.match(r"(plain|lp0|lp1|lp2|rho)-?k?(\d+)?(?:-c(\d+))?", a)
    order = {"rho": 0, "plain": 1, "lp0": 2, "lp1": 3, "lp2": 4}[m[1]]
    return (order, int(m[2] or 0), int(m[3] or 0), a)


out = [r"\begin{tabular}{@{}lrrrll@{}}", r"\toprule",
       r"arm & $S$, $w=0$ & $w=0.1$ & $w=1$ & arm/rho, paired, $w=0$ & arm/best plain, paired \\",
       r"\midrule"]
for g, b in G.items():
    out.append(rf"\multicolumn{{6}}{{@{{}}l}}{{{LAB[g]} (best plain: \texttt{{{lp['best_plain'][g]}}})}} \\")
    out.append(r"\addlinespace[2pt]")
    B = lp["bands"][b]["arms"]
    P = lp["pairs"][g]
    for a in sorted(B, key=armkey):
        if a.endswith("-aa"):
            continue
        s = B[a]["mean_s"]
        vr = P.get(f"{a}/vs/rho-strong")
        vb = next((v for k, v in P.items() if k.startswith(f"{a}/vs/best-plain")), None)
        if a == lp["best_plain"][g]:
            vb = None
        out.append(" & ".join([f"\\texttt{{{a}}}", f"{s[0]:.3f}", f"{s[1]:.3f}", f"{s[2]:.3f}",
                               ci([vr["ratio"]] + vr["ci95"], "{:.1f}") if vr else "--",
                               ci([vb["ratio"]] + vb["ci95"]) if vb else "--"]) + r" \\")
    if g != "n61b":
        out.append(r"\addlinespace")
out += [r"\bottomrule", r"\end{tabular}"]
write("lp-arms", out)

# 4. routing band ----------------------------------------------------------------
ro = J("routing")
order = sorted(ro["bands"], key=lambda k: ro["bands"][k]["log2_r"])
out = [r"\begin{tabular}{@{}llrrrrrr@{}}", r"\toprule",
       r"curve ($\log_2r$) & arm & $w=0$ & $0.01$ & $0.03$ & $0.1$ & $0.3$ & $1$ \\", r"\midrule"]
for k in order:
    b = ro["bands"][k]
    first = True
    for a in ("rho", "pipeline", "pipeline71", "descent"):
        s = b["arms"][a]["mean_s"]
        cells = [f"{x:.3f}" if x is not None else "--" for x in s]
        out.append(" & ".join([f"{k} ({b['log2_r']:.2f})" if first else "", f"\\texttt{{{a}}}"] + cells) + r" \\")
        first = False
    if k != order[-1]:
        out.append(r"\addlinespace")
out += [r"\bottomrule", r"\end{tabular}"]
write("routing-band", out)

# 5. DLP tuning: band S and distance to rho -------------------------------------
dl = J("dlp")
dist = {r["n"]: r for r in dl["distance_to_rhofrob"]}
out = [r"\begin{tabular}{@{}llrrrlll@{}}", r"\toprule",
       r"$n$ & arm & $S$, $w=0$ & $0.1$ & $1$ & $/\rho_{\rm frob}$, $w=0$ & $w=0.1$ & $w=1$ \\", r"\midrule"]
for n in sorted(dl["bands"], key=int):
    b = dl["bands"][n]
    first = True
    for a, key in (("base-frozen", "base_over_rhofrob"), ("dlp-selected", "dlp_over_rhofrob"),
                   ("rho-frob", None), ("rho-strong", None)):
        s = b["arms"][a]["mean_s"]
        t = dist[int(n)]["text"]
        cells = (t[:3] if key == "base_over_rhofrob" else t[3:]) if key else ["", "", ""]
        out.append(" & ".join([f"{n} ({b['log2_r']:.1f})" if first else "", f"\\texttt{{{a}}}",
                               f"{s[0]:.3f}", f"{s[1]:.3f}", f"{s[2]:.3f}"] + cells) + r" \\")
        first = False
    if n != "47":
        out.append(r"\addlinespace")
out += [r"\bottomrule", r"\end{tabular}"]
write("dlp-band", out)

# 6. N83 chained S3, per cell ------------------------------------------------------
nc = J("n83chain")
ARM = {"unordered": "A0 unordered", "all-summands": "A1 order", "all-summands+t2.2": "A2 order+tables",
       "all-summands+affine+t2.2": "A3 A2 affine"}
out = [r"\begin{tabular}{@{}rlrrrrrr@{}}", r"\toprule",
       r" & & \multicolumn{3}{c}{public (3 per cell)} & \multicolumn{3}{c}{planted (2 per cell)} \\",
       r"\cmidrule(lr){3-5}\cmidrule(l){6-8}",
       r"$m$ & arm & refuted & med. confl. & med. s & witness & med. confl. & med. s \\", r"\midrule"]


def cell(rows, m, arm, kind):
    return next(r for r in rows if r["m"] == str(m) and r["arm"] == arm and r["kind"] == kind)


for study, ms, arms in (("order", (2, 3, 4), list(ARM)),
                        ("meet", (5, 6, 7, 8, 9), None)):
    for m in ms:
        al = arms or sorted({r["arm"] for r in nc[study] if r["m"] == str(m) and "meet" in r["arm"]})
        for a in al:
            pu, pl = cell(nc[study], m, a, "public"), cell(nc[study], m, a, "planted")
            lab = ARM.get(a, "A2+" + a.split("+")[-1])
            out.append(" & ".join([str(m), lab, f"{pu['refuted']}/{pu['queries']}",
                                   pu["median_conflicts_decided"] or "--", pu["median_seconds_decided"] or "--",
                                   f"{pl['witness']}/{pl['queries']}",
                                   pl["median_conflicts_decided"] or "--", pl["median_seconds_decided"] or "--"]) + r" \\")
        out.append(r"\addlinespace")
out[-1] = r"\bottomrule"
out.append(r"\end{tabular}")
write("n83chain", out)

# 7. guess closures per draw -------------------------------------------------------
gu = J("guess")
out = [r"\begin{tabular}{@{}llrrrrl@{}}", r"\toprule",
       r"study & guess set & draw & runs & refute at 4 & columns at 4 & s at degree 4, min--max \\", r"\midrule"]
SETS = {4: r"$\{7,15,23,24\}$", 6: r"$\{6,7,14,15,22,23\}$", 3: r"$\{7,15,23\}$", 0: "none ($\\ell=7$, N7)"}
groups = {}
for r in gu["runs"]:
    st = "guess" if r["study"].startswith("dreg_closure_guess_") else "guess3"
    groups.setdefault((st, len(r["fixed"]), r["draw"]), []).append(r)
for (st, k, dr), rs in sorted(groups.items(), key=lambda kv: (kv[0][0], -kv[0][1] if kv[0][1] != 6 else 0, kv[0][2])):
    ref = sum(1 for r in rs if r["deg4"]["closure_refuted"])
    cols = sorted({r["deg4"]["columns"] for r in rs})
    secs = [r["deg4"]["secs"] for r in rs]
    note = ""
    if ref < len(rs):
        note = ", rank " + ", ".join(num(r["deg4"]["closure_rank"]) for r in rs if not r["deg4"]["closure_refuted"])
    sr = f"{min(secs):.1f}" if len(secs) == 1 else f"{min(secs):.1f}--{max(secs):.1f}"
    out.append(" & ".join([st, SETS[k], str(dr), str(len(rs)), f"{ref}/{len(rs)}{note}",
                           "/".join(num(c) for c in cols), sr]) + r" \\")
out += [r"\bottomrule", r"\end{tabular}"]
write("guess", out)
print("ok")
