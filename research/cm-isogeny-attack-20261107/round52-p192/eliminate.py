from pathlib import Path
from itertools import product
import json
root=Path(__file__).resolve().parent;c=json.loads((root/'certificate.json').read_text());toy=c['toy'];p=toy['p']
points=sorted({tuple(v for P in row['output'] for v in P) for row in toy['rows']});N=len(points)
def rank_columns(cols):
 if not cols:return 0
 A=[list(r) for r in zip(*cols)];k=0
 for j in range(len(cols)):
  i=next((i for i in range(k,N) if A[i][j]%p),None)
  if i is None:continue
  A[k],A[i]=A[i],A[k];z=pow(A[k][j],-1,p);A[k]=[z*x%p for x in A[k]]
  for i in range(k+1,N):
   z=A[i][j];A[i]=[(x-z*y)%p for x,y in zip(A[i],A[k])]
  k+=1
 return k
B=[];columns=[]
for degree in range(N):
 for powers in product(range(degree+1),repeat=6):
  if sum(powers)!=degree:continue
  col=[1]*N
  for j,e in enumerate(powers):
   for i,P in enumerate(points):col[i]=col[i]*pow(P[j],e,p)%p
  if rank_columns(columns+[col])>len(columns):B.append(powers);columns.append(col)
  if len(B)==N:break
 if len(B)==N:break
assert len(B)==N
E=[list(r) for r in zip(*columns)]
A=[E[i]+[int(i==j) for j in range(N)] for i in range(N)]
for j in range(N):
 i=next(i for i in range(j,N) if A[i][j]);A[j],A[i]=A[i],A[j];z=pow(A[j][j],-1,p);A[j]=[z*x%p for x in A[j]]
 for i in range(N):
  if i!=j:
   z=A[i][j];A[i]=[(x-z*y)%p for x,y in zip(A[i],A[j])]
V=[row[N:] for row in A]
M=[]
for variable in range(6):
 M.append([[sum(V[i][k]*points[k][variable]*E[k][j] for k in range(N))%p for j in range(N)] for i in range(N)])
out={'all_checks_pass':True,'field':p,'points':points,'basis_monomials':B,'evaluation_matrix':E,'inverse_evaluation_matrix':V,'coordinate_multiplication_matrices':M,'coordinate_ring_dimension':N,'lifted_bitstrings':32,'basis_maximum_total_degree':max(map(sum,B)),'method':'enumerate lifted toy solutions, merge equal projected points, exact finite-field coordinate algebra; exponential enumeration not a P192 elimination algorithm'}
(root/'elimination.json').write_text(json.dumps(out,separators=(',',':'))+'\n');print('TOY_ELIMINATION_DIMENSION',N,'BASIS_MAX_DEGREE',out['basis_maximum_total_degree'])
