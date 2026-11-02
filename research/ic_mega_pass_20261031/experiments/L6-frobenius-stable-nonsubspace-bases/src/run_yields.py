import sys, json, subprocess, os, math
sys.path.insert(0, os.path.dirname(__file__))
from field import *
T=os.environ['TMPDIR']; BIN=T+'/l6'; OUT=sys.argv[1]
res={}
def c(cmd): return json.loads(subprocess.run(cmd,capture_output=True,text=True,check=True).stdout)
def frob_orbits(S,xs):
    xs=set(xs); seen=set(); k=0
    for x in xs:
        if x in seen: continue
        k+=1; y=x
        while y not in seen: seen.add(y); y=S['mul'](y,y)
    return k
for n,plan in [(11,dict(cyc=[1,2,3],ham=[1,2])),(23,dict(cyc=[2],ham=[2,3]))]:
    S=setup(n); R={}
    R['f']=S['F']; R['p']=S['p']
    bases={}
    for w in plan['cyc']:
        d=cyc(S,w); bases[f'CYC{w}']=set(d)
        R[f'CYC{w}_reps']=dict(distinct=len(d),Cpw=math.comb(S['p'],w),multi_rep=sum(1 for v in d.values() if len(v)>1))
        if w==2: pairs=d
    for r in plan['ham']: bases[f'HAM{r}']=ham(S,r)
    R['overlap_CYC2_HAM']={k:len(bases['CYC2']&bases[k]) for k in bases if k.startswith('HAM')}
    if n==11: R['weight_le3_total']=1+23+253+1771
    for nm in list(bases):
        k=round(math.log2(len(bases[nm])))
        for s in range(5):
            vs,sp=subspace(S,k,1000*n+s); bases[f'SUB{k}_s{s}']=sp
    st=c([BIN,'selftest',str(n),str(S['F']),str(S['r']),'/dev/null']); R['selftest']=st
    Y={}
    for nm,xs in bases.items():
        fn=f'{T}/base_{n}_{nm}.txt'; open(fn,'w').write('\n'.join(map(str,sorted(xs))))
        big=len(xs)>3000
        y=c([BIN,'yields',str(n),str(S['F']),str(S['r']),fn,'0' if big else '1','16'])
        Bm=y['B_points']; r=S['r']
        y['stable']=not nm.startswith('SUB')
        subx=[]  # recompute columns: orbits of subgroup x's
        y['size']=len(xs)
        if y['stable']: y['frob_orbits_all']=frob_orbits(S,xs)
        y['N2_null']=Bm**3/r; y['N3_null']=Bm**4/r
        y['N2_ratio']=y['N2']/y['N2_null'] if Bm else None
        y['N3_ratio']=(y['N3']/y['N3_null']) if (Bm and y['N3']>=0) else None
        cols=y['subgroup_x']/(n if y['stable'] else 1)
        y['columns']=cols
        y['N2_per_column']=y['N2']/cols if cols else None
        y['N3_per_column']=(y['N3']/cols) if (cols and y['N3']>=0) else None
        y['decomp_per_target_mean']=sum(t[2] for t in y['targets'])/len(y['targets'])
        y['decomp_null']=Bm*Bm/r
        N=r/(2*n); y['BD2_over_N']=cols*Bm**2/N
        Y[nm]=y; R['bases']=Y; json.dump(res|{n:R},open(OUT,'w'),indent=1); print(n,nm,len(xs),Bm,y['N2_ratio'],y['N3_ratio'],flush=True)
    R['bases']=Y; res[n]=R
json.dump(res,open(OUT,'w'),indent=1)
