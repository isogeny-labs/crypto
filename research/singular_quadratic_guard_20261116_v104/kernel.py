import json
from pathlib import Path
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.residue_tables_20261116_v102b.kernel import multiplication,diagram
from research.table_relations_20261116_v103.kernel import features
from research.local_split_circuit_20261116_v100.kernel import Circuit,BF
from research.guarded_local_search_20261116_v101.kernel import matrix

def power(x,k,G):
    out=1
    while k:
        if k&1:out=G[out][x]
        x=G[x][x];k>>=1
    return out

def bad_quadratic(a,c,s,G):
    # Q=X^2+sqrt(a)X+sqrt(c). a=0 is always split.
    if not a:return False
    value=G[power(c,1<<(s-1),G)][power(a,(1<<s)-2,G)];trace=0
    for _ in range(s):trace^=value;value=G[value][value]
    assert trace in(0,1)
    return bool(trace)

def rank(rows):
    piv={}
    for r in rows:
        while r:
            j=r.bit_length()-1
            if j in piv:r^=piv[j]
            else:piv[j]=r;break
    return len(piv)

def sparse_basis(rows,passes):
    old=list(rows);rows=list(rows)
    for _ in range(passes):
        changed=False
        for i in sorted(range(len(rows)),key=lambda i:-rows[i].bit_count()):
            for j in range(len(rows)):
                if i!=j and (rows[i]^rows[j]).bit_count()<rows[i].bit_count():rows[i]^=rows[j];changed=True
        if not changed:break
    assert rank(old)==rank(rows)==rank(old+rows)==len(rows)
    return rows

def parity(C,mon,relation,chunk):
    # Local memo also shares recursively-built parity prefixes through C.cache.
    partial=[];offset=0
    while relation:
        pattern=relation&((1<<chunk)-1)
        if pattern:
            value=0
            while pattern:
                low=pattern&-pattern;value=C.xor(value,mon[offset+low.bit_length()-1]);pattern^=low
            partial.append(value)
        relation>>=chunk;offset+=chunk
    while len(partial)>1:partial=[C.xor(partial[i],partial[i+1])if i+1<len(partial)else partial[i]for i in range(0,len(partial),2)]
    return partial[0]if partial else 0

def local(C,s,m,t,relations,coeff,passes=16,chunk=4):
    masks=features(4*s,3);bits=[w for c in coeff for w in c];mon={0:1};values=[]
    for mask in masks:
        if mask:
            low=mask&-mask;mon[mask]=C.op('and',mon[mask^low],bits[low.bit_length()-1])
        values.append(mon[mask])
    sparse=sparse_basis(relations,passes);zero=[parity(C,values,r,chunk)for r in sparse]
    if s==5:
        G=multiplication(s,m);bad={a|(c<<s)for a in range(1<<s)for c in range(1<<s)if bad_quadratic(a,c,s,G)};nodes,root=diagram(2*s,bad);wires=coeff[3]+coeff[1];evals=[0,1]
        for bit,lo,hi in nodes:evals.append(C.xor(evals[lo],C.op('and',wires[bit],C.xor(evals[lo],evals[hi]))))
        f=BF(s,m,C);A=f.add(coeff[2],f.mul(f.const(t),coeff[3]));B=f.add(coeff[0],f.mul(f.const(t),coeff[1]));same=1
        for w in A+B:same=C.op('and',same,C.op('not',w))
        zero.append(C.op('and',same,evals[root]))
    return zero,{'s':s,'m':m,'relations':len(relations),'terms_before':sum(r.bit_count()for r in relations),'terms_after':sum(r.bit_count()for r in sparse)}

def compile_all(d):
    rows=matrix(d['matrix']);base,hc=rows[0],rows[1:];profile=json.loads(Path(d['relations_raw']).read_text());assert profile['error']is None;C=Circuit(76);outputs=[];stats=[]
    for record in profile['out']['fields']:
        s,m=record['s'],record['m'];coeff=[]
        for k in range(4):
            fixed=rem(base[k],m);columns=[rem(h[k],m)for h in hc]+([rem(pmul(1<<i,d['modulus']),m)for i in range(3)]if k==0 else[0]*3);wires=[]
            for bit in range(s):
                w=fixed>>bit&1
                for j,v in enumerate(columns):
                    if v>>bit&1:w=C.xor(w,2+j)
                wires.append(w)
            coeff.append(wires)
        rel=next(x['relations']for x in record['degrees']if x['degree']==3);zero,st=local(C,s,m,rem(base[4],m),rel,coeff,d['sparsify_passes'],d['parity_chunk']);outputs.append({'s':s,'m':m,'wires':[[w]for w in zero]});stats.append(st)
    def flag(k,b):
        w=base[k]>>b&1
        for j,h in enumerate(hc):
            if h[k]>>b&1:w=C.xor(w,2+j)
        return w
    u,v,w=flag(3,34),flag(2,51),flag(1,68);outputs.append({'guard':True,'wires':[[C.xor(u,v)],[C.op('and',u,w)],[C.xor(77,w)],[C.op('and',76,C.op('not',w))]]})
    return C,outputs,stats
