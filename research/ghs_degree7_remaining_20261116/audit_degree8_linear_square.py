#!/usr/bin/env python3
"""Audit four source-bound receipts; this does not replay Sage elimination."""
import hashlib
import json
from pathlib import Path

p = Path(__file__).resolve().parent
source_hash = hashlib.sha256((p / 'degree8_linear_square.sage').read_bytes()).hexdigest()
rows = []
for fmask in (32280, 35349):
    for bmask in (68, 84):
        name = f'degree8_linear_square_f{fmask}_e{bmask}'
        data = json.loads((p / (name + '.json')).read_text())
        assert data['source_sha256'] == source_hash
        assert data['fmask'] == fmask and data['bmask'] == bmask
        assert data['target_order'] == 222
        assert data['hits'] == []
        counts = data['counts']
        assert list(sorted(counts)) == list(sorted(('LH2','LH4','LH6','LQiH2','LQiH4')))
        assert [counts[s]['assignments'] for s in ('LH2','LH4','LH6','LQiH2','LQiH4')] == [243,243,243,405,405]
        assert [counts[s]['logical_candidates'] for s in ('LH2','LH4','LH6','LQiH2','LQiH4')] == [58806,58806,58806,11859210,11859210]
        assert sum(c['point_filter_controls'] for c in counts.values()) == 1952100
        assert all(c['full_equation_hits'] == 0 for c in counts.values())
        rows.append({'fmask': fmask, 'bmask': bmask,
                     'logical_candidates': sum(c['logical_candidates'] for c in counts.values()),
                     'point_filter_controls': sum(c['point_filter_controls'] for c in counts.values()),
                     'point_survivors': sum(c['point_survivors'] for c in counts.values()),
                     'cpu_seconds': data['cpu_seconds'], 'exact_hits': 0})
print(json.dumps({'schema':'ghs.trace22.degree8-linear-square-audit/v1',
                  'source_sha256':source_hash,'rows':rows},sort_keys=True))
