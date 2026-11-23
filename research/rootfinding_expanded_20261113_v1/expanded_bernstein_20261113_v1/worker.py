"""Expanded-domain source-coupled exact conditional comparison; coordinator executes only frozen plans."""
from pathlib import Path
import sys,os,time,json,gzip,hashlib,tempfile,subprocess,signal,itertools,statistics,copy
from research.seven_feature_blocks_20261109_v5.worker import coeffs
from research.primal_cubic_20261109_v6.checker import audit,verify_public,norm_verify
from research.width5_certificate_20261112_v1.checker import seed_audit
from research.expanded_bernstein_20261113_v1.bounds import coupled_row, integration_controls
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();RAW=[]
def call(stage,args,cap):
 with tempfile.TemporaryFile()as out,tempfile.TemporaryFile()as err:
  left=min(cap,80-(time.monotonic()-START));assert left>0,'whole worker budget';began=time.monotonic();child=subprocess.Popen(args,stdout=out,stderr=err,start_new_session=True);timed=False
  while True:
   pid,status,usage=os.wait4(child.pid,os.WNOHANG)
   if pid:break
   if time.monotonic()-began>=left:
    timed=True;os.killpg(child.pid,signal.SIGKILL);pid,status,usage=os.wait4(child.pid,0);break
   time.sleep(.005)
  child.returncode=os.waitstatus_to_exitcode(status);out.seek(0);err.seek(0);row=dict(stage=stage,command=args,returncode=child.returncode,timed_out=timed,wall_seconds=time.monotonic()-began,peak_process_rss_kib=usage.ru_maxrss,stdout=out.read().decode(errors='replace'),stderr=err.read().decode(errors='replace'));RAW.append(row);assert not timed and child.returncode==0,stage+' incomplete';return row
def admit(c,row):
 assert row['lll_complete'],'reduction cap: no comparative ratio'
 return seed_audit(c,row)
PANEL=ROOT/'research/width5_domain_expansion_plan_20261112_v1/UNOPENED_INPUTS.json'
PANEL_SHA='534edd6f5cad10dbb5da99bc4b5d2754f18ed6972bd861f3a86f96894eceac5e'
def cases(tiny):
 if tiny:return [dict(id='tiny-'+str(p),p=p,M=2,H=8,h0=0,coefficients=[[1,2,0,1,0,1],[0,1,1,0,1,0]],positive=False,public=None)for p in [5,7,11,13,17]]
 blob=PANEL.read_bytes();assert hashlib.sha256(blob).hexdigest()==PANEL_SHA;data=json.loads(blob);items=data['cases'];assert len(items)==5;out=[]
 for pub in items:
  p=int(pub['p']);M=int(pub['M']);h0=int(pub['h0']);H=pub['H'];assert H==15 and 0<=h0 and h0+H<p
  digest=hashlib.sha256((data['seed']+'|'+pub['field']+'|width=16').encode()).hexdigest();assert digest==pub['placement_digest'];assert h0==int(digest,16)%(p-15)
  cs=coeffs(p,int(pub['curve_a']),int(pub['curve_b']),[tuple(map(int,pub[k]))for k in ['target','anchor_x','anchor_y']],h0);assert cs is not None,'fixed public chart guard failed'
  out.append(dict(id=pub['field'],p=p,M=M,H=H,h0=h0,coefficients=cs,positive=False,public=pub))
 return out
BATCH_MAX=40
def validate_batches(outputs,expected_ids):
 wanted=[expected_ids[i:i+BATCH_MAX]for i in range(0,len(expected_ids),BATCH_MAX)] or [[]]
 assert len(outputs)==len(wanted),'missing recovery batch'
 rows=[]
 for out,ids in zip(outputs,wanted):
  assert len(ids)<=BATCH_MAX and out['count']==len(ids)==len(out['cases']),'batch count mismatch'
  assert {row['id']for row in out['cases']}==set(ids),'batch ID mismatch'
  assert all(row['complete']and row['enumeration_complete']for row in out['cases']),'incomplete recovered fiber'
  rows.extend(out['cases'])
 assert len(rows)==len(expected_ids) and {row['id']for row in rows}==set(expected_ids),'whole mapping coverage'
 return rows
