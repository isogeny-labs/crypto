# Analysis of COMMITTED numbers only: re-evaluates the committed cost model
# research/notes/ecc2k130/descent_degree_cost_20260930.py (copied functions, unchanged formulae)
# to read how much of the m=3/m=4 projected total is the oracle, and what one degree is worth.
from math import comb, factorial, log2
N_FIELD=131; RHO=60.81
def log2_add(a,b): return max(a,b)+log2(1+2**(-abs(a-b)))
def cost(m,l,degree,w,c=1.0):
    n=N_FIELD
    unknowns=(m-2)*n+m*l
    per_relation=max(0.0,log2(factorial(m))+n-m*l)
    targets=l+per_relation
    oracle=(w*log2(sum(comb(unknowns,i) for i in range(degree+1)))+log2(c)) if degree and c>0 else float('-inf')
    algebra=log2(m)+2*l
    tot = log2_add(targets+oracle,algebra) if oracle!=float('-inf') else log2_add(targets,algebra)
    return tot,targets,oracle,algebra,unknowns
def best(m,degree,w,c=1.0):
    return min((cost(m,l,degree,w,c)[0],l) for l in range(4,70))
print("== reproduce committed rows (m,D,w) -> total, vs rho")
for m,D,w in [(4,2,1),(4,4,1),(4,5,1),(4,4,2),(4,5,2),(4,2,2),(3,4,1),(3,5,1)]:
    t,l=best(m,D,w); print(m,D,w,'l=%d'%l,'total 2^%.2f'%t,'vs rho %+.2f'%(t-RHO))
print("== NEW (derived from the same model): degree 3 rows")
for m in (3,4):
    for w in (1,2):
        t,l=best(m,3,w); print('m',m,'D=3 w',w,'l=%d'%l,'total 2^%.2f'%t,'vs rho %+.2f'%(t-RHO))
print("== value of one closure degree at m=4 (bits of total), w=1 and w=2")
for w in (1,2):
    T={D:best(4,D,w)[0] for D in (2,3,4,5,6)}
    print('w',w,{D:round(T[D],2) for D in T}, 'steps', {f'{D+1}->{D}':round(T[D+1]-T[D],2) for D in (2,3,4,5)})
print("== free-oracle floor m=4:", round(best(4,0,1)[0],2), " m=3:", round(best(3,0,1)[0],2))
print("== oracle share inside the D=4,w=1 optimum (m=4, l=33)")
t,tg,o,a,u=cost(4,33,4,1); print('targets 2^%.2f oracle/target 2^%.2f -> targets*oracle 2^%.2f ; LA 2^%.2f ; total 2^%.2f ; oracle share %.3f'%(tg,o,tg+o,a,t,2**(tg+o-t)))
print("== Amdahl sweep: oracle cost multiplier c at fixed degree (re-optimising l), m=4, w=1")
for D in (4,5):
    base=best(4,D,1)[0]
    row=[]
    for c in (1,0.5,0.25,0.1,0.01,0.001,0):
        t,l=best(4,D,1,c) if c>0 else best(4,0,1)
        row.append((c,round(base-t,2),'l=%d'%l,round(t-RHO,2)))
    print('D',D,'base 2^%.2f'%base, row)
print("== same, m=4, w=2")
for D in (4,5):
    base=best(4,D,2)[0]
    row=[]
    for c in (1,0.5,0.1,0.01,0.001,0):
        t,l=best(4,D,2,c) if c>0 else best(4,0,1)
        row.append((c,round(base-t,2),'l=%d'%l,round(t-RHO,2)))
    print('D',D,'base 2^%.2f'%base, row)
