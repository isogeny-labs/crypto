from l08lib import *
d=131
f = BinaryField(d,(1<<131)|(1<<13)|(1<<2)|(1<<1)|1)
rows=[]
for J in (27,81):
    for k in range(1,J+1):
        c = FreeCircuit(); c.J_FIXED=J; c.K_FIXED=k
        a,b=c.word(d),c.word(d)
        try: c.gf_multiply(a,b,f)
        except AssertionError: continue
        st=c.statistics(); m=c.mul_metadata[0]
        rows.append((st["boolean_network_edges"],J,k,m["L"],m["n"],st["maximum_degree"]))
rows.sort()
print("edges J k L n maxdeg (one 131-bit gf_multiply, 20 best of %d)"%len(rows))
for r in rows[:12]: print(*r)
print("prototype (J=27,k=6):",[r for r in rows if r[1]==27 and r[2]==6])
