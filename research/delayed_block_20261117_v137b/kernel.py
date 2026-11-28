from research.residue_quotient_20261116_v99b.kernel import basis,apply,preimage

def solve(rows,rhs,width):
 a=list(rows);b=[rhs>>i&1 for i in range(len(rows))];comb=[1<<i for i in range(len(rows))];piv=[];r=0
 for col in range(width):
  k=next((i for i in range(r,len(a))if a[i]>>col&1),None)
  if k is None:continue
  a[r],a[k]=a[k],a[r];b[r],b[k]=b[k],b[r];comb[r],comb[k]=comb[k],comb[r]
  for i in range(len(a)):
   if i!=r and a[i]>>col&1:a[i]^=a[r];b[i]^=b[r];comb[i]^=comb[r]
  piv.append(col);r+=1
 free=[j for j in range(width)if j not in piv];K=[]
 for j in free:K.append((1<<j)^sum(((a[i]>>j)&1)<<p for i,p in enumerate(piv)))
 compatible=not any(a[i]==0 and b[i]for i in range(r,len(a)));particular=sum(b[i]<<p for i,p in enumerate(piv))if compatible else None
 return {'rank':r,'pivots':piv,'free':free,'particular':particular,'kernel':K,'left_null':[comb[i]for i in range(r,len(a))],'rref':a}

def solutions(record):
 if record['particular']is None:return set()
 return {record['particular']^apply(record['kernel'],x)for x in range(1<<len(record['kernel']))}

def complement(K,n):
 vectors=list(K);rank=len(basis(vectors)[0]);assert rank==len(K);U=[]
 for i in range(n):
  v=1<<i;r=len(basis(vectors+[v])[0])
  if r>rank:U.append(v);vectors.append(v);rank=r
 assert len(U)+len(K)==n and rank==n
 return U

def coefficients(columns,v):return [apply([col[j]for col in columns],v)for j in range(6)]
def rhsbits(values):return sum(v<<i for i,v in enumerate(values))
def row_action(rows,v):return rhsbits([(row&v).bit_count()%2 for row in rows])
