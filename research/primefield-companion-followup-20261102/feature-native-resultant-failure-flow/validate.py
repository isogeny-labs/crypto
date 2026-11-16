"""Source-bound journal/export software validation. Never invokes --panel."""
from pathlib import Path
import subprocess,json,hashlib,tempfile,sys
D=Path(__file__).resolve().parent;R=D.parent;authority=R/'feature-native-resultant-protocol/authority'
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
if __name__=='__main__':
 out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
 for rec in json.loads((D/'generation.json').read_text()):
  s=(D/rec['output']).read_text();require=sha(D/rec['output'])==rec['output_sha256']and sha(R/rec['source'])==rec['source_sha256'];assert require
  for edit in reversed(rec['edits']):assert s.count(edit['new'])==1;s=s.replace(edit['new'],edit['old'])
  assert s==(R/rec['source']).read_text()
 protocol=json.loads((D/'protocol.draft.json').read_text())
 files=set(D.glob('*.py'))|set(D.glob('*.rs'))|{D/'protocol.draft.json',D/'generation.json'}|{(D/x['path']).resolve()for x in protocol['extensions']['bindings'].values()}|{p for p in authority.rglob('*')if p.is_file()and '__pycache__'not in p.parts}|{R/'feature-native-resultant-panel/panel.rs',R/'feature-native-resultant-protocol/export.py',R/'feature-native-resultant-protocol/common.py'}
 receipt={'scope':'journal/canonical failure software tests; no native comparison panel','source_hashes':{str(p.resolve().relative_to(R.resolve())):sha(p)for p in sorted(files)},'commands':[],'compiler':subprocess.check_output(['/home/ec2-user/.cargo/bin/rustc','-Vv'],text=True)}
 with tempfile.TemporaryDirectory(prefix='resultant-journal-check-')as temp:
  binary=Path(temp)/'runner'
  cmds=[('build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-C','opt-level=2','-C','overflow-checks=yes',str(D/'main.rs'),'-o',str(binary)],90),('native-test',[str(binary),'--self-test'],30),('canonical-draft',[sys.executable,str(authority/'tools/experiment_protocol.py'),'validate',str(D/'protocol.draft.json')],30),('failure-tests',[sys.executable,str(D/'test_failure.py')],90)]
  for name,cmd,limit in cmds:
   try:r=subprocess.run(cmd,capture_output=True,timeout=limit);code=r.returncode;stdout=r.stdout;stderr=r.stderr
   except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
   (out/(name+'.stdout')).write_bytes(stdout);(out/(name+'.stderr')).write_bytes(stderr)
   receipt['commands'].append(dict(name=name,argv=cmd,timeout_seconds=limit,exit_code=code,stdout_sha256=hashlib.sha256(stdout).hexdigest(),stderr_sha256=hashlib.sha256(stderr).hexdigest()))
   if code:break
  if binary.exists():receipt['binary_sha256']=sha(binary)
 receipt['passed']=len(receipt['commands'])==4 and all(c['exit_code']==0 for c in receipt['commands']);(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print(json.dumps({'passed':receipt['passed'],'receipt':str(out/'receipt.json')}));sys.exit(0 if receipt['passed']else 1)
