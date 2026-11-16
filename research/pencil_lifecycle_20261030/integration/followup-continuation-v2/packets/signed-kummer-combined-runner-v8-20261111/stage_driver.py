"""Charged staging of exact driver sources and an actually generated published lock."""
import os,json,hashlib,stat,time
from pathlib import Path
from durable_io import load_admission,atomic_json
P=Path(__file__).resolve().parent;A=load_admission(P);W=Path(A['workspace'])
ROOT=W/'driver-source';END=float(os.environ['SIGNED_BUILD_END_MONOTONIC'])
def guard():
 assert time.monotonic()<END-20,'shared build deadline'
def checked_copy(src,dst,sha,size):
 guard();src=Path(src);assert stat.S_ISREG(src.lstat().st_mode)
 assert src.stat().st_size==size and hashlib.sha256(src.read_bytes()).hexdigest()==sha
 dst.parent.mkdir(parents=True,exist_ok=True)
 with dst.open('xb') as f:f.write(src.read_bytes());f.flush();os.fsync(f.fileno())
 assert hashlib.sha256(dst.read_bytes()).hexdigest()==sha
 guard()
ROOT.mkdir(exist_ok=False)
inputs=json.loads((P/'driver-inputs.json').read_text());rows={}
for row in inputs['files']:
 rel=Path(row['destination']);assert not rel.is_absolute() and '..' not in rel.parts
 checked_copy(row['source'],ROOT/rel,row['sha256'],row['bytes']);rows[str(rel)]={'sha256':row['sha256'],'bytes':row['bytes']}
crate=ROOT/inputs['crate']
for kind,lock in A['generated_locks'].items():
 target=ROOT/inputs['workspaces'][kind]/'Cargo.lock'
 checked_copy(lock['path'],target,lock['sha256'],lock['bytes'])
 rows[str(target.relative_to(ROOT))]={'sha256':lock['sha256'],'bytes':lock['bytes']}
fixture=json.loads((crate/'fixture.json').read_text())
for rel,key in [('research/kummer_theta_20261022/results/theta-models.jsonl','theta'),('research/kummer_coordinates_20261022/results/generic-models.jsonl','model'),('research/kummer_coordinates_20261022/results/target-gate-16.jsonl','target')]:
 guard();raw=(json.dumps(fixture[key],separators=(',',':'))+'\n').encode();path=ROOT/rel;path.parent.mkdir(parents=True,exist_ok=True)
 with path.open('xb') as f:f.write(raw);f.flush();os.fsync(f.fileno())
 rows[rel]={'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw)}
atomic_json(W/'driver-source-manifest.json',rows)
