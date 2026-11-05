#!/usr/bin/env python3
"""Collect metadata only; arithmetic verification remains the native instrument.

Usage: collect_centered_panel.py RAW_DIRECTORY FROZEN_PROTOCOL FRESH_OUTPUT
The raw directory must exist, including for an empty S3 result listing.
No raw files are modified. Missing/partial cases retain their planned identities.
"""
import collections
import datetime
import hashlib
import json
import pathlib
import sys


def digest(data):
    return hashlib.sha256(data).hexdigest()


def collect(raw, protocol):
    assert raw.is_dir(), 'raw directory must exist'
    cases = protocol['extensions']['cases']
    assert len(cases) == 84 and len({c['key'] for c in cases}) == 84
    observations = []
    for case in cases:
        folder = raw / 'results' / case['key']
        row = dict(case, status='missing', errors=[], native_replay_verified=False)
        if folder.exists():
            row['status'] = 'partial'
            try:
                manifest = json.loads((folder / 'manifest.json').read_text())
                required = {'input.json', 'certificate.json'} | {
                    f'{prefix}{i}.poly' for prefix in ('A', 'F') for i in range(3)
                }
                names = [f['path'] for f in manifest]
                assert len(names) == len(set(names)) and set(names) == required, 'payload set'
                for item in manifest:
                    data = (folder / item['path']).read_bytes()
                    assert len(data) == item['bytes'], f"size: {item['path']}"
                    assert digest(data) == item['sha256'], f"digest: {item['path']}"
                data = (folder / 'input.json').read_bytes()
                assert digest(data) == case['sha256'], 'frozen input identity'
                inp = json.loads(data)
                cert = json.loads((folder / 'certificate.json').read_text())
                binding = cert['source_binding']
                assert binding['status'] == 'full_source_and_base_identity_verified'
                assert binding['binding'] == inp['binding'], 'embedded binding'
                for field in ('m', 'arm', 'target_origin'):
                    assert binding[field] == case[field], field
                assert cert['exact_lfd'] is None and cert['regularity'] is None
                row.update(status='payload_hashes_verified',
                           certificate_sha256=digest((folder / 'certificate.json').read_bytes()),
                           reported={k: cert[k] for k in (
                               'remainder_degree', 'projection_nonzero', 'centered_target',
                               'f_terms', 'exact_lfd', 'regularity')})
            except (OSError, ValueError, KeyError, TypeError, AssertionError) as exc:
                row['errors'].append(f'{type(exc).__name__}: {exc}')
        observations.append(row)
    terminal_path = raw / 'terminal.json'
    terminal = json.loads(terminal_path.read_text()) if terminal_path.exists() else None
    return {
        'schema': 'n83.centered-panel-collection/v1',
        'observed_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
        'protocol_sha256': protocol['freeze']['protocol_sha256'],
        'planned_cases': 84,
        'counts': dict(collections.Counter(r['status'] for r in observations)),
        'terminal': terminal,
        'arithmetic_recomputed_by_collector': False,
        'interpretation': 'Reported fields are producer observations. Hash checks do not establish arithmetic replay or exact LFD/regularity.',
        'rows': observations,
    }


if __name__ == '__main__':
    raw, protocol_file, output = map(pathlib.Path, sys.argv[1:])
    result = collect(raw, json.loads(protocol_file.read_text()))
    with output.open('x') as f:
        json.dump(result, f, indent=2)
        f.write('\n')
    print(json.dumps({'counts': result['counts'], 'output': str(output)}))
