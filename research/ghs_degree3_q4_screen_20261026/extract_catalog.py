#!/usr/bin/env python3
"""Extract two exact GF(4) cubic Jacobian classes from local LMFDB mirror."""

import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent
ARCHIVE = Path('/Volumes/data/workspaces/lmfdb-public-mirror-20261019/resumable')
SOURCE = ROOT.parent / 'ghs_binary_cubic_isogeny_20261026/results.json'
ZSTD = '/opt/homebrew/bin/zstd'
WANTED = {'3.4.a_a_ah': 58, '3.4.a_a_ad': 62}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    source = json.loads(SOURCE.read_text())
    for order in WANTED.values():
        assert any(z['order'] == order for z in source['source_population'])
    complete = json.loads((ARCHIVE / 'COMPLETE.json').read_text())
    assert complete['verification']['hashes_verified'] is True
    assert complete['verification']['missing_count'] == 0
    plan = json.loads((ARCHIVE / 'plan.json').read_text())
    table = next(z for z in plan['tables'] if z['name'] == 'av_fq_isog')
    cols = table['columns']
    fields = ('label', 'g', 'q', 'poly_str', 'curves', 'hyp_count', 'has_jacobian')
    index = {name: cols.index(name) for name in fields}
    directory = ARCHIVE / 'chunks' / ('table_' + table['oid'])
    chunks = sorted(directory.glob('*.copy.zst'))
    assert len(chunks) == table['chunks']
    rows = {}
    for chunk in chunks:
        with subprocess.Popen([ZSTD, '-dc', str(chunk)], stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE) as proc:
            for line in proc.stdout:
                cells = line.rstrip(b'\n').split(b'\t')
                label = cells[index['label']].decode()
                if label not in WANTED:
                    continue
                assert label not in rows
                row = {name: cells[pos].decode() for name, pos in index.items()}
                assert row['g'] == '3' and row['q'] == '4'
                poly = [int(v) for v in row['poly_str'].split()]
                assert poly == [1, 0, 0, -(65-WANTED[label]), 0, 0, 64]
                rows[label] = row
            error = proc.stderr.read().decode(errors='replace')
            if proc.wait():
                raise RuntimeError(f'{chunk.name}: {error[:300]}')
    assert set(rows) == set(WANTED)
    output = {'schema': 'ghs-q4-degree3-catalog/v1',
              'archive_complete_sha256': sha(ARCHIVE / 'COMPLETE.json'),
              'archive_plan_sha256': sha(ARCHIVE / 'plan.json'),
              'source_sha256': sha(SOURCE),
              'table_oid': table['oid'],
              'rows': [rows[label] for label in sorted(rows)]}
    (ROOT / 'catalog_models_raw.json').write_text(json.dumps(output, indent=2)+'\n')
    print('extracted', len(rows), 'rows',
          [(r['label'], r['hyp_count']) for r in output['rows']], flush=True)


if __name__ == '__main__':
    main()
