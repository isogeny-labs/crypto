import json,glob,math,statistics as st
import numpy as np
cells=[json.load(open(f)) for f in sorted(glob.glob('raw/*.json')) if open(f).read().strip()]
cells.sort(key=lambda c:(c['d'],c['n'],c['k']))
ARMS=['A','Ap','B','C1','C2','D1','D2','D4']
out={'cells':[]}
def ms(v): return (st.mean(v), st.pstdev(v) if len(v)>1 else 0.0)
for c in cells:
    r=c['res'];row=dict(n=c['n'],k=c['k'],d=c['d'],draws=c['draws'],cpu=c['cpu'])
    for a in ARMS:
        low=[x[1] for x in r[a]];dim=[x[0] for x in r[a]];one=[x[2] for x in r[a]]
        row[a]=dict(low_mean=ms(low)[0],low_sd=ms(low)[1],dim_mean=ms(dim)[0],one_frac=sum(one)/len(one))
    row['Ap_equals_A_exact']= r['A']==r['Ap']
    mB=row['B']['low_mean']
    for a in ARMS: row[a]['excess_vs_B']=row[a]['low_mean']-mB
    def z(x,y):
        lx=[t[1] for t in r[x]];ly=[t[1] for t in r[y]];R=len(lx)
        se=math.sqrt(st.pvariance(lx)/R+st.pvariance(ly)/R)
        diff=st.mean(lx)-st.mean(ly)
        return 'inf' if se==0 and diff!=0 else (0.0 if se==0 else round(diff/se,2))
    row['zscore']={p:z(*p.split('-')) for p in ['A-B','C1-B','C2-B','D1-B','A-C1','A-C2','A-D1','D2-B']}
    out['cells'].append(row)
diag=[c for c in out['cells'] if c['d']==3 and c['k']==(c['n']+1)//2 and c['n']>=17]
ns=np.array([c['n'] for c in diag]);
for a in ['A','C1','C2','D1']:
    e=np.array([c[a]['excess_vs_B'] for c in diag]);X=np.vstack([ns,np.ones_like(ns)]).T
    coef,res,_,_=np.linalg.lstsq(X,e,rcond=None);resid=e-X@coef
    s2=(resid@resid)/max(1,len(e)-2);cov=s2*np.linalg.inv(X.T@X)
    out.setdefault('fit_e3_vs_n',{})[a]=dict(slope=coef[0],intercept=coef[1],slope_ci95=1.96*math.sqrt(cov[0,0]),pred_n131=coef[0]*131+coef[1])
out['total_cpu_s']=sum(c['cpu'] for c in cells)
json.dump(out,open('RESULTS.json','w'),indent=1)
for c in out['cells']:
    print(c['n'],c['k'],c['d'],'|',' '.join(f"{a}:{c[a]['low_mean']:.1f}/{c[a]['low_sd']:.1f}/1:{c[a]['one_frac']:.2f}" for a in ARMS),'| Ap==A',c['Ap_equals_A_exact'],c['zscore'])
print(json.dumps(out['fit_e3_vs_n'],indent=0),out['total_cpu_s'])
