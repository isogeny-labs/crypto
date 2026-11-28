"""Replay recorded algebraic certificates only; no candidate/oracle search."""
from pathlib import Path
import collections, datetime, gzip, hashlib, json, math

RAW = Path('research/odd_regular_20261115_v4/raw-controls.json.gz')
REVIEW = Path('research/odd_regular_review_20261115_v4')
blob = RAW.read_bytes()
assert hashlib.sha256(blob).hexdigest() == '642f3b9da55bc3c829064313b6a6c5a0b526647333e998d8a0a6f7ea75cfd3e2'
raw = json.loads(gzip.decompress(blob))
assert raw['error'] is None
out = raw['out']
counts = collections.Counter()
positives = []

for record in out['records']:
    q = record['q']
    p, n, modulus = {3:(3,1,0),5:(5,1,0),9:(3,2,1),25:(5,2,2)}[q]
    def encode(v): return v[0] % p + (p * (v[1] % p) if n == 2 else 0)
    def plus(x,y): return encode([x%p+y%p, x//p+y//p])
    def negative(x): return encode([-x%p, -(x//p)%p])
    def times(x,y):
        aa,bb=[x%p,x//p],[y%p,y//p]
        cc=[0,0,0]
        for i in range(n):
            for j in range(n): cc[i+j] += aa[i]*bb[j]
        cc[0] -= modulus*cc[2]
        return encode(cc[:2])
    def trim(v):
        v=list(v)
        while v and not v[-1]: v.pop()
        return v
    def add(a,b): return trim([plus(a[i] if i<len(a) else 0,b[i] if i<len(b) else 0) for i in range(max(len(a),len(b)))])
    def neg(a): return [negative(x) for x in a]
    def multiply(a,b):
        if not a or not b: return []
        v=[0]*(len(a)+len(b)-1)
        for i,x in enumerate(a):
            for j,y in enumerate(b): v[i+j]=plus(v[i+j],times(x,y))
        return trim(v)
    def value(a,x):
        z=0
        for c in reversed(a): z=plus(times(z,x),c)
        return z
    def derivative(a): return trim([times(i%p,a[i]) for i in range(1,len(a))])
    def remainder(a,b):
        assert b[-1] == 1
        a=trim(a)
        while len(a)>=len(b):
            shift=len(a)-len(b); lead=a[-1]
            sub=[0]*shift+[negative(times(lead,x)) for x in b]
            a=add(a,sub)
        return a
    def roots_product(roots):
        v=[1]
        for x in roots: v=multiply(v,[negative(x),1])
        return v

    C,z,y,h,a,ya=record['cell']
    assert times(y,y)==value(C,z) and times(ya,ya)==value(C,a)
    if record['status']=='excluded_a_equals_z':
        assert a==z
        counts['excluded_target_collision'] += 1
        continue
    assert a!=z and record['common_factor_status']=='UNKNOWN'
    assert record['common_factor_raw_assignments']==q*q
    assert record['expected']==record['derived']
    assert len(record['derived'])==(q-1 if ya else 0)
    assert len({tuple(v) for v in record['derived']})==len(record['derived'])
    counts['admitted_cells'] += 1
    if not ya: counts['ramified_empty_cells'] += 1
    for candidate in record['candidates']:
        k,r,s=candidate['k'],candidate['r'],candidate['s']
        P=candidate['P'];G=[k,h];N=candidate['norm'];H=candidate['H'];Q=candidate['Q']
        assert P==[candidate['t'],s,r,1]
        assert [k,r,s] in record['derived']
        assert N==add(multiply(P,P),neg(multiply(C,multiply(G,G))))
        assert N==multiply([negative(z),1],H)
        assert H==multiply(multiply([negative(a),1],[negative(a),1]),Q)
        ga=plus(times(h,a),k)
        assert ga and value(P,z)==negative(times(plus(times(h,z),k),y))
        assert value(P,a)==negative(times(ga,ya)) and value(derivative(N),a)==0
        domain=candidate['domain']
        assert domain==sorted({a,plus(a,1)})
        Ad=roots_product(domain)
        assert candidate['remainder']==Ad
        residue=remainder(multiply(multiply(Ad,Ad),Ad),Q)
        assert candidate['cube_residue']==residue
        assert candidate['supported']==(not residue)
        sources=candidate['sources']
        if sources:
            assert candidate['supported'] and len({tuple(v) for v in sources})==len(sources)
            multiset=collections.Counter(sources[0])
            assert multiset[a]>=2
            degree3=multiset.copy();degree3[a]-=2
            factors=[v for v,num in degree3.items() for _ in range(num)]
            assert len(factors)==3 and roots_product(factors)==Q
            assert all(collections.Counter(v)==multiset and all(x in domain for x in v) for v in sources)
            denominator=math.prod(math.factorial(v) for v in multiset.values())
            assert len(sources)==math.factorial(5)//denominator
            positives.append(dict(q=q,k=k,source_orders=len(sources),target_disjoint=z not in domain))
        else: assert not candidate['supported']
        counts['candidate_certificates_replayed'] += 1

assert counts['candidate_certificates_replayed']==174
assert len(positives)==out['positive_supported_parameters']==5
result=dict(schema='recorded-result-review/v1',created_utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),
    review_mode='Independent coordinate-convolution replay of stored norm/factor/derivative/support/source-order certificates only; no candidate or dense-grid oracle search rerun.',
    raw_sha256=hashlib.sha256(blob).hexdigest(),raw_bytes=len(blob),error=raw['error'],
    recorded_predicates=raw['predicates'],recorded_raw_states=raw['raw_r_s_k_states'],recorded_norm_evaluations=raw['literal_norm_evaluations'],
    certificate_counts=dict(counts),positive_certificates=positives,
    fields=[dict(counts=v['counts'],missing_categories=v['selection']['missing_categories'],available_categories=v['selection']['available_categories'],selection_candidates=v['selection']['selection_candidates']) for v in out['fields']],
    interpretation='Odd conditional regular norm controls passed. Positive GF3 cases overlap target/source domain; three GF5 positives are disjoint. GF9/GF25 lack positive coverage. GF3 r1_nonzero category absent in declared scan. Common-factor/original auxiliaries/global coverage UNKNOWN. No measured speedup, novelty or NIST hard-instance evidence.')
target=REVIEW/'RESULT_REVIEW.json'
with target.open('x') as stream: json.dump(result,stream,indent=2);stream.write('\n')
pins=REVIEW/'RESULT_PINS.json'
with pins.open('x') as stream:
    json.dump({'sha256':{str(RAW):hashlib.sha256(blob).hexdigest(),str(target):hashlib.sha256(target.read_bytes()).hexdigest(),str(Path(__file__)):hashlib.sha256(Path(__file__).read_bytes()).hexdigest()}},stream,indent=2);stream.write('\n')
print(json.dumps({'certificate_counts':dict(counts),'positive_certificates':positives,'raw_sha256':result['raw_sha256']}))
