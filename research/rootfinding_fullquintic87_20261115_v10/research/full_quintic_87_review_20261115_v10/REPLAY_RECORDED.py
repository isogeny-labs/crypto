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

# Exact verification of supplied affine certificates only; no root search and
# no enumeration of the surviving2^73 parameter assignments.
A=int(data['curve']['a_hex'],16);B=int(data['curve']['b_hex'],16);z=int(data['target']['x_hex'],16);y=int(data['target']['y_hex'],16)^z;h=0x4208336d0d441df79c400
assert x['error']is None and x['out']['profile']['complete_equivalent_relaxation']
proofs=x['out']['profile']['proofs'];base=proofs['baseline166'];block=proofs['frozen104']

def rank(values):
 piv={}
 for v in values:
  while v:
   bit=v.bit_length()-1
   if bit not in piv:piv[bit]=v;break
   v^=piv[bit]
 return len(piv)

def residue(value,values):
 piv={}
 for v in values:
  while v:
   b=v.bit_length()-1
   if b not in piv:piv[b]=v;break
   v^=piv[b]
 while value:
  b=value.bit_length()-1
  if b not in piv:break
  value^=piv[b]
 return value

def norm(raw):
 r=raw&(q-1);s=raw>>83;t=fm(fs(z),z)^fm(r,fs(z))^fm(s,z)^fm(fm(h,z),y);F=[t,s,r,1];G=[0,h]
 N=pa(pa(pm(F,F),pm([0,1],pm(G,F))),pm([B,0,A,1],pm(G,G)));H,rem=pd(N,[z,1]);assert not rem and len(H)==6 and H[-1]==1;return H

def pack(H):
 out=0
 for j,d in enumerate([18,35,52,69,83],1):out|=((H[5-j]>>d)<<d)<<((j-1)*83)
 return out
H0=norm(0);cols=[]
for bit in range(166):cols.append([a^b for a,b in zip(norm(1<<bit),H0)])
rhs=pack(H0);packed=[pack(c)for c in cols];rows=[(sum(((v>>j)&1)<<i for i,v in enumerate(packed)),(rhs>>j)&1)for j in range(415)]
assert base['rows']==[list(r)for r in rows] and base['variables']==166 and base['rank']==rank([m for m,r in rows])==93
assert base['consistent'] and len(base['basis'])==rank(base['basis'])==73
assert all((m&base['particular']).bit_count()%2==r for m,r in rows) and all(all((m&v).bit_count()%2==0 for m,r in rows)for v in base['basis'])
C2,C3,C4,C5=H0[3],H0[2],H0[1],H0[0];assert block['constants']==H0 and C5==fm(z,C4)

def tr(v):
 out=0
 for _ in range(83):out^=v;v=fs(v)
 return out
invh2=fp(fs(h),q-2);trace_row=[sum(tr(fm(1<<i,invh2))<<i for i in range(35)),tr(fm(C2,invh2))]
assert block['trace_row']==trace_row and block['rho']==1 and trace_row[0]
assert (trace_row[0]&block['trace_particular']).bit_count()%2==trace_row[1]
assert len(block['trace_basis'])==rank(block['trace_basis'])==34 and all((trace_row[0]&v).bit_count()%2==0 for v in block['trace_basis'])
r0=block['r0'];w0=block['w0'];rd=block['r_directions'];wd=block['w_directions'];assert fs(r0)^fm(h,r0)==block['trace_particular']^C2 and fs(w0)==C4
assert len(rd)==35 and rank(rd)==35 and rd[-1]==h
assert [fs(v)^fm(h,v)for v in rd[:-1]]==block['trace_basis']
assert len(wd)==69 and [fs(v)for v in wd]==[1<<i for i in range(69)]
raw0=r0|((w0^fm(z,r0))<<83);dirs=[v|(fm(z,v)<<83)for v in rd]+[v<<83 for v in wd];assert len(dirs)==rank(dirs)==104
constant=norm(raw0);columns=[]
for raw in dirs:
 dr=raw&(q-1);ds=raw>>83;dw=ds^fm(z,dr);columns.append(fm(z,fs(dr))^fm(h,dw))
