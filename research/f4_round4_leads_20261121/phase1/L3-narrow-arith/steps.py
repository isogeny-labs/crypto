import re,collections,sys
rx=re.compile(r"f4 step (\d+): degree (\d+), (\d+) pairs, (\d+) rows × (\d+) cols, (\d+) pivots, basis (\d+)")
agg=collections.defaultdict(lambda:[0,0,0,0,0,0])
fl=0;tot=0;cells=0;flcells=0
for l in open(sys.argv[1]):
    m=rx.search(l)
    if not m: continue
    k,d,pa,r,c,pv,b=map(int,m.groups())
    a=agg[k];a[0]+=1;a[1]+=r;a[2]+=c;a[3]+=pv;a[4]+=pa;a[5]+=r*c
    tot+=1;cells+=r*c
    if c>=256: fl+=1;flcells+=r*c
for k in sorted(agg):
    n,r,c,pv,pa,rc=agg[k]
    print(f"step {k}: n={n} mean pairs {pa/n:.1f} rows {r/n:.1f} cols {c/n:.1f} pivots {pv/n:.1f} rowsxcols share {rc/cells:.3f}")
print("steps",tot,"with cols>=256 (FL eligible):",fl, "cell share",flcells/cells)
