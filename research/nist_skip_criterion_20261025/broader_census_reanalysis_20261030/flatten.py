#!/usr/bin/env python3
"""Lossless field selection/label join only; no curve arithmetic or statistics."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
classes = json.loads((root / 'sources/classification.json').read_text())['rows']
lookup = {(r['stream'], r['index']): r for r in classes}
assert len(lookup) == len(classes) == 69
records = [json.loads(s) for s in (root / 'sources/certificates.jsonl').read_text().splitlines()]
assert records[-1]['record'] == 'completed' and records[-1]['status'] == 'validated'
branches = [r for r in records if r['record'] == 'certificate']
assert len(branches) == 82
bits = {'prime192v2': 192, 'P-224': 224, 'prime239v3': 239,
        'P-256': 256, 'P-384': 384, 'P-521': 521}
seen = set()
out = ['# branch\tseed_id\tbits\trole\tcategory\tside\tnarrow\tp\tb\tn\tx\ty\tseed\trun']
for r in sorted(branches, key=lambda r: (bits[r['target']], r['index'], r['side'])):
    key = (r['target'], r['index'])
    c = lookup[key]
    assert r['side'] in c['branches'] and r['seed'] == c['seed']
    assert (key, r['side']) not in seen
    seen.add((key, r['side']))
    for flag in ('hasse_interval_narrower_than_n', 'n_in_hasse_interval',
                 'n_prime_pari', 'n_times_point_identity', 'on_curve', 'order_certified'):
        assert r[flag] is True
    seed_id = f"p{bits[r['target']]}-{r['index']}"
    values = [seed_id + '-' + r['side'], seed_id, str(bits[r['target']]),
              'published' if c['label'] == 'published' else 'unpublished',
              c['label'], r['side'],
              '1' if c['label'] in ('published', 'retrospective_omission') else '0',
              r['p'], r['b'], r['n'], r['point_x'], r['point_y'], r['seed'], str(r['run'])]
    assert all('\t' not in v and '\n' not in v for v in values)
    out.append('\t'.join(values))
assert seen == {(k, side) for k, row in lookup.items() for side in row['branches']}
target = root / 'inputs.tsv'
assert not target.exists()
target.write_text('\n'.join(out) + '\n')
print('Flattened all 82 certified branches at 69 positions; no arithmetic performed')
