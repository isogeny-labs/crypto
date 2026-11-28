"""Exact structured quadratic-extension inverse; coordinator executes frozen code."""
from pathlib import Path
import sys, time, json, gzip, hashlib, itertools, statistics, resource
from research.extension_oracle_20261111_v1.worker import Field, TableOracle

HERE = Path(__file__).resolve().parent
START = time.monotonic()
CHECKS = 0
FIELDS = [(3, 2), (5, 3), (7, 3), (11, 2), (13, 2)]
PROFILE_FIELDS = FIELDS[:3]
FUNCTIONALS = [[1,0,0,0], [0,1,0,0], [0,0,1,0], [0,0,0,1],
               [1,1,0,0], [0,1,1,0], [0,0,1,1], [1,0,0,1]]

def check(value, why):
    global CHECKS
    CHECKS += 1
    assert CHECKS <= 200000, 'predicate budget'
    assert time.monotonic() - START < 80, 'internal work deadline'
    assert value, why

def field(p, d):
    # These are explicitly small prime inputs, not unproved arbitrary parameters.
    check(p > 2 and all(p % k for k in range(2, int(p**0.5)+1)), 'odd prime')
    check(all(x*x % p != d for x in range(p)) and pow(d, (p-1)//2, p) == p-1,
          'irreducible theta squared minus d')
    return Field(p, (-d) % p)

def power(f, a, n):
    result = 1
    while n:
        if n & 1:
            result = f.mul(result, a)
        a = f.mul(a, a)
        n >>= 1
    return result

def oracle_output(o, p, x, y):
    theta = p
    return o.add(x, o.mul(theta, y)), o.add(o.mul(x, x), o.mul(theta, o.mul(y, y)))

def expected_map(p, d):
    o = TableOracle(p, (-d) % p)
    buckets = {}
    for x, y in itertools.product(range(p), repeat=2):
        buckets.setdefault(oracle_output(o, p, x, y), []).append([x, y])
    check(len(buckets) == p*p and all(len(v) == 1 for v in buckets.values()),
          'independent first-coordinate injectivity')
    return o, buckets

def candidate_rows(p, f):
    # Actual generic field powers, not coordinate extraction disguised as Frobenius.
    theta = p
    inverse_two = power(f, 2, f.q-2)
    inverse_two_theta = power(f, f.mul(2, theta), f.q-2)
    check(f.mul(2, inverse_two) == 1 and f.mul(f.mul(2, theta), inverse_two_theta) == 1,
          'constant inverse identities')
    decoded = []
    for t in range(f.q):
        conjugate = power(f, t, p)
        x = f.mul(f.add(t, conjugate), inverse_two)
        y = f.mul(f.add(t, f.neg(conjugate)), inverse_two_theta)
        check(0 <= x < p and 0 <= y < p and f.add(x, f.mul(theta, y)) == t,
              'inverse source lies in declared prime subfield and reconstructs t')
        u = f.add(f.mul(x, x), f.mul(theta, f.mul(y, y)))
        decoded.append((x, y, u))
    rows = []
    for t, u in itertools.product(range(f.q), repeat=2):
        x, y, wanted_u = decoded[t]
        rows.append(dict(target=[t, u], roots=[[x, y]] if u == wanted_u else []))
    return rows

def indexed_rows(p, f):
    # Strong image-index comparator: visit p squared sources once, not per target.
    buckets = {}
    for x, y in itertools.product(range(p), repeat=2):
        t = f.add(x, f.mul(p, y))
        u = f.add(f.mul(x, x), f.mul(p, f.mul(y, y)))
        buckets.setdefault((t, u), []).append([x, y])
    return [dict(target=[t, u], roots=buckets.get((t, u), []))
            for t, u in itertools.product(range(f.q), repeat=2)]

def checked_rows(rows, p, oracle, expected):
    q = p*p
    check(len(rows) == q*q and len({tuple(row['target']) for row in rows}) == q*q,
          'all target rows exactly once')
    for row in rows:
        t, u = row['target']
        roots = row['roots']
        check(roots == expected.get((t, u), []) and
              all(0 <= x < p and 0 <= y < p and oracle_output(oracle, p, x, y) == (t, u)
                  for x, y in roots), 'complete independent ordered root set')
    # Both compared arms serialize exactly the same root-report payload.
    return json.dumps(rows, sort_keys=True, separators=(',', ':'))

def counterexample():
    p = 3
    target = [1, 1, 0, 0]
    images = [(x, y, [x, y, x*x % p, y*y % p])
              for x, y in itertools.product(range(p), repeat=2)]
    witnesses = [(1,0), (0,1), (0,0), (0,0), (1,1), (0,1), (0,0), (1,0)]
    check(not any(coords == target for _, _, coords in images), 'no common source')
    for functional, witness in zip(FUNCTIONALS, witnesses):
        coords = next(coords for x, y, coords in images if (x, y) == witness)
        check(sum(a*b for a,b in zip(functional, coords)) % p ==
              sum(a*b for a,b in zip(functional, target)) % p, 'separate functional witness')
    check(not any([coords[0], coords[2]] == [target[0], target[2]] for _, _, coords in images),
          'joint projection rejects scalar false positive')
    return dict(field_order=9, domain='Fp', target_coordinates=target,
                functionals=FUNCTIONALS, separate_witnesses=[list(v) for v in witnesses],
                common_source_exists=False, joint_projection_rejects=True)

def arm(name, p, d, oracle, expected):
    began = time.monotonic()
    f = field(p, d)
    rows = candidate_rows(p, f) if name == 'frobenius' else indexed_rows(p, f)
    serialized = checked_rows(rows, p, oracle, expected)
    elapsed = time.monotonic() - began
    return dict(arm=name, p=p, d=d, q=p*p, domain='Fp', basis=['1', 'theta'],
                modulus=[(-d) % p, 0, 1], target_count=p**4,
                source_count=p*p, paid_component_seconds=elapsed,
                serialized_output_bytes=len(serialized.encode()),
                complete_structured_target_batch=True, rows=rows)

def main():
    ident = sys.argv[1]
    status = 'passed'
    error = None
    controls = []
    records = []
    summaries = []
    witness = None
    try:
        prepared = {}
        for p, d in FIELDS:
            f = field(p, d)
            oracle, expected = expected_map(p, d)
            for a, b in itertools.product(range(f.q), repeat=2):
                check(f.mul(a, b) == oracle.mul(a, b) and f.add(a, b) == oracle.add(a, b),
                      'independent field arithmetic')
            rows = candidate_rows(p, f)
            checked_rows(rows, p, oracle, expected)
            controls.append(dict(p=p, d=d, q=f.q, target_count=p**4,
                                 source_count=p*p, nonempty_targets=len(expected),
                                 complete_structured_target_batch=True, rows=rows))
            prepared[p] = (oracle, expected)
        witness = counterexample()
        for rep, order in enumerate([['indexed', 'frobenius'], ['frobenius', 'indexed'],
                                     ['indexed', 'frobenius']]):
            for name in order:
                for p, d in PROFILE_FIELDS:
                    row = arm(name, p, d, *prepared[p])
                    row['rep'] = rep
                    records.append(row)
        for p, d in PROFILE_FIELDS:
            medians = {name:statistics.median(v['paid_component_seconds'] for v in records
                                             if v['p'] == p and v['arm'] == name)
                       for name in ['indexed', 'frobenius']}
            summaries.append(dict(p=p, d=d, q=p*p, targets=p**4, nonempty_targets=p*p,
                                  indexed_median_seconds=medians['indexed'],
                                  frobenius_median_seconds=medians['frobenius'],
                                  diagnostic_component_ratio=medians['indexed']/medians['frobenius'],
                                  cost_prediction_supported=medians['frobenius'] < medians['indexed']))
    except (AssertionError, ValueError, RuntimeError, ZeroDivisionError, KeyError, StopIteration) as exc:
        status = 'inconclusive'
        error = str(exc)
    if status != 'passed':
        for row in controls + records:
            row['complete_structured_target_batch'] = False
        for row in summaries:
            row.update(diagnostic_component_ratio=None, cost_prediction_supported=None)
    raw = dict(schema='joint-frobenius-structured-inverse/v1', controls=controls,
               records=records, summaries=summaries, counterexample=witness,
               checks=CHECKS, error=error)
    blob = gzip.compress(json.dumps(raw, sort_keys=True, separators=(',', ':')).encode(), mtime=0)
    with (HERE/'raw-profile.json.gz').open('xb') as stream:
        stream.write(blob)
    evidence = dict(correctness_field_summaries=[{k:v for k,v in row.items() if k!='rows'}
                                               for row in controls],
                    profile_summaries=summaries, checks=CHECKS, predicate_cap=200000,
                    complete_structured_target_comparison=status=='passed',
                    counterexample=witness, raw_sha256=hashlib.sha256(blob).hexdigest(),
                    raw_bytes=len(blob), peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                    memory_metric='peak process self RSS KiB, not aggregate descendants',
                    timing_scope='Each arm constructs and validates field, creates full target-root batch, independently verifies all roots and serializes identical output. Candidate uses binary-exponentiation Frobenius and caches q decoded values; baseline indexes p squared source pairs once.',
                    common_setup_policy='Independent table-oracle construction and arithmetic crosschecks charged whole worker, outside both paid arms equally.',
                    storage_scope='Measured batch caches O(q) decoded values/index and emits O(q squared) target rows; this is not measured constant-space point solving.',
                    elliptic_chart_coverage=False, complete_global_root_search=False,
                    original_root_count_exact=None, qualified_speedup=None,
                    release_gate_claim=False, novelty_claim=False, error=error)
    result = dict(schema='scientific_result_v1', experiment_id=ident, status=status,
                  summary='Complete structured quadratic-extension Frobenius inverse versus indexed image; no elliptic-chart or qualified speed claim.',
                  evidence=evidence,
                  next_actions=['If generic exponentiation loses to indexing, separately test charged precomputed linear Frobenius, or derive an actual guarded source-linear invertible curve chart; never transfer this special bijection without a rank/domain/exceptional proof.'])
    text = json.dumps(result, sort_keys=True, separators=(',', ':'))
    if time.monotonic() - START >= 84:
        result['status'] = 'inconclusive'
        evidence.update(complete_structured_target_comparison=False, error='final serialization deadline')
        for row in evidence['correctness_field_summaries']:
            row['complete_structured_target_batch'] = False
        for row in summaries:
            row.update(diagnostic_component_ratio=None, cost_prediction_supported=None)
        text = json.dumps(result, sort_keys=True, separators=(',', ':'))
    assert time.monotonic() - START < 88, 'full serialized report deadline'
    print(text)

if __name__ == '__main__':
    main()
