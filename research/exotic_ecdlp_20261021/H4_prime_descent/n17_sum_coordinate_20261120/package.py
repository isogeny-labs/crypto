#!/usr/bin/env python3
"""Losslessly package the frozen generated msolve inputs as text."""
import base64
import gzip
import hashlib
import json
import textwrap
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(data):return hashlib.sha256(data).hexdigest()

def main():
    screen=json.loads((HERE/'screen-result.json').read_text())
    assert screen['status']=='PASS'
    records=[]
    for row in screen['records']:
        for case in row['cases']:
            raw=(HERE/case['input_file']).read_bytes()
            assert sha(raw)==case['input_sha256']
            compressed=gzip.compress(raw,mtime=0)
            encoded=base64.b64encode(compressed).decode('ascii')
            text='\n'.join(textwrap.wrap(encoded,76))+'\n'
            target=HERE/(case['input_file']+'.gzip.b64.txt')
            target.write_text(text)
            assert gzip.decompress(base64.b64decode(target.read_text()))==raw
            records.append({'input_file':case['input_file'],'input_sha256':sha(raw),
                            'encoded_file':target.name,'encoded_sha256':sha(target.read_bytes()),
                            'raw_bytes':len(raw),'encoded_bytes':target.stat().st_size})
    result={'status':'PASS','screen_result_sha256':sha((HERE/'screen-result.json').read_bytes()),
            'records':records}
    (HERE/'packaging.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':'PASS','files':len(records),'encoded_bytes':sum(r['encoded_bytes'] for r in records)}))

if __name__=='__main__':main()
