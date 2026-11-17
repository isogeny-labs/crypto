import json,glob
S={}
for L in (4096,6144):
    rs=[json.load(open(f)) for f in sorted(glob.glob(f'raw/fplll_L{L}_s*.json'))]
    d={'seeds':[r['seed'] for r in rs],'xstar_bits_det':rs[0]['xstar_bits']}
    for a in ('below','null','above'):
        d[a]={'X':rs[0][a]['X'],'v1_vanish':sum(r[a]['vecs'][0]['vanish'] for r in rs),'v2_vanish':sum(r[a]['vecs'][1]['vanish'] for r in rs),
              'n':len(rs),'v1_log2norm':[r[a]['vecs'][0]['log2norm'] for r in rs]}
    d['H_build_consistent_all']=all(r['below']['info']['consistent'] and r['below']['info']['support']==105 and r['below']['info']['lambda_evenflip']==1 for r in rs)
    if L==4096:
        d['armX']={'log_p_abs_invariants':[r['armX']['log_p_abs_invariants'] for r in rs],'v1_vanish':sum(r['armX']['vecs'][0]['vanish'] for r in rs),'n':len(rs)}
        d['planted_Ys']=[r['below']['Ys'] for r in rs]
    S[L]=d
far=[json.load(open(f)) for f in sorted(glob.glob('raw/far_L6144_s*.json'))]
S['far_above_6144_X128']={'n':len(far),'v1_vanish':sum(r['vecs'][0]['vanish'] for r in far),'v1_log2norm':[r['vecs'][0]['log2norm'] for r in far]}
print(json.dumps(S,indent=1)); json.dump(S,open('raw/fplll_summary.json','w'),indent=1)
