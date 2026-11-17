"""Independent prime-pool audit; validation secrets are never attack inputs.

Uses Jacobian group arithmetic, separate from the affine producer and the
incidence-polynomial solvers. Exact signed-sum truth is computed on scalar
labels only inside this validation program. That oracle is not an attack.
"""

import argparse
import copy
import hashlib
import json
import math
import os
from pathlib import Path
import time

from sympy import isprime


def twice(P, a, p):
    x,y,z=P
    if not z or not y:return (0,1,0)
    yy=y*y % p
    s=4*x*yy % p
    m=(3*x*x+a*pow(z,4,p)) % p
    xx=(m*m-2*s) % p
    return xx,(m*(s-xx)-8*yy*yy) % p,2*y*z % p


def plus(P,Q,a,p):
    x,y,z=P;u,v,w=Q
    if not z:return Q
    if not w:return P
    z2=z*z % p;w2=w*w % p
    u1=x*w2 % p;u2=u*z2 % p
    s1=y*w*w2 % p;s2=v*z*z2 % p
    if u1==u2:
        return twice(P,a,p) if s1==s2 else (0,1,0)
    h=(u2-u1) % p;r=(s2-s1) % p
    hh=h*h % p;hhh=h*hh % p
    xx=(r*r-hhh-2*u1*hh) % p
    return xx,(r*(u1*hh-xx)-s1*hhh) % p,h*z*w % p


def times(k,P,a,p):
    Q=(0,1,0)
    P=(P[0],P[1],1)
    while k:
        if k & 1:Q=plus(Q,P,a,p)
        P=twice(P,a,p);k >>= 1
    return Q


def affine(P,p):
    x,y,z=P
    if not z:return None
    iz=pow(z,-1,p)
    return x*iz*iz % p,y*iz*iz*iz % p


