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

A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z
h=0x4208336d0d441df79c400;a=1;ya=0;vs=[1<<i for i in range(18)]
assert [int(v,16)for v in data['source_basis_hex']]==vs
assert fs(y)^fm(z,y)==fm(fs(z),z)^fm(A,fs(z))^B
ia=fp(a,q-2);ic2=fp(fs(a^z),q-2)
D=pm([z,1],[fs(a),0,1])
# Build the sparse linearized annihilator ONCE. Verification below evaluates
# the final sparse polynomial rather than replaying the worker's R recurrence.
L=[1];LAM=[]
for v in vs:
 lam=0;p=v
 for c in L:lam^=fm(c,p);p=fs(p)
 assert lam;LAM.append(lam)
 nxt=[0]*(len(L)+1)
 for i,c in enumerate(L):nxt[i]^=fm(lam,c);nxt[i+1]^=fs(c)
 L=nxt

# Every following computation verifies supplied recorded values. It does not
# search for a new candidate k, source root or h value.
assert x['mode']=='profile' and x['error']is None
records=x['out']['records'];assert len(records)==9 and x['out']['complete_batch_comparison']
family_seen=set();norm_seen=set();Q_seen={};h_seen=set();pivot_certificates=0;receipt_count=0;rejection_count=0;support_count=0;recovery_count=0;unique_rejects=set();unique_supports=set()

def family_check(rows,sol,n=83):
 global pivot_certificates
 key=json.dumps([rows,sol],sort_keys=True,separators=(',',':'))
 if key in family_seen:return
 family_seen.add(key)
 for mask,rhs,combo in sol['pivots']:
  assert fx(rows[i][0]for i in range(len(rows))if combo>>i&1)==mask
  assert fx(rows[i][1]for i in range(len(rows))if combo>>i&1)==rhs
  pivot_certificates+=1
 assert len({mask.bit_length()for mask,_,_ in sol['pivots']})==sol['rank']
 assert len(sol['basis'])==n-sol['rank']==sol['nullity']
 # Distinct affine combinations certify independence for the small k kernels.
 if sol['nullity']<=12:assert len({fx(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(1<<sol['nullity'])})==1<<sol['nullity']
 else:
  piv={}
  for value in sol['basis']:
   for j in sorted(piv,reverse=True):
    if value>>j&1:value^=piv[j]
   assert value;piv[value.bit_length()-1]=value
 assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in sol['basis'])
 if sol['consistent']:assert all((m&sol['particular']).bit_count()%2==r for m,r in rows)
 else:
  combo=sol['contradiction'];assert combo is not None
  assert fx(rows[i][0]for i in range(len(rows))if combo>>i&1)==0 and fx(rows[i][1]for i in range(len(rows))if combo>>i&1)==1

