"""Replay software checks and deterministic input construction, never --panel."""
from pathlib import Path
import subprocess,json,hashlib,tempfile,sys
D=Path(__file__).resolve().parent;R=D.parent
sha=lambda b:hashlib.sha256(b).hexdigest()
if __name__=='__main__':
 out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
 files=[D/'main.rs',D/'panel.rs',D/'analyze.py',D/'test_analyze.py',D/'validate.py',D/'inputs.tsv',D/'metrics.txt',R/'feature-native-resultant-accounting/meter.rs',R/'feature-native-resultant-accounting/names.rs',*[R/f'feature-native-resultant-accounting/generated/{a}.rs'for a in ['scalar','euclid','half']]]
 receipt={'scope':'software validation and deterministic input construction; no panel execution','source_hashes':{str(p.relative_to(R)):sha(p.read_bytes())for p in files},'compiler':subprocess.check_output(['/home/ec2-user/.cargo/bin/rustc','-Vv'],text=True),'commands':[]}
 with tempfile.TemporaryDirectory(prefix='resultant-panel-check-')as temp:
  binary=Path(temp)/'runner'
  commands=[('build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-C','opt-level=2','-C','overflow-checks=yes',str(D/'main.rs'),'-o',str(binary)],90),('native-test',[str(binary),'--self-test'],30),('inputs',[str(binary),'--inputs'],30),('metrics',[str(binary),'--metrics'],30),('analyzer-tests',[sys.executable,str(D/'test_analyze.py')],30)]
  for label,cmd,seconds in commands:
   try:r=subprocess.run(cmd,capture_output=True,timeout=seconds);code=r.returncode;stdout=r.stdout;stderr=r.stderr
   except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
   (out/(label+'.stdout')).write_bytes(stdout);(out/(label+'.stderr')).write_bytes(stderr)
   same=stdout==(D/('inputs.tsv'if label=='inputs'else'metrics.txt')).read_bytes()if label in ['inputs','metrics']else None
   receipt['commands'].append(dict(label=label,argv=cmd,timeout_seconds=seconds,exit_code=code,stdout_sha256=sha(stdout),stderr_sha256=sha(stderr),exact_replay=same))
   if code or same is False:break
  if binary.exists():receipt['binary_sha256']=sha(binary.read_bytes())
 receipt['passed']=len(receipt['commands'])==len(commands)and all(c['exit_code']==0 and c['exact_replay']is not False for c in receipt['commands'])
 (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({'passed':receipt['passed'],'receipt':str(out/'receipt.json')}));sys.exit(0 if receipt['passed']else 1)
