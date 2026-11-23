from collections import Counter

def pmul(a,b):
 out=0
 while b:
  if b&1:out^=a
  a<<=1;b>>=1
 return out
def rem(a,m):
 while a.bit_length()>=m.bit_length():a^=m<<(a.bit_length()-m.bit_length())
 return a
def fmul(a,b,m):return rem(pmul(a,b),m)
def poly(xs,m):
 c=[1]
 for x in xs:
  out=[0]*(len(c)+1)
  for i,a in enumerate(c):out[i]^=fmul(a,x,m);out[i+1]^=a
  c=out
 return c
def code(v,s,m):return sum(rem(a,m)<<(s*i)for i,a in enumerate(v[:4]))
def apply(cols,v):
 out=0
 while v:
  b=v&-v;out^=cols[b.bit_length()-1];v^=b
 return out
def basis(cols):
 piv={};kernel=[]
 for j,v in enumerate(cols):
  w=1<<j
  while v:
   p=v.bit_length()-1
   if p not in piv:piv[p]=(v,w);break
   a,b=piv[p];v^=a;w^=b
  if not v:kernel.append(w)
 return piv,kernel
def preimage(v,piv):
 out=0
 while v:
  p=v.bit_length()-1
  if p not in piv:raise ValueError('not surjective')
  a,b=piv[p];v^=a;out^=b
 return out
def transform(a):
 a=list(a);step=1
 while step<len(a):
  for start in range(0,len(a),2*step):
   for i in range(start,start+step):x,y=a[i],a[i+step];a[i]=x+y;a[i+step]=x-y
  step*=2
 return a

def count(E,ebase,widths,tables,F,fbase,last,algorithm,max_quotient_bits=16):
 bits=sum(widths);piv,K=basis(E);assert len(piv)==bits
 R=[preimage(1<<i,piv)for i in range(bits)];FR=[apply(F,v)for v in R];FK=[apply(F,v)for v in K];kp,_=basis(FK);outbits=max([v.bit_length()for v in F]+[fbase.bit_length(),max(last,default=0).bit_length()]);free=[i for i in range(outbits)if i not in kp];assert len(free)<=max_quotient_bits,'quotient cap'
 def quotient(v):
  for i in sorted(kp,reverse=True):
   if v>>i&1:v^=kp[i][0]
  return sum(((v>>i)&1)<<j for j,i in enumerate(free))
 size=1<<len(free);hist=[];offset=0
 for width,table in zip(widths,tables):
  h=Counter(quotient(apply(FR,v<<offset))for v in table);hist.append(h);offset+=width
 if algorithm=='dp':
  distribution=[0]*size;distribution[0]=1;state_counts=[]
  for h in hist:
   new=[0]*size
   for a,ct in enumerate(distribution):
    if ct:
     for b,n in h.items():new[a^b]+=ct*n
   distribution=new;state_counts.append(sum(bool(v)for v in new))
 elif algorithm=='walsh':
  product=[1]*size
  for h in hist:
   a=[h.get(i,0)for i in range(size)];t=transform(a);product=[x*y for x,y in zip(product,t)]
  transformed=transform(product);assert all(x%size==0 for x in transformed);distribution=[x//size for x in transformed];state_counts=None
 else:raise ValueError('algorithm')
 assert min(distribution)>=0 and sum(distribution)==__import__('math').prod(map(len,tables))
 weights=Counter(quotient(v)for v in last);shift=quotient(fbase^apply(FR,ebase));multiplicity=1<<(len(K)-len(kp));total=sum(n*weights[a^shift]for a,n in enumerate(distribution))*multiplicity
 return {'exact_necessary_parameter_count':total,'distribution':distribution,'first_table_product':sum(distribution),'kernel_bits':len(K),'last_kernel_rank':len(kp),'quotient_bits':len(free),'compressed_states':sum(bool(v)for v in distribution),'state_counts':state_counts,'original_complete':False,'original_exact_root_count':None}
