"""Ring/identity checks in pure Python: principal roots, four-step == direct DFT, no-wraparound."""
import sys, random, json
sys.path.insert(0,'.')
from gadget import *
from test_gadget import clmul

def mulx(N,a):
    a<<=1
    if a>>(2*N)&1: a^=(1<<(2*N))|(1<<N)|1
    return a
def principal(N):
    pw=[1]
    for _ in range(3*N): pw.append(mulx(N,pw[-1]))
    if pw[3*N]!=1: return False
    # order exactly 3N and character sums vanish
    for j in range(1,3*N):
        s=0
        for i in range(3*N): s^=pw[(j*i)%(3*N)]
        if s: return False
    return True
res={}
for N in (3,9,27,81):
    res[f'principal_N{N}']=principal(N)
# four-step vs direct DFT on random words in B_{N1}
rng=random.Random(7)
for nu in (3,4,5):
    N1,N2=split(nu); L=3*N2; s=N1//N2; P,Q=factor_L(L); W=2*N1
    C=Circ(0); words=[]; asg={}
    for i in range(L):
        w=[]
        for b in range(W):
            v=C.new(); w.append(frozenset([v])); asg[v]=rng.getrandbits(1)
        words.append(w)
    F4=dft4(C,words,s,N1,P,Q,+1,set(range(L)),'t')
    Dd=dft_small(words,s,N1)
    evaluate(C.gates,asg)
    ok=all(sum(0 for _ in ()) or ( [sum(asg[x] for x in e)%2 for e in F4[k]]==[sum(asg[x] for x in e)%2 for e in Dd[k]]) for k in range(L))
    res[f'fourstep_eq_direct_nu{nu}_L{L}_PxQ{P}x{Q}']=bool(ok)
# no-wraparound bound: product degree 2n-2 < 2N iff n<=N ; exhaustive-ish: gadget correct for n<=N, and the ring product wraps for n=N+1
def ring_mul(N,a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a=mulx(N,a)
    return r
bad=0
for N in (3,9,27):
    for n in range(1,N+1):
        a=rng.getrandbits(n); b=rng.getrandbits(n)
        assert ring_mul(N,a,b)==clmul(a,b)       # no wrap when 2n-1 <= 2N
    # failure at n=N+1 for some operands
    wrapped=sum(ring_mul(N,rng.getrandbits(N+1)|1<<N,rng.getrandbits(N+1)|1<<N)!=0 and False for _ in range(1))
    found=False
    for _ in range(200):
        a=rng.getrandbits(N+2)|(1<<(N+1)); b=rng.getrandbits(N+2)|(1<<(N+1))
        if a>>(2*N)==0 and b>>(2*N)==0 and ring_mul(N,a,b)!=clmul(a,b): found=True;break
    res[f'no_wrap_n_le_N_N{N}']=True
    res[f'wrap_occurs_beyond_N{N}']=found
print(json.dumps(res,indent=1))
json.dump(res,open('../raw/identity_checks.json','w'),indent=1)
