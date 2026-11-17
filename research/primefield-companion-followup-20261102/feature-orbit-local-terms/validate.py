"""Bounded native software checks, no benchmark or experiment panel."""
import pathlib,sys,subprocess,json,hashlib,tempfile
D=pathlib.Path(__file__).resolve().parent
out=pathlib.Path(sys.argv[1]);out.mkdir(parents=True,exist_ok=False)
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
receipt={'source_sha256':sha(D/'local.rs'),'scope':'local Miller software identities, not timings','commands':[]}
with tempfile.TemporaryDirectory(prefix='orbit-local-')as tmp:
 binary=pathlib.Path(tmp)/'tests'
 for name,cmd,timeout in [('build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','--test','-C','opt-level=2','-C','overflow-checks=yes',str(D/'local.rs'),'-o',str(binary)],90),('test',[str(binary),'--test-threads=1'],30)]:
  try:r=subprocess.run(cmd,capture_output=True,timeout=timeout);code=r.returncode;a=r.stdout;b=r.stderr
  except subprocess.TimeoutExpired as e:code=124;a=e.stdout or b'';b=e.stderr or b''
  (out/(name+'.stdout')).write_bytes(a);(out/(name+'.stderr')).write_bytes(b)
  receipt['commands'].append({'argv':cmd,'timeout':timeout,'exit':code,'stdout_sha256':hashlib.sha256(a).hexdigest(),'stderr_sha256':hashlib.sha256(b).hexdigest()})
  if code:break
 if binary.exists():receipt['binary_sha256']=sha(binary)
receipt['passed']=len(receipt['commands'])==2 and all(c['exit']==0 for c in receipt['commands'])
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps(receipt));sys.exit(0 if receipt['passed']else 1)
