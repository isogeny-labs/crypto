"""R8 figure: production-path off/default ratio of means per cell (word operations and CPU), r3 default vs new default.
Run: python3 -I plot.py  (reads runs/r8_*.jsonl and runs/r8n_*.jsonl; writes fig_r8.svg)"""
import json, random, collections, math

random.seed(1)


def load(fs):
    cells = collections.defaultdict(list)
    for f in fs:
        for l in open(f):
            r = json.loads(l)
            if r["mode"] == "prod" and not r.get("same_decomposition", True):
                continue
            if r["mode"] == "prod":
                cells[r["cell"]].append(r)
    return cells


def stat(rs, key):
    f = lambda s: sum(x["i_" + key] for x in s) / sum(x["d_" + key] for x in s)
    v = sorted(f([random.choice(rs) for _ in rs]) for _ in range(2000))
    return f(rs), v[50], v[1949]


old = load(["runs/r8_a.jsonl", "runs/r8_b.jsonl"])
new = load(["runs/r8n_a.jsonl", "runs/r8n_b.jsonl"])


def order(c):
    n = int(c.split("-n")[1].split("-")[0])
    return (n, c)


cells = sorted(old, key=order)
W, H, L, T = 1130, 90 + 38 * len(cells), 190, 60
xs = lambda v: L + (v - 0.6) / (1.6 - 0.6) * 740
out = [f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {W} {H}" font-family="sans-serif" font-size="12">',
       f'<rect width="{W}" height="{H}" fill="white"/>',
       f'<text x="10" y="22" font-size="15" font-weight="bold">R8: harvest default vs off, production path (ratio of means off/default; &gt;1 = default cheaper)</text>',
       f'<text x="10" y="42" fill="#444">word operations, 95% bootstrap CI. blue: round-3 default (gpv OR rank), orange: R8 default (rank only). Fresh targets t=5000.., one thread.</text>']
for v in (0.6, 0.8, 1.0, 1.2, 1.4, 1.6):
    out.append(f'<line x1="{xs(v)}" y1="{T}" x2="{xs(v)}" y2="{H-20}" stroke="{"#000" if v==1 else "#ddd"}"/>')
    out.append(f'<text x="{xs(v)-10}" y="{H-6}" fill="#444">{v}</text>')
for i, c in enumerate(cells):
    y = T + 20 + 38 * i
    n, l = c.split("-")[1], c.split("-")[2]
    m = c.split("-")[3]
    out.append(f'<text x="10" y="{y+6}">({n[1:]},{l[1:]}) {m}</text>')
    for dy, (data, col) in zip((-7, 7), ((old, "#2a6fb0"), (new, "#d9822b"))):
        r, lo, hi = stat(data[c], "word_ops")
        lo, hi, r = max(lo, 0.6), min(hi, 1.6), min(max(r, 0.6), 1.6)
        out.append(f'<line x1="{xs(lo)}" y1="{y+dy}" x2="{xs(hi)}" y2="{y+dy}" stroke="{col}" stroke-width="2"/>')
        out.append(f'<circle cx="{xs(r)}" cy="{y+dy}" r="4" fill="{col}"/>')
        out.append(f'<text x="{xs(1.6)+14}" y="{y+dy+4}" fill="{col}">{r:.3f} [{lo:.3f}, {hi:.3f}]</text>')
out.append("</svg>")
open("fig_r8.svg", "w").write("\n".join(out))
