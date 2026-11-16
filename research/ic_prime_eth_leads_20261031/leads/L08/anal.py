import json, sys, math
fn=sys.argv[1]
rows=json.load(open(fn))
LMIN=int(sys.argv[2]) if len(sys.argv)>2 else 1
if fn.startswith("scanC"):   # (d, [(e,deg,J,k,L),...]) -> pick min edges subject to deg<102 d^.75 else min deg
    R=[]
    for d,c in rows:
        c=[x for x in c if x[4]>=LMIN] or c
        ok=[x for x in c if x[1]<102*d**.75]
        b=min(ok) if ok else min(c,key=lambda x:x[1])
        R.append((d,b[0],b[1],b[2],b[3],b[4]))
    rows=R
H=lambda d:(80*d+228*math.sqrt(d), 102*d**.75)
Bo=lambda d:(1100*(d+1),136*(d+1)**.75)
def first(pred):
    for r in rows:
        if pred(r): return r[0]
    return None
print("n_d",len(rows),"d range",rows[0][0],rows[-1][0])
print("first d: edges>=hand bound 80d+228sqrt(d):",first(lambda r:r[1]>=H(r[0])[0]))
print("first d: deg>=hand bound 102 d^.75:",first(lambda r:r[2]>=H(r[0])[1]))
print("first d: edges>=Bochum 1100(d+1):",first(lambda r:r[1]>=Bo(r[0])[0]))
print("first d: deg>=Bochum 136(d+1)^.75:",first(lambda r:r[2]>=Bo(r[0])[1]))
v=[r for r in rows if r[1]>=H(r[0])[0]]
print("count edge-violations of hand bound:",len(v),"of",len(rows))
mx=max(rows,key=lambda r:r[1]/r[0]); print("max edges/d:",mx[1]/mx[0],"at d",mx[0],mx[1:])
mxd=max(rows,key=lambda r:r[2]/r[0]**.75); print("max deg/d^.75:",mxd[2]/mxd[0]**.75,"at d",mxd[0])
# minimal constants: A with edges <= A d + B sqrt(d)? report sup of edges/d over d>=100 etc
for lo in (100,1000):
    S=[r for r in rows if r[0]>=lo]; print("d>=%d sup edges/d = %.1f ; sup edges/(d+1)=%.1f"%(lo,max(r[1]/r[0] for r in S),max(r[1]/(r[0]+1) for r in S)))
for d in (19,131,1000,5000):
    r=[x for x in rows if x[0]==d]
    if r: print("d",d,r[0])
