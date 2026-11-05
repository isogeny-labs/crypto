"""Finite local custody regression. No Generate, cloud, rebuild, retry or source mutation."""
from pathlib import Path
import hashlib,json,subprocess,time,shutil,sys
HERE=Path(__file__).resolve().parent
OUT=Path('/home/ec2-user/ml-dataset-resume-20261029/custody-four-cell-acceptance-run-20261103-v3')
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def load(p):return json.loads(p.read_text())
def require(ok,msg):
 if not ok:raise ValueError(msg)
def save(p,v):
 with p.open('x') as f:json.dump(v,f,indent=2);f.write('\n')
def snapshot(root):
 result={}
 for p in sorted(root.rglob('*')):
  require(not p.is_symlink(),'symlink in retained tree')
  if p.is_file():result[str(p.relative_to(root))]={'sha256':sha(p),'bytes':p.stat().st_size}
  else:require(p.is_dir(),'nonregular retained member')
 return result
def custody_argv(binary,args):
 require(bool(args) and args[0] in ['create','verify','restore'],'unexpected custody command')
 return [str(binary),'dataset','custody',*map(str,args)]
def main():
 ready=load(Path(sys.argv[1]));fixture=load(HERE/'fixture.json');source=Path(fixture['source_root']);binary=Path(ready['binary'])
 require(ready['schema']=='custody-four-cell-ready/v1' and ready['status']=='PASS','readiness gate absent')
 require(sha(Path(__file__))==ready['driver_sha256'] and sha(HERE/'fixture.json')==ready['fixture_sha256'],'input identity changed')
 require(sha(binary)==ready['binary_sha256'],'binary changed')
 source_manifest_path=Path(ready['source_manifest']);require(sha(source_manifest_path)==ready['source_manifest_sha256'],'source manifest changed')
 compiled=load(Path(ready['compile_receipt']));require(sha(Path(ready['compile_receipt']))==ready['compile_receipt_sha256'],'compile receipt changed')
 require(compiled['exit_code']==0 and compiled['source_unchanged'] is True and compiled['binary_sha256']==ready['binary_sha256'] and compiled['source_manifest_sha256']==ready['source_manifest_sha256'],'compile gate not exact source/binary PASS')
 source_hashes=load(source_manifest_path);repo=Path(ready['source_root'])
 def check_source():return all(sha(repo/n)==h for n,h in source_hashes.items()) and sha(binary)==ready['binary_sha256']
 require(check_source(),'compiled source changed');require(snapshot(source)==fixture['files'],'original fixture changed')
 require(sha(source.parent/'analysis.json')==fixture['analysis_sha256'] and load(source.parent/'analysis.json')['verdict']=='PASS','original analysis changed')
 require(fixture['total_bytes']<=16*1024*1024 and len(fixture['files'])<=1000,'fixture exceeds fixed local control cap')
 OUT.mkdir(exist_ok=False);started=time.monotonic();result={'status':'FAIL','commands':[],'planned_commands':10,'cloud_calls':0,'generate_calls':0}
 try:
  save(OUT/'started.json',{'ready':ready,'fixture_sha256':sha(HERE/'fixture.json'),'driver_sha256':sha(Path(__file__))})
  copied=OUT/'copied-run';shutil.copytree(source,copied);require(snapshot(copied)==fixture['files'],'copy differs')
  def command(name,args,expected=None):
   require(check_source(),'binary/source changed before command');remaining=1800-(time.monotonic()-started);require(remaining>0,'total control deadline')
   d=OUT/name;d.mkdir();row={'argv':custody_argv(binary,args),'exit_code':None,'expected_error':expected};result['commands'].append(row)
   try:
    with (d/'stdout').open('xb') as out,(d/'stderr').open('xb') as err:
     run=subprocess.run(row['argv'],stdout=out,stderr=err,timeout=min(240,remaining),cwd=repo)
    row['exit_code']=run.returncode
   except BaseException as exc:row['exception']=repr(exc);raise
   finally:
    for n in ['stdout','stderr']:
     if (d/n).exists():row[n+'_sha256']=sha(d/n)
    save(d/'receipt.json',row)
   require(check_source(),'binary/source changed after command')
   if expected is not None:
    require(row['exit_code']!=0,'mutation accepted');text=(d/'stderr').read_text()+(d/'stdout').read_text();require(expected in text,'wrong rejection cause');return None
   require(row['exit_code']==0,'positive command failed');return json.loads((d/'stdout').read_text())
  bundle=OUT/'bundle'
  created=command('00-create',['create','--out',copied,'--bundle',bundle,'--max-bytes','16777216','--max-files','1000'])
  require(created['known_answer_replay_performed'] is True and created['cloud_requests']==0 and created['remote_published'] is False,'create scope')
  verified=command('01-verify',['verify','--bundle',bundle]);require(verified['all_member_hashes_verified'] is True and verified['native_run_replay_performed'] is False,'verify scope')
  restored=OUT/'valid-restore';v=command('02-restore',['restore','--bundle',bundle,'--destination',restored]);native=v['native_run_replay'];require(v['cloud_requests']==0 and native['known_answer_replay_performed'] is True and native['completed_receipts']==4 and native['known_answer_verified_cells']==4 and native['planned_cells']==4,'native four-cell restore not complete')
  require(snapshot(restored/'run')==fixture['files'],'restored bytes differ')
  baseline=snapshot(bundle);manifest=load(bundle/'manifest.json');counter=3
  fields=['source_commit','binary_sha256','plan_file_sha256','protocol_file_sha256','run_file_sha256']
  for field in fields:
   bad=OUT/('mutation-'+field);shutil.copytree(bundle,bad);changed=dict(manifest);old=changed[field];changed[field]=('0' if old[0]!='0' else '1')+old[1:]
   (bad/'manifest.json').write_text(json.dumps(changed,sort_keys=True,indent=2)+'\n')
   require({k:v for k,v in snapshot(bad).items() if k!='manifest.json'}=={k:v for k,v in baseline.items() if k!='manifest.json'},'mutation changed chunks')
   save(OUT/('mutation-'+field+'.json'),{'field':field,'old':old,'new':changed[field],'manifest_sha256':sha(bad/'manifest.json'),'members_unchanged':changed['members']==manifest['members']})
   if field in fields[:2]:
    got=command(f'{counter:02d}-verify-'+field,['verify','--bundle',bad]);counter+=1;require(got['all_member_hashes_verified'] is True,'producer mutation must pass chunk-only verification')
    expected='custody producer differs from bound run'
   else:expected='custody missing or changed binding run/'+{'plan_file_sha256':'plan.json','protocol_file_sha256':'protocol.json','run_file_sha256':'run.json'}[field]
   command(f'{counter:02d}-reject-'+field,['restore','--bundle',bad,'--destination',OUT/('rejected-'+field)],expected);counter+=1
  require(len(result['commands'])==10,'incomplete exact command inventory');require(snapshot(bundle)==baseline,'baseline bundle changed');result['status']='PASS'
 except BaseException as exc:result['exception']=repr(exc)
 finally:
  for key,check in [('original_fixture_unchanged',lambda:snapshot(source)==fixture['files']),('source_binary_unchanged',check_source)]:
   try:result[key]=check()
   except BaseException as exc:result[key]=False;result[key+'_exception']=repr(exc)
  if not result['original_fixture_unchanged'] or not result['source_binary_unchanged']:result['status']='FAIL'
  result['elapsed_seconds']=time.monotonic()-started;save(OUT/'result.json',result)
 return 0 if result['status']=='PASS' else 1
if __name__=='__main__':raise SystemExit(main())
