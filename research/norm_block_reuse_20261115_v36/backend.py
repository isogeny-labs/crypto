"""Exact binary affine fibers with reusable E2/E4 blocks."""
def xor(values):
 v=0
 for x in values:v^=x
 return v

def echelon(rows,n):
 piv={};bad=False
 for mask,rhs in rows:
  while mask:
   p=mask.bit_length()-1
   if p not in piv:piv[p]=(mask,rhs);break
   m,r=piv[p];mask^=m;rhs^=r
  if not mask and rhs:bad=True
 return piv,bad

def solve(rows,n):
 piv,bad=echelon(rows,n)
 if bad:return None
 def back(v,hom):
  for p,(m,r)in sorted(piv.items()):
   if((m&v).bit_count()%2)^(0 if hom else r):v^=1<<p
  return v
 return dict(particular=back(0,False),basis=[back(1<<i,True)for i in range(n)if i not in piv],rank=len(piv))

def rows_from(cols,rhs,width):return[(sum(((c>>j)&1)<<i for i,c in enumerate(cols)),(rhs>>j)&1)for j in range(width)]
def apply(cols,mask):return xor(c for i,c in enumerate(cols)if mask>>i&1)
def canonical_basis(vectors):
 piv,_=echelon([(x,0)for x in vectors],max((v.bit_length()for v in vectors),default=0))
 for p in sorted(piv):
  for q in sorted(piv):
   if q>p and piv[q][0]>>p&1:piv[q]=(piv[q][0]^piv[p][0],0)
 return tuple(piv[p][0]for p in sorted(piv))
def same(a,b):
 if a is None or b is None:return a is b
 B=canonical_basis(a['basis']);C=canonical_basis(b['basis'])
 return B==C and not apply_residue(a['particular']^b['particular'],B)
def apply_residue(v,B):
 for b in reversed(B):
  if v>>(b.bit_length()-1)&1:v^=b
 return v

class Context:
 def __init__(self,f,z,d,hs,ks,widths):
  self.f=f;self.n=f.degree;self.z=z;self.d=d;self.hs=hs;self.ks=ks;self.widths=widths
  self.q=[f.square(1<<i)for i in range(self.n)];self.zq=[f.mul(z,x)for x in self.q]
  self.hm={h:[f.mul(h,1<<i)for i in range(self.n)]for h in hs};self.km={k:[f.mul(k,1<<i)for i in range(self.n)]for k in ks}
 def parts(self,h,k):
  f=self.f;n=self.n;a,b,c=self.widths;hu=self.hm[h];ku=self.km[k]
  E2=[(s^v)>>a for s,v in zip(self.q,hu)];G=[(s^v)>>b for s,v in zip(self.zq,ku)];J=[x>>b for x in hu];E4=[(s^v)>>c for s,v in zip(self.q,ku)]
  return E2,G,J,E4,(k>>a),(f.mul(self.d,f.square(h))^f.square(k))>>b
 def mono(self,h,k):
  n=self.n;a,b,c=self.widths;E,G,J,D,r2,r3=self.parts(h,k);wa=n-a;wb=n-b;wc=n-c
  cols=[e|(g<<wa)for e,g in zip(E,G)]+[(j<<wa)|(d<<(wa+wb))for j,d in zip(J,D)]
  return solve(rows_from(cols,r2|(r3<<wa),wa+wb+wc),2*n)
 def prepare_blocks(self):
  n=self.n;a,b,c=self.widths;self.cache2={};self.cache4={}
  for h in self.hs:
   cols=[(s^v)>>a for s,v in zip(self.q,self.hm[h])];rows=rows_from(cols,0,n-a)
   # Retain an explicit inverse linear map for arbitrary RHS via row transforms.
   piv={}
   for j,(mask,_)in enumerate(rows):
    comb=1<<j
    while mask:
     p=mask.bit_length()-1
     if p not in piv:piv[p]=(mask,comb);break
     m,v=piv[p];mask^=m;comb^=v
   kernel=solve(rows,n)['basis'];self.cache2[h]=(cols,piv,kernel)
  for k in self.ks:
   cols=[(s^v)>>c for s,v in zip(self.q,self.km[k])];self.cache4[k]=solve(rows_from(cols,0,n-c),n)['basis']
 def block(self,h,k):
  n=self.n;a,b,c=self.widths;E,G,J,D,r2,r3=self.parts(h,k);_,piv,U=self.cache2[h];V=self.cache4[k]
  u0=0
  for p,(m,comb)in sorted(piv.items()):
   if((m&u0).bit_count()%2)^((comb&r2).bit_count()%2):u0^=1<<p
  if apply(E,u0)!=r2:return None
  cols=[apply(G,u)for u in U]+[apply(J,v)for v in V];small=solve(rows_from(cols,r3^apply(G,u0),n-b),len(cols))
  if small is None:return None
  def lift(bits,affine):
   u=(u0 if affine else 0)^xor(v for i,v in enumerate(U)if bits>>i&1);v=xor(v for i,v in enumerate(V)if bits>>(len(U)+i)&1)
   return u|(v<<n)
  return dict(particular=lift(small['particular'],True),basis=[lift(v,False)for v in small['basis']],rank=2*n-len(small['basis']))
