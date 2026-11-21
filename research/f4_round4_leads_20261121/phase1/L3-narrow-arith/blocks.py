# group per-address costs (output of cgaddr with large N) into runs of equal count = basic-block-ish loops
import sys,re
cur=None;rows=[]
for l in open(sys.argv[1]):
    if l.startswith('##'): print(l.strip()[:120]); continue
    a,v=l.split(); rows.append((int(a,16),int(v.replace(',',''))))
rows.sort()
grp=[];
for a,v in rows:
    if grp and v==grp[-1][2] and a-grp[-1][1]<16: grp[-1][1]=a; grp[-1][3]+=1
    else: grp.append([a,a,v,1])
grp.sort(key=lambda g:-g[2]*g[3])
for s,e,v,n in grp[:int(sys.argv[2])]:
    print(f"{s:#x}-{e:#x} count {v:,} x {n} instr = {v*n:,}")
