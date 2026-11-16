import sys,math,json
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import numpy as np
import recompute as R
# Student t two-sided p via tail integration with Gauss-Legendre (independent of decide.py's incomplete beta)
xs,ws=np.polynomial.legendre.leggauss(400)
u=0.5*(xs+1); wu=0.5*ws
def p_two(T,df=7):
    T=abs(T)
    if T==0: return 1.0
    c=math.gamma((df+1)/2)/(math.sqrt(df*math.pi)*math.gamma(df/2))
    x=T/u
    f=c*(1+x*x/df)**(-(df+1)/2)
    tail=np.sum(wu*f*T/(u*u))
    return min(1.0,2*tail)
# sanity: p(2.364624,7)~0.05, p(5.0016,7)=0.0015625
print('sanity',p_two(2.3646243),p_two(5.0016))
def t975(df=7):
    lo,hi=0,10
    for _ in range(200):
        mid=(lo+hi)/2
        if p_two(mid,df)>0.05: lo=mid
        else: hi=mid
    return lo
T975=t975(); print('t975',T975)
def lin(a,b):
    sa=sum(a); sb=sum(b); r=sb/sa; K=len(a)
    uu=[(y-r*x)/sa for x,y in zip(a,b)]
    ub=sum(uu)/K; ss=sum((x-ub)**2 for x in uu)
    se=math.sqrt(K/(K-1)*ss)
    return r,se
def cell(recs,A,B,curve):
    ps=R.pairs(recs,A,B,curve); out=[]
    for w in [0.0,0.1,1.0]:
        a=[R.priced_s(x,w) for x,_ in ps]; b=[R.priced_s(y,w) for _,y in ps]
        r,se=lin(a,b); half=T975*se/r
        lo,hi=r*math.exp(-half),r*math.exp(half)
        cv=se/r  # equals ln(hi/r)/t975
        T=math.log(r)/cv if cv>0 else float('inf')
        out.append(dict(w=w,R=r,lo=lo,hi=hi,cv=cv,T=T,p=p_two(T) if cv>0 else 0.0,mean_a=sum(a)/len(a),mean_b=sum(b)/len(b)))
    return out
if __name__=='__main__':
    C1014=R.load('/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl')
    C16=R.load('/dev/shm/restore/v2/sessions/C-16/records.jsonl')
    B=R.load('/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')
    dec=json.load(open(''+os.path.dirname(os.path.abspath(__file__))+'/../results/decision.json'))
    byid={(c['comparison'],c['slug'].split('-')[1]):c for c in dec['cells']}
    fam=[('C-rho','C','rho-neg','tensor-m2'),('C-ic','C','ic-mitm','tensor-m2'),('B-rho-selectors','B','rho-neg','native-selectors'),('B-rho-partial','B','rho-neg','native-partial'),('B-rho-ic','B','rho-neg','ic-mitm')]
    mine=[]
    for cid,t,A,Bm in fam:
        for c in ['fp10','fp12','fp14','fp16']:
            recs = (C1014 if c!='fp16' else C16) if t=='C' else B
            ps=R.pairs(recs,A,Bm,c)
            if len(ps)!=8: continue
            # complete only if all 16 rows verified (8+8): pairs counts() handles verified filter
            o=cell(recs,A,Bm,c)
            pc=max(x['p'] for x in o)
            d=byid[(cid,c)]
            order_ok=all(x['mean_a']<x['mean_b'] for x in o)
            mine.append((cid,c,pc))
            print(f"{cid:16s}{c} R={[round(x['R'],1) for x in o]} p_mine={pc:.3g} p_dec={d['p']:.3g} ci_mine_w0=({o[0]['lo']:.4g},{o[0]['hi']:.4g}) ci_dec_w0={d['ci95_linearised_t'][0]} order_ok={order_ok}")
