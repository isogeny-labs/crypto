# Leading-monomial analysis of an msolve -g 2 output (grevlex, vars s1..s4):
# zero-dimensional iff each variable has a pure power among leading monomials; degree = #standard monomials.
import sys, re, itertools
def mon(t):
    ex=[0,0,0,0]
    for v in t.split("*")[1:]:
        if "^" in v: n,e=v.split("^"); ex[int(n[1])-1]=int(e)
        else: ex[int(v[1])-1]=1
    return tuple(ex)
def grevkey(e): return (sum(e), tuple(-x for x in reversed(e)))
txt=open(sys.argv[1]).read()
body=txt[txt.index("["):].strip().rstrip(":").strip()[1:-1]
polys=body.split(",")
LM=[]
for p in polys:
    p=p.strip()
    if not p: continue
    terms=re.split(r"(?<=[0-9a-z])\+", p)
    LM.append(max((mon(t) for t in terms), key=grevkey))
pure={i:min([e[i] for e in LM if all(e[j]==0 for j in range(4) if j!=i)] or [None], key=lambda x: x if x is not None else 10**9) for i in range(4)}
print(sys.argv[1], "basis size", len(LM), "pure powers", pure)
if all(v is not None for v in pure.values()):
    B=[pure[i] for i in range(4)]; cnt=0
    for e in itertools.product(*[range(b) for b in B]):
        if not any(all(e[k]>=m[k] for k in range(4)) for m in LM): cnt+=1
    print("  zero-dimensional, degree", cnt)
else:
    print("  POSITIVE-DIMENSIONAL (some variable has no pure-power leading monomial)")
