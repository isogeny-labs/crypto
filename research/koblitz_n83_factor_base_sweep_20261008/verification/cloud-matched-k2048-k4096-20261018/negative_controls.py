from pathlib import Path
import json,shutil,tempfile,hashlib,sys,subprocess
from audit_stage import audit,ROOT,REPO,read,sha
stage,k=sys.argv[1],int(sys.argv[2]);original=ROOT/(stage+'-k'+str(k));records=ROOT/'negative-controls';records.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='forged-'+stage+'-',dir=ROOT) as tmp:
 out=Path(tmp)/'run';shutil.copytree(original,out)
 outer=read(out/'outer.json');config=read(out/'config.json')
 if stage=='capacity':
  worker=read(out/'worker.json');worker['sat_clauses']+=1
  path=out/'worker.json';path.write_text(json.dumps(worker,indent=2)+'\n');outer['worker_sha256']=sha(path)
  label='forged-capacity-count-k'+str(k);expected='independent all-affine counts differ'
 else:
  path=out/'run/summary.json';wc=read(out/'run/config.json')
  worker=read(path) if path.exists() else read(REPO/'research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-natural-highcap-20261018/evidence/run-01/run/summary.json')
  for key,value in wc.items():
   if key not in ('schema','allow_direct_relation') and key in worker:worker[key]=value
  worker['orbit_columns']=k;worker['object']=config['selected_object'];worker['point_set_blake3']=config['selected_point_set_blake3']
  worker['report']['orbit_count']=k;worker['report']['factor_base_size']=166*k;worker['report']['matrix_columns']=k+1
  for item in worker['rank_history']:item['columns']=k+1
  worker['panel_manifest_blake3']=subprocess.check_output([str(ROOT/'b3file'),str(ROOT/'inputs'/('k'+str(k))/'manifest.json')],text=True).strip()
  worker['status']='PASS_verified_target_only';worker['verified_log']='1'
  path.write_text(json.dumps(worker,indent=2)+'\n')
  outer.update({'status':worker['status'],'report':worker['report'],'worker_exit_code':0,'parse_error':None,'worker_error':None,'group_replay_error':None,'worker_sha256':sha(path),'worker_config_sha256':sha(out/'run/config.json')})
  label=('forged-short-success-k' if stage=='control' else 'forged-success-k')+str(k);expected='verified target lacks rank, linear solve, or scalar receipt'
 (out/'outer.json').write_text(json.dumps(outer,indent=2)+'\n')
 try:audit(stage,k,out)
 except ValueError as error:
  reason=str(error)
  if expected not in reason:raise
 else:raise RuntimeError('auditor accepted forged evidence')
 saved=records/label;saved.mkdir();shutil.copyfile(path,saved/'worker.json');shutil.copyfile(out/'outer.json',saved/'outer.json')
 result={'schema':'n83.cloud-matched-negative-control/v1','status':'PASS_FORGERY_REJECTED','stage':stage,'columns':k,'mutation':'capacity clause count +1' if stage=='capacity' else 'forged verified-target success in worker and outer; scalar 1; complete worker config and matching worker hash retained; absent terminal worker copied only inside negative control from historical template','reason':reason,'forged_worker_sha256':sha(saved/'worker.json'),'forged_outer_sha256':sha(saved/'outer.json'),'original_outer_status':read(original/'outer.json')['status']}
 (saved/'result.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
