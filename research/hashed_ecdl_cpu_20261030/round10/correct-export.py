#!/usr/bin/env python3
"""Correct metadata only; retain the initial invalid export unchanged."""
import hashlib
import json
from pathlib import Path
root = Path(__file__).resolve().parent
out = root / 'results'
data = json.loads((out / 'export.json').read_text())
assert len(data['runs']) == 150
for key in ['planned', 'attempted', 'completed']:
    data['flow'][key] = 150
source = (root / 'analyze.rs').read_bytes()
data['analysis']['script'] = {'path': '../analyze.rs', 'sha256': hashlib.sha256(source).hexdigest(), 'bytes': len(source)}
data['notes'].append({'kind': 'observation', 'by': 'Codex', 'text': 'Metadata correction: initial export retained four-arm flow counts and old analyzer identity from its template. Initial export remains preserved; this corrected export changes only flow and analysis-script reference. Raw data and frozen protocol are unchanged.'})
with (out / 'export-corrected.json').open('x') as f:
    json.dump(data, f, indent=2)
    f.write('\n')
