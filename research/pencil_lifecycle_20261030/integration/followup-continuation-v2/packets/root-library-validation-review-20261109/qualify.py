#!/usr/bin/python3
"""Finite synthetic admission/permit/durability checks; no native worker or compiler."""
import argparse,ast,copy,datetime,hashlib,importlib.util,json,os,signal,time,types,urllib.request,urllib.error
from pathlib import Path
from unittest.mock import patch
P=Path(__file__).resolve().parent
SOURCE=P.parent/'root-library-validation-20261109'
PINS={'durable_io.py':'2034c58e36bb3e3d6727dbd6c640ff437c695f472810e6ae596e6ac339dc854c','durable-launcher.py':'57def5896c88cc512da5edd99bdea423ca87d44935bc064e0995c516db5a9f9c','durable-worker.py':'d8e32926c2d254238e39928105d6d2c6c2c5d318fbdb7cda07382493ada7a77e'}
def h(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def write(p,x):p.write_text(json.dumps(x,allow_nan=True)+'\n')
def main():
 ap=argparse.ArgumentParser();ap.add_argument('--output',type=Path,required=True);a=ap.parse_args()
 assert a.output.is_absolute() and not a.output.exists();a.output.mkdir(mode=0o700)
 class Stop(BaseException):pass
 def stop(*_):raise Stop('50-second work deadline')
 signal.signal(signal.SIGALRM,stop);signal.alarm(50);start=time.monotonic();rows=[]
 for n,d in PINS.items():assert h(SOURCE/n)==d
 spec=importlib.util.spec_from_file_location('qualified_io',SOURCE/'durable_io.py');io=importlib.util.module_from_spec(spec);spec.loader.exec_module(io)
 def case(name,fn):
  try:fn();rows.append({'name':name,'passed':True})
  except Exception as e:rows.append({'name':name,'passed':False,'error':repr(e)})
 def reject(fn):
  try:fn()
  except (AssertionError,RuntimeError,OSError,ValueError,KeyError):return
  raise AssertionError('expected rejection absent')
 def fixture(name,mutate=None,ordinary=False):
  d=a.output/name;d.mkdir();f=json.loads((SOURCE/'admission-draft.json').read_text());f['admitted']=True
  f['workspace']=str(d/'workspace');f['durable_run']='/home/ec2-user/pencil-validation-receipts/synthetic-never-created'
  for n in f['recipe_files']:(d/n).write_bytes((SOURCE/n).read_bytes())
  write(d/'materialized-inputs.json',{'synthetic':True});f['recipe_files']['materialized-inputs.json']=h(d/'materialized-inputs.json');f['inputs_sha256']=h(d/'materialized-inputs.json')
  write(d/'publication.json',{'synthetic':True});f['publication']={'verified_github_commit':'synthetic-no-publication-claim','receipt_file':'publication.json','receipt_sha256':h(d/'publication.json')}
  if ordinary:
   write(d/'allocation.json',{'approved':True,'one_attempt_limit':1,'total_ceiling_usd':10,'existing_debit_usd':1,'maximum_incremental_usd':5});f['cost']={'finite_dollar_ceiling_usd':4,'worst_case_incremental_usd':3,'receipt_file':'allocation.json','receipt_sha256':h(d/'allocation.json')}
  if mutate:mutate(f,d)
  write(d/'admission.json',f)
  return lambda:io.load_admission(d)
 case('waiver_valid',lambda:fixture('waiver-valid')())
 case('ordinary_finite_valid',lambda:fixture('ordinary-valid',ordinary=True)())
 def waiver_change(key,value):
  def mutate(f,d):
   w=json.loads((d/'owner-dollar-waiver.json').read_text());w[key]=value;write(d/'owner-dollar-waiver.json',w);f['cost']['receipt_sha256']=h(d/'owner-dollar-waiver.json');f['recipe_files']['owner-dollar-waiver.json']=h(d/'owner-dollar-waiver.json')
  return mutate
 for key,value in [('approved',False),('scope','other'),('deadline','2099-01-01T00:00:00+00:00'),('active_worker_seconds_ceiling',43201),('validation_attempt_seconds',1801),('one_attempt_limit',2),('preserve_resource_and_ownership_checks',False)]:
  case('waiver_reject_'+key,lambda k=key,v=value:reject(fixture('bad-'+k,waiver_change(k,v))))
 case('waiver_reject_nonnull_dollars',lambda:reject(fixture('nonnull',lambda f,d:f['cost'].update(finite_dollar_ceiling_usd=1))))
 for label,value in [('infinite',float('inf')),('boolean',True)]:
  case('ordinary_reject_'+label,lambda l=label,v=value:reject(fixture('ordinary-'+l,lambda f,d:f['cost'].update(finite_dollar_ceiling_usd=v),True)))
 case('reject_altered_recipe_file',lambda:reject(fixture('bad-hash',lambda f,d:(d/'fixture.json').write_text('{}'))))
 case('reject_omitted_control',lambda:reject(fixture('bad-control',lambda f,d:f['exact_tests'].pop())))
 case('reject_changed_resource',lambda:reject(fixture('bad-resource',lambda f,d:f['limits'].update(aggregate_cpu=3))))
 case('reject_missing_publication_identity',lambda:reject(fixture('bad-pub',lambda f,d:f['publication'].update(verified_github_commit=None))))
 case('reject_invalid_fence',lambda:reject(fixture('bad-fence',lambda f,d:f['lease'].update(fencing_epoch=0))))
 def extract(file,names,env):
  tree=ast.parse((SOURCE/file).read_text());nodes=[x for x in tree.body if isinstance(x,ast.FunctionDef) and x.name in names];assert {x.name for x in nodes}==set(names);exec(compile(ast.Module(body=nodes,type_ignores=[]),str(SOURCE/file),'exec'),env);return env
 permitdir=a.output/'permit';permitdir.mkdir();lease={'task_id':'fixture','attempt_id':'fixture','lease_id':'fixture','fencing_epoch':1}
 env=extract('durable-worker.py',['check_permit'],{'json':json,'Path':Path,'time':time,'F':{'durable_run':str(permitdir),'lease':lease},'R':{'invocation_id':'abc'}})
 def permit_case(override,valid=False):
  data={'invocation_id':'abc','lease':lease,'expires_unix':time.time()+100};data.update(override);write(permitdir/'supervisor-permit.json',data)
  if valid:env['check_permit']()
  else:reject(env['check_permit'])
 case('worker_valid_permit',lambda:permit_case({},True));case('worker_expired_permit',lambda:permit_case({'expires_unix':0}));case('worker_wrong_invocation',lambda:permit_case({'invocation_id':'wrong'}));case('worker_wrong_lease',lambda:permit_case({'lease':{}}))
 hb=extract('durable-launcher.py',['heartbeat'],{'datetime':datetime,'time':time,'urllib':urllib,'json':json,'F':{'lease':lease},'CREDENTIALS':{'endpoint':'http://synthetic.invalid','token':'synthetic'},'HB_NEXT':0,'HB_EXPIRY':None,'R':{},'checkpoint':lambda _:None})
 def denied():
  hb['HB_EXPIRY']=None
  with patch.object(urllib.request,'urlopen',side_effect=urllib.error.HTTPError('synthetic',409,'fixture',{},None)):reject(lambda:hb['heartbeat'](True))
 case('launcher_definitive_lease_loss',denied)
 def unavailable():
  hb['HB_EXPIRY']=None
  with patch.object(urllib.request,'urlopen',side_effect=TimeoutError('fixture')):reject(lambda:hb['heartbeat'](True))
 case('launcher_initial_heartbeat_timeout',unavailable)
 logs=a.output/'logs';logs.mkdir();(logs/'worker.stdout').write_bytes(b'prefix')
 def snapshot():
  r=io.flush_logs(logs);assert r['worker.stdout']['prefix_bytes']==6 and r['worker.stdout']['prefix_sha256']==hashlib.sha256(b'prefix').hexdigest()
 case('fixed_log_prefix',snapshot)
 def atomic_failure():
  target=logs/'old.json';target.write_bytes(b'old')
  with patch.object(io.os,'fsync',side_effect=OSError('fixture')):reject(lambda:io.atomic_json(target,{'new':1}))
  assert target.read_bytes()==b'old'
 case('atomic_fsync_failure_preserves_old',atomic_failure)
 def seal_failure():
  with patch.object(io.os,'fsync',side_effect=OSError('fixture')):reject(lambda:io.seal_outputs(logs))
  assert not (logs/'terminal-output-manifest.json').exists()
 case('seal_fsync_failure_no_manifest',seal_failure)
 signal.alarm(0);result={'schema':'root-validation.synthetic-qualification/v1','source_pins':PINS,'cases':rows,'passed':len(rows)==26 and all(r['passed'] for r in rows),'elapsed_seconds':time.monotonic()-start,'native_unit_started':False,'compiler_started':False,'network_used':False,'scope':'synthetic admission, extracted permit/heartbeat functions, helper filesystem failures; not native lifecycle fault qualification'};write(a.output/'result.json',result);print(json.dumps(result));return 0 if result['passed'] else 1
if __name__=='__main__':raise SystemExit(main())
