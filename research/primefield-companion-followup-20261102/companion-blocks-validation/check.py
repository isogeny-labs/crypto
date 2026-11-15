from pathlib import Path
from random import Random
from datetime import datetime,timezone
import json,hashlib,sys,platform,os,subprocess
import numpy as np
D=Path(__file__).resolve().parent;ROOT=D.parent;OUT=D/'runs'
sys.path[:0]=[str(ROOT/'companion-blocks'),str(ROOT/'structured-block-kernel')]
from kernel import Meter
from operators import apply,BoundaryCache,principal,inverse_generators
now=lambda:datetime.now(timezone.utc).isoformat()
digest=lambda z:hashlib.sha256(json.dumps(z,sort_keys=True).encode()).hexdigest()

def vectors(n,seed,restricted=False):
 rng=Random(seed)
 if restricted:
  return [('basis-'+str(j),[int(i==j) for i in range(n)]) for j in range(n)] if n<=19 else [('seeded',[rng.randrange(1000000) for _ in range(n)])]
 out=[('zero',[0]*n),('ones',[1]*n),('seeded',[rng.randrange(1000000) for _ in range(n)])]
 out += [('basis-'+str(j),[int(i==j) for i in range(n)]) for j in (range(n) if n<=19 else [0,n-1])]
 return out

def dense_source(src):
 r=src['r'];z=src['constructor'];gamma=src['gamma']
 squares={j*j%r for j in range(1,r)};chi=[0 if j==0 else 1 if j in squares else r-1 for j in range(r)]
 F=[]
 for S in z['labels']:
  v=np.ones(r,dtype=np.int64)
  for d in S:v=v*np.array([chi[(i+d)%r] for i in range(r)],dtype=np.int64)%r
  F.append(v)
 F=np.array(F,dtype=np.int64).T
 assert max(r,F.shape[1])*(r-1)**2<2**63
 B=(F*np.array(z['diagonal'],dtype=np.int64))@F.T%r
 return np.array([pow(gamma,-i,r) for i in range(r)],dtype=np.int64)[:,None]*B[::-1,::-1]%r

def displacement(A,a,b,r):
 n=len(A);left=np.concatenate((a*A[-1:],A[:-1]),axis=0)
 right=np.concatenate((A[:,1:],b*A[:,:1]),axis=1)
 return (left-right)%r

