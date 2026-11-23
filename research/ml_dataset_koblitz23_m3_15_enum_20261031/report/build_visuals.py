#!/usr/bin/env python3
"""Render all frozen n23 Koblitz arity cells from the independent audit."""

import collections
import html
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
audit = json.loads((HERE.parent / "AUDIT.json").read_text())
rows = audit["rows"]
assert len(rows) == 26 and [r["index"] for r in rows] == list(range(26))
colors = {"verified": "#237a52", "timeout": "#ae6d13",
          "budget_exhausted": "#9b6d23", "partial": "#80752f",
          "failed": "#a53e3e", "unsupported": "#6e648b"}
pieces = ['<svg xmlns="http://www.w3.org/2000/svg" width="850" height="580" viewBox="0 0 850 580">',
          '<rect width="850" height="580" fill="white"/>',
          '<text x="24" y="32" font-size="20" font-family="sans-serif">Koblitz n23 enumeration, m=3..15: 26 fixed cells</text>',
          '<text x="24" y="55" font-size="12" font-family="sans-serif">Native outcomes under one attempt and 30-second native cap per cell</text>',
          '<text x="190" y="84" font-size="13" font-family="sans-serif">Untwisted</text>',
          '<text x="505" y="84" font-size="13" font-family="sans-serif">Quadratic twist</text>']
for m in range(3, 16):
    y = 95 + (m - 3) * 34
    pieces.append(f'<text x="27" y="{y+22}" font-size="13" font-family="sans-serif">m={m}</text>')
    for twist in (0, 1):
        row = rows[2*(m-3)+twist]
        assert row["m"] == m and row["twist"] == bool(twist)
        status = row["native_status"]
        x = 190 + 315*twist
        pieces += [f'<rect x="{x}" y="{y}" width="285" height="28" rx="3" fill="{colors.get(status, "#555555")}"/>',
                   f'<text x="{x+12}" y="{y+19}" font-size="13" font-family="sans-serif" fill="white">{html.escape(status)}</text>']
counts = collections.Counter(r["native_status"] for r in rows)
summary = ", ".join(f"{k}: {v}" for k, v in sorted(counts.items()))
pieces.append(f'<text x="24" y="555" font-size="12" font-family="sans-serif">Source: AUDIT.json; {html.escape(summary)}; zero exclusions.</text>')
pieces.append('</svg>')
(HERE / "outcomes.svg").write_text("\n".join(pieces) + "\n")