for record in records:
 assert record['complete_batch'] and record['indexed_cells']==65 and len(record['cells'])==65
 hp=record['h_proof'];family_check(hp['rows'],hp['family']);assert hp['family']['consistent'] and hp['family']['nullity']==18
 expected_h=[hp['family']['particular']^fx(v for j,v in enumerate(hp['family']['basis'])if i>>j&1)for i in range(32)]
 assert hp['h_values']==expected_h
 hcols=[(fs(1<<i)^(1<<i))>>18 for i in range(83)]
 expected_rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(hcols)),(z>>(18+j))&1)for j in range(65)]
 assert hp['rows']==[list(t)for t in expected_rows]
 expected_cells=[(0,0x4208336d0d441df79c400,1,0)]+[(1+2*i+yy,hh,1,yy)for i,hh in enumerate(expected_h)for yy in [0,1]]
 assert [(c['cell']['index'],c['cell']['h'],c['cell']['a'],c['cell']['ya'])for c in record['cells']]==expected_cells
 assert record['distinct_cells']==hp['distinct_cells']==len({t[1:]for t in expected_cells})
 if record['method']=='generic_per_k':assert record['lambda_cache']is None
 else:assert record['lambda_cache']==LAM
 eligible=0;regular=0;supported_regular=0;tuples=0
 for cell in record['cells']:
  desc=cell['cell'];h=desc['h'];a=desc['a'];ya=desc['ya'];assert a==1 and ya in [0,1] and (fs(h)^h^z)<1<<18
  assert fs(ya)^fm(a,ya)==fm(fs(a),a)^fm(A,fs(a))^B
  sol=cell['family'];rows=cell['rows'];family_check(rows,sol);assert cell['complete_regular_norm_fiber']
  closure=set()if not sol['consistent']else{sol['particular']^fx(v for i,v in enumerate(sol['basis'])if t>>i&1)for t in range(1<<sol['nullity'])}
  assert sol['nullity']<=12 and {c['k']for c in cell['candidates']}==closure and len(cell['candidates'])==len(closure)
  eligible+=len(closure);localregular=0;localpositive=0
  for c in cell['candidates']:
   receipt_count+=1;k=c['k'];Q=c['Q'];assert 0<=k<q and len(Q)==4 and Q[-1]==1
   assert c['excluded_common_factor']==(k==fm(h,a))
   assert all((mask&k).bit_count()%2==rhs for mask,rhs in rows)
   normkey=(h,a,ya,k,tuple(Q))
   if normkey not in norm_seen:
    norm_seen.add(normkey);ga=fm(h,a)^k;gz=fm(h,z)^k
    s=fm(fm(k,ia),ya)^fs(a)^fm(a,ga)
    r=fm(fm(ga,ya)^fm(fs(a),a)^fm(fs(z),z)^fm(s,a^z)^fm(gz,y),ic2)
    t=fm(fs(z),z)^fm(r,fs(z))^fm(s,z)^fm(gz,y);F=[t,s,r,1];G=[k,h]
    N=pa(pa(pm(F,F),pm([0,1],pm(G,F))),pm([B,0,A,1],pm(G,G)));got,rem=pd(N,D);assert not rem and got==Q
   qkey=tuple(Q)
   if qkey not in Q_seen:
    power=pd([0,1],Q)[1];R=[]
    for coefficient in L:R=pa(R,[fm(coefficient,u)for u in power]);power=sm(power,Q)
    R=pd(R,Q)[1];R4=sm(sm(R,Q),Q);Q_seen[qkey]=(R,R4)
   R,R4=Q_seen[qkey];assert R==c['L_remainder'] and R4==c['fourth_power_residue'] and c['supported']==(not R4)
   if record['method']=='explicit_cached':assert c['multipliers']==[fm(Q[2],Q[0]),fm(Q[2],Q[1])^Q[0],fs(Q[2])^Q[1]]
   else:assert c['multipliers']is None
   if R4:rejection_count+=1;unique_rejects.add(normkey);assert c['recovery']is None
   else:support_count+=1;unique_supports.add(normkey)
   if not c['excluded_common_factor']:
    regular+=1;localregular+=1
    if c['supported']:
     localpositive+=1;supported_regular+=1;rec=c['recovery'];assert rec['status']=='complete'
     mult=list(rec['multiset']);assert len(mult)==5;mult.remove(a);mult.remove(a)
     rebuilt=[1]
     for value in mult:assert 0<=value<1<<18;rebuilt=pm(rebuilt,[value,1])
     assert rebuilt==Q
     expected=[list(t)for t in sorted(set(itertools.permutations(mult+[a,a])))];assert rec['sources']==expected
     tuples+=len(expected);recovery_count+=1
   else:assert c['recovery']is None
  assert cell['regular_parameters']==localregular and cell['supported_parameters']==localpositive
 assert eligible==record['eligible_candidates_preflight']<=8192
 assert record['global_regular_parameters']==regular and record['global_supported_parameters']==supported_regular and record['global_source_tuple_count']==tuples<=100000
assert {r['method']for r in records}=={'generic_per_k','generic_cached','explicit_cached'}
report=dict(schema='recorded-batch-verification/v1',verified=True,method='independent bit-convolution field arithmetic, literal norm, final sparse annihilator evaluation; deduplicated recorded receipts only; no new candidate/root/h search',batch_records=len(records),indexed_cells_per_batch=65,distinct_cells_per_batch=records[0]['distinct_cells'],raw_candidate_receipts=receipt_count,unique_literal_norm_receipts=len(norm_seen),unique_cubics=len(Q_seen),unique_affine_families=len(family_seen),unique_pivot_certificates_replayed=pivot_certificates,raw_support_rejections=rejection_count,unique_support_rejections=len(unique_rejects),raw_supported_receipts=support_count,unique_supported_receipts=len(unique_supports),supported_recoveries_checked=recovery_count,raw_sha256=hashlib.sha256(profile.read_bytes()).hexdigest(),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),seconds=time.monotonic()-START,global_original_root_complete=False,qualified_speedup=None)
if len(sys.argv)>3:
 with Path(sys.argv[3]).open('x')as out:json.dump(report,out,indent=2)
print(json.dumps(report,sort_keys=True))
