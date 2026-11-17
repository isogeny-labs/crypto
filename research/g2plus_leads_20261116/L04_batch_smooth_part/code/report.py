"""Writes RESULTS.md tables and plot.svg from raw/analysis.json (numbers only)."""
import json
D = json.load(open("raw/analysis.json"))
order = ["A_pf", "A_nopf", "A_pf_aa", "B", "B_pf"] + ["C_m%d" % m for m in (1, 4, 16, 64, 256, 1024)] + ["Cpf_m%d" % m for m in (1, 4, 16, 64, 256, 1024)]
cells = ["main_g3_p65521", "main_g3_p262139", "main_g3_p1048573", "pc_g3_p65521_N8", "sec_g2_p65521", "sec_g2_p1048573"]
L = []
L.append("## Verdict string (mechanical, `analyze.py`)\n\n%s  (fpm_lab variant: %s)\n" % (D["verdict"], D["verdict_lab"]))
L.append("## Gates\n\n| gate | result | detail |\n|---|---|---|")
for k in sorted(D["gates"]):
    g = D["gates"][k]
    L.append("| %s | %s | %s |" % (k, "pass" if g["pass"] else "FAIL", g.get("failing", g.get("R_pc"))))
L.append("\n## Per-cell ratio R = best A / best batch (fpm_total_per_cand)\n")
L.append("| cell | g | p | N | best A | best batch | R | R_lab | N* = A_best/(2g) | B dec mul/cand | / g(N-g+1) | setup FPM |\n|---|---|---|---|---|---|---|---|---|---|---|---|")
for c in cells:
    r = D["cells"][c]
    L.append("| %s | %d | %d | %d | %s %.2f | %s %.2f | %.4f | %.4f | %.2f | %.2f | %.4f | %d |" % (
        c, r["g"], r["p"], r["N"], r["A_best"][0], r["A_best"][1], r["batch_best"][0], r["batch_best"][1],
        r["R"], r["R_lab"], r["N_star"], r["B_dec_mul_per_cand"], r["B_dec_mul_per_cand_over_pred"], r["setup_fpm"]))
for c in cells:
    S = D["secondary"][c]
    r = D["cells"][c]
    L.append("\n### %s (g=%d, p=%d, N=%d, T_cell=%d)\n" % (c, r["g"], r["p"], r["N"], S["A_pf"]["T_cell"]))
    L.append("| arm | pre | dec | roots | total | total_lab | accepted | accept frac |\n|---|---|---|---|---|---|---|---|")
    for a in order:
        if a in S:
            x = S[a]
            L.append("| %s | %.2f | %.2f | %.2f | %.2f | %.2f | %d | %.4f |" % (a, x["fpm_pre_per_cand"], x["fpm_dec_per_cand"],
                     x["fpm_roots_per_cand"], x["fpm_total_per_cand"], x["fpm_lab_total_per_cand"], x["accepted"], x["accept_frac"]))
open("RESULTS_tables.md", "w").write("\n".join(L) + "\n")

# SVG: grouped log-scale bars of fpm_total_per_cand, min over C_m* / Cpf_m* shown as C_best / Cpf_best
import math
groups = ["A_pf", "A_nopf", "B", "B_pf", "Cbest", "Cpfbest"]
cols = {"A_pf": "#1f77b4", "A_nopf": "#6baed6", "B": "#d62728", "B_pf": "#ff9896", "Cbest": "#2ca02c", "Cpfbest": "#98df8a"}
vals = {}
for c in cells:
    S = D["secondary"][c]
    v = {a: S[a]["fpm_total_per_cand"] for a in ("A_pf", "A_nopf", "B", "B_pf")}
    v["Cbest"] = min(S[a]["fpm_total_per_cand"] for a in S if a.startswith("C_m"))
    v["Cpfbest"] = min(S[a]["fpm_total_per_cand"] for a in S if a.startswith("Cpf_m"))
    vals[c] = v
W, H, ml, mb, mt = 980, 460, 70, 110, 40
lo, hi = 1.0, 5.0  # log10 range 10^1 .. 10^5
def y(v): return mt + (H - mt - mb) * (1 - (math.log10(v) - lo) / (hi - lo))
s = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="11">' % (W, H),
     '<rect width="100%" height="100%" fill="white"/>',
     '<text x="%d" y="22" font-size="14">L04: FPM per candidate by arm (log scale; batch arms: best m) [pcc:unverified]</text>' % ml]
for e in range(1, 6):
    yy = y(10 ** e)
    s.append('<line x1="%d" x2="%d" y1="%.1f" y2="%.1f" stroke="#ddd"/><text x="%d" y="%.1f" text-anchor="end">1e%d</text>' % (ml, W - 10, yy, yy, ml - 5, yy + 4, e))
gw = (W - ml - 10) / len(cells)
bw = gw / (len(groups) + 1)
for i, c in enumerate(cells):
    x0 = ml + i * gw
    for j, g in enumerate(groups):
        v = vals[c][g]
        yy = y(v)
        s.append('<rect x="%.1f" y="%.1f" width="%.1f" height="%.1f" fill="%s"/>' % (x0 + (j + 0.5) * bw, yy, bw * 0.9, H - mb - yy, cols[g]))
        s.append('<text x="%.1f" y="%.1f" font-size="8" text-anchor="middle">%.0f</text>' % (x0 + (j + 0.95) * bw, yy - 2, v))
    s.append('<text x="%.1f" y="%d" text-anchor="middle">%s</text>' % (x0 + gw / 2, H - mb + 16, c))
for j, g in enumerate(groups):
    s.append('<rect x="%d" y="%d" width="10" height="10" fill="%s"/><text x="%d" y="%d">%s</text>' % (ml + j * 140, H - 40, cols[g], ml + j * 140 + 14, H - 31, g))
s.append("</svg>")
open("plot.svg", "w").write("\n".join(s))
