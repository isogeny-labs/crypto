import json, glob, os, numpy as np
R='../raw'; out={}
k1={(r['n'],r['j']):r for r in json.load(open(f'{R}/calib_K1.json'))}
def estar(n,j):
    r=k1.get((n,j)); return None if r is None else r['E_star']
A=[]
for f in sorted(glob.glob(f'{R}/armA_*.json')):
    r=json.load(open(f)); nm=max(x['E_max'] for x in r['null1sym'])
    A.append({'n':r['curve']['n'],'kind':r['curve']['kind'],'a':r['curve']['a'],'h':r['curve']['h'],'E_max_real':r['E_max'],'null1sym_maxes':[x['E_max'] for x in r['null1sym']],
      'ratio_real_over_nullmax':r['E_max']/nm,'argmax':r['argmax'],'n_degenerate':len(r['degenerate_masks']),'degenerate':r['degenerate_masks'],'trace_mask':r['trace_mask'],
      'null_n_degenerate':[x['n_degenerate'] for x in r['null1sym']],'frobenius_max_absdiff':r.get('frobenius_max_absdiff'),'secs':r['secs']})
out['armA']=A
B=[]
for f in sorted(glob.glob(f'{R}/armB_*.json')):
    for r in json.load(open(f)):
        n=r['curve']['n']; j=r['j']; er=r['real']['E']; en=r['null1sym']['E']; nm=max(en,r['prf_max']); es=estar(n,j)
        B.append({'n':n,'kind':r['curve']['kind'],'a':r['curve']['a'],'h':r['curve']['h'],'mode':r['mode'],'L':r['L'],'j':j,'M_eff':r['real'].get('M_eff'),
          'E_real':er,'E_null1sym':en,'E_prf_max':r['prf_max'],'ratio_real_null1sym':er/en if en else None,'excess_over_nullmax':er-nm,'E_star_K1':es,
          'exceeds_floor':(es is not None and er-nm>es),'secs':r['secs']})
out['armB']=B
for k in ['K2','K2_amended','K3_amended','K3b']:
    p=f'{R}/calib_{k}.json'
    if os.path.exists(p): out['calib_'+k]=json.load(open(p))
out['calib_K1_summary']=[{'n':r['n'],'j':r['j'],'null_max':r['null_max'],'E_star':r['E_star'],'min_rho_detected':min([p['rho'] for p in r['planted'] if p['detected']],default=None)} for r in k1.values()]
P=[x for x in B if x['mode']=='P']
rat=[x['ratio_real_null1sym'] for x in P if x['ratio_real_null1sym']]
out['summary']={'armB_P_cells':len(P),'cells_exceeding_floor':[ (x['n'],x['kind'],x['j']) for x in P if x['exceeds_floor']],
 'median_ratio_real_null1sym':float(np.median(rat)) if rat else None,'ratio_range':[float(min(rat)),float(max(rat))] if rat else None,
 'armA_ratio_range':[min(a['ratio_real_over_nullmax'] for a in A),max(a['ratio_real_over_nullmax'] for a in A)] if A else None}
json.dump(out,open('../RESULTS.json','w'),indent=1)
for a in A: print('A',a['n'],a['kind'],a['a'],'E %.4f'%a['E_max_real'],'null',[round(v,4) for v in a['null1sym_maxes']],'deg',a['n_degenerate'],a['degenerate'][:3],'tr',a['trace_mask'],'frob',a['frobenius_max_absdiff'])
for b in B: print('B',b['n'],b['kind'],b['a'],b['mode'],'j',b['j'],'Meff',b['M_eff'],'Er %.4f En %.4f Eprf %.4f E* %s'%(b['E_real'],b['E_null1sym'],b['E_prf_max'],b['E_star_K1'] and round(b['E_star_K1'],3)),'X' if b['exceeds_floor'] else '')
print(out['summary'])
