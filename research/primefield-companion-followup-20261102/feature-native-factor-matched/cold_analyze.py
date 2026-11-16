"""Derive single-target arithmetic from strictly validated stage records."""
import argparse
import hashlib
import json
from pathlib import Path
from analyze import analyze as query_analysis, Invalid, ARMS, unique_object, forbidden_constant


def analyze(raw, protocol_sha256, producer_exit):
    query = query_analysis(raw, protocol_sha256, producer_exit)
    records = [json.loads(line, object_pairs_hook=unique_object, parse_constant=forbidden_constant)
               for line in raw.decode('utf-8').splitlines()[1:]]
    overhead = {}
    for arm in ARMS:
        rows = [row for row in records if row['arm'] == arm and row['phase'] in ('setup', 'prepare')]
        overhead[arm] = {
            key: sum(row['counts'][stage][component][op] for row in rows
                     for stage in (1, 2) for component in range(7))
            for key, op in [('field_products', 0), ('field_inversions', 1)]
        }
    pairs = []
    for row in query['pairs']:
        b = {key: row['baseline'][key] + overhead['baseline'][key] for key in overhead['baseline']}
        c = {key: row['candidate'][key] + overhead['candidate'][key] for key in overhead['candidate']}
        delta = {key: c[key] - b[key] for key in b}
        dm, di = delta['field_products'], delta['field_inversions']
        pairs.append(dict(target=row['target'], baseline=b, candidate=c, delta=delta,
            weighted_model=dict(delta_products=dm, delta_inversions=di,
                model='single-target delta_products + w*delta_inversions; w>=0; excludes other costs',
                candidate_better=('w > numerator/denominator' if di < 0 else 'w < numerator/denominator') if di else ('all w>=0' if dm < 0 else 'no w>=0'),
                numerator=dm if di < 0 else -dm if di > 0 else None, denominator=abs(di) if di else None)))
    def dominates(row, direction):
        ds = [direction * v for v in row['delta'].values()]
        return all(v <= 0 for v in ds) and any(v < 0 for v in ds)
    verdict = ('counted_pareto_improvement' if all(dominates(row, 1) for row in pairs)
               else 'counted_pareto_regression' if all(dominates(row, -1) for row in pairs)
               else 'counted_tradeoff_or_mixed')
    return dict(schema='native-seed-factor-cold-analysis/v1', verdict=verdict,
        protocol_sha256=protocol_sha256, raw_sha256=hashlib.sha256(raw).hexdigest(),
        setup_prepare=overhead, pairs=pairs, query_analysis=query,
        exponent_setup=query['exponent_setup'], aa_exact=True,
        scope='Derived setup+prepare+query+verify field counts per single target; immutable prepared state, fresh query state; excludes other machine costs and does not establish cold elapsed time')

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
        result = dict(schema='native-seed-factor-cold-analysis/v1', verdict='invalid_measurement',
                      raw_sha256=hashlib.sha256(raw).hexdigest(), error=str(error))
    # Never overwrite a previous analysis receipt or the raw input.
    with args.out.open('x') as stream:
        json.dump(result, stream, indent=2)
        stream.write('\n')
    return 1 if result['verdict'] == 'invalid_measurement' else 0

if __name__ == '__main__':
    raise SystemExit(main())
