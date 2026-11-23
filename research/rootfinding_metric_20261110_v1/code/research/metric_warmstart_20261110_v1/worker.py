"""Fresh exact metric repair controls and bounded six-arm diagnostic."""
from pathlib import Path
import sys,os,json,time,gzip,hashlib,tempfile,subprocess
from research.curved_blocks_20261109_v6.worker import cases,audit,controls as predecessor_controls
from research.curved_blocks_20261109_v6 import worker as predecessor
P=Path(__file__).resolve().parent;ROOT=P.parents[1];START=time.monotonic();RAW=[]
def call(stage,args,cap):
 with tempfile.TemporaryFile()as o,tempfile.TemporaryFile()as e:
  left=min(cap,80-(time.monotonic()-START));assert left>0,'worker budget';t=time.monotonic();child=subprocess.Popen(args,stdout=o,stderr=e);timed=False
  while True:
   pid,status,usage=os.wait4(child.pid,os.WNOHANG)
   if pid:break
   if time.monotonic()-t>=left:timed=True;child.kill();pid,status,usage=os.wait4(child.pid,0);break
   time.sleep(.005)
  child.returncode=os.waitstatus_to_exitcode(status);o.seek(0);e.seek(0);row=dict(stage=stage,command=args,returncode=child.returncode,timed_out=timed,wall_seconds=time.monotonic()-t,peak_process_rss_kib=usage.ru_maxrss,stdout=o.read().decode(errors='replace'),stderr=e.read().decode(errors='replace'));RAW.append(row);assert not timed and child.returncode==0,stage+' incomplete';return row

def determinant(a):
 a=[list(map(int,row))for row in a];n=len(a);previous=1;sign=1
 for k in range(n-1):
  pivot=next((j for j in range(k,n)if a[j][k]),None)
  if pivot is None:return 0
  if pivot!=k:a[k],a[pivot]=a[pivot],a[k];sign=-sign
  value=a[k][k]
  for i in range(k+1,n):
   for j in range(k+1,n):
    q=a[i][j]*value-a[i][k]*a[k][j];assert q%previous==0;a[i][j]=q//previous
  for i in range(k+1,n):a[i][k]=0
  previous=value
 return sign*a[-1][-1]

def key(c):
 # Public context only. Width alone changes; targets/anchors/h0/coefficients do not.
 field=c['id'].split('-training-')[0]
 context=dict(field=field,p=c['p'],M=c['M'],h0=c['h0'],coefficients=c['coefficients'])
 if c['positive']:
  item=next(v['public']for v in json.load(open(ROOT/'research/dual_ladder_20261109_v3/planted_inputs.json'))['cases']if v['public']['field']==c['id'].split('-positive-')[0]);context['public_chart']=item
 else:
  stages=json.loads(gzip.decompress((ROOT/'research/dual_ladder_20261109_v3/raw-profile.json.gz').read_bytes()));rows=next(json.loads(v['stdout'])['evidence']['rows']for v in stages if v['stage']=='native-profile-0');r=next(v for v in rows if v['field']==field and v['pair']==0 and v['h_index']==0);context['public_chart']={k:r[k]for k in ['field','curve_a','curve_b','target','anchor_x','anchor_y']}
 return hashlib.sha256(json.dumps(context,sort_keys=True,separators=(',',':')).encode()).hexdigest()

