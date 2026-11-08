from pathlib import Path
import json
root=Path(__file__).resolve().parent;e=json.loads((root/'elimination.json').read_text());c=json.loads((root/'certificate.json').read_text());p=e['field'];N=e['coordinate_ring_dimension'];E=e['evaluation_matrix'];V=e['inverse_evaluation_matrix'];M=e['coordinate_multiplication_matrices'];P=e['points'];I=[[int(i==j) for j in range(N)] for i in range(N)]
def mm(A,B):return [[sum(a*b for a,b in zip(row,col))%p for col in zip(*B)] for row in A]
def add(A,B,sign=1):return [[(a+sign*b)%p for a,b in zip(x,y)] for x,y in zip(A,B)]
def scale(A,k):return [[k*x%p for x in row] for row in A]
def power(A,k):
 R=I
 while k:
  if k&1:R=mm(R,A)
  A=mm(A,A);k//=2
 return R
assert sorted({tuple(v for T in row['output'] for v in T) for row in c['toy']['rows']})==list(map(tuple,P))
for i,row in enumerate(P):
 for j,mon in enumerate(e['basis_monomials']):
  x=1
  for value,k in zip(row,mon):x=x*pow(value,k,p)%p
  assert x==E[i][j]
assert mm(E,V)==mm(V,E)==I
for k,A in enumerate(M):
 assert mm(E,A)==[[(P[i][k]*x)%p for x in row] for i,row in enumerate(E)]
 assert power(A,p)==A
for i in range(6):
 for j in range(i):assert mm(M[i],M[j])==mm(M[j],M[i])
for X,Y,O in [M[:3],M[3:]]:
 assert mm(O,O)==O and mm(O,X)==mm(O,Y)==[[0]*N for _ in range(N)]
 curve=add(add(add(power(Y,2),power(X,3),-1),scale(X,c['toy']['a']),-1),scale(I,c['toy']['b']),-1)
 assert mm(add(I,O,-1),curve)==[[0]*N for _ in range(N)]
out={'all_checks_pass':True,'dimension':N,'basis_monomial_evaluations':N*N,'inverse_matrix_identities':2,'coordinate_operator_checks':6,'commutation_checks':15,'field_equations':6,'normalized_curve_operator_checks':2,'scope':'exact toy projected coordinate algebra; no P192 elimination'}
(root/'elimination-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
