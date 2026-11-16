#!/usr/bin/env python3
"""Check source binding and exact fields in the finite receipts."""
import hashlib
import json
import math
import re
from pathlib import Path

p=Path(__file__).resolve().parent
for stem in ('check_w0_geometry','check_previous_target'):
    src=(p/(stem+'.magma')).read_bytes()
    d=json.loads((p/(stem+'.json')).read_text())
    assert d['input_sha256']==hashlib.sha256(src).hexdigest()
    assert d['http_status']==200
    assert d['response']['success'] is True
    assert d['response']['exit_code']==0
    stdout=d['response']['stdout']
    assert 'curve_genus 7' in stdout
    assert 'hyperelliptic_over_base false' in stdout
    assert 'geometrically_hyperelliptic false' in stdout

old=json.loads((p/'../ghs_n7_isogenous_w0_20261115/w0_map_receipt.json').read_text())
printed=old['response']['stdout'].split('curve Curve over GF(2) defined by',1)[1].split('point (',1)[0]
printed=printed.replace('$.1','X').replace('$.2','Y').replace('$.3','Z')
written=(p/'check_previous_target.magma').read_text().split('Curve(P,',1)[1].split(');',1)[0]
assert re.sub(r'\s+','',printed)==re.sub(r'\s+','',written)

d=json.loads((p/'prime23_inputs.json').read_text())
assert d['source_sha256']==hashlib.sha256((p/'verify_prime23.sage').read_bytes()).hexdigest()
assert d['source_order']==138 and d['carrier_point_counts']==[3,5,9,17,33,65,192]
assert d['carrier_v_mask']==0x8f and d['carrier_u_mask']==0xc07a
assert d['irreducible_mod_13'] and d['squarefree_mod_23'] and d['discriminant_prime_to_23']
assert d['frobenius_polynomial']=='X^14 + 9*X^7 + 128'

def prime_below_2_to_64(n):
    if n<2:
        return False
    for small in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n%small==0:
            return n==small
    odd=n-1; shifts=0
    while odd%2==0:
        odd//=2; shifts+=1
    for base in (2,325,9375,28178,450775,9780504,1795265022):
        if base%n==0:
            continue
        x=pow(base%n,odd,n)
        if x in (1,n-1):
            continue
        for _ in range(shifts-1):
            x=x*x%n
            if x==n-1:
                break
        else:
            return False
    return True

ext=json.loads((p/'extension_primes.json').read_text())
assert ext['source_sha256']==hashlib.sha256((p/'verify_extensions.sage').read_bytes()).hexdigest()
assert [row['s'] for row in ext['rows']]==[1,3,5,9,11,13,15]
ts=[2,-9]
for _ in range(2,16):
    ts.append(-9*ts[-1]-128*ts[-2])
for row in ext['rows']:
    s=row['s']; n=2**(7*s)+1-ts[s]
    assert row['trace']==ts[s] and row['source_order']==n
    assert math.prod(q**e for q,e in row['factorization'])==n
    assert all(q<2**64 and prime_below_2_to_64(q) for q,e in row['factorization'])
    assert row['largest_prime']==max(q for q,e in row['factorization'])
    assert row['largest_prime_bits']==row['largest_prime'].bit_length()
    assert math.gcd(row['largest_prime'],2*7*(ts[s]**2-4*2**(7*s)))==1
print(json.dumps({'schema':'ghs.w1-twist-bridge.structural-audit/v1',
                  'source_bound_receipts':4,'prior_w0_target_equation_matches':True,
                  'w0_targets_geometrically_nonhyperelliptic':2,
                  'w1_source_order':138,'selected_prime':23,
                  'carrier_counts':d['carrier_point_counts'],
                  'extension_rows':len(ext['rows']),
                  'largest_extension_prime_bits':max(row['largest_prime_bits'] for row in ext['rows'])},sort_keys=True))
