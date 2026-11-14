#!/usr/bin/env python3
"""CT95: exact rational-degree gate and reproducible short Padé negative control.
Uses existing CT84 exact-integer LLL, not an independently developed solver.
"""
from pathlib import Path
from fractions import Fraction
from contextlib import redirect_stdout
from math import gcd
import io, runpy

source = Path(__file__).resolve().parent.parent / "ct84" / "ct84_selective_pade.py"
with redirect_stdout(io.StringIO()):
    data = runpy.run_path(str(source))
ctx = data["lift"].__globals__
p = 37
check = data["check"]  # always executes, even under python -O
D,P3,F4 = data["D"],data["P3"],data["F4"]
mul,sub,ev,lll = (data[n] for n in ("mul","sub","ev","lll"))
phi3 = sub([0]+mul(P3,P3),mul(D,F4))
psi3sq = mul(P3,P3)
check((len(phi3)-1,len(psi3sq)-1)==(9,8), "tripling degree")
check(ev(phi3,9,p)*pow(ev(psi3sq,9,p),-1,p)%p==28, "source tripling x")

def divrem(a,b):
    a=list(map(Fraction,a)); b=list(map(Fraction,b))
    while len(a)>1 and a[-1]==0: a.pop()
    while len(b)>1 and b[-1]==0: b.pop()
    while len(a)>=len(b) and any(a):
        z=a[-1]/b[-1]; shift=len(a)-len(b)
        for i,x in enumerate(b): a[i+shift]-=z*x
        while len(a)>1 and a[-1]==0: a.pop()
    return a

def coprime(a,b):
    while any(b): a,b=b,divrem(a,b)
    return len(a)==1 and a[0]!=0
check(coprime(phi3,psi3sq), "tripling map must be reduced")

def short_pade(source_root,target_root,digits=18):
    ctx["N"]=digits
    a,b=ctx["lift"](source_root),ctx["lift"](target_root)
    modulus=p**digits
    cs=[pow(a,i,modulus) for i in range(5)]+[(-b*pow(a,i,modulus))%modulus for i in range(4)]
    basis=[[modulus]+[0]*8]+[[(-cs[i])%modulus]+[int(i==j+1) for j in range(8)] for i in range(1,9)]
    reduced=lll(basis)
    for row in reduced:
        d=0
        for v in row: d=gcd(d,abs(v))
        v=[z//d for z in row]
        A,B=v[:5],v[5:]
        if ev(B,a,p)==0: continue
        if (ev(A,a,modulus)-b*ev(B,a,modulus))%modulus: continue
        return A,B,max(map(abs,v))
    raise ArithmeticError("No admissible lattice row")

cases=[]
for root in (27,28):
    for digits in (18,24,36):
        A,B,height=short_pade(9,root,digits)
        cases.append((root,digits,height))
        print("target",root,"digits",digits,"candidate_max_coefficient",height)
check(cases==[(27,18,158),(27,24,158),(27,36,158),
               (28,18,905),(28,24,8539),(28,36,802789)], "LLL candidate regression")
# If degree-24 alpha satisfied a (4,3) candidate for the tripling
# rational function with reduced degrees (9,8), then a nonzero
# polynomial of degree <= max(4+8,3+9) = 12 < 24 would vanish at alpha.
# Hence no globally equal (4,3) rational function can exist.
check(max(4+8,3+9)<24, "degree gate")
print("PASS: short (4,3) template cannot globally represent tripling on degree-24 point field")
