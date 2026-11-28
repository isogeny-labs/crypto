"""Leaderboard Table A (rule v9) as a LaTeX tabular, from data-u16front-boardA.json only.

    python3 make_tab_u16front_boardA.py   # writes tab-u16front-boardA.tex

Numbers are copied from the JSON and only rounded for display.
"""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
D = json.load(open(os.path.join(HERE, "data-u16front-boardA.json")))


def f(x, d=2):
    return f"{x:.{d}f}"


def ci(c, d=2):
    return f"[{c[0]:.{d}f},{c[1]:.{d}f}]"


L = ["\\begin{tabular}{@{}lrrrlrlrr@{}}", "\\toprule",
     "row & $\\log_2 r$ & $S_{\\rm IC}$ & $S_\\rho$ & IC/$\\rho$ [95\\%] & IC/$\\rho$, v7 & "
     "online speed-up [95\\%] & break-even & canon.\\ step \\\\",
     "\\midrule"]
for r in D["rows"]:
    L.append(f"{r['slug'].split('-')[1]} & {f(r['log2_r'])} & {f(r['s_ic'])} & {f(r['s_rho'], 3)} & "
             f"{f(r['ratio_rho'])} {ci(r['ratio_rho_ci'])} & {f(r['ratio_rho_rule_v7'])} & "
             f"{f(r['online_speedup'], 1)} {ci(r['online_speedup_ci'], 1)} & {f(r['break_even_targets'])} & "
             f"{f(r['canonical_step_units'])} \\\\")
L += ["\\bottomrule", "\\end{tabular}"]
open(os.path.join(HERE, "tab-u16front-boardA.tex"), "w").write("\n".join(L) + "\n")
print("wrote tab-u16front-boardA.tex")
