#!/usr/bin/env python3
"""Render source-order local-component dimensions recorded by R101."""
import json
from pathlib import Path

root = Path(__file__).resolve().parents[1]
data = json.loads((root / "independent-replay.json").read_text())
left, right = data["component_dimensions_over_F2"]
svg = f'''<svg xmlns="http://www.w3.org/2000/svg" width="750" height="340" viewBox="0 0 750 340">
<rect width="750" height="340" fill="white"/>
<text x="40" y="34" font-size="20" font-family="sans-serif">R101 x=1 source-order fiber (total F2 dimension 112)</text>
<line x1="90" y1="270" x2="680" y2="270" stroke="black"/>
<rect x="160" y="{270-left*3}" width="130" height="{left*3}" fill="#285f9f"/>
<rect x="430" y="{270-right*3}" width="130" height="{right*3}" fill="#285f9f"/>
<text x="205" y="{255-left*3}" font-size="20" font-family="sans-serif">{left}</text>
<text x="475" y="{255-right*3}" font-size="20" font-family="sans-serif">{right}</text>
<text x="165" y="300" font-size="17" font-family="sans-serif">y=0, F4 residue</text>
<text x="435" y="300" font-size="17" font-family="sans-serif">y=1, F4 residue</text>
</svg>\n'''
(root / "publication" / "diagram.svg").write_text(svg)
