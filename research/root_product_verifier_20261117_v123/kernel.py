from research.shared_cubic_prefix_20261117_v122c.circuit import compile_all as generator,xor_poly,multiply
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.systematic_residue_20261116_v109.kernel import prune

def compile_all(cfg,chart):
    old,control=generator(cfg,chart);C,old_outputs,A,B,roots=old;coefficients=[[1]]
    for root in roots:
        new=[[]for _ in range(len(coefficients)+1)]
        for j,value in enumerate(coefficients):new[j]=xor_poly(new[j],multiply(value,root,C),C);new[j+1]=xor_poly(new[j+1],value,C)
        coefficients=new
    F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);raw=[list(reversed(v))for v in F];checks=[]
    for a,b in zip(coefficients,raw):checks+=xor_poly(a,b,C)
    G,outputs=prune(C,[{'exact_raw_root_product':True,'wires':[checks]},{'recovery_only':True,'wires':roots}]);return (G,outputs[:-1],outputs[-1]['wires']),old
