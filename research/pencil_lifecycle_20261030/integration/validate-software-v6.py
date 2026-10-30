import subprocess,json,hashlib,datetime
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=Path(__file__).resolve().parent
manifest=out/'validation-source-hashes-v6.json'
paths=json.loads((out/'validation-source-hashes-v2.json').read_text())
paths[str(Path(__file__).resolve().relative_to(root))]=''
paths['src/bin/ecbench_owner_closure.rs']=''
for extra in ['src/cryptanalysis/koblitz_index_calculus.rs','src/ml_dataset/s3_budget.rs','src/ml_dataset/storage.rs']:
 paths[extra]=''
expected={p:hashlib.sha256((root/p).read_bytes()).hexdigest() for p in paths}
with manifest.open('x') as f:f.write(json.dumps(expected,indent=2)+'\n')
commands=[
 ('regression-check-v6',['cargo','check','--locked','--offline','--lib','-j','2']),
 ('regression-clippy-v6',['cargo','clippy','--locked','--offline','--lib','-j','2','--','-D','warnings']),
 ('chain-regressions-v6',['cargo','test','--locked','--offline','--release','--lib','-j','2','chained_s3','--','--test-threads=2','--quiet']),
 ('budget-regressions-v6',['cargo','test','--locked','--offline','--release','--lib','-j','2','native_budgeted','--','--test-threads=2','--quiet']),
 ('corruption-regression-v6',['cargo','test','--locked','--offline','--release','--lib','-j','2','resumed_publication_rejects_same_size_remote_corruption_without_overwrite','--','--quiet'])]

def check_source():
 for path,want in expected.items():
  if hashlib.sha256((root/path).read_bytes()).hexdigest()!=want:raise SystemExit('source changed: '+path)
for name,cmd in commands:
 check_source()
 print('START',name,flush=True)
 start=datetime.datetime.now(datetime.timezone.utc).isoformat()
 with (out/(name+'.stdout')).open('xb') as so,(out/(name+'.stderr')).open('xb') as se:
  result=subprocess.run(cmd,cwd=root,stdout=so,stderr=se)
 receipt={'command':cmd,'exit_code':result.returncode,'started_utc':start,'ended_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'source_manifest':manifest.name,'source_manifest_sha256':hashlib.sha256(manifest.read_bytes()).hexdigest(),'scientific_cohort_execution':False,'purpose':'software regression; known-answer algorithm fixtures may execute','logs':{ext:hashlib.sha256((out/(name+'.'+ext)).read_bytes()).hexdigest() for ext in ('stdout','stderr')}}
 with (out/(name+'.receipt.json')).open('x') as f:f.write(json.dumps(receipt,indent=2)+'\n')
 print('END',name,result.returncode,flush=True)
 check_source()
 if result.returncode:
  print((out/(name+'.stderr')).read_text()[-6000:],flush=True)
  print((out/(name+'.stdout')).read_text()[-6000:],flush=True)
  raise SystemExit(result.returncode)
print('V6 SCOPED REPAIR CHECKS PASSED; FULL LIBRARY RECHECK PENDING',flush=True)
