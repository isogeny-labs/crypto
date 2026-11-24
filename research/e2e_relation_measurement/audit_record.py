#!/usr/bin/env python3
"""Independently audit saved arithmetic records over bounded prime fields.

This verifier validates supplied witnesses; it never searches for a scalar.
"""
import argparse
from collections import Counter
import hashlib
import json
import math
import time
from pathlib import Path


class Invalid(ValueError):
    pass


def require(condition, message):
    if not condition:
        raise Invalid(message)


def prime32(n):
    if type(n) is not int or not 2 <= n < 2**32:
        return False
    if n % 2 == 0:
        return n == 2
    for d in range(3, math.isqrt(n) + 1, 2):
        if n % d == 0:
            return False
    return True


class Curve:
    def __init__(self, p, a, b):
        require(prime32(p) and p > 3, 'field prime must be between 5 and 2^32-1')
        require(type(a) is int and type(b) is int, 'invalid curve coefficients')
        self.p, self.a, self.b = p, a % p, b % p
        require((4*self.a**3 + 27*self.b**2) % p != 0, 'singular curve')

    def point(self, value):
        if value is None:
            return None
        require(isinstance(value,list) and len(value)==2 and all(type(v) is int for v in value), 'invalid point encoding')
        x,y=value
        require(0 <= x < self.p and 0 <= y < self.p, 'noncanonical point')
        require((y*y-x*x*x-self.a*x-self.b) % self.p == 0, 'point not on curve')
        return x,y

    def neg(self, value):
        return None if value is None else (value[0],(-value[1]) % self.p)

    def add(self, left, right):
        if left is None: return right
        if right is None: return left
        x,y=left; u,v=right; p=self.p
        if x==u and (y+v) % p==0: return None
        if x==u:
            require(y==v, 'invalid same-x point pair')
            slope=(3*x*x+self.a)*pow(2*y,-1,p) % p
        else:
            slope=(v-y)*pow(u-x,-1,p) % p
        z=(slope*slope-x-u) % p
        return z,(slope*(x-z)-y) % p

    def mul(self, scalar, point):
        require(type(scalar) is int and scalar >= 0, 'invalid scalar')
        result=None
        while scalar:
            if scalar & 1: result=self.add(result,point)
            point=self.add(point,point); scalar >>= 1
        return result


def vanishing(xs, axis, p):
    coefficients=[1]
    for x in xs:
        nxt=[0]*(len(coefficients)+1)
        for i,c in enumerate(coefficients):
            nxt[i]=(nxt[i]-x*c)%p; nxt[i+1]=(nxt[i+1]+c)%p
        coefficients=nxt
    return {((i,0) if axis==0 else (0,i)): c for i,c in enumerate(coefficients) if c}


def expected_system(p,a,b,z,left,right):
    values={(2,2):1,(2,1):-2*z,(1,2):-2*z,(2,0):z*z,(0,2):z*z,
            (1,1):-2*z*z-2*a,(1,0):-2*a*z-4*b,(0,1):-2*a*z-4*b,
            (0,0):a*a-4*b*z}
    return [{k:v%p for k,v in values.items() if v%p}, vanishing(left,0,p), vanishing(right,1,p)]


def decode_terms(system, p):
    require(system.get('variables')==['x','y'] and system.get('order')=='degrevlex', 'unexpected polynomial representation')
    require(system.get('coefficient_modulus')==p, 'wrong coefficient modulus')
    rows=system.get('input_terms')
    require(isinstance(rows,list) and len(rows)==3, 'missing input terms')
    result=[]
    for terms in rows:
        require(isinstance(terms,list), 'invalid term list')
        poly={}
        for term in terms:
            exp=term.get('exponents'); c=term.get('coefficient')
            require(isinstance(exp,list) and len(exp)==2 and all(type(e) is int and e>=0 for e in exp), 'invalid exponent')
            require(type(c) is int and 0<c<p, 'invalid coefficient')
            require(tuple(exp) not in poly, 'duplicate term')
            poly[tuple(exp)]=c
        result.append(poly)
    require(system.get('input_term_counts')==[len(poly) for poly in result], 'term counts do not match')
    require(system.get('input_degrees')==[max((sum(exp) for exp in poly),default=-1) for poly in result], 'input degrees do not match')
    return result


