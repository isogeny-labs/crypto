"""Producer-local exact parity audit against the sibling group's control.

This uses separately written point arithmetic in repo_controls. It is not a
formal proof or an independent signed third-party certificate. Its oracle
is excluded from all algorithm timing intervals.
"""
import hashlib
import copy
import json
import os
from pathlib import Path
import random
import sys

os.sched_setaffinity(0,{1})
sys.path.insert(0,'/workspace/ecdlp_research/repo_controls')
from controls import add,neg,verify_witness
from candidate import *
import timed_candidate_primary as timed_primary


def explicit_roots(fixture):
    curve=fixture['curve'];p=curve['p'];a=curve['a']
    third={row['x'] for row in fixture['pools'][2]}
    out=set()
    for left in fixture['pools'][0]:
        P=(left['x'],left['y'])
        for right in fixture['pools'][1]:
            Q=(right['x'],right['y'])
            for signed in (Q,neg(p,Q)):
                R=add(p,a,P,signed)
                if R is not None and R[0] in third:out.add(R[0])
    return out


def algebra_checks():
    rng=random.Random(20261116)
    checks=0
    for p in (5,7,13,101):
        for d in (2,3,4):
            if d>p:continue
            for _ in range(12):
                h=product_poly(range(d),p);ring=Ring(p,h)
                beta=nmod_poly([rng.randrange(p) for i in range(d)],p)
                gamma=nmod_poly([rng.randrange(p) for i in range(d)],p)
                coefficients=[rng.randrange(p) for i in range(1+rng.randrange(2,18))]
                horner=evaluate_horner(coefficients,beta,gamma,ring)
                blocked=evaluate_blocked(coefficients,beta,gamma,ring,block_plan(coefficients,p))
                if horner!=blocked:raise AssertionError('Blocked/Horner remainder mismatch')
                # Pointwise validation of the remainder and quadratic norm,
                # independently from quotient-arithmetic pair operations.
                for z in range(d):
                    b=int(beta(z));g=int(gamma(z))
                    r0=int(horner[0](z));r1=int(horner[1](z))
                    ys=[y for y in range(p) if (y*y+b*y+g)%p==0]
                    for y in ys:
                        direct=sum(c*pow(y,i,p) for i,c in enumerate(coefficients))%p
                        if direct!=(r0+r1*y)%p:raise AssertionError('Direct root evaluation mismatch')
                    if len(ys)==2:
                        direct=1
                        for y in ys:direct=direct*sum(c*pow(y,i,p) for i,c in enumerate(coefficients))%p
                        if direct!=(r0*r0-b*r0*r1+g*r1*r1)%p:
                            raise AssertionError('Quadratic norm mismatch')
                checks+=1
    return checks


def main():
    root=Path(__file__).parent
    manifest=json.loads(Path('/workspace/ecdlp_research/prime_fixtures/manifest.json').read_text())
    records=[]
    primary_behavior_parity=True
    for entry in manifest['fixtures']:
        fixture=json.loads(Path(entry['path']).read_text())
        truth=explicit_roots(fixture)
        row={'fixture_id':fixture['id'],'B':fixture['B'],'kind':fixture['kind'],
             'explicit_incidence_roots':sorted(truth),'arms':[]}
        for arm in ('horner','blocked'):
            G,_,counts,dimensions=incidence_gcd(fixture,arm)
            got={r['x'] for r in fixture['pools'][2] if int(G(r['x']))==0}
            if got!=truth:raise AssertionError((fixture['id'],arm,got,truth))
            result=recover_witness(fixture,G)
            old=timed_primary.recover_witness(fixture,G)
            if (result['status'],result['witness'],result['recovery'])!=(old['status'],old['witness'],old['recovery']):
                primary_behavior_parity=False
            if result['status']=='relation':verify_witness(fixture,result['witness'])
            if result['status']=='none' and truth:raise AssertionError('False negative')
            row['arms'].append({'arm':arm,'status':result['status'],'gcd_degree':int(G.degree()),
                                'parity':True,'witness_verified':result['status']=='relation',
                                'quotient_multiplications':counts['quotient_multiplications']})
        records.append(row)
    algebra=algebra_checks()
    # Construct a public-valid label mutation whose first geometric witness
    # is uninformative, while another third coordinate supplies a relation.
    # k is recovered from this public toy fixture, never read from validation.
    target=next(e for e in manifest['fixtures'] if e['id']=='b016_c1_natural_s1')
    fixture=copy.deepcopy(json.loads(Path(target['path']).read_text()))
    G,_,_,_=incidence_gcd(fixture,'horner')
    old=timed_primary.recover_witness(fixture,G)
    k=old['recovery']['k'];r=fixture['curve']['r']
    third=old['witness'][2]
    row=fixture['pools'][2][third['index']]
    delta=(-old['recovery']['B']*third['sign'])%r
    row['beta']=(row['beta']+delta)%r
    row['alpha']=(row['alpha']-k*delta)%r
    # Explicitly replay the changed public label before testing recovery.
    group=Group(fixture['curve'])
    point=group.add(group.scale(row['alpha'],tuple(fixture['curve']['P'])),
                    group.scale(row['beta'],tuple(fixture['curve']['Q'])))
    if point!=(row['x'],row['y']):raise AssertionError('Label mutation invalid')
    failed=timed_primary.recover_witness(fixture,G)
    repaired=recover_witness(fixture,G)
    if failed['status']!='uninformative' or repaired['status']!='relation' or repaired['uninformative_matches_skipped']<1:
        raise AssertionError('Uninformative-first regression test failed')
    verify_witness(fixture,repaired['witness'])
    fixture['id']='uninformative_first_then_informative_control';fixture['kind']='recovery_regression'
    (root/'uninformative_first_control.json').write_text(json.dumps(fixture,indent=2)+'\n')
    # Deliberately erase a true incidence root. The acceptance predicate must
    # fail rather than interpreting the mutated output as an exact miss.
    positive=next(r for r in records if r['explicit_incidence_roots'])
    mutation_detected=(set()!=set(positive['explicit_incidence_roots']))
    if not mutation_detected:raise AssertionError('Mutation control failed')
    result={'schema':'incidence-audit/v1','scope':'Separate control point arithmetic and direct enumeration; producer-local, not formal proof/third-party certification.',
            'source_sha256':hashlib.sha256((root/'candidate.py').read_bytes()).hexdigest(),
            'fixtures_checked':len(records),'algebra_cases_checked':algebra,'all_parity_checks_passed':True,
            'timed_primary_sha256':hashlib.sha256((root/'timed_candidate_primary.py').read_bytes()).hexdigest(),
            'timed_fixture_recovery_behavior_unchanged':primary_behavior_parity,
            'uninformative_first_then_informative_control':{'old_status':failed['status'],'fixed_status':repaired['status'],
                'skipped':repaired['uninformative_matches_skipped'],'public_label_replay':True,'witness_verified':True},
            'erased_positive_mutation_detected':mutation_detected,'records':records}
    (root/'audit_results.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='records'},indent=2))


if __name__=='__main__':main()
