#!/usr/bin/env python3
"""Submit public Magma input and retain the exact calculator response."""

import hashlib
import json
import ssl
import sys
import urllib.request
from pathlib import Path

import certifi

ROOT=Path(__file__).resolve().parent
URL='https://calculator.magma-maths.org/execute'


def main():
    if len(sys.argv)!=2 or sys.argv[1] not in {'base','geometric'}:
        raise SystemExit('usage: run_magma.py base|geometric')
    name=sys.argv[1]
    path=ROOT/(name+'.magma')
    code=path.read_text()
    body=json.dumps({'code':code}).encode()
    request=urllib.request.Request(URL,body,
                                   {'Content-Type':'application/json'},
                                   method='POST')
    try:
        context=ssl.create_default_context(cafile=certifi.where())
        with urllib.request.urlopen(request,timeout=330,context=context) as response:
            status=response.status
            payload=json.loads(response.read())
    except Exception as exc:
        payload={'transport_error':repr(exc)}
        status=None
    receipt={'schema':'ghs-q8-548-magma-audit/v1',
             'endpoint':URL,'input_file':path.name,
             'input_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
             'http_status':status,'response':payload}
    out=ROOT/(name+'_magma_receipt.json')
    out.write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps({'status':status,'response':payload},indent=2))


if __name__=='__main__':
    main()
