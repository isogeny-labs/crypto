import random
from n3lib import *
p=13; rng=random.Random(1000*1+p); f,crv=fpoly(2,p,rng); a,b,xR=crv
pts=[(x,y) for x in range(p) for y in range(p) if evalf(f,[x,y],p)==0]
fx={(e[0]-1,e[1]):c*e[0]%p for e,c in f.items() if e[0]}; fy={(e[0],e[1]-1):c*e[1]%p for e,c in f.items() if e[1]}
sing=[P for P in pts if evalf(fx,list(P),p)==0 and evalf(fy,list(P),p)==0]
print("curve",crv,"rhs(xR)",(xR**3+a*xR+b)%p,"pts",pts,"singular",sing)
