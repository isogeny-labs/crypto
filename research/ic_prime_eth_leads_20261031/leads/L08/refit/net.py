from intfree import *
from collections import Counter
from eccp import challenge as pc, validate
from ecc2k import parameters
_best={}
def pick(d):
    if d not in _best:
        r=[]
        for k in range(1,d+1):
            _,n,L=valid(d,64,k); lg=math.ceil(math.log2(n)) if n>1 else 0
            step=max(1,L//2); J=((2*k+lg+1+step-1)//step)*step
            r.append((gadget_edges(d,(J,k)),J,k))
        _best[d]=min(r)[1:]
    return _best[d]
class Auto(FreeInt):
    def integer_multiply(self,l,r):
        d=max(len(l),len(r)); self.CHOICE={d:pick(d)}; return FreeInt.integer_multiply(self,l,r)
inst=pc(); validate(inst); out={}
for name,cls in [("default",WordCircuit),("freed",Auto)]:
    for m in (2,3):
        st=parameters(inst,m); p=int(st["hash_prime"]); rng=random.Random(1)
        seeds=[{"prime":p,"alpha":rng.randrange(1,p),"beta":rng.randrange(p),"threshold":st["threshold"]} for _ in range(m)]
        e=PrimeCurveEncoding(inst.curve); e.circuit=cls()
        pts=e.relation(inst.Q,seeds,inst.order,inst.cofactor)
        s=e.circuit.statistics(tuple(v for pt in pts for w in pt.words for v in w))
        h=Counter((x["type"],x["input_bits"]) for x in s["multiplication_encodings"])
        out[f"{name}_m{m}"]={"edges":s["boolean_network_edges"],"maxdeg":s["maximum_degree"],"mults":{f"{a}:{b}":c for (a,b),c in h.items()}}
        print(name,m,json.dumps(out[f"{name}_m{m}"]),flush=True)
print("choices",_best)
for m in (2,3): print("m",m,"ratio default/freed = %.3f"%(out[f"default_m{m}"]["edges"]/out[f"freed_m{m}"]["edges"]))
json.dump({"rows":out,"choices":{str(k):v for k,v in _best.items()}},open("net_rows.json","w"))
