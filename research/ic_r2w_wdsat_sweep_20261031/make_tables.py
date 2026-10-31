import json,sys
d=json.load(open(sys.argv[1]))['cells']
ls=list(range(3,12))
def f(v): return '%.2e'%v if v else 'x'
for title,key in (('whole run, median Ir/target','med_whole'),('solver-only, median Ir/target','med_lower'),('relation systems (WDSat calls) per target, median','med_calls')):
    print('\n'+title); print('| cell | '+' | '.join(f'l={l}' for l in ls)+' |'); print('|:--|'+'--:|'*len(ls))
    for c,v in d.items():
        row=[]
        for l in ls:
            a=v['arms'].get(f'wd-l{l}')
            if not a: row.append('-'); continue
            if a['n']<8: row.append('failed 0/8' if key!='med_calls' else '(%g)'%a['med_calls'])
            else: row.append(('%g'%a[key]) if key=='med_calls' else f(a[key]))
        print(f'| {c} | '+' | '.join(row)+' |')