def replay(inputs,out):
 assert len(out['rows'])==len(inputs)==20;checks=0;summary=[]
 for c,row in zip(inputs,out['rows']):
  assert c['id']==row['id'] and row['key']==key(c)
  checked=audit(c,row);assert row['curved_mask']==('11101111'if c['positive']else'1'*(c['H']+1)),'historical mask regression';basis=[list(map(int,z['vector']))for z in row['candidates'][:7]];det=determinant(basis);assert abs(det)==c['p']**5 and det==int(row['basis_determinant'])
  wanted=list(basis)
  for i in range(7):
   for j in range(i+1,7):
    for sg in [-1,1]:wanted.append([x+sg*y for x,y in zip(basis[i],basis[j])])
  assert wanted==[list(map(int,v['vector']))for v in row['candidates']]
  checks+=checked['candidate_u_checks']
  if row['warm_attempt']is not None:
   warm=dict(row,candidates=row['warm_attempt']['candidates'],curved_mask=row['warm_attempt']['mask']);wm=[False]*c['H'].__add__(1)
   for v in warm['candidates']:
    for i,b in enumerate(v['curved_mask']):wm[i]|=b=='1'
   warm['curved_closed']=all(wm);warm['coarse_closed']=any(v['coarse']for v in warm['candidates']);audit(c,warm);wb=[list(map(int,z['vector']))for z in warm['candidates'][:7]];assert abs(determinant(wb))==c['p']**5;wp=list(wb)
   for i in range(7):
    for j in range(i+1,7):
     for sg in [-1,1]:wp.append([x+sg*y for x,y in zip(wb[i],wb[j])])
   assert wp==[list(map(int,z['vector']))for z in warm['candidates']];checks+=49*(c['H']+1)
  summary.append(dict(id=row['id'],key=row['key'],mask=row['curved_mask'],closed=row['curved_closed'],steps=row['steps'],warm_hit=row['warm_hit'],fallback=row['fallback'],key_rejected=row['key_rejected']))
 assert checks==out['candidate_u_checks'];return summary,checks

