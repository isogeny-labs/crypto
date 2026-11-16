from research.cover_compiler_20261116_v116b.kernel import compile_map
from research.simple_prefix_lift_20261116_v117b.kernel import normalize,lift_circuit
from research.residue_quotient_20261116_v99b.kernel import apply,pmul

def raw_family(rows,modulus):
    def raw(theta):
        value=list(rows[0])
        for i,row in enumerate(rows[1:]):
            if theta>>i&1:value=[a^b for a,b in zip(value,row)]
        if theta>>len(rows[1:]):value[0]^=pmul(theta>>len(rows[1:]),modulus)
        return value
    return raw

def append_guarded_lift(C,outputs,m,raw,d):
    r=len(m['common']);w=m['width'];columns=m['common']+m['extras'][0];indices=list(range(r))+list(range(r,r+len(m['extras'][0])));origin=raw(0);base=normalize(raw(m['particular'][0]),d);directions=[normalize([a^b for a,b in zip(raw(v),origin)],d)for v in columns];G=[]
    for j,a in enumerate(base):
        bits=[]
        for bit in range((5-j)*(d-1)+1):
            value=(a>>bit)&1
            for i,v in enumerate(directions):
                if v[j]>>bit&1:value=C.xor(value,2+indices[i])
            bits.append(value)
        G.append(bits)
    root,tail,low=lift_circuit(G,d,C);s0=2+r+w;s1=s0+1;selector=C.op('and',C.op('not',s0),C.op('not',s1));guard=[C.op('and',selector,v)for v in tail];result=outputs+[{'guarded_high_root':True,'selector':0,'wires':[guard]}]
    return C,result,{'root_wires':root,'tail_wires':tail,'low_residual_wires':low,'k1_selector_wire':selector}

def compile_all(source,outputs,m,raw,d,round_cap=256,event_cap=8000000):
    C,oo,stats=compile_map(source,outputs,m,round_cap,event_cap);C,oo,extra=append_guarded_lift(C,oo,m,raw,d);return C,oo,stats,extra
