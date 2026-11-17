#!/usr/bin/env python3
"""Submit a public exact Magma input and retain the full response."""

import hashlib
import json
import ssl
import sys
import urllib.request
from pathlib import Path

import certifi

ROOT=Path(__file__).resolve().parent
URL='https://calculator.magma-maths.org/execute'
ORDERS={'482','524','554','556','554_basechange','554_quotients','554_elliptic_quotient','554_two_isogenies','554_isogeny_paths'}


def main():
    if len(sys.argv)!=2 or sys.argv[1] not in ORDERS:
        raise SystemExit('usage: run_panel.py 482|524|554|556|554_basechange|554_quotients|554_elliptic_quotient|554_two_isogenies|554_isogeny_paths')
    name=sys.argv[1]
    path=ROOT/(name+'.magma')
    body=json.dumps({'code':path.read_text()}).encode()
    request=urllib.request.Request(URL,body,{'Content-Type':'application/json'},
                                   method='POST')
    try:
        context=ssl.create_default_context(cafile=certifi.where())
        with urllib.request.urlopen(request,timeout=330,context=context) as response:
            status=response.status
            payload=json.loads(response.read())
    except Exception as exc:
        status=None
        payload={'transport_error':repr(exc)}
    receipt={'schema':'ghs-q8-cover-panel-magma/v1','endpoint':URL,
             'input_file':path.name,
             'input_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
             'http_status':status,'response':payload}
    (ROOT/(name+'_receipt.json')).write_text(json.dumps(receipt,indent=2)+'\n')
    print(json.dumps({'order':name,'status':status,'response':payload},indent=2))


if __name__=='__main__':
    main()
