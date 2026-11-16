#!/usr/bin/env python3
"""CT24 independent F101^2 pointwise replay of reciprocal genus2 fibers.

Does not import the CT24 producer, CT23, or any Jacobian code.
[pcc:unverified]
"""
p=101
curves=[('B+A',92,44,33,7),('B+B',26,42,68,21)]

def m(x):return x%p

def inv(x):return pow(x,-1,p)

def mul(x,y):
    return (m(x[0]*y[0]+2*x[1]*y[1]),m(x[0]*y[1]+x[1]*y[0]))
def add(x,y):return (m(x[0]+y[0]),m(x[1]+y[1]))
def neg(x):return (m(-x[0]),m(-x[1]))
def power(x,k):
    y=(1,0)
    while k:
        if k&1:y=mul(y,x)
        x=mul(x,x);k//=2
    return y

def root(t):
    if t==0:return (0,0)
    for s in range(p):
        if m(s*s)==t:return (s,0)
    # 2 nonsquare mod 101 => t/2 is square if t is nonsquare.
    for s in range(p):
        if m(2*s*s)==t:return (0,s)
    raise AssertionError('Fp2 square root missing')

def replay():
    for name,A,B,b,scale in curves:
        c2=B;c1=m(b*A);c0=m(b*b)
        shift=m(c2*inv(3));a=m(c1-c2*shift);bb=m(c0-c1*shift+2*shift**3)
        assert (m(a*scale**4),m(bb*scale**6))==(26,58)
        square_roots=[z for z in range(p) if m(z*z)==b]
        assert len(square_roots)==2
        split=nonsplit=inf=0
        seen=set()
        for X in range(p):
            for Y in range(p):
                if m(Y*Y-X**3-26*X-58):continue
                U=m(X*inv(scale**2)-shift);V=m(Y*inv(scale**3))
                assert m(V*V-U**3-c2*U*U-c1*U-c0)==0
                if U==0:
                    assert V in (b,m(-b))
                    inf+=1;seen.add(('inf',V));continue
                r=m(b*inv(U));t=root(r)
                y=mul((m(V*inv(U)),0),t)
                for t0,y0 in ((t,y),(neg(t),neg(y))):
                    lhs=mul(y0,y0)
                    t2=mul(t0,t0)
                    t4=mul(t2,t2);t6=mul(t4,t2)
                    rhs=add(add(add(t6,mul((A,0),t4)),mul((B,0),t2)),(b,0))
                    assert lhs==rhs
                    # Check quotient directly over Fp2 rather than producer polynomial reduction.
                    assert mul((b,0),power(t0,p*p-3))==(U,0)
                    assert mul((b,0),mul(y0,power(t0,p*p-4)))==(V,0)
                seen.add(('finite',r,m(V*inv(U))))
                if t[1]==0:split+=1
                else:nonsplit+=1
        assert (split,nonsplit,inf)==(58,48,2)
        assert len(seen)==108
        print(name,'Fp2 replay 58 split/48 nonsplit/2 infinity, both point images and quotient identities pass [pcc:unverified]')
    print('PASS separate Fp2 support oracle; no signed PCC or Jacobian group law')
if __name__=='__main__':replay()
