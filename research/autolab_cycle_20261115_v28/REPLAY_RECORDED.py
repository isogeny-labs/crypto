"""Independent arithmetic and complete-set replay; no solver/backend imports."""
from pathlib import Path
import json,hashlib,itertools
p=Path(__file__).resolve().parents[2]/'research/generic_block_oracles_20261115_v28/raw.json';data=p.read_bytes();assert hashlib.sha256(data).hexdigest()=='3cde41bf85e3d955da130e122a853d09a47abb1583dbaaa6f6cf580a64ae76de';raw=json.loads(data);assert raw['error']is None;out=raw['out'];count=0
for f in out['small']:
 p,d,nu=f['p'],f['degree'],f['nu'];q=p**d
 def plus(a,b):return sum((((a//p**i)%p+(b//p**i)%p)%p)*p**i for i in range(d))
 def times(a,b):
  c=[0]*(2*d-1)
  for i in range(d):
   for j in range(d):c[i+j]+=(a//p**i%p)*(b//p**j%p)
  if d==2:c[0]+=nu*c[2]
  return sum((c[i]%p)*p**i for i in range(d))
 def dot(a,b):
  t=0
  for x,y in zip(a,b):t=plus(t,times(x,y))
  return t
 for case in f['cases']:
  n=case['n'];M=case['matrix'];rhs=case['rhs'];truth={x for x in itertools.product(range(q),repeat=n)if all(dot(row,x)==c for row,c in zip(M,rhs))};count+=q**n;got=set();sol=case['result']
  if sol is not None:
   for t in itertools.product(range(q),repeat=len(sol['basis'])):
    x=list(sol['particular'])
    for a,B in zip(t,sol['basis']):x=[plus(v,times(a,b))for v,b in zip(x,B)]
    got.add(tuple(x))
  assert got==truth and len(truth)==case['exact_count']
  assert hashlib.sha256(json.dumps(sorted(truth),separators=(',',':')).encode()).hexdigest()==case['complete_set_sha256']
assert count==92706 and out['literal_assignments']==93454
expected={192:2**192-2**64-1,224:2**224-2**96+1,256:2**256-2**224+2**192+2**96-1,384:2**384-2**128-2**96+2**32-1,521:2**521-1};seen=set()
for f in out['wide']:
 p=f['prime'];assert p==expected[f['prime_bits']];seen.add(f['prime_bits'])
 for case in f['cases']:
  M=case['matrix'];rhs=case['rhs'];sol=case['result'];assert M[0][0]==1 and M[1][:2]==[0,1]
  assert M[2]==[(a+b)%p for a,b in zip(M[0],M[1])]
  if not case['expected_consistent']:assert sol is None and (rhs[2]-rhs[0]-rhs[1])%p==1;continue
  assert (rhs[2]-rhs[0]-rhs[1])%p==0 and len(sol['basis'])==1
  part=sol['particular'];v=sol['basis'][0];b,c=M[0][1:];f=M[1][2];assert v[2];inv=pow(v[2],-1,p)
  assert tuple(x*inv%p for x in v)==((b*f-c)%p,(-f)%p,1)
  t=part[2];assert part==[(rhs[0]-b*((rhs[1]-f*t)%p)-c*t)%p,(rhs[1]-f*t)%p,t]
assert seen==set(expected)
print('VERIFIED: all306recorded small-field full solution sets and80NIST exact line/contradiction cases; linear backend scope only.')
