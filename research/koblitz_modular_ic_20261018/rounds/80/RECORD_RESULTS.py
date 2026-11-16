"""Recheck both frozen candidate prime censuses and bind their raw bytes."""
from datetime import datetime
from pathlib import Path
import hashlib
import json
import re
import subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
specs = [('candidate-v0-80-01', 0), ('candidate-v1-80-01', 1)]
assert sorted(p.name for p in (h / 'runs').iterdir() if p.is_dir()) == sorted(x[0] for x in specs)
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rows = []
pins = []
for name, component in specs:
    raw = h / 'runs' / name
    rev = (raw / 'source-revision.txt').read_text().strip()
    code = int((raw / 'exit.txt').read_text())
    log = (raw / 'native.log').read_text().splitlines()
    errors = [s for s in log if re.match(r'^\s*\?', s)]
    marker = f'CANDIDATE_V{component}_80_ALL_CHECKS_PASS'
    assert code == 0 and not errors and log.count(marker) == 1, name
    assert [s for s in log if s.startswith('CAND80_STAGE ')] == [
        f'CAND80_STAGE component={component} full_dim=16 radical_begin=1',
        f'CAND80_STAGE component={component} reduced_dim=4 primes_begin=1',
    ], name
    assert [s for s in log if s.startswith('CAND80_PRIME ')] == [
        f'CAND80_PRIME component={component} index=1 residue_degree=2',
        f'CAND80_PRIME component={component} index=2 residue_degree=2',
    ], name
    assert [s for s in log if s.startswith('CAND80_RESULT ')] == [
        f'CAND80_RESULT component={component} full_dim=16 reduced_dim=4 prime_count=2 prime_degree_sum=4 candidate_only=1'
    ], name
    source_input = 'ZERO80.sing' if component == 0 else 'ONE80.sing'
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
                 'marker_present': True, 'error_count': 0, 'full_dimension': 16,
                 'reduced_dimension': 4, 'prime_degrees': [2, 2],
                 'started_utc': start, 'finished_utc': finish,
                 'native_wall_seconds': (datetime.fromisoformat(finish.replace('Z', '+00:00')) -
                                         datetime.fromisoformat(start.replace('Z', '+00:00'))).total_seconds(),
                 'artifacts': [{'path': str(p.relative_to(h)), 'sha256': digest(p),
                                'bytes': p.stat().st_size} for p in sorted(raw.iterdir()) if p.is_file()]})
assert len(pins) == 12
results = {'schema': 'koblitz.results/v1', 'round': 80, 'runs': rows,
           'input_pins': pins, 'donor_hashes': 7,
           'candidate_full_dimensions': [16, 16],
           'candidate_reduced_dimensions': [4, 4],
           'candidate_prime_degrees': [[2, 2], [2, 2]],
           'normality_certified': False}
(h / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
export = {'schema': 'experiment.export/v1', 'experiment_id': 'KOBLITZ-R80-CANDIDATE-REDUCED-U0',
          'claim_class': 'exact_computation',
          'protocols': [{'attempt': row['attempt'], 'path': 'runs/' + row['attempt'] + '/protocol.json',
                         'sha256': digest(h / 'runs' / row['attempt'] / 'protocol.json'),
                         'frozen_revision': row['source_revision']} for row in rows],
          'raw_archive': 'runs.tar.xz; content address in .artifacts.json after custody publication',
          'results': {'path': 'results.json', 'sha256': digest(h / 'results.json')},
          'flow': {'accepted': 2, 'rejected': 0, 'interrupted': 0},
          'canonical_schema_validation': 'unavailable in selected Git index and disk; manual contract/source-byte checks only',
          'notes': 'Both rejected-candidate fibers have length 16, reduced dimension 4 and two degree-2 prime quotients; no normality claim.'}
(h / 'export.json').write_text(json.dumps(export, indent=2) + '\n')
print(json.dumps({'accepted': 2, 'source_byte_pins': 12,
                  'candidate_reduced_dimensions': [4, 4], 'candidate_prime_degrees': [[2, 2], [2, 2]]}))
