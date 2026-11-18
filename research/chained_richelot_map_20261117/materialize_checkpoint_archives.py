"""Restore exact immutable containers using Git I/O and content hashes only.

No arithmetic, compiler, test, benchmark or scientific verifier is invoked.
"""
from pathlib import Path
import argparse,hashlib,json,os,re,shutil,subprocess

p=argparse.ArgumentParser(description=__doc__)
p.add_argument('--only',action='append',default=[],help='Declared path or unique basename')
p.add_argument('--all',action='store_true',help='Restore all declared containers')
p.add_argument('--fetch',action='store_true',help='Allow ordinary origin fetch of the immutable checkpoint')
a=p.parse_args();study=Path(__file__).resolve().parent;repo=study.parents[1]
manifest=json.loads((study/'IMMUTABLE_RAW_ARTIFACTS.json').read_text())
assert manifest['schema']=='chained.immutable-artifact-references/v1'
rows=manifest['artifacts'];selected=rows if a.all else [x for x in rows if x['path'] in a.only or Path(x['path']).name in a.only]
for row in rows:
 assert re.fullmatch('[0-9a-f]{40}',row['Git_blob']) and re.fullmatch('[0-9a-f]{40}',row['immutable_public_commit'])
 assert re.fullmatch('[0-9a-f]{64}',row['sha256']) and type(row['bytes']) is int and row['bytes']>0
 assert row['path'].startswith('research/chained_richelot_map_20261117/') and row['path'].endswith('.xz')
if not a.all and not a.only:
 for row in rows:print(row['path'],row['bytes'],row['sha256'])
 raise SystemExit(0)
for name in a.only:assert sum(name in [x['path'],Path(x['path']).name] for x in rows)==1,name
def git(*argv,allow_lazy=False):
 env=dict(os.environ)
 if allow_lazy:env.pop('GIT_NO_LAZY_FETCH',None)
 return subprocess.run(['git','-c','gc.auto=0',*argv],cwd=repo,env=env,stdout=subprocess.PIPE,stderr=subprocess.PIPE)
for row in selected:
 dest=repo/row['path'];assert dest.resolve().is_relative_to(study.resolve())
 if dest.exists():payload=dest.read_bytes()
 else:
  assert shutil.disk_usage(dest.parent).free>=row['bytes'],('insufficient container storage',row['path'])
  result=git('cat-file','blob',row['Git_blob'],allow_lazy=a.fetch)
  if result.returncode and a.fetch:
   fetched=git('fetch','--filter=blob:none','--no-tags','--no-write-fetch-head','origin',row['immutable_public_commit'])
   assert fetched.returncode==0,fetched.stderr
   result=git('cat-file','blob',row['Git_blob'],allow_lazy=True)
  assert result.returncode==0,(row['path'],result.stderr)
  payload=result.stdout
 assert len(payload)==row['bytes'] and hashlib.sha256(payload).hexdigest()==row['sha256'],row['path']
 assert hashlib.sha1(b'blob '+str(len(payload)).encode()+b'\0'+payload).hexdigest()==row['Git_blob']
 if not dest.exists():
  with dest.open('xb') as f:f.write(payload)
 print('exact complete container',row['path'],row['bytes'])
