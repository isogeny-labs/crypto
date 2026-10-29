"""Summarise the table-layout study: new binary against old binary, paired.

sessions/ab-old-<bits>-<rep>  recorded by the old binary (the base of this branch)
sessions/ab-new-<bits>-<rep>  recorded by the new binary (flat pair table, open-addressed target guard)

For each size and arm: paired geometric mean of new/old over (rep, target, round) for the relation
phase, the solve and every other phase, a 95% interval on the log scale, and whether every record
is identical outside timing.  Writes results.json (with the headline list the validator rechecks),
ratios.svg and report.html next to this file.  Standard library only.
"""
import json
import math
import pathlib
import statistics

HERE = pathlib.Path(__file__).parent
BITS = [24, 28, 32]
REPS = [1, 2, 3]
VOLATILE = {"time", "isolation", "env_class_id", "binary_sha256", "record_id", "session_id", "run_id"}


def load(name):
    p = HERE / "sessions" / name / "records.jsonl"
    return [json.loads(l) for l in p.read_text().splitlines() if l.strip()] if p.exists() else []


def counted(r):
    r = {k: v for k, v in r.items() if k not in VOLATILE}
    r["phases"] = [{k: v for k, v in p.items() if k != "wall_ns"} for p in r["phases"]]
    if isinstance(r.get("online"), dict):
        o = dict(r["online"])
        o.pop("wall_ns", None)
        o["phases_ns"] = sorted(o.get("phases_ns") or {})
        r["online"] = o
    return json.dumps(r, sort_keys=True)


def phase_ns(r, name):
    return next((p["wall_ns"] for p in r["phases"] if p["name"] == name), None)


def gm_ci(ratios):
    logs = [math.log(x) for x in ratios]
    m = statistics.mean(logs)
    h = 1.96 * statistics.stdev(logs) / math.sqrt(len(logs)) if len(logs) > 1 else 0.0
    return math.exp(m), math.exp(m - h), math.exp(m + h)


def rows():
    out = []
    for bits in BITS:
        pairs = {}
        same = total = 0
        for rep in REPS:
            old, new = load(f"ab-old-{bits}-{rep}"), load(f"ab-new-{bits}-{rep}")
            key = lambda r: (r["arm"], r["workload"]["target_index"], r["round"], r["warmup"])
            nb = {key(r): r for r in new}
            for o in old:
                n = nb.get(key(o))
                if n is None:
                    continue
                total += 1
                same += counted(o) == counted(n)
                if not o["warmup"]:
                    pairs.setdefault(o["arm"], []).append((rep, o, n))
        for arm, items in sorted(pairs.items()):
            row = {"bits": bits, "arm": arm, "pairs": len(items), "identical": same, "records": total,
                   "mean_s": statistics.mean(o["cost"]["s"] for _, o, _ in items)}
            row["solve"] = gm_ci([n["time"]["solve_wall_ns"] / o["time"]["solve_wall_ns"] for _, o, n in items])
            names = [p["name"] for p in items[0][1]["phases"]]
            row["phases"] = {}
            for name in names:
                xs = [(phase_ns(n, name), phase_ns(o, name)) for _, o, n in items]
                if all(a and b for a, b in xs) and len(xs) > 1:
                    row["phases"][name] = gm_ci([a / b for a, b in xs])
            out.append(row)
    return out


