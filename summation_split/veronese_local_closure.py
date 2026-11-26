"""Repeat direct Macaulay rank diagnostic after adding implied chart equations."""
import sympy as s
from .veronese import system,geometric_system
from .veronese_fall import diagnostic

if __name__=='__main__':
    for p,a,b,m in [(5,0,1,3),(11,1,1,4),(19,2,3,5),(2147483647,1,1,3),(2147483647,1,1,4),(2147483647,1,1,5)]:
        x=s.symbols('x:'+str(m+1));aa,ee,_=system(p,a,b,x);ga,ge=geometric_system(a,b,x)
        for arm,aux,eq in [('veronese',aa,ee),('geo',ga,ge)]:
            ext=[]
            for i in range(m-2):
                z,r,h=s.symbols(f'z{i} r{i} h{i}');ext.append((1-z)*(r-1))
                if arm=='veronese':ext.append((1-z)*h)
            for row in diagnostic(eq+ext,[*x,*aux],p):print(p,m,arm,*row,sep=',',flush=True)
