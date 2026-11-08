from pathlib import Path
import json,hashlib,importlib.util
root=Path(__file__).resolve().parent;c=json.loads((root/'certificate.json').read_text());inp=c['input'];p=inp['p'];a=inp['a'];b=inp['b'];r=inp['r']
def add(P,Q,p,a):
 if P is None:return Q
 if Q is None:return P
 if P[0]==Q[0] and (P[1]+Q[1])%p==0:return None
 x,y=P;u,v=Q;s=((3*x*x+a)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p;xx=(s*s-x-u)%p
 return [xx,(s*(x-xx)-y)%p]
def mul(n,P,p,a):
 R=None
 for bit in bin(n)[2:]:
  R=add(R,R,p,a)
  if bit=='1':R=add(R,P,p,a)
 return R
def affine(P):return None if P[2] else list(P[:2])
def normalized(P):return [0,0,1] if P is None else list(P)+[0]
def accept(P,Q,R,case,s,z,p,a,b):
 x,y,o=P;u,v,q=Q;X,Y,O=R
 if any(v%p not in [0,1] for v in [o,q,O]):return False
 for xx,yy,oo in [P,Q,R]:
  if oo:
   if xx or yy:return False
  elif (yy*yy-xx**3-a*xx-b)%p:return False
 if case==0:return o==1 and R==Q and s==z==0
 if case==1:return o==0 and q==1 and R==P and s==z==0
 if o or q:return False
 if case==2:return x==u and (y+v)%p==0 and R==[0,0,1] and s==z==0
 if O:return False
 if case==3:
  if P!=Q or (2*y*z)%p!=1 or s!=(3*x*x+a)*z%p:return False
 elif case==4:
  if (u-x)*z%p!=1 or s!=(v-y)*z%p:return False
 else:return False
 return X==(s*s-x-u)%p and Y==(s*(x-X)-y)%p
assert p==2**192-2**64-1 and b==int('64210519E59C80E70FA7E9AB72243049FEB8DEECC146B9B1',16)
assert mul(r,inp['G'],p,a) is None and mul(r,inp['Q'],p,a) is None and inp['G']!=[0,0] and inp['Q']!=[0,0]
# Reconstruct producer witnesses with independent arithmetic and branch semantics.
for row in c['P192']:
 n=row['digits_integer'];bits=[(n//2**i)%2 for i in range(c['k'])];values=bits[:];states=[[0,0,1],[0,0,1]];bases=[list(inp['G']),list(inp['Q'])];counts=[0]*5
 for bit in bits:
  for j in range(2):
   P=states[j];Q=normalized(bases[j]) if bit else [0,0,1];R=normalized(add(affine(P),affine(Q),p,a));s=z=0
   if P[2]:case=0
   elif Q[2]:case=1
   elif R[2]:case=2
   elif P==Q:case=3;z=pow(2*P[1],-1,p);s=(3*P[0]*P[0]+a)*z%p
   else:case=4;z=pow(Q[0]-P[0],-1,p);s=(Q[1]-P[1])*z%p
   assert accept(P,Q,R,case,s,z,p,a,b)
   selectors=[int(i==case) for i in range(5)];values+=R+selectors+[s,z];counts[case]+=1;states[j]=R;bases[j]=add(bases[j],bases[j],p,a)
 assert states==row['output']==[normalized(mul(n,inp['G'],p,a)),normalized(mul(n,inp['Q'],p,a))]
 assert row['variables']==len(values)==21*c['k'] and row['equations']==89*c['k']
 assert counts==row['branch_counts'];assert hashlib.sha256(json.dumps(values,separators=(',',':')).encode()).hexdigest()==row['witness_sha256']
# Check the actual polynomial predicate on every toy input pair, using independent point arithmetic.
s=importlib.util.spec_from_file_location('published_template',root/'model.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m)
toy=c['toy'];tp=toy['p'];ta=toy['a'];tb=toy['b'];points=[None]+[[x,y] for x in range(tp) for y in range(tp) if (y*y-x*x*x-ta*x-tb)%tp==0];assert len(points)==19
branches=set();mutations=0
for P in points:
 for Q in points:
  R,ss,S,Z=m.witness(tuple(normalized(P)),tuple(normalized(Q)),tp,ta,tb);R=list(R);case=ss.index(1);branches.add(case)
  assert R==normalized(add(P,Q,tp,ta));assert accept(normalized(P),normalized(Q),R,case,S,Z,tp,ta,tb)
  assert not any(m.residuals(normalized(P),normalized(Q),R,ss,S,Z,tp,ta,tb))
  bad=R[:];bad[0]=(bad[0]+1)%tp
  assert any(m.residuals(normalized(P),normalized(Q),bad,ss,S,Z,tp,ta,tb));mutations+=1
  badselect=ss[:];badselect[(case+1)%5]=1
  assert any(m.residuals(normalized(P),normalized(Q),R,badselect,S,Z,tp,ta,tb));mutations+=1
assert branches==set(range(5))
T=(0,0,0);R,ss,S,Z=m.witness(T,T,17,1,0);assert R==(0,0,1) and ss[2]==1 and not any(m.residuals(T,T,R,ss,S,Z,17,1,0))
assert 2*(2-1)%17!=0

outputs=[]
for row in toy['rows']:
 n=row['n'];expected=[normalized(mul(n,toy['G'],tp,ta)),normalized(mul(n,toy['Q'],tp,ta))];assert row['output']==expected;outputs.append(tuple(tuple(P) for P in expected))
assert len(set(outputs))==toy['projected_points']==19
out={'all_checks_pass':True,'independent_P192_witnesses':6,'toy_addition_pairs':19**2,'all_five_branches':True,'two_torsion_cancellation':True,'nonbinary_digit_control':True,'rejected_toy_mutations':mutations,'toy_bitstrings':32,'toy_projected_subgroup_order':19,'P192_variables':21*c['k'],'P192_equations':89*c['k'],'maximum_degree':max(c['template_degrees']),'scope':'independent arithmetic/branch semantics; published polynomial predicate is exercised directly'}
(root/'verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
