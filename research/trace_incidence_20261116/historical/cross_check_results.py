"""Independent result checks using the root's Jacobian arithmetic.

Validation-only exact truth was written by independent_pool_audit.py. None of
these truth files or secret scalars is supplied to an attack implementation.
"""
import hashlib
import json
from pathlib import Path

from independent_pool_audit import check_witness

ROOT = Path(__file__).resolve().parent


def main():
    truth = {r['id']: r for r in json.loads(
        (ROOT / 'independent_pool_audit.json').read_text())['results']}
    records = json.loads((ROOT / 'incidence/benchmark_results.json').read_text())['records']
    summaries = []
    for record in records:
        key = record['fixture_id']
        raw = (ROOT / 'prime_fixtures' / (key + '.json')).read_bytes()
        fixture = json.loads(raw)
        assert hashlib.sha256(raw).hexdigest() == record['fixture_sha256']
        assert record['status'] in ('relation', 'none', 'uninformative')
        assert (record['status'] == 'relation') == truth[key]['informative_exists'], key
        assert (record['gcd_degree'] > 0) == truth[key]['geometric_exists'], key
        if record['status'] == 'relation':
            terms = sorted(record['witness'], key=lambda term: term['pool'])
            assert [term['pool'] for term in terms] == [0, 1, 2]
            witness = {'positions': [term['index'] for term in terms],
                       'signs': [term['sign'] for term in terms]}
            check_witness(fixture, witness)
            r = fixture['curve']['r']
            A = sum(term['sign'] * fixture['pools'][term['pool']][term['index']]['alpha'] for term in terms) % r
            B = sum(term['sign'] * fixture['pools'][term['pool']][term['index']]['beta'] for term in terms) % r
            assert record['recovery']['k'] == -A * pow(B, -1, r) % r
        summaries.append({'fixture_id': key, 'arm': record['arm'], 'status': record['status']})
    pair_report = json.loads((ROOT / 'repo_controls/pair_benchmark.json').read_text())
    assert pair_report['complete'] == 60
    control_trials = control_witnesses = 0
    for arm in ('native_pairs', 'quotient_scans'):
        assert len(pair_report[arm]) == 60
        for record in pair_report[arm]:
            key = record['fixture_id']
            raw = (ROOT / 'prime_fixtures' / (key + '.json')).read_bytes()
            assert hashlib.sha256(raw).hexdigest() == record['fixture_sha256']
            fixture = json.loads(raw)
            assert len(record['trials']) == 5
            for trial in record['trials']:
                control_trials += 1
                assert trial['status'] == record['status']
                assert (trial['status'] == 'relation') == truth[key]['informative_exists'], key
                if trial['status'] != 'relation':
                    continue
                terms = sorted(trial['witness'], key=lambda term: term['pool'])
                assert [term['pool'] for term in terms] == [0, 1, 2]
                witness = {'positions': [term['index'] for term in terms],
                           'signs': [term['sign'] for term in terms]}
                check_witness(fixture, witness)
                r = fixture['curve']['r']
                A = sum(term['sign'] * fixture['pools'][term['pool']][term['index']]['alpha'] for term in terms) % r
                B = sum(term['sign'] * fixture['pools'][term['pool']][term['index']]['beta'] for term in terms) % r
                assert trial['recovery']['k'] == -A * pow(B, -1, r) % r
                control_witnesses += 1
    report = {'schema': 'root-result-cross-check/v1', 'status': 'passed',
              'method': 'Independent Jacobian witness verification and exact validation-only truth; fixture hash parity.',
              'incidence_records_checked': len(summaries),
              'informative_witnesses_checked': sum(r['status'] == 'relation' for r in summaries),
              'exact_negative_records_checked': sum(r['status'] == 'none' for r in summaries),
              'control_trials_checked': control_trials,
              'control_witnesses_checked': control_witnesses,
              'records': summaries}
    (ROOT / 'cross_check_results.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: v for k, v in report.items() if k != 'records'}, indent=2))


if __name__ == '__main__':
    main()
