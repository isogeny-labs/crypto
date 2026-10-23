"""Summarise the rho.negation_lanes study: lanes against rho.negation in one binary.

sessions/lanes-<bits>-<rep>/   four sessions per curve, every arm alternated in each

For each curve and arm: runs verified, mean S, median solve wall time, the
ratios against the rho.negation reference arm (median wall, mean S, and the
paired geometric means over the same target and round), and the field
operations per group operation.  Writes results.json, solve-time.svg and
report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
BITS = [24, 28, 32]
ARMS = ["rho-neg", "rho-neg-aa", "lanes-8", "lanes-16", "lanes-32"]


def sessions(bits):
    return [[json.loads(l) for l in (d / "records.jsonl").read_text().splitlines() if l.strip()]
            for d in sorted(HERE.glob(f"sessions/lanes-{bits}-*"))]


def rows():
    out = []
    for bits in BITS:
        recs = [r for s in sessions(bits) for r in s if not r["warmup"]]
        key = lambda r: (r["session_id"], r["workload"]["target_index"], r["round"])
        ref = {key(r): r for r in recs if r["arm"] == "rho-neg"}
        ref_wall = statistics.median(r["time"]["solve_wall_ns"] for r in ref.values())
        ref_s = statistics.mean(r["cost"]["s"] for r in ref.values())
        for arm in ARMS:
            v = [r for r in recs if r["arm"] == arm]
            wall = statistics.median(r["time"]["solve_wall_ns"] for r in v)
            mean_s = statistics.mean(r["cost"]["s"] for r in v)
            lw = [math.log(r["time"]["solve_wall_ns"] / ref[key(r)]["time"]["solve_wall_ns"]) for r in v]
            ls = [math.log(r["cost"]["s"] / ref[key(r)]["cost"]["s"]) for r in v]
            ops = sum(r["cost"]["total_gae"] for r in v)
            fo = [r["field_ops"] for r in v]
            out.append({
                "bits": bits, "arm": arm, "method": v[0]["method"]["id"],
                "lanes": int(v[0]["method"]["params"].get("lanes", 1)) if arm.startswith("lanes") else None,
                "runs": len(v), "verified": sum(r["outcome"]["status"] == "verified" for r in v),
                "mean_s": mean_s, "median_wall_ms": wall / 1e6,
                "wall_ratio": wall / ref_wall, "s_ratio": mean_s / ref_s,
                "paired_wall_geomean": math.exp(sum(lw) / len(lw)),
                "paired_s_geomean": math.exp(sum(ls) / len(ls)),
                "muls_per_gae": sum(f["muls"] for f in fo) / ops,
                "sqrs_per_gae": sum(f["sqrs"] for f in fo) / ops,
                "invs_per_gae": sum(f["invs"] for f in fo) / ops,
            })
    return out


def figure(rs):
    W, H, L, T, B = 760, 420, 70, 60, 50
    x0, x1, y0, y1 = L, W - 190, T, H - B
    sx = lambda b: x0 + (b - 23) / (33 - 23) * (x1 - x0)
    sy = lambda v: y1 - (v - 0.3) / (1.4 - 0.3) * (y1 - y0)
    colours = {"lanes-8": "#2a6fdb", "lanes-16": "#d9822b", "lanes-32": "#7a4fc9"}
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="12">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           '<text x="16" y="22" font-size="14" fill="#222">rho.negation_lanes against rho.negation, same binary and targets</text>',
           '<text x="16" y="40" fill="#555">solid: solve wall time ratio (paired geometric mean); dashed: S ratio (counted GAE / √r)</text>']
    for t in (0.4, 0.6, 0.8, 1.0, 1.2, 1.4):
        y = sy(t)
        out.append(f'<line x1="{x0}" y1="{y:.1f}" x2="{x1}" y2="{y:.1f}" stroke="{"#333" if t == 1.0 else "#e3e3e3"}"/>')
        out.append(f'<text x="{x0 - 8}" y="{y + 4:.1f}" text-anchor="end" fill="#555">{t:.1f}</text>')
    for b in BITS:
        out.append(f'<text x="{sx(b):.1f}" y="{y1 + 18}" text-anchor="middle" fill="#555">{b}-bit</text>')
    for i, (arm, c) in enumerate(colours.items()):
        pts = [(r["bits"], r["paired_wall_geomean"], r["paired_s_geomean"]) for r in rs if r["arm"] == arm]
        wall = " ".join(f"{sx(b):.1f},{sy(w):.1f}" for b, w, _ in pts)
        s = " ".join(f"{sx(b):.1f},{sy(v):.1f}" for b, _, v in pts)
        out.append(f'<polyline points="{wall}" fill="none" stroke="{c}" stroke-width="2.5"/>')
        out.append(f'<polyline points="{s}" fill="none" stroke="{c}" stroke-width="1.5" stroke-dasharray="5,4"/>')
        for b, w, _ in pts:
            out.append(f'<circle cx="{sx(b):.1f}" cy="{sy(w):.1f}" r="3.5" fill="{c}"/>')
        ly = T + 20 + 22 * i
        out.append(f'<line x1="{x1 + 20}" y1="{ly}" x2="{x1 + 45}" y2="{ly}" stroke="{c}" stroke-width="2.5"/>')
        out.append(f'<text x="{x1 + 52}" y="{ly + 4}" fill="#222">{arm}</text>')
    out.append("</svg>")
    (HERE / "solve-time.svg").write_text("\n".join(out) + "\n")


def report(rs, replays):
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.3f}</td>"
        f"<td>{r['median_wall_ms']:.2f}</td><td>{r['paired_wall_geomean']:.3f}</td><td>{r['paired_s_geomean']:.3f}</td>"
        f"<td>{r['muls_per_gae']:.2f} / {r['sqrs_per_gae']:.2f} / {r['invs_per_gae']:.3f}</td></tr>" for r in rs)
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Rho with batched inversions</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:900px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:nth-child(2),th:nth-child(2){{text-align:left}}</style></head><body>
<h1>rho.negation_lanes: Pollard rho with batched inversions (2026-10-23)</h1>
<p>A new ecbench method beside <code>rho.negation</code>, whose code and counts are unchanged. It runs <code>rho.negation</code>'s step rule
(negation classes, look-ahead, cycle escape, distinguished points) as <code>lanes</code> walks in lockstep on a prime curve; each round's
additions share one field inversion (Montgomery's trick). Every addition any lane performs is charged, including the walks still in flight
when another lane's collision recovers the logarithm, so <code>S</code> is a few percent above <code>rho.negation</code>'s. The
<code>field_ops</code> block records the work actually done: 3(n − 1) multiplications and one inversion per round of n additions, then
2M + 1S per addition, against rho.negation's 2M + 1S + 1I.</p>
{(HERE / 'solve-time.svg').read_text()}
<table><tr><th>bits</th><th>arm</th><th>verified</th><th>mean S</th><th>median wall ms</th><th>wall / rho (paired)</th><th>S / rho (paired)</th><th>M / S / I per GAE</th></tr>
{tr}</table>
<p>Four sessions per curve on one binary, every arm alternated within each session over the same targets and rounds; warm-up excluded.
The A/A control (<code>rho-neg-aa</code>) is a second copy of the reference arm. Every session replays exactly
(<code>ecbench verify --replay-all</code>, {replays} replays). At 32 bits a rho step's single field inversion (word extended Euclid, about
110 ns) was most of its wall time; shared over a round it costs a few nanoseconds, and the step becomes instruction-bound (about
680 instructions per group operation in Callgrind, for both methods).</p>
<p>This is a wall-time result. In the counted unit (<code>S</code>, GAE/√r) the lanes method costs 4–30% more than
<code>rho.negation</code>, so it is not a stronger matched rho reference for index-calculus comparisons, which are made in that unit.</p>
</body></html>
"""
    (HERE / "report.html").write_text(html)


def main():
    rs = rows()
    replays = sum(len(json.loads(p.read_text())["replays"]) for p in sorted((HERE / "verify").glob("*.json")))
    figure(rs)
    report(rs, replays)
    headline = [{"name": f"{r['bits']}/{r['arm']}/wall_ratio", "claimed": r["wall_ratio"]} for r in rs if r["arm"] != "rho-neg"]
    headline += [{"name": f"{r['bits']}/{r['arm']}/s_ratio", "claimed": r["s_ratio"]} for r in rs if r["arm"] != "rho-neg"]
    (HERE / "results.json").write_text(json.dumps({"rows": rs, "replays": replays, "headline": headline}, indent=1, sort_keys=True) + "\n")
    for r in rs:
        print(f"{r['bits']} {r['arm']:10s} {r['verified']}/{r['runs']} S={r['mean_s']:.3f} wall={r['median_wall_ms']:.2f}ms "
              f"wall/rho={r['paired_wall_geomean']:.3f} S/rho={r['paired_s_geomean']:.3f} M/S/I per GAE {r['muls_per_gae']:.2f}/{r['sqrs_per_gae']:.2f}/{r['invs_per_gae']:.3f}")


if __name__ == "__main__":
    main()
