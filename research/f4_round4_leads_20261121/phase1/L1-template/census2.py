import sys,re,collections
sysl=[];cur=None
for line in open(sys.argv[1]):
    if line.startswith('f4:') and 'polys in' in line:
        if cur is not None: sysl.append(tuple(cur))
        cur=[]
    m=re.match(r'f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)',line)
    if m and cur is not None: cur.append(tuple(int(x) for x in m.groups()[1:]))
if cur is not None: sysl.append(tuple(cur))
n=len(sysl)
def cov(c):
    tot=0;out={}
    for i,(k,v) in enumerate(c.most_common()):
        tot+=v
        for q in (0.5,0.8,0.9,0.95,0.99):
            if q not in out and tot>=q*n: out[q]=i+1
    return out
# basis-growth key: degrees + basis size after each step (new poly count) = necessary for same new_lms
c=collections.Counter(tuple((x[0],x[5]) for x in s) for s in sysl)
print("basis-growth key distinct",len(c),"top",[v for k,v in c.most_common(6)],"cover",cov(c))
for k,v in c.most_common(4): print(v,k)
cd=collections.Counter(sysl)
print("dims key distinct",len(cd),"cover",cov(cd))
# work proxy: sum rows*cols, sum rows
W=sum(x[2]*x[3] for s in sysl for x in s); R=sum(x[2] for s in sysl for x in s); S=sum(len(s) for s in sysl)
print("systems",n,"steps",S,"sum rows",R,"sum rows*cols",W)
# restart detection: degree decreasing within a system
rs=sum(1 for s in sysl if any(s[i+1][0]<s[i][0] for i in range(len(s)-1)))
print("systems with a degree restart (replay then full run)",rs)
