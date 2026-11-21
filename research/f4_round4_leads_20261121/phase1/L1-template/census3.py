import sys,re,collections
sysl=[];cur=None
for line in open(sys.argv[1]):
    if line.startswith('f4:') and 'polys in' in line:
        if cur is not None: sysl.append(tuple(cur))
        cur=[]
    m=re.match(r'f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)',line)
    if m and cur is not None: cur.append(tuple(int(x) for x in m.groups()[1:]))
if cur is not None: sysl.append(tuple(cur))
for par,name in ((0,"even(1st=lead check)"),(1,"odd(main)")):
    S=sysl[par::2]; n=len(S)
    st=sum(len(s) for s in S); R=sum(x[2] for s in S for x in s); W=sum(x[2]*x[3] for s in S for x in s); P=sum(x[4] for s in S for x in s)
    c=collections.Counter(S); cg=collections.Counter(tuple((x[0],x[5]) for x in s) for s in S)
    tot=0;k=[]
    for i,(kk,v) in enumerate(c.most_common()):
        tot+=v
        k.append(tot/n)
    def need(q): return next(i+1 for i,x in enumerate(k) if x>=q)
    print(name,"runs",n,"steps/run %.1f"%(st/n),"rows/run %.0f"%(R/n),"rows*cols/run %.0f"%(W/n),"pivots/run %.0f"%(P/n),"| dims-shapes",len(c),"top share %.3f"%k[0],"90%",need(.9),"99%",need(.99),"| growth-keys",len(cg),"top",cg.most_common(1)[0][1])
