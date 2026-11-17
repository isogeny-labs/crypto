import json,sys
base="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research/gaudry_k4_char2_crossover_20261015/sessions/"
for s in ['d-ladder','e-m15','e-m17-rho','f-lp']:
    print("=== session",s)
    for line in open(base+s+'/records.jsonl'):
        r=json.loads(line)
        m=r['method']; w=r['workload']; c=r['cost']; o=r['outcome']
        tot=c['total_gae']
        ph=[(p['name'],p.get('gae'),p.get('scalar_mults'),p.get('adds'),p.get('doubles'),p.get('native')) for p in r.get('phases',[])]
        shares={p[0]:round(100*(p[1] or 0)/tot,3) if tot else None for p in ph}
        cs=w['curve_spec']
        print(f"arm={r['arm']} role={r['role']} method={m['id']} entry={m.get('entry')} warm={r['warmup']} seq={r['seq']} status={o['status']} m={cs.get('m')} tgt={r['workload']['target_index']}")
        print(f"  lower_bound={c.get('lower_bound')} unpriced={c.get('unpriced')} total_gae={tot} s={c.get('s')} det={c.get('deterministic')} nondet={c.get('nondeterminism')}")
        print(f"  phase_gae_share%={shares}")
        print(f"  phases={[(p[0],p[1]) for p in ph]} native_by_phase={[p[5] for p in ph if p[5]]}")
        print(f"  counters={r.get('counters')}")
        print(f"  factor_base={r.get('factor_base') is not None} boundaries={r.get('boundaries')}")