def evaluate(poly,x,y,p):
    return sum(c*pow(x,i,p)*pow(y,j,p) for (i,j),c in poly.items()) % p


def audit(record):
    require(record.get('schema')=='e2e-relation-measurement/v1','unsupported schema')
    config=record['config']; curve=record['curve']; p=curve['p']; n=curve['n']
    e=Curve(p,curve['a'],curve['b'])
    require(prime32(n),'subgroup order must be prime and below 2^32')
    for key in ('p','a','b'):
        require(curve[key]==config[{'p':'p','a':'curve_a','b':'curve_b'}[key]], 'curve and configuration differ')
    P=e.point(curve['P']); Q=e.point(curve['Q'])
    require(P is not None and Q is not None, 'identity base or target')
    require(all(type(config[key]) is int for key in ('px','py','qx','qy')), 'invalid configured points')
    require(P==(config['px']%p,config['py']%p) and Q==(config['qx']%p,config['qy']%p), 'configured points differ')
    require(e.mul(n,P) is None and e.mul(n,Q) is None, 'subgroup order check failed')
    target=record['target']; ar=target['a']; br=target['b']
    require(type(ar) is int and type(br) is int and 0<=ar<n and 0<=br<n,'invalid target coefficients')
    R=e.point(target['point'])
    require(R is not None and e.add(e.mul(ar,P),e.mul(br,Q))==R, 'target coefficients do not reproduce point')
    block=config['block_size']; require(type(block) is int and block>0,'invalid block size')
    require(0<=config['x_min']<=config['x_max']<p and config['modulus']>0 and 0<=config['residue']<config['modulus'], 'invalid filter')
    seen=set(); accepted=[]; counted={'candidates':0,'identity':0,'filtered':0,'duplicate_x':0}
    trace=record['candidates']; require(isinstance(trace,list), 'invalid candidate trace')
    for i,candidate in enumerate(trace):
        require(candidate['index']==i,'candidate index mismatch')
        a,b=candidate['a'],candidate['b']
        require(type(a) is int and type(b) is int and 0<=a<n and 0<=b<n,'invalid candidate coefficients')
        point=e.point(candidate['point'])
        require(e.add(e.mul(a,P),e.mul(b,Q))==point,'candidate coefficients do not reproduce point')
        counted['candidates']+=1
        if point is None: reason='identity'
        elif not config['x_min']<=point[0]<=config['x_max'] or point[0]%config['modulus']!=config['residue']: reason='filtered'
        elif point[0] in seen: reason='duplicate_x'
        else:
            reason='accepted'; seen.add(point[0]); accepted.append((a,b,point))
            if len(accepted)==2*block: require(i==len(trace)-1,'trace continues after both blocks filled')
        require(candidate['status']==reason,'candidate classification mismatch')
        if reason!='accepted': counted[reason]+=1
    require(record['counts']==counted,'candidate counts mismatch')
    require(counted['candidates']<=config['max_candidates'] and len(accepted)<=2*block,'candidate budget exceeded')
    status=record['status']
    if status=='candidate_budget_exhausted':
        require(len(accepted)<2*block and counted['candidates']==config['max_candidates'],'invalid exhaustion status')
        return {'status':'verified_partial','candidates':len(trace),'accepted':len(accepted),'reason':'no complete system'}
    require(status=='complete' and len(accepted)==2*block,'record is not a completed valid run')
    system=decode_terms(record['system'],p)
    require(system==expected_system(p,e.a,e.b,R[0],[v[2][0] for v in accepted[:block]],[v[2][0] for v in accepted[block:]]),'system coefficients mismatch')
    roots=record['roots']; require(isinstance(roots,list),'invalid root list')
    root_set=set()
    for root in roots:
        require(isinstance(root,list) and len(root)==2 and all(type(v) is int and 0<=v<p for v in root),'invalid root')
        pair=tuple(root); require(pair not in root_set,'duplicate root'); root_set.add(pair)
        require(all(evaluate(poly,*pair,p)==0 for poly in system),'root fails input system')
    expected_witnesses=set()
    for i,(_,_,u) in enumerate(accepted[:block]):
        for j,(_,_,v) in enumerate(accepted[block:],start=block):
            if (u[0],v[0]) not in root_set: continue
            for su in (-1,1):
                for sv in (-1,1):
                    if e.add(u if su==1 else e.neg(u),v if sv==1 else e.neg(v))==R:
                        expected_witnesses.add((i,j,su,sv))
    actual=set(); zero=nonzero=verified=0
    for witness in record['witnesses']:
        indices=witness['indices']; signs=witness['signs']
        require(isinstance(indices,list) and len(indices)==2 and all(type(i) is int for i in indices), 'invalid witness indices')
        require(0<=indices[0]<block<=indices[1]<2*block, 'witness indices outside blocks')
        require(isinstance(signs,list) and len(signs)==2 and all(type(s) is int and s in (-1,1) for s in signs), 'invalid signs')
        i,j=indices; su,sv=signs; key=(i,j,su,sv)
        require(key in expected_witnesses and key not in actual,'invalid or duplicate signed witness'); actual.add(key)
        u,v=accepted[i],accepted[j]
        A=(su*u[0]+sv*v[0]-ar)%n; B=(su*u[1]+sv*v[1]-br)%n
        require(type(witness['A']) is int and type(witness['B']) is int and witness['A']==A and witness['B']==B,'coefficient identity mismatch')
        if B==0:
            zero+=1; require(A==0 and witness['scalar'] is None and witness['verified'] is False,'invalid zero-B witness')
        else:
            nonzero+=1; w=witness['scalar']
            require(type(w) is int and 0<=w<n and (A+B*w)%n==0,'scalar fails coefficient identity')
            require(e.mul(w,P)==Q and witness['verified'] is True,'scalar fails full point verification'); verified+=1
    require(actual==expected_witnesses,'missing signed witnesses for reported roots')
    counts={'total':len(actual),'zero_b':zero,'nonzero_b':nonzero,'nonzero_b_verified':verified,'nonzero_b_verification_failed':0}
    require(record['relation_counts']==counts and record['useful_relations']==verified,'relation counts mismatch')
    return {'status':'verified','candidates':len(trace),'accepted':len(accepted),'reported_roots':len(roots),'signed_witnesses':len(actual),'nonzero_b':nonzero}


