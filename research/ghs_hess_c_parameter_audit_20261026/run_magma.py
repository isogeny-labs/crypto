#!/usr/bin/env python3
"""Submit one frozen Magma input and retain its entire HTTP response."""

import hashlib
import json
import ssl
import sys
import urllib.request
from pathlib import Path

import certifi

ROOT = Path(__file__).resolve().parent
URL = 'https://calculator.magma-maths.org/execute'


def main():
    allowed = {'pilot', 'check_556', 'map_556', 'map_class_556', 'canonical_556', 'aut_548', 'aut_556', 'cert_548', 'index_548', 'canonical_map_548', 'canonical_map_556', 'field_embed', 'basechange_548', 'index_548_q64', 'index_rho_548', 'index_rho_556'} | {f'all_{order}' for order in (482, 524, 548, 554, 556)}
    if len(sys.argv) != 2 or sys.argv[1] not in allowed:
        raise SystemExit('usage: run_magma.py pilot|check_556|map_556|map_class_556|canonical_556|aut_556|cert_548|index_548|all_482|all_524|all_548|all_554|all_556')
    name = sys.argv[1]
    path = ROOT / (name + '.magma')
    body = json.dumps({'code': path.read_text()}).encode()
    request = urllib.request.Request(URL, body,
                                     {'Content-Type': 'application/json'},
                                     method='POST')
    try:
        context = ssl.create_default_context(cafile=certifi.where())
        with urllib.request.urlopen(request, timeout=330, context=context) as response:
            status = response.status
            payload = json.loads(response.read())
    except Exception as exc:
        status = None
        payload = {'transport_error': repr(exc)}
    receipt = {'schema': 'ghs-hess-c-parameter-magma/v1', 'endpoint': URL,
               'input_file': path.name,
               'input_sha256': hashlib.sha256(path.read_bytes()).hexdigest(),
               'http_status': status, 'response': payload}
    (ROOT / (name + '_receipt.json')).write_text(json.dumps(receipt, indent=2) + '\n')
    print(json.dumps({'name': name, 'status': status,
                      'success': payload.get('success'),
                      'exit_code': payload.get('exit_code'),
                      'truncated': payload.get('truncated'),
                      'stdout_bytes': len(payload.get('stdout', ''))},
                     indent=2))


if __name__ == '__main__':
    main()
