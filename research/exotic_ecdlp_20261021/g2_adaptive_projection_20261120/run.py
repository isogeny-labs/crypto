#!/usr/bin/env python3
"""Repair a frozen information set one witnessed column at a time."""
import hashlib
import json
import struct
import subprocess
import tempfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
HEADER = struct.Struct('<8sIIII')
U16 = struct.Struct('<H')

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

def project(source, dest, selected):
    positions = {old: new for new, old in enumerate(selected)}
    nnz = 0
    with Path(source).open('rb') as inp, Path(dest).open('wb') as out:
        header = HEADER.unpack(inp.read(HEADER.size))
        assert header == (b'G2M4R1\0\0', 16399, 18352, 32, 4)
        out.write(HEADER.pack(header[0], 16399, len(selected), 32, 4))
        for _ in range(16399):
            weight, = U16.unpack(inp.read(2))
            raw = inp.read(2 * weight)
            assert len(raw) == 2 * weight
            columns = struct.unpack('<' + 'H' * weight, raw)
            assert all(a < b for a, b in zip(columns, columns[1:]))
            chosen = sorted(positions[col] for col in columns if col in positions)
            out.write(U16.pack(len(chosen)))
            if chosen:
                out.write(struct.pack('<' + 'H' * len(chosen), *chosen))
            nnz += len(chosen)
        assert inp.read(1) == b''
    return nnz

def main():
    protocol = json.loads((HERE / 'protocol.json').read_text())
    assert protocol['status'] == 'frozen'
    assert sha(HERE / 'run.py') == protocol['source_sha256']
    for name, item in protocol['sources'].items():
        assert sha(item['path']) == item['sha256'], name
    panel = json.loads(Path(protocol['sources']['panel']['path']).read_text())
    witnesses = json.loads(Path(protocol['sources']['witnesses']['path']).read_text())
    base = panel['information_set_columns']
    assert len(base) == 15856 and base == sorted(set(base))
    controls = {case['label']: case for case in panel['cases']}
    result = {'status': 'PASS', 'protocol_sha256': sha(HERE / 'protocol.json'), 'cases': []}
    with tempfile.TemporaryDirectory(prefix='g2-adaptive-') as tmp:
        for witness in witnesses['cases']:
            label = witness['label']
            source = protocol['sources'][label]
            assert witness['outside_union_count'] == 11
            assert witness['outside_union_pivots'] == sorted(witness['outside_union_pivots'])
            assert not set(base).intersection(witness['outside_union_pivots'])
            steps = []
            first_full = None
            for count in range(12):
                selected = sorted(base + witness['outside_union_pivots'][:count])
                dest = Path(tmp) / 'projected.bin'
                nnz = project(source['path'], dest, selected)
                proc = subprocess.run([protocol['sources']['ranker']['path'], str(dest)],
                                      capture_output=True, text=True, timeout=30, check=True)
                rank = json.loads(proc.stdout)
                assert rank['rows'] == 16399 and rank['columns'] == len(selected)
                assert rank['nnz'] == nnz
                steps.append({'added': count, 'columns': len(selected), 'nnz': nnz,
                              'rank': rank['rank'], 'stream_sha256': sha(dest),
                              'rank_wall_ns_exploratory': rank['wall_ns_exploratory']})
                if rank['rank'] == 15841 and first_full is None:
                    first_full = count
                dest.unlink()
                if first_full is not None and count == 11:
                    break
            assert steps[0]['rank'] == controls[label]['projected_rank'] == 15835
            assert steps[-1]['rank'] == controls[label]['expected_rank'] == 15841
            assert first_full is not None
            result['cases'].append({'label': label, 'q_bits': witness['q_bits'],
                                    'first_full_rank_added': first_full, 'steps': steps})
    print(json.dumps(result, sort_keys=True, indent=2))

if __name__ == '__main__':
    main()
