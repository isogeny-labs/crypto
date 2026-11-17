from pathlib import Path
import sys,json,hashlib,subprocess,math,os
ROOT=Path(os.environ.get('N83_EVIDENCE_ROOT',str(Path(__file__).resolve().parent)))
REPO=Path(os.environ.get('N83_REPLAY_CHECKOUT','/home/ec2-user/crypto-n83-resume'))
sys.path.insert(0,str(REPO/'research/koblitz_n83_factor_base_sweep_20261008'))
import chain_capacity_supervisor as capacity
import chain_search_supervisor as search

def read(p):return json.loads(p.read_text())
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def require(ok,message):
 if not ok:raise ValueError(message)
def audit(stage,k,out):
 c=read(out/'config.json');o=read(out/'outer.json');i=read(out/'container-inspect.json')
 require(c['source_commit']==('19c2f73268be9c4fa16b18f55f234e077ffb5e53' if stage=='control' else 'f08f9f887ba9f87d0b8976ec1c7a6cd65eb63127'),'wrong frozen source commit')
 require(set(c['source_sha256'])==set(capacity.SOURCES)|{'attestation.json'},'missing source snapshot')
 require(c['binary_sha256']=='9aa6f722d79f460d9388f63954c6ddbed1c2672e08c35c0bd34e697f5d7ddfc0','wrong frozen binary')
 require(c['container_image_id']=='sha256:541096ae9b05d3b850341b78790e2ed31e94d5aa0ea086933c6ff0c09ffa2e63','wrong frozen image')
 require(c['columns']==k and c['summands']==5 and c['policy']=='public_x_hash' and c['seed']==2026100801 and c['domain_encoding']=='frobenius_orbit_phase','wrong selection')
 require(c['max_variables']==150000 and c['max_domain_clauses']==2000000 and c['wall_seconds']==(180 if stage in ('capacity','control') else 600),'wrong caps')
 require(c['memory_cgroup_limit_bytes']==4*1024**3 and c['memory_cgroup_swap_limit_bytes']==0 and c['cpu_limit']==1.0,'wrong resource config')
 require(c['binary_sha256']==sha(Path(os.environ.get('N83_REPLAY_WORKER',c['binary']))) and o['config_sha256']==sha(out/'config.json'),'binary/config custody')
 for name,digest in c['source_sha256'].items():
  p=out/'source'/name
  require(sha(p)==digest,'snapshot hash')
  if name!='attestation.json':
   b=subprocess.check_output(['git','show',c['source_commit']+':'+name],cwd=REPO)
   require(hashlib.sha256(b).hexdigest()==digest,'committed source snapshot')
 require(i['State']['Running'] is True and i['Image']==c['container_image_id'],'not a live frozen image')
 h=i['HostConfig'];require(h['Memory']==4*1024**3 and h['MemorySwap']==4*1024**3 and h['NanoCpus']==1000000000 and h['NetworkMode']=='none' and h['ReadonlyRootfs'] is True,'live resource limits')
 require('ALL' in h['CapDrop'] and any(x.startswith('no-new-privileges') for x in h['SecurityOpt']) and h['PidsLimit']==64,'live security/pid limits')
 mounts={x['Destination']:x for x in i['Mounts']}
 require(all(mounts[x]['RW'] is False for x in ['/panel','/worker','/source']),'writable frozen inputs')
 for key in ['source_changed','config_changed','binary_changed']:
  require(o[key] is False,'frozen data changed')
 require(o['cleanup_error'] is None and o['launch_error'] is None and o['selected_best_total_runtime'] is None and o['total_index_calculus_runtime_ms'] is None,'cleanup/claim invalid')
 require(o['stdout_sha256']==sha(out/'stdout.log') and o['stderr_sha256']==sha(out/'stderr.log'),'raw output hash')
 require(type(o['process_wall_ms']) in (int,float) and math.isfinite(o['process_wall_ms']) and 0<o['process_wall_ms']<=(c['wall_seconds']+30)*1000,'outer wall interval')
 remain=subprocess.check_output(['docker','ps','-aq','--filter','name=^'+c['container_name']+'$'],text=True).strip();require(not remain,'container remains')
 panel=ROOT/'inputs'/('k'+str(k));row=read(panel/'manifest.json')['bases'][0];archive=panel/row['object']
 fresh=read(ROOT/'inputs/input.json');ir=next(x for x in fresh['bases'] if x['columns']==k)
 expected_sha={2048:'aacb62784dee0c9f5e06328dbb85a4b6a1c7243e778328da82900e7284f3a74a',4096:'ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad'}[k]
 require(sha(archive)==ir['sha256']==expected_sha and archive.stat().st_size==ir['bytes'],'fresh input binding')
 require(subprocess.check_output([str(ROOT/'b3file'),str(archive)],text=True).strip()==ir['blake3']==row['compressed_blake3'],'archive BLAKE3')
 for name,digest in ir['panel_sha256'].items():require(sha(panel/name)==digest,'panel changed')
 result={'schema':'n83.cloud-matched-stage-audit/v1','auditor_source_sha256':sha(Path(__file__)),'stage':stage,'columns':k,'source_commit':c['source_commit'],'status':'PASS','worker_status':o['status'],'outer_wall_ms':o['process_wall_ms'],'selected_best_total_runtime':None}
 if stage=='capacity':
  w=read(out/'worker.json');require(o['status']==w['status']=='PASS_model_construction_only' and o['worker_exit_code']==0 and o['worker_error'] is None,'capacity not passed')
  require(o['worker_sha256']==sha(out/'worker.json') and capacity.receipt_error(w,c) is None,'capacity receipt invalid')
  require(w['point_set_blake3']==row['point_set_blake3'] and w['panel_manifest_blake3']==subprocess.check_output([str(ROOT/'b3file'),str(panel/'manifest.json')],text=True).strip(),'capacity manifest binding')
  expected={2048:(111478,892470,1140557),4096:(131958,1773110,2021197)}[k]
  require((w['sat_variables'],w['required_domain_clauses'],w['sat_clauses'])==expected and w['sat_xor_rows']==4980 and w['sat_and_gates']==82668 and w['sat_s3_nodes']==4,'independent all-affine counts differ')
  require(w['source_commit']==c['source_commit'] and 0<=w['memory_cgroup_peak_bytes']<=4*1024**3,'source/peak invalid')
  result.update({'sat_variables':w['sat_variables'],'domain_clauses':w['required_domain_clauses'],'sat_clauses':w['sat_clauses'],'peak_bytes':w['memory_cgroup_peak_bytes']})
 else:
  require(c['max_trials']==1 and c['conflict_budget']==(1000 if stage=='control' else 100000) and c['max_models']==32,'wrong search caps')
  require(o['input_changed'] is False,'input changed')
  fixtures=ROOT/'inputs/fixtures'
  for name,digest in c['fixture_sha256'].items():
   original='research/koblitz_n83_factor_base_sweep_20261008/verification/k2048-natural-highcap-20261018/evidence/frozen-input/fixtures/'+name
   pinned=subprocess.check_output(['git','show',c['source_commit']+':'+original],cwd=REPO)
   require(sha(fixtures/name)==digest==hashlib.sha256(pinned).hexdigest(),'fixture changed')
  if o['status']=='UNKNOWN_wall_cap':
   require(o['worker_exit_code'] is not None,'missing capped exit')
   result.update({'status':'PASS_CENSORED_OUTER_WALL','relations':None,'conflicts':None,'peak_bytes':o['worker_cgroup_peak_bytes']})
  else:
   w=read(out/'run/summary.json');wc=read(out/'run/config.json')
   require(o['worker_exit_code']==0 and o['worker_error'] is None and o['parse_error'] is None and o['group_replay_error'] is None,'worker failed')
   require(o['worker_sha256']==sha(out/'run/summary.json') and o['worker_config_sha256']==sha(out/'run/config.json'),'worker custody')
   require(o['status']==w['status'] and o['report']==w['report'],'worker/outer status or report mismatch')
   reason=search.receipt_error(w,wc,c);require(reason is None,'worker receipt invalid: '+str(reason))
   require(w['panel_manifest_blake3']==subprocess.check_output([str(ROOT/'b3file'),str(panel/'manifest.json')],text=True).strip(),'search manifest binding')
   if w['status']=='PASS_verified_target_only':
    target_rows=[x for x in read(fixtures/'probe-corpus.json')['targets'] if x['a']==0 and x['fixture']==0]
    known_rows=[x for x in read(fixtures/'probe-validation.json')['validation'] if x['a']==0 and x['fixture']==0]
    require(len(target_rows)==len(known_rows)==1 and int(w['verified_log'])==int(known_rows[0]['known_answer_scalar']) and search.point_mul(search.PRIMARY_GENERATOR,int(w['verified_log']))==search.parse_point(target_rows[0]['point']),'forged success')
   replay=search.replay_group(w,fixtures/'probe-corpus.json',archive);require(replay==o['group_replay'] and replay['status']=='PASS','original-group replay differs')
   result.update({'group_replay':replay,'relations':w['report']['relations'],'models':w['report']['sat_models'],'conflicts':w['report']['sat_conflicts'],'rank':w['report']['terminal_matrix_rank'],'peak_bytes':w['memory_cgroup_peak_bytes'],'solver_ms':w['solver_ms'],'attempt_records':w['attempt_records']})
 return result
if __name__=='__main__':
 stage,k=sys.argv[1],int(sys.argv[2]);out=Path(sys.argv[3]) if len(sys.argv)>3 else ROOT/(stage+'-k'+str(k))
 result=audit(stage,k,out);(out/'independent-audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))
