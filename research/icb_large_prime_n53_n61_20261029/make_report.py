"""Figure and report.html from results.json (lead LEAD-ICB-large-prime-variants). Standard library only."""
import json
import math
import pathlib
import re
import statistics

HERE = pathlib.Path(__file__).parent
R = json.loads((HERE / "results.json").read_text())["groups"]


def envelope(arm):
    m = re.match(r"(plain|lp[012])-k(\d+)(?:-c(\d+))?", arm)
    if not m:
        return None
    k = int(m.group(2))
    if m.group(1) == "plain":
        return k
    if m.group(3) is None:
        return None
    return k * (1 + int(m.group(3)))


def rss_wall():
    out = {}
    for p in sorted((HERE / "sessions").glob("*/records.jsonl")):
        g = "n53" if p.parent.name.startswith("n53") else ("n61a" if p.parent.name == "n61-s1" else "n61b")
        for line in p.read_text().splitlines():
            r = json.loads(line)
            if r["warmup"]:
                continue
            t = r["time"]
            out.setdefault((g, r["arm"]), []).append((t["max_rss_kib"] / 1024, t["solve_wall_ns"] / 1e9))
    return {k: (statistics.median(x for x, _ in v), statistics.median(y for _, y in v)) for k, v in out.items()}


def panel(g, x0, w, h, title):
    rows = R[g]["rows"]
    pts = [(envelope(a), r["mean_s"], a) for a, r in rows.items() if envelope(a)]
    rho = rows["rho-strong"]["mean_s"]
    xs = [p[0] for p in pts]
    ys = [p[1] for p in pts] + [rho]
    lx0, lx1 = math.log(min(xs) * 0.8), math.log(max(xs) * 1.25)
    ly0, ly1 = math.log(min(ys) * 0.7), math.log(max(ys) * 1.4)
    sx = lambda v: x0 + 48 + (math.log(v) - lx0) / (lx1 - lx0) * (w - 70)
    sy = lambda v: 40 + h - 40 - (math.log(v) - ly0) / (ly1 - ly0) * (h - 80)
    o = [f'<text x="{x0+48}" y="26" font-size="14" fill="#222">{title}</text>']
    for t in (0.1, 0.2, 0.5, 1, 2, 5, 10, 20, 50, 100):
        if math.log(t) < ly0 or math.log(t) > ly1:
            continue
        o.append(f'<line x1="{x0+48}" y1="{sy(t):.1f}" x2="{x0+w-22}" y2="{sy(t):.1f}" stroke="#e4e4e4"/>')
        o.append(f'<text x="{x0+44}" y="{sy(t)+4:.1f}" text-anchor="end" fill="#555">{t:g}</text>')
    for t in (20, 50, 100, 200, 500, 1000, 2000):
        if math.log(t) < lx0 or math.log(t) > lx1:
            continue
        o.append(f'<line x1="{sx(t):.1f}" y1="40" x2="{sx(t):.1f}" y2="{h}" stroke="#f0f0f0"/>')
        o.append(f'<text x="{sx(t):.1f}" y="{h+16}" text-anchor="middle" fill="#555">{t}</text>')
    o.append(f'<line x1="{x0+48}" y1="{sy(rho):.1f}" x2="{x0+w-22}" y2="{sy(rho):.1f}" stroke="#2a6fdb" stroke-dasharray="5 3"/>')
    o.append(f'<text x="{x0+w-24}" y="{sy(rho)-5:.1f}" text-anchor="end" fill="#2a6fdb">rho (strong signed-Frobenius) {rho:.2f}</text>')
    plain = sorted((p for p in pts if p[2].startswith("plain") and not p[2].endswith("-aa")))
    o.append('<polyline fill="none" stroke="#444" stroke-width="1.5" points="' + " ".join(f"{sx(p[0]):.1f},{sy(p[1]):.1f}" for p in plain) + '"/>')
    for e, s, a in pts:
        if a.endswith("-aa") or a == "lp0-k440":
            continue
        col = "#444" if a.startswith("plain") else ("#c0392b" if a.startswith("lp1") else "#1e8449")
        o.append(f'<circle cx="{sx(e):.1f}" cy="{sy(s):.1f}" r="4.5" fill="{col}"/>')
        lab = a.replace("plain-", "").replace("lp2-", "2LP ").replace("lp1-", "1LP ")
        o.append(f'<text x="{sx(e)+6:.1f}" y="{sy(s)-6:.1f}" font-size="10" fill="{col}">{lab}</text>')
    o.append(f'<text x="{x0+48+(w-70)/2}" y="{h+34}" text-anchor="middle" fill="#222">envelope: plain columns, or k (1 + c) for a large-prime arm</text>')
    return "\n".join(o)


