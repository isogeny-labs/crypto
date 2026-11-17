"""Total five-coordinate orbit update; standalone exact arithmetic checker."""
from pathlib import Path
import json
p=2**192-2**64-1;r=int('FFFFFFFFFFFFFFFFFFFFFFFF99DEF836146BC9B1B4D22831',16)
a=-3;b=int('64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1',16)
mu=4072180873419819102403428347769737935154921989474241554654
G=(int('188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012',16),int('07192B95FFC8DA78631011ED6B24CDD573F977A11E794811',16))
def neg(P):return None if P is None else (P[0],-P[1]%p)
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u:
  if (y+v)%p==0:return None
  s=(3*x*x+a)*pow(2*y,-1,p)%p
 else:s=(v-y)*pow(u-x,-1,p)%p
 X=(s*s-x-u)%p
 return X,(s*(x-X)-y)%p
def mul(k,P):
 A=None
 while k:
  if k&1:A=add(A,P)
  P=add(P,P);k>>=1
 return A
def encode(R):return tuple(mul(pow(mu,j,r),R) for j in range(5))
def canonical(T):
 if T[0] is None:return T,0,1
 # Nonzero orbit has five distinct x coordinates.
 j=min(range(5),key=lambda i:T[i][0]);s=1 if T[j][1]<(-T[j][1]%p) else -1
 out=T[j:]+T[:j]
 return (out if s==1 else tuple(map(neg,out))),j,s
def batch_inverse(ds):
 # Five inputs, twelve multiplications and one inversion.
 prefix=[ds[0]]
 for d in ds[1:]:prefix.append(prefix[-1]*d%p)
 inv=pow(prefix[-1],-1,p);out=[0]*len(ds)
 for i in range(len(ds)-1,0,-1):out[i]=inv*prefix[i-1]%p;inv=inv*ds[i]%p
 out[0]=inv
 return out
def update(T,U):
 if T[0] is None:return U,'identity'
 if U[0] is None:return T,'identity'
 if T[0][0]==U[0][0] and (T[0][1]+U[0][1])%p==0:return (None,)*5,'cancel'
 double=T[0]==U[0]
 ds=[2*P[1]%p if double else (Q[0]-P[0])%p for P,Q in zip(T,U)]
 assert all(ds)
 invs=batch_inverse(ds);out=[]
 for P,Q,di in zip(T,U,invs):
  x,y=P;u,v=Q;num=(3*x*x+a)%p if double else (v-y)%p
  s=num*di%p;X=(s*s-x-u)%p
  out.append((X,(s*(x-X)-y)%p))
 return tuple(out),'double' if double else 'generic'
def check():
 assert pow(mu,5,r)==1 and mu!=1
 # Test full branch coverage, and labels under arbitrary rotations/signs.
 fixtures=[(0,0),(0,7),(7,0),(7,7),(7,r-7),(1,2),(42,137),(r-1,2)]
 rows=[]
 for k,l in fixtures:
  T=encode(mul(k,G));U=encode(mul(l,G));V,branch=update(T,U)
  expected=encode(mul((k+l)%r,G));assert V==expected
  assert all(add(T[i],U[i])==V[i] for i in range(5))
  rep,j,s=canonical(V);label=s*pow(mu,j,r)%r
  assert rep==encode(mul(label*(k+l)%r,G))
  for h in range(5):
   W=V[h:]+V[:h]
   assert canonical(W)[0]==rep and canonical(tuple(map(neg,W)))[0]==rep
  rows.append({'k':str(k),'l':str(l),'branch':branch,'canonical_rotation':j,'canonical_sign':s})
 # Deterministic labelled walk: no random sampling or speed measurement.
 x=20261107;Q=mul(x,G);steps=[(1,0),(0,1),(17,9),(31,7)]
 table=[encode(add(mul(c,G),mul(d,Q))) for c,d in steps]
 T=encode(None);aa=bb=0;trace=[]
 for n in range(32):
  idx=n%4; c,d=steps[idx];T,branch=update(T,table[idx]);aa=(aa+c)%r;bb=(bb+d)%r
  T,j,s=canonical(T);z=s*pow(mu,j,r)%r;aa=aa*z%r;bb=bb*z%r
  assert T==encode(mul((aa+bb*x)%r,G))
  trace.append({'step':n,'table_index':idx,'rotation':j,'sign':s,'branch':branch})
 return {'all_checks_pass':True,'mu':str(mu),'branch_fixtures':rows,'labelled_walk_steps':32,'trace':trace,'generic_update_cost':{'M':22,'S':5,'I':1},'doubling_update_cost':{'M':22,'S':10,'I':1},'count_scope':'Derived generic affine schedule; field additions and small constant multiples separate; no timing measurement.','state_storage':{'affine_field_elements':10,'raw_bytes_at_192_bits':240,'table_entry_same_bytes':240},'unknown_target_recovery':False}
if __name__=='__main__':
 out=check();Path(__file__).with_name('certificate.json').write_text(json.dumps(out,indent=2)+'\n')
 print('CACHE_PASS: eight total-branch fixtures; 32 exact labelled updates; 22M+5S+1I generic update')
