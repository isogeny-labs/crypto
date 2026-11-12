#!/usr/bin/env python3
"""Weierstrass images of the registry E-222 (Edwards) and M-221 (Montgomery) with the
generator mapped; checks on-curve and [r]G = O. Prints one line per curve:
name p a b gx gy n h (hex, no 0x). Source: docs/curves/standards/parameters.json."""
import json,sys
sys.path.insert(0,'.')
import importlib.util,os
HERE=os.path.dirname(os.path.abspath(__file__))
spec=importlib.util.spec_from_file_location("gi",os.path.join(HERE,"..","gen_instances.py"));gi=importlib.util.module_from_spec(spec);spec.loader.exec_module(gi)
reg=json.load(open(sys.argv[1]))
items=reg if isinstance(reg,list) else reg.get("curves",reg)
C={c["name"]:c for c in items if isinstance(c,dict) and c.get("name") in("E-222","M-221")}
def h(x):return format(x,'x')
out=[]
# E-222
c=C["E-222"];p=int(c["field"]["p"],16);d=int(c["params"]["d"]["raw"],16);r=int(c["order"],16);hh=int(c["cofactor"],16)
x=int(c["generator"]["x"]["raw"],16);y=int(c["generator"]["y"]["raw"],16)
assert (x*x+y*y-1-d*x*x*y*y)%p==0
u=(1+y)*pow(1-y,-1,p)%p; v=u*pow(x,-1,p)%p
inv=pow(1-d,-1,p);A=2*(1+d)*inv%p;B=4*inv%p
assert (B*v*v-(u**3+A*u*u+u))%p==0
a2=A*B%p;i3=pow(3,-1,p);X=B*u%p;Y=B*B%p*v%p
a,b=gi.mont_to_weier(A,B,p)
gx=(X+a2*i3)%p;gy=Y
assert (gy*gy-gx**3-a*gx-b)%p==0
assert gi.mul(r,(gx,gy),a,p) is None and gi.mul(1,(gx,gy),a,p)==(gx,gy)
out.append(("E-222",p,a,b,gx,gy,r,hh))
# M-221
c=C["M-221"];p=int(c["field"]["p"],16);A=int(c["params"]["a"]["raw"],16);B=int(c["params"]["b"]["raw"],16);r=int(c["order"],16);hh=int(c["cofactor"],16)
u=int(c["generator"]["x"]["raw"],16);v=int(c["generator"]["y"]["raw"],16)
assert (B*v*v-(u**3+A*u*u+u))%p==0
a2=A*B%p;i3=pow(3,-1,p);X=B*u%p;Y=B*B%p*v%p
a,b=gi.mont_to_weier(A,B,p);gx=(X+a2*i3)%p;gy=Y
assert (gy*gy-gx**3-a*gx-b)%p==0
assert gi.mul(r,(gx,gy),a,p) is None
out.append(("M-221",p,a,b,gx,gy,r,hh))
for n,p,a,b,gx,gy,r,hh in out: print(n,h(p),h(a),h(b),h(gx),h(gy),h(r),hh)
