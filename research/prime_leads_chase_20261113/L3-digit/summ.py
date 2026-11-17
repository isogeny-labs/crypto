import sys,json,glob
for f in sys.argv[1:]:
  for l in open(f):
    try: d=json.loads(l)
    except Exception: print('BAD',l[:200]); continue
    if 'fail' in d: print('FAIL',d); continue
    print(f.split('/')[-1][:-6],d['fam'],'m',d['m'],'k',d['k'],'B',d['B'],'p',d['p'],d['target'][:2],'nsol',d['nsol'],'fdeg',d['fdeg'],'D',d['D'],'cert' if d['certified'] else 'NC','cols',d['cols_D'],'rk',d['rank_D'],'rkm1',d['rank_Dm1'],'t',d['secs'],d['V2'])