def recover_batches(mapping,lines,td,rootbin,tag):
 ids=list(mapping);assert len(ids)==len(lines)<=80
 chunks=[(ids[i:i+BATCH_MAX],lines[i:i+BATCH_MAX])for i in range(0,len(ids),BATCH_MAX)] or [([],[])]
 outputs=[];runs=[]
 for bi,(expected,part) in enumerate(chunks):
  ip=td/(tag+'-roots-'+str(bi)+'.txt');txt='\n'.join(part)+('\n'if part else'');ip.write_text(txt);RAW.append(dict(stage=tag+'-root-input-'+str(bi),batch=bi,expected_ids=expected,text=txt,sha256=hashlib.sha256(txt.encode()).hexdigest(),solver_received_expected_roots=False));run=call(tag+'-recover-batch-'+str(bi),[str(rootbin),str(ip)],40);outputs.append(json.loads(run['stdout']));runs.append(run)
 rows=validate_batches(outputs,ids)
 return dict(count=len(ids),cases=rows),dict(wall_seconds=sum(v['wall_seconds']for v in runs),peak_process_rss_kib=max(v['peak_process_rss_kib']for v in runs),batch_sizes=[len(q[0])for q in chunks])
def batch_controls(inputs,td,rootbin):
 ids=[c['id']+'-u'+str(u)for c in inputs for u in range(c['H']+1)];assert len(ids)==45
 outputs=[json.loads(v['stdout'])for v in RAW if v.get('stage','').startswith('tiny-all-recover-batch-')];assert len(validate_batches(outputs,ids))==45
 faults=[]
 for kind in ['missing-second','capped-second','duplicate-second-id']:
  bad=copy.deepcopy(outputs)
  if kind=='missing-second':bad.pop()
  elif kind=='capped-second':bad[1]['cases'][0]['complete']=False
  else:bad[1]['cases'][0]['id']=bad[0]['cases'][0]['id']
  try:validate_batches(bad,ids)
  except AssertionError:faults.append(kind)
  else:raise AssertionError('bad second batch admitted')
 out,run=recover_batches({},[],td,rootbin,'empty-control');assert out==dict(count=0,cases=[]) and run['batch_sizes']==[0]
 return dict(whole_dense_fibers=45,real_batch_sizes=[40,5],faults_rejected=faults,empty_batch_complete=True)
def fixed(c,u):
 p=c['p'];qs=[]
 for a in c['coefficients']:qs.append([a[0]%p,(-2*a[0]*u*u-a[1]*u+a[3])%p,(a[0]*u**4+a[1]*u**3+a[2]*u*u+a[4]*u+a[5])%p])
 pub=dict(c['public'] or {});pub.update(p=str(p),M=str(c['M']),T=list(map(str,qs[0])),U=list(map(str,qs[1])),h=str(c['h0']+u));return pub
