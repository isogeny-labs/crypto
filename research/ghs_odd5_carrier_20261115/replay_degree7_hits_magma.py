#!/usr/bin/env python3
"""Replay saved degree-seven identities in the independent Magma algebra engine."""
import hashlib
import json
import ssl
import time
import urllib.request
from pathlib import Path

import certifi

P = Path(__file__).resolve().parent
masks = (30, 103, 139, 194, 233)
lines = [
    'F := GF(3); R0<z> := PolynomialRing(F);',
    'K<a> := ext<F | z^5+2*z+1>; R<x> := PolynomialRing(K);',
    'f := x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1;',
    'assert GCD(f, Derivative(f)) eq 1;',
]
for mask in masks:
    hit = json.loads((P / f'degree7_quadratic_square_{mask}.json').read_text())['hits'][0]
    lines.extend([
        'b := ' + ' + '.join(f'{(mask // (3**i)) % 3}*a^{i}' for i in range(5)) + ';',
        f'A := {hit["A"]};', f'B := {hit["B"]};',
        f'C := {hit["C"]};', f'H := {hit["H"]};',
        'assert Degree(A) eq 7 and Degree(B) eq 6 and Degree(C) eq 8;',
        'assert GCD(A,B) eq 1;',
        'assert f*C^2 eq B*(A^3+2*A^2*B+b*B^3);',
        f'print "PASS degree7 identity bmask={mask}";',
    ])
code = '\n'.join(lines) + '\n'
source = P / 'degree7_hits_replay.magma'
if source.exists():
    assert source.read_text() == code
else:
    source.write_text(code)
target = P / 'degree7_hits_replay_magma.json'
if target.exists():
    raise SystemExit('refusing to overwrite immutable receipt')
url = 'https://calculator.magma-maths.org/execute'
request = urllib.request.Request(url, json.dumps({'code': code}).encode(),
                                 {'Content-Type': 'application/json'}, method='POST')
start = time.time()
try:
    with urllib.request.urlopen(request, timeout=180,
                                context=ssl.create_default_context(cafile=certifi.where())) as response:
        status = response.status
        result = json.loads(response.read())
except Exception as exc:
    status = None
    result = {'transport_error': repr(exc)}
out = {'endpoint': url, 'input_sha256': hashlib.sha256(code.encode()).hexdigest(),
       'http_status': status, 'client_wall_sec': time.time()-start, 'response': result}
target.write_text(json.dumps(out, indent=2) + '\n')
print(json.dumps(out, indent=2))
