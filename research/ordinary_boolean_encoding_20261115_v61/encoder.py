"""Exact Boolean ANF for the two original binary S3 equations."""
def encode(n,mod,d,z,B,A=0,y=0):
 if not(1<=d<=n and mod.bit_length()==n+1 and 0<=z<1<<n and 0<B<1<<n):raise ValueError('field/domain/constants')
 powers=[1]
 for _ in range(2*(n+d)):
  v=powers[-1]<<1
  if v>>n:v^=mod
  powers.append(v)
 def mul(a,b):
  v=0
  while b:
   if b&1:v^=a
   b>>=1;a<<=1
   if a>>n:a^=mod
  return v
 def add(poly,mask,c):
  poly[mask]=poly.get(mask,0)^c
  if not poly[mask]:del poly[mask]
 p={0:B};q={0:B};uoff=3*d
 for i in range(d):
  for j in range(d):add(p,(1<<i)|(1<<(d+j)),powers[2*(i+j)])
  for j in range(n):
   add(p,(1<<i)|(1<<(uoff+j)),powers[2*(i+j)]);add(p,(1<<(d+i))|(1<<(uoff+j)),powers[2*(i+j)])
   for k in range(d):add(p,(1<<i)|(1<<(d+k))|(1<<(uoff+j)),powers[i+k+j])
 z2=mul(z,z)
 for i in range(n):
  add(q,1<<(uoff+i),mul(z2,powers[2*i]))
  for j in range(d):add(q,(1<<(uoff+i))|(1<<(2*d+j)),powers[2*(i+j)]^mul(z,powers[i+j]))
 for j in range(d):add(q,1<<(2*d+j),mul(z2,powers[2*j]))
 equations=[[]for _ in range(2*n)]
 for link,poly in enumerate([p,q]):
  for mask,c in sorted(poly.items()):
   indices=[i for i in range(3*d+n)if mask>>i&1]
   while c:
    low=c&-c;equations[link*n+low.bit_length()-1].append(indices);c^=low
 return {'schema':'neutral_boolean_polynomial_system_v1','instance_id':'ordinary-m3-exact-encoding-v61','field':{'kind':'prime','p':2},'domain':[0,1],'n_vars':3*d+n,'equations':equations,'equation_count':2*n,'term_occurrences':sum(map(len,equations)),'max_degree':3,'summands':3,'source_dimension':d,'source_bits':3*d,'source_basis_hex':[hex(1<<i)for i in range(d)],'source_offset_hex':'0x0','extension_field':{'kind':'binary','characteristic':2,'degree':n,'modulus_hex':hex(mod),'low_terms':[i for i in range(n)if mod>>i&1]},'variable_layout':[{'kind':'source_x','index':i,'offset':i*d,'width':d}for i in range(3)]+[{'kind':'auxiliary_x','index':0,'offset':3*d,'width':n}],'coordinate_equation_order':'S3 link index then polynomial-basis field coordinate, least significant bit first','chain_links':[['source_x:0','source_x:1','auxiliary_x:0'],['auxiliary_x:0','source_x:2','target_x']],'curve':{'model':'y^2+x*y=x^3+a*x^2+b','a_hex':hex(A),'b_hex':hex(B)},'target':{'x_hex':hex(z),'y_hex':hex(y)},'source_decomposition_planted':True,'case_role':'training exact-contract pilot; no hard holdout claim'}
def values(bits,n,d):return tuple((bits>>(d*i))&((1<<d)-1)for i in range(3))+((bits>>(3*d))&((1<<n)-1),)
def pack(v,n,d):return v[0]|(v[1]<<d)|(v[2]<<(2*d))|(v[3]<<(3*d))
def evaluate(equations,bits):
 out=0
 for i,row in enumerate(equations):
  parity=0
  for term in row:parity^=all(bits>>j&1 for j in term)
  out|=parity<<i
 return out
