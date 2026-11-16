import sys,json
import os; sys.path.insert(0,os.path.dirname(os.path.abspath(__file__)))
import recompute as R
C1014=R.load('/dev/shm/restore/snapshot-1730/sessions/C-10-14/records.jsonl')
C16=R.load('/dev/shm/restore/v2/sessions/C-16/records.jsonl')
B=R.load('/dev/shm/restore/snapshot-1730/sessions/B/records.jsonl')
dec=json.load(open(''+os.path.dirname(os.path.abspath(__file__))+'/../results/decision.json'))
fam={'C-rho':('C','rho-neg','tensor-m2'),'C-ic':('C','ic-mitm','tensor-m2'),'B-rho-selectors':('B','rho-neg','native-selectors'),'B-rho-partial':('B','rho-neg','native-partial'),'B-rho-ic':('B','rho-neg','ic-mitm')}
mx=0;n=0
for c in dec['cells']:
    if not c['complete']: continue
    t,A,Bm=fam[c['comparison']]; cv=c['slug'].split('-')[1]
    recs=(C1014 if cv!='fp16' else C16) if t=='C' else B
    for i,w in enumerate([0.0,0.1,1.0]):
        r,_=R.ratio(recs,A,Bm,cv,w); d=abs(r/c['ratio'][i]-1); mx=max(mx,d); n+=1
print('complete-cell ratios compared:',n,'max rel diff vs decision.json:',mx)
# RESULTS.md table values (2 sig fig) vs recomputed
tab={'fp10':(4.3e3,5.0e3,9.5e3,259,304,630),'fp12':(2.7e4,3.1e4,5.0e4,3.0e3,3.5e3,7.2e3),'fp14':(1.7e5,1.9e5,3.0e5,2.1e4,2.4e4,4.6e4),'fp16':(6.7e5,7.6e5,1.2e6,1.2e5,1.4e5,2.2e5)}
for cv,vals in tab.items():
    recs=C16 if cv=='fp16' else C1014
    mine=[R.ratio(recs,'rho-neg','tensor-m2',cv,w)[0] for w in (0,0.1,1)]+[R.ratio(recs,'ic-mitm','tensor-m2',cv,w)[0] for w in (0,0.1,1)]
    print(cv,[f"{m:.4g}->{v:.3g}({(v/m-1)*100:+.1f}%)" for m,v in zip(mine,vals)])
