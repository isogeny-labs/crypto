import json, os, collections
from scipy.stats import beta
HERE=os.path.dirname(os.path.abspath(__file__)); R=os.path.join(HERE,'..','raw')
def cp_up(x,n): return 1.0 if x==n else round(float(beta.ppf(0.95,x+1,n-x)),4)   # one-sided 95% upper bound on rate
L=lambda f:[json.loads(l) for l in open(f) if l.strip().startswith('{')]
g=L(os.path.join(R,'n33_k17_d4_gf2close.jsonl'))
l2=[r for r in L(os.path.join(HERE,'..','..','L2-d5-and-n33-control','raw','n33_k17_d4.jsonl')) if 'arm' in r]
out={'n33':{},'n33_seeds_scanned':max(r['s'] for r in g)+1}
for a in ['A','C1','C2','D1','B','RM']:
    rs=[r for r in g if r['arm']==a]; u=[r for r in rs if r['nsol']==0]
    ones=sum(r['one'] for r in u)
    out['n33'][a]=dict(draws=len(rs),unsat=len(u),one_in_W4=ones,
        upper95_rate_minority=cp_up(min(ones,len(u)-ones),len(u)),
        low4=sorted(collections.Counter(r['low4'] for r in u).items()),
        dim4=sorted(collections.Counter(r['dim4'] for r in u).items()),
        unsat_seeds=[r['s'] for r in u],cpu_closure=round(sum(r['cpu'] for r in u),1))
# P5 agreement with L2
key=lambda r:(r['arm'],r['s'])
gm={key(r):r for r in g if r['nsol']==0}; cmp=[(k,gm[k],r) for r in l2 if r['nsol']==0 and (k:=key(r)) in gm]
out['P5_L2_vs_gf2close']=dict(common=len(cmp),mismatch=[k for k,a,b in cmp if (a['dim4'],a['low4'],a['one'])!=(b['dim4'],b['low4'],b['one'])],
   nsol_mismatch=sum(1 for r in l2 for q in g if key(r)==key(q) and r['nsol']!=q['nsol']))
f27=os.path.join(R,'n27_macclose.jsonl')
if os.path.exists(f27):
    m=L(f27); out['n27_macclose']={a:dict(unsat=len([x for x in m if x['arm']==a]),one_in_W4=sum(x['mac']['one'] for x in m if x['arm']==a),
        low4=sorted(set(x['mac']['low'] for x in m if x['arm']==a)),mismatch_vs_N1=[x['s'] for x in m if x['arm']==a and x['mac']!=x['n1']]) for a in ('A','C1','D1')}
out['cpu_total_s']=round(sum(r.get('cpu',0)+r['cpu_sols'] for r in g),1)
print(json.dumps(out,indent=1))