def counts(m,r,n,s,queries=1):
 d=max(1,((n*(r-1)**2).bit_length()+7)//8)
 assert len(m.blocks)==2*s*queries
 for z in m.blocks:
  assert z['a_length']==z['b_length']==n and z['digit_bytes']==d
  assert 0<=z['a_bits']<=8*d*n and 0<=z['b_bits']<=8*d*n
  assert 0<=z['product_bits']<=z['a_bits']+z['b_bits']
 return dict(field_products=queries*(2*s*(n-1)+n),field_additions=queries*(s*(3*n-2)+1),field_inversions=queries,integer_multiplications=queries*2*s,input_integer_encodings=queries*4*s,packing_bytes=queries*4*s*n*d,product_bytes=queries*2*s*(2*n-1)*d,decoded_bytes=queries*2*s*(2*n-1)*d,coefficient_reductions=queries*2*s*(2*n-1),operand_bitlength_product_sum=sum(z['a_bits']*z['b_bits'] for z in m.blocks))

def reject_equal(G,K,r):
 for a in [0,1,r-1]:
  try:apply(G,K,a,a,[0]*len(G),Meter(r))
  except AssertionError:pass
  else:raise AssertionError('equal wraps accepted')
 return 3

def run_vectors(G,K,a,b,A,r,vs,arm):
 n=len(A);tests=[]
 for label,x in vs:
  x=[v%r for v in x]
  for tr in [False,True]:
   m=Meter(r);y=apply(G,K,a,b,x if arm=='candidate' else tuple(x),m,tr)
   ref=((A.T if tr else A)@np.array(x,dtype=np.int64)%r).tolist()
   expected=counts(m,r,n,len(G[0]));assert y==ref and m.counts==expected
   bad=y[:];bad[0]=(bad[0]+1)%r;assert bad!=ref
   bc=dict(m.counts);bc['field_products']+=1;assert bc!=expected
   tests.append(dict(label=label,transpose=tr,input=x,result=y,counts=m.counts,blocks=m.blocks))
 return tests

def main():
 for lock in ['source-lock.json','input-lock.json']:
  for f,h in json.loads((D/lock).read_text()).items():assert hashlib.sha256((ROOT/f).read_bytes()).hexdigest()==h,f
 plan=json.loads((D/'plan.json').read_text());assert len(plan)==369
 sources=[json.loads(x) for x in (ROOT/'direct-companion-validation/runs/cases.jsonl').read_text().splitlines()]
 sources={z['instance_id']:z for z in sources if z['arm']=='candidate'};assert len(sources)==81
 OUT.mkdir(exist_ok=False)
 (OUT/'host.json').write_text(json.dumps(dict(source_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),platform=platform.platform(),logical_cpus=os.cpu_count(),numpy=np.__version__),indent=2)+'\n')
 total=dict(verified=True,cases=0,source_cases=0,synthetic_cases=0,matvecs=0,principal_identities=0,inverse_identities=0,singular_rejections=0,equal_wrap_rejections=0,mutations_rejected=0,boundary_queries=0,aa_exact=True)
 aa={};index=[]
 with (OUT/'cases.jsonl').open('w') as f:
  for arm in ['candidate','aa']:
   for iid,p in enumerate(plan):
    start=now();r=p['r'];seed=p['seed'];tests=[];restrictions=[];inverse=None;acquisition=None;mutations=0;guards=0;singular=0
    if p['kind']=='source':
     src=sources[p['source_instance']];A=dense_source(src);G=src['constructor']['G'];K=src['constructor']['K'];a=src['gamma'];b=1;n=r
     assert np.array_equal(displacement(A,a,b,r),np.array(G,dtype=np.int64)@np.array(K,dtype=np.int64).T%r)
     tests+=run_vectors(G,K,a,b,A,r,vectors(n,seed),arm);guards+=reject_equal(G,K,r)
     bm=Meter(r);cache=BoundaryCache(G,K,a,b,bm)
     try:principal(G,K,a,b,1,1,1,cache,Meter(r))
     except AssertionError:guards+=1
     else:raise AssertionError('equal block wraps accepted')
     for cut in p['cuts']:
      for wi,(c,d) in enumerate(p['wraps']):
       cm=Meter(r);U,V=principal(G,K,a,b,cut,c,d,cache,cm)
       assert cm.counts==dict(field_additions=cut+1,field_products=cut)
       sub=A[:cut,:cut];UV=np.array(U,dtype=np.int64)@np.array(V,dtype=np.int64).T%r
       assert np.array_equal(UV,displacement(sub,c,d,r))
       bad=np.array(U,dtype=np.int64);bad[0,-1]=(bad[0,-1]+1)%r
       assert not np.array_equal(bad@np.array(V,dtype=np.int64).T%r,UV);mutations+=1
       ts=run_vectors(U,V,c,d,sub,r,vectors(cut,seed+100*cut+wi,True),arm)
       tests+=ts
       restrictions.append(dict(n=cut,a=c,b=d,U=U,V=V,correction_counts=cm.counts,tests=len(ts),boundary_mutation_rejected=True))
     expected=counts(bm,r,n,len(G[0]),10)
     expected.update(boundary_requests=64,boundary_queries=10,boundary_cache_hits=54)
     assert bm.counts==expected and cache.profile()==dict(cached_vectors=10,cached_field_slots=10*r)
     for (tr,j),v in cache.saved.items():assert v==(A[j,:] if tr else A[:,j]).tolist()
     acquisition=dict(counts=bm.counts,blocks=bm.blocks,profile=cache.profile())
     total['source_cases']+=1;total['principal_identities']+=16;total['boundary_queries']+=10
    else:
     n=p['n'];a=p['a'];b=p['b'];rng=Random(seed);typ=p['matrix']
     A=np.zeros((n,n),dtype=np.int64)
     if typ=='upper':
      for i in range(n):
       A[i,i]=1
       for j in range(i+1,n):A[i,j]=rng.randrange(r)
     elif typ=='singular':
      for i in range(n-1):A[i,i]=1
     else:assert typ=='zero'
     G=displacement(A,a,b,r).tolist();K=np.eye(n,dtype=np.int64).tolist()
     tests+=run_vectors(G,K,a,b,A,r,vectors(n,seed),arm);guards+=reject_equal(G,K,r)
     im=Meter(r)
     if typ=='upper':
      Inv=np.eye(n,dtype=np.int64)
      for i in range(n-1,-1,-1):
       for j in range(i+1,n):Inv[i]=(Inv[i]-A[i,j]*Inv[j])%r
      assert np.array_equal(A@Inv%r,np.eye(n,dtype=np.int64)) and np.array_equal(Inv@A%r,np.eye(n,dtype=np.int64))
      U,V=inverse_generators(G,K,lambda x:(Inv@np.array(x,dtype=np.int64)%r).tolist(),lambda x:(Inv.T@np.array(x,dtype=np.int64)%r).tolist(),im)
      assert im.counts==dict(inverse_oracle_calls=n,inverse_transpose_oracle_calls=n,field_additions=n*n)
      UV=np.array(U,dtype=np.int64)@np.array(V,dtype=np.int64).T%r
      assert np.array_equal(UV,displacement(Inv,b,a,r))
      bad=np.array(U,dtype=np.int64);bad[0,-1]=(bad[0,-1]+1)%r
      assert not np.array_equal(bad@np.array(V,dtype=np.int64).T%r,UV);mutations+=1
      tests+=run_vectors(U,V,b,a,Inv,r,vectors(n,seed+1),arm)
      inverse=dict(status='reference_inverse_transport_verified',U=U,V=V,counts=im.counts);total['inverse_identities']+=1
     else:
      witness=[int(i==n-1) for i in range(n)];assert not np.any(A@np.array(witness))
      def unavailable(x):raise ZeroDivisionError('singular reference; no inverse oracle')
      try:inverse_generators(G,K,unavailable,unavailable,im)
      except ZeroDivisionError:singular=1
      else:raise AssertionError('singular oracle failure suppressed')
      assert im.counts==dict(inverse_oracle_calls=1)
      inverse=dict(status='singular_oracle_rejected',witness=witness,counts=im.counts);total['singular_rejections']+=1
     if n>1:
      cut=n//2;bm=Meter(r);cache=BoundaryCache(G,K,a,b,bm);cm=Meter(r)
      U,V=principal(G,K,a,b,cut,b,a,cache,cm)
      assert cm.counts==dict(field_additions=cut+1,field_products=cut)
      sub=A[:cut,:cut];UV=np.array(U,dtype=np.int64)@np.array(V,dtype=np.int64).T%r
      assert np.array_equal(UV,displacement(sub,b,a,r))
      bad=np.array(U,dtype=np.int64);bad[0,-1]=(bad[0,-1]+1)%r
      assert not np.array_equal(bad@np.array(V,dtype=np.int64).T%r,UV);mutations+=1
      ts=run_vectors(U,V,b,a,sub,r,vectors(cut,seed+2,True),arm);tests+=ts
      restrictions.append(dict(n=cut,a=b,b=a,U=U,V=V,correction_counts=cm.counts,tests=len(ts),boundary_mutation_rejected=True))
      expected=counts(bm,r,n,len(G[0]),4)
      expected.update(boundary_requests=4,boundary_queries=4)
      assert bm.counts==expected and cache.profile()==dict(cached_vectors=4,cached_field_slots=4*n)
      for (tr,j),v in cache.saved.items():assert v==(A[j,:] if tr else A[:,j]).tolist()
      acquisition=dict(counts=bm.counts,blocks=bm.blocks,profile=cache.profile())
      total['principal_identities']+=1;total['boundary_queries']+=4
     total['synthetic_cases']+=1
    mutations+=2*len(tests)
    payload=dict(plan=p,tests=tests,restrictions=restrictions,acquisition=acquisition,inverse=inverse)
    fingerprint=digest(payload)
    if arm=='candidate':aa[iid]=fingerprint
    else:assert aa[iid]==fingerprint
    row=dict(arm=arm,instance_id=str(iid),replicate=0,seed=seed,status='verified',kind=p['kind'],matvecs=len(tests),mutations_rejected=mutations,equal_wrap_rejections=guards,aa_sha256=fingerprint,payload=payload,started_at=start,finished_at=now())
    encoded=json.dumps(row)+'\n';offset=f.tell();f.write(encoded);f.flush()
    index.append(dict(arm=arm,instance_id=str(iid),offset=offset,bytes=len(encoded.encode()),sha256=hashlib.sha256(encoded.encode()).hexdigest()))
    total['cases']+=1;total['matvecs']+=len(tests);total['mutations_rejected']+=mutations;total['equal_wrap_rejections']+=guards
    print(arm,iid,p['kind'],r,n,'verified',flush=True)
 expected=json.loads((D/'expected-summary.json').read_text());assert total==expected,(total,expected)
 (OUT/'row-index.json').write_text(json.dumps(index,indent=2)+'\n')
 (OUT/'summary.json').write_text(json.dumps(total,indent=2)+'\n');print(total)
if __name__=='__main__':main()
