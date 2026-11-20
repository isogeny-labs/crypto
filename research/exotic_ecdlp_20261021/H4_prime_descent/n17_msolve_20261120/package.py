#!/usr/bin/env python3
"""Losslessly package or restore the two frozen H4 n17 msolve inputs."""
import base64
import gzip
import hashlib
import json
import sys
import textwrap
from pathlib import Path

HERE=Path(__file__).resolve().parent
def sha(data):return hashlib.sha256(data).hexdigest()

def records():
    prep=json.loads((HERE/'preparation.json').read_text())
    assert prep['status']=='PASS'
    return prep['records']

def pack():
    output=[]
    for row in records():
        raw=(HERE/row['input_file']).read_bytes()
        assert sha(raw)==row['input_sha256']
        encoded=base64.b64encode(gzip.compress(raw,mtime=0)).decode('ascii')
        target=HERE/(row['input_file']+'.gzip.b64.txt')
        target.write_text('\n'.join(textwrap.wrap(encoded,76))+'\n')
        assert gzip.decompress(base64.b64decode(target.read_text()))==raw
        output.append({'arm':row['arm'],'raw_file':row['input_file'],
                       'raw_sha256':sha(raw),'raw_bytes':len(raw),
                       'encoded_file':target.name,'encoded_sha256':sha(target.read_bytes()),
                       'encoded_bytes':target.stat().st_size})
    result={'status':'PASS','preparation_sha256':sha((HERE/'preparation.json').read_bytes()),
            'records':output}
    (HERE/'packaging.json').write_text(json.dumps(result,sort_keys=True,indent=2)+'\n')
    print(json.dumps({'status':'PASS','encoded_bytes':sum(x['encoded_bytes'] for x in output)}))

def restore():
    result=json.loads((HERE/'packaging.json').read_text())
    assert result['status']=='PASS'
    assert result['preparation_sha256']==sha((HERE/'preparation.json').read_bytes())
    for item in result['records']:
        encoded=(HERE/item['encoded_file']).read_bytes()
        assert sha(encoded)==item['encoded_sha256']
        raw=gzip.decompress(base64.b64decode(encoded))
        assert len(raw)==item['raw_bytes'] and sha(raw)==item['raw_sha256']
        target=HERE/item['raw_file']
        target.write_bytes(raw)
        assert sha(target.read_bytes())==item['raw_sha256']
    print(json.dumps({'status':'PASS','restored_files':len(result['records'])}))

if __name__=='__main__':
    assert len(sys.argv)==2 and sys.argv[1] in ('pack','restore')
    pack() if sys.argv[1]=='pack' else restore()
