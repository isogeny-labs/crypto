"""Validation ladder (must pass before any real arm). Output -> raw/validate.json"""
import sys, json, numpy as np, cypari2, time
sys.path.insert(0,'.'); from lib import *
pari=cypari2.Pari(); out={}; C=load_curves()
# 1. enumeration vs PARI ellmul
chk=[]
for c in [x for x in C if x['n'] in (7,13)]:
    for mode in 'PE':
        X,g=enumerate_x(c,mode,seed=3); n=c['n']
        pari(f"T=Pol(binary({c['poly']}),'w)"); pari("w=ffgen(T*Mod(1,2),'w)")
        pari(f"mk(m)=sum(i=0,{n}-1,bittest(m,i)*w^i)"); pari("tob(e)=subst(lift(e.pol),'w,2)")
        pari(f"E=ellinit([1,{c['a']},0,0,mk({c['b']})],w)"); pari(f"G=[mk({g['Gx']}),mk({g['Gy']})]")
        idx=list(range(1,min(40,len(X))))+[int(v) for v in np.random.default_rng(1).integers(1,len(X),150)]
        bad=0
        for k in idx:
            P=pari(f"ellmul(E,G,{k})")
            v=0xFFFFFFFF if len(P)==1 else int(pari(f"tob(ellmul(E,G,{k})[1])"))
            bad+= v!=int(X[k])
        sym=bool(np.all(X[1:]==X[1:][::-1]))
        chk.append({'curve':c,'mode':mode,'L':len(X),'checked':len(idx),'mismatch':bad,'ellorder':int(pari('ellorder(E,G)')),'x_even':sym})
out['enum_vs_pari']=chk; print(chk[-1])
# 2. fast gain == brute force, n=7 curves, random masks j=1..3, both domains, plus random labels j=1..3
rng=np.random.default_rng(7); bf=[]
for c in [x for x in C if x['n']==7]:
    for mode in 'PE':
        X,_=enumerate_x(c,mode,1)
        for j in (1,2,3):
            for rep in range(3):
                h=rng.integers(0,1<<j,len(X)) if rep==0 else labels(X,[int(v) for v in rng.integers(1,128,j)])
                r=gain(h,j)
                if r is None: bf.append({'skip':'empty bucket'}); continue
                b=gain_brute(h,j); bf.append({'n':7,'mode':mode,'j':j,'fast':r['G'],'brute':b,'absdiff':abs(r['G']-b)})
out['fast_vs_brute']=bf; out['fast_vs_brute_maxdiff']=max(x.get('absdiff',0) for x in bf); print('maxdiff',out['fast_vs_brute_maxdiff'])
# 3. positive control: trace filter. Tr mask: bit i = Tr(w^i).
def trmask(c):
    n=c['n']; pari(f"T=Pol(binary({c['poly']}),'w)"); pari("w=ffgen(T*Mod(1,2),'w)"); return int(pari(f"sum(i=0,{n}-1,lift(trace(w^i))*2^i)"))
pc=[]
for c in [x for x in C if x['n'] in (13,15)]:
    tm=trmask(c)
    for mode in 'EP':
        X,_=enumerate_x(c,mode,1); Xv=X[X!=SENT]; p=parity(Xv&np.uint32(tm))
        rec={'n':c['n'],'kind':c['kind'],'a':c['a'],'h':c['h'],'mode':mode,'tr_values':sorted(set(p.tolist())),'Tr_a':c['a']%2 if True else None}
        if p.min()!=p.max():
            r=gain(labels(X,[tm]),1); rec['G_trace']=r['G']; rec['delta_gain']=r['delta_gain']
        else: rec['G_trace']='degenerate (constant on domain)'
        pc.append(rec); print(rec)
out['trace_control']=pc
json.dump(out,open('../raw/validate.json','w'),indent=1,default=str)