def arm(name,td,maskbin,rootbin,tag,tiny):
 began=time.monotonic();bernchecks=extra=0;inputs=cases(tiny);masks={c['id']:'0'*(c['H']+1)for c in inputs};discovery=None;maskchecks=0
 if name in ['old','bern']:
  ip=td/(tag+'-mask.txt');txt='\n'.join(' '.join(map(str,[c['id'],c['p'],c['M'],c['H'],*c['coefficients'][0],*c['coefficients'][1]]))for c in inputs)+'\n';ip.write_text(txt);RAW.append(dict(stage=tag+'-mask-input',text=txt,sha256=hashlib.sha256(txt.encode()).hexdigest(),solver_received_expected_roots=False));discovery=call(tag+'-discover',[str(maskbin['five']),str(ip)],30);rows=json.loads(discovery['stdout']);assert len(rows)==len(inputs)==5
  for c,row in zip(inputs,rows):
   assert c['id']==row['id'];checked=admit(c,row);maskchecks+=checked['candidate_u_checks'];masks[c['id']]=row['curved_mask']
   if name=='bern':
    coupled=coupled_row(c,row,tiny);masks[c['id']]=coupled['new_mask'];bernchecks+=coupled['candidate_u_checks'];extra+=coupled['extra_exclusions'];RAW.append(dict(stage=tag+'-bernstein-replay',field=c['id'],**coupled))
  assert maskchecks<=2000
  if tiny and name=='old':
   for c,row in zip(inputs,rows):
    for change in range(3):
     bad=copy.deepcopy(row)
     if change==0:bad['candidates'][0]['seed'][0]=str(int(bad['candidates'][0]['seed'][0])+1)
     elif change==1:bad['candidates'][0]['seed']=[str(2*int(x))for x in bad['candidates'][0]['seed']]
     else:bad['candidates'][0]['phases'][0]=str(int(bad['candidates'][0]['phases'][0])+1)
     try:seed_audit(c,bad)
     except AssertionError:pass
     else:raise AssertionError('seed/lift tamper accepted')
 mapping={};lines=[]
 for c in inputs:
  for u in range(c['H']+1):
   if masks[c['id']][u]=='1':continue
   pub=fixed(c,u);ident=c['id']+'-u'+str(u);mapping[ident]=(c,u,pub);lines.append(' '.join(map(str,[ident,pub['p'],pub['M'],*pub['T'],*pub['U']])))
 assert len(mapping)<=80
 assert len(mapping)+sum(m.count('1')for m in masks.values())==sum(c['H']+1 for c in inputs),'complete u coverage'
 out,run=recover_batches(mapping,lines,td,rootbin,tag);tuples=[];nodes=independent=0
 for row in out['cases']:
  c,u,pub=mapping[row['id']];assert row['complete']and row['enumeration_complete'];sols,n=audit(pub,row);nodes+=row['nodes'];independent+=n
  if not tiny:assert verify_public(pub)
  for sol in sols:
   root_extra=[]if tiny else[norm_verify(pub,sol)[3]];tuples.append([c['id'],u,*map(str,sol),*map(str,root_extra)])
 tuples.sort();assert len({tuple(t)for t in tuples})==len(tuples)
 serialized=json.dumps(tuples,separators=(',',':'));assert serialized;elapsed=time.monotonic()-began
 return dict(arm=name,paid_component_seconds=elapsed,masks=masks,solved_fibers=len(mapping),excluded_fibers=sum(m.count('1')for m in masks.values()),candidate_u_checks=maskchecks,bernstein_candidate_u_checks=bernchecks,bernstein_extra_exclusions=extra,native_nodes=nodes,independent_nodes=independent,conditional_ordered_root_count_exact=len(tuples),conditional_ordered_root_tuples=tuples,complete_conditional_block=True,native_recovery_seconds=run['wall_seconds'],native_recovery_peak_rss_kib=run['peak_process_rss_kib'],recovery_batch_sizes=run['batch_sizes'],discovery_seconds=None if discovery is None else discovery['wall_seconds'],discovery_peak_rss_kib=None if discovery is None else discovery['peak_process_rss_kib'])
def tiny_oracle(inputs):
 result=[];checks=0
 for c in inputs:
  for u in range(c['H']+1):
   pub=fixed(c,u);t=list(map(int,pub['T']));v=list(map(int,pub['U']));p=c['p']
   for x,y,z in itertools.product(range(c['M']+1),repeat=3):
    checks+=1;S=x+y+z;T=x*y+x*z+y*z;U=x*y*z
    if (T-t[0]*S*S-t[1]*S-t[2])%p==0 and(U-v[0]*S*S-v[1]*S-v[2])%p==0:result.append([c['id'],u,*map(str,[S,x,y,z])])
 return sorted(result),checks
