#!/usr/bin/env python3
"""Render the four retained native outcomes without rerunning a solver."""
import html
import json
from pathlib import Path

HERE = Path(__file__).resolve().parent
AUDIT = json.loads((HERE.parent / "AUDIT.json").read_text())
assert len(AUDIT["rows"]) == 4
pieces = ['<svg xmlns="http://www.w3.org/2000/svg" width="760" height="290" viewBox="0 0 760 290">',
          '<rect width="760" height="290" fill="white"/>',
          '<text x="20" y="30" font-size="19" font-family="sans-serif">Ordinary binary n47, m4: all four fixed cells</text>',
          '<text x="20" y="52" font-size="12" font-family="sans-serif">Native status; one attempt per index, both twists, shared public inputs within each method pair</text>']
for row in AUDIT["rows"]:
    i = row["index"]
    y = 76 + i * 43
    status = row["native_status"]
    color = "#237a52" if status == "verified" else "#b46b1c"
    label = f"{i}: {row['method']} / {'twisted' if row['twist'] else 'untwisted'}"
    pieces += [f'<text x="20" y="{y+19}" font-size="14" font-family="sans-serif">{html.escape(label)}</text>',
               f'<rect x="225" y="{y}" width="350" height="28" rx="3" fill="{color}"/>',
               f'<text x="238" y="{y+19}" font-size="14" font-family="sans-serif" fill="white">{html.escape(status)}</text>']
pieces.append('<text x="20" y="275" font-size="12" font-family="sans-serif">Source: AUDIT.json and experiment-export.json; 3 verified, 1 native timeout, 0 exclusions.</text>')
pieces.append('</svg>')
(HERE / "outcomes.svg").write_text("\n".join(pieces) + "\n")
