#!/usr/bin/env python3
"""Independent consistency checks over retained Hess receipts and attacks."""

import collections
import hashlib
import json
import re
import statistics
from pathlib import Path

root=Path(__file__).resolve().parent

def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def magma(name):
    receipt=json.loads((root/f'{name}_receipt.json').read_text())
    assert receipt['input_sha256']==sha(root/f'{name}.magma')
    response=receipt['response']
    assert receipt['http_status']==200
    assert response['success'] is True and response['truncated'] is False
    return response['stdout']

def rank_mod(rows,p):
    if not rows: return 0
    work=[list(map(lambda x:x%p,row)) for row in rows]
    width=len(work[0]); rank=0
    for col in range(width):
        pivot=next((i for i in range(rank,len(work)) if work[i][col]),None)
        if pivot is None: continue
        work[rank],work[pivot]=work[pivot],work[rank]
        scale=pow(work[rank][col],-1,p)
        work[rank]=[(scale*x)%p for x in work[rank]]
        for i in range(len(work)):
            if i==rank: continue
            c=work[i][col]
            if c: work[i]=[(x-c*y)%p for x,y in zip(work[i],work[rank])]
        rank+=1
    return rank

population=json.loads((root/'population_screen.json').read_text())
records=population['records']
assert len(records)==315
assert population['criterion_outside_direct_order_closure']==189
assert sum(row['order']%8==4 for row in records)==144
assert all(row['a2']==0 and row['full_degree_b'] for row in records)
split=json.loads((root/'split_criterion_receipt.json').read_text())
assert [r['product_count'] for r in split['records']]==[6,45,315,2295]
assert [r['direct_branch_intersection_count'] for r in split['records']]==[3,15,63,255]
assert [r['distinct_factor_outside_direct_count'] for r in split['records']]==[3,30,252,2040]
assert [r['subfield_product_count'] for r in split['records']]==[0,3,0,15]

sweeps={}
for order in (524,548,554,556):
    stdout=magma(f'all_{order}')
    rows=[line.split() for line in stdout.splitlines() if line.startswith('row ')]
    assert len(rows)==511
    assert [int(row[3]) for row in rows]==list(range(511))
    hist=dict(sorted(collections.Counter(int(row[4]) for row in rows).items()))
    sweeps[str(order)]={'rows':len(rows),'genus_histogram':hist}
assert sweeps['548']['genus_histogram']=={3:14,4:14,6:98,7:385}
assert sweeps['556']['genus_histogram']=={3:14,4:14,6:98,7:385}
assert sweeps['524']['genus_histogram']=={4:14,6:126,7:371}
assert sweeps['554']['genus_histogram']=={4:14,6:126,7:371}

for order in (548,556):
    stdout=magma(f'canonical_map_{order}')
    assert 'orders false true true' in stdout
    assert 'D_support_degrees [ <3, 1>, <3, 1>, <1, -6> ]' in stdout
    assert 'T_support_degrees [ <3, 1>, <3, 1>, <1, -6> ]' in stdout
assert 'map_generator_nonzero true' in magma('cert_548')
assert 'map_generator_137_zero true' in magma('cert_548')
assert 'map_target_equals_7G true' in magma('cert_548')

attacks={}
for order,prime in ((548,137),(556,139)):
    attack_file=root/f'attack_{order}.json'
    rho_file=root/f'rho_{order}.json'
    if not attack_file.exists():
        attacks[str(order)]={'status':'running_or_missing'}
        continue
    attack=json.loads(attack_file.read_text())
    rho=json.loads(rho_file.read_text())
    assert attack['status']=='solved' and attack['answer']==7
    assert attack['source_b']==(155 if order==548 else 32)
    assert attack['prime']==prime and rho['prime']==prime
    assert rho['trial_count']==32 and all(t['answer']==7 for t in rho['trials'])
    logs=attack['kernel_vector']; m=attack['factor_base_size']
    assert len(logs)==m and attack['kernel_dimension']==1
    assert rank_mod(attack['relations'],prime)==m-1
    assert all(sum(x*y for x,y in zip(row,logs))%prime==0
               for row in attack['relations'])
    decomp=attack['decompositions']
    g=sum(x*y for x,y in zip(decomp['G'],logs))%prime
    q=sum(x*y for x,y in zip(decomp['Q'],logs))%prime
    assert g and q*pow(g,-1,prime)%prime==attack['answer']
    attacks[str(order)]={'status':'solved','answer':attack['answer'],
                         'transfer_total_seconds':attack['total_seconds'],
                         'rho_median_seconds':rho['median_seconds'],
                         'time_ratio':attack['total_seconds']/rho['median_seconds'],
                         'walk_steps':attack['walk_steps'],
                         'relation_rank':m-1,
                         'attack_input_sha256':sha(root/f'attack_{order}.sage'),
                         'rho_input_sha256':sha(root/f'rho_{order}.sage')}

native_index={}
for order,prime in ((548,137),(556,139)):
    runs=[]
    input_hash=sha(root/f'index_rho_{order}.magma')
    for trial in (1,2,3):
        name=f'index_rho_{order}_run{trial}'
        receipt=json.loads((root/f'{name}_receipt.json').read_text())
        assert receipt['input_sha256']==input_hash
        assert receipt['http_status']==200
        response=receipt['response']
        assert response['success'] is True and response['truncated'] is False
        stdout=response['stdout']
        assert 'lifted_degree 0 0' in stdout
        assert 'lifted_orders false true true' in stdout
        assert f'DLP mod {prime}: 7' in stdout
        assert 'index_answer 7 ' in stdout
        assert 'source_verified true' in stdout
        def number(label):
            match=re.search(rf'\b{re.escape(label)} ([0-9.]+)',stdout)
            assert match is not None, (name,label)
            return float(match.group(1))
        relations=int(re.search(r'Number of relations found: (\d+)',stdout).group(1))
        factor_base=int(re.search(r'Number of elements in factor base: (\d+)',stdout).group(1))
        attack=number('attack_total_wall')
        rho_total=number('rho_total_wall')
        rho_trials=int(number('rho_trials'))
        assert rho_trials==512 and attack>0 and rho_total>0
        runs.append({'trial':trial,'attack_total_seconds':attack,
                     'rho_total_seconds':rho_total,
                     'rho_trials':rho_trials,'rho_mean_seconds':rho_total/rho_trials,
                     'attack_over_rho_mean':attack/(rho_total/rho_trials),
                     'relations':relations,'factor_base_size':factor_base,
                     'index_wall_seconds':number('index_wall')})
    attack_median=statistics.median(x['attack_total_seconds'] for x in runs)
    rho_median=statistics.median(x['rho_mean_seconds'] for x in runs)
    native_index[str(order)]={
        'status':'solved','answer':7,'input_sha256':input_hash,
        'runs':runs,
        'attack_median_seconds':attack_median,
        'rho_mean_median_seconds':rho_median,
        'ratio_of_median_costs':attack_median/rho_median}

result={'schema':'ghs-hess-cubic-audit/v1','product_family_count':315,
        'outside_direct_order_closure_count':189,'order_four_mod8_count':144,
        'sweeps':sweeps,'attacks':attacks,'native_index':native_index}
(root/'audit.json').write_text(json.dumps(result,indent=2)+'\n')
print(json.dumps(result,indent=2))
