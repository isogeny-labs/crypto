#!/usr/bin/env python3
"""Independent finite-field linear replay of the retained Magma receipts."""
from hashlib import sha256
from fractions import Fraction
import json
from pathlib import Path
import re
from math import gcd

HERE = Path(__file__).resolve().parent
REPLAY = HERE.parent/'ghs_q16_residual_quartic_search_20261027/verification.json'

def receipt(name):
    path = HERE / f'{name}.magma'
    obj = json.loads((HERE / f'{name}_receipt.json').read_text())
    assert obj['input_file'] == path.name
    assert obj['input_sha256'] == sha256(path.read_bytes()).hexdigest()
    assert obj['http_status'] == 200
    response = obj['response']
    assert response['success'] and response['exit_code'] == 0
    assert response['truncated'] is False
    return response['stdout']

def rank_and_solve(rows, right, prime):
    ncols = len(rows[0])
    a = [[v % prime for v in row] + [b % prime] for row,b in zip(rows,right)]
    rank = 0
    pivots = []
    for col in range(ncols):
        pivot = next((j for j in range(rank,len(a)) if a[j][col]),None)
        if pivot is None:
            continue
        a[rank],a[pivot] = a[pivot],a[rank]
        inv = pow(a[rank][col],-1,prime)
        a[rank] = [(v*inv)%prime for v in a[rank]]
        for j in range(len(a)):
            if j == rank or not a[j][col]:
                continue
            mul = a[j][col]
            a[j] = [(x-mul*y)%prime for x,y in zip(a[j],a[rank])]
        pivots.append(col)
        rank += 1
    assert rank == ncols
    assert all(not any(row[:ncols]) or row[ncols] == 0 for row in a[rank:])
    solution = [0]*ncols
    for j,col in enumerate(pivots):
        solution[col] = a[j][ncols]
    assert all(sum(x*y for x,y in zip(row,solution))%prime == b%prime
               for row,b in zip(rows,right))
    return rank,solution

def audit_pair(order,prime):
    assert prime>2 and all(prime%d for d in range(2,int(prime**0.5)+1))
    suffix='' if order==4126 else f'_{order}'
    attack = receipt('attack'+suffix)
    rho = receipt('rho'+suffix)
    flat = re.sub(r'\s+',' ',attack)
    m=re.search(r'genus (\d+) jacobian_order (\d+) subgroup_order (\d+) '
                r'projection_cofactor (\d+) factor_base (\d+)',flat)
    assert m
    genus,jorder,order_r,cofactor,fbsize=map(int,m.groups())
    assert genus==4 and order_r==prime and jorder==cofactor*prime
    assert gcd(cofactor,prime)==1
    matches = re.findall(r'relation (\d+) attempt (\d+) a (\d+) b (\d+) '
                         r'row \[ ([\d, ]+) \] rhs (\d+)',flat)
    assert len(matches)>=fbsize+1
    rows = []
    right = []
    attempts = []
    for seq,attempt,aa,bb,csv,rhs in matches:
        assert int(seq) == len(rows)+1
        row = [int(v.strip()) for v in csv.split(',')]
        assert len(row) == fbsize+1
        assert row[-1] == (-cofactor*int(bb))%prime
        assert int(rhs) == (cofactor*int(aa))%prime
        assert 1 <= sum(row[:-1]) <= 4
        rows.append(row)
        right.append(int(rhs))
        attempts.append(int(attempt))
    assert attempts == sorted(attempts)
    rank,sol = rank_and_solve(rows,right,prime)
    assert rank == fbsize+1 and sol[-1] == 7
    stats=re.search(r'attempted (\d+) smooth (\d+) relation_rows (\d+) '
                    r'matrix_rank (\d+) required_rank (\d+) '
                    r'relation_seconds ([.\d]+)',flat)
    assert stats
    tried,smooth,nrows,printed_rank,required,relation_seconds=stats.groups()
    tried,smooth,nrows,printed_rank,required=map(int,
                                                (tried,smooth,nrows,printed_rank,required))
    assert tried<=attempts[-1] and smooth==nrows==len(rows)
    assert printed_rank==required==rank
    assert 'target_jacobian_verified true source_verified true' in flat
    assert 'recovered_scalar 7' in flat
    m = re.search(r'sorted_walk_operations \[ ([\d, ]+) \]',re.sub(r'\s+',' ',rho))
    assert m
    counts = [int(v.strip()) for v in m.group(1).split(',')]
    assert len(counts)==512 and counts==sorted(counts)
    rflat=re.sub(r'\s+',' ',rho)
    rstats=re.search(r'trials (\d+) restarts (\d+) min_group_operations (\d+) '
                     r'median_group_operations (\d+) max_group_operations (\d+) '
                     r'mean_group_operations (\S+) total_seconds ([.\d]+)',rflat)
    assert rstats
    trials,restarts,minimum,median,maximum,mean,rho_seconds=rstats.groups()
    assert int(trials)==512 and int(restarts)>=0
    assert (int(minimum),int(median),int(maximum))==\
        (counts[0],counts[256],counts[-1])
    assert Fraction(mean)==Fraction(sum(counts),512)
    assert 'all_source_targets_verified true' in rho
    attack_seconds=re.search(r'attack_seconds ([.\d]+)',flat)
    assert attack_seconds
    return {'source_order':order,'subgroup_order':prime,
            'jacobian_order':jorder,'projection_cofactor':cofactor,
            'factor_base_size':fbsize,'relation_attempts':tried,
            'last_draw_index':attempts[-1],'smooth_relations':smooth,
            'matrix_rank':rank,'recovered_scalar':sol[-1],
            'relation_seconds':float(relation_seconds),
            'attack_seconds':float(attack_seconds.group(1)),
            'rho_trials':512,'rho_restarts':int(restarts),
            'rho_median_walk_operations':int(median),
            'rho_mean_walk_operations':float(Fraction(mean)),
            'rho_total_seconds':float(rho_seconds),
            'rho_seconds_per_target':float(rho_seconds)/512,
            'attack_to_rho_wall_ratio':float(attack_seconds.group(1))*512/float(rho_seconds),
            'attack_receipt_sha256':sha256((HERE/f'attack{suffix}_receipt.json').read_bytes()).hexdigest(),
            'rho_receipt_sha256':sha256((HERE/f'rho{suffix}_receipt.json').read_bytes()).hexdigest()}

