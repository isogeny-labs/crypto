#!/usr/bin/env python3
"""Lay out data-falldegree-instances.json as LaTeX tables (no computation
beyond formatting). Writes tab-fd-binary.tex, tab-fd-kw.tex, tab-fd-chain.tex."""
import json, os
H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-falldegree-instances.json")))

def num(x):
    return "" if x is None else f"{x:,}".replace(",", "{,}")

def dF(r):
    c = r["dF_counts"]
    if len(c) == 1:
        v = next(iter(c))
        return f"$\\ge{v[2:]}$" if v.startswith(">=") else v
    return ", ".join(f"{k}$\\times${n}" for k, n in sorted(c.items()))

LABEL = {"P": "P", "S": "S", "R": "R", "U": "U", "T": "T"}
out = [r"\begin{tabular}{@{}clrrrrrrrr@{}}", r"\toprule",
       r"$m$ & $V$ & $n$ & $k$ & inst. & $D_{\rm ff}$ & $d_{\tilde F}$ & cols at $D=4$ & s at $D=4$ & codim $W_4\ge$ \\",
       r"\midrule"]
prev = None
for r in D["m2_rows"]:
    fam = r["family"]
    if prev is not None and fam != prev:
        out.append(r"\addlinespace")
    first = "2" if prev is None else ""
    out.append(" & ".join([first, LABEL[fam] if fam != prev else "", str(r["n"]), str(r["k"]),
                           str(r["instances"]), "2", dF(r), num(r["ncols_D4"]),
                           "" if r["seconds_D4_median"] is None else f"{r['seconds_D4_median']:.1f}",
                           num(r["cert_codim_W4"][0]) if r["cert_codim_W4"] else ""]) + r" \\")
    prev = fam
out += [r"\bottomrule", r"\end{tabular}"]
open(os.path.join(H, "tab-fd-binary.tex"), "w").write("\n".join(out) + "\n")

kw = D["kousidis_wiemers_2019_table1"]["m2"]
ours = {}
for r in D["m2_rows"]:
    ours.setdefault(r["n"], {})[r["family"]] = dF(r)
out = [r"\begin{tabular}{@{}rrrrrrcc@{}}", r"\toprule",
       r"\multicolumn{6}{c}{Kousidis--Wiemers, Table 1 ($m=2$, F4)} & \multicolumn{2}{c}{this work, $d_{\tilde F}$} \\",
       r"\cmidrule(r){1-6}\cmidrule(l){7-8}",
       r"$n$ & $n'$ & $D_{\rm ff}$ & $D_{\rm reg}$ & s & GB & P or S & R, U or T \\", r"\midrule"]
for i, n in enumerate(kw["n"]):
    o = ours.get(n, {})
    poly = o.get("P") or (o.get("S") and o["S"] + "$^{\\rm S}$") or ""
    rnd = "; ".join(f"{v}$^{{\\rm {f}}}$" for f in "RUT" if (v := o.get(f)))
    out.append(f"{n} & {kw['n_prime'][i]} & {kw['D_ff'][i]} & {kw['D_reg'][i]} & {num(kw['s'][i])} & {kw['GB'][i]:.1f} & {poly} & {rnd} \\\\")
out += [r"\bottomrule", r"\end{tabular}"]
open(os.path.join(H, "tab-fd-kw.tex"), "w").write("\n".join(out) + "\n")

out = [r"\begin{tabular}{@{}lccrrlrrr@{}}", r"\toprule",
       r"instance & $m=t$ & $V$ & $n$ & $k$ & $d_{\tilde F}$ & cols at $D=4$ & s at $D=4$ & codim $W_4\ge$ \\", r"\midrule"]
for r in sorted(D["chain_m3"], key=lambda r: (r["basis"] != "poly", r["n"], r["name"])):
    d = r["dF"].replace(">=", "$\\ge$").replace("<=", "$\\le$")
    out.append(f"{r['name']} & 3 & {'P' if r['basis']=='poly' else 'R'} & {r['n']} & {r['k']} & {d} & {num(r['ncols_D4'])} & {r['seconds_D4']:.1f} & {num(r['cert_codim_W4'])} \\\\")
out += [r"\bottomrule", r"\end{tabular}"]
open(os.path.join(H, "tab-fd-chain.tex"), "w").write("\n".join(out) + "\n")
print("ok")

# Prime-field Kummer towers, m=2 (values from data-new.json).
K = json.load(open(os.path.join(H, "data-new.json")))["prime_kummer_m2"]
out = [r"\begin{tabular}{@{}rrrr@{}}", r"\toprule",
       r"$t$ (PKM: $n_1$) & $D_{\rm ff}$ (proved) & $d_F$ (session median) & PKM $D_{\rm reg}^{\rm corr}$ \\", r"\midrule"]
for i, n1 in enumerate(K["pkm_n1"]):
    t = n1
    j = K["t"].index(t) if t in K["t"] else None
    ffd = "" if j is None or K["ffd_proved"][j] is None else str(K["ffd_proved"][j])
    dF = "" if j is None else str(K["dF_session_reported"][j])
    out.append(f"{t} & {ffd} & {dF} & {int(K['pkm_Dcorr'][i])} \\\\")
out += [r"\bottomrule", r"\end{tabular}"]
open(os.path.join(H, "tab-fd-pkm.tex"), "w").write("\n".join(out) + "\n")
