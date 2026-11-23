"""Frozen bounded conditional block-union comparison; coordinator runs commands."""
from pathlib import Path
import sys,os,time,json,gzip,hashlib,tempfile,subprocess,signal,itertools,statistics
from research.seven_feature_blocks_20261109_v5.worker import coeffs
from research.curved_blocks_20261109_v6.worker import audit as mask_audit
from research.primal_cubic_20261109_v6.checker import audit,verify_public,norm_verify
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
def cases(tiny):
 if tiny:return [dict(id='tiny-'+str(p),p=p,M=2,H=2,h0=0,coefficients=[[1,2,0,1,0,1],[0,1,1,0,1,0]],positive=False,public=None)for p in [5,7,11,13,17]]
 source=ROOT/'research/dual_ladder_20261109_v3/planted_inputs.json';items=json.load(open(source))['cases'];assert len(items)==5;out=[]
 for item in items:
  pub=item['public'];p=int(pub['p']);h=int(pub['h']);h0=max(0,min(h-3,p-8));cs=coeffs(p,int(pub['curve_a']),int(pub['curve_b']),[tuple(map(int,pub[k]))for k in ['target','anchor_x','anchor_y']],h0);assert cs is not None
  out.append(dict(id=pub['field'],p=p,M=int(pub['M']),H=7,h0=h0,coefficients=cs,positive=True,known_u=h-h0,public=pub))
 return out
def fixed(c,u):
 p=c['p'];qs=[]
 for a in c['coefficients']:qs.append([a[0]%p,(-2*a[0]*u*u-a[1]*u+a[3])%p,(a[0]*u**4+a[1]*u**3+a[2]*u*u+a[4]*u+a[5])%p])
 pub=dict(c['public'] or {});pub.update(p=str(p),M=str(c['M']),T=list(map(str,qs[0])),U=list(map(str,qs[1])),h=str(c['h0']+u));return pub
def arm(name,td,maskbin,rootbin,tag,tiny):
 began=time.monotonic();inputs=cases(tiny);masks={c['id']:'0'*(c['H']+1)for c in inputs};discovery=None;maskchecks=0
 if name=='masked':
  ip=td/(tag+'-mask.txt');txt='\n'.join(' '.join(map(str,[c['id'],c['p'],c['M'],c['H'],*c['coefficients'][0],*c['coefficients'][1]]))for c in inputs)+'\n';ip.write_text(txt);RAW.append(dict(stage=tag+'-mask-input',text=txt,sha256=hashlib.sha256(txt.encode()).hexdigest(),solver_received_expected_roots=False));discovery=call(tag+'-discover',[str(maskbin),str(ip)],30);rows=json.loads(discovery['stdout']);assert len(rows)==len(inputs)==5
  for c,row in zip(inputs,rows):
   assert c['id']==row['id'];checked=mask_audit(c,row);maskchecks+=checked['candidate_u_checks'];masks[c['id']]=row['curved_mask']
  assert maskchecks<=1960
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
   for stem in ['native','mask_native']:
    binary=td/stem;call('compile-'+stem,['/usr/bin/g++','-O2','-std=c++17',str(P/(stem+'.cpp')),'-o',str(binary),*libs],25);binary_hashes[stem]=hashlib.sha256(binary.read_bytes()).hexdigest();bins[stem]=binary
    linked=call('linked-'+stem,['/usr/bin/ldd',str(binary)],10)
    for d in deps['entries']:
     if 'link/runtime'not in d['role']:continue
     soname=Path(d['path']).name.split('.so')[0]+'.so';hits=[z.split('=>',1)[1].split('(',1)[0].strip()for z in linked['stdout'].splitlines()if soname in z and'=>'in z];assert len(hits)==1 and Path(hits[0]).resolve()==Path(d['resolved_path'])
   arms=[];evidence=dict(arms=arms,complete_comparison=False)
   if mode=='controls':
    native=json.loads(call('native-small-oracles',[str(bins['native']),'controls'],25)['stdout']);inputs=cases(True);direct,checks=tiny_oracle(inputs)
    for name in ['all','masked']:
     row=arm(name,td,bins['mask_native'],bins['native'],'tiny-'+name,True);assert row['conditional_ordered_root_tuples']==direct;arms.append(row)
    evidence.update(tiny_direct_checks=checks,tiny_direct_ordered_roots=len(direct),native_controls=native,complete_comparison=True)
   else:
    for rep,order in enumerate([['all','masked'],['masked','all'],['all','masked']]):
     pair={}
     for name in order:
      row=arm(name,td,bins['mask_native'],bins['native'],str(rep)+'-'+name,False);row['rep']=rep;arms.append(row);pair[name]=row
     assert pair['all']['conditional_ordered_root_tuples']==pair['masked']['conditional_ordered_root_tuples'],'complete conditional unions differ'
    baseline=statistics.median(a['paid_component_seconds']for a in arms if a['arm']=='all');candidate=statistics.median(a['paid_component_seconds']for a in arms if a['arm']=='masked');evidence.update(complete_comparison=True,baseline_median_component_seconds=baseline,masked_median_component_seconds=candidate,diagnostic_median_ratio=baseline/candidate,prediction_supported=candidate<baseline,three_fresh_native_paired_repetitions=True)
  status='passed';summary='Complete conditional unions independently replayed; masking cost hypothesis tested only on exposed controls.'
 except (OSError,RuntimeError,AssertionError,ValueError,KeyError,StopIteration)as exc:status='inconclusive';summary='Conditional block experiment incomplete; no empty-root or performance claim.';evidence.update(error=str(exc),complete_comparison=False)
 blob=gzip.compress(json.dumps(RAW,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as f:f.write(blob)
 evidence.update(scope_complete=status=='passed',compiled_binary_sha256=binary_hashes,raw_sha256=hashlib.sha256(blob).hexdigest(),stage_metrics=[{k:r[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']if k in r}for r in RAW],memory_metric='wait4 per-child peak RSS KiB; not aggregate descendants',compilation_policy='two common compilations charged whole worker, excluded equally from paid arms',timing_scope='fresh native processes, public construction, discovery, independent certificate/root/norm replay, full tuple serialization; one shared Python wrapper',qualified_isolation=False,A_A_not_run=True,training_post_result=True,original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False)
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary=summary,evidence=evidence,next_actions=['If masks cost more than saved recovery, reduce certificate discovery or use cheaper reused vectors on new inputs; do not repeat identical paid family.', 'Compare against external FLINT/fplll and obtain global exceptional/auxiliary branch closure before release comparisons.']);text=json.dumps(result)
 if time.monotonic()-START>=84:
  result.update(status='inconclusive',summary='Final report exceeded acceptance budget.');evidence.update(scope_complete=False,complete_comparison=False,diagnostic_median_ratio=None)
  for row in evidence.get('arms',[]):row.update(complete_conditional_block=False,conditional_ordered_root_count_exact=None)
  text=json.dumps(result)
 assert time.monotonic()-START<88,'final serialized stdout deadline';print(text)
if __name__=='__main__':main()
