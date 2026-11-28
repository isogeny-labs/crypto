from pathlib import Path
import subprocess,json,hashlib,sys
p=Path(__file__).resolve().parent;out=Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
rustc='/home/ec2-user/.cargo/bin/rustc'
cmd=[rustc,'--edition','2021','--cfg','escape_probe','--emit=metadata','--error-format=json',str(p/'modes.rs'),'-o',str(out/'check.rmeta')]
r=subprocess.run(cmd,capture_output=True,timeout=60);(out/'compiler.stdout').write_bytes(r.stdout);(out/'compiler.stderr').write_bytes(r.stderr)
diagnostics=[json.loads(line) for line in r.stderr.decode().splitlines() if line.startswith('{')]
codes=[d['code']['code'] for d in diagnostics if d.get('level')=='error' and d.get('code')]
expected=r.returncode!=0 and codes==['E0515'] and any('owner' in d.get('rendered','') for d in diagnostics if d.get('code') and d['code']['code']=='E0515')
inputs=json.loads((p/'validation02/receipt.json').read_text())['input_hashes']
for f,h in inputs.items():assert hashlib.sha256(Path(f).read_bytes()).hexdigest()==h,f
receipt={'command':cmd,'compiler_exit':r.returncode,'diagnostic_codes':codes,'expected_escape_rejection':expected,'check_exit':0 if expected else 1,'compiler':subprocess.check_output([rustc,'--version']).decode().strip(),'input_hashes':inputs,'stderr_sha256':hashlib.sha256(r.stderr).hexdigest()}
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n');print({k:receipt[k] for k in ['compiler_exit','diagnostic_codes','expected_escape_rejection']});sys.exit(receipt['check_exit'])
