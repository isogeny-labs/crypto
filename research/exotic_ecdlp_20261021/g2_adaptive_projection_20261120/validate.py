#!/usr/bin/env python3
"""Independently rank threshold projections with M4RI echelonization."""
import json
import subprocess
import tempfile
from pathlib import Path
from run import HERE, project, sha

def main():
    protocol = json.loads((HERE / 'protocol.json').read_text())
    result = json.loads((HERE / 'result.json').read_text())
    assert result['status'] == 'PASS'
    for name, item in protocol['sources'].items():
        assert sha(item['path']) == item['sha256'], name
    panel = json.loads(Path(protocol['sources']['panel']['path']).read_text())
    witness = {c['label']: c for c in json.loads(Path(protocol['sources']['witnesses']['path']).read_text())['cases']}
    checked = []
    with tempfile.TemporaryDirectory(prefix='g2-adaptive-check-') as tmp:
        binary = Path(tmp) / 'rank_m4ri'
        command = ['clang++', '-O2', '-std=c++17',
                   '-I/private/var/tmp/sage-10.9-current/local/include',
                   str(HERE / 'rank_m4ri.cpp'),
                   '-L/private/var/tmp/sage-10.9-current/local/lib', '-lm4ri', '-o', str(binary)]
        subprocess.run(command, check=True, capture_output=True, text=True)
        for case in result['cases']:
            label = case['label']
            for added in (case['first_full_rank_added'] - 1, case['first_full_rank_added']):
                selected = sorted(panel['information_set_columns'] + witness[label]['outside_union_pivots'][:added])
                stream = Path(tmp) / 'matrix.bin'
                project(protocol['sources'][label]['path'], stream, selected)
                expected = case['steps'][added]
                assert sha(stream) == expected['stream_sha256']
                measured = int(subprocess.run([str(binary), str(stream)], check=True,
                                              capture_output=True, text=True, timeout=30).stdout)
                assert measured == expected['rank']
                checked.append({'label': label, 'added': added, 'rank': measured,
                                'stream_sha256': sha(stream)})
    print(json.dumps({'status': 'PASS', 'm4ri_source_sha256': sha(HERE / 'rank_m4ri.cpp'),
                      'threshold_checks': checked}, indent=2, sort_keys=True))

if __name__ == '__main__':
    main()
