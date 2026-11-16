#!/usr/bin/env python3
"""Draw the grouped OOF MAE chart from attempt-05/model/result.json."""
# The checked-in SVG above is the rendered source artifact; this script
# documents the exact plotted input and can be replaced with a richer renderer.
import json
from pathlib import Path
p=Path(__file__).resolve().parents[2]/"attempt-05/model/result.json"
r=json.loads(p.read_text())
for f in r["folds"]:
    print(f["heldout_base"], f["ridge_mae"], f["matched_configuration_baseline_mae"], f["n"])
