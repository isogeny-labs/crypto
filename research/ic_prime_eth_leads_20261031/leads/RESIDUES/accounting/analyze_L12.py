import sys, math, random, os
random.seed(1)
R='../../L12/raw/'
cells=[('m2_b12_l4',2),('m2_b16_l4',2),('m3_b12_l3',3),('m3_b16_l3',3)]
def load(f):
    r=[]
    for l in open(f):
        p=l.split('\t')
        if len(p)<8: continue
        r.append(dict(kind=p[1],sat=int(p[3])==10,cen=int(p[3])==0,conf=int(p[4]),prop=int(p[5]),cpu=float(p[7])))
    return r
def header(c):
    h=open(R+c+'/targets.tsv').readline()
    return dict(t.split('=') for t in h.strip('# \n').split() if '=' in t)
def nF(p,a,b,l):
    k=0
    for x in range(2**l):
        f=(x**3+a*x+b)%p
        if f==0 or pow(f,(p-1)//2,p)==1: k+=1
    return k
out=[]
out.append('cell | |F| | pts 2|F| | N | inc_total_conf | fresh_total_conf (censored) | inc_relations | fresh_relations | tail_per_target(last quarter) | setup_est=inc-N*tail | inc conf/relation | enum_ops: setup (2|F|)^(m-1) + N*2|F| | inc conf per enum op | break-even N vs fresh')
for c,m in cells:
    h=header(c); F=nF(int(h['p']),int(h['a']),int(h['b']),int(h['l'])); P=2*F
    inc=load(R+c+'.inc.tsv'); fr=load(R+c+'.fresh.tsv')
    Nmax=min(len(inc),len(fr))
    for N in sorted({16,64,Nmax}):
        if N>Nmax: continue
        I=inc[:N]; Fr=fr[:N]
        ti=sum(x['conf'] for x in I); tf=sum(x['conf'] for x in Fr); cf=sum(x['cen'] for x in Fr)
        q=max(1,N//4); tail=sum(x['conf'] for x in I[-q:])/q
        setup=max(0,ti-N*tail)
        ri=sum(x['sat'] for x in I); rf=sum(x['sat'] for x in Fr)
        ops=P**(m-1)+N*P
        fmean=tf/N
        be = (setup/(fmean-tail)) if fmean>tail else float('nan')
        out.append(f"{c} | {F} | {P} | {N} | {ti} | {tf} ({cf}/{N} cens) | {ri} | {rf} | {tail:.0f} | {setup:.0f} | {ti/ri if ri else float('nan'):.0f} | {ops} | {ti/ops:.0f} | {be:.1f}")
open('raw/L12_restated.txt','w').write('\n'.join(out)+'\n'); print('\n'.join(out))
