from pathlib import Path
import json
import reference3 as B
root=Path(__file__).resolve().parent;c=json.loads((root/'certificate.json').read_text())
def coerce(a):return (a%11,0) if isinstance(a,int) else a
def plus(a,b):a,b=coerce(a),coerce(b);return B.plus(a[0],b[0]),B.plus(a[1],b[1])
def minus(a):a=coerce(a);return B.neg(a[0]),B.neg(a[1])
def sub(a,b):return plus(a,minus(b))
def times(a,b):
 a,b=coerce(a),coerce(b)
 return B.plus(B.times(a[0],b[0]),B.times(2,B.times(a[1],b[1]))),B.plus(B.times(a[0],b[1]),B.times(a[1],b[0]))
def inv(a):
 a=coerce(a);z=B.inverse(B.sub(B.times(a[0],a[0]),B.times(2,B.times(a[1],a[1]))));return B.times(a[0],z),B.neg(B.times(a[1],z))
def divide(a,b):return times(a,inv(b))
def power(a,n):
 out=(1,0)
 for bit in bin(n)[2:]:
  out=times(out,out)
  if bit=='1':out=times(out,a)
 return out
def read(a):return B.pack(a[:3]),B.pack(a[3:])
def readpoint(P):return None if P is None else tuple(read(a) for a in P)
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and plus(y,v)==(0,0):return None
 s=divide(plus(times(3,times(x,x)),2),times(2,y)) if P==Q else divide(sub(v,y),sub(u,x))
 X=sub(sub(times(s,s),x),u);return X,sub(times(s,sub(x,X)),y)
def mul(n,P):
 out=None
 for bit in bin(n)[2:]:
  out=add(out,out)
  if bit=='1':out=add(out,P)
 return out
G=readpoint(c['curve']['G']);T=readpoint(c['curve']['T']);S=readpoint(c['shift_generator']);assert mul(7,G) is None and mul(7,T) is None and mul(17,S) is None and mul(7,S) is not None
assert tuple(power(a,11) for a in S)==(S[0],minus(S[1]))
Er=[add(mul(i,G),mul(j,T)) for i in range(7) for j in range(7)];assert len(set(Er))==49
idx={(P):i for i,P in enumerate(Er)};addidx=lambda a,b:((a//7+b//7)%7)*7+(a%7+b%7)%7
shifts=[mul(n,S) for n in range(1,9)];X=[add(S,U)[0] for U in Er];Y=[[add(R,U)[0] for U in Er] for R in shifts]
def total(vals):
 out=(0,0)
 for v in vals:out=plus(out,v)
 return out
assert total(X)==times(49,mul(7,S)[0]) and all(total(ys)==times(49,mul(7,R)[0]) for ys,R in zip(Y,shifts))
checked=0;all_counts=[]
for target in c['targets']:
 Q=readpoint(target['Q']);x=next(j for j in range(1,7) if mul(j,G)==Q)
 psi=[(x*(i//7)%7)*7+((-pow(x,-1,7)*(i%7))%7) for i in range(49)];assert psi==target['psi_permutation']
 arrays=[];counts=[]
 for n,(ys,attempt) in enumerate(zip(Y,target['attempts']),1):
  values=[total(times(X[u],ys[addidx(psi[u],d)]) for u in range(49)) for d in range(49)]
  assert values==[read(v) for v in attempt['values']];checked+=49
  assert len(set(values))==attempt['distinct_conjugates'];assert len(set(values[7*i] for i in range(7)))==attempt['distinct_rational_cosets']
  for d,v in enumerate(values):
   dst=((-(d//7))%7)*7+((-4*(d%7))%7);assert power(v,11)==values[dst]
   if d%7==0:assert power(v,121)==v
  arrays.append(values);counts.append([n,len(set(values)),len(set(values[7*i] for i in range(7)))])
 assert len(set(tuple(a[d] for a in arrays) for d in range(49)))==49
 assert target['joint_translation_stabilizer']==[[i,psi[i]] for i in range(49)]
 # Recompute the initial repeated-root norm product, preserving every factor.
 poly=[(1,0)]
 for v in arrays[0]:
  out=[(0,0)]*(len(poly)+1)
  for i,a in enumerate(poly):out[i]=sub(out[i],times(v,a));out[i+1]=plus(out[i+1],a)
  poly=out
 assert poly==[read(v) for v in target['norm_polynomial_coefficients']]
 # The retained common specialization n=6 has 49 distinct conjugates.
 roots=arrays[5];assert len(set(roots))==49 and target['separating_shift_n']==6
 poly6=[read(v) for v in target['separating_norm_polynomial_coefficients']];assert len(poly6)==50 and poly6[-1]==(1,0)
 for v in roots:
  f=(0,0)
  for a in reversed(poly6):f=plus(times(f,v),a)
  assert f==(0,0)
 assert all(power(a,11)==a for a in poly6)
 seen=set();orbit_lengths=[]
 for v in roots:
  if v in seen:continue
  cur=v;orbit=[]
  while cur not in orbit:orbit.append(cur);cur=power(cur,11)
  assert cur==v;seen.update(orbit);orbit_lengths.append(len(orbit))
 assert sorted(orbit_lengths)==[1]+[2]*3+[6]*7
 all_counts.append({'Q':target['Q'],'attempt_counts':counts})
out={'all_checks_pass':True,'independent_trace_values':checked,'targets':len(c['targets']),'common_separating_shift_n':6,'conjugates_per_target':49,'rational_cosets_per_target':7,'degree49_norm_polynomials':12,'marginal_trace_checks':9,'all_panel_outcomes':all_counts,'scope':'independent tower field, curve arithmetic and complete finite evaluation panel; generic quotient-field argument is written'}
(root/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps({k:v for k,v in out.items() if k!='all_panel_outcomes'},indent=2))
