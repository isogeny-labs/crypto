#!/usr/bin/env python3
"""Freeze the thirteen exact alternate-target survivors as Magma input."""
import json
from pathlib import Path

P = Path(__file__).resolve().parent
target = P / 'degree5_isogeny_survivors.magma'
if target.exists():
    raise SystemExit('refusing to overwrite immutable Magma input')
masks = (27, 30, 103, 139, 167, 182, 194, 230, 233)
lines = [
    '// Independent Magma full-equation check of thirteen alternate-target survivors.',
    'F := GF(3);',
    'R0<z> := PolynomialRing(F);',
    'K<a> := ext<F | z^5+2*z+1>;',
    'R<x> := PolynomialRing(K);',
    'f := x^11+x^9+2*x^8+x^7+2*x^5+x^3+x^2+x+1;',
    'assert GCD(f,Derivative(f)) eq 1;',
]
for mask in masks:
    data = json.loads((P / f'degree5_isogeny_replay_mask{mask}.json').read_text())
    assert data['a6_mask'] == mask and not data['hits']
    for i, row in enumerate(data['survivors'], 1):
        assert row['equation_remainder_zero'] and not row['quotient_square']
        lines.extend([
            f'// b-mask {mask}, survivor {i}',
            f'b := K!({data["a6"]});',
            f'A := R!({row["A"]});',
            f'B := R!({row["B"]});',
            'assert Degree(A) eq 5 and Degree(B) eq 4 and GCD(A,B) eq 1;',
            'N := B*(A^3+2*A^2*B+b*B^3);',
            'assert N mod f eq 0;',
            'Q := N div f;',
            f'print "mask",{mask},"survivor",{i},"quotient_degree",Degree(Q),"square",IsSquare(Q);',
            'print "factor_degrees",[<Degree(t[1]),t[2]> : t in Factorization(Q)];',
            'assert not IsSquare(Q);',
        ])
assert sum(len(json.loads((P / f'degree5_isogeny_replay_mask{m}.json').read_text())['survivors']) for m in masks) == 13
target.write_text('\n'.join(lines) + '\n')
print(target)
