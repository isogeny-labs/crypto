from pathlib import Path
from datetime import datetime,timezone
from random import Random
import json,hashlib,sys,importlib.util,subprocess,platform,os
import numpy as np
D=Path(__file__).resolve().parent;ROOT=D.parent;OUT=D/'runs'
sys.path[:0]=[str(ROOT/name) for name in ['companion-schur-fused-validation','companion-schur-fused','companion-schur-lift','companion-fused','companion-schur','companion-blocks','structured-block-kernel','companion-schur-validation']]
from kernel import Meter
from schur import LeadingBlockObstruction
from audit import AuditedSchur,check_ledger,check_generated
from lift_checks import fixture as lift_fixture,obstructions as lift_obstructions
from multiply import apply
from schur_reference import inverse_or_null,oracles
spec=importlib.util.spec_from_file_location('block_reference',ROOT/'companion-blocks-validation/check.py');ref=importlib.util.module_from_spec(spec);spec.loader.exec_module(ref)
now=lambda:datetime.now(timezone.utc).isoformat()
digest=lambda z:hashlib.sha256(json.dumps(z,sort_keys=True).encode()).hexdigest()

def synthetic(p):
 r=p['r'];N=p['N'];k=p['cut'];m=N-k;rng=Random(p['seed']);typ=p['matrix']
 if typ=='swap_obstruction':
  A=np.eye(N,dtype=np.int64);A[[0,k]]=A[[k,0]];assert np.array_equal(A@A%r,np.eye(N,dtype=np.int64));return A
 if typ=='zero_matrix':return np.zeros((N,N),dtype=np.int64)
 E=np.eye(k,dtype=np.int64)
 for i in range(k):
  for j in range(i+1,k):E[i,j]=rng.randrange(r)
 Inv,_=inverse_or_null(E,r)
 B=np.array([[rng.randrange(r) for _ in range(m)] for _ in range(k)],dtype=np.int64)
 C=np.array([[rng.randrange(r) for _ in range(k)] for _ in range(m)],dtype=np.int64)
 T=np.zeros((m,m),dtype=np.int64)
 if typ=='regular':
  for i in range(m):
   T[i,i]=1
   for j in range(i+1,m):T[i,j]=rng.randrange(r)
 elif typ=='singular_schur':
  for i in range(m-1):T[i,i]=1
 else:assert typ=='zero_schur'
 return np.block([[E,B],[C,(T+(C@Inv%r)@B)%r]])

def require_obstruction(fn):
 try:fn()
 except LeadingBlockObstruction as e:assert 'undetermined' in str(e)
 else:raise AssertionError('leading obstruction lost')

