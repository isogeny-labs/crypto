import json,os,sys
E=sys.argv[1]
for c in sys.argv[2:]:
    print('CELL',c)
    for sub in ('sessions/'+c,'audits','tables','comparisons/'+c):
        base=os.path.join(E,sub)
        for root,dirs,files in sorted(os.walk(base)):
            for f in sorted(files):
                if sub in ('audits','tables') and not f.startswith(c+'.'): continue
                fp=os.path.join(root,f)
                print('FILE',fp,os.path.getsize(fp))
    for line in open(os.path.join(E,'audits',c+'.verify.stderr.txt')).read().splitlines():
        if line.startswith('audit '): print('VERDICT',c,line)
    j=json.load(open(os.path.join(E,'audits',c+'.verify.txt')))
    print('AUDIT-JSON',c,'ok',j.get('ok'),'session_status',j.get('session_status'),'records',j.get('records'),'verified_records',j.get('verified_records'),'problems',len(j.get('problems') or []))
    print('AA-TEXT',c)
    print(open(os.path.join(E,'comparisons',c,'ic-b64-aa.txt')).read())
    a=json.load(open(os.path.join(E,'sessions',c,'comparisons','ic-b64__ic-b64-aa.json')))
    print('AA-JSON',c,'ops.ratio_b_over_a',a['ops']['ratio_b_over_a'],'ops.ci95',a['ops'].get('ci95'))
