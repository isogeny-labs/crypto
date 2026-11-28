# Independent deterministic quotient-ring fixture arithmetic; no curve/corpus run.
import itertools,json
from pathlib import Path

def mul(a,b,n,mod):
 p=0
 while b:
  if b&1:p^=a
  a<<=1;b>>=1
 while p.bit_length()>n:p^=mod<<(p.bit_length()-n-1)
 return p

def det(x,b,n,mod):
 def coeff(x,y):
  xy=mul(x,y,n,mod)
  return [mul(x^y,x^y,n,mod),xy,mul(xy,xy,n,mod)^b]
 a,c,d=coeff(x[0],x[1]);e,f,g=coeff(x[2],x[3]);matrix=[[a,c,d,0],[0,a,c,d],[e,f,g,0],[0,e,f,g]]
 out=0
 for order in itertools.permutations(range(4)):
  product=1
  for row,col in enumerate(order):product=mul(product,matrix[row][col],n,mod)
  out^=product
 return out
cases=[]
for n,low,b,target,codes in [(2,[0,1],2,3,[0,1]),(61,list(range(61)),(1<<60)+0x35,(1<<59)+0x123,[0x1234,(1<<60)+9])]:
 mod=(1<<n)|sum(1<<k for k in low)
 values=[det([codes[a&1],codes[(a>>1)&1],codes[(a>>2)&1],target],b,n,mod) for a in range(8)]
 # Coefficient of maskS is XOR of evaluations at every subset ofS.
 masks=[]
 for bit in range(n):
  mask=0
  for s in range(8):
   coefficient=0
   for subset in range(8):
    if subset&s==subset:coefficient^=(values[subset]>>bit)&1
   mask|=coefficient<<s
  masks.append(mask)
 cases.append(dict(n=n,low_terms=low,b=b,target=target,codes=codes,values=values,anf_support_masks=masks,scope='quotient algebra only; dense61 modulus not claimed irreducible'))
p=Path('/home/ec2-user/ml-dataset-resume-20261029/binary-resultant-boundary-reference.json');p.write_text(json.dumps(cases,indent=2)+'\n');print(json.dumps(cases))
