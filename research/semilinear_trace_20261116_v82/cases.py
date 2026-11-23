import hashlib
from research.image_cells_20261115_v67.algebra import BField,roots_poly

def fields(d):return[(BField(n,M),'GF2^'+str(n))for n,M in d['fields']]
def cases(f,d,name):
 for m in d['degrees']:
  def rand(tag):return int.from_bytes(hashlib.shake_256((str(d['seed'])+name+':'+str(m)+':'+tag).encode()).digest((f.n+7)//8),'little')%f.q
  for kind in ['split','repeated','arbitrary']:
   H=roots_poly([(i+1)%f.q for i in range(m)],f)if kind=='split'else roots_poly([1]*m,f)if kind=='repeated'else[rand('H'+str(i))for i in range(m)]+[1]
   for label,t in [('zero',[]),('constant',[rand('c')]),('dense',[rand('t'+str(i))for i in range(m)])]:yield m,kind,label,H,t
