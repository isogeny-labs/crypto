#!/usr/bin/env python3
"""Select a deterministic probe-count diagnostic; leave speed selection open."""
import itertools
import json
import math
from pathlib import Path

ROOT=Path(__file__).resolve().parent
ARMS=(1104,1107,1120)
ALPHA=0.05/3


def sign_p(wins,losses):
    n=wins+losses
    return min(1,2*sum(math.comb(n,i) for i in range(min(wins,losses)+1))/2**n) if n else 1


def select():
    rows={}
    for k in ARMS:
        path=ROOT/f'k{k}'/'independent_replay.json'
        if not path.exists():
            return {'schema':'n83.nearby-k-batch16-probe-selection/v1',
                    'status':'BOUNDED_UNKNOWN','reason':f'K{k} replay missing',
                    'performance_selection_status':'BOUNDED_UNKNOWN'}
        record=json.loads(path.read_text())
        if record['status']!='PASS_SECONDARY_PROBE_REPLAY' or not all(record['checks'].values()):
            return {'schema':'n83.nearby-k-batch16-probe-selection/v1',
                    'status':'BOUNDED_UNKNOWN','reason':f'K{k} replay failed',
                    'performance_selection_status':'BOUNDED_UNKNOWN'}
        rows[k]=record['targets']
    comparisons=[]
    for first,second in itertools.combinations(ARMS,2):
        a,b=rows[first],rows[second]
        assert [r['fixture_index'] for r in a]==[r['fixture_index'] for r in b]==list(range(16))
        wins_first=sum(x['probes']<y['probes'] for x,y in zip(a,b))
        wins_second=sum(x['probes']>y['probes'] for x,y in zip(a,b))
        ties=16-wins_first-wins_second
        comparisons.append({'first_k':first,'second_k':second,
                            'first_wins':wins_first,'second_wins':wins_second,
                            'ties':ties,'two_sided_sign_p':sign_p(wins_first,wins_second),
                            'paired_probe_ratios':[x['probes']/y['probes'] for x,y in zip(a,b)]})
    geometric={k:math.exp(math.fsum(math.log(row['probes']) for row in values)/16)
               for k,values in rows.items()}
    leader=min(ARMS,key=lambda k:geometric[k])
    passes=all((c['two_sided_sign_p']<ALPHA and
                (c['first_wins']>c['second_wins'] if c['first_k']==leader else
                 c['second_wins']>c['first_wins']))
               for c in comparisons if leader in (c['first_k'],c['second_k']))
    return {'schema':'n83.nearby-k-batch16-probe-selection/v1',
            'status':'PASS_PROBE_SCREEN' if passes else 'BOUNDED_UNKNOWN',
            'probe_leader_k':leader if passes else None,
            'descriptive_lowest_geometric_probes_k':leader,
            'geometric_mean_probes':geometric,'bonferroni_alpha_per_pair':ALPHA,
            'comparisons':comparisons,'target_count':16,
            'performance_selection_status':'BOUNDED_UNKNOWN',
            'performance_selection_reason':'secondary shared-host probe counts cannot establish paired Tier 1 one-target online or cold performance'}


if __name__=='__main__':
    result=select()
    (ROOT/'selection.json').write_text(json.dumps(result,indent=2,sort_keys=True)+'\n')
    print(json.dumps({'status':result['status'],'probe_leader_k':result.get('probe_leader_k'),
                      'performance_selection_status':result['performance_selection_status']},sort_keys=True))
