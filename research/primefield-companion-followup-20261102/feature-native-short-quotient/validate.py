#!/usr/bin/env python3
"""Reproduce deterministic native arithmetic checks in a new output directory."""
import sys,pathlib,subprocess,hashlib,json
root=pathlib.Path(__file__).resolve().parent
out=pathlib.Path(sys.argv[1]).resolve();out.mkdir(parents=True,exist_ok=False)
rustc='/home/ec2-user/.cargo/bin/rustc'
cmd=[rustc,'--edition','2021','--cfg','source_audit','--cfg','resultant_audit','--cfg','matrix_audit','--cfg','remainder_audit','--cfg','quotient_audit','-O','-C','overflow-checks=yes','--emit=link,dep-info',str(root/'check.rs'),'-o',str(out/'check')]
p=subprocess.run(cmd,capture_output=True);(out/'build.stdout').write_bytes(p.stdout);(out/'build.stderr').write_bytes(p.stderr)
receipt={'command':cmd,'compile_exit':p.returncode,'compiler':subprocess.check_output([rustc,'--version']).decode().strip()}
if p.returncode==0:
 q=subprocess.run([str(out/'check')],capture_output=True,timeout=60);receipt['check_exit']=q.returncode
 (out/'check.stdout').write_bytes(q.stdout);(out/'check.stderr').write_bytes(q.stderr)
 paths=(out/'check.d').read_text().splitlines()[0].split(': ',1)[1].split()
 receipt['input_hashes']={str(pathlib.Path(x).resolve()):hashlib.sha256(pathlib.Path(x).read_bytes()).hexdigest() for x in paths}
 receipt['stdout_sha256']=hashlib.sha256(q.stdout).hexdigest()
(out/'receipt.json').write_text(json.dumps(receipt,indent=2)+'\n')
sys.exit(receipt.get('check_exit',p.returncode or 1))
