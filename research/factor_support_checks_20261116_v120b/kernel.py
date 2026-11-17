from research.systematic_residue_20261116_v109.kernel import prune
from research.coprime_factor_checks_20261116_v119b.circuit import lift,normalized_affine
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.chart_compiler_20261116_v112.kernel import source_graph
from research.parity_chart_20261116_v113d.kernel import optimized_import
from research.factor_local_support_20261116_v120.kernel import append_support

def recover_prune(C,outputs,A,B):
    G,oo=prune(C,outputs+[{'recovery_only':True,'wires':A+B}]);ab=oo[-1]['wires'];return G,oo[:-1],ab[:3],ab[3:]

def compile_all(cfg,chart):
    C=Circuit(chart['dimension']);source,outputs=source_graph(cfg);forms=[((chart['particular']>>i)&1)|sum(((v>>i)&1)<<(j+1)for j,v in enumerate(chart['columns']))for i in range(source.n)];C,old,stats=optimized_import(source,outputs,C,forms,list(range(2,C.n+2)),256,8000000);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);A,B,checks=lift(F,18,C);factor=[{'bounded_coprime_factorization':True,'wires':[checks]}];C,new=append_support(C,factor,A,B,18,cfg['fields']);candidate=recover_prune(C,new,A,B);control=recover_prune(C,factor+old,A,B);return candidate,control,stats
