"""Replay the exact frozen component fixture, including its retained failure."""
import hashlib
import importlib.util
import json
from pathlib import Path
import re
from polynomial_replay import verify_row, verify_field_row

ROOT = Path(__file__).resolve().parent
FIXTURE_SHA = '0698e33d00c8e0c53e31b478da9f7cca0c277306cac085da3777fb2023a3c4d0'

def replay(reader):
    fixture = ROOT / 'fixture/snapshot.bin'
    assert fixture.stat().st_size == 24509
    assert hashlib.sha256(fixture.read_bytes()).hexdigest() == FIXTURE_SHA
    with fixture.open('rb') as stream:
        doc = reader.inspect_selected([stream])
    rows, fields = [], []
    for pi, partition in enumerate(doc['retained']['continuation']['partitions']):
        for ni, node in enumerate(partition['search']['nodes']):
            native = node['native']
            if native is None:
                continue
            for ri, row in enumerate(native['basis']):
                rows.append({'partition': pi, 'node': ni, 'basis_row': ri,
                             'replay': verify_row(2, node['equations'], row)})
            for bi, batch in enumerate(native['batches']):
                for ti, task in enumerate(batch['tasks']):
                    match = re.fullmatch(r'Field\((\d+), (\d+)\)', task['kind'])
                    if match and batch['stage'] == 'Committed':
                        basis, variable = map(int, match.groups())
                        fields.append({'partition': pi, 'node': ni, 'batch': bi,
                                       'initial_row': ti, 'task': task['kind'],
                                       'replay': verify_field_row(2, node['equations'],
                                           native['basis'][basis], variable, batch['initial'][ti])})
    return {'fixture_sha256': FIXTURE_SHA, 'bytes': 24509, 'variables': 2,
            'variable_count_source': 'Rust fixture producer Session::new(2,...); wire expansion is absent',
            'producer_outcome': 'Err(Audit), due nested certificate maximum reservation Cap before BQI invoked',
            'rows': rows, 'field_rows': fields, 'source_authenticated': False}

if __name__ == '__main__':
    import argparse
    parser = argparse.ArgumentParser()
    parser.add_argument('--reader', type=Path, default=ROOT.parent/'owned-search-semantic-reader-20261107/semantic_reader.py')
    args = parser.parse_args()
    spec = importlib.util.spec_from_file_location('fixture_semantic_reader', args.reader)
    reader = importlib.util.module_from_spec(spec)
    import sys
    sys.path.insert(0, str(args.reader.resolve().parent))
    sys.modules[spec.name] = reader
    spec.loader.exec_module(reader)
    print(json.dumps(replay(reader), indent=2))
