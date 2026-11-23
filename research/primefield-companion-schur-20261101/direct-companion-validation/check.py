from pathlib import Path
from random import Random
from math import comb
from datetime import datetime,timezone
import json,hashlib,importlib.util,sys,subprocess,platform,os,copy
import numpy as np
D=Path(__file__).resolve().parent;R=D.parent;O=D/'runs'
sys.path.insert(0,str(R/'structured-block-kernel'))
from kernel import Meter
def imp(n,p):
 s=importlib.util.spec_from_file_location(n,p);m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
c=imp('direct_companion',R/'direct-cyclic-companion/generators.py')
old=imp('old_boundary',R/'structured-block-kernel/features.py')
now=lambda:datetime.now(timezone.utc).isoformat()
digest=lambda x:hashlib.sha256(json.dumps(x,sort_keys=True).encode()).hexdigest()
def contexts():
 small=[(7,6),(13,12),(19,15),(43,15),(73,15),(349,15)]
 large=[(r,q) for r,qs in [(359,[13,14,15]),(479,[15,16,17]),(683,[16,17,18])] for q in qs]
 return small+large+[(r,1) for r in [7,13,19,43,73,349,359,479,683]]+[(7,3),(13,4),(19,4)]
def pc(n):return bin(n).count('1')+n.bit_length()-1
def verify(z,counts,ref,expected):
 assert z==ref and counts==expected
