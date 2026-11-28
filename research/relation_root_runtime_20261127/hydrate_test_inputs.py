"""Git file-metadata restoration only, for manifest-root literal concat paths."""
from pathlib import Path
import hashlib,json,re,subprocess,base64
ROOT=Path(__file__).resolve().parents[2];S=Path(__file__).resolve().parent
pattern=re.compile(r'concat!\s*\(\s*env!\s*\(\s*"CARGO_MANIFEST_DIR"\s*\)\s*,\s*"([^"\n]+)"\s*,?\s*\)',re.S)
targets=set()
for folder in ['src','crates']:
 for p in (ROOT/folder).rglob('*.rs'):
  for text in pattern.findall(p.read_text(errors='replace')):
   target=Path(str(ROOT)+text).resolve()
   if target.is_relative_to(ROOT):targets.add(target.relative_to(ROOT).as_posix())
entries={}
for line in subprocess.check_output(['git','ls-tree','-r','HEAD','--',*sorted(targets)],cwd=ROOT,text=True).splitlines():
 meta,path=line.split('\t',1);mode,kind,blob=meta.split()
 if mode=='100644' and kind=='blob' and path in targets:entries[path]=blob
with(S/'hydration-repair/UNIT_CONCAT_REQUEST.json').open('x') as f:json.dump({'files':entries,'directories_or_nonfile_targets':sorted(targets-entries.keys()),'before_fetch':True,'runtime_directories_not_synthesized':True},f,indent=2);f.write('\n')
records={}
for path,blob in sorted(entries.items()):
 target=ROOT/path
 if target.is_file():
  raw=target.read_bytes();actual=hashlib.sha1(('blob '+str(len(raw))+'\0').encode()+raw).hexdigest()
  if actual!=blob:raise RuntimeError('existing source mismatch: '+path)
  route='already exact'
 else:
  response=json.loads(subprocess.check_output(['gh','api','repos/isogeny-labs/crypto/git/blobs/'+blob],text=True))
  raw=base64.b64decode(response['content']);actual=hashlib.sha1(('blob '+str(len(raw))+'\0').encode()+raw).hexdigest()
  if actual!=blob or response['sha']!=blob:raise RuntimeError('Git blob transport mismatch')
  subprocess.run(['git','hash-object','-w','--stdin'],input=raw,stdout=subprocess.DEVNULL,check=True)
  subprocess.run(['git','-c','gc.auto=0','-c','maintenance.auto=false','restore','--worktree','--ignore-skip-worktree-bits','--',path],check=True)
  if target.read_bytes()!=raw:raise RuntimeError('restored bytes mismatch')
  route='exact GitHub blob and Git restore'
 records[path]={'git_blob':blob,'sha256':hashlib.sha256(raw).hexdigest(),'bytes':len(raw),'route':route}
with(S/'hydration-repair/UNIT_CONCAT_RECEIPT.json').open('x') as f:json.dump({'files':records,'complete_parser':False,'no_generated_or_historical_substitutes':True,'numerical_execution':False},f,indent=2);f.write('\n')
print(json.dumps({'verified_files':len(records),'nonfile_targets':len(targets-entries.keys())}))
