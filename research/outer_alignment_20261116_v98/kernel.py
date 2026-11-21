from research.separator_images_20261116_v91.kernel import Solver
from research.binary_rebracket_20261116_v96.kernel import solve as block
from research.affine_dual_cells_20261116_v97.kernel import Cells,parity
from research.affine_dual_cells_20261116_v97.checker import verify

def family(n,mod,d):
 C=Cells(n,mod,d,1);f=C.S.f;trace=sum(f.trace(1<<i)<<i for i in range(n));out=[]
 for degree in range(3,min(6,n-1)+1):
  for middle in range(1<<(degree-1)):
   Q=(1<<degree)|(middle<<1)|1
   for r in[1,2]:
    if degree<=r:continue
    g=f.mul(1<<r,f.inv(Q))
    if not parity(g&trace):continue
    v=f.sq(g);dual=0
    for i in range(n):dual|=parity(v&trace)<<i;v=C.S.timesT(v)
    cell=C.cell(dual)
    if cell['empty_cell']:continue
    assert verify(C,cell);out.append(cell)
 return C,out

def covered(w,cells):return any(all(parity(a&w)==b for a,b in cell['equations'])for cell in cells)

def zero_pair(F,a,b,d):
 if not a or not b:return False
 f=F.f;return f.inv(a)<1<<d and f.inv(b)<1<<d # B=1

def solve(spec,method):
 n,mod,d,T=[spec[k]for k in['n','mod','d','T']];stats={'quadratic_calls':0,'block_calls':0,'no_w_rejects':0,'cell_rejects':0};assert d<=5,'full-chain outer enumeration cap'
 F=Solver(n,mod,n);D=range(1<<d);out=set();complete=True
 def quad(a,b):stats['quadratic_calls']+=1;return F.solve(*F.coefficients(a,b,1),'stream')
 if method=='join':
  table={}
  for x5 in D:
   for u3 in quad(x5,T):
    for x4 in D:
     for u2 in quad(u3,x4):table.setdefault(u2,[]).append((x4,x5,u3))
  for x1 in D:
   for x2 in D:
    for u1 in quad(x1,x2):
     for x3 in D:
      for u2 in quad(u1,x3):
       for x4,x5,u3 in table.get(u2,[]):out.add((x1,x2,x3,x4,x5,u1,u2,u3))
 elif method in['block','filtered']:
  cells=family(n,mod,d)[1]if method=='filtered'else[];stats['cells']=len(cells)
  right=[(x5,b)for x5 in D for b in quad(x5,T)]
  for x1 in D:
   for x2 in D:
    for a in quad(x1,x2):
     for x5,b in right:
      if method=='filtered'and a!=b and not zero_pair(F,a,b,d):
       ws=quad(a,b)
       if not ws:stats['no_w_rejects']+=1;continue
       if all(covered(w,cells)for w in ws):stats['cell_rejects']+=1;continue
      stats['block_calls']+=1;r=block({'n':n,'mod':mod,'d':d,'a':a,'b':b,'B':1,'source_cap_bits':12,'diagonal_cap_bits':12,'pair_cap_bits':12},'rebracket');complete &=r['complete']
      for x3,x4,u2 in r['roots']:out.add((x1,x2,x3,x4,x5,a,u2,b))
 else:raise ValueError('method')
 return{'complete':complete,'roots':sorted(out),'exact_root_count':len(out)if complete else None,'stats':stats}
