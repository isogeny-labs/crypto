import json, collections, numpy as np, sys
L1={}
for s in (1,2):
    d=json.load(open(f'raw/layer1_S3_b128_s{s}.json'))
    for r in d['rows']:
        A=r['tau=None']['delta_star']; Bs={k:v['delta_star'] for k,v in r.items() if k.startswith('tau=') and k!='tau=None'}
        best=max(Bs,key=Bs.get)
        L1[f"S3_s{s}_l{r['l']}"]={"A":A,"B_tau0":Bs['tau=0.0'],"bestBprime":Bs[best],"bestBprime_tau":best,"best_minus_A":Bs[best]-A,
          "B_tau0_logvol_excess_over_diag_logp":r['tau=0.0'].get('logvol_minus_logdiag')}
d=json.load(open('raw/layer1_S4_b128_s1.json')); L1["S4_s1_l1"]={k:v['delta_star'] for k,v in d['rows'][0].items() if k.startswith('tau=')}
ca=json.load(open('raw/layer1_closedA.json'))
l2=json.load(open('raw/layer2.json')); agg=collections.defaultdict(list)
for r in l2['records']: agg[(r.get('control','planted'),r['l'],r['delta'],r['arm'])].append(r)
L2={f"{k[0]}|l{k[1]}|d{k[2]}|{k[3]}":{"HG":sum(x['hg'] for x in v),"heur":sum(x['heur'] for x in v),"unplanted_fp":sum(x['unplanted_recovery'] for x in v),"n":len(v),"r":v[0]['r'],
  "margin2_logp_mean":(float(np.mean([x['margin_logp'] for x in v if x['margin_logp'] is not None])) if any(x['margin_logp'] is not None for x in v) else None)} for k,v in sorted(agg.items())}
def maxd(arm,l,key):
    ds=[k[2] for k,v in agg.items() if k[0]=='planted' and k[1]==l and k[3]==arm and k[2]<0.4 and sum(x[key] for x in v)==len(v)]
    return max(ds) if ds else 0.0
summ={f"l{l}_{arm}":{"HG_all4_max_delta":maxd(arm,l,'hg'),"heur_all4_max_delta":maxd(arm,l,'heur')} for l in (2,3) for arm in "ABC"}
res={"idea_id":"MP-lattices-2","date":"2026-10-31","versions":{"python":sys.version.split()[0],"cypari2":"2.2.2","pari":"2.17.3","numpy":np.__version__},
 "params":{"layer1_p":"nextprime(2^128)","layer1_seeds":[1,2],"layer2_p":l2['p'],"layer2_seeds_planted":[100,101,102,103],"control_seeds":[200,201,202,203]},
 "runtime_s":{"layer2":l2['runtime_s'],"total_cpu_min_approx":8},
 "layer1_S3":L1,"layer1_closed_form_A":ca,"layer2_summary_max_delta_4of4":summ,"layer2_cells":L2,
 "decision":{"rule":"closed if B/B'/C <= A+0.02 at every l and fits <=1/6, <=1/24","B_minus_A_layer1_max":max(v['best_minus_A'] for k,v in L1.items() if k.startswith('S3')),
   "A_fit_delta_inf_S3":ca['S3_fit_delta_inf_c'][0],"A_fit_delta_inf_S4":ca['S4_fit_delta_inf_c'][0],"outcome":"supports"}}
json.dump(res,open('RESULTS.json','w'),indent=1)
print(json.dumps(summ)); print({k:(round(v['A'],4),round(v['B_tau0'],4),round(v['bestBprime'],4),v['bestBprime_tau']) for k,v in L1.items() if 'A' in v}); print(res['decision'])
