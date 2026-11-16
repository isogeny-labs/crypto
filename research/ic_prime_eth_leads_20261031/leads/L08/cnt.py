import sys
sys.dont_write_bytecode=True
from l08lib import *

class CB:
    def __init__(self): self.variables=0; self.occ=[]; self.maxar=3
    def bit(self):
        self.variables+=1; self.occ.append(0); return self.variables-1
    def word(self,n): return tuple(self.bit() for _ in range(n))
    def constraint(self,variables,predicate=None,name=""):
        u=dict.fromkeys(variables)
        if len(u)>self.maxar: self.maxar=len(u)
        o=self.occ
        for v in u: o[v]+=1
    def pin(self,v,val): self.constraint((v,))

class CC(FreeCircuit):
    def __init__(self):
        self.builder=CB(); self.steps=[]; self.constants={}; self.mul_metadata=[]
    def compute(self, inputs, widths, function, name):
        inputs=tuple(map(tuple,inputs)); outputs=tuple(self.word(w) for w in widths)
        self.builder.constraint(tuple(v for w in inputs+outputs for v in w)); return outputs

def count_poly(d,J,k):
    c=CC(); c.J_FIXED=J; c.K_FIXED=k
    a,b=c.word(d),c.word(d)
    out=c.polynomial_multiply(a,b)
    occ=c.builder.occ
    for v in dict.fromkeys(a+b+out): occ[v]+=1
    edges=sum(0 if x==0 else (1 if x<=2 else 2*x-3) for x in occ)
    return 2*edges, c.builder.maxar, c.mul_metadata[0]["L"]

def valid_params(d):
    res=[]
    J=3
    while J<=3**8:
        k=J; n=-(-d//k); L=1
        while L<2*n-1: L*=3
        if (3*J)%L==0: res.append((J,k))
        J*=3
        if J>6*d+30: break
    return res

if __name__=="__main__":
    # cross-check counter vs full builder
    for d in (19,131):
        for J,k in ((27,27),(27,6)) if d==131 else ((9,9),):
            full=FreeCircuit(); full.J_FIXED=J; full.K_FIXED=k
            a,b=full.word(d),full.word(d); out=full.polynomial_multiply(a,b)
            s=full.statistics(tuple(dict.fromkeys(a+b+out)))
            print(d,J,k,"full",s["boolean_network_edges"],s["maximum_degree"],"counter",count_poly(d,J,k)[:2])
