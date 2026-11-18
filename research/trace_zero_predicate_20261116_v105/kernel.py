import json
from pathlib import Path
from research.residue_tables_20261116_v102b.kernel import multiplication
from research.residue_quotient_20261116_v99b.kernel import rem,pmul
from research.singular_quadratic_guard_20261116_v104.kernel import local,power
from research.local_split_circuit_20261116_v100.kernel import Circuit,BF
from research.guarded_local_search_20261116_v101.kernel import matrix

def normalization(h,t,G):
    d,c,b,a=h;t2=G[t][t];t3=G[t2][t];t4=G[t2][t2]
    return [d^G[t][c]^G[t2][b]^G[t3][a],c^G[t2][a]^t4,b^G[t][a],a]

def inverse(g,t,G):
    d,c,b,a=g;t2=G[t][t];t3=G[t2][t];t4=G[t2][t2];t5=G[t4][t]
    return [d^G[t][c]^G[t2][b]^G[t3][a]^t5,c^G[t2][a]^t4,b^G[t][a],a]

def field_isomorphism(s,m,canonical):
    q=1<<s;G=multiplication(s,canonical)
    def at(poly,beta):
        value=0
        for i in range(s,-1,-1):value=G[value][beta]^(poly>>i&1)
        return value
    beta=next(x for x in range(q)if at(m,x)==0);powers=[1]
    for i in range(1,s):powers.append(G[powers[-1]][beta])
    mapping=[]
    for x in range(q):
        value=0
        for j in range(s):
            if x>>j&1:value^=powers[j]
        mapping.append(value)
    assert len(set(mapping))==q and mapping[0]==0 and mapping[1]==1
    return mapping

def translate_poly(H,shift,f):
    out=[]
    for a in reversed(H):
        new=[0]*(len(out)+1)
        for i,c in enumerate(out):new[i]=f.add(new[i],f.mul(shift,c));new[i+1]=f.add(new[i+1],c)
        new[0]=f.add(new[0],a);out=new
    while out and out[-1]==0:out.pop()
    return out

def depress_quintic(H,f):
    if f.p==5:raise ValueError('characteristic divides degree; separate branch required')
    shift=f.sub(0,f.mul(H[4],f.inv(5%f.p)))
    return translate_poly(H,shift,f),shift

def compile_all(d):
    templates=json.loads(Path(d['templates_path']).read_text());rows=matrix(d['matrix']);base,hc=rows[0],rows[1:];C=Circuit(76);outputs=[]
    for s,m in d['fields']:
        template=next(r for r in templates['fields']if r['s']==s);canonical=template['m'];mapping=field_isomorphism(s,m,canonical);f=BF(s,canonical,C);coeff=[]
        # Reduce and map constants first: all instance wires stay affine.
        for k in range(4):
            fixed=mapping[rem(base[k],m)];columns=[mapping[rem(h[k],m)]for h in hc]+([mapping[rem(pmul(1<<i,d['modulus']),m)]for i in range(3)]if k==0 else[0]*3);bits=[]
            for bit in range(s):
                wire=fixed>>bit&1
                for j,a in enumerate(columns):
                    if a>>bit&1:wire=C.xor(wire,2+j)
                bits.append(wire)
            coeff.append(bits)
        t=mapping[rem(base[4],m)];G=multiplication(s,canonical);t2=G[t][t];t3=G[t2][t];t4=G[t2][t2];h0,h1,h2,h3=coeff
        mul=lambda a,c:f.mul(f.const(a),c)
        normalized=[f.add(f.add(h0,mul(t,h1)),f.add(mul(t2,h2),mul(t3,h3))),f.add(f.add(h1,mul(t2,h3)),f.const(t4)),f.add(h2,mul(t,h3)),h3]
        zero,_=local(C,s,canonical,0,template['relations'],normalized,d['sparsify_passes'],d['parity_chunk']);outputs.append({'s':s,'m':m,'wires':[[w]for w in zero]})
    def flag(k,b):
        wire=base[k]>>b&1
        for j,h in enumerate(hc):
            if h[k]>>b&1:wire=C.xor(wire,2+j)
        return wire
    u,v,w=flag(3,34),flag(2,51),flag(1,68);outputs.append({'guard':True,'wires':[[C.xor(u,v)],[C.op('and',u,w)],[C.xor(77,w)],[C.op('and',76,C.op('not',w))]]})
    return C,outputs,[]
