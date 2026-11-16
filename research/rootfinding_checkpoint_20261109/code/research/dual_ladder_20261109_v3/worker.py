"""Bounded native search with independent exact certificate audit."""
from pathlib import Path
import os,sys,time,tempfile,subprocess,json,gzip,hashlib
from .checker import check,verify_row,verify_positive
p=Path(__file__).resolve().parent;root=p.parents[1];ident=sys.argv[1];mode=sys.argv[2];assert mode in('controls','profile');started=time.monotonic();raw=[];compiler=Path('/usr/bin/g++').resolve();binary_sha=None

def call(stage,args,cap):
 with tempfile.TemporaryFile()as out,tempfile.TemporaryFile()as err:
  left=83-(time.monotonic()-started)
  if left<=0:raise RuntimeError('whole worker child budget')
  t=time.monotonic();child=subprocess.Popen(args,stdout=out,stderr=err);deadline=t+min(left,cap);timed=False
  while True:
   pid,status,usage=os.wait4(child.pid,os.WNOHANG)
   if pid:break
   if time.monotonic()>=deadline:timed=True;child.kill();pid,status,usage=os.wait4(child.pid,0);break
   time.sleep(.005)
  child.returncode=os.waitstatus_to_exitcode(status);out.seek(0);err.seek(0);item=dict(stage=stage,command=args,returncode=child.returncode,timed_out=timed,wall_seconds=time.monotonic()-t,peak_process_rss_kib=usage.ru_maxrss,stdout=out.read().decode(errors='replace'),stderr=err.read().decode(errors='replace'));raw.append(item)
  assert not timed and child.returncode==0,stage+' incomplete'
  return item
try:
 deps=json.load(open(p/'EXTERNAL_GMP.json'));libs=[]
 for dep in deps['entries']:
  q=Path(dep['resolved_path']);assert hashlib.sha256(q.read_bytes()).hexdigest()==dep['sha256'],'GMP dependency mismatch'
  if 'link/runtime'in dep['role']:libs.append(str(q))
 with tempfile.TemporaryDirectory(prefix='dual-lattice-')as td:
  td=Path(td);b=td/'native';call('compile',['/usr/bin/g++','-std=c++17','-O2','-I/usr/include/x86_64-linux-gnu',str(p/'native.cpp'),'-o',str(b),*libs],35);binary_sha=hashlib.sha256(b.read_bytes()).hexdigest();linked=call('linked-libraries',['/usr/bin/ldd',str(b)],10)
  for dep in deps['entries']:
   if 'link/runtime'not in dep['role']:continue
   soname=Path(dep['path']).name.split('.so')[0]+'.so';hits=[line.split('=>',1)[1].split('(',1)[0].strip()for line in linked['stdout'].splitlines()if soname in line and '=>'in line];assert len(hits)==1 and Path(hits[0]).resolve()==Path(dep['resolved_path']),'GMP loader identity'
  if mode=='controls':
   run=call('native-controls',[str(b),ident,'controls'],80);result=json.loads(run['stdout']);assert result['status']=='passed'and result['evidence']['false_exclusions']==0
   certs=result['evidence'].pop('certificates');assert all(check(c)for c in certs);tamper=0
   for cert in certs:
    for key in ['constant','maximum','quotient_low']:
     bad=dict(cert);bad[key]=str(int(bad[key])+1)
     try:check(bad)
     except AssertionError:tamper+=1
     else:raise AssertionError('tampered certificate accepted')
   result['evidence'].update(independent_python_certificates=len(certs),tamper_rejections=tamper,controls_complete=True,certificates_retained_in_raw_stdout=True)
  else:
   rounds=[];planted=None;canonical=None
   for ri in range(3):
    begin=time.monotonic();rr=call('exact-field-group-setup-'+str(ri),[sys.executable,'-m','research.nist_sum_gmp_20261109_v1.setup'],45);setup=json.loads(rr['stdout']);assert [v['name']for v in setup['rows']]==['P-192','P-224','P-256','P-384','P-521'];inputs=td/('inputs-'+str(ri));inputs.write_text(setup['native_input']);records={}
    for line in setup['native_input'].splitlines():
     name,bits,dim,*ints=line.split();records[name]=[int(ints[0]),int(dim),*map(int,ints[1:])]
    for row in setup['rows']:
     for key in ['field','group_order']:
      proof=row['proof'][key];assert proof['proven_prime']and hashlib.sha256(Path(proof['library_path']).read_bytes()).hexdigest()==proof['library_sha256']
    posrow=call('positive-controls-'+str(ri),[str(b),ident+'-positive-'+str(ri),'positive',str(inputs)],80);pos=json.loads(posrow['stdout']);assert len(pos['cases'])==5 and all(verify_positive(v,records)for v in pos['cases']);assert {v['public']['field']for v in pos['cases']}==set(records)
    if planted is None:
     planted=pos;payload=json.dumps(planted,sort_keys=True,indent=2)+'\n'
     with(p/'planted_inputs.json').open('x')as out:out.write(payload)
     raw.append(dict(stage='verified-positive-artifact-export',sha256=hashlib.sha256(payload.encode()).hexdigest(),cases=5,label='PLANTED_NOT_BENCHMARK'))
    else:assert pos==planted,'planted generation changed across repetitions'
    run=call('native-profile-'+str(ri),[str(b),ident+'-native-'+str(ri),'profile',str(inputs)],80);value=json.loads(run['stdout']);assert value['status']=='passed'and value['evidence']['conditional_lines']==320;rows=value['evidence']['rows'];assert {(v['field'],v['pair'],v['h_index'])for v in rows}=={(name,pi,hi)for name in records for pi in range(8)for hi in range(8)};verified=0;compact=[]
    for row in rows:
     out={k:row[k]for k in ['field','pair','h_index','closed','A0','steps','declared_parameter_count','unresolved_parameter_count']}
     if row['closed']:
      assert verify_row(row,records);cert=row['certificate'];out['certificate_sha256']=hashlib.sha256(json.dumps(cert,sort_keys=True,separators=(',',':')).encode()).hexdigest();verified+=1
     else:assert row['certificate']is None and int(row['unresolved_parameter_count'])==int(row['declared_parameter_count'])
     compact.append(out)
    assert verified==value['evidence']['certified_closed_lines'];answer=[(v['field'],v['pair'],v['h_index'],v['closed'],v['unresolved_parameter_count'])for v in compact]
    if canonical is None:canonical=answer
    else:assert answer==canonical,'whole-chart outcomes differ'
    rounds.append(dict(round=ri,cold_pipeline_seconds=time.monotonic()-begin,native_profile_seconds=run['wall_seconds'],native_peak_rss_kib=run['peak_process_rss_kib'],certified_closed_lines=verified,LLL_capped_lines=value['evidence']['LLL_capped_lines'],LLL_total_steps=value['evidence']['LLL_total_steps'],rows=compact,paid_exact_setup=setup['rows']))
   times=[v['cold_pipeline_seconds']for v in rounds];result=dict(schema='scientific_result_v1',experiment_id=ident,status='passed',summary='Three cold expanded five-NIST dual-certificate panels and planted repeated-source nonrejection controls completed.',evidence=dict(bounded_profile_complete=True,fields=5,conditional_lines_per_round=320,cold_repetitions=3,rounds=rounds,all_round_outcomes_match=True,cold_pipeline_seconds=times,repeat_relative_range=(max(times)-min(times))/min(times),planted_positive_fields=5,planted_positive_checks=15,planted_control_label='PLANTED_NOT_BENCHMARK',planted_artifact='research/dual_ladder_20261109_v3/planted_inputs.json',planted_artifact_sha256=hashlib.sha256(payload.encode()).hexdigest(),independent_positive_norm_checks=15,full_certificates_retained_in_raw_stdout=True,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False),next_actions=['Use separately frozen planted public inputs for bounded primal enumeration; preserve all uncertified/global branches.'])
