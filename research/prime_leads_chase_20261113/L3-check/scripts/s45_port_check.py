# Port of pcdeg's s4/s5 (res22 + 6x6 Sylvester over interpolated S4) compared with sympy resultants at random points,
# and vanishing on genuine 4- and 5-term decompositions.
import random, importlib.util, sys
spec = importlib.util.spec_from_file_location("st", "/tmp/claude-0/-home-user/56e3c4bf-6e3c-5268-b3a1-8d397ec01ddb/scratchpad/chase/L3-check/scripts/semaev_top_lib.py")
st = importlib.util.module_from_spec(spec); spec.loader.exec_module(st)
p, a, b = st.p, st.a, st.b
def s3coef(x1,x2):
    d=(x1-x2)%p; c2=d*d%p; s=(x1+x2)%p; pr=x1*x2%p
    c1=(-2*((s*((pr+a)%p)+2*b)%p))%p; t=(pr-a)%p; c0=(t*t-4*b*s)%p
    return c2,c1,c0
def res22(f,g):
    a2,a1,a0=f; b2,b1,b0=g
    t1=(a2*b0-a0*b2)%p; t2=(a2*b1-a1*b2)%p; t3=(a1*b0-a0*b1)%p
    return (t1*t1-t2*t3)%p
def s4(x1,x2,x3,r): return res22(s3coef(x1,x2),s3coef(x3,r))
def det(M):
    M=[row[:] for row in M]; n=len(M); d=1
    for c in range(n):
        r0=next((r for r in range(c,n) if M[r][c]%p),None)
        if r0 is None: return 0
        if r0!=c: M[r0],M[c]=M[c],M[r0]; d=-d
        d=d*M[c][c]%p; iv=pow(M[c][c],p-2,p)
        for r in range(c+1,n):
            f=M[r][c]*iv%p
            for k in range(c,n): M[r][k]=(M[r][k]-f*M[c][k])%p
    return d%p
def s5(x1,x2,x3,x4,r):
    ys=[s4(u,x3,x4,r) for u in range(5)]
    g=st.interp_mod(list(range(5)),ys); g=[g[i] for i in range(5)]
    c2,c1,c0=s3coef(x1,x2)
    M=[[0]*6 for _ in range(6)]
    for i in range(4): M[i][i]=c2; M[i][i+1]=c1; M[i][i+2]=c0
    for i in range(2):
        for j in range(5): M[4+i][i+j]=g[4-j]
    return det(M)
random.seed(99)
r4=set(); r5=set()
for _ in range(6):
    x=[random.randrange(p) for _ in range(5)]
    A=s4(*x[:4]); B=st.S4(*x[:4],a,b); r4.add(A*pow(B,p-2,p)%p if B else ('zero',A))
    A=s5(*x); B=st.S5(*x,a,b); r5.add(A*pow(B,p-2,p)%p if B else ('zero',A))
print('pcdeg-port s4 / sympy S4 ratio set:', r4)
print('pcdeg-port s5 / sympy S5 ratio set:', r5)
