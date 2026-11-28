from research.residue_quotient_20261116_v99b.kernel import fmul,rem,pmul

class Circuit:
 def __init__(self,n):self.n=n;self.nodes=[('false',),('true',)]+[('input',i)for i in range(n)];self.cache={}
 def op(self,kind,a,b=None):
  if kind=='xor':
   if a==b:return 0
   if a==0:return b
   if b==0:return a
   if a==1:return self.op('not',b)
   if b==1:return self.op('not',a)
  if kind=='and':
   if a==0 or b==0:return 0
   if a==1:return b
   if b==1 or a==b:return a
  if kind=='not':
   if a in[0,1]:return a^1
   if self.nodes[a][0]=='not':return self.nodes[a][1]
  if b is not None and a>b:a,b=b,a
  key=(kind,a)if b is None else(kind,a,b)
  if key not in self.cache:
   assert len(self.nodes)<500000,'gate cap';self.cache[key]=len(self.nodes);self.nodes.append(key)
  return self.cache[key]
 def xor(self,a,b):return self.op('xor',a,b)
 def evaluate(self,value):
  out=[0,1]+[(value>>i)&1 for i in range(self.n)]
  for node in self.nodes[2+self.n:]:
   kind,a,*rest=node;out.append((out[a]^out[rest[0]])if kind=='xor'else(out[a]&out[rest[0]])if kind=='and'else out[a]^1)
  return out

class BF:
 def __init__(self,s,m,C):self.s=s;self.m=m;self.C=C
 def const(self,x):return[(x>>i)&1 for i in range(self.s)]
 def add(self,a,b):return[self.C.xor(x,y)for x,y in zip(a,b)]
 def mul(self,a,b):
  v=[0]*(2*self.s-1)
  for i,x in enumerate(a):
   for j,y in enumerate(b):v[i+j]=self.C.xor(v[i+j],self.C.op('and',x,y))
  for k in range(len(v)-1,self.s-1,-1):
   for i in range(self.s):
    if self.m>>i&1:v[k-self.s+i]=self.C.xor(v[k-self.s+i],v[k])
  return v[:self.s]
 def square(self,a):
  out=self.const(0)
  for i,x in enumerate(a):
   v=fmul(1<<i,1<<i,self.m)
   for j in range(self.s):
    if v>>j&1:out[j]=self.C.xor(out[j],x)
  return out

def square_mod(v,H,f):
 out=[f.const(0)for _ in range(9)]
 for i,a in enumerate(v):out[2*i]=f.square(a)
 for k in range(8,4,-1):
  c=out[k]
  if not any(c):continue
  for i in range(5):out[k-5+i]=f.add(out[k-5+i],f.mul(c,H[i]))
 return out[:5]

def split_output(H,f):
 v=[f.const(0),f.const(1)]+[f.const(0)for _ in range(3)]
 for _ in range(f.s):v=square_mod(v,H,f)
 v[1]=f.add(v[1],f.const(1))
 for _ in range(3):v=square_mod(v,H,f)
 return v

def compile_actual(base,hc,M,fields):
 C=Circuit(76);outputs=[]
 for s,m in fields:
  f=BF(s,m,C);H=[]
  for k in range(5):
   b=rem(base[k],m);coeff=[rem(v[k],m)for v in hc]+([rem(pmul(1<<i,M),m)for i in range(3)]if k==0 else[0]*3);bits=[]
   for bit in range(s):
    v=(b>>bit)&1
    for j,a in enumerate(coeff):
     if a>>bit&1:v=C.xor(v,2+j)
    bits.append(v)
   H.append(bits)
  outputs.append({'s':s,'m':m,'wires':split_output(H,f)})
 return C,outputs

def cnf(C,outputs):
 clauses=[]
 for y,node in enumerate(C.nodes):
  kind=node[0]
  if kind in['false','true','input']:continue
  a=node[1]-1;v=y-1
  if kind=='not':clauses.extend([(a,v),(-a,-v)]);continue
  b=node[2]-1
  if kind=='and':clauses.extend([(-a,-b,v),(a,-v),(b,-v)])
  else:clauses.extend([(-a,-b,-v),(a,b,-v),(a,-b,v),(-a,b,v)])
 for row in outputs:
  for field in row['wires']:
   for wire in field:
    if wire==1:clauses.append(())
    elif wire:clauses.append((-(wire-1),))
 return clauses
