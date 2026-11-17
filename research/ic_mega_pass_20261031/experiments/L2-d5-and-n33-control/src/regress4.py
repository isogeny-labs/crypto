import json
from l2lib import *
R=os.path.join(HERE,'..','..','N1-gf2-d4-profile','raw')
recs=[json.loads(l) for l in open(R+'/n19_k10_d4.jsonl') if '"one"' in l]
recs+= [r for r in map(json.loads,open(R+'/n31_k16_d4.jsonl')) if (r['arm'],r['s']) in (('C1',1),('D1',25))]
bad=0
for r in recs:
    sy=system(r['n'],r['k'],r['s']); x=call5('closure',2*r['k'],4,sy[r['arm']])
    ok=[x['dim'],x['low'],x['one']]==[r['dim4'],r['low4'],r['one']]; bad+=not ok
    print(json.dumps(dict(n=r['n'],s=r['s'],arm=r['arm'],ok=ok,mine=[x['dim'],x['low'],x['one']],n1=[r['dim4'],r['low4'],r['one']],cpu=x['cpu'])),flush=True)
print(json.dumps(dict(total=len(recs),mismatches=bad)))
