"""Generates the numeric tables of RESULTS.md and plot.svg from raw/*.jsonl and raw/analysis.json."""
import glob, json, math, os
from collections import defaultdict

raw = "raw"
A = json.load(open(os.path.join(raw, "analysis.json")))
rel = []
for f in sorted(glob.glob(os.path.join(raw, "rel_*.jsonl"))):
    rel += [json.loads(l) for l in open(f) if l.strip()]
cell = defaultdict(lambda: defaultdict(list))
for r in rel:
    cell[(r["g"], r["p"])][r["arm"]].append(r)


def mean(xs):
    return sum(xs) / len(xs)


def gm(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


L = []
L.append("### Per-cell mean counts per run (6 runs per cell = 3 curves x 2 solver seeds); FPM/rel as geometric mean (min-max)\n")
L.append("#### P_full\n")
L.append("| g | p | n_cols | N_target | relations | dup | tuples | mul | inv | add | fd_add | bucket_ops | bucket_scan | FPM | FPM/rel GM (min-max) | row wt mean | max col wt | cols unhit |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for (g, p) in sorted(cell):
    rs = cell[(g, p)]["P_full"]
    fr = [r["fpm_per_rel"] for r in rs]
    L.append("| %d | %d | %.1f | %.1f | %.1f | %.1f | %.1f | %.0f | %.1f | %.0f | %.0f | %.0f | %.0f | %.0f | %.3f (%.3f-%.3f) | %.3f | %.1f | %.1f |" % (
        g, p, mean([r["n_cols"] for r in rs]), mean([r["n_target"] for r in rs]), mean([r["relations"] for r in rs]),
        mean([r["duplicates"] for r in rs]), mean([r["tuples"] for r in rs]), mean([r["mul"] for r in rs]),
        mean([r["inv"] for r in rs]), mean([r["add"] for r in rs]), mean([r["fd_add"] for r in rs]),
        mean([r["bucket_ops"] for r in rs]), mean([r["bucket_scan"] for r in rs]), mean([r["fpm"] for r in rs]),
        gm(fr), min(fr), max(fr), mean([r["row_weight_mean"] for r in rs]), mean([r["col_weight_max"] for r in rs]),
        mean([r["cols_unhit"] for r in rs])))
L.append("\n#### B_lab\n")
L.append("| g | p | relations | duplicates (sum of 6) | trials | candidates | splits | walk_add | restarts | cantor_mul | cantor_inv | rel_mul | rel_inv | FPM | FPM/rel GM (min-max) | strong_fpm/rel GM (g=2) | row wt mean | max col wt | cols unhit |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for (g, p) in sorted(cell):
    rs = cell[(g, p)]["B_lab"]
    fr = [r["fpm_per_rel"] for r in rs]
    sf = ("%.3f" % gm([r["strong_fpm"] / r["relations"] for r in rs])) if g == 2 else "n/a"
    L.append("| %d | %d | %.1f | %d | %.0f | %.0f | %.0f | %.0f | %.1f | %.0f | %.0f | %.0f | %.0f | %.0f | %.3f (%.3f-%.3f) | %s | %.3f | %.1f | %.1f |" % (
        g, p, mean([r["relations"] for r in rs]), sum(r["duplicates"] for r in rs),
        mean([r["trials"] for r in rs]), mean([r["candidates"] for r in rs]), mean([r["splits"] for r in rs]),
        mean([r["walk_add"] for r in rs]), mean([r["restarts"] for r in rs]),
        mean([r["cantor_mul"] for r in rs]), mean([r["cantor_inv"] for r in rs]),
        mean([r["rel_mul"] for r in rs]), mean([r["rel_inv"] for r in rs]), mean([r["fpm"] for r in rs]),
        gm(fr), min(fr), max(fr), sf, mean([r["row_weight_mean"] for r in rs]),
        mean([r["col_weight_max"] for r in rs]), mean([r["cols_unhit"] for r in rs])))
L.append("\n#### P_naive1 (first tuple only, every lambda in F_p)\n")
L.append("| g | p | relations (first tuple) | mul | inv | FPM | FPM/rel GM | relset_equal (all 6) |")
L.append("|---|---|---|---|---|---|---|---|")
for (g, p) in sorted(cell):
    rs = cell[(g, p)]["P_naive1"]
    L.append("| %d | %d | %.1f | %.0f | %.1f | %.0f | %.3f | %s |" % (
        g, p, mean([r["relations"] for r in rs]), mean([r["mul"] for r in rs]), mean([r["inv"] for r in rs]),
        mean([r["fpm"] for r in rs]), gm([r["fpm_per_rel"] for r in rs]), all(r["relset_equal"] for r in rs)))
L.append("\n#### B_lab runs with duplicates > 0 (all cells)\n")
for r in rel:
    if r["arm"] == "B_lab" and r["duplicates"] > 0:
        L.append("- g=%d p=%d curve %d seed %d: relations %d, duplicates %d, trials %d, restarts %d, FPM/rel %.1f" % (
            r["g"], r["p"], r["curve_index"], r["solver_seed"], r["relations"], r["duplicates"], r["trials"],
            r["restarts"], r["fpm_per_rel"]))
L.append("\n#### Split-rate check reported only: B_lab splits/candidates divided by C(n,g) 2^g / p^g (mean over 6)\n")
for (g, p) in sorted(cell):
    rs = cell[(g, p)]["B_lab"]
    L.append("- g=%d p=%d: %.4f" % (g, p, mean([r["split_rate_model_ratio"] for r in rs])))
open("raw/results_tables.md", "w").write("\n".join(L) + "\n")

# ---------- plot.svg ----------
W, H = 900, 420
def panel(ox, g, title):
    ps = [1031, 4099, 16411, 65537]
    series = {"P_full": [], "B_lab": [], "B_strong": []}
    for p in ps:
        rs = cell[(g, p)]
        series["P_full"].append(gm([r["fpm_per_rel"] for r in rs["P_full"]]))
        series["B_lab"].append(gm([r["fpm_per_rel"] for r in rs["B_lab"]]))
        if g == 2:
            series["B_strong"].append(gm([r["strong_fpm"] / r["relations"] for r in rs["B_lab"]]))
    if g != 2:
        del series["B_strong"]
    lo, hi = math.log10(5), math.log10(6000)
    pw, ph, px, py = 360, 300, ox + 60, 50
    out = ['<text x="%d" y="30" font-size="15" font-family="sans-serif" font-weight="bold">%s</text>' % (px, title)]
    out.append('<rect x="%d" y="%d" width="%d" height="%d" fill="none" stroke="#888"/>' % (px, py, pw, ph))
    for t in (10, 100, 1000):
        y = py + ph * (1 - (math.log10(t) - lo) / (hi - lo))
        out.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" stroke="#ddd"/>' % (px, y, px + pw, y))
        out.append('<text x="%d" y="%.1f" font-size="11" font-family="sans-serif" text-anchor="end">%d</text>' % (px - 5, y + 4, t))
    xs = [px + 40 + i * (pw - 80) / 3 for i in range(4)]
    for x, p in zip(xs, ps):
        out.append('<text x="%.1f" y="%d" font-size="11" font-family="sans-serif" text-anchor="middle">%d</text>' % (x, py + ph + 16, p))
    out.append('<text x="%d" y="%d" font-size="11" font-family="sans-serif" text-anchor="middle">p</text>' % (px + pw // 2, py + ph + 34))
    cols = {"P_full": "#1b6ca8", "B_lab": "#c0392b", "B_strong": "#7d8c1f"}
    for k, vals in series.items():
        pts = " ".join("%.1f,%.1f" % (x, py + ph * (1 - (math.log10(v) - lo) / (hi - lo))) for x, v in zip(xs, vals))
        out.append('<polyline points="%s" fill="none" stroke="%s" stroke-width="2"/>' % (pts, cols[k]))
        for x, v in zip(xs, vals):
            y = py + ph * (1 - (math.log10(v) - lo) / (hi - lo))
            out.append('<circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (x, y, cols[k]))
            out.append('<text x="%.1f" y="%.1f" font-size="10" font-family="sans-serif" text-anchor="middle" fill="%s">%.1f</text>' % (x, y - 7, cols[k], v))
    ly = py + ph + 55
    for i, k in enumerate(series):
        out.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d" font-size="12" font-family="sans-serif">%s</text>' % (px + i * 120, ly, cols[k], px + i * 120 + 17, ly + 11, k))
    return out
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" viewBox="0 0 %d %d">' % (W, H, W, H),
       '<rect width="100%" height="100%" fill="white"/>']
svg += panel(0, 2, "g = 2: FPM per distinct relation (geometric mean of 6 runs, log scale)")
svg += panel(450, 3, "g = 3: FPM per distinct relation (geometric mean of 6 runs, log scale)")
svg.append('</svg>')
open("plot.svg", "w").write("\n".join(svg))
