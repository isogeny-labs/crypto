#!/usr/bin/env python3
"""Submit the thirteen frozen alternate-target survivors to public Magma."""
import hashlib
import json
import ssl
import time
import urllib.request
from pathlib import Path
import certifi

P = Path(__file__).resolve().parent
source = (P / 'degree5_isogeny_survivors.magma').read_bytes()
assert hashlib.sha256(source).hexdigest() == 'a7085464e17f89309fca5200c15862d4ec0f95dba1161856d6b0a37a422ed97c'
target = P / 'degree5_isogeny_survivors_magma.json'
if target.exists():
    raise SystemExit('refusing to overwrite immutable receipt')
url = 'https://calculator.magma-maths.org/execute'
request = urllib.request.Request(url, json.dumps({'code': source.decode()}).encode(),
                                 {'Content-Type': 'application/json'}, method='POST')
started = time.time()
try:
    with urllib.request.urlopen(request, timeout=180,
                                context=ssl.create_default_context(cafile=certifi.where())) as response:
        status = response.status
        result = json.loads(response.read())
except Exception as exc:
    status = None
    result = {'transport_error': repr(exc)}
out = {'endpoint': url, 'input_sha256': hashlib.sha256(source).hexdigest(),
       'http_status': status, 'client_wall_sec': time.time()-started,
       'response': result}
target.write_text(json.dumps(out, indent=2) + '\n')
print(json.dumps(out, indent=2))