def main():
 ident,mode=sys.argv[1:];assert mode in ['controls','bench'];evidence={};binary_hash=None
 try:
  deps=json.load(open(P/'EXTERNAL_GMP.json'));libs=[]
  for d in deps['entries']:
   assert hashlib.sha256(Path(d['resolved_path']).read_bytes()).hexdigest()==d['sha256']
   if 'link/runtime'in d['role']:libs.append(d['resolved_path'])
  with tempfile.TemporaryDirectory(prefix='metric-warm-')as td:
   td=Path(td);binary=td/'native';call('compile',['/usr/bin/g++','-O2','-std=c++17',str(P/'native.cpp'),'-o',str(binary),*libs],20);binary_hash=hashlib.sha256(binary.read_bytes()).hexdigest();linked=call('linked-libraries',['/usr/bin/ldd',str(binary)],10)
   for d in deps['entries']:
    if 'link/runtime'not in d['role']:continue
    soname=Path(d['path']).name.split('.so')[0]+'.so';hits=[z.split('=>',1)[1].split('(',1)[0].strip()for z in linked['stdout'].splitlines()if soname in z and '=>'in z];assert len(hits)==1 and Path(hits[0]).resolve()==Path(d['resolved_path'])
   if mode=='controls':
    native=json.loads(call('basis-controls',[str(binary),'basis-controls'],20)['stdout']);counts=predecessor_controls(binary);RAW.extend(predecessor.RAW);predecessor.RAW.clear()
    # Independent determinant oracle for bounded exact row operations.
    from fractions import Fraction
    n=0
    for p in [3,5,7]:
     base=[[p*int(i==j)for j in range(7)]for i in range(7)]
     for i in [5,6]:base[i][i]=1
     for shear in range(-4,5):
      a=[z[:]for z in base];a[0],a[1]=a[1],a[0];a[2]=[x+shear*y for x,y in zip(a[2],a[6])];assert abs(determinant(a))==p**5;a[0]=[2*x for x in a[0]];assert abs(determinant(a))==2*p**5;n+=1
    inputs=cases();ks=[key(c)for c in inputs];assert len(set(ks))==10
    for i in range(0,15,3):assert ks[i]==ks[i+1]==ks[i+2]
    altered=dict(inputs[0],h0=inputs[0]['h0']+1);assert key(altered)!=ks[0]
    altered=dict(inputs[0],M=inputs[0]['M']+1);assert key(altered)!=ks[0]
    evidence=dict(native=native,independent_basis_cases=n,public_context_key_checks=True,**counts)
   else:
    arms=[];whole_checks=0;all_equal=True;evidence=dict(arms=arms,complete_comparison=False,whole_work_cap=180000)
    for rep in range(3):
     order=['cold','warm']if rep%2==0 else['warm','cold'];pair={}
     for arm in order:
      began=time.monotonic();inputs=cases();lines=[' '.join(map(str,[c['id'],key(c),c['p'],c['M'],c['H'],*c['coefficients'][0],*c['coefficients'][1]]))for c in inputs];ip=td/f'{rep}-{arm}.txt';text='\n'.join(lines)+'\n';ip.write_text(text);RAW.append(dict(stage='public-input',rep=rep,arm=arm,text=text,sha256=hashlib.sha256(text.encode()).hexdigest(),solver_received_expected_roots=False));remaining=5-len(arms);cap=min(33000,180000-whole_checks-remaining*29400);assert cap>=29400,'whole work preflight';run=call(f'arm-{rep}-{arm}',[str(binary),arm,str(ip),str(cap)],70);out=json.loads(run['stdout']);rows,checks=replay(inputs,out);whole_checks+=checks;assert whole_checks<=180000;serialized_rows=json.dumps(rows,separators=(',',':'));assert serialized_rows;elapsed=time.monotonic()-began;pair[arm]=rows;arms.append(dict(rep=rep,arm=arm,paid_component_seconds=elapsed,native_seconds=run['wall_seconds'],peak_process_rss_kib=run['peak_process_rss_kib'],candidate_u_checks=checks,rows=rows))
     assert [r['mask']for r in pair['cold']]==[r['mask']for r in pair['warm']],'complete outcome mismatch'
    assert len(arms)==6
    import statistics
    cold=[a['paid_component_seconds']for a in arms if a['arm']=='cold'];warm=[a['paid_component_seconds']for a in arms if a['arm']=='warm'];evidence=dict(arms=arms,cold_median_component_seconds=statistics.median(cold),warm_median_component_seconds=statistics.median(warm),diagnostic_median_ratio=statistics.median(cold)/statistics.median(warm),candidate_u_checks=whole_checks,whole_work_cap=180000,three_fresh_native_pairs=True,all_masks_equal=True,complete_comparison=True,independent_exact_basis_and_certificate_replay=True,compilation_policy='one common compilation in wrapper; charged whole worker, excluded identically from arm ratios',timing_scope='per-arm import/public-input/native process/independent replay/complete mask-row serialization; one common Python wrapper',qualified_isolation=False,A_A_not_run=True)
  status='passed';summary='Exact metric basis controls/paired component benchmark completed; no global root or qualified baseline gain claim.'
 except (OSError,RuntimeError,AssertionError,ValueError,KeyError,StopIteration)as exc:status='inconclusive';summary='Metric component incomplete; every attempted arm retained.';evidence.update(error=str(exc),complete_comparison=False)
 blob=gzip.compress(json.dumps(RAW,separators=(',',':')).encode(),mtime=0)
 with(P/('raw-'+mode+'.json.gz')).open('xb')as f:f.write(blob)
 evidence.update(component_complete=status=='passed',compiled_binary_sha256=binary_hash,raw_sha256=hashlib.sha256(blob).hexdigest(),stage_metrics=[{k:v[k]for k in ['stage','returncode','timed_out','wall_seconds','peak_process_rss_kib']if k in v}for v in RAW],original_root_count_exact=None,complete_global_root_search=False,qualified_speedup=None,release_gate_claim=False,memory_metric='wait4 per-process peak RSS KiB, not aggregate descendants')
 result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,summary=summary,evidence=evidence,next_actions=['If metric repair plus replay/fallback does not reduce paid costs, change discovery or survivor root union; do not retry same timeout.']);text=json.dumps(result)
 if time.monotonic()-START>=84:
  result.update(status='inconclusive',summary='Final metric report exceeded acceptance deadline.');evidence.update(component_complete=False,complete_comparison=False,diagnostic_median_ratio=None)
  for arm in evidence.get('arms',[]):
   for row in arm.get('rows',[]):row['verified_mask_lower_bound']=row.get('mask');row['mask']=None;row['closed']=False
  text=json.dumps(result)
 assert time.monotonic()-START<88,'serialized report deadline';print(text)
if __name__=='__main__':main()
