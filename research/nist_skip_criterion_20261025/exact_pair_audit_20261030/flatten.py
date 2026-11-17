#!/usr/bin/env python3
"""Copy source integer score/label fields only; do not compute statistics."""
import json
import sys
from pathlib import Path

root = Path(__file__).resolve().parent
target = Path(sys.argv[1])
assert not target.exists()
source = json.loads((root / 'sources/heldout.json').read_text())
rows = ['# source_id\tseed_id\tbits\trole\tcategory\tstrict691']
for r in source['heldout']:
    rows.append('\t'.join([
        'heldout:' + r['stream'] + ':' + str(r['index']),
        'p' + str(r['bits']) + '-' + str(r['index']), str(r['bits']),
        'published' if r['label'] == 'published' else 'unpublished',
        r['kind'], str(r['split'])]))
for r in source['named19']:
    if r['label'] == 'omitted':
        _, bits, index = r['name'].split('_')
        seed, role, category = 'p' + bits + '-' + index, 'unpublished', 'retrospective_omission'
    else:
        seed, role, category = 'NA_no_seed_id_in_named_row', 'published', 'published'
    rows.append('\t'.join(['named:' + r['name'], seed, str(r['bits']),
                           role, category, str(r['split'])]))
target.write_text('\n'.join(rows) + '\n')
print('Copied source integer fields; no arithmetic or model recomputation')
