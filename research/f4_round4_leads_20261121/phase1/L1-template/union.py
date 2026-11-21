import sys,re,collections,statistics as stt
sysl=[];cur=None
for line in open(sys.argv[1]):
    if line.startswith('f4:') and 'polys in' in line:
        if cur is not None: sysl.append(tuple(cur))
        cur=[]
    m=re.match(r'f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)',line)
    if m and cur is not None: cur.append(tuple(int(x) for x in m.groups()[1:]))
if cur is not None: sysl.append(tuple(cur))
par=int(sys.argv[2]); S=sysl[par::int(sys.argv[3])]
g=collections.defaultdict(list)
for s in S: g[tuple((x[0],x[5]) for x in s)].append(s)
key,grp=max(g.items(),key=lambda kv:len(kv[1]))
L=len(key)
mean_rc=sum(stt.mean(s[i][2]*s[i][3] for s in grp) for i in range(L))
max_rc=sum(max(s[i][2] for s in grp)*max(s[i][3] for s in grp) for i in range(L))
p95=lambda v: sorted(v)[int(.95*len(v))]
p95_rc=sum(p95([s[i][2] for s in grp])*p95([s[i][3] for s in grp]) for i in range(L))
mean_r=sum(stt.mean(s[i][2] for s in grp) for i in range(L)); max_r=sum(max(s[i][2] for s in grp) for i in range(L))
print("group size",len(grp),"steps",L,"sum mean rows",round(mean_r),"sum max rows",max_r,"ratio %.3f"%(max_r/mean_r),"| rows*cols mean",round(mean_rc),"max",max_rc,"ratio %.3f"%(max_rc/mean_rc),"p95 ratio %.3f"%(p95_rc/mean_rc))
