"""Software checks only. Supply a fresh output directory; never overwrite evidence."""
from pathlib import Path
import hashlib,json,subprocess,sys,tempfile
D=Path(__file__).resolve().parent
sha=lambda b:hashlib.sha256(b).hexdigest()
def audit():
 records=json.loads((D/'generation.json').read_text())
 for r in records:
  original=(D.parent/r['source']).read_bytes();generated=(D/'generated'/f"{r['arm']}.rs").read_bytes()
  assert sha(original)==r['source_sha256'] and sha(generated)==r['generated_sha256']
  reverse=generated.decode()
  for e in reversed(r['edits']):
   assert reverse.count(e['new'])==e['occurrences']
   reverse=reverse.replace(e['new'],e['old'])
  assert reverse.encode()==original
 return len(records)
if __name__=='__main__':
 out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
 count=audit();compiler=Path('/home/ec2-user/.cargo/bin/rustc');receipt={'scope':'software instrument self-tests; no panel rows','reversible_arms':count,'compiler':subprocess.check_output([str(compiler),'-Vv'],text=True),'source_hashes':{},'commands':[]}
 for p in [D/'main.rs',D/'meter.rs',D/'names.rs',D/'generate.py',D/'generation.json',D/'validate.py',*sorted((D/'generated').glob('*.rs'))]:receipt['source_hashes'][str(p.relative_to(D))]=sha(p.read_bytes())
 with tempfile.TemporaryDirectory(prefix='resultant-instrument-')as temp:
  binary=Path(temp)/'instrument'
  for label,cmd,limit in [('build',[str(compiler),'--edition=2021','-C','opt-level=2','-C','overflow-checks=yes',str(D/'main.rs'),'-o',str(binary)],90),('self-test',[str(binary),'--self-test'],30)]:
   try:
    r=subprocess.run(cmd,capture_output=True,timeout=limit);code=r.returncode;stdout=r.stdout;stderr=r.stderr
   except subprocess.TimeoutExpired as e:code=124;stdout=e.stdout or b'';stderr=e.stderr or b''
   (out/f'{label}.stdout').write_bytes(stdout);(out/f'{label}.stderr').write_bytes(stderr)
   receipt['commands'].append(dict(label=label,argv=cmd,timeout_seconds=limit,exit_code=code,stdout_sha256=sha(stdout),stderr_sha256=sha(stderr)))
   if code:break
  if binary.exists():receipt['binary_sha256']=sha(binary.read_bytes())
 receipt['passed']=len(receipt['commands'])==2 and all(x['exit_code']==0 for x in receipt['commands'])
 (out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
 print(json.dumps({'passed':receipt['passed'],'reversible_arms':count,'receipt':str(out/'receipt.json')}))
 sys.exit(0 if receipt['passed'] else 1)
