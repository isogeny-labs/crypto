from pathlib import Path
import json,subprocess,re,hashlib
r=Path('/home/ec2-user/crypto-pencil-evidence-recovery-20261105');prefix='research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets';p=r/prefix;out=p/'affine-mul-replay-plan-20261109'
inv=p/'dispatch-acceptance/AcceptanceReadback-attempt-6fn9Fors/before.sha256';hist=dict((s,h) for h,s in (l.split(None,1) for l in inv.read_text().splitlines()))
check=(p/'dispatch-acceptance/check.sh').read_text();line=next(x for x in check.splitlines() if x.startswith('export LEAN_PATH='));oldD='/var/tmp/pencil-dispatch-acceptance-20261101-v1';front=line.split('"',1)[1].split(':$(cat',1)[0].replace('$D',oldD);roots=front.split(':')+(p/'dispatch-acceptance/dependencies.txt').read_text().strip().split(':');roots=list(dict.fromkeys(roots))
module_names={Path(s).stem for s in hist if s.endswith('.lean')}
args=['git','ls-files','-z','--']+[prefix+'/**/'+name+'.lean.txt' for name in sorted(module_names)]
paths=subprocess.check_output(args,cwd=r).decode().split('\0');byname={}
for path in filter(None,paths):byname.setdefault(Path(path).name.removesuffix('.lean.txt'),[]).append(path)
base=json.loads(Path('/var/tmp/pencil-norm-recovery-20261109/baseline.json').read_text());fr=json.loads((p/'solve4-pivot-swap-replay-20261109/freeze.json').read_text());cache_roots=fr['lean_roots'];extra=json.loads((p/'solve4-pivot-swap-replay-20261109/additional-runtime-pins.json').read_text());available={s:v['sha256'] for s,v in base.items()};available.update(extra)
resolved={};missing=[];order=[];visiting=set();external={}
def imports(text):
 return [m for line in text.splitlines() if line.strip().startswith('import ') for m in line.strip()[7:].split('--',1)[0].split()]
def visit(name):
 if name in resolved or name in external:return
 if name in visiting:raise RuntimeError('cycle '+name)
 visiting.add(name)
 origin=next((str(Path(root)/(name.replace('.','/')+'.lean')) for root in roots if str(Path(root)/(name.replace('.','/')+'.lean')) in hist),None)
 if origin:
  sha=hist[origin];candidates=[s for s in byname.get(name,[]) if hashlib.sha256((r/s).read_bytes()).hexdigest()==sha]
  if not candidates:
   missing.append({'module':name,'origin':origin,'sha256':sha,'reason':'no exact tracked archive match'});visiting.remove(name);return
  path=min(candidates,key=lambda s:(len(s),s));text=(r/path).read_text();deps=imports(text)
  resolved[name]={'module':name,'origin':origin,'path':path,'sha256':sha,'imports':deps}
  for dep in deps:visit(dep)
  order.append(name)
 else:
  candidates=[str(Path(root)/(name.replace('.','/')+'.olean')) for root in cache_roots];found=next((s for s in candidates if s in available),None)
  if found:external[name]={'path':found,'sha256':available[found]}
  else:missing.append({'module':name,'reason':'no source in historical precedence or compiled entry in current baseline'})
 visiting.remove(name)
for name in ['DispatchAcceptance','AcceptanceControls','AffineControls','Init.Data.UInt.Bitwise']:visit(name)
x={'schema':'research.source-closure/v1','lookup_method':'Exact historical before.sha256 and LEAN_PATH precedence; git index filename lookup limited to286 recorded source module basenames; archived candidatebytes required exact recorded SHA. No recursive filesystem scan or compile.','roots':roots,'modules':resolved,'topological_order':order,'external_cached_modules':external,'missing':missing,'local_module_count':len(resolved),'full_replay_calls_if_complete':len(resolved)+4}
(out/'source-closure.json').write_text(json.dumps(x,indent=2)+'\n');print('local',len(resolved),'external',len(external),'missing',len(missing));print(json.dumps(missing,indent=2))