def main():
 upstream=ROOT/'companion-fused-validation'
 for name in ['exit-code.txt','analysis-exit.txt','export-check-exit.txt','postprocessor-exit.txt']:
  assert (upstream/name).read_text().strip()=='0','Upstream kernel validation must finish successfully before integration'
 upstream_summary=json.loads((upstream/'runs/recomputed-summary.json').read_text())
 assert upstream_summary['cases']==1698 and upstream_summary['products']==34248 and upstream_summary['aa_exact']
 for lock in ['source-lock.json','input-lock.json']:
  for f,h in json.loads((D/lock).read_text()).items():assert hashlib.sha256((ROOT/f).read_bytes()).hexdigest()==h,f
 plan=json.loads((D/'plan.json').read_text());assert len(plan)==321
 sources={z['instance_id']:z for z in map(json.loads,(ROOT/'direct-companion-validation/runs/cases.jsonl').read_text().splitlines()) if z['arm']=='candidate'}
 OUT.mkdir(exist_ok=False)
 (OUT/'host.json').write_text(json.dumps(dict(source_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),platform=platform.platform(),logical_cpus=os.cpu_count(),numpy=np.__version__),indent=2)+'\n')
 total=dict(verified=True,cases=0,source_cases=0,synthetic_cases=0,valid_schur_cases=0,obstruction_cases=0,synthetic_valid=0,synthetic_obstructions=0,full_invertible_obstructions=0,directional_checks=0,mutations_rejected=0,equal_wrap_rejections=0,lift_fixtures=0,normalized_witnesses=0,lift_obstructions=0,lift_mutations_rejected=0,aa_exact=True)
 index=[];aa={}
 with (OUT/'cases.jsonl').open('w') as f:
  for arm in ['candidate','aa']:
   for iid,p in enumerate(plan):
    started=now();r=p['r'];k=p['cut']
    if p['kind']=='source':
     src=sources[p['source_instance']];A=ref.dense_source(src);G=src['constructor']['G'];K=src['constructor']['K'];a=src['gamma'];b=1
    else:
     A=synthetic(p);a=p['a'];b=p['b'];G=ref.displacement(A,a,b,r).tolist();K=np.eye(len(A),dtype=np.int64).tolist()
    N=len(A);m=N-k;s=len(G[0]);c,d=p['new_wraps'];Inv,w=inverse_or_null(A[:k,:k],r)
    assert np.array_equal(ref.displacement(A,a,b,r),np.array(G,dtype=np.int64)@np.array(K,dtype=np.int64).T%r)
    solve,solveT,stats=oracles(Inv,r);meter=Meter(r);obj=AuditedSchur(G,K,a,b,k,solve,solveT,meter,dense_reference=A.tolist())
    try:obj.generators(1,1)
    except AssertionError:pass
    else:raise AssertionError('equal wraps accepted')
    total['equal_wrap_rejections']+=1
    tests=[];mutations=0;generation=None
    if Inv is None:
     require_obstruction(lambda:obj.generators(c,d));assert meter.counts==dict(leading_forward_calls=1)
     assert stats==dict(forward_calls=1,transpose_calls=0,dense_terms=0,coefficient_reductions=0)
     solve,solveT,st=oracles(None,r);mm=Meter(r);obj2=AuditedSchur(G,K,a,b,k,solve,solveT,mm,dense_reference=A.tolist())
     require_obstruction(lambda:obj2.apply([0]*m))
     expected=check_ledger(obj2,1,0,dict(schur_forward_calls=1,leading_forward_calls=1))
     assert mm.counts==expected and st['forward_calls']==1 and st['dense_terms']==0
     assert any(w) and not np.any(A[:k,:k]@np.array(w,dtype=np.int64)%r)
     assert not any([0]*k);mutations=1
     generation=dict(status='leading_block_obstruction',null_witness=w,generator_attempt_counts=meter.counts,generator_reference_calls=stats,application_attempt_counts=mm.counts,application_reference_calls=st,blocks=mm.blocks,full_matrix_invertible=p.get('matrix')=='swap_obstruction',source_receipts=obj2.source_receipts)
     total['obstruction_cases']+=1
     if p['kind']=='synthetic':total['synthetic_obstructions']+=1
     if p.get('matrix')=='swap_obstruction':total['full_invertible_obstructions']+=1
    else:
     S=(A[k:,k:]-(A[k:,:k]@Inv%r)@A[:k,k:])%r
     if p.get('matrix')=='zero_schur':assert not np.any(S)
     if p.get('matrix')=='singular_schur':assert not np.any(S@np.array([0]*(m-1)+[1],dtype=np.int64))
     if p.get('matrix')=='regular':assert inverse_or_null(S,r)[0] is not None
     GS,KS=obj.generators(c,d);assert len(GS)==len(KS)==m and len(GS[0])==len(KS[0])==s+2
     H=ref.displacement(S,c,d,r);assert np.array_equal(np.array(GS,dtype=np.int64)@np.array(KS,dtype=np.int64).T%r,H)
     expected=check_ledger(obj,s+3,s+3,dict(field_products=m,field_additions=m*(2*s+5),leading_forward_calls=s+2,leading_transpose_calls=s+2,schur_forward_calls=1,schur_transpose_calls=1))
     assert meter.counts==expected
     assert stats==dict(forward_calls=s+2,transpose_calls=s+2,dense_terms=2*(s+2)*k*k,coefficient_reductions=2*(s+2)*k)
     extra=np.zeros((m,1),dtype=np.int64);extra[0,0]=1
     badG=np.concatenate((np.array(GS,dtype=np.int64),extra),axis=1);badK=np.concatenate((np.array(KS,dtype=np.int64),extra),axis=1)
     assert not np.array_equal(badG@badK.T%r,H)
     bad=dict(meter.counts);bad['source_forward_calls']+=1;assert bad!=expected;mutations=2
     generation=dict(status='schur_verified',GS=GS,KS=KS,counts=meter.counts,reference_oracle_work=stats,blocks=meter.blocks,source_receipts=obj.source_receipts)
     for label,x in ref.vectors(m,p['seed']+991):
      x=[v%r for v in x]
      for tr in [False,True]:
       fm=Meter(r);fs,ft,st=oracles(Inv,r);query=AuditedSchur(G,K,a,b,k,fs,ft,fm,dense_reference=A.tolist())
       y=query.apply(x if arm=='candidate' else tuple(x),tr)
       expected=check_ledger(query,0 if tr else 2,2 if tr else 0,{'field_additions':m,('schur_transpose_calls' if tr else 'schur_forward_calls'):1,('leading_transpose_calls' if tr else 'leading_forward_calls'):1})
       want=((S.T if tr else S)@np.array(x,dtype=np.int64)%r).tolist()
       assert y==want and fm.counts==expected
       assert st==dict(forward_calls=int(not tr),transpose_calls=int(tr),dense_terms=k*k,coefficient_reductions=k)
       gm=Meter(r);gy=apply(GS,KS,c,d,x if arm=='candidate' else tuple(x),gm,tr);ge=check_generated(gm,r,m,s+2,c,d,tr)
       assert gy==want and gm.counts==ge
       for out,counts,ex in [(y,fm.counts,expected),(gy,gm.counts,ge)]:
        bad=out[:];bad[0]=(bad[0]+1)%r;assert bad!=want
        bc=dict(counts);bc['field_products']+=1;assert bc!=ex;mutations+=2
       tests.append(dict(label=label,transpose=tr,input=x,result=y,generator_result=gy,operator_counts=fm.counts,operator_blocks=fm.blocks,reference_oracle_work=st,generator_counts=gm.counts,generator_blocks=gm.blocks,source_receipts=query.source_receipts))
     total['valid_schur_cases']+=1
     if p['kind']=='synthetic':total['synthetic_valid']+=1
    if Inv is None:
     lifts=lift_obstructions(A.tolist(),G,K,a,b,k,r,arm)
     total['lift_obstructions']+=len(lifts)
    else:
     lifts=lift_fixture(A.tolist(),S.tolist(),G,K,a,b,k,Inv,r,p['seed']+2003,arm)
     total['lift_fixtures']+=len(lifts)
     total['normalized_witnesses']+=sum(z['label']=='normalized' for z in lifts)
     total['lift_mutations_rejected']+=sum(z['mutations_rejected'] for z in lifts)
    payload=dict(plan=p,generation=generation,tests=tests,lifts=lifts);fingerprint=digest(payload)
    if arm=='candidate':aa[iid]=fingerprint
    else:assert aa[iid]==fingerprint
    row=dict(arm=arm,instance_id=str(iid),replicate=0,seed=p['seed'],kind=p['kind'],status='verified',outcome=generation['status'],directional_checks=len(tests),mutations_rejected=mutations,aa_sha256=fingerprint,payload=payload,started_at=started,finished_at=now())
    encoded=json.dumps(row)+'\n';offset=f.tell();f.write(encoded);f.flush()
    index.append(dict(arm=arm,instance_id=str(iid),offset=offset,bytes=len(encoded.encode()),sha256=hashlib.sha256(encoded.encode()).hexdigest()))
    total['cases']+=1;total[p['kind']+'_cases']+=1;total['directional_checks']+=len(tests);total['mutations_rejected']+=mutations
    print(arm,iid,p['kind'],r,k,generation['status'],flush=True)
 assert total['cases']==642 and total['source_cases']==162 and total['synthetic_cases']==480
 assert total['valid_schur_cases']+total['obstruction_cases']==642
 assert total['synthetic_valid']==288 and total['synthetic_obstructions']==192 and total['full_invertible_obstructions']==96 and total['equal_wrap_rejections']==642
 (OUT/'row-index.json').write_text(json.dumps(index,indent=2)+'\n');(OUT/'summary.json').write_text(json.dumps(total,indent=2)+'\n');print(total)
if __name__=='__main__':main()
