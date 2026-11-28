from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.coprime_factor_checks_20261116_v119b.circuit import coefficient,normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly
from research.product_tree_checks_20261117_v126c.kernel import product_tree

def defects(F,d,C):
 A=[[0],[0],[1]];B=[[1],[1],[1],[1]];m=d-1;rows=[]
 for n in range(1,3*m+1):
  e=[C.xor(F[j][n]if n<len(F[j])else 0,coefficient(A,B,j,n,C))for j in range(5)];a1=C.xor(e[0],e[1]);a=[e[0],a1];b=[C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])]
  for label,factor,degree,values in [('A',A,2,a),('B',B,3,b)]:
   for j,value in enumerate(values):
    if n<=(degree-j)*m:factor[j].append(value)
    else:rows.append({'precision':n,'factor':label,'coefficient':j,'wire':value})
 return rows

def build(C,F,d,count):
 outputs,roots=generate(C,F,d);rows=defects(F,d,C);coeff=product_tree(C,roots,(0,(1,(2,(3,4)))));checks=[]
 for a,b in zip(coeff,[list(reversed(v))for v in F]):checks+=xor_poly(a,b,C)
 selected=rows[:count];G,oo,rr=isolated(C,[{'exact_raw_root_product':True,'wires':[checks]},{'early_clipping_defects':count,'wires':[[r['wire']for r in selected]]}],roots);return G,oo,rr,[{k:v for k,v in row.items()if k!='wire'}for row in selected]

def tiny(d,count):
 C=Circuit(15*(d-1));F=[];offset=2
 for j in range(5):
  cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
 F.append([1]);return build(C,F,d,count)

def compile_all(cfg,chart,count):
 C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);return build(C,F,18,count)
