#!/usr/bin/env python3
"""Run and retain all frozen trace +22 carrier/source pairs locally."""
import concurrent.futures
import hashlib
import json
import os
import subprocess
from pathlib import Path

HERE=Path(__file__).resolve().parent
SOURCE=HERE/'screen_variant.sage'
PREPARSED=HERE/'screen_variant.sage.py'
FMASKS=(32280,35349,36380,51009,67186,106121,112490,113379,127268,168265,175537,175771)
BMASKS=(68,84)
source_sha=hashlib.sha256(SOURCE.read_bytes()).hexdigest()
assert PREPARSED.exists(), 'preparse the frozen Sage source before starting'
preparsed_sha=hashlib.sha256(PREPARSED.read_bytes()).hexdigest()

def run_one(pair):
    fm,bm=pair
    stem=f'f{fm}_e{bm}'
    output=HERE/f'{stem}.json'
    stderr=HERE/f'{stem}.stderr'
    if output.exists() and stderr.exists():
        data=json.loads(output.read_text())
        assert data['source_sha256']==source_sha and data['fmask']==fm and data['bmask']==bm
        return {'fmask':fm,'bmask':bm,'hits':len(data['hits']),'status':'reused'}
    assert not output.exists() and not stderr.exists(), f'partial final receipt for {stem}'
    otmp=HERE/f'{stem}.json.tmp'
    etmp=HERE/f'{stem}.stderr.tmp'
    assert not otmp.exists() and not etmp.exists(), f'partial temporary receipt for {stem}'
    with otmp.open('wb') as out, etmp.open('wb') as err:
        result=subprocess.run(['/usr/bin/time','-l','sage','-python',str(PREPARSED),str(fm),str(bm)],stdout=out,stderr=err)
    if result.returncode:
        raise RuntimeError(f'{stem} exit {result.returncode}; temporary files preserved')
    data=json.loads(otmp.read_text())
    assert data['source_sha256']==source_sha and data['fmask']==fm and data['bmask']==bm
    os.replace(otmp,output)
    os.replace(etmp,stderr)
    return {'fmask':fm,'bmask':bm,'hits':len(data['hits']),'status':'completed'}

jobs=[(fm,bm) for fm in FMASKS for bm in BMASKS]
rows=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
    futures={pool.submit(run_one,pair):pair for pair in jobs}
    for future in concurrent.futures.as_completed(futures):
        row=future.result()
        rows.append(row)
        print(json.dumps(row,sort_keys=True),flush=True)
rows.sort(key=lambda x:(x['fmask'],x['bmask']))
assert len(rows)==24
manifest={'schema':'ghs.odd5.trace22-carrier-variant-manifest/v1',
          'source_sha256':source_sha,'preparsed_sha256':preparsed_sha,
          'runner_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
          'jobs':rows,'total_hits':sum(row['hits'] for row in rows)}
target=HERE/'manifest.json'
assert not target.exists(), 'refusing to overwrite manifest'
target.write_text(json.dumps(manifest,indent=2,sort_keys=True)+'\n')
print(json.dumps({'total_jobs':len(rows),'total_hits':manifest['total_hits']}),flush=True)