def figure(rs):
    items = [(r, "relations") for r in rs if r["arm"] != "rho-neg" and "relations" in r["phases"]]
    items += [(r, "solve") for r in rs if r["arm"] != "rho-neg"]
    W, L, rowh = 760, 170, 17
    H = 70 + rowh * len(items) + 30
    lo, hi = 0.6, 1.4
    sx = lambda v: L + (math.log(v) - math.log(lo)) / (math.log(hi) - math.log(lo)) * (W - 60 - L)
    out = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>',
           '<text x="16" y="20" font-size="14" fill="#222">New binary against old binary, wall time ratio (below 1 = faster)</text>',
           '<text x="16" y="38" fill="#555">paired geometric mean over (rep, target, round); bar = 95% interval; blue = interval below 1, grey = spans 1</text>']
    for t in (0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.4):
        x = sx(t)
        out.append(f'<line x1="{x:.1f}" y1="48" x2="{x:.1f}" y2="{H-26}" stroke="{"#333" if t == 1.0 else "#ddd"}"/>')
        out.append(f'<text x="{x:.1f}" y="{H-10}" text-anchor="middle" fill="#555">{t:.1f}</text>')
    for i, (r, what) in enumerate(items):
        v, a, b = r["solve"] if what == "solve" else r["phases"][what]
        y = 54 + i * rowh
        colour = "#2a6fdb" if b < 1 else ("#c0392b" if a > 1 else "#999")
        x0, x1 = sorted((sx(1.0), sx(min(max(v, lo), hi))))
        out.append(f'<text x="{L-8}" y="{y+11}" text-anchor="end" fill="#222">{r["bits"]}-bit {r["arm"]} {what}</text>')
        out.append(f'<rect x="{x0:.1f}" y="{y+2}" width="{max(x1-x0,1):.1f}" height="{rowh-7}" fill="{colour}"/>')
        out.append(f'<line x1="{sx(min(max(a,lo),hi)):.1f}" y1="{y+rowh/2-1:.1f}" x2="{sx(min(max(b,lo),hi)):.1f}" y2="{y+rowh/2-1:.1f}" stroke="#222"/>')
        out.append(f'<text x="{sx(hi)+6:.1f}" y="{y+11}" fill="#222">{v:.3f}</text>')
    out.append("</svg>")
    (HERE / "ratios.svg").write_text("\n".join(out) + "\n")


def report(rs):
    tr = "\n".join(
        f"<tr><td>{r['bits']}</td><td>{r['arm']}</td><td>{r['pairs']}</td><td>{r['identical']}/{r['records']}</td>"
        + "".join(f"<td>{r['phases'][n][0]:.3f} [{r['phases'][n][1]:.3f}, {r['phases'][n][2]:.3f}]</td>" if n in r["phases"] else "<td></td>"
                  for n in ("setup", "relations", "linear_algebra"))
        + f"<td>{r['solve'][0]:.3f} [{r['solve'][1]:.3f}, {r['solve'][2]:.3f}]</td></tr>" for r in rs if r["arm"] != "rho-neg")
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Pair table and guard layout</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:980px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>Flat pair table and open-addressed target guard for the prime IC relation trial (2026-10-29)</h1>
<p>Lead <code>ICP-prime-ic-constant-engineering</code>. The pair table of <code>mitm-prime-batch</code> becomes one open-addressed
region of 64-bit words (key, 16-bit witness indices) and the target guard an open-addressed set; the lane round issues real
prefetches for both instead of touching them. Counted output is meant to be identical; the table below says whether it was.
New binary against old binary, same spec, sessions alternated, three repetitions per size.</p>
{(HERE / 'ratios.svg').read_text()}
<table><tr><th>bits</th><th>arm</th><th>pairs</th><th>records identical outside timing</th><th>setup new/old</th><th>relations new/old</th><th>linear algebra new/old</th><th>solve new/old</th></tr>
{tr}</table>
</body></html>"""
    (HERE / "report.html").write_text(html)


def main():
    rs = rows()
    figure(rs)
    report(rs)
    headline = []
    for r in rs:
        if r["arm"] == "rho-neg":
            continue
        for name, v in sorted(r["phases"].items()):
            headline.append({"name": f"{r['bits']}/{r['arm']}/{name}", "claimed": v[0]})
        headline.append({"name": f"{r['bits']}/{r['arm']}/solve", "claimed": r["solve"][0]})
    (HERE / "results.json").write_text(json.dumps({"rows": rs, "headline": headline}, indent=1, sort_keys=True) + "\n")
    for r in rs:
        if r["arm"] != "rho-neg":
            ph = " ".join(f"{n}={v[0]:.3f}[{v[1]:.3f},{v[2]:.3f}]" for n, v in r["phases"].items())
            print(f"{r['bits']:3d} {r['arm']:5s} n={r['pairs']:3d} identical={r['identical']}/{r['records']} solve={r['solve'][0]:.3f} {ph}")


if __name__ == "__main__":
    main()
