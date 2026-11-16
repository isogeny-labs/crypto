"""v-first exact fibers, with sparse coupled fallback for deficient v-rank."""
from research.norm_sparse_lift_20261115_v36.backend import Context as Prior,apply
from research.norm_block_reuse_20261115_v36 import backend as b
from research.inversion_free_full_20261115_v35 import compiler
class Context(Prior):
 def raw_block(self,ctx,h,k):
  n=self.n;a,c,d=self.widths;E,piv,U=self.cache2[h];u0=0;rhs=k>>a
  for p,(m,comb)in sorted(piv.items()):
   if((m&u0).bit_count()%2)^((comb&rhs).bit_count()%2):u0^=1<<p
  if apply(E,u0)!=rhs:return None
  G=[(s^v)>>c for s,v in zip(self.zq,self.km[k])];J=[v>>c for v in self.hm[h]];D=[(s^v)>>d for s,v in zip(self.q,self.km[k])];w=n-c;cols=[j|(v<<w)for j,v in zip(J,D)];rows=b.rows_from(cols,0,2*n-c-d);pv={}
  for i,(mask,_)in enumerate(rows):
   comb=1<<i
   while mask:
    p=mask.bit_length()-1
    if p not in pv:pv[p]=(mask,comb);break
    m,v=pv[p];mask^=m;comb^=v
  if len(pv)!=len(rows):return super().raw_block(ctx,h,k)
  def lift(rhs):
   v=0
   for p,(m,comb)in sorted(pv.items()):
    if((m&v).bit_count()%2)^((comb&rhs).bit_count()%2):v^=1<<p
   return v
  vk=[]
  for j in range(n):
   if j in pv:continue
   v=1<<j
   for p,(m,_)in sorted(pv.items()):
    if(m&v).bit_count()%2:v^=1<<p
   vk.append(v)
  r3=((self.f.mul(self.d,self.f.square(h))^self.f.square(k))>>c)^apply(G,u0);v0=lift(r3);r,s=compiler.to_original(self.f,ctx,h,k,u0,v0)
  basis=[u|((lift(apply(G,u))^self.f.mul(self.z,u))<<n)for u in U]+[v<<n for v in vk]
  return dict(particular=r|(s<<n),basis=basis,rank=2*n-len(basis))
