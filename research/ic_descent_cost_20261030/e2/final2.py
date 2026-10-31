import math, statistics as st, json, os, sys
sys.argv=['x']; exec(open('final.py').read().split("rows=[]")[0])
RK={41:10,51:4,53:23,57:7,59:3,61:35}
def summ(label,items):
    rows=[]
    for n,base,rec in items:
        out,cuts=row(n,base,rec); lc=[math.log2(c) for c in cuts]
        rows.append((n,base,rec,out,cuts))
        mb=lambda i,j: st.mean(o[i][j] for o in out)
        print(f'{label} n={n} base K{base[0]}N{base[1]}s{base[2]} S0={mb(0,0):.3g} d={mb(0,1):.3g} | rec K{rec[0]}s{rec[2]} S0={mb(1,0):.3g} d={mb(1,1):.3g} | cut gm={2**st.mean(lc):.2f} sd={st.stdev(lc):.2f} seeds={[round(c,2) for c in cuts]}')
    x=[r[0] for r in rows]; y=[st.mean(math.log2(c) for c in r[4]) for r in rows]
    if len(x)>3:
      for lab,xs in (('n',x),('log2 r',[math.log2(R[n]) for n in x])):
        b,se=ols(xs,y); t=T[len(xs)-2]; print(f'  {label} slope log2(cut) vs {lab}: {b:.4f} [{b-t*se:.4f},{b+t*se:.4f}]')
    return rows
ns=[41,51,53,57,59,61]
S={n:sel(n) for n in ns}
rec={n:(S[n][1][1],16,1) for n in ns}
summ('A rule-baseline',[(n,(RK[n],16,0),rec[n]) for n in ns])
summ('B S-argmin baseline',[(n,(S[n][0][1],16,0),rec[n]) for n in ns])
orig={41:((12,16,0),(96,16,1)),53:((20,16,0),(80,16,1)),61:((18,32,0),(112,16,1))}
summ('C original recipes',[(n,)+orig[n] for n in orig])
for n in ns:
    pl=min(x[2]*x[3] for x in S[n][(0,'all')]); sy=S[n][1][0]
    print('decomp',n,'best-plain/best-sym S0d',round(pl/sy,2),'rec K',rec[n][0])
