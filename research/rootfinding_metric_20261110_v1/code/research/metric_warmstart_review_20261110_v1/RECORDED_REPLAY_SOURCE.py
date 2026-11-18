from pathlib import Path
import json,gzip,hashlib,statistics,datetime
r=Path('/workspace/generalized_roots');a=r/'research/metric_warmstart_20261110_v1';d=r/'research/metric_warmstart_review_20261110_v1'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
sp=json.loads((d/'SOURCE_PINS.json').read_text())['sha256']
for p,h in sp.items():assert sha(Path(p)if Path(p).is_absolute()else r/p)==h,p
bp=r/'lab/ledger/attempts/metric-bench-20261110-v1/0001';br=json.loads((bp/'scientific_result.json').read_text());ev=br['evidence'];assert br['status']=='passed'and ev['complete_comparison']and len(ev['arms'])==6
rawfile=a/'raw-bench.json.gz';assert sha(rawfile)==ev['raw_sha256'];raw=json.loads(gzip.decompress(rawfile.read_bytes()));exports={(x['rep'],x['arm']):x for x in raw if x['stage']=='public-input'};arms=[x for x in raw if x['stage'].startswith('arm-')];assert len(arms)==6
checks=bases=positive=hits=fallbacks=0;allmasks={};peaks=[]
def det(z):
 z=[v[:]for v in z];pr=1;sign=1
 for k in range(6):
  j=next((i for i in range(k,7)if z[i][k]),None)
  if j is None:return 0
  if j!=k:z[j],z[k]=z[k],z[j];sign=-sign
  piv=z[k][k]
  for i in range(k+1,7):
   for j in range(k+1,7):q=z[i][j]*piv-z[i][k]*z[k][j];assert q%pr==0;z[i][j]=q//pr
  for i in range(k+1,7):z[i][k]=0
  pr=piv
 return sign*z[6][6]
