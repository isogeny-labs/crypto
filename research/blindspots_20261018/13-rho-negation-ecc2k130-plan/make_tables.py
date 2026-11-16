#!/usr/bin/env python3
"""Layout only: reads raw/summary.json (computed by b13 summary) and writes the
per-setting verdict table (tables.md, tables.typ). The verdict words apply
PROTOCOL.md decision rule item 5 to the recorded intervals and bands."""
import json, sys, collections
D = sys.argv[1]
j = json.load(open(f"{D}/raw/summary.json"))
S = collections.defaultdict(dict)
for c in j["cells"]:
    if c["admissible"] and c["theta"] == c["bits"] // 4:
        S[(c["arm"], c["r"])][c["bits"]] = c
order = ["min2", "la", "per2", "per4", "per8", "per16", "per32", "per64", "laper16", "laper32", "laper64"]
rows = []; cnt = collections.Counter()
for r in (16, 32, 128, 2048):
    for a in order:
        d = S[(a, r)]
        adv = all(d[b]["gain_full"][1] > 1 and min(d[b]["band"]) > 1 for b in d)
        reg = all(d[b]["gain_full"][2] < 1 and max(d[b]["band"]) < 1 for b in d)
        flip = any(min(d[b]["band"]) < 1 < max(d[b]["band"]) for b in d)
        cover = all(d[b]["gain_full"][1] <= 1 <= d[b]["gain_full"][2] for b in d)
        v = "ADVANCES" if adv else "REGRESSES" if reg else "TRADES" if flip else "MATCHES" if cover else "INCONCLUSIVE"
        cnt[v] += 1
        cells = []
        for b in (40, 44, 48):
            g = d[b]["gain_full"]
            dag = "+" if d[b]["solved"] < d[b]["targets"] else ""
            cells.append(f"{g[0]:.2f} [{g[1]:.2f}, {g[2]:.2f}]{dag}")
        lo = min(min(d[b]["band"]) for b in d); hi = max(max(d[b]["band"]) for b in d)
        best = max(d[b]["gain_full"][0] for b in d) / 1.4142
        rows.append((a, r, *cells, f"{lo:.2f}..{hi:.2f}", f"{best:.2f}", v))
hdr = ["rule", "r", "2^40", "2^44", "2^48", "nine-point range", "best/sqrt2", "verdict"]
with open(f"{D}/tables.md", "w") as f:
    f.write("| " + " | ".join(hdr) + " |\n|" + "|".join(["--"] * len(hdr)) + "|\n")
    for x in rows: f.write("| " + " | ".join(map(str, x)) + " |\n")
    f.write("\n" + ", ".join(f"{k} {v}" for k, v in sorted(cnt.items())) + "\n")
with open(f"{D}/tables.typ", "w") as f:
    f.write("#table(columns: 8, align: (left, right, left, left, left, left, right, left), stroke: 0.4pt + gray, inset: 3pt,\n")
    f.write("  " + ", ".join(f"[*{h}*]" for h in hdr) + ",\n")
    for x in rows: f.write("  " + ", ".join(f"[{c}]" for c in x) + ",\n")
    f.write(")\n")
print(cnt)
