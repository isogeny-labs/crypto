"""Record the inconclusive nonstandard-basis reduction and raw pins."""
from datetime import datetime
from pathlib import Path
import hashlib
import json
import subprocess

h = Path(__file__).resolve().parent
repo = h.parents[3]
raw = h / 'runs' / 'candidate-direct84-01'
assert [p.name for p in (h / 'runs').iterdir() if p.is_dir()] == [raw.name]
digest = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
rev = (raw / 'source-revision.txt').read_text().strip()
code = int((raw / 'exit.txt').read_text())
log = (raw / 'native.log').read_text().splitlines()
assert code == 0 and log == [
    'CAND84_STAGE loaded=1 module_strings=27 full_std_invoked=0',
    '// ** norid is no standard basis',
    'CAND84_INCONCLUSIVE index=2 nonzero_remainder=1',
]
pins = []
for local, source in [('input.sing', 'DIRECT84.sing'), ('driver.sh', 'NATIVE_DRIVER.sh'),
                      ('launcher.sh', 'REPLAY_CAS.sh'), ('PROTOCOL.md', 'PROTOCOL.md'),
                      ('protocol.json', 'protocol.json'), ('donors.json', 'donors.json')]:
    source_path = str((h / source).relative_to(repo))
    assert subprocess.check_output(['git', 'show', rev + ':' + source_path], cwd=repo) == (raw / local).read_bytes()
    pins.append({'artifact': local, 'git_source': source_path, 'revision': rev,
                 'sha256': digest(raw / local)})
for donor in json.loads((raw / 'donors.json').read_text()):
    p = Path(donor['path']) if donor['path'].startswith('/') else h / donor['path']
    assert p.stat().st_size == donor['bytes'] and digest(p) == donor['sha256']
start = (raw / 'started-utc.txt').read_text().strip()
finish = (raw / 'finished-utc.txt').read_text().strip()
run = {'attempt': raw.name, 'kind': 'Singular', 'status': 'rejected',
       'reason': 'first direct reduction returned nonzero remainder against nonstandard generators; inconclusive',
       'source_revision': rev, 'native_exit': code, 'marker_present': False,
       'fraction_polynomials_tried': 1, 'positive_memberships_certified': 0,
       'started_utc': start, 'finished_utc': finish,
       'native_wall_seconds': (datetime.fromisoformat(finish.replace('Z', '+00:00')) -
                               datetime.fromisoformat(start.replace('Z', '+00:00'))).total_seconds(),
       'artifacts': [{'path': str(p.relative_to(h)), 'sha256': digest(p), 'bytes': p.stat().st_size}
                     for p in sorted(raw.iterdir()) if p.is_file()]}
results = {'schema': 'koblitz.results/v1', 'round': 84, 'runs': [run],
           'input_pins': pins, 'donor_hashes': 10, 'fraction_polynomials_tried': 1,
           'positive_memberships_certified': 0, 'normality_certified': False}
(h / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
export = {'schema': 'experiment.export/v1', 'experiment_id': 'KOBLITZ-R84-CORRECTED-DIRECT-CANDIDATE-FRACTIONS',
          'claim_class': 'exact_computation',
          'protocols': [{'attempt': raw.name, 'path': 'runs/' + raw.name + '/protocol.json',
                         'sha256': digest(raw / 'protocol.json'), 'frozen_revision': rev}],
          'raw_archive': 'runs.tar.xz; content address in .artifacts.json after custody publication',
          'results': {'path': 'results.json', 'sha256': digest(h / 'results.json')},
          'flow': {'accepted': 0, 'rejected': 1, 'interrupted': 0},
          'canonical_schema_validation': 'unavailable in selected Git index and disk; manual contract/source-byte checks only',
          'notes': 'One inconclusive nonzero remainder against nonstandard generators; no fraction-membership or nonmembership claim.'}
(h / 'export.json').write_text(json.dumps(export, indent=2) + '\n')
print(json.dumps({'rejected': 1, 'tried': 1, 'certified_positive': 0, 'source_byte_pins': 6}))
