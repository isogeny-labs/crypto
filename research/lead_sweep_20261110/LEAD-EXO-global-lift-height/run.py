import json,math,random,sys
smoke='--smoke' in sys.argv
BMAX=24 if smoke else 200
PS=[7,11] if smoke else [7,11,13,17,19,23]
SEED=20261110
# enumerate integer (a,b,x,y), a,x,y in [0,BMAX), b=y^2-x^3-a*x, height=max(a,x,y,|b|)<=BMAX
sols=[]
for a in range(BMAX):
    for x in range(BMAX):
        ax=a*x+x**3
        for y in range(BMAX):
            b=y*y-ax
            if -BMAX<=b<=BMAX:
                sols.append((max(a,x,y,abs(b)),a,b,x,y))
sols.sort()
def N(B):
    lo,hi=0,len(sols)
    while lo<hi:
        m=(lo+hi)//2
        if sols[m][0]<=B: lo=m+1
        else: hi=m
    return lo
def slope(xs,ys):
    lx=[math.log(v) for v in xs];ly=[math.log(v) for v in ys];n=len(xs)
    mx=sum(lx)/n;my=sum(ly)/n
    return sum((a-mx)*(b-my) for a,b in zip(lx,ly))/sum((a-mx)**2 for a in lx)
Bs=[b for b in (6,8,12,16,24,32,48,64,96,128,200) if b<=BMAX]
Bs=[b for b in Bs if N(b)>0]
cnt=[N(b) for b in Bs]
s_real=slope(Bs,cnt)
# positive control: b free in [0,B^3): count solutions grows ~B^3 (slope 3)
pos=[]
for B in Bs[:6]:
    c=0
    for a in range(B):
        for x in range(B):
            for y in range(B):
                c+=1  # every (a,x,y) yields an integer b of height <=B^3 by construction
    pos.append(c)
s_pos=slope(Bs[:6],pos)
# null: random integer tuples (a,b,x,y) in cube [0,B)^4 are on-curve mod p w.p. 1/p; A/A: two seeds
def nullrate(seed,p,n):
    r=random.Random(seed);h=0
    for _ in range(n):
        a,b,x,y=(r.randrange(p*50) for _ in range(4))
        h+=((y*y-x**3-a*x-b)%p==0)
    return h/n
n=2000 if smoke else 60000
null1=nullrate(SEED,11,n);null2=nullrate(SEED+1,11,n)
# coverage: targets (a,b,x,y) mod p on curve (p^3 of them); B50 = least B covering >=50%
B50={}
for p in PS:
    seen=set();target=p**3;found=None
    for h,a,b,x,y in sols:
        seen.add((a%p,b%p,x%p,y%p))
        if found is None and len(seen)>=0.5*target: found=h;break
    B50[p]=found
ok=[p for p in PS if B50[p]]
s_b50=slope(ok,[B50[p] for p in ok]) if len(ok)>=3 else None
if smoke: verdict="inconclusive"
elif 1.5<=s_real<=2.5 and abs(s_pos-3)<0.1 and abs(null1-1/11)<0.01 and abs(null2-1/11)<0.01 and s_b50 is not None and s_b50>=1.0:
    verdict="consistent_with_prediction"
elif s_real<1.2 or (s_b50 is not None and s_b50<0.5): verdict="falsified"
else: verdict="inconclusive"
print(json.dumps({"lead":"LEAD-EXO-global-lift-height","smoke":smoke,"seed":SEED,"BMAX":BMAX,"primes":PS,
 "B_grid":Bs,"N_B":cnt,"slope_N_vs_B":s_real,"positive_control_slope":s_pos,
 "null_rate_seed1":null1,"null_rate_seed2":null2,"null_expected":1/11,
 "B50":B50,"slope_B50_vs_p":s_b50,"verdict":verdict}))