coupling=[(sum(((v>>bit)&1)<<i for i,v in enumerate(columns)),(constant[2]>>bit)&1)for bit in range(52,83)]
assert block['rows']==[list(r)for r in coupling] and block['variables']==104 and block['rank']==rank([m for m,r in coupling])==31
cp=block['coordinate_particular'];cb=block['coordinate_basis'];assert len(cb)==rank(cb)==73
assert all((m&cp).bit_count()%2==r for m,r in coupling)and all(all((m&v).bit_count()%2==0 for m,r in coupling)for v in cb)
lift=lambda bits:fx(v for i,v in enumerate(dirs)if bits>>i&1)
assert block['particular']==raw0^lift(cp)and block['basis']==[lift(v)for v in cb]
assert len(block['basis'])==rank(block['basis'])==73 and residue(base['particular']^block['particular'],base['basis'])==0
assert all(residue(v,base['basis'])==0 for v in block['basis'])and all(residue(v,block['basis'])==0 for v in base['basis'])
for proof in [base,block]:
 assert all((m&proof['particular']).bit_count()%2==r for m,r in rows)
 assert all(all((m&v).bit_count()%2==0 for m,r in rows)for v in proof['basis'])
# New87: independently reconstruct its exact e2/e3 coordinate map and e4 border.
new=proofs['new87'];assert new['constants']==H0 and new['rho']==1 and new['trace_row']==trace_row
assert (trace_row[0]&new['trace_particular']).bit_count()%2==trace_row[1]
assert len(new['trace_basis'])==rank(new['trace_basis'])==34 and all((trace_row[0]&v).bit_count()%2==0 for v in new['trace_basis'])
r0n=new['r0'];w0n=new['w0'];assert fs(r0n)^fm(h,r0n)==new['trace_particular']^C2
assert fm(h,w0n)==C3^fm(z,fs(r0n))
dirsn=new['directions'];assert len(dirsn)==rank(dirsn)==87
rdn=[v&(q-1)for v in dirsn[:35]];assert len(rdn)==rank(rdn)==35 and rdn[-1]==h
assert [fs(v)^fm(h,v)for v in rdn[:-1]]==new['trace_basis']
for raw in dirsn[:35]:
 dr=raw&(q-1);dw=(raw>>83)^fm(z,dr);assert fm(h,dw)==fm(z,fs(dr))
for i,raw in enumerate(dirsn[35:]):assert not raw&(q-1) and fm(h,raw>>83)==1<<i
raw0n=r0n|((w0n^fm(z,r0n))<<83);constantn=norm(raw0n)
assert constantn[3]>>35==0 and constantn[2]>>52==0 and constantn[4]>>18==0
colsn=[]
for raw in dirsn:
 dr=raw&(q-1);dw=(raw>>83)^fm(z,dr);colsn.append(fs(dw))
rowsn=[(sum(((v>>bit)&1)<<i for i,v in enumerate(colsn)),(constantn[1]>>bit)&1)for bit in range(69,83)]
assert new['rows']==[list(r)for r in rowsn] and new['variables']==87 and new['rank']==rank([m for m,r in rowsn])==14
cpn=new['coordinate_particular'];cbn=new['coordinate_basis'];assert len(cbn)==rank(cbn)==73
assert all((m&cpn).bit_count()%2==r for m,r in rowsn)and all(all((m&v).bit_count()%2==0 for m,r in rowsn)for v in cbn)
liftn=lambda bits:fx(v for i,v in enumerate(dirsn)if bits>>i&1)
assert new['particular']==raw0n^liftn(cpn)and new['basis']==[liftn(v)for v in cbn]
assert len(new['basis'])==rank(new['basis'])==73 and residue(base['particular']^new['particular'],base['basis'])==0
assert all(residue(v,base['basis'])==0 for v in new['basis'])and all(residue(v,new['basis'])==0 for v in base['basis'])
assert all((m&new['particular']).bit_count()%2==r for m,r in rows)and all(all((m&v).bit_count()%2==0 for m,r in rows)for v in new['basis'])
report=dict(schema='recorded-full-quintic87-verification/v1',verified=True,method='independent bit-convolution arithmetic/literal norms; reconstruct166baseline columns,104AS/Frobenius map and87AS/e3 map, allborder matrices/ranks/coordinate andrawkernels/cosets; no2^73 orroot search',baseline_variables=166,baseline_rank=93,frozen104_variables=104,frozen104_border_rank=31,new87_variables=87,new87_border_rows=14,new87_border_rank=14,trace_rho=1,surviving_bits=73,all_three_same_complete_affine_family=True,supported_roots_computed=False,raw_sha256=hashlib.sha256(profile.read_bytes()).hexdigest(),seconds=time.monotonic()-START,qualified_speedup=None,original_global_complete=False)
if len(sys.argv)>3:
 with Path(sys.argv[3]).open('x')as out:json.dump(report,out,indent=2)
print(json.dumps(report,sort_keys=True))