def validate_points(fixture, secret):
    c=fixture['curve'];p,a,b,r=c['p'],c['a'],c['b'],c['r']
    if not (p>3 and isprime(p) and isprime(r) and r!=p):
        raise ValueError('Field/order/anomalous curve gate')
    if (4*a*a*a+27*b*b) % p==0:raise ValueError('Singular curve')
    P,Q=tuple(c['P']),tuple(c['Q'])
    for name,R in (('P',P),('Q',Q)):
        if (R[1]*R[1]-R[0]**3-a*R[0]-b) % p:
            raise ValueError('Off-curve public '+name)
    radius=math.isqrt(4*p)
    if not ((p+1-radius+r-1)//r==(p+1+radius)//r==1):
        raise ValueError('Prime subgroup does not certify full order in Hasse interval')
    if affine(times(r,P,a,p),p) is not None:raise ValueError('Wrong subgroup order')
    if affine(times(secret,P,a,p),p)!=Q:raise ValueError('Wrong public Q')
    seen=set();count=0
    for pool in fixture['pools']:
        if len(pool)!=fixture['B']:raise ValueError('Wrong pool size')
        for row in pool:
            x,y=row['x'],row['y']
            if (y*y-x*x*x-a*x-b) % p:raise ValueError('Off-curve point')
            if x in seen:raise ValueError('Pools have coincident abscissae')
            seen.add(x)
            if not (0<=row['alpha']<r and 0<=row['beta']<r):
                raise ValueError('Noncanonical mixed label')
            value=plus(times(row['alpha'],P,a,p),times(row['beta'],Q,a,p),a,p)
            if affine(value,p)!=(x,y):raise ValueError('Mixed-label identity failure')
            count+=1
    return count


def scalar_reference(fixture,secret):
    r=fixture['curve']['r'];pools=fixture['pools']
    labels=[[(row['alpha']+secret*row['beta']) % r for row in pool] for pool in pools]
    third={}
    for k,value in enumerate(labels[2]):
        for s in (1,-1):third.setdefault(s*value % r,[]).append((k,s))
    geometric=False
    for i,x in enumerate(labels[0]):
        for j,y in enumerate(labels[1]):
            for s1 in (1,-1):
                for s2 in (1,-1):
                    for k,s3 in third.get((-s1*x-s2*y) % r,[]):
                        geometric=True
                        rows=[pools[0][i],pools[1][j],pools[2][k]]
                        signs=(s1,s2,s3)
                        A=sum(s*row['alpha'] for s,row in zip(signs,rows)) % r
                        B=sum(s*row['beta'] for s,row in zip(signs,rows)) % r
                        if B:
                            recovered=-A*pow(B,-1,r) % r
                            if recovered!=secret:raise ValueError('Recovery failure')
                            return {'geometric_exists':True,'informative_exists':True,
                                    'witness':{'positions':[i,j,k],'signs':list(signs)},
                                    'recovery_verified':True}
    return {'geometric_exists':geometric,'informative_exists':False,'witness':None}


def check_witness(fixture,witness):
    c=fixture['curve'];p,a,r=c['p'],c['a'],c['r']
    points=[];A=B=0
    for j,(index,sign) in enumerate(zip(witness['positions'],witness['signs'])):
        row=fixture['pools'][j][index]
        if sign not in (-1,1):raise ValueError('Bad sign')
        points.append((row['x'],sign*row['y'] % p,1))
        A=(A+sign*row['alpha']) % r;B=(B+sign*row['beta']) % r
    total=plus(plus(points[0],points[1],a,p),points[2],a,p)
    if total[2]:raise ValueError('Witness does not sum to infinity')
    if not B:raise ValueError('Witness is not informative')
    k=-A*pow(B,-1,r) % r
    if affine(times(k,tuple(c['P']),a,p),p)!=tuple(c['Q']):
        raise ValueError('Recovered scalar fails public check')
    return True


def main():
    arg=argparse.ArgumentParser()
    arg.add_argument('--fixtures',type=Path,default=Path(__file__).parent/'prime_fixtures')
    arg.add_argument('--output',type=Path,default=Path(__file__).with_name('independent_pool_audit.json'))
    args=arg.parse_args()
    os.sched_setaffinity(0,{4})
    start=time.process_time();results=[];mutation_checks=0
    for path in sorted(args.fixtures.glob('*.json')):
        if path.name.endswith('.validation.json'):continue
        fixture=json.loads(path.read_text())
        if fixture.get('schema')!='prime-pools/v1':continue
        validation=json.loads(path.with_suffix('.validation.json').read_text())
        secret=validation['secret_k']
        points=validate_points(fixture,secret)
        truth=scalar_reference(fixture,secret)
        if truth['geometric_exists']!=validation['signed_zero_sum_exists']:
            raise ValueError('Producer truth disagreement: '+path.name)
        if truth['witness']:
            check_witness(fixture,truth['witness'])
            wrong=copy.deepcopy(truth['witness']);wrong['signs'][0]*=-1
            try:check_witness(fixture,wrong)
            except ValueError:mutation_checks+=1
            else:raise ValueError('Single-sign mutation survived')
        for field in ('x','alpha','beta'):
            wrong=copy.deepcopy(fixture)
            modulus=fixture['curve']['p'] if field=='x' else fixture['curve']['r']
            wrong['pools'][0][0][field]=(wrong['pools'][0][0][field]+1) % modulus
            try:validate_points(wrong,secret)
            except ValueError:mutation_checks+=1
            else:raise ValueError('Fixture mutation survived: '+field)
        results.append({'id':fixture['id'],'B':fixture['B'],'kind':fixture['kind'],
                        'fixture_sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                        'point_labels_checked':points,**truth})
    report={'schema':'independent-prime-pool-audit/v1','scope':'Independent Jacobian point arithmetic and validation-only scalar sum oracle; no attack uses secret values.',
            'status':'passed','fixture_count':len(results),'point_labels_checked':sum(x['point_labels_checked'] for x in results),
            'mutation_checks':mutation_checks,'cpu_seconds':time.process_time()-start,'results':results}
    args.output.write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps({k:v for k,v in report.items() if k!='results'},indent=2))


if __name__=='__main__':main()
