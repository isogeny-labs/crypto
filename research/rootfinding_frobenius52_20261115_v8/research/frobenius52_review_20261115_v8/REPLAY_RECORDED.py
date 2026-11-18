"""Verify listed recorded candidates only; never search for new k or roots."""
from pathlib import Path
import sys,json,hashlib,time,gzip,itertools
START=time.monotonic()
profile=Path(sys.argv[1]);fixture=Path(sys.argv[2]);x=json.loads(gzip.decompress(profile.read_bytes()));data=json.loads(fixture.read_text())
n=data['extension_field']['degree'];mod=int(data['extension_field']['modulus_hex'],16);q=1<<n
assert n==83 and mod==0x800000000200000000007

def fm(a,b):
 assert 0<=a<q and 0<=b<q
 p=0
 while b:
  if b&1:p^=a
  a<<=1;b>>=1
 while p.bit_length()>n:p^=mod<<(p.bit_length()-n-1)
 return p

def fs(a):return fm(a,a)
def fp(a,e):
 r=1
 while e:
  if e&1:r=fm(r,a)
  a=fs(a);e>>=1
 return r

def trim(p):
 p=list(p)
 while p and p[-1]==0:p.pop()
 return p

def pa(a,b):return trim([(a[i]if i<len(a)else 0)^(b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))])
def pm(a,b):
 out=[0]*max(0,len(a)+len(b)-1)
 for i,c in enumerate(a):
  for j,d in enumerate(b):out[i+j]^=fm(c,d)
 return trim(out)

def pd(a,b):
 a=trim(a);b=trim(b);assert b and b[-1]==1
 out=[0]*max(0,len(a)-len(b)+1)
 while len(a)>=len(b):
  j=len(a)-len(b);c=a[-1];out[j]=c
  for i,d in enumerate(b):a[i+j]^=fm(c,d)
  a=trim(a)
 return trim(out),a

def sm(a,Q):return pd(pm(a,a),Q)[1]
def fx(vs):
 x=0
 for v in vs:x^=v
 return x

A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
assert x['error']is None
p=x['out']['profile'];assert p['complete_comparison'] and len(p['records'])==6

def rank(vectors):
 piv={}
 for v in vectors:
  while v:
   bit=v.bit_length()-1
   if bit not in piv:piv[bit]=v;break
   v^=piv[bit]
 return len(piv)

def fq(c,k):return [fm(c['alphaU'],fs(k))^fm(c['betaU'],k)^c['gammaU'],fm(c['alphaT'],fs(k))^fm(c['betaT'],k)^c['gammaT'],c['b'],1]
def scalar(a,ya):
 a2=fs(a);z2=fs(z);cc=a^z;ia=fp(a,q-2);ic2=fp(fs(cc),q-2)
 s0=fm(1^h,a2);s1=fm(ya,ia)^a;r0=fm(fm(fm(h,a),ya)^fm(a2,a)^fm(z2,z)^fm(s0,cc)^fm(fm(h,z),y),ic2);r1=fm(ya^fm(s1,cc)^y,ic2);b=fs(h)^h^z
 at=fs(r1);bt=1^fm(h,r1);au=1^r1^fm(z,at);bu=r0^fm(h,s1)^fm(z,bt);gt=fs(r0)^fm(h,r0)^fm(fs(h),A)^fm(z,b)^a2;gu=fm(h,s0)^fm(z,gt^a2)^fm(a2,b)
 return dict(alphaT=at,betaT=bt,gammaT=gt,alphaU=au,betaU=bu,gammaU=gu,Delta=fm(au,bt)^fm(at,bu),b=b,r0=r0,r1=r1,s0=s0,s1=s1)
L=[1]
for v in [1<<i for i in range(18)]:
 lam=0;power=v
 for c in L:lam^=fm(c,power);power=fs(power)
 assert lam;nxt=[0]*(len(L)+1)
 for i,c in enumerate(L):nxt[i]^=fm(lam,c);nxt[i+1]^=fs(c)
 L=nxt
families={};proofs=[]
for method,reps in p['representations'].items():
 assert len(reps)==4
 for ci,rec in enumerate(reps):
  a=rec['cell']['a'];ya=rec['cell']['ya'];c=scalar(a,ya);assert c==rec['coefficients'] and c['betaU']==0
  f=rec['family'];dirs=f['k_directions'];k0=f['k0'];nv=len(dirs);assert rank(dirs)==nv
  if f['branch']=='U_coordinates':
   assert nv==52 and c['alphaU'] and fq(c,k0)[0]==0
   assert [fm(c['alphaU'],fs(d))for d in dirs]==[1<<i for i in range(52)]
   positions=list(range(35,83));pack=lambda Q:Q[1]
  else:
   assert nv==83 and dirs==[1<<i for i in range(83)] and k0==0
   positions=list(range(35,83))+list(range(83+52,166));pack=lambda Q:Q[1]|(Q[0]<<83)
  base=fq(c,k0);columns=[[u^v for u,v in zip(fq(c,k0^d),base)]for d in dirs];rhs=pack(base);packed=[pack(v)for v in columns]
  rows=[(sum(((v>>bit)&1)<<i for i,v in enumerate(packed)),(rhs>>bit)&1)for bit in positions];sol=f['coordinate_family'];assert sol['consistent']and sol['rank']==rank([m for m,r in rows]) and sol['nullity']==nv-sol['rank']
  assert len(sol['basis'])==rank(sol['basis'])==sol['nullity']
  assert all((m&sol['particular']).bit_count()%2==r for m,r in rows) and all(all((m&v).bit_count()%2==0 for m,r in rows)for v in sol['basis'])
  lift=lambda bits:fx(d for i,d in enumerate(dirs)if bits>>i&1)
  assert f['k_particular']==k0^lift(sol['particular']) and f['k_basis']==[lift(v)for v in sol['basis']]
  assert rank(f['k_basis'])==len(f['k_basis'])<=12
  values={f['k_particular']^fx(v for i,v in enumerate(f['k_basis'])if t>>i&1)for t in range(1<<len(f['k_basis']))}
  families[method,ci]=(rec,values);proofs.append(dict(method=method,cell=ci,anchor=a,variables=nv,rank=sol['rank'],nullity=sol['nullity'],candidates=len(values)))
