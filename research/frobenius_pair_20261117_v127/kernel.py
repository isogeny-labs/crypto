from research.degree_clipped_checks_20261117_v125c.kernel import generate,isolated
from research.degree_clipped_checks_20261117_v125c.clipped_lift import lift
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.shared_cubic_prefix_20261117_v122c.circuit import raw_factor,xor_poly,multiply
from research.product_tree_checks_20261117_v126c.kernel import product_tree

def combine(C,A,B):
    out=[[]for _ in range(len(A)+len(B)-1)]
    for i,a in enumerate(A):
        for j,b in enumerate(B):out[i+j]=xor_poly(out[i+j],multiply(a,b,C),C)
    return out

def build(C,F,d):
    outputs,roots=generate(C,F,d);A,B,checks=lift(F,d,C);m=d-1;a=raw_factor(A,d)[1][:m]+[0];r=roots[0]
    assert roots[1]==[C.xor(x,y)for x,y in zip(r,a)]
    square=[0]*(2*len(r)-1)
    for i,w in enumerate(r):square[2*i]=w
    low=[xor_poly(square,multiply(a,r,C),C),a,[1]];high=product_tree(C,roots,((2,3),4));coeff=combine(C,low,high);checks=[]
    for left,right in zip(coeff,[list(reversed(v))for v in F]):checks+=xor_poly(left,right,C)
    return isolated(C,[{'exact_raw_root_product':True,'wires':[checks]}],roots)

def tiny(d):
    C=Circuit(15*(d-1));F=[];offset=2
    for j in range(5):
        cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
    F.append([1]);return build(C,F,d)

def compile_all(cfg,chart):
    C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);return build(C,F,18)
