import json,sys,glob,collections
# M-eq at w=0 with N4 fixed constants r_inv=15, r_lmul=1.1 (summary_xeonconst convention); all attempts of an arm charged.
PH=["fb","relations","matrix","linear_algebra","log_recovery","descent","verify"]
def meq(d,ri=15.0,rl=1.1):
    return sum(d.get(p+".mul",0)+ri*d.get(p+".inv",0)+rl*d.get(p+".lmul",0) for p in PH)
def unit(files):
    out=[]
    for f in files:
        recs=[json.loads(l) for l in open(f) if l.startswith('{"rec":"ic"')]
        for arm in ("D","DL1","DL"):
            r=[x for x in recs if x["arm"]==arm]
            if not r: continue
            out.append(dict(file=f.split('/')[-1],q=r[0]["q"],arm=arm,ell=r[0]["ell"],cof=r[0].get("cof"),attempts=len(r),ok=r[-1]["ok"],eq=r[-1].get("k_equals_planted"),
              meq=sum(meq(x) for x in r),hashcmp=sum(sum(x.get(p+".hash",0)+x.get(p+".cmp",0) for p in PH) for x in r),
              wall=sum(sum(x.get(p+".wall",0) for p in PH) for x in r)))
    return out
if __name__=="__main__":
    for u in unit(sys.argv[1:]): print(json.dumps(u))
