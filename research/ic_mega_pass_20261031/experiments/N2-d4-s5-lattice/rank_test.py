"""Hypothesis for anomalously short LLL vectors: the coefficient vector of H (105 entries) is the image of a fixed
integer matrix applied to monomials in (A,c,xR), with rank r<105. Test: F_p-rank of H vectors over random (A,c,xR);
also rank with A,c fixed and xR varying (expect <=9: degree 8 in xR)."""
import sys,random,json; sys.path.insert(0,'.')
from build_h import *
p=int(pari.nextprime(2**61)); rng=random.Random(11); out={}
def rank(rows): return int(pari.matrank(pari.matrix(len(rows),105,[pari.Mod(x,p) for r in rows for x in r])))
rows=[]
for _ in range(140):
    A,B,c=curve(p,rng); coef,info=interpolate(rng.randrange(p),A,B,c,p,rng=rng); assert info['consistent']; rows.append(coef)
out['rank_random_A_c_xR_140']=rank(rows)
A,B,c=curve(p,rng); rows=[interpolate(rng.randrange(p),A,B,c,p,rng=rng)[0] for _ in range(20)]
out['rank_fixed_curve_xR_varies_20']=rank(rows)
print(json.dumps(out)); json.dump(out,open('raw/rank_test.json','w'))