for ci in range(4):assert families['k83',ci][1]==families['U52',ci][1]
checked=set();cubics={};all_receipts=0;rejected=0;supported=0
for record in p['records']:
 assert record['complete_batch']and len(record['cells'])==4
 for ci,alias in enumerate(record['cells']):
  rec,values=families[record['method'],ci];assert rec['cell']==alias['cell'];items=p['receipts'][alias['receipt_alias']]
  assert hashlib.sha256(json.dumps(items,sort_keys=True,separators=(',',':')).encode()).hexdigest()==alias['receipt_alias']
  assert {c['k']for c in items}==values and len(items)==len(values)
  a=rec['cell']['a'];ya=rec['cell']['ya'];c=rec['coefficients']
  for item in items:
   all_receipts+=1;key=(a,ya,item['k'],tuple(item['Q']),tuple(item['L_remainder']),tuple(item['fourth_power_residue']))
   if key in checked:continue
   checked.add(key);k=item['k'];Q=item['Q'];assert Q==fq(c,k)
   r=c['r0']^fm(c['r1'],k);s=c['s0']^fm(c['s1'],k);t=fm(fs(z),z)^fm(r,fs(z))^fm(s,z)^fm(fm(h,z)^k,y);F=[t,s,r,1];G=[k,h]
   N=pa(pa(pm(F,F),pm([0,1],pm(G,F))),pm([B,0,A,1],pm(G,G)));got,rem=pd(N,pm([z,1],[fs(a),0,1]));assert not rem and got==Q
   qkey=tuple(Q)
   if qkey not in cubics:
    power=[0,1];R=[]
    for coefficient in L:R=pa(R,[fm(coefficient,v)for v in power]);power=sm(power,Q)
    R=pd(R,Q)[1];cubics[qkey]=(R,sm(sm(R,Q),Q))
   R,R4=cubics[qkey];assert R==item['L_remainder']and R4==item['fourth_power_residue']and item['supported']==(not R4)and item['excluded_common_factor']==(k==fm(h,a))
   if R4:rejected+=1;assert item['recovery']is None
   else:
    supported+=1
    if not item['excluded_common_factor']:
     recovery=item['recovery'];assert recovery['status']=='complete';mult=list(recovery['multiset']);mult.remove(a);mult.remove(a);rebuild=[1]
     for root in mult:assert root<1<<18;rebuild=pm(rebuild,[root,1])
     assert rebuild==Q and recovery['sources']==[list(v)for v in sorted(set(itertools.permutations(mult+[a,a])))]
selection=p['records'][0]['selection'];assert all(r['selection']==selection for r in p['records'])
for expected,row in enumerate(selection,2):
 a=row[0];assert a==expected;rhs=a^A^fm(B,fp(fs(a),q-2));tr=0;v=rhs
 for _ in range(83):tr^=v;v=fs(v)
 if row[1]=='no_rational_lift':assert tr;continue
 assert not tr;ya=row[2];assert fs(ya)^fm(a,ya)==fm(fs(a),a)^fm(A,fs(a))^B
 lam=a^fm(ya,fp(a,q-2));x2=fs(lam)^lam^A;y2=fs(a)^fm(lam^1,x2);assert row[3]==[x2,y2]
 if row[1]=='outside_order4':assert x2 and row is selection[-1]
 else:assert row[1]=='order_divides4'and not x2
assert [families['k83',i][0]['cell']for i in range(4)]==[{'a':1,'ya':0},{'a':1,'ya':1},{'a':selection[-1][0],'ya':selection[-1][2]},{'a':selection[-1][0],'ya':selection[-1][2]^selection[-1][0]}]
report=dict(schema='recorded-frobenius52-verification/v1',verified=True,method='independent field/polynomial arithmetic; reconstruct both full coordinate constraint matrices and ranks/kernels/maps, equal complete k sets; every recorded norm/remainder/alias; source-only anchor scan; no new roots/candidates searched',family_proofs=proofs,raw_receipt_occurrences=all_receipts,unique_norm_receipts=len(checked),unique_cubics=len(cubics),unique_rejected_norm_parameters=rejected,unique_supported_norm_parameters=supported,generic_anchor_a=selection[-1][0],scan_decisions=len(selection),outside_order4=True,raw_sha256=hashlib.sha256(profile.read_bytes()).hexdigest(),seconds=time.monotonic()-START,qualified_speedup=None,global_original_root_complete=False)
if len(sys.argv)>3:
 with Path(sys.argv[3]).open('x')as stream:json.dump(report,stream,indent=2)
print(json.dumps(report,sort_keys=True))
