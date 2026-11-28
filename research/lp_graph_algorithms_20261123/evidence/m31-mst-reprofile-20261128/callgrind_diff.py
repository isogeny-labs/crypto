import re,subprocess,sys,collections
def load(f,incl):
    out=subprocess.run(['callgrind_annotate','--auto=no','--threshold=100',f'--inclusive={"yes" if incl else "no"}',f],capture_output=True,text=True).stdout
    d=collections.Counter()
    for line in out.splitlines():
        if 'PROGRAM TOTALS' in line:
            d['__TOTAL__']+=int(line.split()[0].replace(',',''));continue
        m=re.match(r'\s*([\d,]+)\s+\([^)]*\)\s+(.*)$',line)
        if not m or line.lstrip().startswith('=>') or 'events annotated' in line: continue
        n=int(m.group(1).replace(',',''));name=m.group(2)
        name=re.sub(r'\s*\[[^\]]*\]$','',name)
        name=name.split(':',1)[1] if ':' in name and name.startswith('/') else name
        name=re.sub(r'::\{closure#\d+\}','::{closure}',name)
        d[name]+=n
    return d
arm=sys.argv[1]; incl=sys.argv[2]=='incl'
a=load(f'{sys.argv[3]}/{arm}-1.out',incl); b=load(f'{sys.argv[3]}/{arm}-11.out',incl)
tot=None
rows=[(k,(b[k]-a[k])/10) for k in set(a)|set(b)]
rows.sort(key=lambda x:-x[1])
total=dict(rows)['__TOTAL__']; rows=[r for r in rows if r[0]!='__TOTAL__']
print(f'{arm} {"inclusive" if incl else "exclusive"} per MST call; total {total/1e6:.3f} M')
for k,v in rows[:int(sys.argv[4])]:
    if v<=0: break
    short=re.sub(r'crypto_lib::cryptanalysis::large_prime_filter::','',k)
    short=re.sub(r'<alloc::|alloc::|core::|std::','',short)
    print(f'{v/1e6:8.3f} M {100*v/total:5.1f}%  {short[:150]}')
