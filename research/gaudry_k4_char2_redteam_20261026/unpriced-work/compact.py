import json,collections
base="/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/research/gaudry_k4_char2_crossover_20261015/sessions/"
PRICED={"sqrt_solves","as_solves","s4_pairs","lookups","row_ops","legendre_symbols","inversions","frobenius_maps","canonicalisations","target_guard_probes"}
for s in ['d-ladder','e-m15','e-m17-rho','f-lp']:
    print("=== session",s)
    sess=json.load(open(base+s+'/session.json'))
    print("  status",sess.get('status'),"records",sess.get('records_written'),"label",sess.get('label'))
    for line in open(base+s+'/records.jsonl'):
        r=json.loads(line)
        if r['warmup']: continue
        m=r['method']; c=r['cost']; w=r['workload']
        tot=c['total_gae'] or 0
        phs={p['name']:p for p in r['phases']}
        shares={k:round(100*(p['gae'] or 0)/tot,2) for k,p in phs.items()} if tot else {}
        nat=collections.OrderedDict()
        for p in r['phases']:
            for k,v in (p.get('native') or {}).items():
                if v: nat[k]=nat.get(k,0)+v
        notpriced={k:v for k,v in nat.items() if k not in PRICED}
        cnt={k:v for k,v in r['counters'].items() if k.endswith('_uncharged') and v}
        print(f" {r['arm']:8s} m={w['curve_spec']['m']} tgt={w['target_index']} method={m['id']} S={c['s']:.4g} totGAE={tot:.6g} LB={c['lower_bound']} unpriced={c['unpriced']}")
        print(f"    shares%={shares}")
        print(f"    native_nonzero_not_in_PRICED={notpriced}")
        print(f"    uncharged_counters={cnt}")
