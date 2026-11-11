from functools import lru_cache
# F_n=psi_n for odd n and psi_n/(2y) for even n. No divisions are used.
def normalized(n,x,a,b,p):
 ops={'dual_additions':0,'dual_multiplications':0}
 def C(v):return v%p,0
 def A(u,v):ops['dual_additions']+=1;return (u[0]+v[0])%p,(u[1]+v[1])%p
 def N(u):return (-u[0])%p,(-u[1])%p
 def S(u,v):return A(u,N(v))
 def M(u,v):ops['dual_multiplications']+=1;return u[0]*v[0]%p,(u[0]*v[1]+u[1]*v[0])%p
 def powj(u,k):
  out=C(1)
  for bit in bin(k)[2:]:
   out=M(out,out)
   if bit=='1':out=M(out,u)
  return out
 X=x%p,1;X2=M(X,X);X3=M(X2,X);X4=M(X2,X2);X6=M(X3,X3)
 f=A(A(X3,M(C(a),X)),C(b));fourY=M(C(16),M(f,f))
 f3=S(A(A(M(C(3),X4),M(C(6*a),X2)),M(C(12*b),X)),C(a*a))
 f4=M(C(2),S(S(S(A(A(X6,M(C(5*a),X4)),M(C(20*b),X3)),M(C(5*a*a),X2)),M(C(4*a*b),X)),C(8*b*b+a*a*a)))
 base=[C(0),C(1),C(1),f3,f4]
 @lru_cache(None)
 def F(k):
  if k<5:return base[k]
  m=k//2
  if k%2:
   u=M(F(m+2),powj(F(m),3));v=M(F(m-1),powj(F(m+1),3))
   return S(M(fourY,u),v) if m%2==0 else S(u,M(fourY,v))
  return M(F(m),S(M(F(m+2),powj(F(m-1),2)),M(F(m-2),powj(F(m+1),2))))
 result=F(n);ops['memoized_indices']=F.cache_info().currsize
 return result,ops
