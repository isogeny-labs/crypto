"""CT215: deterministic, producer-side sensitivity analysis of CT214 raw toy costs.

Does not attest runner, memory, process coldness, PCC or scalable ECDLP.
"""
import json
import sys
from collections import defaultdict

PAIR = 'CT214_PAIR_JSON '
AGG = 'CT213_TOY_COST_JSON '
N, ROUNDS = 43, 7


def fail(message):
    raise ValueError(message)


def analyze(log, bootstrap_reps=12000):
    rows, aggregates = [], []
    for line in log.splitlines():
        if PAIR in line:
            rows.append(json.loads(line.split(PAIR, 1)[1].strip()))
        if AGG in line:
            aggregates.append(json.loads(line.split(AGG, 1)[1].strip()))
    if len(rows) != N * ROUNDS or len(aggregates) != 1:
        fail('expected exactly 301 pairs and one aggregate')
    by_target, by_round, by_order = defaultdict(list), defaultdict(list), defaultdict(list)
    seen = set()
    for row in rows:
        r, o, k = row['round'], row['offset'], row['target_index']
        if type(r) is not int or type(o) is not int or not 0 <= r < ROUNDS or not 0 <= o < N:
            fail('invalid round/offset')
        if type(k) is not int or k != (o + 17 * r) % N or (r, o) in seen:
            fail('duplicate or wrong roster target')
        seen.add((r, o))
        order = 'EJ' if (k+r) % 2 == 0 else 'JE'
        if row['arm_order'] != order:
            fail('execution order mismatch')
        ea, ja = row['direct_additions'], row['jacobian_additions']
        if type(ea) is not int or type(ja) is not int or ea != ja or ea != 13 + k//7:
            fail('wrong BSGS operation counts')
        if row['direct_baby_entries'] != 7 or row['jacobian_baby_entries'] != 7:
            fail('wrong baby-table size')
        for key in ('direct_ns','jacobian_ns'):
            if type(row[key]) is not int or row[key] <= 0:
                fail('invalid time')
        by_target[k].append(row)
        by_round[r].append(row)
        by_order[order].append(row)
    if len(by_target) != N or any(len(x) != ROUNDS for x in by_target.values()):
        fail('target repetition error')
    def totals(rs):
        return (sum(x['direct_ns'] for x in rs),sum(x['jacobian_ns'] for x in rs))
    e, j = totals(rows)
    agg = aggregates[0]
    if e != agg['direct_total_ns'] or j != agg['jacobian_total_ns'] or abs(e/j - agg['ratio_direct_over_jacobian']) > 1e-8:
        fail('aggregate mismatch')
    if agg['direct_bsgs_additions'] != 4690 or agg['jacobian_bsgs_additions'] != 4690:
        fail('wrong addition sum')
    if agg['memory'] != 'unmeasured' or agg['pcc'] != 'unverified':
        fail('unsupported PCC/memory status')
    rounds = [{'round':r,'ratio':(lambda a,b:a/b)(*totals(by_round[r]))} for r in range(ROUNDS)]
    orders = {k: {'pairs': len(v),'ratio': (lambda a,b: a/b)(*totals(v))} for k,v in sorted(by_order.items())}
    if set(by_order) != {'EJ','JE'} or sorted([len(v) for v in by_order.values()]) != [150,151]:
        fail('missing execution order')
    # Target-cluster resampling preserves seven repeated rounds per target.
    # It does NOT account for host systematic effects, and is NOT a confidence
    # interval for hardware or complete-ECDLP performance.
    clusters = [totals(by_target[k]) for k in range(N)]
    seed = 0x2152026
    ratios = []
    for _ in range(bootstrap_reps):
        ee, jj = 0, 0
        for _ in range(N):
            seed = (1664525 * seed + 1013904223) & 0xffffffff
            pair = clusters[(seed * N) >> 32]
            ee += pair[0]
            jj += pair[1]
        ratios.append(ee/jj)
    ratios.sort()
    q = lambda p: ratios[int(p*(len(ratios)-1))]
    return {
        'schema':'CT215-toy-negative-sensitivity-v1',
        'pairs':len(rows), 'distinct_targets':len(by_target),
        'direct_total_ns':e,'jacobian_total_ns':j,
        'direct_over_jacobian':e/j,'jacobian_slowdown':j/e,
        'direct_faster_pairs':sum(x['direct_ns'] < x['jacobian_ns'] for x in rows),
        'round_ratios':rounds,'order_ratios':orders,
        'descriptive_target_cluster_bootstrap':{'seed':'0x2152026','replicates':bootstrap_reps,'ratio_q025':q(.025),'ratio_q975':q(.975)},
        'standing':'single-hosted-run toy BSGS; not independent PCC or complete single-target IC/rho frontier'
    }


def self_test():
    rows = []
    for r in range(ROUNDS):
        for o in range(N):
            k=(o+17*r)%N
            rows.append(dict(round_=r, offset=o,target_index=k,arm_order='EJ' if (k+r)%2==0 else 'JE',
                             direct_ns=9000+k,jacobian_ns=600000+k,direct_additions=13+k//7,jacobian_additions=13+k//7,
                             direct_baby_entries=7,jacobian_baby_entries=7))
    for row in rows:row['round']=row.pop('round_')
    ee=sum(x['direct_ns'] for x in rows)
    jj=sum(x['jacobian_ns'] for x in rows)
    total={'direct_total_ns':ee,'jacobian_total_ns':jj,'ratio_direct_over_jacobian':ee/jj,
           'direct_bsgs_additions':4690,'jacobian_bsgs_additions':4690,'memory':'unmeasured','pcc':'unverified'}
    log='\n'.join(PAIR+json.dumps(x) for x in rows)+'\n'+AGG+json.dumps(total)
    assert analyze(log, 20)['direct_faster_pairs']==301
    try:
        analyze(log.replace('"target_index": 0','"target_index": 1',1),20)
    except ValueError:
        pass
    else:fail('tamper undetected')
    try:
        analyze(log.replace('"jacobian_total_ns":','"jacobian_total_ns": 1, "ignored":',1),20)
    except (ValueError,json.JSONDecodeError):
        pass
    else:fail('aggregate tamper undetected')
    print('CT215 self-tests passed')


if __name__ == '__main__':
    if len(sys.argv)==2 and sys.argv[1]=='--self-test':
        self_test()
    elif len(sys.argv)==2:
        with open(sys.argv[1],encoding='utf-8') as f: log=f.read()
        print('CT215_SENSITIVITY_JSON '+json.dumps(analyze(log),sort_keys=True))
    else:
        print('usage: ct215_paired_cost_audit.py [--self-test|native.log]', file=sys.stderr)
        sys.exit(2)
