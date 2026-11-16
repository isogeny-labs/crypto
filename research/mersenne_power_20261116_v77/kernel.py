from research.prime_orbit_roots_20261116_v71.solver import power as generic_power
from research.image_cells_20261115_v67.algebra import rem,pmul
COUNTS={'squarings':0,'multiplications':0,'chain_calls':0,'fallback_calls':0}
def reset():
 for k in COUNTS:COUNTS[k]=0

def mersenne(a,k,H,f):
 assert k>=0 and len(H)>=2
 if not k:return [1]
 a=rem(a,H,f);r=a;length=1
 for bit in bin(k)[3:]:
  t=r
  for _ in range(length):t=rem(pmul(t,t,f),H,f);COUNTS['squarings']+=1
  r=rem(pmul(r,t,f),H,f);COUNTS['multiplications']+=1;length*=2
  if bit=='1':
   r=rem(pmul(r,r,f),H,f);COUNTS['squarings']+=1;r=rem(pmul(r,a,f),H,f);COUNTS['multiplications']+=1;length+=1
 assert length==k
 return r

def power(a,n,H,f):
 assert n>=0
 if n&(n+1):COUNTS['fallback_calls']+=1;return generic_power(a,n,H,f)
 COUNTS['chain_calls']+=1
 return mersenne(a,(n+1).bit_length()-1,H,f)