except (OSError,RuntimeError,AssertionError,ValueError,KeyError)as exc:
 result=dict(schema='scientific_result_v1',experiment_id=ident,status='inconclusive',summary='Dual-lattice attempt incomplete; raw stages retained.',evidence=dict(error=str(exc),bounded_profile_complete=False,certified_closed_lines=None,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False),next_actions=['Inspect exact failure and freeze a changed assumption before another experiment.'])
blob=gzip.compress(json.dumps(raw,separators=(',',':')).encode(),mtime=0)
with(p/('raw-'+mode+'.json.gz')).open('xb')as f:f.write(blob)
result['evidence'].update(raw_artifact_sha256=hashlib.sha256(blob).hexdigest(),compiler_sha256=hashlib.sha256(compiler.read_bytes()).hexdigest(),compiled_binary_sha256=binary_sha,stage_metrics=[{k:x[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']if k in x}for x in raw],memory_metric='wait4 per-process peak RSS KiB, not aggregate descendants')
text=json.dumps(result,separators=(',',':'))
if time.monotonic()-started>=86:
 result.update(status='inconclusive',summary='Dual report exceeded conservative acceptance budget');result['evidence'].update(bounded_profile_complete=False,controls_complete=False,certified_closed_lines=None,closed_maps=None,positive_maps=None,maps=None,independent_python_certificates=None,original_root_count_exact=None,qualified_speedup=None,release_gate_claim=False)
 for row in result['evidence'].get('rows',[]):row.update(closed=False,unresolved_parameter_count=row['declared_parameter_count']);row.pop('certificate_sha256',None)
 for rd in result['evidence'].get('rounds',[]):
  rd['certified_closed_lines']=None
  for row in rd['rows']:row.update(closed=False,unresolved_parameter_count=row['declared_parameter_count']);row.pop('certificate_sha256',None)
 text=json.dumps(result,separators=(',',':'))
assert time.monotonic()-started<88,'final serialization deadline';print(text)
