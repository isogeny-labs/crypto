from pathlib import Path
import re,json,hashlib,subprocess,datetime,os
r=Path('research/primefield-companion-followup-20261102');d=r/'feature-native-batch-reciprocals';h=lambda p:hashlib.sha256(p.read_bytes()).hexdigest();seen={}
def visit(p):
 p=p.resolve();rel=str(p.relative_to(r.resolve()))
 if rel in seen:return
 seen[rel]=h(p)
 if p.suffix=='.rs':
  s=p.read_text()
  for m in re.finditer(r'#\[path\s*=\s*"([^"]+)"\]',s):visit(p.parent/m.group(1))
  for m in re.finditer(r'include_str!\("([^"]+)"\)',s):visit(p.parent/m.group(1))
  for m in re.finditer(r'\bmod\s+(\w+)\s*;',s):
   q=p.parent/(m.group(1)+'.rs')
   if q.exists():visit(q)
visit(d/'accounting.rs');(d/'accounting-source-lock.json').write_text(json.dumps(seen,indent=2)+'\n')
binary=Path('/home/ec2-user/.cache/primefield-native-batch-accounting-20261105/tests');binary.parent.mkdir(parents=True,exist_ok=True)
steps=[]
for name,argv,cap in [('build',['/home/ec2-user/.cargo/bin/rustc','--edition=2021','-O','-C','overflow-checks=yes','--test',str(d/'accounting.rs'),'-o',str(binary)],90),('test',[str(binary),'accounting_tests::','--test-threads=1','--nocapture'],60)]:
 env=dict(os.environ);[env.pop(k,None) for k in ('ADAPTIVE_PACKET_OUT','EXTENSION_TRACE_OUT','COLLAPSE_OUT')];env.pop('QUARTIC_RECOVERY_OUT',None);env.pop('BATCH_RECIPROCALS_OUT',None)
 with(d/('accounting-'+name+'.log')).open('w')as out:p=subprocess.run(argv,env=env,stdout=out,stderr=subprocess.STDOUT,timeout=cap)
 steps.append(dict(step=name,argv=argv,timeout_seconds=cap,exit_code=p.returncode))
 if p.returncode:break
for rel,v in seen.items():assert h(r/rel)==v,rel
(d/'accounting-receipt.json').write_text(json.dumps(dict(recorded_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),steps=steps,source_lock_entries=len(seen),binary_sha256=h(binary)if binary.exists()else None,overflow_checks=True,comparative_panel=False),indent=2)+'\n')
print((d/('accounting-'+name+'.log')).read_text()[-3000:]);assert p.returncode==0
