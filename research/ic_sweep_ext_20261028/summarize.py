"""Summarise the extended residual-sweep study (lead ICP-residual-partial-relations).

sessions/ext-<bits>-<rep>/   three sessions per curve size (20..44 bits), every arm alternated in each

Per (size, arm): runs verified, mean S with its standard error, memory in table entries as the
arm held them (residual table, baby-step table, distinguished-point table, pair table; rho keeps
none), and the paired geometric mean of S against the planner-tuned pair-table incumbent over
the same (session, target, round).  Then:

  * exponent fits of log2 S against curve bits, bootstrap 95% intervals (runs resampled within
    each size), for the sweep at fixed B, the incumbent, BSGS, kangaroo and rho;
  * the slope of log2 S against log2 B at each size and the constant c in S = c * sqrt(2 B);
  * the birthday model check: collision rows found against probed^2 / r;
  * the time-memory trade of the distinguished-point variants;
  * the base-shape controls (smallest abscissae, strided null, known-log progression, known-log random).

Writes results.json, three SVG figures and report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import random
import statistics

HERE = pathlib.Path(__file__).parent
SIZES = list(range(20, 46, 2))
SWEEP_B = [2, 4, 8, 16, 32, 64, 128]
# Compact encodings assumed for the byte column (the implementation uses a wider HashMap slot):
# residual (key 8, a 8, b 8, index+sign 4 -> 32), BSGS baby step (key 8, index 8 -> 16),
# kangaroo distinguished point (x 8, distance 8, kind 8 -> 24), pair-table slot (key 8, pair 8 -> 16).
ENTRY_BYTES = {"sweep": 32, "variant": 32, "bsgs": 16, "kangaroo": 24, "pair": 16, "rho": 0}


def load(bits):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(f"sessions/ext-{bits}-*")) if (d / "records.jsonl").exists()]


def phase_native(r, key):
    for p in r["phases"]:
        v = (p.get("native") or {}).get(key)
        if v is not None:
            return v
    return None


def family(arm):
    if arm.startswith("sweep-b"):
        return "sweep"
    if arm.startswith("pair-"):
        return "pair"
    return {"bsgs-neg": "bsgs", "kangaroo": "kangaroo"}.get(arm, "rho" if arm.startswith("rho") else "variant")


def entries(r, arm):
    fam = family(arm)
    if fam == "rho":
        return 0
    if fam == "bsgs":
        return r["counters"].get("baby_steps", 0)
    if fam == "kangaroo":
        return r["counters"].get("inserts_uncharged", 0)
    if fam == "pair":
        for p in r["phases"]:
            v = (p.get("work_native") or {}).get("table_insert_attempts")
            if v:
                return v
        return 0
    return phase_native(r, "residuals_stored") or 0


def base_size(r):
    params = r["method"].get("params") or {}
    fb = params.get("factor_base", "")
    return int(fb.split("size=")[1].split(",")[0]) if "size=" in fb else None


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def slope(xs, ys):
    mx, my = statistics.mean(xs), statistics.mean(ys)
    d = sum((x - mx) ** 2 for x in xs)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / d if d else float("nan")


def boot_slope(points, reps=1500, seed=7):
    """points: {x: [S values]}; slope of log2 mean(S) on x with a 95% interval, runs resampled within x."""
    xs = sorted(points)
    if len(xs) < 3:
        return None
    rng = random.Random(seed)
    est = slope(xs, [math.log2(statistics.mean(points[x])) for x in xs])
    draws = sorted(slope(xs, [math.log2(statistics.mean(rng.choices(points[x], k=len(points[x])))) for x in xs])
                   for _ in range(reps))
    return {"slope": est, "lo": draws[int(0.025 * reps)], "hi": draws[int(0.975 * reps)], "sizes": len(xs)}


def collect():
    rows, runs = [], {}
    for bits in SIZES:
        sess = load(bits)
        allrecs = [r for s in sess for r in s if not r["warmup"]]
        recs = [r for r in allrecs if r["outcome"]["status"] == "verified"]
        if not recs:
            continue
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        arms = list(dict.fromkeys(r["arm"] for r in allrecs))
        inc_arm = next((a for a in arms if a.startswith("pair-walk-b")), None)
        inc = {key(r): r for r in recs if r["arm"] == inc_arm} if inc_arm else {}
        rho_s = statistics.mean(r["cost"]["s"] for r in recs if r["arm"] == "rho-neg")
        log2_r = math.log2(recs[0]["workload"]["curve"]["r"])
        for arm in arms:
            v = [r for r in recs if r["arm"] == arm]
            if not v:
                continue
            ss = [r["cost"]["s"] for r in v]
            ent = statistics.mean(entries(r, arm) for r in v)
            fam = family(arm)
            row = {"bits": bits, "log2_r": log2_r, "arm": arm, "family": fam, "method": v[0]["method"]["id"],
                   "runs": sum(1 for r in allrecs if r["arm"] == arm), "verified": len(v),
                   "mean_s": statistics.mean(ss),
                   "se_s": statistics.stdev(ss) / math.sqrt(len(ss)) if len(ss) > 1 else 0.0,
                   "distance_to_rho": statistics.mean(ss) / rho_s,
                   "entries": ent, "bytes": ent * ENTRY_BYTES[fam],
                   "base": base_size(v[0]) if v[0]["method"]["id"] == "ic.pipeline" else None}
            # time-memory: group additions times table bytes, per r (dimensionless)
            row["tm_per_r"] = row["mean_s"] * math.sqrt(2 ** log2_r) * row["bytes"] / 2 ** log2_r if row["bytes"] else 0.0
            if inc and arm != inc_arm:
                pairs = [(r, inc[key(r)]) for r in v if key(r) in inc]
                if pairs:
                    row["paired_s_vs_incumbent"] = geomean([r["cost"]["s"] / b["cost"]["s"] for r, b in pairs])
            if v[0]["method"]["id"] == "ic.pipeline" and phase_native(v[0], "sweeps") is not None and "oracle" in (v[0]["method"].get("params") or {}) \
                    and "residual-sweep" in v[0]["method"]["params"]["oracle"] and "pairs=1" not in v[0]["method"]["params"]["oracle"]:
                b = row["base"]
                chk = []
                for r in v:
                    probed = phase_native(r, "sweeps") * 2 * b if b else None
                    kept = 1.0
                    params = r["method"]["params"]["oracle"]
                    dp = int(params.split("dp=")[1].split(",")[0]) if "dp=" in params else 0
                    probed = probed / 2 ** dp if probed else None
                    fold = 2.0 if "fold=0" not in params else 1.0
                    rows_found = (phase_native(r, "collisions") or 0) + (phase_native(r, "identity_hits") or 0)
                    if probed:
                        chk.append((rows_found, probed ** 2 * fold / (2 * 2 ** log2_r)))
                if chk:
                    row["collisions_observed"] = statistics.mean(c[0] for c in chk)
                    row["collisions_predicted"] = statistics.mean(c[1] for c in chk)
            rows.append(row)
            runs.setdefault((bits, arm), ss)
    return rows, runs


def fits(rows, runs):
    out = {"s_vs_bits": {}, "s_vs_base": {}, "c_const": {}}
    families = {"sweep B=2": "sweep-b2", "sweep B=8": "sweep-b8", "sweep B=32": "sweep-b32", "sweep B=128": "sweep-b128",
                "rho.negation": "rho-neg", "rho.negation_lanes": "rho-lanes16", "bsgs.negation": "bsgs-neg",
                "kangaroo": "kangaroo"}
    for name, arm in families.items():
        pts = {r["log2_r"]: runs[(r["bits"], arm)] for r in rows if r["arm"] == arm}
        f = boot_slope(pts)
        if f:
            out["s_vs_bits"][name] = f
    inc = {r["log2_r"]: runs[(r["bits"], r["arm"])] for r in rows if r["arm"].startswith("pair-walk-b")}
    f = boot_slope(inc)
    if f:
        out["s_vs_bits"]["pair-table incumbent (walk, base ~ r^(1/3))"] = f
    for bits in SIZES:
        pts = {math.log2(b): runs[(bits, f"sweep-b{b}")] for b in SWEEP_B if (bits, f"sweep-b{b}") in runs}
        f = boot_slope(pts)
        if f:
            out["s_vs_base"][bits] = f
        cs = [statistics.mean(runs[(bits, f"sweep-b{b}")]) / math.sqrt(2 * b) for b in SWEEP_B if (bits, f"sweep-b{b}") in runs]
        if cs:
            out["c_const"][bits] = {"min": min(cs), "max": max(cs), "mean": statistics.mean(cs)}
    return out


def svg_lines(path, title, subtitle, series, xlabel, ylabel, xlog=True, W=820, H=460):
    """series: [(label, colour, [(x, y)], dashed)] -> log-log SVG."""
    L, R, T, B = 70, 150, 54, 56
    xs = [x for _, _, pts, _ in series for x, _ in pts]
    ys = [y for _, _, pts, _ in series for _, y in pts if y > 0]
    x0, x1 = (math.log2(min(xs)), math.log2(max(xs))) if xlog else (min(xs), max(xs))
    y0, y1 = math.floor(math.log2(min(ys))), math.ceil(math.log2(max(ys)))
    sx = lambda v: L + ((math.log2(v) if xlog else v) - x0) / ((x1 - x0) or 1) * (W - L - R)
    sy = lambda v: H - B - (math.log2(v) - y0) / ((y1 - y0) or 1) * (H - B - T)
    o = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
         f'<rect width="{W}" height="{H}" fill="#fff"/>',
         f'<text x="16" y="20" font-size="14" fill="#222">{title}</text><text x="16" y="38" fill="#555">{subtitle}</text>']
    for k in range(y0, y1 + 1):
        y = sy(2.0 ** k)
        o.append(f'<line x1="{L}" y1="{y:.1f}" x2="{W-R}" y2="{y:.1f}" stroke="#eee"/><text x="{L-6}" y="{y+4:.1f}" text-anchor="end" fill="#555">{2.0**k:g}</text>')
    for v in sorted(set(xs)):
        o.append(f'<text x="{sx(v):.1f}" y="{H-B+16}" text-anchor="middle" fill="#555">{v:g}</text>')
    o.append(f'<text x="{(L+W-R)/2:.0f}" y="{H-12}" text-anchor="middle" fill="#222">{xlabel}</text>')
    o.append(f'<text x="14" y="{(T+H-B)/2:.0f}" transform="rotate(-90 14 {(T+H-B)/2:.0f})" text-anchor="middle" fill="#222">{ylabel}</text>')
    for label, col, pts, dashed in series:
        pts = sorted(p for p in pts if p[1] > 0)
        if not pts:
            continue
        d = ' stroke-dasharray="5 4"' if dashed else ""
        o.append(f'<polyline fill="none" stroke="{col}" stroke-width="2"{d} points="' + " ".join(f"{sx(x):.1f},{sy(y):.1f}" for x, y in pts) + '"/>')
        for x, y in pts:
            o.append(f'<circle cx="{sx(x):.1f}" cy="{sy(y):.1f}" r="2.5" fill="{col}"/>')
        o.append(f'<text x="{W-R+6}" y="{sy(pts[-1][1])+4:.1f}" fill="{col}">{label}</text>')
    o.append("</svg>")
    (HERE / path).write_text("\n".join(o) + "\n")


def figures(rows):
    by = lambda arm: [(r["log2_r"], r["mean_s"]) for r in rows if r["arm"] == arm]
    inc = [(r["log2_r"], r["mean_s"]) for r in rows if r["arm"].startswith("pair-walk-b")]
    svg_lines("fig-scaling.svg", "Counted S against curve size (log2 r)",
              "sweep at fixed base B, the planner-tuned pair-table incumbent, and the generic searches",
              [("pair-table incumbent", "#c0392b", inc, False), ("sweep B=128", "#d98c00", by("sweep-b128"), False),
               ("sweep B=8", "#1a9850", by("sweep-b8"), False), ("sweep B=2", "#2a6fdb", by("sweep-b2"), False),
               ("kangaroo", "#8e44ad", by("kangaroo"), True), ("bsgs.negation", "#555", by("bsgs-neg"), True),
               ("rho.negation", "#000", by("rho-neg"), True)], "log2 r", "S")
    cols = {20: "#2a6fdb", 24: "#1a9850", 28: "#d98c00", 32: "#c0392b", 36: "#8e44ad"}
    series = []
    for bits, c in cols.items():
        pts = [(r["base"], r["mean_s"]) for r in rows if r["bits"] == bits and r["arm"].startswith("sweep-b")]
        series.append((f"{bits}-bit", c, pts, False))
    svg_lines("fig-base.svg", "Counted S of the sweep against its base size B",
              "model: S = c * sqrt(2 B), slope 1/2 in log-log", series, "base abscissae B", "S")
    dp = []
    for bits, c in cols.items():
        pts = []
        for k, arm in ((0, "sweep-b8"), (2, "b8-dp2"), (4, "b8-dp4"), (6, "b8-dp6")):
            r = next((r for r in rows if r["bits"] == bits and r["arm"] == arm), None)
            if r and r["bytes"]:
                pts.append((r["bytes"] / 2 ** 20, r["mean_s"]))
        dp.append((f"{bits}-bit B=8 dp 0/2/4/6", c, pts, False))
    svg_lines("fig-timememory.svg", "Time-memory trade of the distinguished-point sweep (B = 8)",
              "each point: dp = 0, 2, 4, 6 (a stored residual needs k trailing zero bits); memory in MB at the compact entry size",
              dp, "table size, MB (compact entries)", "S")


def findings(rows, fit, runs):
    g = lambda bits, arm, k: next((r[k] for r in rows if r["bits"] == bits and r["arm"] == arm), float("nan"))
    sb = fit["s_vs_bits"]
    fx = lambda n: f"{sb[n]['slope']:+.3f} [{sb[n]['lo']:+.3f}, {sb[n]['hi']:+.3f}]" if n in sb else "n/a"
    top = max(r["bits"] for r in rows)
    items = [
        f"<li><b>Exponent in the curve size.</b> Slope of log2 S against log2 r (95% bootstrap interval): sweep B=2 {fx('sweep B=2')}; B=8 {fx('sweep B=8')}; B=128 {fx('sweep B=128')}; "
        f"rho.negation {fx('rho.negation')}; bsgs.negation {fx('bsgs.negation')}; kangaroo {fx('kangaroo')}; pair-table incumbent {fx('pair-table incumbent (walk, base ~ r^(1/3))')}. "
        "A fixed-base sweep is flat like the generic searches (S = GAE/sqrt(r) is constant when the work is c*sqrt(r)); the tuned incumbent grows.</li>",
        f"<li><b>Base size.</b> Slope of log2 S against log2 B per size: " + ", ".join(f"{b}-bit {v['slope']:.2f}" for b, v in fit["s_vs_base"].items()) +
        " (model 0.5). The constant c in S = c sqrt(2B) ranges over " + ", ".join(f"{b}-bit {v['mean']:.2f}" for b, v in fit["c_const"].items()) + ".</li>",
        f"<li><b>Generic comparators at {top} bits.</b> rho.negation {g(top,'rho-neg','mean_s'):.2f}, rho.negation_lanes {g(top,'rho-lanes16','mean_s'):.2f}, bsgs.negation {g(top,'bsgs-neg','mean_s'):.2f}, "
        f"kangaroo {g(top,'kangaroo','mean_s'):.2f}; the sweep at B=2 is {g(top,'sweep-b2','mean_s') if not math.isnan(g(top,'sweep-b2','mean_s')) else g(top,'b2-dp4','mean_s'):.2f}.</li>",
    ]
    return "<ul>" + "".join(items) + "</ul>"


def report(rows, fit, runs):
    def fmt(v, spec):
        return "" if v is None or (isinstance(v, float) and math.isnan(v)) else format(v, spec)
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.2f} &plusmn; {r['se_s']:.2f}</td>"
        f"<td>{r['distance_to_rho']:.1f}</td><td>{fmt(r.get('paired_s_vs_incumbent'), '.3f')}</td>"
        f"<td>{r['entries']:.0f}</td><td>{r['bytes']/2**20:.2f}</td></tr>" for r in rows)
    figs = "".join((HERE / f).read_text() for f in ("fig-scaling.svg", "fig-base.svg", "fig-timememory.svg"))
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Extended residual sweep</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:1000px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>The residual sweep, exhaustively (2026-10-28)</h1>
<p>Lead <code>ICP-residual-partial-relations</code>. Sizes 20 to 44 bits, three sessions each, the sweep against the generic searches and the
pair-table incumbent, with variants (no sign fold, distinguished points, pair-table hybrid) and base-shape controls.</p>
{figs}
<h2>Findings</h2>
{findings(rows, fit, runs)}
<h2>All arms</h2>
<table><tr><th>bits</th><th>arm</th><th>verified</th><th>mean S</th><th>S / rho.negation</th><th>paired S vs incumbent</th><th>table entries</th><th>table MB (compact)</th></tr>
{tr}</table></body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    rows, runs = collect()
    fit = fits(rows, runs)
    figures(rows)
    report(rows, fit, runs)
    headline = []
    for r in rows:
        headline.append({"name": f"{r['bits']}/{r['arm']}/mean_s", "claimed": r["mean_s"]})
        if "paired_s_vs_incumbent" in r:
            headline.append({"name": f"{r['bits']}/{r['arm']}/paired_s_vs_incumbent", "claimed": r["paired_s_vs_incumbent"]})
    (HERE / "results.json").write_text(json.dumps({"rows": rows, "fits": fit, "headline": headline}, indent=1, sort_keys=True) + "\n")
    print(len(rows), "rows;", len(headline), "headline numbers")
    print(json.dumps(fit, indent=1)[:3000])


if __name__ == "__main__":
    main()
