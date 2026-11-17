from pathlib import Path
import json,re,time
P=Path('/home/ec2-user/crypto-pencil-signed-graph-20261029/research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/weighted-nonbranch-norm-composition');B=Path('/home/ec2-user/crypto-pr2304-persist/research/projective_law_followup_20261022/formal');T=Path('/home/ec2-user/.elan/toolchains/leanprover--lean4---v4.34.1');packages=sorted((B/'.lake/packages').iterdir());roots=[B]+packages+[T/'src/lean'];cache=[B/'.lake/build/lib/lean']+[p/'.lake/build/lib/lean' for p in packages]+[T/'lib/lean'];pending=['WeightedProjNonbranchDVR','WeightedProjQuinticHead','Common22SourceHead','Mathlib.Data.ZMod.Basic','Init'];done={};missing=[];start=time.monotonic()
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
while pending:
 assert time.monotonic()-start<110,'read-only closure cutoff'
 n=pending.pop()
 if n in done:continue
 rel=n.replace('.','/');src=next((r/(rel+'.lean') for r in roots if (r/(rel+'.lean')).is_file()),None)
 if src is None:missing.append(n);done[n]={'missing_source':True};continue
 raw=src.read_text();clean=uncomment(raw);im=[]
 for line in re.findall(r'^\s*(?:(?:public|private)\s+)?import\s+([^\n]+)',clean,re.M):
  im.extend(x for x in line.split() if re.fullmatch(r'[A-Za-z_][A-Za-z_0-9.]*',x) and x!='all')
 if not re.search(r'^\s*prelude\b',clean,re.M) and n!='Init':im.append('Init')
 obj=next((r/(rel+'.olean') for r in cache if (r/(rel+'.olean')).is_file()),None)
 files=[str(src)];files+= [str(Path(str(obj)+suffix)) for suffix in ['', '.private','.server'] if obj is not None and Path(str(obj)+suffix).is_file()]
 done[n]={'source':str(src),'object':str(obj) if obj else None,'imports':im,'files':files,'bytes':sum(Path(f).stat().st_size for f in files)};pending.extend(im)
 if len(done)%100==0:(P/'closure-progress.json').write_text(json.dumps({'modules':len(done),'pending':len(pending),'elapsed':time.monotonic()-start})+'\n')
result={'modules':done,'missing_sources':missing,'missing_objects':[n for n,v in done.items() if not v.get('object')],'module_count':len(done),'file_count':sum(len(v.get('files',[])) for v in done.values()),'bytes':sum(v.get('bytes',0) for v in done.values()),'elapsed_seconds':time.monotonic()-start,'compiler_invocations':0,'source_parser':'Nested block/line comments stripped; imports and implicit Init recursively resolved. Does not certify olean/source correspondence.'}
(P/'resolved-import-closure.json').write_text(json.dumps(result,indent=2)+'\n');print({k:v for k,v in result.items() if k!='modules'})
