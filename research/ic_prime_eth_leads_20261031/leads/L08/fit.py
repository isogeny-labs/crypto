import json, numpy as np
from l08lib import *
rows=json.load(open("t3_rows.json"))
res={}
for cfg in sorted({r["cfg"] for r in rows}):
    R=[r for r in rows if r["cfg"]==cfg]; m=np.array([r["m"] for r in R],float); e=np.array([r["edges"] for r in R],float)
    A=np.vstack([m,np.ones_like(m)]).T; (a,c0),*_=np.linalg.lstsq(A,e,rcond=None)
    resid=e-(a*m+c0)
    res[cfg]={"a":a,"c0":c0,"max_abs_resid":float(abs(resid).max()),"edges":dict(zip(map(int,m),map(int,e))),"maxdeg":sorted({r["maxdeg"] for r in R})}
    print(cfg,"a=%.1f c0=%.1f maxres=%.1f"%(a,c0,abs(resid).max()),res[cfg]["edges"],res[cfg]["maxdeg"])
# exact differences
for cfg in res:
    E=res[cfg]["edges"]; print(cfg,"diffs",E[3]-E[2],E[4]-E[3],(E[6]-E[4])/2)
# integer gadget alone at 133 bits, with interface
c=FreeCircuit(); a=c.word(133); b=c.word(133); out=c.integer_multiply(a,b)
s=c.statistics(tuple(dict.fromkeys(a+b+out)))
print("integer_multiply 133 bits: edges",s["boolean_network_edges"],"deg",s["maximum_degree"],"per 1100(d+1)=",1100*134,"deg bound 136*134^.75=",136*134**.75)
c=FreeCircuit(); a=c.word(131); b=c.word(131); c.J_FIXED=27; c.K_FIXED=27; out=c.polynomial_multiply(a,b)
s=c.statistics(tuple(dict.fromkeys(a+b+out)))
print("poly_mult 131 J=27 k=27 edges",s["boolean_network_edges"],"deg",s["maximum_degree"])
c=FreeCircuit(); a=c.word(131); b=c.word(131); out=c.polynomial_multiply(a,b)
s=c.statistics(tuple(dict.fromkeys(a+b+out)))
print("poly_mult 131 prototype edges",s["boolean_network_edges"],"deg",s["maximum_degree"])
json.dump(res,open("fit_rows.json","w"))
