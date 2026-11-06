#!/usr/bin/env python3
"""Lay out data-bin-koblitz-online.json as tab-bin-koblitz-online.tex.
Formatting only: every number is copied from the data file and printed to at
most five significant figures in panel (a), four in panel (b) (three for its intervals) and three
for the operation-count band and the no-table column, rounded half-up."""
import json, math, os
from decimal import Decimal, ROUND_HALF_UP

H = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(H, "data-bin-koblitz-online.json")))


def sig(x, s=4):
    """x to s significant figures, half-up on its decimal repr; never more digits than the repr."""
    if x is None:
        return "--"
    if x == 0:
        return "0"
    e = math.floor(math.log10(abs(x)))
    if x < 0.01:
        m = Decimal(repr(x / 10 ** e)).quantize(Decimal(1).scaleb(-(s - 1)), ROUND_HALF_UP)
        return f"${m}\\cdot10^{{{e}}}$"
    d = max(s - 1 - e, 0)
    q = Decimal(repr(x))
    q = q.quantize(Decimal(1).scaleb(-d), ROUND_HALF_UP) if -q.as_tuple().exponent > d else q
    return f"{q:,}".replace(",", "{,}")


def iv(v, s=4):
    return "--" if not v else f"[{sig(v[0], s)}, {sig(v[1], s)}]"


def mb(b):
    return f"{b / 1e6:.1f}\\,MB" if b < 1e9 else f"{b / 1e9:.2f}\\,GB"


# Short labels for the layout only (the data rows carry the full description).
LA = {"n61-K600-R4": ("61", "600 col., R4"), "n71-K600-R2": ("71", "600 col., R2"),
      "n73-K600-R3": ("73", "600 col., R3"), "n83-a1-K600-R1": ("83", "$a=1$, 600 col., R1"),
      "n83-K1798-dense": ("83", "dense root, 1{,}798 col."),
      "n61-K500-x86_64": ("61", "$K=500$, x86\\_64 pinned"), "n61-K500-arm64": ("61", "$K=500$, arm64"),
      "n61-K800-arm64": ("61", "$K=800$, arm64"), "n71-K600-arm64": ("71", "$K=600$, arm64")}
# Standing codes, explained in the caption: e exploratory, c contradicted as a
# win by the validation sweep, r admitted then retired as a win, u producer-reported,
# q pending independent validation. Panel (b): n <= 61 validated by the sweep (74.1),
# n = 83 validated with caveats (24.1); stated in the caption.
ST = {"n61-K600-R4": "e", "n71-K600-R2": "e", "n73-K600-R3": "e", "n83-a1-K600-R1": "e",
      "n83-K1798-dense": "c", "n61-K500-x86_64": "r", "n61-K500-arm64": "r", "n61-K800-arm64": "u",
      "n71-K600-arm64": "q"}

L = [r"(a) Rho with no table, same public point: rho/IC online ($>1$: IC faster online)\\[2pt]",
     r"\begin{tabular}{@{}rlrrrrll@{}}",
     r"\toprule",
     r"$n$ & IC base, run & targets & IC ms & rho ms & rho/IC [95\%] & range & standing \\", r"\midrule"]
for r in D["a_tableless_rho"]:
    n, lab = LA[r["id"]]
    tg = f"1 ($\\times{r['runs']}$)" if r["points"] == 1 and r["runs"] > 1 else str(r["points"])
    ratio = sig(r["rho_over_ic"], 5) + (" " + iv(r["ci95"], 5) if r.get("ci95") else "")
    rng = "--" if not r["range"] else f"{sig(r['range'][0], 3)}--{sig(r['range'][1], 3)}"
    L.append(f"{n} & {lab} & {tg} & {sig(r['ic_online_ms'], 5)} & {sig(r['rho_online_ms'], 5)} & {ratio} & {rng} & {ST[r['id']]} \\\\")
L += [r"\bottomrule", r"\end{tabular}", "", r"\medskip", ""]

L += [r"(b) Rho given a Bernstein--Lange table of equal or smaller advice: IC/rho online ($>1$: rho faster online)\\[2pt]",
      r"\begin{tabular}{@{}rllrrrlll@{}}",
      r"\toprule",
      r"$n$ & IC base & rho table & tgts & IC ms & rho ms & IC/rho [95\%] & band $w=0$; $0.1$; $1$ & no-table rho/IC \\",
      r"\midrule"]
for r in D["b_equal_advice_rho"]:
    if r["n"] < 83:
        tab = f"{mb(r['rho_table_bytes'])} ({r['advice_vs_ic']})"
        base = f"{r['ic_base']} ({mb(r['ic_advice_bytes'])})"
        stat = "$^{m}$"
        ic_ms, rho_ms = r["ic_online_ms_mean"], r["rho_online_ms_mean"]
    else:
        kind = {"bl_a2_wall_equal": "time", "bl_b_memory_matched": "mem.", "bl_a_time_matched": "650\\,s"}
        k = [v for key, v in kind.items() if key in r["id"]][0]
        tab = f"{mb(r['rho_table_bytes'])}, {k}, r{r['run']}"
        base = r["ic_base"]
        stat = "$^{p}$"
        ic_ms, rho_ms = r["ic_online_ms_median"], r["rho_online_ms_median"]
    band = "--" if not r["band"] else "; ".join(sig(b["median_ratio"], 3) for b in r["band"])
    L.append(f"{r['n']} & {base} & {tab} & {r['targets']} & {sig(ic_ms)} & "
             f"{sig(rho_ms)} & {sig(r['ic_over_rho'])}{stat} {iv(r['ci95'], 3)} & {band} & "
             f"{sig(r['plain_rho_over_ic'], 3)} {iv(r['plain_rho_over_ic_ci95'], 3)} \\\\")
L += [r"\bottomrule", r"\end{tabular}"]
open(os.path.join(H, "tab-bin-koblitz-online.tex"), "w").write("\n".join(L) + "\n")
