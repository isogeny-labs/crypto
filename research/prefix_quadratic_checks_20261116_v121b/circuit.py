from research.systematic_residue_20261116_v109.kernel import prune
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine,lift
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.factor_local_support_20261116_v120.kernel import append_support

def prefix_quad(a,b,C):
    m=len(a);assert len(b)==2*m-1;higher_zero=1;guards=[];roots=[];checks=[]
    for k in range(m-1,-1,-1):
        guard=C.op('and',higher_zero,a[k]);guards.append(guard);higher_zero=C.op('and',higher_zero,C.op('not',a[k]));residual=list(b);root=[0]*m;branch_a=a[:k]+[1]
        for j in list(range(m-1,k,-1))+list(range(k-1,-1,-1)):
            degree=2*j if j>k else k+j;bit=residual[degree];root[j]=bit
            for p,value in enumerate(branch_a):residual[p+j]=C.xor(residual[p+j],C.op('and',bit,value))
            residual[2*j]=C.xor(residual[2*j],bit)
        checks.extend(C.op('and',guard,v)for v in residual);roots.append(root)
    guards.append(higher_zero);roots.append([b[2*j]for j in range(m)]);checks.extend(C.op('and',higher_zero,b[j])for j in range(1,2*m-1,2));canonical=[]
    for j in range(m):
        value=0
        for g,r in zip(guards,roots):value=C.xor(value,C.op('and',g,r[j]))
        canonical.append(value)
    return checks,canonical

def append_prefix(C,outputs,A,d):
    cap=d-1;m=d-1;a=[A[1][cap-i]for i in range(cap+1)];b=[A[0][2*cap-i]for i in range(2*cap+1)];checks,root=prefix_quad(a[:m],b[:2*m-1],C);checks+=a[m:]+b[2*m-1:];return outputs+[{'exact_quadratic_prefix':True,'wires':[checks]}],root

def recover_prune(C,outputs,A,B,root):
    G,oo=prune(C,outputs+[{'recovery_only':True,'wires':A+B+[root]}]);w=oo[-1]['wires'];return G,oo[:-1],w[:3],w[3:7],w[7]

def compile_all(cfg,chart):
    C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);A,B,checks=lift(F,18,C);factor=[{'bounded_coprime_factorization':True,'wires':[checks]}];C,alllocal=append_support(C,factor,A,B,18,cfg['fields']);both,root=append_prefix(C,alllocal,A,18);minimal=[row for row in both if row.get('factor')!='A'];return recover_prune(C,minimal,A,B,root),recover_prune(C,both,A,B,root)