def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','profile'];evidence={};binary_hashes={}
 try:
  deps=json.load(open(P/'EXTERNAL_GMP.json'));libs=[]
  for d in deps['entries']:
   assert hashlib.sha256(Path(d['resolved_path']).read_bytes()).hexdigest()==d['sha256']
   if 'link/runtime'in d['role']:libs.append(d['resolved_path'])
  with tempfile.TemporaryDirectory(prefix='masked-union-')as work:
   td=Path(work);bins={}
   for stem,source in [('native',ROOT/'research/masked_union_20261111_v1/native.cpp'),('five',ROOT/'research/width5_certificate_20261112_v1/mask5_native.cpp')]:
    binary=td/stem;call('compile-'+stem,['/usr/bin/g++','-O2','-std=c++17',str(source),'-o',str(binary),*libs],25);binary_hashes[stem]=hashlib.sha256(binary.read_bytes()).hexdigest();bins[stem]=binary
    linked=call('linked-'+stem,['/usr/bin/ldd',str(binary)],10)
    for d in deps['entries']:
     if 'link/runtime'not in d['role']:continue
     soname=Path(d['path']).name.split('.so')[0]+'.so';hits=[z.split('=>',1)[1].split('(',1)[0].strip()for z in linked['stdout'].splitlines()if soname in z and'=>'in z];assert len(hits)==1 and Path(hits[0]).resolve()==Path(d['resolved_path'])
   arms=[];evidence=dict(arms=arms,complete_comparison=False)
   if mode=='controls':
    native=dict(legacy_exhaustive_kernel_controls_reused=True,legacy_bulk_controls_rerun=False);inputs=cases(True);direct,checks=tiny_oracle(inputs)
    for name in ['all','old','bern']:
     row=arm(name,td,bins,bins['native'],'tiny-'+name,True);assert row['conditional_ordered_root_tuples']==direct;arms.append(row)
    evidence.update(tiny_direct_checks=checks,tiny_direct_ordered_roots=len(direct),batch_controls=batch_controls(inputs,td,bins['native']),native_controls=native,bernstein_integration_controls=integration_controls(),complete_comparison=True)
   else:
    for rep,order in enumerate([['all','old','bern'],['bern','old','all'],['old','all','bern']]):
     pair={}
     for name in order:
      row=arm(name,td,bins,bins['native'],str(rep)+'-'+name,False);row['rep']=rep;arms.append(row);pair[name]=row
     assert pair['all']['conditional_ordered_root_tuples']==pair['old']['conditional_ordered_root_tuples']==pair['bern']['conditional_ordered_root_tuples'],'complete conditional unions differ'
    medians={name:statistics.median(a['paid_component_seconds']for a in arms if a['arm']==name)for name in ['all','old','bern']};evidence.update(complete_comparison=True,median_component_seconds=medians,diagnostic_all_over_bern=medians['all']/medians['bern'],diagnostic_old_over_bern=medians['old']/medians['bern'],prediction_supported=medians['bern']<medians['old'],three_fresh_native_paired_repetitions=True)
  status='passed';summary='Complete expanded-domain conditional unions with source-coupled checks; no global or qualified GB claim.'
 except (OSError,RuntimeError,AssertionError,ValueError,KeyError,StopIteration)as exc:
  status='inconclusive';summary='Conditional block experiment incomplete; no empty-root or performance claim.';evidence.update(error=str(exc),complete_comparison=False,diagnostic_all_over_bern=None,diagnostic_old_over_bern=None,prediction_supported=None)
  for row in evidence.get('arms',[]):row.update(complete_conditional_block=False,conditional_ordered_root_count_lower_bound=len(row['conditional_ordered_root_tuples']),conditional_ordered_root_count_exact=None)
 blob=gzip.compress(json.dumps(RAW,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as f:f.write(blob)
 evidence.update(scope_complete=status=='passed',compiled_binary_sha256=binary_hashes,raw_sha256=hashlib.sha256(blob).hexdigest(),stage_metrics=[{k:r[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']if k in r}for r in RAW],memory_metric='wait4 per-child peak RSS KiB; not aggregate descendants',compilation_policy='two common compilations charged whole worker, excluded equally from paid arms',timing_scope='fresh native processes, public construction, discovery, independent certificate/root/norm replay, full tuple serialization; one shared Python wrapper',qualified_isolation=False,A_A_not_run=True,training_post_result=True,expanded_domain_panel_first_evaluation=True,panel_sha256=PANEL_SHA,field_setup_policy='Prior exact field/group proof records reused; no cold proof regeneration charged to arm.',original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary=summary,evidence=evidence,next_actions=['If expanded domain has no extra exclusion headroom or Bernstein total cost loses, inspect source-coupled discovery coordinates or bounded disjoint source cells in a new variant; never retry identical capped inputs.', 'Compare against external FLINT/fplll and obtain global exceptional/auxiliary branch closure before release comparisons.']);text=json.dumps(result)
 if time.monotonic()-START>=84:
  result.update(status='inconclusive',summary='Final report exceeded acceptance budget.');evidence.update(scope_complete=False,complete_comparison=False,diagnostic_all_over_bern=None,diagnostic_old_over_bern=None,prediction_supported=None)
  for row in evidence.get('arms',[]):row.update(complete_conditional_block=False,conditional_ordered_root_count_lower_bound=len(row['conditional_ordered_root_tuples']),conditional_ordered_root_count_exact=None)
  text=json.dumps(result)
 assert time.monotonic()-START<88,'final serialized stdout deadline';print(text)
if __name__=='__main__':main()
