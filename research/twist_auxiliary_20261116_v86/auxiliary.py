from itertools import product
from collections import defaultdict
import json,time,hashlib
from .common import P,finish
from .algebra import Field,s3
from .groups import points,add,target,phi
START=time.monotonic()
def main():
 out={'point_memberships':0,'homomorphism_pairs':0,'source_relation_tuples':0,'S3_forward_checks':0,'cases':[],'auxiliary_false_positive':None,'qualified_speedup':None};error=None
 try:
  d=json.loads((P/'DATA.json').read_text())
  for row in d['fields']:
   f=Field(row['p'],row['mod']);q=f.q;binary=f.p==2
   for a,b in([(A,B)for A in[0,1]for B in([1,2]if f.n==3 else[1])]if binary else[(0,1)]):
    pts=points(f,a,b,binary);a2,b2=target(a,b,f,binary);pts2=set(points(f,a2,b2,binary));images={T:phi(T,f,a,b,binary)for T in pts};assert all(v in pts2 for v in images.values());out['point_memberships']+=len(pts);kernel=[T for T in pts if images[T]is None];assert len(kernel)==2 and len(set(images.values()))*2==len(pts)
    for U,V in product(pts,repeat=2):assert images[add(U,V,f,a,b,binary)]==add(images[U],images[V],f,a2,b2,binary);out['homomorphism_pairs']+=1
    R=[T for T in pts if T is not None and T[0]<min(q,d['source_x_limit'])];fibers=defaultdict(list)
    for T in R:fibers[images[T]].append(T)
    original=defaultdict(set);recovered=defaultdict(set)
    for xs in product(R,repeat=3):original[add(add(xs[0],xs[1],f,a,b,binary),xs[2],f,a,b,binary)].add(xs)
    for us in product(fibers,repeat=3):
     S=add(add(us[0],us[1],f,a2,b2,binary),us[2],f,a2,b2,binary)
     for xs in product(*(fibers[u]for u in us)):
      T=add(add(xs[0],xs[1],f,a,b,binary),xs[2],f,a,b,binary);assert images[T]==S;recovered[T].add(xs)
    assert dict(original)==dict(recovered);count=sum(map(len,original.values()));out['source_relation_tuples']+=count;pole=0
    if binary:
     U={x:f.add(x,f.mul(b2,f.inv(x)))for x in range(1,q)}
     for xs in product(range(q),repeat=3):
      v=s3(*xs,a,b,f)
      if not v and 0 in xs:pole+=1
      if 0 in xs:continue
      ys=tuple(U[x]for x in xs);w=s3(*ys,a2,b2,f)
      if not v:assert not w;out['S3_forward_checks']+=1
      elif not w and out['auxiliary_false_positive']is None:out['auxiliary_false_positive']={'field':row['name'],'A':a,'B':b,'source_x':xs,'auxiliary_x':ys,'source_residual':v}
    out['cases'].append({'field':row['name'],'binary':binary,'a':a,'b':b,'aux_a':a2,'aux_b':b2,'curve_points':len(pts),'aux_curve_points':len(pts2),'image_points':len(set(images.values())),'source_points':len(R),'source_x_count':len({T[0]for T in R}),'image_x_count_including_infinity':len({None if images[T]is None else images[T][0]for T in R}),'kernel_size':2,'kernel_hit_S3_triples':pole,'all_source_triple_relations':count,'source_triples_sha256':hashlib.sha256(repr(sorted(set().union(*original.values()))).encode()).hexdigest(),'root_transfer_equal':True});assert time.monotonic()-START<65
  assert out['auxiliary_false_positive'];orders=[]
  for A,t1 in [(0,-1),(1,1)]:
   t0=2;t=t1
   for k in range(2,84):t0,t=t,t1*t-2*t0
   N=(1<<83)+1-t;orders.append({'A':A,'q':1<<83,'derived_curve_order':N,'two_adic_valuation':(N&-N).bit_length()-1})
  assert[ r['two_adic_valuation']for r in orders]==[2,1];out['actual_Koblitz_orders']=orders;out['complete']=True
 except Exception as e:error=repr(e)
 finish(out,error,START,'Explicit binary/odd2isogeny transfer,complete fibers,all small source relations and false-positive controls.','Test kernel-orbit factor bases with matching size,density and all lift costs;preserve current prefix-domain constraints and do not infer compression from isogeny degree.')
if __name__=='__main__':main()
