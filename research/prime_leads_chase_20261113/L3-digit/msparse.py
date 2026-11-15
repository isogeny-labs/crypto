import re,glob,json,sys,os
root=sys.argv[1]
res=[]
for jf in sorted(glob.glob(root+'/out/ms_*.jsonl')):
    name=os.path.basename(jf)[3:-6]
    for i,l in enumerate(open(jf)):
        d=json.loads(l)
        log=f"{root}/ms/{name}_{i}.log"
        if not os.path.exists(log): res.append((name,i,d['fam'],d['m'],d['k'],d['B'],d['D'],'pending')); continue
        txt=open(log).read()
        degs=[int(m.group(1)) for m in re.finditer(r'^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+',txt,re.M)]
        mats=[(int(a),int(b)) for a,b in re.findall(r'^\s*\d+\s+\d+\s+\d+\s+(\d+) x (\d+)',txt,re.M)]
        unsat='No solution' in txt or 'single element' in txt
        tf=f"{root}/ms/{name}_{i}.time"; t=open(tf).read() if os.path.exists(tf) else ''
        rc=re.findall(r'rc=(\d+)',t); el=re.findall(r'overall\(elapsed\)\s+(\S+ sec)',txt); mem=re.findall(r'Maximum resident set size \(kbytes\): (\d+)',t)
        res.append((name,i,d['fam'],d['m'],d['k'],d['B'],d['D'],max(degs) if degs else None,unsat,max(mats,key=lambda x:x[0]*x[1]) if mats else None,rc,el,mem, d['nsol']))
print("| system | fam | m | k | B | PC degree D (pcdeg, same system; >c = dump only, cell value in TABLES_main) | msolve max F4 degree | msolve verdict | grid nsol | largest F4 matrix | log_B(matrix cols) | wall | rc |")
print("|"+"---|"*13)
for r in res:
    if r[-1]=='pending': print(f"| {r[0]}_{r[1]} | {r[2]} | {r[3]} | {r[4]} | {r[5]} | {r[6]} | pending | | | | | |"); continue
    print(f"| {r[0]}_{r[1]} | {r[2]} | {r[3]} | {r[4]} | {r[5]} | {r[6]} | {r[7]} | {'no solution' if r[8] else 'solutions/other'} | {r[13]} | {(str(r[9][0])+'x'+str(r[9][1])) if r[9] else '-'} | {(f"{__import__('math').log(r[9][1])/__import__('math').log(r[5]):.2f}") if r[9] else '-'} | {r[11][0] if r[11] else ''} | {r[10][0] if r[10] else ''} |")