def main():
 for f,h in json.loads((D/'source-lock.json').read_text()).items():assert hashlib.sha256((R/f).read_bytes()).hexdigest()==h,f
 O.mkdir(exist_ok=False)
 (O/'host.json').write_text(json.dumps(dict(source_commit=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),platform=platform.platform(),logical_cpus=os.cpu_count(),numpy=np.__version__),indent=2)+'\n')
 public={}
 for r,q,path in [(479,15,'packed-lift-recovery/runs/r479-candidate-q15-construction.json'),(13,4,'packed-lift-preflight/runs/candidate-r13-q4-construction.json')]:
  public[r,q]=json.loads((R/path).read_text())['certificate']['dual']
 rows=[];cache={};aa={}
 with (O/'cases.jsonl').open('w') as out:
  for arm in ['candidate','aa']:
   for ci,(r,q) in enumerate(contexts()):
    if r not in cache:
     cache[r]=np.array([[(-1)**(i-j)*comb(i,j)%r if j<=i else 0 for j in range(r)] for i in range(r)],dtype=np.int64)
    squares={k*k%r for k in range(1,r)};chi=np.array([0 if k==0 else 1 if k in squares else r-1 for k in range(r)],dtype=np.int64)
    patterns=[(0,)]+[(0,a) for a in range(1,q)]+[(0,a,b) for a in range(1,q) for b in range(a+1,q)]
    def column(S):
     v=np.ones(r,dtype=np.int64)
     for d in S:v=v*chi[(np.arange(r)+d)%r]%r
     return v
    labels=[()]+[tuple(a+d for d in P) for P in patterns for a in range(q-max(P))]
    features=np.array([column(S) for S in labels]).T
    V=np.array([np.ones(r,dtype=np.int64)]+[column(tuple(a+d for d in P)) for P in patterns for a in [0,q-max(P)]]).T
    n=len(labels);s=V.shape[1];assert max(n,s,r)*(r-1)**2<2**63
    mm=lambda a,b:(a@b)%r
    gammas=[2,r-1,Random(20261101+r*100+q).randrange(2,r)]
    for gi,gamma in enumerate(gammas):
     started=now();rng=Random(20261101+100000*r+100*q+gi);weights=[rng.randrange(1,r) for _ in range(len(patterns)+1)]
     m=Meter(r);z=c.build(r,q,gamma,weights if arm=='candidate' else tuple(weights),m)
     diagonal=[weights[0]]+[w*pow(gamma,a,r)%r for P,w in zip(patterns,weights[1:]) for a in range(q-max(P))]
     coeff=[(1-gamma)*weights[0]%r]+[v for P,w in zip(patterns,weights[1:]) for v in [w,-w*pow(gamma,q-max(P),r)%r]]
     scales=np.array([pow(gamma,-i,r) for i in range(r)],dtype=np.int64)
     B=mm(features*np.array(diagonal,dtype=np.int64),features.T)
     H=mm(V*np.array(coeff,dtype=np.int64),V.T)
     cyclic=(np.arange(r)+1)%r
     assert np.array_equal((B-gamma*B[cyclic][:,cyclic])%r,H)
     A=scales[:,None]*B[::-1,::-1]%r
     G=-scales[:,None]*V[::-1]%r*np.array(coeff,dtype=np.int64)%r
     K=V[(r-2-np.arange(r))%r]
     rhs=scales*np.arange(r-1,-1,-1)%r
     ref=dict(left_wrap=gamma,right_wrap=1,G=G.tolist(),K=K.tolist(),rhs=rhs.tolist(),inverse_diagonal=scales.tolist(),labels=labels,diagonal=diagonal,generator_width=s)
     expected=dict(field_products=r*pc((r-1)//2)+2*r*sum(map(len,patterns))+sum(pc(q-max(P)) for P in patterns)+n-1+2*r+2*r*s,field_additions=1+len(patterns)+r*s,field_inversions=1)
     verify(z,m.counts,ref,expected)
     left=np.concatenate((gamma*A[-1:],A[:-1]),axis=0)%r
     right=np.concatenate((A[:,1:],A[:,:1]),axis=1)
     assert np.array_equal((left-right)%r,mm(G,K.T))
     if r<=19:
      Z=np.zeros((r,r),dtype=np.int64)
      for i in range(1,r):Z[i,i-1]=1
      Z[0,-1]=1;ZG=Z.copy();ZG[0,-1]=gamma
      assert np.array_equal((mm(ZG,A)-mm(A,Z))%r,mm(G,K.T))
     assert pow(gamma,r,r)==gamma and (1-gamma)%r!=0
     om=Meter(r);prior=old.build(r,q,gamma,weights,om)
     assert prior['labels']==labels and prior['diagonal']==diagonal
     PV=mm(cache[r],V)
     assert PV.tolist()==prior['V']
     U=PV*np.array(coeff,dtype=np.int64)%r
     for i in range(r-2,-1,-1):U[i]=(U[i]-U[i+1])%r
     assert U.tolist()==prior['U']
     x=np.array([rng.randrange(r) for _ in range(r)],dtype=np.int64);b=mm(B,x);y=x[::-1]
     assert np.array_equal(mm(A,y),scales*b[::-1]%r)
     v=np.array([rng.randrange(r) for _ in range(r)],dtype=np.int64);w=(v*scales)[::-1]%r
     assert np.array_equal(mm(v,A),mm(w,B)[::-1])
     assert int(mm(v,rhs))==int(mm(w,np.arange(r,dtype=np.int64)))
     witness=public.get((r,q))
     if q==1:
      j=next(j for j in range(2,r-1) if j not in squares);t=pow(1+j,-1,r)
      witness=[0]*r;witness[0]=-2*t%r;witness[1]=t;witness[j]=t
     normalized=False
     if witness is not None:
      w=np.array(witness,dtype=np.int64);v=w[::-1]*np.array([pow(gamma,i,r) for i in range(r)],dtype=np.int64)%r
      assert not np.any(mm(w,B)) and int(mm(w,np.arange(r,dtype=np.int64)))==1
      assert not np.any(mm(v,A)) and int(mm(v,rhs))==1
      assert int(mm(np.zeros(r,dtype=np.int64),rhs))!=1;normalized=True
     mutations=0
     for mode in ['G','K','left_wrap','scale','rhs','field_count']:
      bad=copy.deepcopy(z);bc=dict(m.counts)
      if mode in ('G','K'):bad[mode][0][0]=(bad[mode][0][0]+1)%r
      if mode=='left_wrap':bad['left_wrap']=1
      if mode=='scale':bad['inverse_diagonal'][0]=0
      if mode=='rhs':bad['rhs'][0]=(bad['rhs'][0]+1)%r
      if mode=='field_count':bc['field_products']+=1
      try:verify(bad,bc,ref,expected)
      except AssertionError:mutations+=1
      else:raise AssertionError(('accepted mutation',mode))
     invalid=0
     for badgamma in [0,1]:
      try:c.build(r,q,badgamma,weights,Meter(r))
      except AssertionError:invalid+=1
      else:raise AssertionError('invalid gamma accepted')
     key=(ci,gi);fingerprint=digest([z,m.counts])
     if arm=='candidate':aa[key]=fingerprint
     else:assert aa[key]==fingerprint
     row=dict(arm=arm,instance_id=str(3*ci+gi),replicate=0,seed=20261101+100000*r+100*q+gi,r=r,q=q,gamma=gamma,status='verified',counts=m.counts,matrix_entries_checked=r*r,normalized_witness=normalized,mutations_rejected=mutations+int(normalized),invalid_gamma_rejections=invalid,aa_sha256=fingerprint,constructor=z,started_at=started,finished_at=now())
     out.write(json.dumps(row)+'\n');out.flush();rows.append(row)
    print(arm,r,q,'verified',flush=True)
 summary=dict(verified=True,cases=len(rows),contexts=len(contexts()),normalized_witness_cases=sum(z['normalized_witness'] for z in rows),mutations_rejected=sum(z['mutations_rejected'] for z in rows),invalid_gamma_rejections=sum(z['invalid_gamma_rejections'] for z in rows),aa_exact=True)
 (O/'summary.json').write_text(json.dumps(summary,indent=2)+'\n');print(summary)
if __name__=='__main__':main()
