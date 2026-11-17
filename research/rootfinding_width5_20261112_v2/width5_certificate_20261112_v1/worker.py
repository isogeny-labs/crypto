"""Three-arm exact5D-width certificate comparison; coordinator executes only frozen plans."""
from pathlib import Path
import sys,os,time,json,gzip,hashlib,tempfile,subprocess,signal,itertools,statistics,copy
from research.seven_feature_blocks_20261109_v5.worker import coeffs
from research.curved_blocks_20261109_v6.worker import audit as mask_audit
from research.primal_cubic_20261109_v6.checker import audit,verify_public,norm_verify
from research.width5_certificate_20261112_v1.checker import seed_audit
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
def admit_seven(c,row):
 assert row['lll_complete'],'reduction cap: no comparative ratio'
 return mask_audit(c,row)
def determinant_controls():
 from research.width5_certificate_20261112_v1.checker import determinant
 def bareiss(rows):
  a=[row[:]for row in rows];previous=1;sign=1;n=len(a)
  for k in range(n-1):
   pivot=next((j for j in range(k,n)if a[j][k]),None)
   if pivot is None:return 0
   if pivot!=k:a[k],a[pivot]=a[pivot],a[k];sign=-sign
   value=a[k][k]
   for i in range(k+1,n):
    for j in range(k+1,n):
     numerator=value*a[i][j]-a[i][k]*a[k][j];assert numerator%previous==0;a[i][j]=numerator//previous
   for i in range(k+1,n):a[i][k]=0
   previous=value
  return sign*a[-1][-1]
 identity=[[int(i==j)for j in range(5)]for i in range(5)];matrices=[identity]
 swapped=[row[:]for row in identity];swapped[3],swapped[4]=swapped[4],swapped[3];assert determinant(swapped)==-1;matrices.append(swapped)
 doubled=[row[:]for row in identity];doubled[4]=[2*x for x in doubled[4]];assert determinant(doubled)==2;matrices.append(doubled)
 singular=[row[:]for row in identity];singular[4]=singular[3][:];assert determinant(singular)==0;matrices.append(singular)
 work=[row[:]for row in identity]
 for step in range(25):
  i,j=step%5,(step*3+1)%5
  if i==j:j=(j+1)%5
  if step%4==0:work[i],work[j]=work[j],work[i]
  else:work[i]=[x+(1 if step%2 else -1)*y for x,y in zip(work[i],work[j])]
  matrices.append([row[:]for row in work])
 for matrix in matrices:assert determinant(matrix)==bareiss(matrix)
 return len(matrices)
def phase_controls():
 from research.curved_blocks_20261109_v6.worker import curve
 from research.width5_certificate_20261112_v1.checker import interval_excludes
 checks=0
 for M in range(3):
  for u in range(4):
   for j in range(12):
    p=19;v=[(j*(i+1)+i*i)%9-4 for i in range(7)];cc=j-6;L,R=curve(v,cc,M,u)
    for k in [-3,-1,1,4]:
     ll,rr=curve(v,cc+k*p,M,u);assert(ll,rr)==(L+k*p,R+k*p);assert interval_excludes(ll,rr,p)==interval_excludes(L,R,p);checks+=1
 # T=9 mod19 has no roots for M=1 because elementary T is in[0,3].
 v=[0,0,0,0,0,1,0];cc=9
 for u in range(3):
  L,R=curve(v,cc,1,u);assert(L,R)==(6,9)and interval_excludes(L,R,19)
  for xyz in itertools.product(range(2),repeat=3):assert(sum(xyz[i]*xyz[j]for i in range(3)for j in range(i))-9)%19;checks+=1
 return dict(signed_phase_shift_checks=576,nonvacuous_source_triples=24,phase_control_predicates=checks,nonvacuous_exclusion_control=True)
def cases(tiny):
 if tiny:return [dict(id='tiny-'+str(p),p=p,M=2,H=2,h0=0,coefficients=[[1,2,0,1,0,1],[0,1,1,0,1,0]],positive=False,public=None)for p in [5,7,11,13,17]]
 source=ROOT/'research/dual_ladder_20261109_v3/planted_inputs.json';items=json.load(open(source))['cases'];assert len(items)==5;out=[]
 for item in items:
  pub=item['public'];p=int(pub['p']);h=int(pub['h']);offset={'P-192':0,'P-224':1,'P-256':3,'P-384':5,'P-521':7}[pub['field']];h0=h-offset;assert 0<=h0 and h0+7<p;cs=coeffs(p,int(pub['curve_a']),int(pub['curve_b']),[tuple(map(int,pub[k]))for k in ['target','anchor_x','anchor_y']],h0);assert cs is not None
  out.append(dict(id=pub['field'],p=p,M=int(pub['M']),H=7,h0=h0,coefficients=cs,positive=True,known_u=h-h0,public=pub))
 return out
