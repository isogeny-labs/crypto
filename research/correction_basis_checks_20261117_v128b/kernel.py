from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.clip_defect_checks_20261117_v127c.kernel import defects
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly
from research.product_tree_checks_20261117_v126c.kernel import product_tree

def build(C,F,d,count):
 outputs,roots=generate(C,F,d);rows=[r for r in defects(F,d,C)if r['factor']=='A'];selected=rows[:count];coeff=product_tree(C,roots,(0,(1,(2,(3,4)))));checks=[]
 for a,b in zip(coeff,[list(reversed(v))for v in F]):checks+=xor_poly(a,b,C)
 G,oo,rr=isolated(C,[{'exact_raw_root_product':True,'wires':[checks]},{'correction_basis_prefix':count,'wires':[[r['wire']for r in selected]]}],roots)
 return G,oo,rr,[{k:v for k,v in row.items()if k!='wire'}for row in selected]

def tiny(d,count):
 C=Circuit(15*(d-1));F=[];offset=2
 for j in range(5):
  cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
 F.append([1]);return build(C,F,d,count)

def compile_all(cfg,chart,count):
 C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);return build(C,F,18,count)
