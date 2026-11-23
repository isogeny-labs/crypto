import json,glob,math,collections,sys
js={}
for l in open('jstar.jsonl'):
    d=json.loads(l); js[(d['m'],d['fam'],d['B'])]=d['ffd']
rows=[]; seen=set()
for f in sorted(glob.glob('runs/*.jsonl')+glob.glob('podout/*.jsonl')):
    if 'census' in f: continue
    for l in open(f):
        try: d=json.loads(l)
        except: continue
        if 'fail' in d: rows.append(dict(fail=d['fail'],src=f)); continue
        d['src']=f.split('/')[-1]
        key=(d['fam'],d['m'],d['k'],d['B'],d['p'],d['r'],d['target'])
        if key in seen: continue
        seen.add(key); rows.append(d)
def pred(d):
    fam=d['fam']; m=d['m']; B=d['B']
    if fam in('lin','linrand'): return math.ceil(d['k']/2)+1
    if fam=='linb3': return d['k']+1
    if fam=='sym': return math.ceil(d['k']/2)+1
    e=2**(m-1)
    if fam.startswith('uni'): top=B-1
    else:
        rad={'bool':[2]*d['k'],'supp':[2]*d['k'],'b3':[3]*d['k'],'b4':[4]*d['k'],'mix':[2 if i%2==0 else 3 for i in range(d['k'])]}[fam]
        top=sum(b-1 for b in rad)
    if top < e: return m*top+1   # degenerate: l^e = 0 in gr, saturation (whole ring)
    if fam.startswith('uni'): return B+(m-1)*e
    key=(m,'bool' if fam=='supp' else fam,B)
    return js.get(key)
def E(m,th): return 2/(m+1-th) if th<m-1 else float('inf')
g=collections.OrderedDict(); fails=[]
for d in rows:
    if 'fail' in d: fails.append(d); continue
    key=(d['fam'],d['m'],d['B'],d['target'])
    g.setdefault(key,[]).append(d)
print('| fam | m | k | B | target | draws | D (measured) | certified | ffd bound | D==bound | cols(D) | rank(D) | log_B cols | E_m w=1 | E_m w=2 | E_m-0.5 (w=1) |')
print('|'+'---|'*16)
for key in sorted(g, key=lambda k:(k[1],k[0],k[3],k[2])):
    ds=g[key]; fam,m,B,t=key
    Ds=[d['D'] for d in ds]; cert=sum(d['certified'] for d in ds)
    pr=pred(ds[0])
    if fam=='supp': pr='null (k+3 cited)'; eq='n/a'
    else:
        eq=all((isinstance(x,int) and x==pr) or (isinstance(x,str) and int(x[1:])<pr) for x in Ds) if pr is not None else None
    cols=max(d['cols_D'] for d in ds); rk=max(d['rank_D'] for d in ds); cert=f"{cert} (+{sum(1 for x in Ds if isinstance(x,str))} censored at cmax, bound only)" if any(isinstance(x,str) for x in Ds) else cert
    th=math.log(cols)/math.log(B) if B>1 and cols>0 else float('nan')
    e1=E(m,th) if m>=2 else float('nan'); e2=E(m,2*th) if m>=2 else float('nan')
    f=lambda x: '∞(≥1)' if x==float('inf') else ('-' if x!=x else f'{x:.2f}')
    print(f"| {fam} | {m} | {'-' if fam.startswith('uni') else ds[0]['k']} | {B} | {t} | {len(ds)} | {sorted(set(map(str,Ds)))} | {cert}/{len(ds)} | {pr} | {eq} | {cols} | {rk} | {th:.2f} | {f(e1)} | {f(e2)} | {f(e1-0.5) if e1==e1 and e1!=float('inf') else f(e1)} |")

print()
for d in fails: print(f"- censored/failed run (no record written): `{d['fail']}` rc={d.get('rc','124')} (124 = 1200 s timeout) [{d['src']}]")
