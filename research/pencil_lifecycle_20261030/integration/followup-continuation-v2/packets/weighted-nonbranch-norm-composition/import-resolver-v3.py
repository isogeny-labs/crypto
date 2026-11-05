from pathlib import Path
import json,re,time,hashlib,os,signal,tempfile
P=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/weighted-nonbranch-norm-composition');B=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal');T=Path('/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1');packages=sorted((B/'.lake/packages').iterdir());roots=[B]+packages+[T/'src/lean'];cache=[B/'.lake/build/lib/lean']+[p/'.lake/build/lib/lean' for p in packages]+[T/'lib/lean'];initial=['WeightedProjNonbranchDVR','WeightedProjQuinticHead','Common22SourceHead','Mathlib.Data.ZMod.Basic','Init'];H=lambda b:hashlib.sha256(b).hexdigest();start=time.monotonic()
config={'source_roots':list(map(str,roots)),'cache_roots':list(map(str,cache)),'root_modules':initial,'SYSROOT':str(T),'parser':'header-only module/prelude/repeated public private meta/import all; v3','work_limit_seconds':170};config_hash=H(json.dumps(config,sort_keys=True).encode());W=Path(tempfile.mkdtemp(prefix='nonbranch-import-v3-',dir='/var/tmp'));checkpoint=W/'checkpoint.json';(P/'resolver-v3-workspace.txt').write_text(str(W)+'\n');done={};pending=list(initial);status='incomplete';error=None
# Direct module path probes only; no recursive tree census.
def atomic(path,obj):
 tmp=path.with_suffix('.tmp')
 with tmp.open('w') as f:json.dump(obj,f,sort_keys=True);f.write('\n');f.flush();os.fsync(f.fileno())
 os.replace(tmp,path);fd=os.open(path.parent,os.O_DIRECTORY);os.fsync(fd);os.close(fd)
def save():atomic(checkpoint,{'config':config,'config_sha256':config_hash,'done':done,'pending':pending,'status':status,'elapsed_seconds':time.monotonic()-start,'error':error})
def interrupted(s,f):raise RuntimeError('signal '+str(s))
signal.signal(signal.SIGTERM,interrupted);signal.signal(signal.SIGINT,interrupted)
def uncomment(t):
 out=[];depth=0;i=0
 while i<len(t):
  if t[i:i+2]=='/-':depth+=1;i+=2
  elif depth and t[i:i+2]=='-/':depth-=1;i+=2
  elif not depth and t[i:i+2]=='--':
   j=t.find('\n',i);i=len(t) if j<0 else j
  else:
   if not depth:out.append(t[i])
   i+=1
 return ''.join(out)
def header_imports(raw):
 clean=uncomment(raw);imports=[];prelude=False
 for line in clean.splitlines():
  line=line.strip()
  if not line:continue
  if line=='module':continue
  if line=='prelude':prelude=True;continue
  m=re.fullmatch(r'(?:(?:public|private|meta)\s+)*import\s+(.+)',line)
  if not m:break
  tokens=m.group(1).split()
  if tokens and tokens[0]=='all':tokens=tokens[1:]
  if not tokens or not all(re.fullmatch(r'[A-Za-z_][A-Za-z_0-9.]*',x) for x in tokens):raise RuntimeError('ambiguous import header '+line)
  imports.extend(tokens)
 return imports,prelude
fixtures=[('module\npublic meta import Foo.Bar\npublic section\ndef s := "\nimport React from X"',(['Foo.Bar'],False)),('prelude\nimport all Init.Prelude\n',(['Init.Prelude'],True)),('/- outer /- nested -/ end -/\n-- comment\nprivate meta import A B\nnamespace X',(['A','B'],False))]
for raw,wanted in fixtures:assert header_imports(raw)==wanted
predecessor=Path('/var/tmp/nonbranch-import-v2-gnr3xti3/checkpoint.json');prior=json.loads(predecessor.read_text());prior_sources={v['source']:v['files'][0]['sha256'] for v in prior['done'].values() if v.get('source')};config['predecessor_checkpoint_sha256']=H(predecessor.read_bytes());config['fixtures_passed']=len(fixtures);config_hash=H(json.dumps(config,sort_keys=True).encode())
try:
 save()
 while pending:
  if time.monotonic()-start>=170:raise RuntimeError('read-only work cutoff')
  n=pending[-1]
  if n in done:pending.pop();continue
  rel=n.replace('.','/');sources=[r/(rel+'.lean') for r in roots if (r/(rel+'.lean')).is_file()];objs=[r/(rel+'.olean') for r in cache if (r/(rel+'.olean')).is_file()]
  if not sources:done[n]={'missing_source':True,'objects':list(map(str,objs))};pending.pop();continue
  src=sources[0];raw=src.read_bytes()
  if str(src) in prior_sources:assert H(raw)==prior_sources[str(src)],'predecessor source changed '+str(src)
  im,prelude=header_imports(raw.decode())
  if not prelude and n!='Init':im.append('Init')
  obj=objs[0] if objs else None;files=[{'path':str(src),'realpath':str(src.resolve()),'sha256':H(raw),'bytes':len(raw),'kind':'source'}]
  for suffix in ['', '.private','.server']:
   f=Path(str(obj)+suffix) if obj else None
   if f and f.is_file():files.append({'path':str(f),'realpath':str(f.resolve()),'bytes':f.stat().st_size,'kind':'import-object','sha256':None})
  done[n]={'source':str(src),'object':str(obj) if obj else None,'imports':im,'files':files,'shadowed_sources':list(map(str,sources[1:])),'shadowed_objects':list(map(str,objs[1:]))}
  pending.pop();pending.extend(im)
  if len(done)%100==0:save()
 status='resolved'
except BaseException as e:error=repr(e)
finally:
 save();allfiles={v['path']:v for row in done.values() for v in row.get('files',[])};summary={'status':status,'error':error,'checkpoint':str(checkpoint),'checkpoint_sha256':H(checkpoint.read_bytes()),'config_sha256':config_hash,'module_count':len(done),'pending_count':len(pending),'files':len(allfiles),'bytes':sum(v['bytes'] for v in allfiles.values()),'missing_sources':[n for n,v in done.items() if v.get('missing_source')],'missing_objects':[n for n,v in done.items() if not v.get('object')],'shadowed_modules':[n for n,v in done.items() if v.get('shadowed_sources') or v.get('shadowed_objects')],'source_hashes_retained':True,'object_hashes_pending':True,'elapsed_seconds':time.monotonic()-start,'compiler_invocations':0};atomic(W/'receipt.json',summary);(P/'resolver-v3-receipt.json').write_text(json.dumps(summary,indent=2)+'\n');print(json.dumps(summary));raise SystemExit(0 if status=='resolved' else 1)