def main():
    started=time.perf_counter_ns(); cpu=time.process_time_ns()
    parser=argparse.ArgumentParser(); parser.add_argument('record'); parser.add_argument('--output',required=True)
    args=parser.parse_args(); path=Path(args.record); raw=path.read_bytes()
    result={'record_sha256':hashlib.sha256(raw).hexdigest(),'audit_source_sha256':hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
            'limitations':['No root-completeness proof or independent solver execution.', 'No seed-stream regeneration, timing validation, resource verification, or record authentication.', 'Without a verified scalar, subgroup membership of Q in <P> is not independently certified.', 'Primality and curve arithmetic are checked only for primes below 2^32.']}
    try:
        result.update(audit(json.loads(raw)))
    except (Invalid,KeyError,TypeError,ValueError,OverflowError) as exc:
        result.update({'status':'invalid','error':str(exc)})
    result['audit_interval']={'wall_ns':time.perf_counter_ns()-started,'cpu_ns':time.process_time_ns()-cpu,'scope':'from main entry through input read, hashing, and verification; excludes interpreter startup and receipt writing'}
    with Path(args.output).open('x') as out:
        json.dump(result,out,indent=2,sort_keys=True); out.write('\n')
    print(json.dumps({'status':result['status']}))
    return 1 if result['status']=='invalid' else 0

if __name__=='__main__': raise SystemExit(main())
