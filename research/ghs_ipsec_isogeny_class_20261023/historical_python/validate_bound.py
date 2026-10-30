#!/usr/bin/env python3
"""Independent product-span exclusion checks using binary linear algebra.

No eigenvalue support is used to decide these exclusions: the kernels of
f(sigma) are constructed as 155-by-155 binary linear maps, and all products
of their basis vectors are spanned directly in the original field basis.
"""
import json
from pathlib import Path
import time
from analyze import F, FACTORS31, SUPPORTS, IPSEC_B, eval_sigma, pmul, polystr


def add_row(rows, v):
    while v:
        pivot = v.bit_length()-1
        if pivot in rows:
            v ^= rows[pivot]
        else:
            rows[pivot] = v
            return True
    return False


def contains(rows, v):
    while v:
        pivot = v.bit_length()-1
        if pivot not in rows:
            return False
        v ^= rows[pivot]
    return True


def kernel(poly):
    rows = {}
    relations = []
    for i in range(155):
        v = eval_sigma(poly, 1 << i)
        combination = 1 << i
        while v:
            pivot = v.bit_length()-1
            if pivot in rows:
                w, c = rows[pivot]
                v ^= w
                combination ^= c
            else:
                rows[pivot] = v, combination
                break
        if not v:
            assert eval_sigma(poly, combination) == 0
            relations.append(combination)
    assert len(relations) == 5*(poly.bit_length()-1)
    return relations


def main():
    start = time.perf_counter()
    fundamental = [kernel(p) for p in FACTORS31]
    all_bases = {}
    for mask, support in SUPPORTS:
        basis = []
        for i, component in enumerate(fundamental):
            if (mask >> i) & 1:
                basis.extend(component)
        assert len(basis) == 5*len(support)
        all_bases[mask] = basis
    checks = []
    for i, (mask_a, a) in enumerate(SUPPORTS):
        for mask_b, b in SUPPORTS[i:]:
            t, r, s = len(a | b), len(a), len(b)
            genus = (1 << t) - (1 << (t-r)) - (1 << (t-s)) + 1
            if genus >= 992:
                continue
            rows = {}
            for alpha in all_bases[mask_a]:
                for beta in all_bases[mask_b]:
                    add_row(rows, F.mul(alpha, beta))
            member = contains(rows, IPSEC_B)
            assert not member, (mask_a, mask_b, genus)
            checks.append({'alpha_mask': mask_a, 'beta_mask': mask_b,
                           'genus': genus, 'binary_product_span_rank': len(rows),
                           'b_in_product_span': member})
    # The first support-admissible pair must pass the linear relaxation.
    rows = {}
    for alpha in all_bases[8]:
        for beta in all_bases[72]:
            add_row(rows, F.mul(alpha, beta))
    assert contains(rows, IPSEC_B)
    output = {'method': 'explicit binary kernel/product-space elimination',
              'fixed_curve_b': hex(IPSEC_B), 'descent_subfield': 'F32',
              'Hess_genus_lower_bound_validated': 992,
              'support_pairs_below_bound_checked': len(checks),
              'all_pairs_excluded': True,
              'first_admissible_pair_product_span_rank': len(rows),
              'first_admissible_pair_has_actual_decomposition': 'not established',
              'checks': checks, 'elapsed_seconds': time.perf_counter()-start}
    Path(__file__).with_name('bound_certificate.json').write_text(
        json.dumps(output, indent=2)+'\n')
    print(json.dumps({k:v for k,v in output.items() if k != 'checks'}, indent=2))


if __name__ == '__main__':
    main()
