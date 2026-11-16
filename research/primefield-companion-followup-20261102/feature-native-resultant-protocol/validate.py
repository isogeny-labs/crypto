"""Source-bound canonical adapter tests; never execute the native panel."""
from pathlib import Path
import json,subprocess,sys,hashlib
D=Path(__file__).resolve().parent;R=D.parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
 out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
 p=json.loads((D/'protocol.draft.json').read_text())
 files=set(D.glob('*.py'))|{D/'protocol.draft.json',D/'authority.json'}|{D/'authority'/x['path']for x in json.loads((D/'authority.json').read_text())['files']}|{(D/x['path']).resolve()for x in p['extensions']['bindings'].values()}
 receipt={'scope':'canonical software adapter validation only; no native panel or cloud job','python':sys.version,'files':{str(f.resolve().relative_to(R.resolve())):sha(f)for f in sorted(files)},'commands':[]}
 for name,argv in [('canonical-draft',[sys.executable,str(D/'authority/tools/experiment_protocol.py'),'validate',str(D/'protocol.draft.json')]),('synthetic-tests',[sys.executable,str(D/'test_adapter.py')])]:
  try:r=subprocess.run(argv,capture_output=True,timeout=90);code=r.returncode;stdout=r.stdout;stderr=r.stderr
  except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
  (out/(name+'.stdout')).write_bytes(stdout);(out/(name+'.stderr')).write_bytes(stderr)
  receipt['commands'].append(dict(name=name,argv=argv,timeout_seconds=90,exit_code=code,stdout_sha256=hashlib.sha256(stdout).hexdigest(),stderr_sha256=hashlib.sha256(stderr).hexdigest()))
  if code:break
 receipt['passed']=len(receipt['commands'])==2 and all(c['exit_code']==0 for c in receipt['commands']);(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({'passed':receipt['passed'],'receipt':str(out/'receipt.json')}));sys.exit(0 if receipt['passed']else 1)
