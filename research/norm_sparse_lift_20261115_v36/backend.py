"""Exact set-bit composition and raw-coordinate fused fiber lift."""
from research.norm_block_reuse_20261115_v36 import backend as old
from research.inversion_free_full_20261115_v35 import compiler

def apply(cols,bits):
 v=0
 while bits:
  low=bits&-bits;v^=cols[low.bit_length()-1];bits^=low
 return v
class Context(old.Context):
 def raw_block(self,ctx,h,k):
  n=self.n;a,b,c=self.widths;E,piv,U=self.cache2[h];V=self.cache4[k];r2=k>>a;u0=0
  for p,(m,comb)in sorted(piv.items()):
   if((m&u0).bit_count()%2)^((comb&r2).bit_count()%2):u0^=1<<p
  if apply(E,u0)!=r2:return None
  G=[(s^v)>>b for s,v in zip(self.zq,self.km[k])];J=[v>>b for v in self.hm[h]]
  cols=[apply(G,u)for u in U]+[apply(J,v)for v in V];rhs=(self.f.mul(self.d,self.f.square(h))^self.f.square(k))>>b
  small=old.solve(old.rows_from(cols,rhs^apply(G,u0),n-b),len(cols))
  if small is None:return None
  lift=[u|(self.f.mul(self.z,u)<<n)for u in U]+[v<<n for v in V]
  r,s=compiler.to_original(self.f,ctx,h,k,u0,0);base=r|(s<<n)
  return dict(particular=base^apply(lift,small['particular']),basis=[apply(lift,v)for v in small['basis']],rank=2*n-len(small['basis']))
