import sys,re,collections
sysl=[];cur=None
for line in open(sys.argv[1]):
    if line.startswith('f4:') and 'polys in' in line:
        if cur is not None: sysl.append(tuple(cur))
        cur=[]
    m=re.match(r'f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)',line)
    if m and cur is not None: cur.append(tuple(int(x) for x in m.groups()[1:]))
if cur is not None: sysl.append(tuple(cur))
def report(name,keyf):
    c=collections.Counter(keyf(s) for s in sysl)
    n=len(sysl);tot=0;k90=k99=None
    for i,(k,v) in enumerate(c.most_common()):
        tot+=v
        if k90 is None and tot>=0.9*n:k90=i+1
        if k99 is None and tot>=0.99*n:k99=i+1
    top=[v for k,v in c.most_common(5)]
    print(f"{name}: systems {n} distinct {len(c)} top5 {top} top1 share {top[0]/n:.4f} shapes for 90% {k90} 99% {k99} singletons {sum(1 for v in c.values() if v==1)}")
    return c
report("full(deg,pairs,rows,cols,piv,basis)",lambda s:s)
report("rows/cols/pivots",lambda s:tuple((x[0],x[2],x[3],x[4]) for x in s))
report("pivots/basis only",lambda s:tuple((x[0],x[4],x[5]) for x in s))
report("nsteps+degrees",lambda s:tuple(x[0] for x in s))
c=report("full",lambda s:s)
for k,v in c.most_common(3): print(v,k)
