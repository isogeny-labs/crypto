"""Bounded exact source-coupled interval controls; coordinator executes frozen code."""
from pathlib import Path
from fractions import Fraction
from math import comb
import sys, time, json, gzip, hashlib, itertools, resource

HERE = Path(__file__).resolve().parent
START = time.monotonic()
CHECKS = 0
PRIMES = [2, 3, 5, 7, 11]
INDICES27 = list(itertools.product(range(3), repeat=3))
INDICES10 = list(itertools.combinations_with_replacement(range(3), 3))

def check(value, why):
    global CHECKS
    CHECKS += 1
    assert CHECKS <= 200000, 'declared predicate cap'
    assert time.monotonic() - START < 80, 'internal work deadline'
    assert value, why

def ceil_div(a, b):
    assert b > 0
    return -((-a)//b)

def excludes(interval, p):
    low, high = interval
    return low > high or ceil_div(low, p) > high//p

def ten_coefficients(params, M):
    a, B, C, gamma, delta = params
    coupling = 2*a-gamma
    result = {}
    for i, j, k in INDICES10:
        result[(i,j,k)] = (8*C + 4*B*M*(i+j+k)
            + 8*a*M*M*((i==2)+(j==2)+(k==2))
            + 2*coupling*M*M*(i*j+i*k+j*k) - delta*M**3*i*j*k)
    return result

def bernstein_interval(params, M):
    values = list(ten_coefficients(params, M).values())
    return ceil_div(min(values), 8), max(values)//8

def generic_coefficients(params, M):
    # Independent generic power-basis to Bernstein transform. It does not use
    # the closed ten-coefficient expression or assume permutation symmetry.
    a, B, C, gamma, delta = params
    power = {(0,0,0): C, (1,1,1): -delta*M**3}
    for axis in range(3):
        exponent = [0,0,0]; exponent[axis] = 1
        power[tuple(exponent)] = B*M
        exponent[axis] = 2
        power[tuple(exponent)] = a*M*M
    for i, j in [(0,1),(0,2),(1,2)]:
        exponent = [0,0,0]; exponent[i] = exponent[j] = 1
        power[tuple(exponent)] = (2*a-gamma)*M*M
    out = {}
    for index in INDICES27:
        value = Fraction(0)
        for exponent, coefficient in power.items():
            weight = Fraction(coefficient)
            for i, degree in zip(index, exponent):
                if degree > i:
                    weight = Fraction(0); break
                weight *= Fraction(comb(i, degree), comb(2, degree))
            value += weight
        out[index] = value
    return out

def independent_box(params, M):
    a, B, C, gamma, delta = params
    values = [a*S*S+B*S+C for S in range(3*M+1)]
    low, high = min(values), max(values)
    for term in [-gamma*3*M*M, -delta*M**3]:
        low += min(0, term); high += max(0, term)
    return low, high

def literal(params, source):
    a, B, C, gamma, delta = params
    x, y, z = source
    S, T, U = x+y+z, x*y+x*z+y*z, x*y*z
    return a*S*S+B*S+C-gamma*T-delta*U

def expanded(params, source):
    a, B, C, gamma, delta = params
    x, y, z = source
    return (C+B*(x+y+z)+a*(x*x+y*y+z*z)
            +(2*a-gamma)*(x*y+x*z+y*z)-delta*x*y*z)

def controls():
    records = []
    counts = dict(coefficient_cases=0, ordered_source_values=0,
                  prime_source_checks=0, phase_shift_checks=0,
                  old_exclusions=0, intersected_exclusions=0,
                  extra_exclusions=0, narrower_intervals=0)
    for params in itertools.product([-1,0,1], repeat=5):
        for M in range(4):
            coeff10 = ten_coefficients(params, M)
            coeff27 = generic_coefficients(params, M)
            check(len(coeff10)==10 and len(coeff27)==27 and
                  all(v*8 == coeff10[tuple(sorted(i))] for i,v in coeff27.items()),
                  'independent tensor coefficients agree, including all symmetry classes')
            bern = bernstein_interval(params, M)
            oracle_bern = (ceil_div(min(coeff27.values()).numerator,
                                    min(coeff27.values()).denominator),
                           max(coeff27.values()).numerator//max(coeff27.values()).denominator)
            old = independent_box(params, M)
            joint = max(old[0],bern[0]), min(old[1],bern[1])
            check(bern==oracle_bern and old[0]<=joint[0]<=joint[1]<=old[1],
                  'exact negative rounding and sound interval intersection')
            sources = list(itertools.product(range(M+1), repeat=3))
            values = [literal(params, s) for s in sources]
            expanded_values = [expanded(params, s) for s in sources]
            for value, second in zip(values, expanded_values):
                check(value==second and old[0]<=value<=old[1]
                      and bern[0]<=value<=bern[1] and joint[0]<=value<=joint[1],
                      'literal original elementary coefficients lie in both bounds')
                counts['ordered_source_values'] += 1
            rows = []
            for p in PRIMES:
                reject_old, reject_joint = excludes(old,p), excludes(joint,p)
                expected = [list(s) for s,v in zip(sources,values) if v%p==0]
                recovered = [] if reject_joint else [list(s) for s,v in zip(sources,expanded_values) if v%p==0]
                for value in values:
                    check(not reject_joint or value%p!=0,
                          'every modular exclusion is checked against all original sources')
                    counts['prime_source_checks'] += 1
                check(recovered==expected, 'complete ordered abstract support set, not only witness membership')
                for shift in [-3,-1,1,4]:
                    shifted = list(params); shifted[2] += shift*p
                    bb, oo = bernstein_interval(shifted,M), independent_box(shifted,M)
                    jj = max(bb[0],oo[0]), min(bb[1],oo[1])
                    check(jj==(joint[0]+shift*p,joint[1]+shift*p)
                          and excludes(jj,p)==reject_joint,
                          'signed phase shifts preserve exact modular masks')
                    counts['phase_shift_checks'] += 1
                counts['old_exclusions'] += int(reject_old)
                counts['intersected_exclusions'] += int(reject_joint)
                counts['extra_exclusions'] += int(reject_joint and not reject_old)
                rows.append(dict(modulus=p, old_excludes=reject_old,
                                 intersection_excludes=reject_joint,
                                 ordered_source_count_exact=len(expected), ordered_sources=recovered))
            counts['coefficient_cases'] += 1
            counts['narrower_intervals'] += int(joint[1]-joint[0]<old[1]-old[0])
            records.append(dict(parameters=list(params), M=M, old_interval=list(old),
                                bernstein_interval=list(bern), intersection=list(joint),
                                scaled_coefficients10=[[list(k),v] for k,v in coeff10.items()],
                                scaled_coefficients27=[[list(k),int(v*8)] for k,v in coeff27.items()],
                                modular_support=rows))
    return records, counts

def exceptional_controls():
    params = [1,0,1,2,0]; M=1; p=7
    old, bern = independent_box(params,M), bernstein_interval(params,M)
    values = [literal(params,s) for s in itertools.product(range(2),repeat=3)]
    check(old==(-5,10) and bern==(1,4) and not excludes(old,p)
          and excludes(bern,p) and all(v%p for v in values),
          'nonvacuous useful abstract coefficient witness')
    # Corrupt just the upper-corner coefficient. For this witness the other
    # coefficients are at most 3; the actual (1,1,1) value is 4.
    corrupt = ten_coefficients(params,M); corrupt[(2,2,2)] = 0
    wrong = ceil_div(min(corrupt.values()),8), max(corrupt.values())//8
    check(not wrong[0]<=literal(params,(1,1,1))<=wrong[1],
          'literal original-source oracle detects a single corrupted coefficient')
    for numerator in [-17,-9,-8,-7,-1,0,1,7,8,9,17]:
        low, high = ceil_div(numerator,8), numerator//8
        check(low==-(Fraction(-numerator,8).__floor__()) and high==Fraction(numerator,8).__floor__(),
              'negative integer endpoint rounding')
    return dict(parameters=params,M=M,modulus=p,old_interval=list(old),
                bernstein_interval=list(bern),ordered_source_values=values,
                abstract_coefficient_chart_only=True, actual_elliptic_realization_proved=False,
                corrupted_coefficient_rejected=True,negative_rounding_cases=11)

def main():
    ident = sys.argv[1]; records=[]; counts={}; witness=None; status='passed'; error=None
    try:
        records, counts = controls()
        witness = exceptional_controls()
    except (AssertionError, ValueError, RuntimeError, ZeroDivisionError) as exc:
        status='inconclusive'; error=str(exc)
    raw=dict(schema='source-coupled-bernstein-controls/v1', records=records,
             counts=counts,witness=witness,predicates=CHECKS,error=error)
    blob=gzip.compress(json.dumps(raw,sort_keys=True,separators=(',',':')).encode(),mtime=0)
    with(HERE/'raw-controls.json.gz').open('xb')as f:f.write(blob)
    evidence=dict(counts=counts,witness=witness,predicates=CHECKS,predicate_cap=200000,
                  complete_abstract_support_controls=status=='passed',
                  raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),
                  peak_process_rss_kib=resource.getrusage(resource.RUSAGE_SELF).ru_maxrss,
                  memory_metric='peak process self RSS KiB, not aggregate descendants',
                  actual_elliptic_chart_profiled=False,complete_global_root_search=False,
                  original_root_count_exact=None,qualified_speedup=None,
                  release_gate_claim=False,novelty_claim=False,error=error)
    result=dict(schema='scientific_result_v1',experiment_id=ident,status=status,
                summary='Exact source-coupled bounds and complete abstract modular support controls; no curve performance claim.',
                evidence=evidence,next_actions=['Freeze a new panel with empty fibers still unresolved by old5D; the saturated width8 panel can measure only overhead. Charge Bernstein construction, exact replay and every survivor before any gain claim.'])
    text=json.dumps(result,sort_keys=True,separators=(',',':'))
    if time.monotonic()-START>=84:
        result['status']='inconclusive';evidence['complete_abstract_support_controls']=False
        evidence['error']='final serialization deadline';text=json.dumps(result)
    assert time.monotonic()-START<88,'final report acceptance deadline'
    print(text)

if __name__=='__main__':main()
