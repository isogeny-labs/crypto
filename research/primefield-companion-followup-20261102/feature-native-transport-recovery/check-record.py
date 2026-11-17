from pathlib import Path
import subprocess,hashlib,json,datetime
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-transport-recovery';h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
b=Path('/home/ec2-user/.cache/primefield-native-transport-recovery-20261105/checker');steps=[]
argv=['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-O','-C','overflow-checks=yes',str(d/'check.rs'),'-o',str(b)]
with(d/'checker-build.log').open('w')as out:p=subprocess.run(argv,stdout=out,stderr=subprocess.STDOUT,timeout=60)
steps.append(dict(step='build',argv=argv,timeout_seconds=60,exit_code=p.returncode));assert p.returncode==0
for name,column in [('scalar',3),('initialization',10)]:
 rows=(d/'recovery.tsv').read_text().splitlines();f=rows[1].split('\t');f[column]=str(int(f[column])+1);rows[1]='\t'.join(f);(d/f'negative-{name}.tsv').write_text('\n'.join(rows)+'\n')
program=r/'feature-native-quartic-recovery/program.tsv';rules=r/'feature-native-quartic-recovery/corrections.tsv'
for name,file in [('positive','recovery.tsv'),('scalar-negative','negative-scalar.tsv'),('initialization-negative','negative-initialization.tsv')]:
 argv=[str(b),str(program),str(rules),str(d/file)]
 with(d/('checker-'+name+'.log')).open('w')as out:p=subprocess.run(argv,stdout=out,stderr=subprocess.STDOUT,timeout=60)
 steps.append(dict(step=name,argv=argv,timeout_seconds=60,exit_code=p.returncode,packet_sha256=h(d/file)));assert(p.returncode==0)==(name=='positive')
(d/'checker-receipt.json').write_text(json.dumps(dict(recorded_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),steps=steps,inputs={str(p.relative_to(r)):h(p)for p in[program,rules,d/'check.rs']},binary_sha256=h(b),producer_imports=0,checker_author='producing session; no independent-review standing'),indent=2)+'\n')
print((d/'checker-positive.log').read_text());print('Both corruptions rejected')
