#!/usr/bin/env python3
"""Plot exact factor degree contributions from independent-replay.json."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
rows = json.loads((root / "independent-replay.json").read_text())["rows"]
bars = []
for i, row in enumerate(rows):
    degree = row["degree"] * row["exponent"]
    x = 95 + i * 220
    height = degree * 3
    bars.append(f'<rect x="{x}" y="{280-height}" width="100" height="{height}" fill="#285f9f"/>')
    bars.append(f'<text x="{x+25}" y="{270-height}" font-size="18" font-family="sans-serif">{degree}</text>')
    bars.append(f'<text x="{x-2}" y="310" font-size="16" font-family="sans-serif">degree {row["degree"]}, exponent {row["exponent"]}</text>')
svg = '<svg xmlns="http://www.w3.org/2000/svg" width="800" height="380" viewBox="0 0 800 380">\n'
svg += '<rect width="800" height="380" fill="white"/><text x="45" y="35" font-size="20" font-family="sans-serif">R100 root factor degree contributions (sum 123)</text>\n'
svg += '<line x1="50" y1="280" x2="760" y2="280" stroke="black"/>\n' + '\n'.join(bars)
svg += '<text x="250" y="350" font-size="15" font-family="sans-serif">Degree times multiplicity in the degree-123 root</text></svg>\n'
(root / "publication" / "diagram.svg").write_text(svg)
