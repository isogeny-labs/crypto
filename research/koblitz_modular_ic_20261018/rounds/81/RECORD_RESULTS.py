"""Validate all four candidate F4 coordinate maps against frozen raw runs."""
from datetime import datetime
from pathlib import Path
import hashlib
import json
import re
import subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
specs = [('candidate-v0-81-01', 0), ('candidate-v1-81-01', 1)]
assert sorted(p.name for p in (h / 'runs').iterdir() if p.is_dir()) == sorted(x[0] for x in specs)
rows = []
pins = []
maps = []
for name, component in specs:
    raw = h / 'runs' / name
    rev = (raw / 'source-revision.txt').read_text().strip()
    code = int((raw / 'exit.txt').read_text())
    log = (raw / 'native.log').read_text().splitlines()
    errors = [s for s in log if re.match(r'^\s*\?', s)]
    marker = f'CANDIDATE_V{component}_81_ALL_CHECKS_PASS'
    assert code == 0 and not errors and log.count(marker) == 1, name
    assert [s for s in log if s.startswith('CAND81_RESULT ')] == [
        f'CAND81_RESULT component={component} full_dim=16 reduced_dim=4 primes=2 maps_verified=2 candidate_only=1'
    ]
    starts = [re.fullmatch(r'CAND81_BEGIN component=(\d+) prime=(\d+) generator_index=(\d+)', s)
              for s in log if s.startswith('CAND81_BEGIN ')]
    assert len(starts) == 2 and all(starts)
    assert [(int(x[1]), int(x[2])) for x in starts] == [(component, 1), (component, 2)]
    mrows = [re.fullmatch(r'CAND81_MAP component=(\d+) prime=(\d+) variable=(\d+) name=(\S+) code=(\d+)', s)
             for s in log if s.startswith('CAND81_MAP ')]
    assert len(mrows) == 44 and all(mrows)
    assert [s for s in log if s.startswith('CAND81_PRIME ')] == [
        f'CAND81_PRIME component={component} index={i} degree=2 map_verified=1' for i in (1, 2)]
    local_maps = []
    for branch in (1, 2):
        entries = [x for x in mrows if int(x[2]) == branch]
        assert len(entries) == 22
        assert [int(x[3]) for x in entries] == list(range(1, 23))
        assert all(int(x[1]) == component and 0 <= int(x[5]) <= 3 for x in entries)
        names = [x[4] for x in entries]
        assert names[-3:] == ['x', 'y', 'p']
        codes = [int(x[5]) for x in entries]
        assert codes[-2:] == [component, 0]
        assert codes[-3] == (2 if component == 0 else 0)
        gen = int(starts[branch - 1][3])
        assert 1 <= gen <= 22 and codes[gen - 1] == 2
        local_maps.append({'component': component, 'prime': branch, 'generator_index': gen,
                           'variable_names': names, 'codes_0_1_g_gplus1': codes,
                           'source_xyp_codes': codes[-3:], 'ideal_equality_verified': True})
    assert local_maps[0]['variable_names'] == local_maps[1]['variable_names']
    differences = [i for i in range(1, 23) if local_maps[0]['codes_0_1_g_gplus1'][i - 1] !=
                   local_maps[1]['codes_0_1_g_gplus1'][i - 1]]
    assert differences and all(i < 20 for i in differences)
    for row in local_maps:
        row['distinguishing_variable_indices'] = differences
    maps += local_maps
    source_input = 'ZERO81.sing' if component == 0 else 'ONE81.sing'
    for local, source in [('input.sing', source_input), ('driver.sh', 'NATIVE_DRIVER.sh'),
                          ('launcher.sh', 'REPLAY_CAS.sh'), ('PROTOCOL.md', 'PROTOCOL.md'),
                          ('protocol.json', 'protocol.json'), ('donors.json', 'donors.json')]:
        source_path = str((h / source).relative_to(repo))
        assert subprocess.check_output(['git', 'show', rev + ':' + source_path], cwd=repo) == (raw / local).read_bytes()
        pins.append({'attempt': name, 'artifact': local, 'git_source': source_path,
                     'revision': rev, 'sha256': digest(raw / local)})
    for donor in json.loads((raw / 'donors.json').read_text()):
        path = h / donor['path']
        assert path.stat().st_size == donor['bytes'] and digest(path) == donor['sha256']
    start = (raw / 'started-utc.txt').read_text().strip()
    finish = (raw / 'finished-utc.txt').read_text().strip()
    rows.append({'attempt': name, 'kind': 'Singular', 'component': component,
                 'status': 'accepted', 'source_revision': rev, 'native_exit': code,
                 'marker_present': True, 'error_count': 0, 'maps_verified': 2,
                 'started_utc': start, 'finished_utc': finish,
                 'native_wall_seconds': (datetime.fromisoformat(finish.replace('Z', '+00:00')) -
                                         datetime.fromisoformat(start.replace('Z', '+00:00'))).total_seconds(),
                 'artifacts': [{'path': str(p.relative_to(h)), 'sha256': digest(p), 'bytes': p.stat().st_size}
                               for p in sorted(raw.iterdir()) if p.is_file()]})
assert len(pins) == 12 and len(maps) == 4
results = {'schema': 'koblitz.results/v1', 'round': 81, 'runs': rows,
           'input_pins': pins, 'donor_hashes': 8, 'candidate_maps': maps,
           'normality_certified': False, 'source_field_compatibility_certified': False}
(h / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
export = {'schema': 'experiment.export/v1', 'experiment_id': 'KOBLITZ-R81-EXPLICIT-CANDIDATE-F4-MAPS',
          'claim_class': 'exact_computation',
          'protocols': [{'attempt': row['attempt'], 'path': 'runs/' + row['attempt'] + '/protocol.json',
                         'sha256': digest(h / 'runs' / row['attempt'] / 'protocol.json'),
                         'frozen_revision': row['source_revision']} for row in rows],
          'raw_archive': 'runs.tar.xz; content address in .artifacts.json after custody publication',
          'results': {'path': 'results.json', 'sha256': digest(h / 'results.json')},
          'flow': {'accepted': 2, 'rejected': 0, 'interrupted': 0},
          'canonical_schema_validation': 'unavailable in selected Git index and disk; manual contract/source-byte checks only',
          'notes': 'Four explicit F4 maps of rejected candidate prime quotients; source coordinate x,y,p images checked; no normality claim.'}
(h / 'export.json').write_text(json.dumps(export, indent=2) + '\n')
print(json.dumps({'accepted': 2, 'candidate_maps': 4, 'source_byte_pins': 12,
                  'distinguishing_indices': [maps[0]['distinguishing_variable_indices'],
                                             maps[2]['distinguishing_variable_indices']]}))
