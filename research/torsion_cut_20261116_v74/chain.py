from itertools import product
from collections import defaultdict
import hashlib
from .core import BField,s3,points_and_tags,components

def binary_chain():
 f=BField(5,37);q=32;M=[[f.mul(a,b)for b in range(q)]for a in range(q)];S=[M[a][a]for a in range(q)];I=[0]+[M[a].index(1)for a in range(1,q)]
 N={(a,b):[z for z in range(q)if s3(a,b,z,0,1,f)==0]for a in range(q)for b in range(q)};out=[];witness=None
 for bits in [2,3]:
  D=list(range(1<<bits));pair_image={z for a,b in product(D,repeat=2)for z in N[a,b]};eligible={z for z in range(1,q)if z not in D and I[z]not in D and z not in pair_image and I[z]not in pair_image};poly={z:set()for z in eligible}
  for a,b,c,d,e in product(D,repeat=5):
   for u in N[a,b]:
    for v in N[u,c]:
     for w in N[v,d]:
      for z in N[w,e]:
       if z in eligible:poly[z].add((a,b,c,d,e,u,v,w))
  for A in [0,1]:
   pts,tags=points_and_tags(f,q,A,1);targets=sorted(z for z in eligible if tags[z]=='R');R=[(x,y)for x in D for y in pts[x]]
   def add(P,Q):
    if P is None:return Q
    if Q is None:return P
    x,y=P;u,v=Q
    if x==u:
     if y^v==x:return None
     assert x and y==v;slope=x^M[y][I[x]]
    else:slope=M[y^v][I[x^u]]
    w=S[slope]^slope^x^u^A;return(w,M[slope][x^w]^w^y)
   pairs=defaultdict(list)
   for P1,P2 in product(R,repeat=2):
    U=add(P1,P2)
    if U is not None:pairs[U].append((P1,P2))
   for z in targets:
    Q=(z,pts[z][0]);joined=set()
    for P3,P4,P5 in product(R,repeat=3):
     V=add(add(P3,P4),P5);negV=None if V is None else(V[0],V[0]^V[1]);U=add(Q,negV)
     for P1,P2 in pairs.get(U,[]):
      W=add(U,P3)
      if W is None:continue
      Z=add(W,P4)
      if Z is None:continue
      assert add(Z,P5)==Q;joined.add((P1[0],P2[0],P3[0],P4[0],P5[0],U[0],W[0],Z[0]))
    mixed=0
    for t in poly[z]:
     values=t+(z,);cs=components(values,tags)
     for comp in cs:assert len({tags[values[i]]for i in comp})==1
     assert all(tags[values[i]]=='R'for comp in cs if 8 in comp for i in comp)
     if any(tags[x]=='A'for x in t):
      mixed+=1
      if witness is None:witness={'A':A,'domain_bits':bits,'target_x':z,'root':t,'torsion_vertices':[i for i,x in enumerate(values)if tags[x]=='T'],'components':cs}
    if A==0:assert joined==poly[z] and mixed==0
    else:assert joined<=poly[z]
    out.append({'A':A,'domain_bits':bits,'target_x':z,'guard_G1_G2':True,'trace_A':A,'rational_join_roots':len(joined),'original_roots':len(poly[z]),'nonrational_roots':mixed,'rational_join_equal':joined==poly[z],'rootset_sha256':hashlib.sha256(repr(sorted(poly[z])).encode()).hexdigest(),'pair_image_size':len(pairs),'pair_fiber_entries':sum(map(len,pairs.values())),'rational_source_points':len(R)})
 return {'cases':out,'positive_A0_cases':sum(r['A']==0 and r['original_roots']>0 for r in out),'control_nonrational_witness':witness}
