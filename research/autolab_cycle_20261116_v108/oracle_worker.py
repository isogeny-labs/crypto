from pathlib import Path
import contextlib
import io
import json
import runpy
import subprocess
import sys
from research.discriminant_roots_20261116_v108.kernel import Field, gcd, solve

C = Path(__file__).resolve().parent
ROOT = C.parents[1]
f = Field(3, [0, 1])
for padding in range(10):
    assert gcd([0, 1, 1], [0] * padding, f) == [0, 1, 1]
    assert gcd([0] * padding, [0, 1, 1], f) == [0, 1, 1]
    assert gcd([0] * padding, [0] * padding, f) == []
for method in ['direct', 'radical', 'initial']:
    assert solve([0, 0, 1, 1, 1, 1], f, method, 1061611)['roots'] == [[0, 2], [2, 1]]
capture = io.StringIO()
with contextlib.redirect_stdout(capture):
    runpy.run_module('research.discriminant_roots_20261116_v108.oracles', run_name='__main__')
result = json.loads(capture.getvalue())
assert result['status'] == 'passed', result
smoke = []
for group in ['binary', 'odd']:
    command = [sys.executable, '-m', 'research.discriminant_roots_20261116_v108.verified_profile_wrapper', 'harness-smoke-' + group, 'smoke_' + group + '_raw', group, '--smoke']
    child = subprocess.run(command, capture_output=True, text=True, timeout=30)
    assert child.returncode == 0, child.stderr
    record = json.loads(child.stdout)
    assert record['status'] == 'passed', record
    smoke.append({'group': group, 'rows': len(record['evidence']['out']['runs']), 'passed': True})
(C / 'HARNESS_SMOKE.json').open('x').write(json.dumps(smoke, indent=2) + '\n')
result['evidence']['harness_smoke'] = smoke
print(json.dumps(result))
