#!/usr/bin/env python3
"""Layout of paired.json (analyze.py) and analysis.json (genus_ladder_analyze) as tables.md and fig_ratio.svg."""
import json, math
P = json.load(open("paired.json")); A = json.load(open("analysis.json"))
C = P["cells"]; X = P["exponents"]
f = lambda v: f"{v:.3f}"
iv = lambda t: f"{t[0]:.3f} [{t[1]:.3f}, {t[2]:.3f}]"
sci = lambda v: f"{v:.3e}"
o = []; w = o.append
w("# Tables (`[pcc:unverified]`, from paired.json and analysis.json)\n")
w("Per prime: n runs per arm, mean cantor_mul, rel_mul, la_mul and FPM_mid per run, ratio to measured rho and to expected rho "
  "(hierarchical bootstrap, 4000 draws). Rows 8209-32771 are earlier studies (3 curves); 65537-262147 are this run (9 curves).\n")
w("| p | curves | arm | runs | cantor_mul | rel_mul | la_mul | FPM_mid | / measured rho | / expected rho |\n|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|")
for p, c in C.items():
    for k, nm in (("unm", "2LP+pre"), ("sge", "2LP+pre+SGE(8,3)")):
        a = c[k]
        w(f"| {p} | {c['n_curves']} | {nm} | {a['n_runs']} | {sci(a['cantor_mul'])} | {sci(a['rel_mul'])} | {sci(a['la_mul'])} | {sci(a['fpm'])} | {iv(a['ratio_rho'])} | {iv(a['ratio_exp'])} |")
    r = c["rho"]
    w(f"| {p} | {c['n_curves']} | rho (measured) | {r['n_runs']} | {sci(r['cantor_mul'])} | - | - | {sci(r['fpm'])} | 1 | - |")
    w(f"| {p} | {c['n_curves']} | rho (expected, 0.886 sqrt r) | - | - | - | - | {sci(r['exp_fpm'])} | {c['rho']['fpm']/r['exp_fpm']:.3f} (measured/expected) | 1 |")
w("\n## Rho reference inputs\n\n| p | cantor_mul / step | cantor_inv / step | mean measured steps | mean 0.886 sqrt(r) | measured / expected steps |\n|--:|--:|--:|--:|--:|--:|")
for p, c in C.items():
    w(f"| {p} | {c['cm_step']:.2f} | {c['ci_step']:.3f} | {c['rho_steps_mean']:.4g} | {c['exp_steps_mean']:.4g} | {c['rho_steps_mean']/c['exp_steps_mean']:.3f} |")
w("\n## Exponents of FPM_mid in p (log-log least squares on cell means; 95% bootstrap over curves)\n\n| arm | six primes 8209 ... 262147 | five primes 8209 ... 131101 |\n|---|--:|--:|")
for k, nm in (("rho", "rho (measured)"), ("sge", "2LP+pre+SGE(8,3)"), ("unm", "2LP+pre")):
    w(f"| {nm} | {iv(X['six_'+k])} | {iv(X['five_'+k])} |")
w(f"| rho (expected, descriptive, no interval) | {X['six_exp_rho']:.3f} | - |")
w("\n## Linear-algebra share of the SGE arm (9 la_mul / FPM_mid)\n\n| p | SGE arm | unmerged arm |\n|--:|--:|--:|")
for p, c in C.items(): w(f"| {p} | {c['sge']['la_share']:.3f} | {c['unm']['la_share']:.3f} |")
w("\n## Analyzer cross-check (genus_ladder_analyze, this run's rows)\n\n| p | arm | FPM_mid / rho | GAE w=1 / rho (unresolved band) |\n|--:|---|--:|--:|")
for c in A["cells"]:
    if "fpm_mid_over_rho" in c:
        r = c["fpm_mid_over_rho"]; g = c["gae_w1_over_rho"]
        w(f"| {c['p']} | {c['arm']} | {r['ratio']:.3f} [{r['lo']:.3f}, {r['hi']:.3f}] | {g['ratio']:.2f} [{g['lo']:.2f}, {g['hi']:.2f}] |")
open("tables.md", "w").write("\n".join(o) + "\n")

# ---- figure
SURF, INK, INK2, GRID = "#fcfcfb", "#0b0b0b", "#52514e", "#e4e3df"
BL, OR, GR, YE = "#2a78d6", "#eb6834", "#1baf7a", "#eda100"
ps = sorted(C, key=int); W, H, L, Rm, T, Bm = 760, 440, 70, 30, 50, 70
x = lambda p: L + (math.log(float(p)) - math.log(7000)) / (math.log(300000) - math.log(7000)) * (W - L - Rm)
y = lambda v: T + (0.9 - v) / 0.9 * (H - T - Bm)
s = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
     f'<rect width="{W}" height="{H}" fill="{SURF}"/>',
     f'<text x="{L}" y="24" font-size="15" font-weight="bold" fill="{INK}">Genus 3: 2LP + prefilter + SGE(8,3) over rho, FPM_mid (9 curves per prime from 65537) [pcc:unverified]</text>']
for v in (0, 0.2, 0.4, 0.6, 0.8):
    s.append(f'<line x1="{L}" x2="{W-Rm}" y1="{y(v):.1f}" y2="{y(v):.1f}" stroke="{GRID}"/><text x="{L-8}" y="{y(v)+4:.1f}" text-anchor="end" fill="{INK2}">{v:.1f}</text>')
for p in ps:
    s.append(f'<text x="{x(p):.1f}" y="{H-Bm+18}" text-anchor="middle" fill="{INK2}">{p}</text>')
s.append(f'<text x="{(L+W-Rm)/2}" y="{H-Bm+42}" text-anchor="middle" fill="{INK2}">prime p (log scale); open markers: 3 curves, filled: 9 curves</text>')
s.append(f'<text x="18" y="{(T+H-Bm)/2}" transform="rotate(-90 18 {(T+H-Bm)/2})" text-anchor="middle" fill="{INK2}">FPM_mid ratio (SGE arm / rho)</text>')
def series(key, col, dx, label, ly):
    pts = []
    for p in ps:
        t = C[p]["sge"][key]; filled = C[p]["n_curves"] == 9; X0 = x(int(p)) + dx
        s.append(f'<line x1="{X0:.1f}" x2="{X0:.1f}" y1="{y(t[1]):.1f}" y2="{y(t[2]):.1f}" stroke="{col}" stroke-width="2"/>')
        s.append(f'<circle cx="{X0:.1f}" cy="{y(t[0]):.1f}" r="5" fill="{col if filled else SURF}" stroke="{col}" stroke-width="2"/>')
        pts.append(f"{X0:.1f},{y(t[0]):.1f}")
    s.append(f'<polyline points="{" ".join(pts)}" fill="none" stroke="{col}" stroke-width="1.5" stroke-dasharray="4 3"/>')
    s.append(f'<rect x="{L+10}" y="{ly-9}" width="12" height="12" fill="{col}"/><text x="{L+28}" y="{ly+2}" fill="{INK}">{label}</text>')
series("ratio_rho", BL, -6, "SGE / measured rho (hierarchical bootstrap 95%)", T+10)
series("ratio_exp", OR, 6, "SGE / expected rho (0.886 sqrt(r) steps x measured cost per step)", T+30)
s.append("</svg>")
open("fig_ratio.svg", "w").write("\n".join(s) + "\n")
