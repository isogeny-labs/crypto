"""Exact parity-projection certificates for zero-avoiding Boolean output sets."""
def equal_one(values):
 piv={}
 for index,vector in enumerate(values):
  rhs=1;comb=1<<index
  while vector:
   j=vector.bit_length()-1
   if j in piv:
    row,b,c=piv[j];vector^=row;rhs^=b;comb^=c
   else:piv[j]=(vector,rhs,comb);break
  if not vector and rhs:return {'solution':None,'odd_dependency':comb}
 answer=0
 for j,(row,rhs,comb)in sorted(piv.items()):
  if rhs^((row&answer).bit_count()&1):answer^=1<<j
 assert all((v&answer).bit_count()%2==1 for v in values)
 return {'solution':answer,'odd_dependency':None}

def rank_two(values,width,all_first=False):
 first=equal_one(values)
 if first['solution']is not None:return [first['solution']],first
 candidates=range(1<<width)if all_first else[1<<j for j in range(width)]
 for a in candidates:
  remaining=[v for v in values if(v&a).bit_count()%2==0];b=equal_one(remaining)
  if b['solution']is not None:
   forms=[a,b['solution']];assert all(any((v&x).bit_count()%2 for x in forms)for v in values);return forms,first
 return None,first

def split_half(values,width):
 partial=0
 for j in range(width):
  future=((1<<width)-1)^((1<<(j+1))-1);scores=[]
  for choice in [0,1]:
   trial=partial^(choice<<j);scores.append(sum(1 if v&future else(0 if(v&trial).bit_count()%2 else 2)for v in values))
  if scores[1]<scores[0]:partial^=1<<j
 assert sum((v&partial).bit_count()%2==0 for v in values)<=len(values)//2
 return partial

def cover(values,width):
 if 0 in values:return None
 remaining=list(values);forms=[]
 while remaining:
  last=equal_one(remaining)
  if last['solution']is not None:forms.append(last['solution']);break
  a=split_half(remaining,width);forms.append(a);remaining=[v for v in remaining if(v&a).bit_count()%2==0]
 assert len(forms)<=len(values).bit_length();return forms

def zeta(coefficients):
 out=list(coefficients);size=len(out);width=(size-1).bit_length();assert size==1<<width
 for j in range(width):
  for x in range(size):
   if x>>j&1:out[x]^=out[x^(1<<j)]
 return out

def boolean_product(a,b):
 result=0;aa=a
 while aa:
  low=aa&-aa;i=low.bit_length()-1;aa^=low;bb=b
  while bb:
   lowb=bb&-bb;j=lowb.bit_length()-1;bb^=lowb;result^=1<<(i|j)
 return result

def compose_or(coefficients,forms):
 out=0
 for form in forms:
  p=sum(((v&form).bit_count()%2)<<i for i,v in enumerate(coefficients));out^=p^boolean_product(out,p)
 return out
