"""Independent whole-cell checker from original field column evaluations."""
def bit(x):return bin(x).count('1')%2
def rank(vectors):
 rows={}
 for v in vectors:
  while v:
   p=(v&-v).bit_length()-1
   if p not in rows:rows[p]=v;break
   v^=rows[p]
 return len(rows)
def verify(C,r):
 assert bit(r['dual']&C.B)==1
 # Compute affine equations directly from original M(0) and M(e_i), independent of transpose tables.
 if not hasattr(C,'_check_basis'):C._check_basis=(C.columns(0),[C.columns(1<<i)for i in range(C.n)])
 zero,table=C._check_basis;eq=[]
 for j,c in enumerate(zero):eq.append((sum(bit(r['dual']&(table[i][j]^c))<<i for i in range(C.n)),bit(r['dual']&c)))
 assert eq==[tuple(v)for v in r['equations']]
 # Independently check consistency using augmented row rank.
 rowrank=rank([v for v,b in eq]);augmented=rank([(v<<1)|b for v,b in eq]);assert rowrank==r['rank']or r['empty_cell']
 if r['empty_cell']:assert augmented>rowrank;return True
 assert augmented==rowrank and r['dimension']==C.n-rowrank
 assert rank(r['directions'])==r['dimension']
 assert all(bit(row&r['base'])==rhs for row,rhs in eq)
 assert all(bit(row&v)==0 for v in r['directions']for row,rhs in eq)
 return True
