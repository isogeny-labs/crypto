"""Summarise the residual-sweep ladder (lead ICP-residual-partial-relations).

sessions/sweep-<bits>-<rep>/   three sessions per curve size, every arm alternated within each

For each size and arm: runs verified, mean S (the ecbench counted unit, GAE / sqrt(r)),
the distance to the matched rho.negation reference (S / S_rho), the residuals stored
(targets tried x 2B) with the table's memory estimate, and the paired geometric mean of
S against the pair-table incumbent over the same (session, target, round).  Fits
log2 S against log2 B for the sweep at each size (the model S = c sqrt(2B) predicts a
slope of 1/2) and log2 S against the curve size at fixed B (the model predicts 0 for the
sweep and 1/6 per bit... of the base-optimised pair table).  Writes results.json,
sweep-s.svg and report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
BITS = [20, 24, 28, 32]
INCUMBENT = {20: "pair-table-b128", 24: "pair-table-b256", 28: "pair-table-b512", 32: "pair-table-b1024"}
SWEEP = [2, 4, 8, 16, 32, 64, 128]
ENTRY_BYTES = 56  # HashMap<u64,(u64,u64,u32,bool)> slot + control byte, rounded


def load(bits):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(f"sessions/sweep-{bits}-*")) if (d / "records.jsonl").exists()]


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def slope(xs, ys):
    mx, my = statistics.mean(xs), statistics.mean(ys)
    return sum((x - mx) * (y - my) for x, y in zip(xs, ys)) / sum((x - mx) ** 2 for x in xs)


def rows():
    out = []
    for bits in BITS:
        recs = [r for s in load(bits) for r in s if not r["warmup"]]
        if not recs:
            continue
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        inc = {key(r): r for r in recs if r["arm"] == INCUMBENT[bits]}
        rho_s = statistics.mean(r["cost"]["s"] for r in recs if r["arm"] == "rho-neg")
        arms = ["rho-neg", "rho-lanes16", INCUMBENT[bits]] + [f"sweep-b{b}" for b in SWEEP]
        for arm in arms:
            v = [r for r in recs if r["arm"] == arm]
            if not v:
                continue
            row = {"bits": bits, "arm": arm, "method": v[0]["method"]["id"], "runs": len(v),
                   "verified": sum(r["outcome"]["status"] == "verified" for r in v),
                   "mean_s": statistics.mean(r["cost"]["s"] for r in v),
                   "distance_to_rho": statistics.mean(r["cost"]["s"] for r in v) / rho_s,
                   "median_solve_ms": statistics.median(r["time"]["solve_wall_ns"] for r in v) / 1e6}
            if arm.startswith("sweep-b"):
                b = int(arm[7:])
                tried = [r["counters"]["targets_tried"] for r in v]
                row["base_abscissae"] = b
                row["mean_targets_tried"] = statistics.mean(tried)
                row["residuals_stored"] = statistics.mean(t * 2 * b for t in tried)
                row["table_mb"] = row["residuals_stored"] * ENTRY_BYTES / 2**20
            if arm.startswith(("sweep-b", "pair-table")) and inc:
                pairs = [(r, inc[key(r)]) for r in v if key(r) in inc]
                row["paired_s_vs_incumbent"] = geomean([r["cost"]["s"] / b["cost"]["s"] for r, b in pairs])
            out.append(row)
    return out


def fits(rs):
    out = {"s_vs_base_slope": {}, "s_vs_bits_slope": {}}
    for bits in BITS:
        pts = [(math.log2(r["base_abscissae"]), math.log2(r["mean_s"])) for r in rs
               if r["bits"] == bits and r["arm"].startswith("sweep-b")]
        if len(pts) >= 3:
            out["s_vs_base_slope"][bits] = slope(*zip(*pts))
    for b in SWEEP:
        pts = [(r["bits"], math.log2(r["mean_s"])) for r in rs if r["arm"] == f"sweep-b{b}"]
        if len(pts) >= 3:
            out["s_vs_bits_slope"][f"sweep-b{b}"] = slope(*zip(*pts))
    pts = [(r["bits"], math.log2(r["mean_s"])) for r in rs if r["arm"].startswith("pair-table")]
    if len(pts) >= 3:
        out["s_vs_bits_slope"]["pair-table (base ~ r^(1/3))"] = slope(*zip(*pts))
    return out


def figure(rs):
    W, H, L, B = 760, 420, 70, 50
    colours = {20: "#2a6fdb", 24: "#1a9850", 28: "#d98c00", 32: "#c0392b"}
    xs = [math.log2(b) for b in SWEEP]
    ymax = max(r["mean_s"] for r in rs if r["arm"].startswith(("sweep", "pair")))
    ymax = 2 ** math.ceil(math.log2(ymax))
    sx = lambda v: L + (v - xs[0]) / (xs[-1] - xs[0]) * (W - L - 30)
    sy = lambda v: H - B - (math.log2(v) - 0) / (math.log2(ymax) - 0) * (H - B - 40)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           '<text x="16" y="20" font-size="14" fill="#222">Counted S of the residual sweep against its base size B (log-log)</text>',
           '<text x="16" y="36" fill="#555">solid: sweep; dashed horizontal: pair-table incumbent at the same size; model S = c sqrt(2B)</text>']
    for k in range(0, int(math.log2(ymax)) + 1):
        y = sy(2 ** k)
        out.append(f'<line x1="{L}" y1="{y:.1f}" x2="{W-30}" y2="{y:.1f}" stroke="#eee"/><text x="{L-6}" y="{y+4:.1f}" text-anchor="end" fill="#555">{2**k}</text>')
    for b in SWEEP:
        x = sx(math.log2(b))
        out.append(f'<text x="{x:.1f}" y="{H-B+16}" text-anchor="middle" fill="#555">{b}</text>')
    out.append(f'<text x="{(L+W-30)/2:.0f}" y="{H-10}" text-anchor="middle" fill="#222">base abscissae B</text>')
    for bits, col in colours.items():
        pts = [(math.log2(r["base_abscissae"]), r["mean_s"]) for r in rs if r["bits"] == bits and r["arm"].startswith("sweep-b")]
        if not pts:
            continue
        pts.sort()
        out.append(f'<polyline fill="none" stroke="{col}" stroke-width="2" points="' + " ".join(f"{sx(x):.1f},{sy(y):.1f}" for x, y in pts) + '"/>')
        inc = next((r["mean_s"] for r in rs if r["bits"] == bits and r["arm"] == INCUMBENT[bits]), None)
        if inc:
            out.append(f'<line x1="{L}" y1="{sy(inc):.1f}" x2="{W-30}" y2="{sy(inc):.1f}" stroke="{col}" stroke-dasharray="5 4"/>')
        out.append(f'<text x="{W-28}" y="{sy(pts[-1][1])+4:.1f}" fill="{col}">{bits}-bit</text>')
    out.append("</svg>")
    (HERE / "sweep-s.svg").write_text("\n".join(out) + "\n")


def report(rs, fit):
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.2f}</td>"
        f"<td>{r['distance_to_rho']:.1f}</td><td>{r.get('paired_s_vs_incumbent', float('nan')):.3f}</td>"
        f"<td>{r.get('residuals_stored', float('nan')):.0f}</td><td>{r.get('table_mb', float('nan')):.2f}</td></tr>" for r in rs)
    f1 = ", ".join(f"{b}-bit {s:+.2f}" for b, s in fit["s_vs_base_slope"].items())
    f2 = ", ".join(f"{k} {s:+.3f}" for k, s in fit["s_vs_bits_slope"].items())
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Residual sweep</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:980px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>Single-large-prime residual sweep (2026-10-28)</h1>
<p>Lead <code>ICP-residual-partial-relations</code>. Every residual R - P_i of a target is stored by abscissa; a collision of two
residuals is a base-only relation. Three sessions per size in one binary; all arms alternated within each session.</p>
{(HERE / 'sweep-s.svg').read_text()}
<p>Slope of log2 S against log2 B per size (model: +0.5): {f1}.<br>
Slope of log2 S against curve bits at fixed base (model: 0 for the sweep): {f2}.</p>
<table><tr><th>bits</th><th>arm</th><th>verified</th><th>mean S</th><th>S / rho.negation</th><th>paired S vs pair-table</th>
<th>residuals stored</th><th>table MB (est.)</th></tr>{tr}</table></body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    rs = rows()
    fit = fits(rs)
    figure(rs)
    report(rs, fit)
    (HERE / "results.json").write_text(json.dumps({"rows": rs, "fits": fit}, indent=1, sort_keys=True) + "\n")
    for r in rs:
        print(f"{r['bits']:3d} {r['arm']:18s} {r['verified']}/{r['runs']} S={r['mean_s']:8.2f} dist={r['distance_to_rho']:6.1f} "
              f"vsInc={r.get('paired_s_vs_incumbent', float('nan')):.3f} stored={r.get('residuals_stored', float('nan')):.0f}")
    print(json.dumps(fit, indent=1))


if __name__ == "__main__":
    main()