def figure():
    W, H = 1180, 430
    body = [panel("n53", 0, 580, H - 40, "n = 53 (log2 r = 44.3): counted S, mean over workloads"),
            panel("n61b", 590, 580, H - 40, "n = 61 (log2 r = 47.2), bracket session (3 workloads)")]
    svg = [f'<svg xmlns="http://www.w3.org/2000/svg" width="{W}" height="{H}" viewBox="0 0 {W} {H}" font-family="Helvetica,Arial,sans-serif" font-size="11">',
           f'<rect width="{W}" height="{H}" fill="#fff"/>'] + body + ["</svg>"]
    (HERE / "large-prime-s.svg").write_text("\n".join(svg) + "\n")


def table(g):
    rows = R[g]["rows"]
    pairs = R[g]["pairs"]
    priced = R[g]["priced"]
    mem = rss_wall()
    ctl = {"lp2-k60-c10": "plain-k660", "lp2-k40-c20": "plain-k880", "lp2-k20-c10": "plain-k220", "lp2-k80-c3": "plain-k320", "lp2-k40-c10": "plain-k440", "lp2-k20-c30": "plain-k660",
           "lp2-k40-c3": "plain-k220", "lp1-k20-c10": "plain-k220", "lp1-k40-c10": "plain-k440", "lp0-k440": "plain-k440", "plain-k440-aa": "plain-k440"}
    tr = []
    for arm in sorted(rows, key=lambda a: (envelope(a) is None, rows[a]["mean_s"])):
        r = rows[arm]
        p = pairs.get(f"{arm}/vs/{ctl.get(arm)}")
        pr = priced.get(arm, {})
        cp = f"{p['ratio']:.3f} [{p['ci95'][0]:.3f}, {p['ci95'][1]:.3f}] vs {ctl[arm]}" if p else "-"
        vr = pr.get("vs_rho")
        rho = f"{vr[0]:.1f} / {vr[1]:.1f} / {vr[2]:.1f}" if vr else "-"
        vc = pr.get("vs_control")
        pc = f"{vc[0]:.3f} / {vc[1]:.3f} / {vc[2]:.3f}" if vc else "-"
        m = mem.get((g, arm))
        tr.append(f"<tr><td>{arm}</td><td>{r['verified']}/{r['runs']}</td><td>{r['mean_s']:.3f}</td><td>{cp}</td><td>{pc}</td><td>{rho}</td>"
                  f"<td>{m[0]:.0f}</td><td>{m[1]:.1f}</td></tr>")
    return ("<table><tr><th>arm</th><th>verified</th><th>mean S</th><th>paired S vs equal-envelope control (95% interval)</th>"
            "<th>priced ratio vs control, w = 0 / 0.1 / 1</th><th>S / rho, w = 0 / 0.1 / 1</th><th>median RSS MiB</th><th>median solve wall s</th></tr>"
            + "\n".join(tr) + "</table>")


def main():
    figure()
    findings = (HERE / "FINDINGS.html").read_text() if (HERE / "FINDINGS.html").exists() else ""
    html = f"""<!doctype html><html><head><meta charset="utf-8"><title>Large primes on the Koblitz compact-orbit pipeline</title>
<style>body{{font-family:Helvetica,Arial,sans-serif;max-width:1100px;margin:24px auto;color:#222;font-size:13px}}
table{{border-collapse:collapse;margin:8px 0;font-size:12px}}td,th{{border:1px solid #ccc;padding:3px 6px;text-align:right}}td:first-child,th:first-child{{text-align:left}}</style></head><body>
<h1>One- and two-large-prime collectors on the Koblitz compact-orbit pipeline, n = 53 and 61 (2026-10-29)</h1>
<p>Atlas lead <code>LEAD-ICB-large-prime-variants</code>. Same ecbench harness, same planted-log workloads, counted S = GAE / sqrt(r); the arms differ only in the oracle's large-prime parameters and column counts. See <code>PROTOCOL.md</code>.</p>
{(HERE / 'large-prime-s.svg').read_text()}
<h2>Findings</h2>{findings}
<h2>n = 53, icv1-f2m53-tm56619371-dac20a85</h2>{table('n53')}
<h2>n = 61, session s1 (icv1-f2m61-t158598901-ab42b6c5, 4 workloads)</h2>{table('n61a')}
<h2>n = 61, session s2: bracket of the plain optimum (3 other workloads)</h2>{table('n61b')}
</body></html>"""
    (HERE / "report.html").write_text(html)
    print("wrote report.html")


if __name__ == "__main__":
    main()
