#!/usr/bin/env sage -python
"""Independent full hidden-point replay for a second order-4126 source."""

from hashlib import sha256
import itertools
import json
from pathlib import Path
import sys

from sage.all import EllipticCurve, GF, HyperellipticCurve, PolynomialRing


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
PARENT = ROOT / 'ghs_automorphism_screen_q16_20261026'
sys.path.insert(0, str(PARENT))
from screen_degree3_pattern import (
    as_coboundary, branch_residue, curves, parse_model,
    scalar_from_simple_zero, second_pattern,
)


def digest(path):
    return sha256(path.read_bytes()).hexdigest()


def main():
    catalog_path = PARENT/'catalog_models_raw.json'
    fast_path = HERE/'all_sources_certificate.json'
    catalog = json.loads(catalog_path.read_text())
    fast = json.loads(fast_path.read_text())
    expected = next(z for z in fast['per_source'] if z['a6'] == 378)
    lines = curves(next(z for z in catalog['rows']
                        if z['label']=='3.16.a_a_bd')['curves'])
    assert len(lines) == 140
    k = GF(16, 'a', modulus='conway')
    K = GF(4096, 'w', modulus='conway')
    embed = k.embeddings(K)[0]
    Rk, RK = PolynomialRing(k, 'x'), PolynomialRing(K, 'x')
    xk, x = Rk.gen(), RK.gen()
    a2, a6 = K.from_integer(32), K.from_integer(378)
    E = EllipticCurve(K, [K.one(), a2, K.zero(), K.zero(), a6])
    assert E.cardinality() == 4126
    count4 = count5 = residue5 = hits = 0
    for model_index, line in enumerate(lines):
        h0, f0 = parse_model(line, Rk, k.gen(), xk)
        assert HyperellipticCurve(f0,h0).genus() == 3
        h = RK([embed(z) for z in h0.list()])
        f = RK([embed(z) for z in f0.list()])
        roots = sorted(h.roots(multiplicities=False),
                       key=lambda z: int(z.to_integer()))
        assert len(roots) == 4
        assert all(branch_residue(h,f,r) != 0 for r in roots)
        for indices in itertools.permutations(range(4)):
            r0,r1,r2,r3 = [roots[j] for j in indices]
            scalar = scalar_from_simple_zero(
                h,f,r0,r1,r2,r3,a6.sqrt())
            R = scalar*(x-r0)**2*(x-r1)/((x-r2)**2*(x-r3))
            difference = f/h**2 + R + a2 + a6/R**2
            hits += as_coboundary(difference, roots, x, K) is not None
            count4 += 1
        local5, local_residue, local_hits = second_pattern(
            h,f,roots,x,K,a2,a6,model_index)
        count5 += local5
        residue5 += local_residue
        hits += len(local_hits)
    assert count4 == expected['four_assignments_covered']
    assert count5 == expected['five_assignments_covered']
    assert residue5 == expected['five_residue_pass']
    assert hits == expected['hits'] == 0
    output = {'schema':'ghs-q16-4126-degree3-replay-378/v1',
              'script_sha256':digest(Path(__file__)),
              'catalog_sha256':digest(catalog_path),
              'fast_certificate_sha256':digest(fast_path),
              'source_a2':32,'source_a6':378,'source_order':4126,
              'models_checked':len(lines),
              'four_assignments_enumerated':count4,
              'five_assignments_enumerated':count5,
              'five_residue_pass':residue5,'hits':hits}
    (HERE/'replay_378_certificate.json').write_text(
        json.dumps(output,indent=2)+'\n')
    print(json.dumps(output,indent=2),flush=True)


if __name__=='__main__':
    main()
