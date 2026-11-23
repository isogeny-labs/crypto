"""Portable independent tiny-field Euclid and known-factor wide-field replay."""
from pathlib import Path
import hashlib,itertools,json,resource,time
ROOT=Path(__file__).resolve().parents[2];START=time.monotonic()
pins=json.loads((Path(__file__).parent/'EVIDENCE_SHA256.json').read_text());raws={}
for name,digest in pins.items():
 data=(ROOT/name).read_bytes();assert hashlib.sha256(data).hexdigest()==digest;raws[name]=json.loads(data)
def mul(a,b,mod):
 out=0
 while b:
  if b&1:out^=a
  a<<=1;b>>=1
 while out.bit_length()>=mod.bit_length():out^=mod<<(out.bit_length()-mod.bit_length())
 return out
def trim(a):
 a=list(a)
 while a and a[-1]==0:a.pop()
 return a
def product(A,B,mod):
 out=[0]*(len(A)+len(B)-1)
 for i,a in enumerate(A):
  for j,b in enumerate(B):out[i+j]^=mul(a,b,mod)
 return out
def bag(roots,mod):
 H=[1]
 for r in roots:H=product(H,[r,1],mod)
 return H
count=repeats=supported=0;digest=hashlib.sha256()
for n,mod in[(2,7),(3,11)]:
 q=1<<n;table=[[mul(a,b,mod)for b in range(q)]for a in range(q)];inverses={a:next(b for b in range(1,q)if table[a][b]==1)for a in range(1,q)}
 def rem(A,B):
  A=trim(A)
  while len(A)>=len(B):
   c=table[A[-1]][inverses[B[-1]]];shift=len(A)-len(B)
   for i,b in enumerate(B):A[i+shift]^=table[c][b]
   A=trim(A)
  return A
 supported_polys={tuple(bag(rs,mod))for rs in itertools.combinations_with_replacement(range(4),5)}
 for cs in itertools.product(range(q),repeat=5):
  H=list(cs)+[1];A=H;B=[H[1],0,H[3],0,1]
  while B:A,B=B,rem(A,B)
  repeated=len(A)>1;good=tuple(H)in supported_polys
  count+=1;repeats+=repeated;supported+=good;digest.update(bytes([repeated,good]))
control=raws['research/quintic_discriminant_20261115_v44/raw.json']['out']['controls']
assert (count,repeats,supported)==(control['complete_quintics'],control['repeated'],control['supported'])
assert digest.hexdigest()==control['transcript_sha256']
wide=raws['research/quintic_wide_oracles_20261115_v44/raw.json'];assert wide['error']is None;wide_cases=0
for profile in wide['profiles']:
 for row in profile['rows']:
  assert bag(row['roots'],profile['modulus'])==row['H'];assert row['repeated']==(len(set(row['roots']))<5);assert row['supported']==all(r<1<<18 for r in row['roots']);wide_cases+=1
for name in ['research/certified_support_20261115_v44/raw.json','research/quintic_discriminant_20261115_v44/raw.json']:
 x=raws[name];assert x['error']is None;assert len(x['out']['records'])==6 and len({r['fingerprint']for r in x['out']['records']})==1
print(json.dumps(dict(independent_complete_quintics=count,repeated=repeats,supported=supported,wide_known_factor_cases=wide_cases,field_degrees=[p['n']for p in wide['profiles']],seconds=time.monotonic()-START,peak_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,qualified_speedup=None)))
