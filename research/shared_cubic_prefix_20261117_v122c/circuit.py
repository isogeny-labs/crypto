from research.prefix_quadratic_checks_20261116_v121b.circuit import prefix_quad
from research.coprime_factor_checks_20261116_v119b.circuit import normalized_affine,lift
from research.factor_local_support_20261116_v120.kernel import append_support
from research.guarded_high_root_20261116_v118.kernel import raw_family
from research.guarded_local_search_20261116_v101.kernel import matrix
from research.local_split_circuit_20261116_v100.kernel import Circuit
from research.systematic_residue_20261116_v109.kernel import prune

def xor_poly(a,b,C):return [C.xor(a[i]if i<len(a)else 0,b[i]if i<len(b)else 0)for i in range(max(len(a),len(b)))]
def multiply(a,b,C):
    out=[0]*(len(a)+len(b)-1)
    for i,x in enumerate(a):
        for j,y in enumerate(b):out[i+j]=C.xor(out[i+j],C.op('and',x,y))
    return out

def shift_poly(a,k,gate,C):return [0]*k+[C.op('and',gate,x)for x in a]
def translate(H,k,gate,C):
    c=[0]*k+[gate];c2=[0]*(2*k)+[gate];c3=[0]*(3*k)+[gate];return [xor_poly(xor_poly(xor_poly(H[0],shift_poly(H[1],k,gate,C),C),shift_poly(H[2],2*k,gate,C),C),c3,C),xor_poly(H[1],c2,C),xor_poly(H[2],c,C),[1]]
def eval_coefficient(G,r,n,C):
    value=G[0][n]if n<len(G[0])else 0
    for i,x in enumerate(r):
        j=n-2*i
        if 0<=j<len(r):value=C.xor(value,C.op('and',x,r[j]))
        j=n-2*i
        if 0<=j<len(G[2]):value=C.xor(value,C.op('and',x,G[2][j]))
        j=n-i
        if 0<=j<len(G[1]):value=C.xor(value,C.op('and',x,G[1][j]))
    return value

def support_circuit(raw,m,C):
    H=[];checks=[];active=1;offset=[0]*m;exits=[]
    for j,v in enumerate(raw):
        cap=(3-j)*(m-1)+1;checks.extend(v[cap:]);H.append((v+[0]*cap)[:cap])
    assert H[3]==[1]
    for width in range(m,1,-1):
        k=width-1;e1,e2,e3=H[2][k],H[1][2*k],H[0][3*k];checks.append(C.op('and',active,C.xor(e3,C.op('and',e1,e2))));different=C.xor(e1,e2);mixed=C.op('and',active,different);pure=C.op('and',active,C.op('not',different));G=[list(reversed(v))for v in H];series=[e1]
        for n in range(1,width):series.append(eval_coefficient(G,series,n,C))
        root=list(reversed(series));a=xor_poly(H[2],root,C);b=xor_poly(H[1],multiply(root,a,C),C);remainder=xor_poly(H[0],multiply(root,b,C),C);qchecks,qroot=prefix_quad(a,b,C);checks.extend(C.op('and',mixed,z)for z in remainder+qchecks);other=[C.xor(x,y)for x,y in zip(qroot,a)];triples=[root,qroot,other];exit_roots=[[C.xor(v[j]if j<len(v)else 0,offset[j])for j in range(m)]for v in triples];exits.append((mixed,exit_roots));nextH=translate(H,k,e1,C)
        for j,v in enumerate(nextH):
            cap=(3-j)*(width-2)+1;checks.extend(C.op('and',pure,z)for z in v[cap:]);H[j]=(v+[0]*cap)[:cap]
        offset[k]=C.op('and',pure,e1);active=pure
    e1,e2,e3=H[2][0],H[1][0],H[0][0];both=C.op('and',e1,e2);checks.append(C.op('and',active,C.xor(e3,both)));last=[both,e2,C.xor(C.xor(e1,e2),both)];exits.append((active,[[C.xor(bit if j==0 else 0,offset[j])for j in range(m)]for bit in last]));roots=[]
    for index in range(3):
        row=[]
        for j in range(m):
            value=0
            for guard,triple in exits:value=C.xor(value,C.op('and',guard,triple[index][j]))
            row.append(value)
        roots.append(row)
    return checks,roots

def raw_factor(factor,d):
    degree=len(factor)-1;return [list(reversed(v[:(degree-j)*(d-1)+1]))for j,v in enumerate(factor)]
def recover_prune(C,outputs,A,B,roots):
    G,oo=prune(C,outputs+[{'recovery_only':True,'wires':A+B+roots}]);v=oo[-1]['wires'];return G,oo[:-1],v[:3],v[3:7],v[7:]

def compile_all(cfg,chart):
    C=Circuit(chart['dimension']);F=normalized_affine(raw_family(matrix(cfg['matrix']),cfg['modulus']),chart['particular'],chart['columns'],18,C);A,B,checks=lift(F,18,C);a_raw=raw_factor(A,18);a=a_raw[1][:17];b=a_raw[0][:33];qchecks,qroot=prefix_quad(a,b,C);qchecks+=a_raw[1][17:]+a_raw[0][33:];other=[C.xor(x,y)for x,y in zip(qroot,a)];shifted=translate(raw_factor(B,18),17,1,C);cchecks,croots=support_circuit(shifted,17,C);highroots=[r+[1]for r in croots];roots=[qroot+[0],other+[0]]+highroots;outputs=[{'bounded_coprime_factorization':True,'wires':[checks]},{'exact_quadratic_prefix':True,'wires':[qchecks]},{'exact_cubic_prefix':True,'wires':[cchecks]}];C,control=append_support(C,outputs,A,B,18,cfg['fields']);return recover_prune(C,outputs,A,B,roots),recover_prune(C,control,A,B,roots)
