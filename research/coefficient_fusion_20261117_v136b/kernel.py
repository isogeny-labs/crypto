from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.coprime_factor_checks_20261116_v119b.circuit import coefficient
from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.clip_defect_checks_20261117_v127c.kernel import defects
from research.product_tree_checks_20261117_v126c.kernel import product_tree
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly
from research.residue_quotient_20261116_v99b.kernel import apply

def first_correction(F,d,C):
 A=[[0],[0],[1]];B=[[1],[1],[1],[1]];m=d-1
 for n in range(1,d+1):
  if n==d:return C.xor(C.xor(F[0][n]if n<len(F[0])else 0,coefficient(A,B,0,n,C)),C.xor(F[1][n]if n<len(F[1])else 0,coefficient(A,B,1,n,C)))
  e=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)];a1=C.xor(e[0],e[1]);a=[e[0],a1];b=[C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])]
  for factor,degree,values in [(A,2,a),(B,3,b)]:
   for j,value in enumerate(values):
    if n<=(degree-j)*m:factor[j].append(value)
 raise AssertionError('unreachable')

def fused(base,columns,d,count,direction,pivot,max_nodes=100000):
 n=len(columns);assert 0<direction<1<<n and direction>>pivot&1;C=Circuit(n-1);x=[0 if i==pivot else 2+i-(i>pivot)for i in range(n)];F=[]
 for j,b in enumerate(base):
  group=[]
  for k in range((5-j)*(d-1)+1):
   w=b>>k&1
   for i,col in enumerate(columns):
    if col[j]>>k&1:w=C.xor(w,x[i])
   group.append(w)
  F.append(group)
 h=first_correction(F,d,C);cofactor_nodes=len(C.nodes)-2;D=[apply([col[j]for col in columns],direction)for j in range(6)];reconstructed=[[C.xor(w,h)if D[j]>>k&1 else w for k,w in enumerate(group)]for j,group in enumerate(F)];theta=[C.xor(w,h)if direction>>i&1 else w for i,w in enumerate(x)];_,roots=generate(C,reconstructed,d);rows=[r for r in defects(reconstructed,d,C)if r['factor']=='A'];assert rows[0]['precision']==d and rows[0]['coefficient']==1;selected=rows[:count];coeff=product_tree(C,roots,(0,(1,(2,(3,4)))));checks=[]
 for a,b in zip(coeff,[list(reversed(v))for v in reconstructed]):checks+=xor_poly(a,b,C)
 oo=[{'exact_raw_root_product':True,'wires':[checks]},{'correction_basis_prefix':count,'wires':[[0]+[r['wire']for r in selected[1:]]]}];assert len(C.nodes)<=max_nodes,'fusionnodecap';G,oo,allroots=isolated(C,oo,roots+[theta,[h]]+reconstructed)
 return G,oo,allroots[:5],allroots[5],allroots[6][0],allroots[7:],{'free_inputs':G.n,'old_selectors':n,'cofactor_nodes_before_final_prune':cofactor_nodes,'unpruned_variables':len(C.nodes)-2,'variables':len(G.nodes)-2,'direction_polynomials_hex':[hex(v)for v in D]}

def tiny_family(d):
 base=[int(j>=2)for j in range(6)];columns=[]
 for j in range(5):
  for k in range(1,(5-j)*(d-1)+1):
   col=[0]*6;col[j]=1<<k;columns.append(col)
 return base,columns