def main():
    replay=json.loads(REPLAY.read_text())
    manifest=json.loads((HERE/'variants.json').read_text())
    assert manifest['source_replay_sha256']==sha256(REPLAY.read_bytes()).hexdigest()
    for kind in ('attack','rho'):
        assert manifest['template_sha256'][kind]==sha256((HERE/f'{kind}.magma').read_bytes()).hexdigest()
    for variant,source in zip(manifest['variants'],replay['checked'][:3]):
        assert variant['source_order']==source['source_order']
        assert variant['subgroup_order']==source['subgroup_order']
        assert variant['G']==source['G'] and variant['Q']==source['Q']
        for kind in ('attack','rho'):
            path=HERE/variant['scripts'][kind]['file']
            assert variant['scripts'][kind]['sha256']==sha256(path.read_bytes()).hexdigest()
    pilot=receipt('pilot')
    plane=receipt('plane_pilot')
    assert all(s in pilot for s in ('image_nonzero true','image_r_torsion true',
                                   'target_relation true','curve_type CrvHyp genus 4'))
    assert all(s in plane for s in ('image_nonprincipal true','image_r_principal true',
                                   'target_relation_principal true'))
    records=[audit_pair(s['source_order'],s['subgroup_order'])
             for s in replay['checked']]
    assert [x['source_order'] for x in records]==[4006,4054,4078,4126]
    summary={'schema':'ghs-q16-direct-hess-full-attack-benchmark/v1',
             'source_replay_sha256':sha256(REPLAY.read_bytes()).hexdigest(),
             'variant_manifest_sha256':sha256((HERE/'variants.json').read_bytes()).hexdigest(),
             'records':records}
    (HERE/'benchmark_summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print('PASS: four Hess subgroup maps, retained relation systems, '
          'source verification, and 512 rho trials per target')
    for x in records:
        print(x['source_order'],x['smooth_relations'],x['relation_attempts'],
              x['attack_seconds'],x['rho_seconds_per_target'])

if __name__ == '__main__':
    main()
