from research.shared_cubic_prefix_20261117_v122c.circuit import raw_factor,translate,support_circuit,xor_poly,multiply
from research.prefix_quadratic_checks_20261116_v121b.circuit import prefix_quad
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine
from .clipped_lift import lift
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune

def generate(C,F,d):
    assert d>=2;m=d-1;A,B,checks=lift(F,d,C);ar=raw_factor(A,d);a=ar[1][:m];b=ar[0][:2*m-1];qc,qr=prefix_quad(a,b,C);qc+=ar[1][m:]+ar[0][2*m-1:];other=[C.xor(x,y)for x,y in zip(qr,a)];bc,br=support_circuit(translate(raw_factor(B,d),m,1,C),m,C);roots=[qr+[0],other+[0]]+[r+[1]for r in br];outputs=[{'bounded_coprime_factorization':True,'wires':[checks]},{'exact_quadratic_prefix':True,'wires':[qc]},{'exact_cubic_prefix':True,'wires':[bc]}];return outputs,roots

def isolated(C,outputs,roots):
    G,oo=prune(C,outputs+[{'recovery_only':True,'wires':roots}]);return G,oo[:-1],oo[-1]['wires']

def compare(C,F,outputs,roots):
    control=isolated(C,outputs,roots);before=tuple(control[0].nodes);coefficients=[[1]]
    for root in roots:
        new=[[]for _ in range(len(coefficients)+1)]
        for j,value in enumerate(coefficients):new[j]=xor_poly(new[j],multiply(value,root,C),C);new[j+1]=xor_poly(new[j+1],value,C)
        coefficients=new
    raw=[list(reversed(v))for v in F];checks=[]
    for a,b in zip(coefficients,raw):checks+=xor_poly(a,b,C)
    candidate=isolated(C,[{'exact_raw_root_product':True,'wires':[checks]}],roots);assert candidate[0]is not control[0]and tuple(control[0].nodes)==before
    return candidate,control

def tiny(d):
    C=Circuit(15*(d-1));F=[];offset=2
    for j in range(5):
        cap=(5-j)*(d-1);F.append([int(j>=2)]+list(range(offset,offset+cap)));offset+=cap
    F.append([1]);outputs,roots=generate(C,F,d);return compare(C,F,outputs,roots)

def compile_all(cfg,chart):
    C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);outputs,roots=generate(C,F,18);return compare(C,F,outputs,roots)
