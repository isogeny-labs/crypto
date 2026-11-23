"""Verify listed recorded candidates only; never search for new k or roots."""
from pathlib import Path
import sys,json,hashlib,time
START=time.monotonic()
profile=Path(sys.argv[1]);fixture=Path(sys.argv[2]);x=json.loads(profile.read_text());data=json.loads(fixture.read_text())
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

A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z
h=0x4208336d0d441df79c400;a=1;ya=0;vs=[1<<i for i in range(18)]
assert [int(v,16)for v in data['source_basis_hex']]==vs
assert fs(y)^fm(z,y)==fm(fs(z),z)^fm(A,fs(z))^B
ia=fp(a,q-2);ic2=fp(fs(a^z),q-2)
D=pm([z,1],[fs(a),0,1])
# Build the sparse linearized annihilator ONCE. Verification below evaluates
# the final sparse polynomial rather than replaying the worker's R recurrence.
L=[1]
for v in vs:
 lam=0;p=v
 for c in L:lam^=fm(c,p);p=fs(p)
 assert lam
 nxt=[0]*(len(L)+1)
 for i,c in enumerate(L):nxt[i]^=fm(lam,c);nxt[i+1]^=fs(c)
 L=nxt

certificates=0;candidate_replays=0;rejected=0;supported=0;unique=set();by_arm=[]
records=x['evidence']['details']['records']
for record in records:
 rows=record['rows'];sol=record['family'];assert sol['consistent'] and sol['rank']==79 and sol['nullity']==4
 for mask,rhs,combo in sol['pivots']:
  assert fx(rows[i][0]for i in range(len(rows))if combo>>i&1)==mask
  assert fx(rows[i][1]for i in range(len(rows))if combo>>i&1)==rhs
  certificates+=1
 assert len({mask.bit_length()for mask,_,_ in sol['pivots']})==79
 assert all((m&sol['particular']).bit_count()%2==r for m,r in rows)
 assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in sol['basis'])
 listed={c['k']for c in record['candidates']}
 closure={sol['particular']^fx(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(16)}
 assert listed==closure and len(listed)==len(record['candidates'])==16
 localreject=0
 for c in record['candidates']:
  k=c['k'];Q=c['Q'];assert 0<=k<q and len(Q)==4 and Q[-1]==1 and all(0<=v<q for v in Q)
  ga=fm(h,a)^k;gz=fm(h,z)^k
  s=fm(fm(k,ia),ya)^fs(a)^fm(a,ga)
  r=fm(fm(ga,ya)^fm(fs(a),a)^fm(fs(z),z)^fm(s,a^z)^fm(gz,y),ic2)
  t=fm(fs(z),z)^fm(r,fs(z))^fm(s,z)^fm(gz,y)
  F=[t,s,r,1];G=[k,h]
  N=pa(pa(pm(F,F),pm([0,1],pm(G,F))),pm([B,0,A,1],pm(G,G)))
  quotient,remainder=pd(N,D);assert not remainder and quotient==Q
  power=pd([0,1],Q)[1];R=[]
  for coefficient in L:
   R=pa(R,[fm(coefficient,u)for u in power]);power=sm(power,Q)
  R=pd(R,Q)[1];R4=sm(sm(R,Q),Q)
  assert R==c['L_remainder'] and R4==c['fourth_power_residue']
  assert c['supported']==(not R4) and c['excluded_common_factor']==(k==fm(h,a))
  assert all((mask&k).bit_count()%2==rhs for mask,rhs in rows)
  if R4:rejected+=1;localreject+=1;assert c['recovery']is None
  else:supported+=1
  unique.add((k,tuple(Q),tuple(R),tuple(R4)));candidate_replays+=1
 assert record['regular_parameters']==16 and record['supported_parameters']==0 and record['complete_regular_norm_fiber']
 by_arm.append({'method':record['method'],'rep':record['rep'],'listed_candidates':len(listed),'independent_support_rejections':localreject})
assert len(records)==6 and len(unique)==16 and rejected==96 and supported==0
print(json.dumps({'schema':'recorded-candidate-verification/v1','verified':True,'method':'independent bit-convolution field arithmetic, direct literal norm, final sparse annihilator evaluation; recorded candidates only, no new candidate/root search','records':len(records),'row_provenance_certificates':certificates,'candidate_receipts_verified':candidate_replays,'unique_candidate_receipts':len(unique),'independent_support_rejections':rejected,'unique_support_rejections':16,'supported_candidates':supported,'by_arm':by_arm,'profile_sha256':hashlib.sha256(profile.read_bytes()).hexdigest(),'fixture_sha256':hashlib.sha256(fixture.read_bytes()).hexdigest(),'seconds':time.monotonic()-START,'global_original_root_complete':False,'qualified_speedup':None},sort_keys=True))
