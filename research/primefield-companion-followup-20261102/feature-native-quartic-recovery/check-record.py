from pathlib import Path
import subprocess,json,hashlib,datetime
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-quartic-recovery';h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
b=Path('/home/ec2-user/.cache/primefield-native-quartic-recovery-20261105/checker');source_before=h(d/'check.rs');steps=[]
def run(name,argv,expected):
 with(d/(name+'.log')).open('w')as out:p=subprocess.run(argv,stdout=out,stderr=subprocess.STDOUT,timeout=60)
 steps.append(dict(step=name,argv=argv,timeout_seconds=60,exit_code=p.returncode))
 assert (p.returncode==0)==expected,(name,p.returncode)
run('checker-build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-O','-C','overflow-checks=yes',str(d/'check.rs'),'-o',str(b)],True)
lines=(d/'corrections.tsv').read_text().splitlines();f=lines[0].split('\t');f[1]=str((int(f[1])+1)%479);lines[0]='\t'.join(f);(d/'negative-corrections.tsv').write_text('\n'.join(lines)+'\n')
lines=(d/'recovery.tsv').read_text().splitlines();f=lines[1].split('\t');f[3]=str((int(f[3])+1)%479);lines[1]='\t'.join(f);(d/'negative-recovery.tsv').write_text('\n'.join(lines)+'\n')
for name,rules,rows,expected in [('positive','corrections.tsv','recovery.tsv',True),('negative-rule','negative-corrections.tsv','recovery.tsv',False),('negative-recovery','corrections.tsv','negative-recovery.tsv',False)]:
 run('checker-'+name,[str(b),str(d/'program.tsv'),str(d/rules),str(d/rows)],expected)
assert source_before==h(d/'check.rs')
(d/'checker-receipt.json').write_text(json.dumps(dict(recorded_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),steps=steps,binary_sha256=h(b),source_sha256=source_before,inputs={p.name:h(p)for p in d.glob('*.tsv')},producer_imports=0),indent=2)+'\n')
print((d/'checker-positive.log').read_text());print(steps)
