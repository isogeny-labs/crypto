"""Strict analysis of native-monic-row/v1 receipts; performs no cryptography."""
import argparse
import hashlib
import json
import re
from pathlib import Path

TARGETS = (0, 1, 2, 25, 173, 292, 440, 478)
ARMS = ('baseline', 'candidate', 'aa')
STAGES = ['off', 'setup', 'prepare', 'query', 'verify']
COMPONENTS = ['geometry', 'phase', 'value', 'polynomial', 'source', 'decoder', 'verifier', 'allocator']
OPS = ['products', 'inversions', 'add_sub', 'negations', 'scalar_products', 'scalar_inversions', 'euclid_steps', 'terms', 'bit_ops', 'popcounts', 'alloc_calls', 'zero_alloc_calls', 'realloc_calls', 'free_calls', 'requested_bytes', 'freed_bytes', 'unit_inversions']

class Invalid(ValueError):
    pass

def require(condition, message):
    if not condition:
        raise Invalid(message)

def unique_object(pairs):
    result = {}
    for k, v in pairs:
        require(k not in result, 'duplicate JSON key: ' + k)
        result[k] = v
    return result

def forbidden_constant(value):
    raise Invalid('nonfinite JSON constant: ' + value)

def schedule():
    return ([(phase, arm, None) for phase in ('setup', 'prepare') for arm in ARMS]
            + [('query', arm, k) for k in TARGETS
               for arm in (ARMS if k % 2 == 0 else ('aa', 'candidate', 'baseline'))])

def metrics(c):
    return dict(field_inversions=sum(c[s][i][1] for s in (3,4) for i in range(7)),
                field_products=sum(c[s][i][0] for s in (3, 4) for i in range(7)),
                polynomial_products=c[3][3][0],
                unit_inversions=sum(c[s][i][16] for s in (3, 4) for i in range(7)),
                allocation_requests=c[3][7][14])

def analyze(raw, protocol_sha256, producer_exit):
    require(type(producer_exit) is int and producer_exit == 0, 'producer did not exit successfully')
    require(isinstance(protocol_sha256, str) and re.fullmatch('[0-9a-f]{64}', protocol_sha256), 'invalid expected protocol hash')
    lines = raw.decode('utf-8').splitlines()
    require(len(lines) == 31 and all(lines), 'expected exactly one header and 30 nonempty records')
    records = [json.loads(line, object_pairs_hook=unique_object, parse_constant=forbidden_constant) for line in lines]
    require(records[0] == dict(schema='native-monic-run/v1', protocol_sha256=protocol_sha256,
                              stages=STAGES, components=COMPONENTS, operations=OPS), 'header/protocol mismatch')
    rows = {}
    for index, (row, expected) in enumerate(zip(records[1:], schedule())):
        require(type(row) is dict and set(row) == {'schema', 'arm', 'target', 'phase', 'scalar', 'verified', 'counts'}, 'row keys')
        require(row['schema'] == 'native-monic-row/v1', 'row schema')
        phase, arm, target = expected
        require((row['phase'], row['arm'], row['target']) == expected, 'row order/identity at ' + str(index))
        if phase == 'query':
            require(type(row['target']) is int and type(row['scalar']) is int and row['scalar'] == target and row['verified'] is True, 'failed recovery')
        else:
            require(row['target'] is None and row['scalar'] is None and row['verified'] is False, 'setup metadata')
        c = row['counts']
        require(type(c) is list and len(c) == 5, 'stage dimensions')
        active = {'setup': (1,), 'prepare': (2,), 'query': (3, 4)}[phase]
        for s, stage in enumerate(c):
            require(type(stage) is list and len(stage) == 8, 'component dimensions')
            for i, component in enumerate(stage):
                require(type(component) is list and len(component) == 17, 'operation dimensions')
                require(all(type(v) is int and 0 <= v < 2**64 for v in component), 'counter must be u64 integer')
                require(s in active or not any(component), 'inactive stage has events')
                require(component[16] <= component[1], 'unit inversions exceed inversions')
                require(i != 7 or not any(component[:10] + component[16:]), 'allocator arithmetic contamination')
                require(i == 7 or not any(component[10:16]), 'allocator events in arithmetic component')
                if s == 4:
                    require(i in (6, 7) or not any(component), 'verifier stage contamination')
                if s == 3:
                    require(i != 6 or not any(component), 'verifier events in query stage')
        if phase == 'query':
            require(c[3][0][1] == 1 and c[3][5][7] == 477 and c[3][5][0] == 0, 'query geometry/decoder contract')
            require(metrics(c)['field_products'] > 0, 'empty arithmetic record')
        rows[expected] = row
    for phase, target in [('setup', None), ('prepare', None)] + [('query', k) for k in TARGETS]:
        require(rows[phase, 'baseline', target]['counts'] == rows[phase, 'aa', target]['counts'], 'A/A counter mismatch')
    pairs = []
    for k in TARGETS:
        b = metrics(rows['query', 'baseline', k]['counts'])
        c = metrics(rows['query', 'candidate', k]['counts'])
        pairs.append(dict(target=k, baseline=b, candidate=c, delta={m: c[m]-b[m] for m in b}))
    for pair in pairs:
        dm, di = pair['delta']['field_products'], pair['delta']['field_inversions']
        pair['weighted_model'] = dict(delta_products=dm, delta_inversions=di,
            model='delta_cost = delta_products + w*delta_inversions; constant w>=0; excludes other costs',
            candidate_better=('w > numerator/denominator' if di < 0 else 'w < numerator/denominator') if di else ('all w>=0' if dm < 0 else 'no w>=0'),
            numerator=dm if di < 0 else -dm if di > 0 else None,
            denominator=abs(di) if di else None)
    dims = ('field_products','field_inversions')
    def dominates(pair, direction):
        values = [direction*pair['delta'][key] for key in dims]
        return all(v <= 0 for v in values) and any(v < 0 for v in values)
    if all(dominates(p,1) for p in pairs):
        verdict = 'counted_pareto_improvement'
    elif all(dominates(p,-1) for p in pairs):
        verdict = 'counted_pareto_regression'
    else:
        verdict = 'counted_tradeoff_or_mixed' 
    return dict(schema='native-pair-seed-analysis/v1', verdict=verdict, protocol_sha256=protocol_sha256,
                raw_sha256=hashlib.sha256(raw).hexdigest(), query_rows=24, setup_prepare_rows=6,
                aa_exact=True, pairs=pairs, field_products_sum_delta=sum(p['delta']['field_products'] for p in pairs),
                scope='Eight fixed fixtures; Pareto verdict covers only products/inversions, not total cost; source/binary custody and independent replay are external gates')

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('rows', type=Path)
    parser.add_argument('--protocol-sha256', required=True)
    parser.add_argument('--producer-exit', required=True, type=int)
    parser.add_argument('--out', required=True, type=Path)
    args = parser.parse_args()
    raw = args.rows.read_bytes()
    try:
        result = analyze(raw, args.protocol_sha256, args.producer_exit)
    except (Invalid, ValueError, UnicodeError, TypeError) as error:
        result = dict(schema='native-pair-seed-analysis/v1', verdict='invalid_measurement',
                      raw_sha256=hashlib.sha256(raw).hexdigest(), error=str(error))
    # Never overwrite a previous analysis receipt or the raw input.
    with args.out.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.write('\n')
    return 1 if result['verdict'] == 'invalid_measurement' else 0

if __name__ == '__main__':
    raise SystemExit(main())