def pool(pool,p,M,H,aa,bb):
 global checks,bases
 vs=[list(map(int,v['vector']))for v in pool];assert len(vs)==49 and abs(det(vs[:7]))==p**5;bases+=1;want=vs[:7]
 for i in range(7):
  for j in range(i+1,7):
   for sg in [-1,1]:want.append([x+sg*y for x,y in zip(vs[i],vs[j])])
 assert want==vs
 A=3*M;lo=[0,-A*H,0,0,0,0,0];hi=[max(A,H*H)**2,H**3,H*H,A,H,3*M*M,M**3];mask=[False]*(H+1)
 for v,row in zip(vs,pool):
  assert all((v[i]-v[5]*aa[i]-v[6]*bb[i])%p==0 for i in range(5));cc=(v[5]*aa[5]+v[6]*bb[5])%p
  if 2*cc>p:cc-=p
  assert cc==int(row['constant']);LL=RR=cc
  for i in range(7):
   z=v[i]if i<5 else-v[i];LL+=min(z*lo[i],z*hi[i]);RR+=max(z*lo[i],z*hi[i])
  assert (LL,RR)==(int(row['L']),int(row['R']));wm=[];widthsum=0;maxwidth=0;tighter=0
  for u in range(H+1):
   ac=v[0];bc=-2*ac*u*u-v[1]*u+v[3];co=ac*u**4+v[1]*u**3+v[2]*u*u+v[4]*u+cc;xs=[0,A]
   # Independent discrete-difference turning point; no author extrema helper.
   if ac:xs.append(max(0,min(A,-((ac+bc)//(2*ac)))))
   vals=[ac*x*x+bc*x+co for x in xs];L=min(vals);R=max(vals)
   for z in [-v[5]*3*M*M,-v[6]*M**3]:L+=min(0,z);R+=max(0,z)
   ok=-((-L)//p)>R//p;wm.append(ok);mask[u]|=ok;w=R-L;widthsum+=w;maxwidth=max(maxwidth,w);tighter+=w<RR-LL;checks+=1
  assert ''.join('1'if x else'0'for x in wm)==row['curved_mask'];assert (widthsum,maxwidth,tighter)==(int(row['curved_width_sum']),int(row['curved_max_width']),row['tighter_u'])
 return ''.join('1'if x else'0'for x in mask)
for st in arms:
 _,rep,mode=st['stage'].split('-');rep=int(rep);ex=exports[rep,mode];assert hashlib.sha256(ex['text'].encode()).hexdigest()==ex['sha256'];inputs=[v.split()for v in ex['text'].splitlines()];out=json.loads(st['stdout']);assert len(inputs)==len(out['rows'])==20;peaks.append(st['peak_process_rss_kib']);local=0
 for vals,row in zip(inputs,out['rows']):
  assert len(vals)==17;ident,key=vals[:2];p,M,H=map(int,vals[2:5]);aa=list(map(int,vals[5:11]));bb=list(map(int,vals[11:17]));assert ident==row['id']and key==row['key'];mask=pool(row['candidates'],p,M,H,aa,bb);local+=49*(H+1);assert mask==row['curved_mask'];assert int(row['basis_determinant'])==det([list(map(int,v['vector']))for v in row['candidates'][:7]])
  pos='-positive-'in ident;assert mask==('11101111'if pos else'1'*(H+1));positive+=pos;hits+=row['warm_hit'];fallbacks+=row['fallback'];assert not(pos and row['warm_hit']);allmasks.setdefault(rep,{})[mode]=allmasks.setdefault(rep,{}).get(mode,[])+[mask]
  if row['warm_attempt']is not None:pool(row['warm_attempt']['candidates'],p,M,H,aa,bb);local+=49*(H+1)
 assert out['candidate_u_checks']==local
for pair in allmasks.values():assert pair['cold']==pair['warm']
assert checks==ev['candidate_u_checks']==176400 and fallbacks==0 and bases==120 and positive==30 and hits==30
cold=[x['paid_component_seconds']for x in ev['arms']if x['arm']=='cold'];warm=[x['paid_component_seconds']for x in ev['arms']if x['arm']=='warm'];ratio=statistics.median(cold)/statistics.median(warm);assert ratio==ev['diagnostic_median_ratio']
audit=dict(recorded_evidence_only=True,search_or_scientific_rerun=False,all_frozen_pins_match=True,arms=6,case_bases=bases,candidate_vectors=5880,candidate_u_checks=checks,positive_cases=positive,warm_hits=hits,fallbacks=fallbacks,all_masks_exact=True,cold_seconds=cold,warm_seconds=warm,median_ratio=ratio,native_peak_rss_kib=peaks,raw_sha256=sha(rawfile),scope='amortized component; common compile/wrapper not arm ratio; no A/A, isolation or genuine GB baseline')
with(d/'RECORDED_EVIDENCE_AUDIT.json').open('x')as f:json.dump(audit,f,indent=2);f.write('\n')
with(d/'RESULTS_REVIEW.md').open('x')as f:f.write('# Recorded results review\n\nControls passed their declared bounded cases:36 native basis/metric cases,27 Python determinant/sign/shear/corruption cases and public-key separation, plus inherited literal/extrema/characteristic-three and signed-bound controls. No dedicated sequential-decreasing-metric test is inferred.\n\nBench passed all6 arms/3 alternating fresh-native pairs. Recorded-data replay independently checked120 exact basis determinants,5880 candidate vectors and176400 per-u ranges using a discrete-difference turning-point formula, all49pool identities and all masks. Thirty positive cases preserve11101111; warm hits30, fallback0. Cold and warm outputs agree for every case and repetition. All source/input pins and raw/result identities remain unchanged. This review ran no search or experiment.\n\nCold paid component seconds: '+repr(cold)+'. Warm: '+repr(warm)+'. Median ratio '+repr(ratio)+'. Native RSS KiB: '+repr(peaks)+'. The common compile/wrapper are charged only to whole worker and excluded equally from arm ratios; no A/A/qualified isolation or genuine F4/F5/F6 comparison exists. This is an observed amortized-component gain on the fixed training panel, not a complete rootfinder or qualified release ratio. The exact historical mask contract passed without fallback, but unexercised fallback paths are not measurements.\n\nNext: complete survivor-root-set unions on disjoint masks, or a separately frozen stronger common-output timing comparison with full overhead and missing controls. Preserve requested field/global coverage and immutable gate.\n')
files=[d/'RECORDED_EVIDENCE_AUDIT.json',d/'RESULTS_REVIEW.md',rawfile,bp/'scientific_result.json',bp/'process.json',r/'lab/ledger/attempts/metric-controls-20261110-v1/0001/scientific_result.json']
with(d/'RESULT_PINS.json').open('x')as f:json.dump({'created_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sha256':{str(p.relative_to(r)):sha(p)for p in files}},f,indent=2);f.write('\n')
print(json.dumps(audit))
