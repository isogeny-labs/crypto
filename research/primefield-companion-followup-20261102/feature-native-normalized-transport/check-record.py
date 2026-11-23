from pathlib import Path
import subprocess,json,hashlib,datetime
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-normalized-transport';h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
b=Path('/home/ec2-user/.cache/primefield-native-normalized-transport-20261105/checker');source_before=h(d/'check.rs')
steps=[]
argv=['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-O','-C','overflow-checks=yes',str(d/'check.rs'),'-o',str(b)]
with (d/'checker-build.log').open('w')as out:p=subprocess.run(argv,stdout=out,stderr=subprocess.STDOUT,timeout=60)
steps.append(dict(step='build',argv=argv,timeout_seconds=60,exit_code=p.returncode));assert p.returncode==0
lines=(d/'columns.tsv').read_text().splitlines();idx=next(i for i,l in enumerate(lines[1:],1) if l.split('\t')[2]=='1');f=lines[idx].split('\t');f[6]=str((int(f[6])+1)%int(f[0]));lines[idx]='\t'.join(f);(d/'negative-columns.tsv').write_text('\n'.join(lines)+'\n')
for name,packet in [('positive','columns.tsv'),('negative','negative-columns.tsv')]:
 argv=[str(b),str(d/packet)]
 with(d/('checker-'+name+'.log')).open('w')as out:p=subprocess.run(argv,stdout=out,stderr=subprocess.STDOUT,timeout=60)
 steps.append(dict(step=name,argv=argv,timeout_seconds=60,exit_code=p.returncode,packet_sha256=h(d/packet)))
 assert (p.returncode==0)==(name=='positive')
assert source_before==h(d/'check.rs')
(d/'checker-receipt.json').write_text(json.dumps(dict(recorded_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),steps=steps,binary_sha256=h(b),source_sha256=source_before,transport_imports=0,frozen_jet_import=True,negative_control='changed one finite numerator; expected normalized-value rejection'),indent=2)+'\n')
print((d/'checker-positive.log').read_text());print('negative checker exit:',p.returncode)
