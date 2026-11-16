import json
from pathlib import Path
from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.residue_quotient_20261116_v99b.kernel import pmul,rem
from research.local_nilpotency_20261116_v100.kernel import Arithmetic,add,square,power,trim

def matrix(path):return [[int(x,16)for x in s.split()]for s in Path(path).read_text().splitlines()[1:]]
def decode(rows,theta):
 H=list(rows[0])
 for j,v in enumerate(rows[1:]):
  if theta>>j&1:H=[a^b for a,b in zip(H,v)]
 return H
def valid_guard(H,q):
 u,v,w=(H[3]>>34)&1,(H[2]>>51)&1,(H[1]>>68)&1
 return (u,v,w)in[(0,0,0),(1,1,0),(0,0,1)]and(q>>2)==w and(w or not(q>>1&1))
def guarded(graph_path,rows):
 g=json.loads(Path(graph_path).read_text());C=Circuit(76);C.nodes=[tuple(n)for n in g['nodes']];C.cache={v:i for i,v in enumerate(C.nodes)if v[0]in['and','xor','not']};old=len(C.nodes)
 def flag(k,b):
  v=(rows[0][k]>>b)&1
  for j,h in enumerate(rows[1:]):
   if h[k]>>b&1:v=C.xor(v,2+j)
  return v
 u,v,w=flag(3,34),flag(2,51),flag(1,68);extra=[C.xor(u,v),C.op('and',u,w),C.xor(77,w),C.op('and',76,C.op('not',w))];outputs=g['outputs']+[{'guard':True,'wires':[[x]for x in extra]}]
 delta={'base_node_count':old,'new_nodes':C.nodes[old:],'guard_outputs':outputs[-1],'new_variable_count':len(C.nodes)-2}
 return C,outputs,delta
def source_product(xs):
 H=[1]
 for x in xs:
  out=[0]*(len(H)+1)
  for i,a in enumerate(H):out[i]^=pmul(a,x);out[i+1]^=a
  H=out
 return H
def divrem(a,m):
 q=0
 while a.bit_length()>=m.bit_length():j=a.bit_length()-m.bit_length();q^=1<<j;a^=m<<j
 return q,a
def prefix_remainder(H,n,mod,d):
 f=Arithmetic(2,mod);cs=[1]
 for j in range(d):
  b=1<<j;v=0;z=b
  for c in cs:v^=f.mul(c,z);z=f.sq(z)
  assert v;new=[0]*(len(cs)+1)
  for i,c in enumerate(cs):new[i]^=f.mul(v,c);new[i+1]^=f.sq(c)
  cs=new
 R=[];X=[0,1]
 for c in cs:R=add(R,[f.mul(c,v)for v in X],f);X=square(X,H,f)
 return power(R,8,H,f)
