#!/usr/bin/env python3
"""Publish terminal audit files as immutable S3 objects under the r35 run."""
import concurrent.futures
import hashlib
import json
import subprocess
from pathlib import Path

root=Path(__file__).resolve().parent
store=json.loads((root/'S3_PERSISTENCE_INDEX.json').read_text())
files=store['files']

def upload(row):
    src=root/row['path']
    proc=subprocess.run(['aws','s3','cp',str(src),row['s3_uri'],'--only-show-errors','--region','us-east-2'],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    return {'path':row['path'],'s3_uri':row['s3_uri'],'bytes':row['bytes'],'sha256':row['sha256'],'exit_code':proc.returncode,'stderr':proc.stderr.decode(errors='replace')[-1000:]}
with concurrent.futures.ThreadPoolExecutor(max_workers=8) as pool:
    results=list(pool.map(upload,files))
failed=[r for r in results if r['exit_code']]
index_path=root/'S3_PERSISTENCE_INDEX.json'
raw=index_path.read_bytes(); index_sha=hashlib.sha256(raw).hexdigest()
index_uri=f"s3://isolab-results-104186811093-us-east-2/runs/n83-fb-variants-r35-20261031-r1/evidence/terminal-20261101T0633Z/index/S3_PERSISTENCE_INDEX-{index_sha}.json"
if not failed:
    proc=subprocess.run(['aws','s3','cp',str(index_path),index_uri,'--only-show-errors','--region','us-east-2'],stdout=subprocess.PIPE,stderr=subprocess.PIPE)
    if proc.returncode: failed.append({'path':'S3_PERSISTENCE_INDEX.json','s3_uri':index_uri,'exit_code':proc.returncode,'stderr':proc.stderr.decode(errors='replace')[-1000:]})
report={'schema':'n83.factor-base-evidence-publication/v1','job_name':'n83-fb-variants-r35-20261031-r1','uploaded_files':len(results)-len([r for r in results if r['exit_code']]),'planned_files':len(files),'index_sha256':index_sha,'index_uri':index_uri,'failures':failed}
(root/'s3-publish-receipt.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
if failed: raise SystemExit(1)
