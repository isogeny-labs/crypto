from research.orbit_roots_20261116_v70.solver import Context,gcd,shift_roots
from research.image_cells_20261115_v67.algebra import rem,binary_tower
from research.translation_orbits_20261115_v69.algebra import orbit_polynomial,literal_s3

def solve(inst):
 ctx=Context(inst['n'],inst['modulus']);f=ctx.f;d=inst['shift_dimension'];beta=inst['beta'];z=inst['z'];B=inst['B'];R=orbit_polynomial(beta,z,inst['A'],B,f);q=[0,1]
 for _ in range(f.n):
  squared=[0]*(2*len(q)-1)
  for i,c in enumerate(q):squared[2*i]=f.sq(c)
  q=rem(squared,R,f)
 if len(q)<2:q +=[0]*(2-len(q))
 q[1]^=1;G=gcd(R,q,f)
 if len(G)<=1:shifts=[]
 elif len(G)==2:shifts=[G[0]]if G[0]<(1<<d)else[]
 else:shifts=shift_roots(G,binary_tower(f,d),f)
 roots=[]
 for t in shifts:
  a,b,c=[x^t for x in beta]
  for u in ctx.common(a,b,c,z,B):
   assert not literal_s3(a,b,u,inst['A'],B,f)and not literal_s3(u,c,z,inst['A'],B,f);roots.append([t,u])
 return {'complete':True,'orbit_polynomial':R,'roots':sorted(roots),'candidate_shifts':shifts,'source_family':'fixed ordered common-shift offsets','original_tuples':[[beta[0]^t,beta[1]^t,beta[2]^t,u]for t,u in sorted(roots)]}
