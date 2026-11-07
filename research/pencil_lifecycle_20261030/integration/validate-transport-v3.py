import subprocess,json,hashlib,datetime,os
os.environ["CARGO_TARGET_DIR"]="/tmp/pencil-transport-software-target"
from pathlib import Path
root=Path(__file__).resolve().parents[3]
out=Path(__file__).resolve().parent
manifest=out/'transport-source-hashes-v3.json'
paths={str(p):'' for component in ['controller','progress','fresh-owner'] for p in Path('taskq/deploy/eks-karpenter/resume',component).rglob('*') if p.is_file() and (p.suffix=='.rs' or p.name in ['Cargo.toml','Cargo.lock']) and 'target' not in p.parts}
for p in ['taskq/deploy/eks-karpenter/run-ecbench','taskq/deploy/eks-karpenter/Dockerfile.ecbench','taskq/deploy/eks-karpenter/build-bundle','taskq/deploy/eks-karpenter/publish-image','taskq/deploy/eks-karpenter/submit-job','taskq/deploy/eks-karpenter/resume/request.schema.json',str(Path(__file__).resolve().relative_to(root))]:paths[p]=''
expected={p:hashlib.sha256((root/p).read_bytes()).hexdigest() for p in paths}
with manifest.open('x') as f:f.write(json.dumps(expected,indent=2)+'\n')
commands=[]
for component in ['controller','progress','fresh-owner']:
 manifest_path=f'taskq/deploy/eks-karpenter/resume/{component}/Cargo.toml'
 commands += [(component+'-test-transport-v3',['cargo','test','--manifest-path',manifest_path,'--locked','--offline','-j','2','--','--test-threads=1','--quiet']), (component+'-clippy-transport-v3',['cargo','clippy','--manifest-path',manifest_path,'--locked','--offline','--all-targets','-j','2','--','-D','warnings'])]
for script in ['run-ecbench','build-bundle','publish-image']:
 commands += [(script+'-syntax-transport-v3',['bash','-n','taskq/deploy/eks-karpenter/'+script])]

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
print('TRANSPORT SOFTWARE CHECKS PASSED; NO CLOUD OR SCIENTIFIC EXECUTION',flush=True)
