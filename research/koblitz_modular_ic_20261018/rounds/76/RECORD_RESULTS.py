"""Verify the complete R76 native receipt against its frozen source."""
from datetime import datetime
from pathlib import Path
import hashlib, json, re, subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
raw = h / 'runs/quadratic76-01'
assert sorted(p.name for p in (h / 'runs').iterdir() if p.is_dir()) == [
    'quadratic-lean76-01', 'quadratic-lean76-02', 'quadratic76-01']
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
revision = (raw / 'source-revision.txt').read_text().strip()
log = (raw / 'native.log').read_text().splitlines()
factor_lines = [s for s in log if s.startswith('QUAD76_FACTOR ')]
result_lines = [s for s in log if s.startswith('QUAD76_RESULT ')]
expected = [
    'QUAD76_FACTOR fiber=0 mask=1 e=14 V=square_zero_nonzero',
    'QUAD76_FACTOR fiber=0 mask=3 e=2 V=square_zero_nonzero',
    'QUAD76_FACTOR fiber=1 mask=6 e=1 V=1',
    'QUAD76_FACTOR fiber=1 mask=7 e=7 V=1',
    'QUAD76_FACTOR fiber=1 mask=8 e=1 V=0',
    'QUAD76_FACTOR fiber=1 mask=10 e=7 V=0',
]
assert factor_lines == expected
assert result_lines == ['QUAD76_RESULT branches=6 zero_fiber=2 one_fiber_V0=2 one_fiber_V1=2 all_source_checks=1']
assert int((raw / 'exit.txt').read_text()) == 0
assert log.count('QUADRATIC76_ALL_CHECKS_PASS') == 1
assert not [s for s in log if re.match(r'^\s*\?', s)]
pins = []
for local, source in [('input.sing', 'QUADRATIC76.sing'), ('driver.sh', 'NATIVE_DRIVER.sh'),
                      ('launcher.sh', 'REPLAY_CAS.sh'), ('PROTOCOL.md', 'PROTOCOL.md'),
                      ('protocol.json', 'protocol.json'), ('donors.json', 'donors.json')]:
    path = str((h / source).relative_to(repo))
    assert subprocess.check_output(['git', 'show', revision + ':' + path], cwd=repo) == (raw / local).read_bytes()
    pins.append({'artifact': local, 'git_source': path, 'revision': revision, 'sha256': digest(raw / local)})
for donor in json.loads((raw / 'donors.json').read_text()):
    p = h / donor['path']
    assert p.stat().st_size == donor['bytes'] and digest(p) == donor['sha256']
start = (raw / 'started-utc.txt').read_text().strip()
finish = (raw / 'finished-utc.txt').read_text().strip()
row = {'attempt': raw.name, 'source_revision': revision, 'kind': 'Singular', 'status': 'accepted',
       'native_exit': 0, 'marker': 'QUADRATIC76_ALL_CHECKS_PASS', 'marker_present': True,
       'error_count': 0, 'factor_lines': factor_lines, 'result_lines': result_lines,
       'started_utc': start, 'finished_utc': finish,
       'native_wall_seconds': (datetime.fromisoformat(finish.replace('Z', '+00:00')) - datetime.fromisoformat(start.replace('Z', '+00:00'))).total_seconds(),
       'artifacts': [{'path': str(p.relative_to(h)), 'sha256': digest(p), 'bytes': p.stat().st_size}
                     for p in sorted(raw.iterdir()) if p.is_file()]}
results = {'schema': 'koblitz.results/v1', 'round': 76, 'runs': [row], 'input_pins': pins,
           'donor_hashes': len(json.loads((h / 'donors.json').read_text()))}
(h / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
export = {'schema': 'experiment.export/v1', 'experiment_id': 'KOBLITZ-R76-QUADRATIC-FIBER-CLASSIFICATION',
          'claim_class': 'exact_computation',
          'protocols': [{'attempt': raw.name, 'path': 'runs/' + raw.name + '/protocol.json',
                         'sha256': digest(raw / 'protocol.json'), 'frozen_revision': revision}],
          'raw_archive': 'runs.tar.xz; content address in .artifacts.json after custody publication',
          'results': {'path': 'results.json', 'sha256': digest(h / 'results.json')},
          'flow': {'accepted': 1, 'rejected': 0, 'interrupted': 0},
          'canonical_schema_validation': 'unavailable in selected Git index and disk; manual contract/source-byte checks only',
          'notes': 'Complete quadratic-coordinate classification of all six rational finite branches; derived b,c valuations in CM.md.'}
(h / 'export.json').write_text(json.dumps(export, indent=2) + '\n')
print(json.dumps({'accepted': 1, 'source_byte_pins': len(pins), 'factors': len(factor_lines)}))
