from pathlib import Path
import hashlib,json,subprocess,sys,tempfile
D=Path(__file__).resolve().parent;sha=lambda b:hashlib.sha256(b).hexdigest()
out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
for row in json.loads((D/'generation.json').read_text()):
 t=(D/row['output']).read_text();assert sha(t.encode())==row['sha256']
 for old,new in reversed(row['edits']):
  if not old:assert t.endswith(new);t=t[:-len(new)]
  else:assert t.count(new)==1;t=t.replace(new,old)
 assert t==(D/row['source']).read_text()
rc={'scope':'disjoint r41/m5 software fixture and synthetic parser mutations only; production panel not executed','commands':[]}
rustc='/home/ec2-user/.cargo/bin/rustc';rc['compiler']=subprocess.check_output([rustc,'-Vv'],text=True)
with tempfile.TemporaryDirectory(prefix='width-panel-')as tmp:
 binary=Path(tmp)/'panel'
 for name,cmd,limit in [('build',[rustc,'--edition=2021','-O','-C','overflow-checks=yes','--emit=link,dep-info',str(D/'main.rs'),'-o',str(binary)],90),('native',[str(binary),'--self-test'],30),('parser',[sys.executable,str(D/'test_analyze.py')],30)]:
  try:r=subprocess.run(cmd,capture_output=True,timeout=limit);code=r.returncode;stdout=r.stdout;stderr=r.stderr
  except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
  (out/(name+'.stdout')).write_bytes(stdout);(out/(name+'.stderr')).write_bytes(stderr);rc['commands'].append(dict(name=name,command=cmd,limit_seconds=limit,exit=code))
  if name=='build'and code==0:
   rc['binary_sha256']=sha(binary.read_bytes());paths=binary.with_suffix('.d').read_text().splitlines()[0].split(': ',1)[1].split();rc['dependencies']={p:sha(Path(p).read_bytes()) for p in paths}
  if code:break
rc['passed']=len(rc['commands'])==3 and all(x['exit']==0 for x in rc['commands'])
rc['source_hashes']={f.name:sha(f.read_bytes())for f in D.iterdir()if f.suffix in ['.rs','.py','.json']}
(out/'receipt.json').write_text(json.dumps(rc,indent=2)+'\n');print(json.dumps({'passed':rc['passed']}));sys.exit(0 if rc['passed'] else 1)
