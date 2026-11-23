import random, sys
sys.path.insert(0,'.')
from gadget import *
from pdp_anf import Field

def clmul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
    return r
def test(n, mod, trials=5, seed=1):
    N,nu = ceil_pow3(n)
    rows = reduction_rows(n, mod)
    C = Circ(2*n)
    A=[frozenset([i+1]) for i in range(n)]
    B=[frozenset([n+i+1]) for i in range(n)]
    out = mul_circuit(C,A,B,n,N,rows)
    F=Field(n,mod); rng=random.Random(seed)
    for _ in range(trials):
        a=rng.randrange(1<<n); b=rng.randrange(1<<n)
        asg={i+1:(a>>i)&1 for i in range(n)}; asg.update({n+i+1:(b>>i)&1 for i in range(n)})
        evaluate(C.gates, asg)
        got=0
        for i,e in enumerate(out):
            v=0
            for x in e: v^=asg[x]
            got|=v<<i
        assert got==F.mul(a,b),(n,a,b)
    return C
if __name__=='__main__':
    for n,mod in ((5,0b100101),(9,(1<<9)|(1<<4)|1),(15,(1<<15)|3),(17,(1<<17)|(1<<3)|1),(19,(1<<19)|(1<<5)|(1<<2)|(1<<1)|1)):
        C=test(n,mod); print(n,'ok',C.nxt,len(C.gates))
