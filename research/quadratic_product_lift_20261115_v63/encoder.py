"""Bijective raw-product quadratic lifting of the binary three-source chain."""
def rawmul(a,b):
 out=0
 while b:
  if b&1:out^=a
  b>>=1;a<<=1
 return out

def layout(n,d):
 v=3*d+n;p=2*d-1;w=n+d-1
 return v,p,w,v,v+p,v+p+w

def extend(bits,n,d):
 v,p,w,po,wo,nv=layout(n,d);m=(1<<d)-1
 a=bits&m;b=(bits>>d)&m;c=(bits>>(2*d))&m;u=(bits>>(3*d))&((1<<n)-1)
 return bits|(rawmul(a,b)<<po)|(rawmul(u,c)<<wo)

def encode(n,mod,d,z,B):
 if not(1<=d<=n and mod.bit_length()==n+1 and 0<=z<1<<n and 0<=B<1<<n):raise ValueError('field/domain/constants')
 v,pw,ww,po,wo,nv=layout(n,d);uo=3*d;powers=[1]
 for _ in range(2*(n+d)):
  x=powers[-1]<<1
  if x>>n:x^=mod
  powers.append(x)
 def mul(a,b):
  x=rawmul(a,b)
  while x.bit_length()>n:x^=mod<<(x.bit_length()-n-1)
  return x
 def add(poly,mask,c):
  poly[mask]=poly.get(mask,0)^c
  if not poly[mask]:del poly[mask]
 first={0:B};last={0:B}
 for i in range(d):
  for j in range(n):
   add(first,(1<<i)|(1<<(uo+j)),powers[2*(i+j)])
   add(first,(1<<(d+i))|(1<<(uo+j)),powers[2*(i+j)])
 for i in range(pw):
  add(first,1<<(po+i),powers[2*i])
  for j in range(n):add(first,(1<<(po+i))|(1<<(uo+j)),powers[i+j])
 z2=mul(z,z)
 for i in range(ww):add(last,1<<(wo+i),powers[2*i]^mul(z,powers[i]))
 for i in range(n):add(last,1<<(uo+i),mul(z2,powers[2*i]))
 for i in range(d):add(last,1<<(2*d+i),mul(z2,powers[2*i]))
 rows=[set()for _ in range(2*n)]
 for li,poly in enumerate([first,last]):
  for mask,c in poly.items():
   while c:
    b=c&-c;rows[li*n+b.bit_length()-1].symmetric_difference_update([mask]);c^=b
 for k in range(pw):rows.append({1<<(po+k)}|{(1<<i)|(1<<(d+j))for i in range(d)for j in range(d)if i+j==k})
 for k in range(ww):rows.append({1<<(wo+k)}|{(1<<(uo+i))|(1<<(2*d+j))for i in range(n)for j in range(d)if i+j==k})
 equations=[[[i for i in range(nv)if mask>>i&1]for mask in sorted(row)]for row in rows]
 return {'schema':'neutral_boolean_polynomial_system_v1','instance_id':'ordinary-m3-bijective-quadratic-v63','field':{'kind':'prime','p':2},'domain':[0,1],'n_vars':nv,'equations':equations,'equation_count':len(rows),'term_occurrences':sum(map(len,rows)),'max_degree':max(len(t)for row in equations for t in row),'projection':{'original_low_bits':v,'product_ab':{'offset':po,'width':pw},'product_uc':{'offset':wo,'width':ww},'unique_extension':True},'original_descriptor':{'n':n,'modulus_hex':hex(mod),'d':d,'z_hex':hex(z),'B_hex':hex(B)},'case_role':'training representation diagnostic; not a hard holdout'}

def masks(system):return [[sum(1<<i for i in t)for t in row]for row in system['equations']]
def evaluate(rows,bits):return sum((sum(bits&t==t for t in row)&1)<<i for i,row in enumerate(rows))
