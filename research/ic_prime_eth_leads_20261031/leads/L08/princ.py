from l08lib import *
out=[]
for J,L in ((27,9),(27,27),(27,81),(81,3),(81,81),(9,27)):
    f=BinaryField(2*J,(1<<(2*J))|(1<<J)|1); w=f.reduce(1<<(3*J//L))
    pw=[field_power(f,w,i) for i in range(L)]
    ok=True
    for t in range(1,L):
        s=0
        for i in range(L): s^=pw[(t*i)%L]
        ok&=(s==0)
    out.append((J,L,ok)); print("ring x^%d+x^%d+1, omega=x^%d, L=%d principal:"%(2*J,J,3*J//L,L),ok)
