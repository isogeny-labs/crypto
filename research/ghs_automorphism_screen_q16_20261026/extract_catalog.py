#!/usr/bin/env python3
"""Extract the GF(16), dimension-three curve rows from a verified LMFDB archive."""

import hashlib
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parent
ARCHIVE = Path('/Volumes/data/workspaces/lmfdb-public-mirror-20261019/resumable')
CERT = ROOT.parent / 'ghs_cubic_catalog_q16_20261026/certificate.json'
ZSTD = '/opt/homebrew/bin/zstd'


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    complete = json.loads((ARCHIVE / 'COMPLETE.json').read_text())
    assert complete['verification']['missing_count'] == 0
    assert complete['verification']['hashes_verified'] is True
    plan = json.loads((ARCHIVE / 'plan.json').read_text())
    table = next(row for row in plan['tables'] if row['name'] == 'av_fq_isog')
    cols = table['columns']
    index = {name: cols.index(name) for name in
             ('label', 'g', 'q', 'poly_str', 'curves', 'hyp_count', 'has_jacobian')}
    wanted = {row['catalog_label'] for row in json.loads(CERT.read_text())['orders']}
    assert len(wanted) == 128
    chunk_dir = ARCHIVE / 'chunks' / ('table_' + table['oid'])
    chunks = sorted(chunk_dir.glob('*.copy.zst'))
    assert len(chunks) == table['chunks']
    rows = {}
    for chunk_no, chunk in enumerate(chunks, 1):
        with subprocess.Popen([ZSTD, '-dc', str(chunk)], stdout=subprocess.PIPE,
                              stderr=subprocess.PIPE) as proc:
            for line in proc.stdout:
                fields = line.rstrip(b'\n').split(b'\t')
                label = fields[index['label']].decode('utf-8')
                if label not in wanted:
                    continue
                assert label not in rows
                row = {name: fields[pos].decode('utf-8') for name, pos in index.items()}
                assert row['g'] == '3' and row['q'] == '16'
                rows[label] = row
            error = proc.stderr.read().decode('utf-8', errors='replace')
            if proc.wait() != 0:
                raise RuntimeError(f'{chunk.name}: {error[:300]}')
        print(f'chunk {chunk_no}/{len(chunks)} matching_rows={len(rows)}', flush=True)
    assert set(rows) == wanted, f'missing={sorted(wanted - set(rows))}'
    output = {'schema': 'ghs-q16-catalog-models/v1',
              'archive_complete_sha256': sha(ARCHIVE / 'COMPLETE.json'),
              'archive_plan_sha256': sha(ARCHIVE / 'plan.json'),
              'source_certificate_sha256': sha(CERT),
              'table_oid': table['oid'], 'rows': [rows[label] for label in sorted(rows)]}
    (ROOT / 'catalog_models_raw.json').write_text(json.dumps(output, indent=2) + '\n')
    print(f'extracted {len(rows)} labels', flush=True)


if __name__ == '__main__':
    main()
