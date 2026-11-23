#!/usr/bin/env python3
"""CT329 independent-language same-producer finite-field / BSGS count oracle.

No cryptographic or statistical claim. Recomputes original F103 known answers
and group-add counts without reading Rust's instrumented results.
"""
P=103
R=109
G=(1,51)

def m(x): return x%P

def add(a,z):
    if a is None: return z
    if z is None: return a
    x,y=a
    u,v=z
    if x==u and m(y+v)==0: return None
    if x==u:
        den=m(2*y)
        num=m(3*x*x+1)
    else:
        den=m(u-x)
        num=m(v-y)
    if den==0: raise AssertionError("undefined slope")
    lam=m(num*pow(den,P-2,P))
    xx=m(lam*lam-x-u)
    return xx,m(lam*(x-xx)-y)

def mul(k,g):
    out=None
    while k:
        if k&1: out=add(out,g)
        g=add(g,g)
        k>>=1
    return out

def mul_add_count(k):
    return k.bit_count()+k.bit_length() if k else 0

def solve_and_count(q):
    n=11
    baby=[]
    cur=None
    for j in range(n):
        baby.append((cur,j))
        cur=add(cur,G)
    step=mul(n,(G[0],m(-G[1])))
    giant=q
    for i in range(n+1):
        for p,j in baby:
            if p==giant:
                k=(i*n+j)%R
                assert mul(k,G)==q
                return (k, n+i, n, mul_add_count(n),i,mul_add_count(k), n+i+mul_add_count(n)+mul_add_count(k))
        giant=add(giant,step)
    raise AssertionError("BSGS missing target")

def main():
    assert mul(R,G) is None
    assert mul(73,G)==(42,71)
    rows=[]
    for k in range(R):
        row=solve_and_count(mul(k,G))
        assert row[0]==k
        assert row[1]==row[2]+row[4]
        assert row[-1]==sum(row[2:6])
        assert mul((k+1)%R,G)!=mul(k,G)
        rows.append(row)
    assert rows[73]==(73,17,11,7,6,10,34)
    assert (min(x[-1] for x in rows),max(x[-1] for x in rows))==(18,39)
    print("CT329_PY_ORACLE_PASS logs=109 q73_partial=17 q73_total=34 min_total=18 max_total=39 pcc=unverified")

if __name__=="__main__": main()
