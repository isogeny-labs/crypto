"""Summarise the wide-batching ladder (lead ICP-prime-ic-constant-engineering).

sessions/wide-<bits>-<rep>/   three sessions per curve size, every arm alternated within each

For each size and arm: runs verified, mean S, median relation-phase and solve wall
time, and the paired geometric means of S, relation-phase wall and solve wall against
the incumbent `walk` arm over the same (session, target, round).  Writes results.json,
wall-ratio.svg and report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
BITS = [20, 24, 28, 32]
BASE = "walk"
ORDER = ["rho-neg", "walk", "fast", "batch-16", "batch-64", "batch-256", "batch-1024", "serial-1024"]


def sessions(bits):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(f"sessions/wide-{bits}-*")) if (d / "records.jsonl").exists()]


def relation_ns(r):
    return next((p["wall_ns"] for p in r["phases"] if p["name"] == "relations"), None)


def geomean(xs):
    return math.exp(sum(math.log(x) for x in xs) / len(xs))


def rows():
    out = []
    for bits in BITS:
        recs = [r for s in sessions(bits) for r in s if not r["warmup"]]
        if not recs:
            continue
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        base = {key(r): r for r in recs if r["arm"] == BASE}
        rho = [r for r in recs if r["arm"] == "rho-neg"]
        rho_s = statistics.mean(r["cost"]["s"] for r in rho)
        for arm in ORDER:
            v = [r for r in recs if r["arm"] == arm]
            if not v:
                continue
            row = {"bits": bits, "arm": arm, "method": v[0]["method"]["id"],
                   "targets": v[0]["method"]["params"].get("targets") if v[0]["method"]["id"] == "ic.pipeline" else None,
                   "runs": len(v), "verified": sum(r["outcome"]["status"] == "verified" for r in v),
                   "mean_s": statistics.mean(r["cost"]["s"] for r in v),
                   "distance_to_rho": statistics.mean(r["cost"]["s"] for r in v) / rho_s,
                   "median_solve_ms": statistics.median(r["time"]["solve_wall_ns"] for r in v) / 1e6}
            if arm != "rho-neg":
                rel = [relation_ns(r) for r in v]
                row["median_relations_ms"] = statistics.median(rel) / 1e6
                pairs = [(r, base[key(r)]) for r in v if key(r) in base]
                row["pairs"] = len(pairs)
                row["paired_s"] = geomean([r["cost"]["s"] / b["cost"]["s"] for r, b in pairs])
                row["paired_relations_wall"] = geomean([relation_ns(r) / relation_ns(b) for r, b in pairs])
                row["paired_solve_wall"] = geomean([r["time"]["solve_wall_ns"] / b["time"]["solve_wall_ns"] for r, b in pairs])
                lw = [math.log(relation_ns(r) / relation_ns(b)) for r, b in pairs]
                n = len(lw)
                sd = statistics.stdev(lw) if n > 1 else 0.0
                row["relations_wall_ci95"] = [math.exp(sum(lw) / n - 1.96 * sd / math.sqrt(n)),
                                              math.exp(sum(lw) / n + 1.96 * sd / math.sqrt(n))]
            out.append(row)
    return out


def figure(rs):
    items = [r for r in rs if r["arm"] not in ("rho-neg", BASE)]
    W, L, rowh = 820, 150, 18
    H = 70 + rowh * len(items) + 30
    x0, x1 = L, W - 60
    lo, hi = 0.6, 1.6
    sx = lambda v: x0 + (math.log(v) - math.log(lo)) / (math.log(hi) - math.log(lo)) * (x1 - x0)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           '<text x="16" y="20" font-size="14" fill="#222">Relation-phase wall time against the incumbent walk (below 1 = faster)</text>',
           '<text x="16" y="38" fill="#555">paired geometric mean over the same (session, target, round); bar = 95% interval; grey = interval spans 1</text>']
    for t in (0.6, 0.7, 0.8, 0.9, 1.0, 1.2, 1.4, 1.6):
        x = sx(t)
        out.append(f'<line x1="{x:.1f}" y1="48" x2="{x:.1f}" y2="{H-26}" stroke="{"#333" if t == 1.0 else "#ddd"}"/>')
        out.append(f'<text x="{x:.1f}" y="{H-10}" text-anchor="middle" fill="#555">{t:.1f}</text>')
    for i, r in enumerate(items):
        y = 54 + i * rowh
        v = r["paired_relations_wall"]
        lo_c, hi_c = r["relations_wall_ci95"]
        colour = "#2a6fdb" if hi_c < 1 else ("#c0392b" if lo_c > 1 else "#999")
        a, b = sorted((sx(1.0), sx(min(max(v, lo), hi))))
        out.append(f'<text x="{L-8}" y="{y+11}" text-anchor="end" fill="#222">{r["bits"]}-bit {r["arm"]}</text>')
        out.append(f'<rect x="{a:.1f}" y="{y+2}" width="{max(b-a,1):.1f}" height="{rowh-7}" fill="{colour}"/>')
        out.append(f'<line x1="{sx(min(max(lo_c,lo),hi)):.1f}" y1="{y+rowh/2-1:.1f}" x2="{sx(min(max(hi_c,lo),hi)):.1f}" y2="{y+rowh/2-1:.1f}" stroke="#222"/>')
        out.append(f'<text x="{sx(hi)+6:.1f}" y="{y+11}" fill="#222">{v:.3f}</text>')
    out.append("</svg>")
    (HERE / "wall-ratio.svg").write_text("\n".join(out) + "\n")


def report(rs):
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.2f}</td>"
        f"<td>{r['distance_to_rho']:.1f}</td><td>{r.get('median_relations_ms', float('nan')):.2f}</td>"
        f"<td>{r.get('paired_s', float('nan')):.3f}</td><td>{r.get('paired_relations_wall', float('nan')):.3f}</td>"
        f"<td>{r.get('paired_solve_wall', float('nan')):.3f}</td></tr>" for r in rs)
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Wide target batching</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:980px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>Wide target batching for prime index calculus (2026-10-28)</h1>
<p>Lead <code>ICP-prime-ic-constant-engineering</code>: step 64, 256 and 1024 target walks in lock-step so that one field
inversion serves a whole round, against the incumbent guarded walk, on the pair-table recipe
(<code>mitm-prime-batch:m=2</code>). Three sessions per size in one binary, every arm alternated within each session.</p>
{(HERE / 'wall-ratio.svg').read_text()}
<table><tr><th>bits</th><th>arm</th><th>verified</th><th>mean S</th><th>S / rho.negation</th><th>median relations ms</th>
<th>paired S vs walk</th><th>paired relations wall vs walk</th><th>paired solve wall vs walk</th></tr>
{tr}</table>
<p>Counted S includes every lane's in-flight work and the start-up of every lane, so S cannot fall with width; the measurement
is the relation-phase wall time.</p></body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    rs = rows()
    figure(rs)
    report(rs)
    (HERE / "results.json").write_text(json.dumps({"rows": rs}, indent=1, sort_keys=True) + "\n")
    for r in rs:
        if r["arm"] != "rho-neg":
            print(f"{r['bits']:3d} {r['arm']:12s} S={r['mean_s']:8.2f} dist={r['distance_to_rho']:6.1f} "
                  f"rel_ms={r['median_relations_ms']:9.3f} pairedS={r['paired_s']:.3f} "
                  f"relwall={r['paired_relations_wall']:.3f} [{r['relations_wall_ci95'][0]:.3f},{r['relations_wall_ci95'][1]:.3f}] "
                  f"solve={r['paired_solve_wall']:.3f}")


if __name__ == "__main__":
    main()
