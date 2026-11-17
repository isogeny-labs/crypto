#!/usr/bin/env python3
"""Retain the full calculator response for the frozen order-4126 control."""

from hashlib import sha256
import json
from pathlib import Path
import ssl
import sys
import urllib.request

import certifi


HERE = Path(__file__).resolve().parent
URL = 'https://calculator.magma-maths.org/execute'


def main():
    allowed={'pilot','plane_pilot','base_pilot','attack_probe','attack','rho',
             'c1_controls'}
    allowed.update(f'{kind}_{order}' for kind in ('attack','rho')
                   for order in (4006,4054,4078))
    if len(sys.argv)!=2 or sys.argv[1] not in allowed:
        raise SystemExit('usage: run_magma.py <allowed Magma control name>')
    name=sys.argv[1]
    path=HERE/(name+'.magma')
    request=urllib.request.Request(
        URL,json.dumps({'code':path.read_text()}).encode(),
        {'Content-Type':'application/json'},method='POST')
    try:
        context=ssl.create_default_context(cafile=certifi.where())
        with urllib.request.urlopen(request,timeout=330,context=context) as response:
            status=response.status
            payload=json.loads(response.read())
    except Exception as exc:
        status=None
        payload={'transport_error':repr(exc)}
    receipt={'schema':'ghs-q16-4126-hess-g4-magma/v1',
             'endpoint':URL,'input_file':path.name,
             'input_sha256':sha256(path.read_bytes()).hexdigest(),
             'http_status':status,'response':payload}
    (HERE/(name+'_receipt.json')).write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps({'status':status,'success':payload.get('success'),
                      'exit_code':payload.get('exit_code'),
                      'truncated':payload.get('truncated'),
                      'stdout_bytes':len(payload.get('stdout',''))},indent=2))


if __name__=='__main__':
    main()
