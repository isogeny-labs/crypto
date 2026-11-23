from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly,multiply

def trees(items):
    if len(items)==1:return [items[0]]
    return [(a,b)for k in range(1,len(items))for a in trees(items[:k])for b in trees(items[k:])]

def product_tree(C,roots,tree):
    if isinstance(tree,int):return [roots[tree],[1]]
    A=product_tree(C,roots,tree[0]);B=product_tree(C,roots,tree[1]);out=[[]for _ in range(len(A)+len(B)-1)]
    for i,a in enumerate(A):
        for j,b in enumerate(B):out[i+j]=xor_poly(out[i+j],multiply(a,b,C),C)
    return out

def build(C,F,d,tree):
    outputs,roots=generate(C,F,d);coeff=product_tree(C,roots,tree);checks=[]
    for a,b in zip(coeff,[list(reversed(v))for v in F]):checks+=xor_poly(a,b,C)
    return isolated(C,[{'exact_raw_root_product':True,'wires':[checks]}],roots)

def tiny(d,tree):
    C=Circuit(15*(d-1));F=[];offset=2
    for j in range(5):
        cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
    F.append([1]);return build(C,F,d,tree)

def compile_all(cfg,chart,tree):
    C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);return build(C,F,18,tree)
