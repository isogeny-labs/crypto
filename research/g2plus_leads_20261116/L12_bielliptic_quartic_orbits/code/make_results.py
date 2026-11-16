"""Builds RESULTS.md tables and plot.svg from raw/ (numbers only)."""
import json, os, re, statistics, math, collections
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RUNS = os.path.join(ROOT, "raw", "runs")
pat = re.compile(r"^(F1|F0)_p(\d+)_j(\d+)_s(\d+)_(A|B|C|Bp|N)\.json$")
runs = {}
for fn in os.listdir(RUNS):
    m = pat.match(fn)
    if m:
        d = json.load(open(os.path.join(RUNS, fn)))
        runs[(m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4)), m.group(5))] = d
dec = json.load(open(os.path.join(ROOT, "raw", "decision.json")))
med = statistics.median
def f(x):
    if x is None: return "NA"
    if isinstance(x, float): return "%.4g" % x
    return str(x)
L = []
L.append("## 1. Per (family, p, arm) medians over runs (counts; `n` = number of runs, `ok` = status ok)\n")
L.append("| family | p | arm | n | ok | ncols | trials | relations | duplicate_rows | rows_used | W | la_attempts (med / max) | cert_rejected_attempts (sum) | SETUP_FPM | COLLECT_FPM | LA_FPM | TOTAL |")
L.append("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for fam, primes, arms in (("F1", [263, 523, 1031, 2063], "ABCBpN"), ("F0", [269, 521, 1033, 2069], "ABC")):
    for p in primes:
        for arm in ["A", "B", "C", "Bp", "N"]:
            if arm not in (["A","B","C","Bp","N"] if fam=="F1" else ["A","B","C"]): continue
            ds = [d for k, d in runs.items() if k[0]==fam and k[1]==p and k[4]==arm]
            if not ds: continue
            ok = [d for d in ds if d.get("status") == "ok"]
            use = ds
            def m(key): return med([d[key] for d in use if d.get(key) is not None])
            L.append("| %s | %d | %s | %d | %d | %s | %s | %s | %s | %s | %s | %s / %s | %d | %s | %s | %s | %s |" % (
                fam, p, arm, len(ds), len(ok), f(m("ncols")), f(m("trials")), f(m("relations")), f(m("duplicate_rows")),
                f(m("rows_used")), f(m("W")), f(m("la_attempts")), max(d["la_attempts"] for d in ds),
                sum(d.get("cert_rejected_attempts", 0) for d in ds),
                f(m("SETUP_FPM")), f(m("COLLECT_FPM")), f(m("LA_FPM")), f(m("TOTAL"))))
L.append("\n(All arms: medians are over all runs of the cell including non-ok runs; the N rows are la_fail runs.)\n")
L.append("## 2. Paired ratios per (family, p): median [min, max], n pairs used, excluded\n")
rs = dec["ratio_stats"]
names = ["TOTAL_A/TOTAL_B","COLLECT_A/COLLECT_B","LA_A/LA_B","rel_A/rel_B","ncols_A/ncols_B","TOTAL_B/TOTAL_C","TOTAL_A/TOTAL_C","TOTAL_B/TOTAL_Bp"]
for fam, primes in (("F1", [263,523,1031,2063]), ("F0", [269,521,1033,2069])):
    use = [n for n in names if not (fam == "F0" and "Bp" in n)]
    L.append("### %s\n" % fam)
    L.append("| ratio | " + " | ".join(str(p) for p in primes) + " |")
    L.append("|---|" + "---|" * len(primes))
    for n in use:
        cells = []
        for p in primes:
            s = rs["%s|%d|%s" % (fam, p, n)]
            cells.append("%s [%s, %s] n=%d x=%d" % (f(s["median"]), f(s["min"]), f(s["max"]), s["n"], s["excluded"]))
        L.append("| %s | " % n + " | ".join(cells) + " |")
    L.append("")
L.append("## 3. The eight TOTAL_A/TOTAL_B values at p = 2063 and the eight TOTAL_B/TOTAL_B' (A/A) values\n")
L.append("- TOTAL_A/TOTAL_B: " + ", ".join(f(v) for v in rs["F1|2063|TOTAL_A/TOTAL_B"]["values"]))
L.append("- TOTAL_B/TOTAL_B': " + ", ".join(f(v) for v in dec["AA_2063_values"]))
L.append("- count of TOTAL_A/TOTAL_B >= 1.5 at 2063: %d of %d; A/A max/min: %s\n" % (dec["count_ge_1.5_at_2063"], dec["n_values_2063"], f(dec["AA_2063_spread"])))
L.append("## 4. Positive control (F0, p = 2069)\n")
pc = dec["positive_control_2069"]
L.append("- median TOTAL_A/TOTAL_C = %s, median TOTAL_A/TOTAL_B = %s; frozen pass rule (>=6 and >=2) evaluates to %s\n" % (f(pc["median_TOTAL_A_over_C"]), f(pc["median_TOTAL_A_over_B"]), pc["pass"]))
L.append("## 5. Gap to the matched rho reference (TOTAL_X / rho median FPM, per curve; orientation only)\n")
L.append("| p | j | rho median FPM (5 walks) | rho all correct | gap A | gap B | gap C |")
L.append("|---|---|---|---|---|---|---|")
for p in [263,523,1031,2063]:
    for j in range(4):
        L.append("| %d | %d | %s | %s | %s | %s | %s |" % (p, j, f(dec["rho_median_fpm"].get("%d|%d"%(p,j))), dec["rho_all_correct"].get("%d|%d"%(p,j)),
                 *[f(dec["gap_TOTAL_over_rho"].get("%d|%d|%s"%(p,j,a))) for a in "ABC"]))
L.append("\nPer-p median over curves of the gap (median of the per-curve gaps):\n")
L.append("| p | gap A | gap B | gap C |")
L.append("|---|---|---|---|")
for p in [263,523,1031,2063]:
    row = []
    for a in "ABC":
        vs = [dec["gap_TOTAL_over_rho"].get("%d|%d|%s"%(p,j,a)) for j in range(4)]
        vs = [v for v in vs if v is not None]
        row.append(f(med(vs)))
    L.append("| %d | %s |" % (p, " | ".join(row)))
L.append("\n## 6. Relation yield per trial: F1 arm A vs generic quartic G\n")
L.append("| p | F1 split/trials | F1 yield | G split/trials | G yield | abs diff |")
L.append("|---|---|---|---|---|---|")
for p in [263,523,1031,2063]:
    y = dec["yields"][str(p)]
    L.append("| %d | %d/%d | %s | %d/%d | %s | %s |" % (p, y["F1_split"], y["F1_trials"], f(y["F1_A_yield"]), y["G_split"], y["G_trials"], f(y["G_yield"]), f(abs(y["F1_A_yield"]-y["G_yield"]))))
# plot
series = collections.OrderedDict()
for arm in ["A", "B", "C", "Bp"]:
    pts = []
    for p in [263, 523, 1031, 2063]:
        v = [d["TOTAL"] for k, d in runs.items() if k[0]=="F1" and k[1]==p and k[4]==arm and d.get("status")=="ok"]
        if v: pts.append((p, med(v)))
    series[arm] = pts
W, H, ml, mb, mt, mr = 640, 420, 80, 60, 40, 130
xs = [math.log10(p) for p in [263, 523, 1031, 2063]]
allv = [v for s in series.values() for _, v in s]
ylo, yhi = math.floor(math.log10(min(allv))), math.ceil(math.log10(max(allv)))
def X(p): return ml + (math.log10(p) - xs[0]) / (xs[-1] - xs[0]) * (W - ml - mr)
def Y(v): return H - mb - (math.log10(v) - ylo) / (yhi - ylo) * (H - mt - mb)
col = {"A": "#1f77b4", "B": "#d62728", "C": "#2ca02c", "Bp": "#7f7f7f"}
svg = ['<svg xmlns="http://www.w3.org/2000/svg" width="%d" height="%d" font-family="sans-serif" font-size="12">' % (W, H),
       '<rect width="100%" height="100%" fill="white"/>',
       '<text x="%d" y="22" font-size="14">L12 F1: median TOTAL (FPM) per arm vs p (log-log; counts, no wall time)</text>' % ml]
for e in range(ylo, yhi + 1):
    svg.append('<line x1="%d" y1="%.1f" x2="%d" y2="%.1f" stroke="#ddd"/><text x="%d" y="%.1f" text-anchor="end">1e%d</text>' % (ml, Y(10**e), W-mr, Y(10**e), ml-6, Y(10**e)+4, e))
for p in [263, 523, 1031, 2063]:
    svg.append('<line x1="%.1f" y1="%d" x2="%.1f" y2="%d" stroke="#ddd"/><text x="%.1f" y="%d" text-anchor="middle">%d</text>' % (X(p), mt, X(p), H-mb, X(p), H-mb+18, p))
svg.append('<text x="%d" y="%d" text-anchor="middle">prime p</text>' % ((ml + W - mr)//2, H-12))
for i, (arm, pts) in enumerate(series.items()):
    c = col[arm]
    svg.append('<polyline fill="none" stroke="%s" stroke-width="2" points="%s"/>' % (c, " ".join("%.1f,%.1f" % (X(p), Y(v)) for p, v in pts)))
    for p, v in pts:
        svg.append('<circle cx="%.1f" cy="%.1f" r="3.5" fill="%s"/>' % (X(p), Y(v), c))
    svg.append('<rect x="%d" y="%d" width="12" height="12" fill="%s"/><text x="%d" y="%d">arm %s</text>' % (W-mr+10, mt+10+i*20, c, W-mr+28, mt+20+i*20, arm if arm != "Bp" else "B'"))
svg.append('</svg>')
open(os.path.join(ROOT, "plot.svg"), "w").write("\n".join(svg) + "\n")
open(os.path.join(ROOT, "raw", "results_tables.md"), "w").write("\n".join(L) + "\n")
print("\n".join(L))
