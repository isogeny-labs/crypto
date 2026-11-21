import re,sys,collections
runs=[];cur=None
for line in open(sys.argv[1]):
    if line.startswith('f4: ') and 'polys in' in line:
        cur={'steps':[],'pend':[]};runs.append(cur);continue
    m=re.match(r'f4: (\d+) pairs pending, basis (\d+)',line)
    if m and cur is not None: cur['pend'].append((int(m[1]),int(m[2])));continue
    m=re.match(r'f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)',line)
    if m and cur is not None: cur['steps'].append(tuple(map(int,m.groups())))
    if line.startswith('f4 interreduce') and cur is not None: cur['done']=True
agg=collections.defaultdict(lambda:[0,0,0,0,0,0])  # by step: runs, rows, pivots, zero, new, cols
tot=[0,0,0,0,0]; lastzero=0; nrun=0; unknown_last=0; rowcol=[0,0]; zerocol=0
for r in runs:
    if not r['steps']: continue
    nrun+=1
    st=r['steps']
    for i,s in enumerate(st):
        k,d,pairs,R,C,P,B=s
        # basis after add: pend line at index i+1 (pend[0] is before step 1)
        if i+1 < len(r['pend']): N=r['pend'][i+1][1]-B
        else: N=None
        Z=R-P
        a=agg[k]; a[0]+=1;a[1]+=R;a[2]+=P;a[3]+=Z;a[5]+=C
        if N is None: unknown_last+=1; N=0
        a[4]+=N
        tot[0]+=R;tot[1]+=P;tot[2]+=Z;tot[3]+=N
        rowcol[0]+=R*C; zerocol+=Z*C
print('runs',nrun,'last-step N unknown (set 0):',unknown_last)
print('step runs avgR avgP avgZero avgNew avgReduced zero/reduced zero/rows')
for k in sorted(agg):
    n,R,P,Z,N,C=agg[k]; red=R-P+N
    print(k,n,round(R/n,1),round(P/n,1),round(Z/n,1),round(N/n,1),round(red/n,1),round(Z/max(red,1),3),round(Z/R,3))
R,P,Z,N,_=tot; red=R-P+N
print('TOTAL rows',R,'pivots',P,'zero',Z,'new',N,'reducers',P-N,'reduced(nonreducer) rows',red)
print('zero/rows',round(Z/R,4),'zero/reduced',round(Z/red,4),'zero R*C share',round(zerocol/rowcol[0],4))
# weighted by cols^2 proxy for per-row reduction work (reducers hit ~ cols, length ~ cols)
num=den=num1=den1=0
for r in runs:
    st=r['steps']
    for i,s in enumerate(st):
        k,d,pairs,R,C,P,B=s
        N=r['pend'][i+1][1]-B if i+1<len(r['pend']) else 0
        Z=R-P; red=R-P+N
        num+=Z*C*C; den+=red*C*C; num1+=Z*C; den1+=red*C
print('zero share of reduced-row work, weight C:',round(num1/den1,4),' weight C^2:',round(num/den,4))
