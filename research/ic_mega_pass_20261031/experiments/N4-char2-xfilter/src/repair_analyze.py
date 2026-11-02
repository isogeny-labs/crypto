import json, glob, numpy as np
R=json.load(open('../RESULTS.json')); C=[]
for f in sorted(glob.glob('../raw/repair_samemask_*.json')): C+=json.load(open(f))
nk=[c for c in C if c['kind']!='koblitz']; kb=[c for c in C if c['kind']=='koblitz']
zs=[c['z'] for c in nk]; mr=[c['E_marg']/c['E_real'] for c in C]
A=R['armA']
R['repair']={'amendment':4,'retired_rule':'REAL<=max(nulls)+E* (vacuous: E*>E_real in all 29 cells with E*; 24/53 cells had no E*)',
 'samemask_cells':C,'n_cells':len(C),'n_nonkoblitz':len(nk),'n_koblitz_excluded':len(kb),
 'nonkoblitz_z_range':[min(zs),max(zs)],'nonkoblitz_cells_z_gt_3p5':[(c['n'],c['kind'],c['a'],c['j'],c['z']) for c in nk if c['z']>3.5],
 'koblitz_z':[(c['n'],c['a'],c['j'],c['z']) for c in kb],'Emarg_over_Ereal_range':[min(mr),max(mr)],'Emarg_over_Ereal_median':float(np.median(mr)),
 'recheck_maxdiff':max(abs(c['E_real']-c['E_real_recheck']) for c in C),
 'armA_nonkoblitz_ratio_range':[min(a['ratio_real_over_nullmax'] for a in A if a['kind']!='koblitz'),max(a['ratio_real_over_nullmax'] for a in A if a['kind']!='koblitz')],
 'verdict':'no growth seen' if not [c for c in nk if c['z']>3.5] else 'flagged cells present'}
R['summary']['superseded_note']='cells_exceeding_floor uses the retired vacuous E* rule; see repair'
json.dump(R,open('../RESULTS.json','w'),indent=1)
for c in C: print(c['n'],c['kind'],c['a'],c['h'],'j',c['j'],'Er %.5f Em %.5f mu %.5f sd %.2e z %+.2f'%(c['E_real'],c['E_marg'],c['null_mean'],c['null_sd'],c['z']))
print({k:v for k,v in R['repair'].items() if k!='samemask_cells'})
