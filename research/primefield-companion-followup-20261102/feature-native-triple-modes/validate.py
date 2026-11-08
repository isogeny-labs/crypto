"""Software qualification only: known-answer counters and inherited regression."""
from pathlib import Path
import hashlib,json,subprocess,sys,tempfile
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
records=json.loads((D/'generation.json').read_text())
for row in records:
 original=(D/row['source']).read_bytes();generated=(D/row['output']).read_bytes()
 assert sha(original)==row['source_sha256'] and sha(generated)==row['sha256']
 t=generated.decode()
 for old,new in reversed(row['edits']):
  if not old:assert t.endswith(new);t=t[:-len(new)]
  else:assert new in t;t=t.replace(new,old)
 assert t.encode()==original
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
rc={'scope':'progression shared inverse exact controls and inherited regression; no comparative panel','reversible_sources':len(records),'commands':[],'source_hashes':{f.name:sha(f.read_bytes()) for f in D.iterdir() if f.suffix in ['.rs','.py','.json']}}
compiler='/home/ec2-user/.cargo/bin/rustc';rc['compiler']=subprocess.check_output([compiler,'-Vv'],text=True)
with tempfile.TemporaryDirectory(prefix='interval-meter-')as tmp:
 binary=Path(tmp)/'check'
 commands=[('build',[compiler,'--edition=2021','-O','-C','overflow-checks=yes','--emit=link,dep-info',str(D/'modes.rs'),'-o',str(binary)],90),('self-test',[str(binary)],60),('inherited',[str(binary),'--inherited'],60)]
 for name,cmd,limit in commands:
  try:r=subprocess.run(cmd,capture_output=True,timeout=limit);code=r.returncode;stdout=r.stdout;stderr=r.stderr
  except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
  (out/(name+'.stdout')).write_bytes(stdout);(out/(name+'.stderr')).write_bytes(stderr)
  rc['commands'].append(dict(name=name,command=cmd,limit_seconds=limit,exit=code,stdout_sha256=sha(stdout),stderr_sha256=sha(stderr)))
  if name=='build' and code==0:
   rc['binary_sha256']=sha(binary.read_bytes())
   paths=binary.with_suffix('.d').read_text().splitlines()[0].split(': ',1)[1].split()
   rc['dependencies']={p:sha(Path(p).read_bytes()) for p in paths}
  if code:break
reference=D.parent/'feature-native-planned-width/validation01/check.stdout'
rc['inherited_reference']=str(reference);rc['reference_sha256']=sha(reference.read_bytes())
rc['inherited_byte_equal']=(out/'inherited.stdout').exists() and (out/'inherited.stdout').read_bytes()==reference.read_bytes()
rc['passed']=len(rc['commands'])==3 and all(x['exit']==0 for x in rc['commands']) and rc['inherited_byte_equal']
(out/'receipt.json').write_text(json.dumps(rc,indent=2)+'\n');print(json.dumps({k:rc[k] for k in ['passed','reversible_sources','inherited_byte_equal']}));sys.exit(0 if rc['passed'] else 1)