def fixed(c,u):
 p=c['p'];qs=[]
 for a in c['coefficients']:qs.append([a[0]%p,(-2*a[0]*u*u-a[1]*u+a[3])%p,(a[0]*u**4+a[1]*u**3+a[2]*u*u+a[4]*u+a[5])%p])
 pub=dict(c['public'] or {});pub.update(p=str(p),M=str(c['M']),T=list(map(str,qs[0])),U=list(map(str,qs[1])),h=str(c['h0']+u));return pub
def arm(name,td,maskbin,rootbin,tag,tiny):
 began=time.monotonic();inputs=cases(tiny);masks={c['id']:'0'*(c['H']+1)for c in inputs};discovery=None;maskchecks=0
 if name in ['seven','five']:
  ip=td/(tag+'-mask.txt');txt='\n'.join(' '.join(map(str,[c['id'],c['p'],c['M'],c['H'],*c['coefficients'][0],*c['coefficients'][1]]))for c in inputs)+'\n';ip.write_text(txt);RAW.append(dict(stage=tag+'-mask-input',text=txt,sha256=hashlib.sha256(txt.encode()).hexdigest(),solver_received_expected_roots=False));discovery=call(tag+'-discover',[str(maskbin[name]),str(ip)],30);rows=json.loads(discovery['stdout']);assert len(rows)==len(inputs)==5
  for c,row in zip(inputs,rows):
   assert c['id']==row['id'];checked=(admit(c,row) if name=='five' else admit_seven(c,row));maskchecks+=checked['candidate_u_checks'];masks[c['id']]=row['curved_mask']
  assert maskchecks<=1960
  if tiny and name=='five':
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
 assert len(mapping)<=40
 ip=td/(tag+'-roots.txt');txt='\n'.join(lines)+('\n'if lines else'');ip.write_text(txt);RAW.append(dict(stage=tag+'-root-input',text=txt,sha256=hashlib.sha256(txt.encode()).hexdigest(),solver_received_expected_roots=False));run=call(tag+'-recover',[str(rootbin),str(ip)],40);out=json.loads(run['stdout']);assert out['count']==len(mapping)==len(out['cases']);assert {r['id']for r in out['cases']}==set(mapping);tuples=[];nodes=independent=0
 for row in out['cases']:
  c,u,pub=mapping[row['id']];assert row['complete']and row['enumeration_complete'];sols,n=audit(pub,row);nodes+=row['nodes'];independent+=n
  if not tiny:assert verify_public(pub)
  for sol in sols:
   extra=[]if tiny else[norm_verify(pub,sol)[3]];tuples.append([c['id'],u,*map(str,sol),*map(str,extra)])
 tuples.sort();assert len({tuple(t)for t in tuples})==len(tuples)
 serialized=json.dumps(tuples,separators=(',',':'));assert serialized;elapsed=time.monotonic()-began
 return dict(arm=name,paid_component_seconds=elapsed,masks=masks,solved_fibers=len(mapping),excluded_fibers=sum(m.count('1')for m in masks.values()),candidate_u_checks=maskchecks,native_nodes=nodes,independent_nodes=independent,conditional_ordered_root_count_exact=len(tuples),conditional_ordered_root_tuples=tuples,complete_conditional_block=True,native_recovery_seconds=run['wall_seconds'],native_recovery_peak_rss_kib=run['peak_process_rss_kib'],discovery_seconds=None if discovery is None else discovery['wall_seconds'],discovery_peak_rss_kib=None if discovery is None else discovery['peak_process_rss_kib'])
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
   for stem,source in [('native',ROOT/'research/masked_union_20261111_v1/native.cpp'),('seven',ROOT/'research/masked_union_20261111_v1/mask_native.cpp'),('five',P/'mask5_native.cpp')]:
    binary=td/stem;call('compile-'+stem,['/usr/bin/g++','-O2','-std=c++17',str(source),'-o',str(binary),*libs],25);binary_hashes[stem]=hashlib.sha256(binary.read_bytes()).hexdigest();bins[stem]=binary
    linked=call('linked-'+stem,['/usr/bin/ldd',str(binary)],10)
    for d in deps['entries']:
     if 'link/runtime'not in d['role']:continue
     soname=Path(d['path']).name.split('.so')[0]+'.so';hits=[z.split('=>',1)[1].split('(',1)[0].strip()for z in linked['stdout'].splitlines()if soname in z and'=>'in z];assert len(hits)==1 and Path(hits[0]).resolve()==Path(d['resolved_path'])
   arms=[];evidence=dict(arms=arms,complete_comparison=False)
   if mode=='controls':
    native=dict(legacy_exhaustive_kernel_controls_reused=True,legacy_bulk_controls_rerun=False);inputs=cases(True);direct,checks=tiny_oracle(inputs)
    for name in ['all','seven','five']:
     row=arm(name,td,bins,bins['native'],'tiny-'+name,True);assert row['conditional_ordered_root_tuples']==direct;arms.append(row)
    capinput=td/'tiny-five-mask.txt';caprows=json.loads(call('forced-zero-step-cap',[str(bins['five']),str(capinput),'cap0'],15)['stdout']);assert len(caprows)==5
    for c,row in zip(inputs,caprows):
     assert not row['lll_complete']
     try:admit(c,row)
     except AssertionError:pass
     else:raise AssertionError('capped discovery admitted')
    evidence.update(tiny_direct_checks=checks,tiny_direct_ordered_roots=len(direct),native_controls=native,seed_tamper_rejections=15,phase_controls=phase_controls(),independent_determinant_controls=determinant_controls(),new_declared_oracle_predicate_cap=200000,oracle_cap_scope='fresh declared scalar predicate cases; arithmetic operations and independently bounded lattice visits are separate',optional_reciprocal_tested=False,forced_cap_branches_rejected=5,complete_comparison=True)
   else:
    for rep,order in enumerate([['all','seven','five'],['five','seven','all'],['seven','all','five']]):
     pair={}
     for name in order:
      row=arm(name,td,bins,bins['native'],str(rep)+'-'+name,False);row['rep']=rep;arms.append(row);pair[name]=row
     assert pair['all']['conditional_ordered_root_tuples']==pair['seven']['conditional_ordered_root_tuples']==pair['five']['conditional_ordered_root_tuples'],'complete conditional unions differ'
    medians={name:statistics.median(a['paid_component_seconds']for a in arms if a['arm']==name)for name in ['all','seven','five']};evidence.update(complete_comparison=True,median_component_seconds=medians,diagnostic_all_over_five=medians['all']/medians['five'],diagnostic_seven_over_five=medians['seven']/medians['five'],prediction_supported=medians['five']<medians['seven'],three_fresh_native_paired_repetitions=True)
  status='passed';summary='Complete conditional unions independently replayed; masking cost hypothesis tested only on exposed controls.'
 except (OSError,RuntimeError,AssertionError,ValueError,KeyError,StopIteration)as exc:status='inconclusive';summary='Conditional block experiment incomplete; no empty-root or performance claim.';evidence.update(error=str(exc),complete_comparison=False)
 blob=gzip.compress(json.dumps(RAW,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as f:f.write(blob)
 evidence.update(scope_complete=status=='passed',compiled_binary_sha256=binary_hashes,raw_sha256=hashlib.sha256(blob).hexdigest(),stage_metrics=[{k:r[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']if k in r}for r in RAW],memory_metric='wait4 per-child peak RSS KiB; not aggregate descendants',compilation_policy='three common compilations charged whole worker, excluded equally from paid arms',timing_scope='fresh native processes, public construction, discovery, independent certificate/root/norm replay, full tuple serialization; one shared Python wrapper',qualified_isolation=False,A_A_not_run=True,training_post_result=True,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary=summary,evidence=evidence,next_actions=['If width5 discovery loses pruning or cost, test a fixed eight-pivot4D cover or compact simultaneous phase optimization; do not retry the same seed longer.', 'Compare against external FLINT/fplll and obtain global exceptional/auxiliary branch closure before release comparisons.']);text=json.dumps(result)
 if time.monotonic()-START>=84:
  result.update(status='inconclusive',summary='Final report exceeded acceptance budget.');evidence.update(scope_complete=False,complete_comparison=False,diagnostic_all_over_five=None,diagnostic_seven_over_five=None)
  for row in evidence.get('arms',[]):row.update(complete_conditional_block=False,conditional_ordered_root_count_exact=None)
  text=json.dumps(result)
 assert time.monotonic()-START<88,'final serialized stdout deadline';print(text)
if __name__=='__main__':main()
