"""Verify listed recorded candidates only; never search for new k or roots."""
from pathlib import Path
import sys,json,hashlib,time,gzip
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

# Recorded-data verification only. No new shared-t points, roots or functionals
# are sought. All numeric support replays below are on already recorded points.
actual=x['out']['actual'];proof=actual['proof'];sol=proof['family'];rows=proof['rows']
assert x['error']is None and sol['consistent'] and sol['rank']==93 and sol['nullity']==73
for mask,rhs,combo in sol['pivots']:
 assert fx(rows[i][0]for i in range(len(rows))if combo>>i&1)==mask
 assert fx(rows[i][1]for i in range(len(rows))if combo>>i&1)==rhs
assert len({m.bit_length()for m,r,c in sol['pivots']})==93
assert all((m&sol['particular']).bit_count()%2==r for m,r in rows)
assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in sol['basis'])
piv={}
for value in sol['basis']:
 for bit in sorted(piv,reverse=True):
  if value>>bit&1:value^=piv[bit]
 assert value;piv[value.bit_length()-1]=value
A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=actual['h'];k=actual['k'];vs=[1<<i for i in range(18)]
assert h==0x4208336d0d441df79c400 and k==0 and [int(v,16)for v in data['source_basis_hex']]==vs
L=[1];lams=[]
for v in vs:
 lam=0;power=v
 for c in L:lam^=fm(c,power);power=fs(power)
 assert lam;lams.append(lam);nxt=[0]*(len(L)+1)
 for i,c in enumerate(L):nxt[i]^=fm(lam,c);nxt[i+1]^=fs(c)
 L=nxt
assert lams==actual['lambdas']
points=actual['points'];assert len(points)==actual['unique_evaluated_points']<=2048
assert len({p[0]for p in points})==len(points)
for t,raw,H,Rgiven,R8given in points:
 assert 0<=t<1<<73 and 0<=raw<1<<166
 assert raw==sol['particular']^fx(v for bit,v in enumerate(sol['basis'])if t>>bit&1)
 assert H==[proof['H_base'][j]^fx(c[j]for bit,c in enumerate(proof['H_columns'])if t>>bit&1)for j in range(6)]
 r=raw&(q-1);s=raw>>83;t0=fm(fs(z),z)^fm(r,fs(z))^fm(s,z)^fm(fm(h,z)^k,y)
 F=[t0,s,r,1];G=[k,h];N=pa(pa(pm(F,F),pm([0,1],pm(G,F))),pm([B,0,A,1],pm(G,G)));got,rem=pd(N,[z,1]);assert not rem and got==H
 power=[0,1];R=[]
 for c in L:R=pa(R,[fm(c,v)for v in power]);power=sm(power,H)
 R=pd(R,H)[1];R8=sm(sm(sm(R,H),H),H)
 assert R+[0]*(5-len(R))==Rgiven and R8+[0]*(5-len(R8))==R8given
assert len(actual['cells'])==7
selection=[(0,0),(1,0),(1,1),(2,0),(2,1),(2,2),(2,3)];witnesses=[];occurrences=0
for cell,(depth,prefix)in zip(actual['cells'],selection):
 assert (cell['depth'],cell['prefix'])==(depth,prefix)
 indices=cell['point_indices'];assert len(indices)<=512 and len(set(indices))==len(indices)
 for free_index,pi in enumerate(indices):assert 0<=pi<len(points) and points[pi][0]==prefix|(free_index<<depth)
 occurrences+=len(indices)
 if 'proof'not in cell:assert cell['class_status']=='unknown';continue
 wp=cell['proof'];values=[sum(v<<(83*i)for i,v in enumerate(points[pi][4]))for pi in indices]
 basis={}
 for value in values:
  value|=1<<415
  while value:
   bit=value.bit_length()-1
   if bit not in basis:basis[bit]=value;break
   value^=basis[bit]
 assert wp['rank']==len(basis) and wp['pivots']==[[i,v]for i,v in sorted(basis.items())]
 target=1<<415
 while target:
  bit=target.bit_length()-1
  if bit not in basis:break
  target^=basis[bit]
 assert wp['found']==(target==0)
 if wp['found']:
  selected=wp['selected'];assert len(selected)%2==1 and len(set(selected))==len(selected) and all(0<=i<len(indices)for i in selected)
  assert fx(values[i]for i in selected)==0
  assert cell['selected_table_indices']==[indices[i]for i in selected]
  assert cell['class_status']=='constant_functional_class_impossible'
  witnesses.append(dict(depth=depth,prefix=prefix,size=len(selected),rank=wp['rank']))
 else:assert cell['class_status']=='unknown'and not cell['selected_table_indices']
assert occurrences==actual['point_occurrences']<=3584
assert actual['root_cell_class_obstructed']==bool(witnesses and witnesses[0]['depth']==0)
assert actual['overlapping_diagnostics']and not actual['disjoint_cover']and not actual['cell_unsat_claim']and not actual['original_global_complete']
report=dict(schema='recorded-dual-obstruction-verification/v1',verified=True,method='independent bit-convolution field arithmetic, literal curve norms and final sparse annihilator evaluation on every recorded point; exact Gaussian/odd-XOR replay; no new points/root/functionals searched',gaussian_pivot_certificates=93,independent_kernel_dimension=len(piv),all_recorded_points_independently_verified=len(points),point_occurrences=occurrences,witnesses=witnesses,root_constant_functional_class_obstructed=actual['root_cell_class_obstructed'],zero_support_point_observations=sum(not any(p[4])for p in points),diagnostic_cells_overlap=True,cell_unsat_claim=False,global_original_root_complete=False,qualified_speedup=None,raw_sha256=hashlib.sha256(profile.read_bytes()).hexdigest(),fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),seconds=time.monotonic()-START)
if len(sys.argv)>3:
 with Path(sys.argv[3]).open('x')as out:json.dump(report,out,indent=2)
print(json.dumps(report,sort_keys=True))
