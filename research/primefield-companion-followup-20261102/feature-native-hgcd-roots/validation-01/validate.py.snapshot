"""Bounded native software checks and a normal-build dependency exclusion check."""
import pathlib,sys,subprocess,json,hashlib,tempfile,re
D=pathlib.Path(__file__).resolve().parent;R=D.parent
out=pathlib.Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
files=set()
def visit(p):
 p=p.resolve()
 if p in files:return
 files.add(p)
 if p.suffix=='.rs':
  for x in re.findall(r'(?:path\s*=\s*|include(?:_str|_bytes)?!\(\s*)"([^"]+)"',p.read_text()):visit(p.parent/x)
  for name in re.findall(r'\bmod\s+(\w+)\s*;',p.read_text()):
   for q in [p.parent/(name+'.rs'),p.parent/name/'mod.rs']:
    if q.exists():visit(q)
visit(D/'lib.rs')
rec={'scope':'half-GCD root update software checks; no timing panel','compiler':subprocess.check_output(['/home/ec2-user/.cargo/bin/rustc','-Vv'],text=True),'source_hashes':{str(p.relative_to(R)):sha(p)for p in sorted(files)},'commands':[]}
with tempfile.TemporaryDirectory(prefix='standalone-source-')as temp:
 binary=pathlib.Path(temp)/'tests';normal=pathlib.Path(temp)/'production.rmeta';dep=pathlib.Path(temp)/'production.d'
 cmds=[('build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','--test','-C','opt-level=2','-C','overflow-checks=yes',str(D/'lib.rs'),'-o',str(binary)],90),('test',[str(binary),'tests::','--skip','tests::reference::','--skip','polynomial::','--skip','seed::','--skip','source::','--skip','group::','--test-threads=1','--nocapture'],30),('normal-build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','--crate-type=lib','--emit=metadata,dep-info='+str(dep),str(D/'lib.rs'),'-o',str(normal)],90)]
 for name,cmd,limit in cmds:
  try:z=subprocess.run(cmd,capture_output=True,timeout=limit);code=z.returncode;a=z.stdout;b=z.stderr
  except subprocess.TimeoutExpired as e:code=124;a=e.stdout or b'';b=e.stderr or b''
  (out/(name+'.stdout')).write_bytes(a);(out/(name+'.stderr')).write_bytes(b);rec['commands'].append(dict(name=name,argv=cmd,timeout=limit,exit=code,stdout_sha256=hashlib.sha256(a).hexdigest(),stderr_sha256=hashlib.sha256(b).hexdigest()))
  if code:break
 rec['binary_hashes']={p.name:sha(p)for p in [binary,normal]if p.exists()}
 if dep.exists():
  text=dep.read_text();(out/'production.d').write_text(text)
  forbidden=['feature-native-source/','feature-native-division/','feature-native-normalized-transport/','feature-native-half-gcd/transport.rs','feature-native-monomial-recovery/','feature-native-batch-reciprocals/']
  rec['normal_dependency_exclusions']={s:s not in text for s in forbidden}
rec['sources_unchanged']=all(sha(R/n)==h for n,h in rec['source_hashes'].items());rec['passed']=len(rec['commands'])==3 and all(c['exit']==0 for c in rec['commands'])and rec['sources_unchanged'] and len(rec.get('normal_dependency_exclusions',{}))==6 and all(rec['normal_dependency_exclusions'].values())
(out/'receipt.json').write_text(json.dumps(rec,indent=2)+'\n');print('passed',rec['passed'],'sources',len(files));sys.exit(0 if rec['passed']else 1)
