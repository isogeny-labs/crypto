# Independent SageMath check of exact EC subgroup and isogeny image receipts.
from sage.all import GF, PolynomialRing, EllipticCurve
from pathlib import Path
import json

root=Path(__file__).resolve().parent/'results'
count=0
for path in sorted(root.glob('exact-*.json')):
    receipt=json.loads(path.read_text())
    src=receipt['source']; n=int(src['absolute_degree']); modulus=int(src['modulus'],0)
    ring=PolynomialRing(GF(2),'x'); x=ring.gen()
    mod=sum(x**i for i in range(n+1) if modulus >> i & 1)
    field=GF(2**n,'z',modulus=mod); z=field.gen()
    elem=lambda word: sum(z**i for i in range(n) if int(word) >> i & 1)
    def make_curve(a,b):
        return EllipticCurve(field,[field.one(),elem(a),field.zero(),field.zero(),elem(b)])
    E=make_curve(int(src['a'],0),int(src['b'],0))
    def point(curve,coordinates):
        return curve(0) if coordinates[2] else curve(elem(coordinates[0]),elem(coordinates[1]))
    P=point(E,src['generator']);Q=point(E,src['target'])
    r=int(src['subgroup_order']);scalar=int(src['target_scalar_for_replay'])
    assert E.cardinality()==int(src['group_order'])
    assert r*P==E(0) and P!=E(0)
    assert Q==scalar*P
    iso=receipt['isogeny']
    if iso:
        E2=make_curve(int(iso['codomain_a'],0),int(iso['codomain_b'],0))
        assert E2.cardinality()==E.cardinality()
        kernel=[point(E,t) for t in iso['kernel_points']]
        assert all(t!=E(0) and int(iso['degree'])*t==E(0) for t in kernel)
        imageP=point(E2,iso['image_generator']);imageQ=point(E2,iso['image_target'])
        assert imageP!=E2(0) and r*imageP==E2(0)
        assert imageQ==scalar*imageP
    assert receipt['ghs_status']=='verified'
    assert int(receipt['ghs']['target_descent']['scalar'])==scalar
    assert int(receipt['rho']['result']['scalar'])==scalar
    print(f'{path.name} Sage EC/subgroup/isogeny image and scalar checks=pass')
    count+=1
print(f'{count} exact receipts independently checked')
