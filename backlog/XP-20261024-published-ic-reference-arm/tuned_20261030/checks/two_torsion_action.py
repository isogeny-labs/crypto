"""D6a check: on K_0 (a=0, b=1) over F_2^9, x(P+T) = 1/x(P) for T=(0,1), and V=<1,z,z^2,z^3> is not stable under it."""
n=9; M=(1<<9)|(1<<4)|1
def mul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>n&1: a^=M
    return r
def inv(a):
    r=1;e=(1<<n)-2
    while e:
        if e&1:r=mul(r,a)
        a=mul(a,a);e>>=1
    return r
A=0;B=1
pts=[(x,y) for x in range(1<<n) for y in range(1<<n) if mul(y,y)^mul(x,y)==mul(mul(x,x),x)^mul(A,mul(x,x))^B]
T=(0,1)
def addx(P,Q):
    (x1,y1),(x2,y2)=P,Q
    l=mul(y1^y2,inv(x1^x2)); return mul(l,l)^l^x1^x2^A
print(len(pts),'affine points; x(P+T)==1/x(P) for all x!=0:', all(addx(P,T)==inv(P[0]) for P in pts if P[0]!=0))
V=set(range(16)); print('nonzero v in V with 1/v in V:', sum(inv(v) in V for v in V if v), 'of 15')
