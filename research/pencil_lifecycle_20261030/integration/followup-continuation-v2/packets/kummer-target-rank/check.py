import json
from pathlib import Path
P=11;K=3
def norm(d,a,b,r=11,g=1,q=3,identity='same'):
 assert identity=='same','target_identity_mismatch'
 assert r==11 and g!=0,'unsupported_order_or_generator'
 assert d==(a*g+b*q)%r,'label_invariant_failed'
 key=min(d,(-d)%r);sgn=1 if d==key else -1
 return key,(sgn*a)%r,(sgn*b)%r
rows=[]
for name,old,new,expected in [('same_sign',(4,1,1),(4,9,2),3),('opposite_sign',(4,1,1),(7,1,2),3),('redundant_same',(4,1,1),(4,1,1),None),('redundant_opposite',(4,1,1),(7,10,10),None)]:
 x=norm(*old);y=norm(*new);assert x[0]==y[0];da=(y[1]-x[1])%P;db=(y[2]-x[2])%P;ans=(-da*pow(db,-1,P))%P if db else None
 assert db or da==0;assert ans==expected;rows.append({'name':name,'old':old,'new':new,'normalized':[x,y],'delta':[da,db],'answer':ans,'pass':True})
for name,args,kwargs in [('bad_label',(4,2,1),{}),('mixed_target',(4,1,1),{'identity':'other'}),('composite_order',(4,1,1),{'r':15}),('zero_generator',(3,1,1),{'g':0})]:
 try:norm(*args,**kwargs)
 except AssertionError as e:rows.append({'name':name,'rejected':str(e),'pass':True})
 else:raise AssertionError(name)
def nullspace(m,n):
 a=[[x%P for x in r]for r in m];piv=[];i=0
 for c in range(n):
  j=next((j for j in range(i,len(a))if a[j][c]),None)
  if j is None:continue
  a[i],a[j]=a[j],a[i];z=pow(a[i][c],-1,P);a[i]=[(x*z)%P for x in a[i]]
  for j in range(len(a)):
   if j!=i:
    z=a[j][c];a[j]=[(x-z*y)%P for x,y in zip(a[j],a[i])]
  piv.append(c);i+=1
 out=[]
 for f in range(n):
  if f in piv:continue
  v=[0]*n;v[f]=1
  for j,c in enumerate(piv):v[c]=-a[j][f]%P
  out.append(v)
 return out
rank=[]
forest=[[1,-1,0],[0,1,-1]];balanced=forest+[[1,0,-1]]
for name,A,b,dim,bearing in [('forest',forest,[1,2],0,False),('balanced_target',balanced,[0,0,1],1,True),('balanced_redundant',balanced,[1,2,3],1,False),('unbalanced_anchor',[[1,1,0],[0,1,1],[1,0,1]],[0,0,1],0,False)]:
 a=[(sum(z*x for z,x in zip(row,[2,5,7]))-bi*K)%P for row,bi in zip(A,b)];ys=nullspace(list(map(list,zip(*A))),len(A));assert len(ys)==dim;constraints=[]
 for y in ys:
  assert all(sum(y[i]*A[i][j]for i in range(len(A)))%P==0 for j in range(3))
  ya=sum(x*z for x,z in zip(y,a))%P;yb=sum(x*z for x,z in zip(y,b))%P;ans=-ya*pow(yb,-1,P)%P if yb else None;assert (ans==K)if yb else ya==0;constraints.append({'y':y,'ya':ya,'yb':yb,'answer':ans})
 assert any(x['yb']for x in constraints)==bearing;rank.append({'name':name,'A':A,'a':a,'b':b,'constraints':constraints,'nullity':dim,'target_bearing':bearing,'pass':True})
print(json.dumps({'schema':'research.fixed-algebra-check/v1','duplicate_controls':rows,'rank_controls':rank,'cases':12,'all_pass':True,'scientific_execution':False},indent=2))
