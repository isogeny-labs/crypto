#!/usr/bin/env python3
"""CT99: scalar-blind *online* orientation by the fixed 7-torsion cubic chord.

The fixed CT87 cubic setup used small scalar multiples; online target selection
does not reconstruct m or use CT98's multiplication-map differentiation.
Global identities are exact QQ[x]/psi_7 certificates. [pcc:unverified]
"""
import json
import runpy
from pathlib import Path
from contextlib import redirect_stdout
from io import StringIO

with redirect_stdout(StringIO()):
    c = runpy.run_path(str(Path(__file__).resolve().parent.parent /
                           "ct97" / "ct97_selective_x_root.py"))
sp,x = c["sp"],c["x"]
QQ,X,I,rem,inv,D = (c[t] for t in ("QQ","X","I","rem","inv","D"))
s,e2,e3,roots = (c[t] for t in ("s","e2","e3","roots"))
check = c["check"]
f = QQ(x**3-x**2-10*x-20)
check(rem(X**3-s*X**2+e2*X-e3).is_zero,
      "base generator does not satisfy fixed relative cubic")
# If f(Z)-(lambda*Z+mu)^2-(lambda*Z+mu)=Z^3-sZ^2+e2 Z-e3,
# then lambda^2=s-1 and lambda(2Y+1)=2X(s-1)-e2-10.
# With (2Y+1)^2=D(X), let H=M*(beta-X)/D(X).
# The chord ordinate is Y_beta = H+(1+2H)Y.
M=rem(2*X*(s-I)-e2-QQ(10))
check(rem(M*M-(s-I)*D).is_zero,
      "chord slope square identity fails globally")
check(rem(D-4*f-I).is_zero,"curve discriminant identity")

p=37
eval37=lambda P: int(P.eval(9))%p
s37=eval37(c["N"])*pow(eval37(c["B"]),-1,p)%p
q2=c["q2"]; C2=c["C2"]
e237=sum(int(v)*pow(s37,i,p) for i,v in enumerate(C2))*pow(q2,-1,p)%p
check((s37,e237)==(27,30),"fixed source cubic specialization")
D37=eval37(D)
check(D37!=0,"source D is not a unit")
M37=(2*9*(s37-1)-e237-10)%p
check(M37*M37%p==(s37-1)*D37%p,"source slope square identity")
# CT90 selected the compatible binary prime, not the other CRT factor.
s2,e22,a2=1545,1906,2
m12,p12=(c[t] for t in ("m12","p12"))
check(m12(e22,e22)==s2^1,"fixed binary slope square identity")
check(m12(1906,1906)==1544,"CT90 binary e2 control")

def mul24(u,v):
    t=m12(u[1],v[1])
    return [m12(u[0],v[0])^m12(t,12),
            m12(u[0],v[1])^m12(u[1],v[0])^t]
def on_binary(xb,yb):
    yy=mul24(yb,yb)
    return [yy[0]^yb[0],yy[1]^yb[1]]==[m12(m12(xb,xb),xb)^m12(xb,xb),0]

algebraic_roots=[]
for beta in roots:
    # No input scalar, multiplication-function oracle or degree pair is passed.
    A,B=c["rational_reconstruct"](beta)
    check(rem(A-beta*B).is_zero,"global x root unverified")
    d37=int(B.eval(9))%p
    check(d37!=0,"source x denominator nonunit")
    sx=int(A.eval(9))*pow(d37,-1,p)%p
    d2=c["binary_eval"](B)
    check(d2!=0,"binary x denominator nonunit")
    xb=m12(c["binary_eval"](A),p12(d2,4094))
    H=rem(M*(beta-X)*inv(D))
    V=rem(I+2*H)
    check(rem(V*(2*H+I-V)).is_zero,"spurious Y coefficient")
    check(rem(H*H+H+V*V*f-(beta**3-beta**2-10*beta-20*I)).is_zero,
          "global chord ordinate does not lie on elliptic curve")
    h37=M37*(sx-9)*pow(D37,-1,p)%p
    y37=(h37+(1+2*h37)*4)%p
    # D(alpha)=1 modulo 2 and M=e2 modulo 2; the fixed target reduction of
    # H is e2*(beta-alpha). The global element was checked BEFORE reduction.
    h2=m12(e22,xb^a2)
    y2=[h2,1]   # V=1+2H reduces to 1.
    check(on_binary(xb,y2),"binary chord point fails curve equation")
    algebraic_roots.append(dict(x=sx,x_binary=xb,y_line_source=y37,
                                y_line_binary=y2))

check(sorted(z["x"] for z in algebraic_roots)==[9,27,28],
      "source x roots not exhaustive")
check(sorted(z["x_binary"] for z in algebraic_roots)==[2,17,1562],
      "binary x roots not exhaustive")
def transport(xx,yy):
    """Public-input online selector: no supplied scalar or target-side root."""
    candidates=[z for z in algebraic_roots if z["x"]==xx]
    check(len(candidates)==1,"not a unique public x lift")
    z=candidates[0]
    if yy==z["y_line_source"]:
        out_y=z["y_line_binary"]
    else:
        check(yy==(-1-z["y_line_source"])%p,"wrong source y orientation")
        out_y=[z["y_line_binary"][0]^1,z["y_line_binary"][1]]
    check(on_binary(z["x_binary"],out_y),"binary oriented point fails curve")
    return dict(source=[xx,yy],target_x=z["x_binary"],target_y=out_y)

public=((9,4),(9,32),(27,9),(27,27),(28,0),(28,36))
rows=[transport(*q) for q in public]
# Nonmembers and wrong ordinates cannot be silently transported.
for bad in ((10,4),(27,10),(9,10)):
    try:
        transport(*bad)
    except ArithmeticError:
        pass
    else:
        raise ArithmeticError("nonmember or wrong orientation was accepted")
check([z["target_y"] for z in rows]==
      [[0,1],[1,1],[77,1],[76,1],[2879,1],[2878,1]],
      "fixed oriented CT91/98 known-answer fixtures")
import sys
if len(sys.argv)>1:
    check(len(sys.argv)==3 and sys.argv[1]=="--point",
          "usage: ct99_cubic_chord_orientation.py --point x,y")
    fields=sys.argv[2].split(",")
    check(len(fields)==2 and all(t.isdigit() for t in fields),
          "expected public x,y")
    q=tuple(map(int,fields))
    print(json.dumps(transport(*q),sort_keys=True,separators=(",",":")))
else:
    print(json.dumps(dict(
        binary_line_certificate=True,experiment="CT99-cubic-chord-orientation",
        fixed_source_place=True,fixed_target_place=True,global_line_certificate=True,
        online_scalar_observed=False,scalar_independent_online_selection=True,
        source_rows=rows),sort_keys=True,separators=(",",":")))
